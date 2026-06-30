//! The `spec` artifact — the typed IR for `temper`'s own `specs/*.md` corpus.
//!
//! `spec` is temper's own custom artifact kind (`90-spec-system.md`): a spec is
//! **pure prose** — "No frontmatter, no schema, no" structured header at all. So
//! this is the no-frontmatter [`Rule`](crate::rule::Rule) shape stripped down: a
//! byte-faithful body (the *whole* file) plus provenance, and nothing else — no
//! `paths`, no `extra` catch-all, because a spec never carries frontmatter keys
//! to type or preserve. The spec name is the file stem, not an internal field.
//!
//! A spec is read from its source file with [`Spec::from_source_file`] (no
//! frontmatter to split — the entire file is the body; the original bytes are
//! hashed for provenance), projected to the typed surface header with
//! [`Spec::to_meta_document`] (a provenance-only `meta.toml`), and reloaded from
//! the written surface with [`Spec::from_surface_dir`].
//!
//! Round-trip discipline (`.claude/rules/rust.md`): the markdown body is
//! byte-faithful — never re-rendered. Only the structured header (`meta.toml`) is
//! written, via `toml_edit`. `import_hash` is the SHA-256 of the original file
//! bytes — the drift anchor, computed at import so the provenance lock is complete
//! before write-back exists.
//!
//! The SHA-256 helper is duplicated from `src/rule.rs` rather than shared: one
//! artifact kind per module (`.claude/rules/rust.md`) keeps the spec
//! self-contained, and clarity beats a premature de-duplication.

use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use toml_edit::{DocumentMut, Item, Table, value};

/// A temper spec: a byte-faithful prose body and the provenance lock that anchors
/// drift. A spec carries no frontmatter (`90-spec-system.md`), so it has no typed
/// header fields beyond its name and origin.
#[derive(Debug, Clone, PartialEq)]
pub struct Spec {
    /// Spec name, derived from the source file stem (`20-surface.md` ->
    /// `20-surface`).
    pub name: String,
    /// The whole markdown file, byte-faithful (trailing bytes intact). A spec has
    /// no frontmatter to strip, so this is the entire source.
    pub body: String,
    /// Where the spec came from and the hash of its original bytes.
    pub provenance: Provenance,
}

/// The import lock for a spec: its origin path and a content hash of the original
/// source bytes. `import_hash` drives future drift detection.
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    /// Absolute or workspace-relative path to the source `.md` file.
    pub source_path: PathBuf,
    /// Lowercase hex SHA-256 of the original source bytes.
    pub import_hash: String,
}

/// Errors raised while reading or projecting a [`Spec`]. These are hard failures
/// (missing file, malformed surface) — distinct from a lint `Diagnostic`, which
/// is a finding the engine collects rather than an error.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum SpecError {
    /// A file could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::spec::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The source `.md` file is not valid UTF-8, so its body cannot be modelled.
    #[error("{path} is not valid UTF-8")]
    #[diagnostic(code(temper::spec::not_utf8))]
    NotUtf8 {
        /// The offending file.
        path: PathBuf,
        /// The decode error.
        #[source]
        source: std::string::FromUtf8Error,
    },

    /// The source path has no file stem to derive the spec name from.
    #[error("{path} has no file stem to use as the spec name")]
    #[diagnostic(code(temper::spec::no_name))]
    NoName {
        /// The path missing a usable stem.
        path: PathBuf,
    },

    /// A required field is absent from the written surface (`meta.toml` or its
    /// surface directory name).
    #[error("{path}: surface is missing required field `{field}`")]
    #[diagnostic(code(temper::spec::missing_field))]
    MissingField {
        /// The surface whose header is incomplete.
        path: PathBuf,
        /// The required key that was absent.
        field: &'static str,
    },

    /// `meta.toml` could not be parsed as TOML.
    #[error("failed to parse {path} as TOML")]
    #[diagnostic(code(temper::spec::bad_toml))]
    Toml {
        /// The surface header that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
    },
}

impl Spec {
    /// Parse a spec from its source file `specs/<name>.md`: read the bytes, take
    /// the *whole* file as the body, and record provenance.
    ///
    /// A spec carries no frontmatter ever (`90-spec-system.md`), so — unlike a
    /// rule — there is nothing to split: the body is the entire file, byte-faithful.
    /// The name is the file stem; the `import_hash` is the SHA-256 of the original
    /// file bytes.
    pub fn from_source_file(path: &Path) -> Result<Self, SpecError> {
        let bytes = fs::read(path).map_err(|source| SpecError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let import_hash = sha256_hex(&bytes);
        let body = String::from_utf8(bytes).map_err(|source| SpecError::NotUtf8 {
            path: path.to_path_buf(),
            source,
        })?;

        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(str::to_string)
            .ok_or_else(|| SpecError::NoName {
                path: path.to_path_buf(),
            })?;

        Ok(Self {
            name,
            body,
            provenance: Provenance {
                source_path: path.to_path_buf(),
                import_hash,
            },
        })
    }

    /// Reload a spec from its written surface `<dir>/`: `meta.toml` (the
    /// `[provenance]` table) plus `<dir>/SPEC.md` (the body alone). The name is the
    /// surface directory name.
    ///
    /// The inverse of [`Spec::to_meta_document`] over the same directory:
    /// `import_hash` is read back from the provenance lock, not recomputed (the
    /// surface body lives in its own file, separate from the original source path).
    pub fn from_surface_dir(dir: &Path) -> Result<Self, SpecError> {
        let name = dir
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_string)
            .ok_or_else(|| SpecError::MissingField {
                path: dir.to_path_buf(),
                field: "name",
            })?;

        let meta_path = dir.join("meta.toml");
        let meta_src = fs::read_to_string(&meta_path).map_err(|source| SpecError::Io {
            path: meta_path.clone(),
            source,
        })?;
        let doc = meta_src
            .parse::<DocumentMut>()
            .map_err(|source| SpecError::Toml {
                path: meta_path.clone(),
                source,
            })?;
        let table = doc.as_table();

        let provenance =
            table
                .get("provenance")
                .and_then(Item::as_table)
                .ok_or(SpecError::MissingField {
                    path: meta_path.clone(),
                    field: "provenance",
                })?;
        let source_path = PathBuf::from(required_str(provenance, "source_path", &meta_path)?);
        let import_hash = required_str(provenance, "import_hash", &meta_path)?;

        let body_path = dir.join("SPEC.md");
        let body = fs::read_to_string(&body_path).map_err(|source| SpecError::Io {
            path: body_path,
            source,
        })?;

        Ok(Self {
            name,
            body,
            provenance: Provenance {
                source_path,
                import_hash,
            },
        })
    }

    /// Project the typed header to a format-preserving `toml_edit` document. A spec
    /// has no frontmatter, so the surface header is provenance-only: a single
    /// `[provenance]` table.
    pub fn to_meta_document(&self) -> DocumentMut {
        let mut doc = DocumentMut::new();

        let mut provenance = Table::new();
        provenance["source_path"] =
            value(self.provenance.source_path.to_string_lossy().into_owned());
        provenance["import_hash"] = value(self.provenance.import_hash.clone());
        doc["provenance"] = Item::Table(provenance);

        doc
    }
}

/// Lowercase hex SHA-256 of `bytes`.
fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// A required string field of a TOML table.
fn required_str(table: &Table, field: &'static str, path: &Path) -> Result<String, SpecError> {
    table
        .get(field)
        .and_then(Item::as_str)
        .map(str::to_string)
        .ok_or(SpecError::MissingField {
            path: path.to_path_buf(),
            field,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-spec-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A spec body whose trailing bytes (no final newline) must survive intact.
    /// Note the `---` line: in a rule this would open frontmatter, but a spec has
    /// none, so it is ordinary body content and must be kept verbatim.
    const SPEC_BODY: &str = "# The config surface\n\
\n\
The surface is temper's composition write surface.\n\
\n\
---\n\
\n\
Trailing prose, no final newline.";

    fn write_source(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, body).unwrap();
        path
    }

    #[test]
    fn parses_name_from_stem_and_body_is_byte_faithful() {
        let dir = tmpdir("body");
        let path = write_source(&dir, "20-surface.md", SPEC_BODY);

        let spec = Spec::from_source_file(&path).unwrap();

        // Name is the file stem, not an internal field.
        assert_eq!(spec.name, "20-surface");
        // The whole file is the body, byte-for-byte — a leading `---` is prose
        // here (a spec has no frontmatter), and the missing final newline is kept.
        assert_eq!(spec.body, SPEC_BODY);
    }

    #[test]
    fn import_hash_is_stable_and_64_char_hex() {
        let dir = tmpdir("hash");
        let path = write_source(&dir, "90-spec-system.md", SPEC_BODY);

        let first = Spec::from_source_file(&path).unwrap();
        let second = Spec::from_source_file(&path).unwrap();

        assert_eq!(first.provenance.import_hash, second.provenance.import_hash);
        assert_eq!(first.provenance.import_hash.len(), 64);
        assert!(
            first
                .provenance
                .import_hash
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
        );
    }

    #[test]
    fn surface_round_trips_a_spec_exactly() {
        let src = tmpdir("rt-src");
        let path = write_source(&src, "20-surface.md", SPEC_BODY);
        let original = Spec::from_source_file(&path).unwrap();

        // The surface mirrors a skill/rule: a `<name>/` dir carries the
        // provenance-only header and the body alone. The reloaded name comes from
        // that directory name.
        let surface_root = tmpdir("rt-surface");
        let surface = surface_root.join("20-surface");
        fs::create_dir_all(&surface).unwrap();
        fs::write(
            surface.join("meta.toml"),
            original.to_meta_document().to_string(),
        )
        .unwrap();
        fs::write(surface.join("SPEC.md"), &original.body).unwrap();

        let reloaded = Spec::from_surface_dir(&surface).unwrap();

        assert_eq!(original, reloaded);
    }

    #[test]
    fn meta_document_is_deterministic_and_provenance_only() {
        let dir = tmpdir("meta");
        let path = write_source(&dir, "00-intent.md", SPEC_BODY);
        let spec = Spec::from_source_file(&path).unwrap();

        let once = spec.to_meta_document().to_string();
        let twice = spec.to_meta_document().to_string();
        assert_eq!(once, twice, "rendering must be deterministic");

        // A spec has no frontmatter, so the header is nothing but `[provenance]`.
        assert!(once.contains("[provenance]"));
        assert!(once.contains("source_path = "));
        assert!(once.contains("import_hash = "));
    }
}
