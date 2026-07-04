//! Assembly-fact artifacts — the two **locus-less** assembly facts `emit` lands as
//! small committed temper-owned TOML files beside `temper.toml`/`lock.toml`
//! (`specs/architecture/20-surface.md`, "The surface: the assembly over its contents":
//! "the bindings, the roster — are emitted as small committed temper-owned artifacts").
//! Bindings and the roster have no harness locus, so emit compiles them here rather than
//! into any one member — everything the offline engine reads stays a committed artifact.
//!
//! Read-only: this mirrors the SDK emitter (`sdk/src/assembly_artifacts.ts`) on the
//! parse side. The gate consumes these as the **assembly source** when they sit beside a
//! members-only `temper.toml`, so an SDK-emitted manifest resolves its requirements and
//! bindings instead of reporting a spurious dangling `satisfies`. The data dialect is
//! TOML (the surface's declared-data dialect); a kind name is carried **qualified**
//! (`claude-code.rule`) and resolved here to the **bare** key the gate dispatches on.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use toml_edit::{DocumentMut, Table};

use crate::compose::Requirement;

/// The requirement roster's committed filename, beside `temper.toml`
/// (`sdk/src/assembly_artifacts.ts`, `ROSTER_PATH`).
pub const ROSTER_FILE: &str = "roster.toml";

/// The kind→package bindings' committed filename, beside `temper.toml`
/// (`sdk/src/assembly_artifacts.ts`, `BINDINGS_PATH`).
pub const BINDINGS_FILE: &str = "bindings.toml";

/// The two parsed assembly facts the gate folds over a members-only `temper.toml`: the
/// requirement roster (keyed by name) and the kind→package bindings (keyed by the
/// **bare** kind name the gate dispatches on). Either map is empty when its file is
/// absent; the whole value is `None` only when neither file exists (the pure
/// `temper.toml` path).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AssemblyArtifacts {
    /// The requirement roster parsed from `roster.toml`. Each requirement carries only
    /// the three facets emit serializes — `means`, `kind`, `required` — the richer
    /// set-scope facets stay `temper.toml`-only and default here.
    pub requirements: BTreeMap<String, Requirement>,
    /// The kind→package bindings parsed from `bindings.toml`, keyed by bare kind name.
    pub bindings: BTreeMap<String, String>,
}

/// A failure reading or parsing one of the assembly-fact artifacts. Every variant names
/// the file it came from so a malformed emit output is traced to its source.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AssemblyArtifactsError {
    /// An assembly-fact file exists but could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::assembly_artifacts::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// An assembly-fact file is not valid TOML.
    #[error("failed to parse {path} as TOML")]
    #[diagnostic(code(temper::assembly_artifacts::toml))]
    Toml {
        /// The file that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
    },

    /// A top-level key outside the file's single modeled root — a typo that would
    /// silently drop meaning, so it is rejected, not ignored.
    #[error("{path}: unknown top-level key `{key}` (expected `{expected}`)")]
    #[diagnostic(code(temper::assembly_artifacts::unknown_root_key))]
    UnknownRootKey {
        /// The malformed artifact.
        path: PathBuf,
        /// The stray root key.
        key: String,
        /// The one root key the file models (`requirement` or `binding`).
        expected: &'static str,
    },

    /// The modeled root key is present but is not a table of named entries.
    #[error("{path}: `{root}` must be a table")]
    #[diagnostic(code(temper::assembly_artifacts::root_not_table))]
    RootNotTable {
        /// The malformed artifact.
        path: PathBuf,
        /// The root key that was not a table (`requirement` or `binding`).
        root: &'static str,
    },

    /// A `[requirement.<name>]` or `[binding.<kind>]` entry is not a table.
    #[error("{path}: `[{root}.{name}]` must be a table")]
    #[diagnostic(code(temper::assembly_artifacts::entry_not_table))]
    EntryNotTable {
        /// The malformed artifact.
        path: PathBuf,
        /// The root the entry sits under (`requirement` or `binding`).
        root: &'static str,
        /// The entry name/kind whose table is malformed.
        name: String,
    },

    /// A requirement/binding field carries the wrong TOML type.
    #[error("{path}: `[{root}.{name}]` key `{key}` must be {expected}")]
    #[diagnostic(code(temper::assembly_artifacts::bad_field))]
    BadField {
        /// The malformed artifact.
        path: PathBuf,
        /// The root the entry sits under (`requirement` or `binding`).
        root: &'static str,
        /// The entry the field belongs to.
        name: String,
        /// The mistyped key.
        key: String,
        /// The type the key must carry (`a string`, `a boolean`).
        expected: &'static str,
    },

    /// A requirement/binding table carries a key outside the file's closed vocabulary.
    #[error("{path}: `[{root}.{name}]` has unknown key `{key}`")]
    #[diagnostic(code(temper::assembly_artifacts::unknown_key))]
    UnknownKey {
        /// The malformed artifact.
        path: PathBuf,
        /// The root the entry sits under (`requirement` or `binding`).
        root: &'static str,
        /// The entry the stray key belongs to.
        name: String,
        /// The stray key.
        key: String,
    },

    /// A `[binding.<kind>]` table declares no `package`.
    #[error("{path}: `[binding.{kind}]` must declare a `package`")]
    #[diagnostic(code(temper::assembly_artifacts::binding_no_package))]
    BindingNoPackage {
        /// The malformed artifact.
        path: PathBuf,
        /// The kind whose binding named no package.
        kind: String,
    },
}

/// Load the two assembly-fact artifacts from `dir` (the directory holding `temper.toml`).
/// Returns `None` when **neither** file exists — the pure `temper.toml` path, where the
/// gate reads the roster/bindings off the manifest layer alone. When either is present,
/// the missing one contributes an empty map.
///
/// # Errors
///
/// Returns an [`AssemblyArtifactsError`] if a present file is unreadable, is not valid
/// TOML, or violates the closed shape emit writes.
pub fn load(dir: &Path) -> Result<Option<AssemblyArtifacts>, AssemblyArtifactsError> {
    let roster_path = dir.join(ROSTER_FILE);
    let bindings_path = dir.join(BINDINGS_FILE);
    let roster_src = read_opt(&roster_path)?;
    let bindings_src = read_opt(&bindings_path)?;

    if roster_src.is_none() && bindings_src.is_none() {
        return Ok(None);
    }

    let requirements = match roster_src {
        Some(src) => parse_roster(&src, &roster_path)?,
        None => BTreeMap::new(),
    };
    let bindings = match bindings_src {
        Some(src) => parse_bindings(&src, &bindings_path)?,
        None => BTreeMap::new(),
    };
    Ok(Some(AssemblyArtifacts {
        requirements,
        bindings,
    }))
}

/// Read a file to a string, mapping a missing file to `None` (not an error — an absent
/// artifact is the pure `temper.toml` path).
fn read_opt(path: &Path) -> Result<Option<String>, AssemblyArtifactsError> {
    match fs::read_to_string(path) {
        Ok(src) => Ok(Some(src)),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(AssemblyArtifactsError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

/// Parse `roster.toml` into the requirement roster — one `[requirement.<name>]` table
/// per requirement carrying `means`, `kind` (resolved to its bare name), and the
/// `required` flag (`sdk/src/assembly_artifacts.ts`, `serializeRoster`).
fn parse_roster(
    src: &str,
    path: &Path,
) -> Result<BTreeMap<String, Requirement>, AssemblyArtifactsError> {
    let doc = parse_document(src, path)?;
    reject_unknown_roots(&doc, "requirement", path)?;

    let mut requirements = BTreeMap::new();
    if let Some(item) = doc.as_table().get("requirement") {
        let table = item
            .as_table()
            .ok_or_else(|| AssemblyArtifactsError::RootNotTable {
                path: path.to_path_buf(),
                root: "requirement",
            })?;
        for (name, item) in table.iter() {
            let entry = item
                .as_table()
                .ok_or_else(|| AssemblyArtifactsError::EntryNotTable {
                    path: path.to_path_buf(),
                    root: "requirement",
                    name: name.to_string(),
                })?;
            requirements.insert(name.to_string(), parse_requirement(entry, name, path)?);
        }
    }
    Ok(requirements)
}

/// Parse one `[requirement.<name>]` table into a [`Requirement`]. Only the three facets
/// emit serializes are admitted — `means`, `kind`, `required`; the richer set-scope
/// facets a hand-written `temper.toml` may carry are not emitted here and default. The
/// `kind` is resolved to its **bare** name so it keys the gate's bare corpus, exactly as
/// a spliced-in `temper.toml` requirement's bare `kind` would.
fn parse_requirement(
    table: &Table,
    name: &str,
    path: &Path,
) -> Result<Requirement, AssemblyArtifactsError> {
    for (key, _) in table.iter() {
        if !matches!(key, "means" | "kind" | "required") {
            return Err(AssemblyArtifactsError::UnknownKey {
                path: path.to_path_buf(),
                root: "requirement",
                name: name.to_string(),
                key: key.to_string(),
            });
        }
    }

    let means = opt_str(table, "means", "requirement", name, path)?;
    let kind = bare_kind(opt_str(table, "kind", "requirement", name, path)?);
    let required = opt_bool(table, "required", name, path)?;

    Ok(Requirement {
        name: name.to_string(),
        means,
        kind,
        package: None,
        required,
        count: None,
        unique: Vec::new(),
        membership: None,
        degree: None,
        verified_by: None,
    })
}

/// Parse `bindings.toml` into the kind→package map — one `[binding.<kind>]` table per
/// binding, its dotted kind quoted into a single sub-key
/// (`sdk/src/assembly_artifacts.ts`, `serializeBindings`). Each kind is resolved to its
/// **bare** name, the key the gate's `[kind.<name>]` layering and dispatch read.
fn parse_bindings(
    src: &str,
    path: &Path,
) -> Result<BTreeMap<String, String>, AssemblyArtifactsError> {
    let doc = parse_document(src, path)?;
    reject_unknown_roots(&doc, "binding", path)?;

    let mut bindings = BTreeMap::new();
    if let Some(item) = doc.as_table().get("binding") {
        let table = item
            .as_table()
            .ok_or_else(|| AssemblyArtifactsError::RootNotTable {
                path: path.to_path_buf(),
                root: "binding",
            })?;
        for (kind, item) in table.iter() {
            let entry = item
                .as_table()
                .ok_or_else(|| AssemblyArtifactsError::EntryNotTable {
                    path: path.to_path_buf(),
                    root: "binding",
                    name: kind.to_string(),
                })?;
            for (key, _) in entry.iter() {
                if key != "package" {
                    return Err(AssemblyArtifactsError::UnknownKey {
                        path: path.to_path_buf(),
                        root: "binding",
                        name: kind.to_string(),
                        key: key.to_string(),
                    });
                }
            }
            let package = opt_str(entry, "package", "binding", kind, path)?.ok_or_else(|| {
                AssemblyArtifactsError::BindingNoPackage {
                    path: path.to_path_buf(),
                    kind: kind.to_string(),
                }
            })?;
            // Resolve the qualified `claude-code.rule` to the bare `rule` the gate keys on.
            let bare = kind.rsplit('.').next().unwrap_or(kind).to_string();
            bindings.insert(bare, package);
        }
    }
    Ok(bindings)
}

/// Parse a source string into a TOML document, tagging a parse error with `path`.
fn parse_document(src: &str, path: &Path) -> Result<DocumentMut, AssemblyArtifactsError> {
    src.parse::<DocumentMut>()
        .map_err(|source| AssemblyArtifactsError::Toml {
            path: path.to_path_buf(),
            source,
        })
}

/// Reject any top-level key other than the file's single modeled root — a stray root is
/// a typo, never silently ignored (`specs/architecture/10-contracts.md`, unknown keys rejected).
fn reject_unknown_roots(
    doc: &DocumentMut,
    expected: &'static str,
    path: &Path,
) -> Result<(), AssemblyArtifactsError> {
    for (key, _) in doc.as_table().iter() {
        if key != expected {
            return Err(AssemblyArtifactsError::UnknownRootKey {
                path: path.to_path_buf(),
                key: key.to_string(),
                expected,
            });
        }
    }
    Ok(())
}

/// A required-optional string field: `None` when absent, an error when present but not a
/// string. `root`/`name` label a type-mismatch diagnostic.
fn opt_str(
    table: &Table,
    key: &str,
    root: &'static str,
    name: &str,
    path: &Path,
) -> Result<Option<String>, AssemblyArtifactsError> {
    match table.get(key) {
        None => Ok(None),
        Some(item) => item.as_str().map(|s| Some(s.to_string())).ok_or_else(|| {
            AssemblyArtifactsError::BadField {
                path: path.to_path_buf(),
                root,
                name: name.to_string(),
                key: key.to_string(),
                expected: "a string",
            }
        }),
    }
}

/// A requirement's `required` flag: `false` when absent (matching emit, which omits it
/// when false), an error when present but not a boolean.
fn opt_bool(
    table: &Table,
    key: &str,
    name: &str,
    path: &Path,
) -> Result<bool, AssemblyArtifactsError> {
    match table.get(key) {
        None => Ok(false),
        Some(item) => item
            .as_bool()
            .ok_or_else(|| AssemblyArtifactsError::BadField {
                path: path.to_path_buf(),
                root: "requirement",
                name: name.to_string(),
                key: key.to_string(),
                expected: "a boolean",
            }),
    }
}

/// Resolve a requirement's declared `kind` to its **bare** name — strip any provider
/// prefix (`claude-code.skill` → `skill`), leaving a bare name unchanged. The gate keys
/// its corpus and admissibility on the bare name (`resolve_member_kind`, `src/main.rs`),
/// so an SDK-qualified roster kind must resolve the same way a member's does.
fn bare_kind(kind: Option<String>) -> Option<String> {
    kind.map(|k| k.rsplit('.').next().unwrap_or(&k).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The byte shape the SDK emitter produces for a two-requirement roster
    /// (`sdk/test/assembly_artifacts.test.ts`): requirements name-sorted, `required`
    /// emitted only when set, the kind carried qualified.
    const ROSTER: &str = "[requirement.agent-playbook]\n\
         means = \"a shared agent playbook exists\"\n\
         kind = \"claude-code.skill\"\n\
         required = true\n\
         \n\
         [requirement.engineering-standards]\n\
         means = \"the repo carries a rule fixing the engineering bar\"\n\
         kind = \"claude-code.rule\"\n";

    /// The byte shape the SDK emitter produces for two bindings — the dotted kind quoted
    /// into a single sub-key.
    const BINDINGS: &str = "[binding.\"claude-code.rule\"]\n\
         package = \"rule.anthropic\"\n\
         \n\
         [binding.\"claude-code.skill\"]\n\
         package = \"skill.anthropic\"\n";

    #[test]
    fn parses_the_roster_resolving_each_kind_to_its_bare_name() {
        let requirements = parse_roster(ROSTER, Path::new("roster.toml")).unwrap();
        assert_eq!(requirements.len(), 2);

        let agent = &requirements["agent-playbook"];
        assert_eq!(
            agent.means.as_deref(),
            Some("a shared agent playbook exists")
        );
        // The qualified `claude-code.skill` resolves to the bare `skill` the gate keys on.
        assert_eq!(agent.kind.as_deref(), Some("skill"));
        assert!(agent.required);

        let eng = &requirements["engineering-standards"];
        assert_eq!(eng.kind.as_deref(), Some("rule"));
        // `required` absent ⇒ false, matching emit's omit-when-false spelling.
        assert!(!eng.required);
    }

    #[test]
    fn parses_the_bindings_resolving_each_kind_to_its_bare_name() {
        let bindings = parse_bindings(BINDINGS, Path::new("bindings.toml")).unwrap();
        assert_eq!(
            bindings.get("rule").map(String::as_str),
            Some("rule.anthropic")
        );
        assert_eq!(
            bindings.get("skill").map(String::as_str),
            Some("skill.anthropic")
        );
    }

    #[test]
    fn empty_artifacts_parse_to_empty_maps() {
        assert!(
            parse_roster("", Path::new("roster.toml"))
                .unwrap()
                .is_empty()
        );
        assert!(
            parse_bindings("", Path::new("bindings.toml"))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn load_returns_none_when_neither_file_is_present() {
        let dir = std::env::temp_dir().join(format!("temper-aa-none-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        assert_eq!(load(&dir).unwrap(), None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_reads_whichever_files_are_present() {
        let dir = std::env::temp_dir().join(format!("temper-aa-some-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(ROSTER_FILE), ROSTER).unwrap();
        fs::write(dir.join(BINDINGS_FILE), BINDINGS).unwrap();

        let artifacts = load(&dir).unwrap().expect("both files present");
        assert_eq!(artifacts.requirements.len(), 2);
        assert_eq!(artifacts.bindings.len(), 2);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_stray_root_key_is_rejected() {
        let err =
            parse_roster("[binding.x]\npackage = \"p\"\n", Path::new("roster.toml")).unwrap_err();
        assert!(matches!(err, AssemblyArtifactsError::UnknownRootKey { .. }));
    }

    #[test]
    fn a_stray_requirement_key_is_rejected() {
        let err = parse_roster(
            "[requirement.x]\nmeans = \"m\"\ncount = { min = 1, max = 2 }\n",
            Path::new("roster.toml"),
        )
        .unwrap_err();
        assert!(matches!(err, AssemblyArtifactsError::UnknownKey { .. }));
    }

    #[test]
    fn a_binding_without_a_package_is_rejected() {
        let err = parse_bindings(
            "[binding.\"claude-code.rule\"]\n",
            Path::new("bindings.toml"),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            AssemblyArtifactsError::BindingNoPackage { .. }
        ));
    }
}
