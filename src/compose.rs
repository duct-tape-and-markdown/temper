//! Composition — layer an optional author-declared `temper.toml` over the
//! embedded by-kind floor contracts.
//!
//! Implements `specs/40-composition.md` ("Decision: the author-declared contract
//! lives in `temper.toml`, layered"). `check` gates every harness against the
//! built-in contract for each artifact kind — the **floor** (`specs/20-surface.md`,
//! "contract selection is by artifact kind"). The floor needs no author input, but
//! a built-in is only `temper`'s curated default; `00-intent.md` law 2 (the author
//! declares; built-ins are overridable *data*) is only half-real until the author
//! can declare on top of it. This module is that other half — the optional,
//! project-root `temper.toml` that **layers over** the floor.
//!
//! ## What the layer does (this tier)
//!
//! A `temper.toml` carries a `[kind.<k>]` table per artifact kind it customizes.
//! Each does up to two things, both settled here:
//!
//! - **Adopt** — name the kind's shipped template explicitly (`adopt = "..."`),
//!   the default made visible. Today the sole shipped template per kind *is* the
//!   embedded floor, so the only admissible name is the floor's own; adopting any
//!   other is a load error — a template the tool does not ship cannot be selected.
//!   Omitting `adopt` takes the floor implicitly.
//! - **Extend / override / flip** — an inline `[[kind.<k>.clause]]` array of the
//!   *same* closed-vocabulary clauses a bare contract carries. Each layered clause
//!   either **overrides** the floor clause with the same identity (its predicate
//!   [`key`](crate::contract::Predicate::key) and the field it
//!   [`target`](crate::contract::Predicate::target)s) — which is how a severity
//!   flip (`required` ⟷ `advisory`) and a parameter change are both expressed — or,
//!   when no floor clause shares that identity, **extends** the floor with it.
//!
//! Binding the harness (roles, `verified_by`) — the interface/trait tier no
//! built-in can carry — is the next layer, deliberately out of this entry's scope.
//!
//! ## Closed vocabulary, end to end
//!
//! The clause array is parsed by the *same* [`crate::contract`] parser a bare
//! contract uses ([`contract::parse_clauses`]), so a layered clause naming an
//! unknown predicate is rejected at load exactly as it is in a standalone contract
//! — the author layer earns no escape hatch the floor lacks. And the effective
//! contract (floor ⊕ layer) is run through *both* greens (admissibility +
//! conformance) in `check`, so an inadmissible override — an empty `enum`, say —
//! fails admissibility on the layered result, never slipping through because the
//! floor was clean.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use toml_edit::{DocumentMut, Table};

use crate::contract::{self, Clause, Contract, ContractError};

/// The author-declared layer parsed from a project-root `temper.toml`: a per-kind
/// set of adoptions and clause overrides to apply over the embedded floor.
#[derive(Debug, Clone)]
pub struct AuthorLayer {
    /// The source path, retained so a layering error (an unknown adopted
    /// template) can name the file it came from.
    path: PathBuf,
    /// The per-kind layers, keyed by artifact kind (`skill`, `rule`, …). A kind
    /// the author did not name falls through to the floor unchanged.
    kinds: BTreeMap<String, KindLayer>,
}

/// One kind's customization: an optional adopted template and the clauses to layer
/// over that kind's floor.
#[derive(Debug, Clone)]
struct KindLayer {
    /// The explicitly adopted template, if the author named one. Validated against
    /// the kind's floor at layering time (`AuthorLayer::layer_over`).
    adopt: Option<String>,
    /// The override / extend clauses, in declaration order.
    clauses: Vec<Clause>,
}

/// Errors raised while loading or applying a `temper.toml`. Hard failures (an
/// unreadable or malformed file, a layer that adopts a template the tool does not
/// ship, a clause outside the closed vocabulary) — distinct from a lint finding,
/// which the check engine collects rather than throws.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ComposeError {
    /// The `temper.toml` exists but could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::compose::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// The `temper.toml` is not valid TOML.
    #[error("failed to parse {path} as TOML")]
    #[diagnostic(code(temper::compose::toml))]
    Toml {
        /// The file that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
    },

    /// The top-level `kind` key is present but is not a table of per-kind layers.
    #[error("{path}: `kind` must be a table of per-kind contract layers")]
    #[diagnostic(code(temper::compose::kind_not_table))]
    KindRootNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
    },

    /// A `[kind.<k>]` entry is present but is not a table.
    #[error("{path}: `[kind.{kind}]` must be a table")]
    #[diagnostic(code(temper::compose::kind_layer_not_table))]
    KindLayerNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The artifact kind whose layer is malformed.
        kind: String,
    },

    /// A `[kind.<k>]` layer's `adopt` value is not a string.
    #[error("{path}: `[kind.{kind}]` `adopt` must be a string")]
    #[diagnostic(code(temper::compose::adopt_not_string))]
    AdoptNotString {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The artifact kind whose `adopt` is mistyped.
        kind: String,
    },

    /// A `[kind.<k>]` layer adopts a template the tool does not ship. With only
    /// the embedded floor shipped per kind, the sole admissible name is the floor's
    /// own — selecting anything else is rejected, never silently ignored.
    #[error(
        "{path}: `[kind.{kind}]` adopts unknown template `{adopt}` (the only shipped `{kind}` template is `{expected}`)"
    )]
    #[diagnostic(
        code(temper::compose::unknown_template),
        help("adopt the kind's shipped template by its name, or omit `adopt` to take the floor")
    )]
    UnknownTemplate {
        /// The `temper.toml` that named the template.
        path: PathBuf,
        /// The artifact kind whose layer adopts it.
        kind: String,
        /// The unrecognized adopted template name.
        adopt: String,
        /// The kind's actual shipped (floor) template name.
        expected: String,
    },

    /// A layered clause is outside the closed vocabulary (or otherwise malformed).
    /// Bubbled verbatim from [`crate::contract`] so the author layer's clauses are
    /// held to the exact same closed-vocabulary contract as a bare one's.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Contract(#[from] ContractError),
}

impl AuthorLayer {
    /// Load the optional `temper.toml` at `path`. A missing file is not an error —
    /// it is the floor-only path — so absence returns `Ok(None)`, and the floor
    /// runs unchanged.
    pub fn load(path: &Path) -> Result<Option<Self>, ComposeError> {
        match fs::read_to_string(path) {
            Ok(src) => Ok(Some(Self::parse(&src, path)?)),
            Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(source) => Err(ComposeError::Io {
                path: path.to_path_buf(),
                source,
            }),
        }
    }

    /// Parse a `temper.toml` from source. `path` only labels diagnostics, so this
    /// is the seam tests drive without touching disk.
    pub fn parse(src: &str, path: &Path) -> Result<Self, ComposeError> {
        let doc = src
            .parse::<DocumentMut>()
            .map_err(|source| ComposeError::Toml {
                path: path.to_path_buf(),
                source,
            })?;

        let mut kinds = BTreeMap::new();
        if let Some(item) = doc.as_table().get("kind") {
            let kind_table = item
                .as_table()
                .ok_or_else(|| ComposeError::KindRootNotTable {
                    path: path.to_path_buf(),
                })?;
            for (name, item) in kind_table.iter() {
                let table = item
                    .as_table()
                    .ok_or_else(|| ComposeError::KindLayerNotTable {
                        path: path.to_path_buf(),
                        kind: name.to_string(),
                    })?;
                kinds.insert(name.to_string(), parse_kind_layer(table, name, path)?);
            }
        }

        Ok(Self {
            path: path.to_path_buf(),
            kinds,
        })
    }

    /// The effective contract for `kind`: this layer's clauses for that kind
    /// applied over `floor`. A kind the author did not name returns `floor`
    /// unchanged, so a `temper.toml` that customizes only some kinds leaves the
    /// rest exactly at the floor.
    ///
    /// Each layered clause **overrides** the floor clause sharing its identity (the
    /// predicate key plus targeted field) — the severity flip and the parameter
    /// change both land here — or, with a new identity, **extends** the floor by
    /// appending. An `adopt` that names a template other than the kind's floor is a
    /// load error: the floor is the only shipped template this tier can select.
    pub fn layer_over(&self, kind: &str, floor: Contract) -> Result<Contract, ComposeError> {
        let Some(layer) = self.kinds.get(kind) else {
            return Ok(floor);
        };

        if let Some(adopt) = &layer.adopt
            && adopt != &floor.name
        {
            return Err(ComposeError::UnknownTemplate {
                path: self.path.clone(),
                kind: kind.to_string(),
                adopt: adopt.clone(),
                expected: floor.name.clone(),
            });
        }

        let mut clauses = floor.clauses;
        for clause in &layer.clauses {
            match clauses
                .iter()
                .position(|existing| same_identity(existing, clause))
            {
                Some(index) => clauses[index] = clause.clone(),
                None => clauses.push(clause.clone()),
            }
        }
        Ok(Contract {
            name: floor.name,
            clauses,
        })
    }
}

/// The effective contract for `kind` given an *optional* author layer: `floor`
/// unchanged when there is no layer, else [`AuthorLayer::layer_over`] applied. The
/// `Option` seam keeps the absent-`temper.toml` path — every existing test's path —
/// a verbatim pass-through of the floor.
pub fn effective(
    layer: Option<&AuthorLayer>,
    kind: &str,
    floor: Contract,
) -> Result<Contract, ComposeError> {
    match layer {
        Some(layer) => layer.layer_over(kind, floor),
        None => Ok(floor),
    }
}

/// Parse one `[kind.<k>]` table into its [`KindLayer`] — the optional `adopt`
/// template name and the inline `[[clause]]` array, the latter through the shared
/// closed-vocabulary parser ([`contract::parse_clauses`]).
fn parse_kind_layer(table: &Table, kind: &str, path: &Path) -> Result<KindLayer, ComposeError> {
    let adopt = match table.get("adopt") {
        None => None,
        Some(item) => Some(
            item.as_str()
                .ok_or_else(|| ComposeError::AdoptNotString {
                    path: path.to_path_buf(),
                    kind: kind.to_string(),
                })?
                .to_string(),
        ),
    };
    let clauses = contract::parse_clauses(table, path)?;
    Ok(KindLayer { adopt, clauses })
}

/// Whether two clauses address the same thing — the same predicate key and the
/// same targeted field (or both field-less). This is a clause's *layering
/// identity*: a layered clause sharing it overrides the floor clause, while a
/// clause with a fresh identity extends the floor.
fn same_identity(a: &Clause, b: &Clause) -> bool {
    a.predicate.key() == b.predicate.key() && a.predicate.target() == b.predicate.target()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::contract::{Predicate, Severity};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-compose-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A small skill-shaped floor: a required `max_len` on `name`, a required
    /// `forbidden_keys`, and an advisory `max_lines`. Enough distinct identities to
    /// exercise override-vs-extend.
    fn floor() -> Contract {
        Contract {
            name: "skill.anthropic".to_string(),
            clauses: vec![
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::MaxLen {
                        field: "name".to_string(),
                        max: 64,
                    },
                },
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::ForbiddenKeys {
                        keys: vec!["globs".to_string()],
                    },
                },
                Clause {
                    severity: Severity::Advisory,
                    predicate: Predicate::MaxLines { max: 500 },
                },
            ],
        }
    }

    #[test]
    fn no_layer_for_a_kind_returns_the_floor_unchanged() {
        let toml = r#"
[kind.rule]
[[kind.rule.clause]]
severity = "advisory"
predicate = "max_lines"
max = 100
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        // The layer names only `rule`; the `skill` floor passes through verbatim.
        assert_eq!(layer.layer_over("skill", floor()).unwrap(), floor());
    }

    #[test]
    fn a_severity_flip_overrides_the_matching_floor_clause_in_place() {
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "advisory"
predicate = "forbidden_keys"
keys = ["globs"]
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let effective = layer.layer_over("skill", floor()).unwrap();

        // Same identity (key + no field) ⇒ override in place, not append: the
        // clause count is unchanged and the order is preserved.
        assert_eq!(effective.clauses.len(), floor().clauses.len());
        assert_eq!(effective.clauses[1].severity, Severity::Advisory);
        assert_eq!(
            effective.clauses[1].predicate,
            Predicate::ForbiddenKeys {
                keys: vec!["globs".to_string()]
            }
        );
    }

    #[test]
    fn a_parameter_override_replaces_the_floor_clause_with_the_same_identity() {
        // Same predicate key *and* field (`max_len` on `name`) ⇒ the layered bound
        // replaces the floor's, in place.
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 32
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let effective = layer.layer_over("skill", floor()).unwrap();

        assert_eq!(effective.clauses.len(), floor().clauses.len());
        assert_eq!(
            effective.clauses[0].predicate,
            Predicate::MaxLen {
                field: "name".to_string(),
                max: 32
            }
        );
    }

    #[test]
    fn a_new_identity_extends_the_floor_by_appending() {
        // `min_len` on `name` shares no identity with any floor clause (the floor's
        // `max_len` on `name` is a different key) ⇒ appended, floor preserved.
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "required"
predicate = "min_len"
field = "name"
min = 1
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let effective = layer.layer_over("skill", floor()).unwrap();

        assert_eq!(effective.clauses.len(), floor().clauses.len() + 1);
        // The original floor clauses are untouched and the new clause is last.
        assert_eq!(&effective.clauses[..3], &floor().clauses[..]);
        assert_eq!(
            effective.clauses[3].predicate,
            Predicate::MinLen {
                field: "name".to_string(),
                min: 1
            }
        );
    }

    #[test]
    fn an_unknown_predicate_in_a_layered_clause_is_a_load_error() {
        // The shared closed-vocabulary parser rejects it at parse, exactly as it
        // does for a bare contract — the author layer has no escape hatch.
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "required"
predicate = "word_count"
field = "description"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::Contract(ContractError::UnknownPredicate { ref predicate, .. })
                if predicate == "word_count"
        ));
    }

    #[test]
    fn adopting_the_floor_template_by_name_is_the_default_made_explicit() {
        // Naming the kind's own floor template is admissible and changes nothing.
        let toml = r#"
[kind.skill]
adopt = "skill.anthropic"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        assert_eq!(layer.layer_over("skill", floor()).unwrap(), floor());
    }

    #[test]
    fn adopting_a_template_the_tool_does_not_ship_is_a_load_error() {
        let toml = r#"
[kind.skill]
adopt = "skill.cursor"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let err = layer.layer_over("skill", floor()).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::UnknownTemplate { ref adopt, ref expected, .. }
                if adopt == "skill.cursor" && expected == "skill.anthropic"
        ));
    }

    #[test]
    fn an_empty_temper_toml_and_an_absent_one_both_yield_the_floor() {
        // Present-but-declares-nothing parses to a layer with no kinds, so every
        // kind falls through to the floor — the same result as `effective(None,..)`.
        let layer = AuthorLayer::parse("# nothing here\n", Path::new("temper.toml")).unwrap();
        assert_eq!(layer.layer_over("skill", floor()).unwrap(), floor());
        assert_eq!(effective(None, "skill", floor()).unwrap(), floor());
        assert_eq!(effective(Some(&layer), "skill", floor()).unwrap(), floor());
    }

    #[test]
    fn load_returns_none_for_an_absent_file_and_some_for_a_present_one() {
        let dir = tmpdir("load");
        let path = dir.join("temper.toml");
        // Absent ⇒ None (the floor-only path).
        assert!(AuthorLayer::load(&path).unwrap().is_none());

        // Present ⇒ Some, parsed from disk.
        fs::write(&path, "[kind.skill]\nadopt = \"skill.anthropic\"\n").unwrap();
        let layer = AuthorLayer::load(&path)
            .unwrap()
            .expect("a present file loads");
        assert_eq!(layer.layer_over("skill", floor()).unwrap(), floor());
    }

    #[test]
    fn a_non_table_kind_entry_is_a_load_error() {
        let err = AuthorLayer::parse("kind = 7\n", Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::KindRootNotTable { .. }));
    }
}
