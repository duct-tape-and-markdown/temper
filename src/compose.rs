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
//! ## The role roster (parse-only)
//!
//! A `temper.toml` may also carry top-level `[role.<name>]` tables — the
//! harness-contract tier (`specs/10-contracts.md`, "Roles and matching"): an
//! abstract role bound to whichever concrete artifact fills it. Each parses into
//! a typed [`Role`] — its artifact kind, the contract the filler must conform to
//! (a template path or inline `[[clause]]`s, the latter through the same
//! [`contract::parse_clauses`]), a decidable [`MatchSelector`] (a name glob or a
//! `role` marker, stored *verbatim* — never matched here), an optional `required`
//! flag (absent ⇒ false; `temper` never fabricates a gate the author did not
//! declare, `00-intent.md` law 4), and an optional `verified_by` verifier.
//!
//! This tier is **parse only**: the roster loads into typed values and a
//! malformed role is a load error, but no selection, single-filler conformance,
//! or admissibility (does `match` resolve, does `verified_by` resolve) runs yet —
//! those are separate follow-on entries.
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
    /// The harness-contract roles parsed from `[role.<name>]` tables, keyed by
    /// role name. Empty when the `temper.toml` declares none. Parse-only in this
    /// tier — selection, required-filling, and admissibility are follow-on
    /// entries.
    roles: BTreeMap<String, Role>,
}

/// A harness-contract **role**: an abstract slot bound to whichever concrete
/// artifact fills it (`specs/10-contracts.md`, "Roles and matching"). The engine
/// checks a filler is `present`, `conforms-to` the role's contract, and is picked
/// by its `match` selector — but this tier only *parses* the declaration into
/// typed values; none of those checks run here.
///
/// Not `Eq`: its [`RoleContract`] may carry inline clauses with `f64` `range`
/// bounds (see [`crate::contract::Contract`]); equality stays derived as
/// `PartialEq`.
#[derive(Debug, Clone, PartialEq)]
pub struct Role {
    /// The role's name — the `[role.<name>]` table key.
    pub name: String,
    /// The artifact kind expected to fill the role (`skill`, `command`, …),
    /// stored verbatim. Not validated against a closed kind set in this tier.
    pub artifact: String,
    /// The contract the filling artifact must conform to: an adopted template
    /// named by path, or inline clauses over the closed vocabulary.
    pub contract: RoleContract,
    /// The decidable selector that picks the filling artifact. Stored verbatim —
    /// the glob/marker is *not* evaluated against any surface in this tier.
    pub selector: MatchSelector,
    /// Whether an absent filler is a conformance violation. Absent in source ⇒
    /// `false`: `temper` never fabricates a gate the author did not declare
    /// (`00-intent.md` law 4). Mutually exclusive with [`Role::count`]: `required`
    /// is the single-filler shorthand, `count` the general cardinality form.
    pub required: bool,
    /// An optional bound on the matched-set cardinality — the set-scope `count`
    /// predicate (`specs/45-governance.md`, "The set scope (the roster)"): the
    /// number of artifacts matching the selector must land in `[min, max]`. Absent
    /// ⇒ `None` (no cardinality gate beyond `required`'s single-filler one). The
    /// general form of `required`; the two are mutually exclusive.
    pub count: Option<CountBound>,
    /// The declared field names held unique across the role's matched set — the
    /// set-scope `unique` predicate (`specs/45-governance.md`, "The set scope (the
    /// roster)"): each named field's extracted scalar must not repeat across the
    /// role's matched fillers. Absent ⇒ empty (no uniqueness gate). Generalizes the
    /// kind-wide `unique-name` engine predicate from name-only over a whole kind to
    /// an arbitrary field over a role's matched subset. Checked in [`crate::roster`].
    pub unique: Vec<String>,
    /// An optional external verifier for the behavioral remainder (`verified_by`).
    /// Stored verbatim; whether it *resolves* is an admissibility check left to a
    /// follow-on entry.
    pub verified_by: Option<String>,
}

/// An inclusive bound on the cardinality of a role's matched set — the set-scope
/// `count` predicate (`specs/45-governance.md`, "The set scope (the roster)"). The
/// number of artifacts the role's selector matches must land in `[min, max]`;
/// "at most N agents" is `{ min = 0, max = N }`, "exactly one planner" is
/// `{ min = 1, max = 1 }`. An inverted `min > max` bound admits no cardinality and
/// is rejected as inadmissible (`crate::roster`), mirroring `range`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountBound {
    /// The inclusive lower bound on the matched-set size.
    pub min: usize,
    /// The inclusive upper bound on the matched-set size.
    pub max: usize,
}

/// A role's contract reference: the filler's contract is either an adopted
/// template named by path, or inline clauses declared under the role
/// (`[[role.<name>.clause]]`) over the same closed vocabulary a bare contract
/// carries (`specs/10-contracts.md`, "Roles and matching").
///
/// Not `Eq` — its inline [`Clause`]s may carry `f64` `range` bounds; equality
/// stays derived as `PartialEq`.
#[derive(Debug, Clone, PartialEq)]
pub enum RoleContract {
    /// A template adopted by path (`contract = "contracts/skill.anthropic.toml"`).
    /// Stored verbatim; whether the path resolves is a follow-on admissibility
    /// check.
    Template(String),
    /// Inline clauses declared under the role, parsed by the shared
    /// [`contract::parse_clauses`] so an unknown predicate is rejected exactly as
    /// in a bare contract.
    Inline(Vec<Clause>),
}

impl RoleContract {
    /// Resolve this reference into the concrete [`Contract`] the engine validates
    /// a role's filler against (`specs/10-contracts.md`, "Roles and matching":
    /// the `role` primitive's `conforms-to` half). `Inline` wraps its already-
    /// parsed clauses directly, labelled `label` (the role name) for diagnostics;
    /// `Template` loads its path **relative to `base_dir`** — the `temper.toml`
    /// directory — and parses it through [`Contract::load`].
    ///
    /// A non-resolving or malformed template path is an *admissibility* concern
    /// (the template-resolve clause of the roster-admissibility follow-on entry),
    /// bubbled here as the [`ContractError`] so the caller can skip the
    /// conformance check rather than double-report what admissibility owns.
    pub fn resolve(&self, base_dir: &Path, label: &str) -> Result<Contract, ContractError> {
        match self {
            RoleContract::Inline(clauses) => Ok(Contract {
                name: label.to_string(),
                clauses: clauses.clone(),
            }),
            RoleContract::Template(rel) => Contract::load(&base_dir.join(rel)),
        }
    }
}

/// The decidable `match` selector picking a role's filler — a closed set
/// (`specs/10-contracts.md`, "Roles and matching"). The pattern is stored
/// *verbatim* and never matched here; resolving it against artifacts is a
/// follow-on entry, so no glob crate enters at this tier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSelector {
    /// By artifact name glob (`match = { name = "plan*" }`).
    Name {
        /// The glob, stored verbatim — not compiled or matched in this tier.
        glob: String,
    },
    /// By an explicit role marker the artifact declares / opts into
    /// (`match = { role = "task-planning" }`) — the "artifact opts in" option.
    Role {
        /// The marker the filling artifact must declare, stored verbatim.
        marker: String,
    },
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

    /// The top-level `role` key is present but is not a table of role definitions.
    #[error("{path}: `role` must be a table of harness-contract roles")]
    #[diagnostic(code(temper::compose::role_root_not_table))]
    RoleRootNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
    },

    /// A `[role.<name>]` entry is present but is not a table.
    #[error("{path}: `[role.{role}]` must be a table")]
    #[diagnostic(code(temper::compose::role_not_table))]
    RoleNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role whose definition is malformed.
        role: String,
    },

    /// A `[role.<name>]` is missing its required `artifact` kind.
    #[error("{path}: `[role.{role}]` is missing required key `artifact`")]
    #[diagnostic(code(temper::compose::role_missing_artifact))]
    RoleMissingArtifact {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role missing its artifact kind.
        role: String,
    },

    /// A `[role.<name>]` is missing its required `match` selector.
    #[error("{path}: `[role.{role}]` is missing required key `match`")]
    #[diagnostic(code(temper::compose::role_missing_match))]
    RoleMissingMatch {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role missing its selector.
        role: String,
    },

    /// A `[role.<name>]` key has the wrong TOML type.
    #[error("{path}: `[role.{role}]` key `{key}` must be {expected}")]
    #[diagnostic(code(temper::compose::role_wrong_type))]
    RoleWrongType {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role whose key is mistyped.
        role: String,
        /// The mistyped key.
        key: &'static str,
        /// The type that was expected, for the message.
        expected: &'static str,
    },

    /// A `[role.<name>]` declares neither a `contract` template path nor inline
    /// `[[clause]]`s — a role with no contract names no shape for its filler to
    /// conform to.
    #[error(
        "{path}: `[role.{role}]` must declare a contract — a `contract` template path or inline `[[clause]]`s"
    )]
    #[diagnostic(code(temper::compose::role_no_contract))]
    RoleNoContract {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role missing a contract.
        role: String,
    },

    /// A `[role.<name>]` declares *both* a `contract` template path and inline
    /// `[[clause]]`s — the reference is ambiguous; exactly one is admissible.
    #[error(
        "{path}: `[role.{role}]` declares both a `contract` template path and inline `[[clause]]`s; choose one"
    )]
    #[diagnostic(code(temper::compose::role_ambiguous_contract))]
    RoleAmbiguousContract {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with a doubly-declared contract.
        role: String,
    },

    /// A `[role.<name>]`'s `match` selector is not exactly one of the closed set
    /// (a `name` glob or a `role` marker). Zero, many, or an unknown key all land
    /// here — matching is a decidable selector, never an open guess.
    #[error(
        "{path}: `[role.{role}]` `match` must name exactly one decidable selector (`name` glob or `role` marker)"
    )]
    #[diagnostic(code(temper::compose::role_bad_match))]
    RoleBadMatch {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with the malformed selector.
        role: String,
    },

    /// A `[role.<name>]`'s `count` bound is malformed — not an inline table, or its
    /// `min`/`max` are missing, non-integer, or negative. The matched-set
    /// cardinality bound is a pair of `usize` counts, never an open guess; zero,
    /// missing, mistyped, or negative bounds all land here.
    #[error(
        "{path}: `[role.{role}]` `count` must be an inline table with non-negative integer `min` and `max` bounds"
    )]
    #[diagnostic(code(temper::compose::role_bad_count))]
    RoleBadCount {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with the malformed count bound.
        role: String,
    },

    /// A `[role.<name>]`'s `unique` declaration is malformed — not an array, or an
    /// element that is not a string. The set-scope `unique` predicate names a list
    /// of declared field names, never an open guess; a non-array or a non-string
    /// element lands here.
    #[error("{path}: `[role.{role}]` `unique` must be an array of declared field-name strings")]
    #[diagnostic(code(temper::compose::role_bad_unique))]
    RoleBadUnique {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with the malformed `unique` declaration.
        role: String,
    },

    /// A `[role.<name>]` declares *both* `required` and a `count` bound. The two
    /// are mutually exclusive: `required` is the single-filler shorthand, `count`
    /// the general cardinality form, so declaring both is ambiguous.
    #[error(
        "{path}: `[role.{role}]` declares both `required` and `count`; they are mutually exclusive (`count` is the general form of `required`'s single-filler bound)"
    )]
    #[diagnostic(code(temper::compose::role_count_and_required))]
    RoleCountAndRequired {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role declaring both.
        role: String,
    },

    /// A layered clause is outside the closed vocabulary (or otherwise malformed).
    /// Bubbled verbatim from [`crate::contract`] so the author layer's clauses are
    /// held to the exact same closed-vocabulary contract as a bare one's. Covers a
    /// role's inline `[[clause]]`s too, since they reuse [`contract::parse_clauses`].
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

        let mut roles = BTreeMap::new();
        if let Some(item) = doc.as_table().get("role") {
            let role_table = item
                .as_table()
                .ok_or_else(|| ComposeError::RoleRootNotTable {
                    path: path.to_path_buf(),
                })?;
            for (name, item) in role_table.iter() {
                let table = item.as_table().ok_or_else(|| ComposeError::RoleNotTable {
                    path: path.to_path_buf(),
                    role: name.to_string(),
                })?;
                roles.insert(name.to_string(), parse_role(table, name, path)?);
            }
        }

        Ok(Self {
            path: path.to_path_buf(),
            kinds,
            roles,
        })
    }

    /// The parsed role roster, keyed by role name. Empty when the `temper.toml`
    /// declares no `[role.<name>]` tables — a kind-only (or empty) layer carries
    /// an empty roster. Parse-only in this tier.
    #[must_use]
    pub fn roles(&self) -> &BTreeMap<String, Role> {
        &self.roles
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

/// Parse one `[role.<name>]` table into a typed [`Role`]: the required `artifact`
/// kind and `match` selector, the contract reference (a `contract` path string or
/// an inline `[[clause]]` array — exactly one), the optional `required` flag
/// (absent ⇒ `false`), and the optional `verified_by` verifier. Each malformed
/// field is a load error, mirroring `[kind.<k>]` parsing.
fn parse_role(table: &Table, role: &str, path: &Path) -> Result<Role, ComposeError> {
    let artifact = role_str(table, "artifact", role, path)?.ok_or_else(|| {
        ComposeError::RoleMissingArtifact {
            path: path.to_path_buf(),
            role: role.to_string(),
        }
    })?;
    let contract = parse_role_contract(table, role, path)?;
    let selector = parse_match(table, role, path)?;
    // `required` and `count` are two ways to express the same dimension (matched-set
    // cardinality), so declaring both is ambiguous — reject it before parsing either.
    if table.contains_key("required") && table.contains_key("count") {
        return Err(ComposeError::RoleCountAndRequired {
            path: path.to_path_buf(),
            role: role.to_string(),
        });
    }
    let required = parse_role_required(table, role, path)?;
    let count = parse_count(table, role, path)?;
    let unique = parse_unique(table, role, path)?;
    let verified_by = role_str(table, "verified_by", role, path)?;

    Ok(Role {
        name: role.to_string(),
        artifact,
        contract,
        selector,
        required,
        count,
        unique,
        verified_by,
    })
}

/// The role's optional `count` bound: an inline `count = { min, max }` table whose
/// `min` and `max` are non-negative integers (`usize`). Absent ⇒ `None`. Any
/// malformation — not a table, a missing/mistyped/negative bound — collapses to
/// [`ComposeError::RoleBadCount`], the way [`parse_match`] folds its malformations
/// into one error. The bound is stored verbatim; whether `min > max` (an
/// unsatisfiable bound) is an *admissibility* concern, checked in [`crate::roster`].
fn parse_count(table: &Table, role: &str, path: &Path) -> Result<Option<CountBound>, ComposeError> {
    let Some(item) = table.get("count") else {
        return Ok(None);
    };
    let bad_count = || ComposeError::RoleBadCount {
        path: path.to_path_buf(),
        role: role.to_string(),
    };
    let count_table = item.as_table_like().ok_or_else(bad_count)?;
    let min = count_bound(count_table, "min").ok_or_else(bad_count)?;
    let max = count_bound(count_table, "max").ok_or_else(bad_count)?;
    Ok(Some(CountBound { min, max }))
}

/// Read one `count` bound (`min`/`max`) off the inline table as a `usize`: present,
/// a TOML integer, and non-negative. Any miss — absent, a non-integer, or a
/// negative value (`usize` cannot hold one) — is `None`, which [`parse_count`]
/// reports as a single [`ComposeError::RoleBadCount`].
fn count_bound(table: &dyn toml_edit::TableLike, key: &str) -> Option<usize> {
    table
        .get(key)?
        .as_integer()
        .and_then(|n| usize::try_from(n).ok())
}

/// The role's optional `unique` field list: a `unique = ["field", …]` array of
/// declared field names, each held unique across the role's matched set by the
/// roster check (`specs/45-governance.md`, "The set scope (the roster)"). Absent ⇒
/// an empty vec (no uniqueness gate). Any malformation — not an array, or a
/// non-string element — collapses to [`ComposeError::RoleBadUnique`], the way
/// [`parse_count`] folds its malformations into one error. The names are stored
/// verbatim; grouping the matched fillers by each is left to [`crate::roster`].
fn parse_unique(table: &Table, role: &str, path: &Path) -> Result<Vec<String>, ComposeError> {
    let Some(item) = table.get("unique") else {
        return Ok(Vec::new());
    };
    let bad_unique = || ComposeError::RoleBadUnique {
        path: path.to_path_buf(),
        role: role.to_string(),
    };
    let array = item.as_array().ok_or_else(bad_unique)?;
    let mut fields = Vec::new();
    for value in array.iter() {
        fields.push(value.as_str().ok_or_else(bad_unique)?.to_string());
    }
    Ok(fields)
}

/// The role's contract reference — exactly one of a `contract` template path or
/// an inline `[[role.<name>.clause]]` array. Declaring neither names no shape;
/// declaring both is ambiguous; both are load errors. Inline clauses go through
/// the shared [`contract::parse_clauses`], so an unknown predicate is rejected
/// just as in a bare contract.
fn parse_role_contract(
    table: &Table,
    role: &str,
    path: &Path,
) -> Result<RoleContract, ComposeError> {
    let template = role_str(table, "contract", role, path)?;
    let has_clauses = table.contains_key("clause");
    match (template, has_clauses) {
        (Some(_), true) => Err(ComposeError::RoleAmbiguousContract {
            path: path.to_path_buf(),
            role: role.to_string(),
        }),
        (Some(template), false) => Ok(RoleContract::Template(template)),
        (None, true) => Ok(RoleContract::Inline(contract::parse_clauses(table, path)?)),
        (None, false) => Err(ComposeError::RoleNoContract {
            path: path.to_path_buf(),
            role: role.to_string(),
        }),
    }
}

/// The role's `match` selector: the inline `match` table must name exactly one of
/// the closed set — a `name` glob or a `role` marker — whose value is a string.
/// Absent ⇒ [`ComposeError::RoleMissingMatch`]; zero/many/unknown keys ⇒
/// [`ComposeError::RoleBadMatch`]. The pattern is stored verbatim, never matched.
fn parse_match(table: &Table, role: &str, path: &Path) -> Result<MatchSelector, ComposeError> {
    let item = table
        .get("match")
        .ok_or_else(|| ComposeError::RoleMissingMatch {
            path: path.to_path_buf(),
            role: role.to_string(),
        })?;
    let selector_table = item
        .as_table_like()
        .ok_or_else(|| ComposeError::RoleWrongType {
            path: path.to_path_buf(),
            role: role.to_string(),
            key: "match",
            expected: "an inline table",
        })?;

    let bad_match = || ComposeError::RoleBadMatch {
        path: path.to_path_buf(),
        role: role.to_string(),
    };

    let mut selector = None;
    for (key, value) in selector_table.iter() {
        if selector.is_some() {
            // A second selector key — `match` must name exactly one.
            return Err(bad_match());
        }
        let pattern = value.as_str().ok_or_else(|| ComposeError::RoleWrongType {
            path: path.to_path_buf(),
            role: role.to_string(),
            key: "match",
            expected: "a string selector value",
        })?;
        selector = Some(match key {
            "name" => MatchSelector::Name {
                glob: pattern.to_string(),
            },
            "role" => MatchSelector::Role {
                marker: pattern.to_string(),
            },
            _ => return Err(bad_match()),
        });
    }
    selector.ok_or_else(bad_match)
}

/// The role's optional `required` flag: absent ⇒ `false` (`temper` never
/// fabricates a gate the author did not declare); present-but-not-a-boolean ⇒ a
/// load error.
fn parse_role_required(table: &Table, role: &str, path: &Path) -> Result<bool, ComposeError> {
    match table.get("required") {
        None => Ok(false),
        Some(item) => item.as_bool().ok_or_else(|| ComposeError::RoleWrongType {
            path: path.to_path_buf(),
            role: role.to_string(),
            key: "required",
            expected: "a boolean",
        }),
    }
}

/// Read an optional string key off a `[role.<name>]` table: absent ⇒ `None`,
/// present-but-not-a-string ⇒ [`ComposeError::RoleWrongType`].
fn role_str(
    table: &Table,
    key: &'static str,
    role: &str,
    path: &Path,
) -> Result<Option<String>, ComposeError> {
    match table.get(key) {
        None => Ok(None),
        Some(item) => {
            Some(
                item.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| ComposeError::RoleWrongType {
                        path: path.to_path_buf(),
                        role: role.to_string(),
                        key,
                        expected: "a string",
                    }),
            )
            .transpose()
        }
    }
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

    // ---- role roster (parse-only) -----------------------------------------

    #[test]
    fn a_full_role_table_parses_into_a_typed_role() {
        // Every field present: artifact kind, a path-string contract, a name-glob
        // selector, an explicit `required`, and a `verified_by` verifier.
        let toml = r#"
[role.task-planning]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "plan*" }
required = true
verified_by = "tests/plan.rs"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer
            .roles()
            .get("task-planning")
            .expect("the role parses into the roster");
        assert_eq!(
            role,
            &Role {
                name: "task-planning".to_string(),
                artifact: "skill".to_string(),
                contract: RoleContract::Template("contracts/skill.anthropic.toml".to_string()),
                selector: MatchSelector::Name {
                    glob: "plan*".to_string(),
                },
                required: true,
                count: None,
                unique: Vec::new(),
                verified_by: Some("tests/plan.rs".to_string()),
            }
        );
    }

    #[test]
    fn an_inline_clause_contract_parses_via_the_shared_parser() {
        // No `contract` path: the role's contract is inline `[[clause]]`s, parsed
        // by the same closed-vocabulary parser a bare contract uses. The selector
        // is the opt-in `role` marker form.
        let toml = r#"
[role.release-tool]
artifact = "command"
match = { role = "release" }
[[role.release-tool.clause]]
severity = "required"
predicate = "required"
field = "description"
[[role.release-tool.clause]]
severity = "required"
predicate = "must_define"
marker = "executable"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("release-tool").expect("the role parses");
        assert_eq!(
            role.selector,
            MatchSelector::Role {
                marker: "release".to_string(),
            }
        );
        assert_eq!(
            role.contract,
            RoleContract::Inline(vec![
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::Required {
                        field: "description".to_string(),
                    },
                },
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::MustDefine {
                        marker: "executable".to_string(),
                    },
                },
            ])
        );
    }

    #[test]
    fn an_absent_required_flag_defaults_to_false() {
        // `temper` never fabricates a gate the author did not declare: an absent
        // `required` is `false`, not `true`.
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("linter").expect("the role parses");
        assert!(!role.required);
        assert_eq!(role.verified_by, None);
    }

    #[test]
    fn an_unknown_predicate_in_an_inline_role_contract_is_a_load_error() {
        // The shared parser rejects an out-of-vocabulary predicate in a role's
        // inline clauses exactly as it does in a bare contract — no escape hatch.
        let toml = r#"
[role.linter]
artifact = "skill"
match = { name = "lint*" }
[[role.linter.clause]]
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
    fn a_role_missing_its_artifact_kind_is_a_load_error() {
        let toml = r#"
[role.linter]
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleMissingArtifact { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_role_with_neither_a_contract_nor_inline_clauses_is_a_load_error() {
        let toml = r#"
[role.linter]
artifact = "skill"
match = { name = "lint*" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleNoContract { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_role_with_both_a_contract_and_inline_clauses_is_a_load_error() {
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*" }
[[role.linter.clause]]
severity = "required"
predicate = "required"
field = "name"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleAmbiguousContract { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_role_missing_its_match_selector_is_a_load_error() {
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleMissingMatch { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_match_with_an_unknown_selector_key_is_a_load_error() {
        // `path` is not in the closed selector set {name, role}.
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { path = "skills/lint" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleBadMatch { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_match_naming_two_selectors_is_a_load_error() {
        // Exactly one selector — `name` and `role` together is ambiguous.
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*", role = "lint" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadMatch { .. }));
    }

    #[test]
    fn a_non_boolean_required_flag_is_a_load_error() {
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*" }
required = "yes"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleWrongType {
                key: "required",
                ..
            }
        ));
    }

    #[test]
    fn a_non_table_role_root_is_a_load_error() {
        let err = AuthorLayer::parse("role = 7\n", Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleRootNotTable { .. }));
    }

    #[test]
    fn a_count_bound_parses_into_a_typed_role() {
        // The set-scope `count` predicate: an inline `{ min, max }` table parses
        // into a `CountBound`, and (being the general form of `required`) no
        // `required` flag rides alongside it.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = { min = 0, max = 3 }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert_eq!(role.count, Some(CountBound { min: 0, max: 3 }));
        assert!(!role.required);
    }

    #[test]
    fn a_non_table_count_is_a_load_error() {
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = 3
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleBadCount { ref role, .. } if role == "agents"
        ));
    }

    #[test]
    fn a_count_with_a_non_integer_bound_is_a_load_error() {
        // A `max` that is not a non-negative integer collapses to `RoleBadCount`,
        // the way a malformed `match` collapses to `RoleBadMatch`.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = { min = 0, max = "three" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadCount { .. }));
    }

    #[test]
    fn a_count_missing_a_bound_is_a_load_error() {
        // Both `min` and `max` are required — the bound is a closed pair, never a
        // half-open guess.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = { max = 3 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadCount { .. }));
    }

    #[test]
    fn a_negative_count_bound_is_a_load_error() {
        // A negative `min` cannot be a `usize` cardinality — rejected, not floored.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = { min = -1, max = 3 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadCount { .. }));
    }

    #[test]
    fn declaring_both_required_and_count_is_a_load_error() {
        // The two express the same dimension (matched-set cardinality); declaring
        // both is ambiguous, so it is rejected before either is read.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
required = true
count = { min = 0, max = 3 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleCountAndRequired { ref role, .. } if role == "agents"
        ));
    }

    #[test]
    fn a_unique_field_list_parses_into_a_typed_role() {
        // The set-scope `unique` predicate: a `unique = ["model"]` array parses into
        // `Role.unique`, the declared fields the roster holds unique across the set.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
unique = ["model"]
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert_eq!(role.unique, vec!["model".to_string()]);
    }

    #[test]
    fn an_absent_unique_defaults_to_an_empty_vec() {
        // `temper` never fabricates a gate the author did not declare: an absent
        // `unique` is no uniqueness gate, an empty vec.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert!(role.unique.is_empty());
    }

    #[test]
    fn a_non_array_unique_is_a_load_error() {
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
unique = "model"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleBadUnique { ref role, .. } if role == "agents"
        ));
    }

    #[test]
    fn a_unique_with_a_non_string_element_is_a_load_error() {
        // A non-string element collapses to `RoleBadUnique`, the way a malformed
        // `count` bound collapses to `RoleBadCount`.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
unique = ["model", 7]
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadUnique { .. }));
    }

    #[test]
    fn a_kind_only_temper_toml_carries_an_empty_roster() {
        // Customizing only `[kind.*]` leaves the role roster empty — and the kind
        // layer still works exactly as before.
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "advisory"
predicate = "max_lines"
max = 100
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        assert!(layer.roles().is_empty());
    }
}
