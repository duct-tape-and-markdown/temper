//! The extraction algebra — a custom kind's read side, composed from data.
//!
//! Models `specs/15-kinds.md` ("The extraction algebra — the soundness boundary,
//! as data"). Where `crate::contract` is the engine's *predicate* half (what an
//! artifact must **satisfy**), this is the *extraction* half (what an artifact
//! **is**, and how it is read):
//!
//! > predicates : contracts  ::  extraction : kinds
//!
//! Extraction is **the soundness boundary** — a clause is sound only if its
//! feature is *deterministically extractable*. A built-in harness kind keeps its
//! engine-code extractor (its format is external and evolving; `src/skill.rs`),
//! but a **custom** kind carries no code of its own: its extractor is
//! **composed from a closed algebra of deterministic extraction primitives**,
//! the identical mechanism that keeps the predicate algebra too weak to lie. The
//! closed vocabulary makes unsound extraction ("extract the meaning of paragraph
//! 3") **unsayable by construction** — an out-of-vocabulary primitive is a load
//! error, never a per-kind escape hatch (`specs/15-kinds.md`, "Decision:
//! extraction is a closed algebra, not author parsing").
//!
//! ## The vocabulary (harvested from the built-in kinds)
//!
//! Each primitive names a locus and yields one deterministic feature into
//! [`crate::extract::Features`]. The engine implements the primitive; the author
//! only composes:
//!
//! - **`field`** — a frontmatter value at a key, projected as a named field
//!   feature (kind-preserving, via the shared [`crate::extract`] projector);
//! - **`headings`** — the body's ATX headings;
//! - **`line_count`** — the body's line count (the `max_lines` feature);
//! - **`placement`** — the source directory the unit sits under;
//! - **`references`** — the backtick-filename references in the body (`` `NN-name.md` ``,
//!   the corpus's declared reference syntax; `specs/15-kinds.md`, "Worked
//!   example: `spec`"), as a named list feature a `references-resolve` clause or
//!   a declared edge (`crate::graph`) then reads.
//!
//! The `## Decision`-block primitive (heading + its body) waits on the
//! `(decision-marker-predicate)` fork; this tier ships the primitives the `spec`
//! kind's `max_lines`/references clauses need now.
//!
//! ## Why reuse `crate::extract`, never a second extractor
//!
//! Every primitive delegates to the *same* surface extractor the built-in
//! projectors use ([`crate::extract::body_headings`],
//! [`body_line_count`](crate::extract::body_line_count),
//! [`source_dir_name`](crate::extract::source_dir_name)). A custom kind that
//! composes `headings` reads the byte-identical ATX/fence logic a skill does —
//! there is no forked implementation to drift, so the soundness boundary is one
//! boundary, not two.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value as JsonValue;
use toml_edit::{DocumentMut, Item, Table};

use crate::compose::Edge;
use crate::document::Document;
use crate::extract::{self, FeatureValue, Features};

/// The built-in harness kind names temper ships an engine-code extractor for
/// (`specs/15-kinds.md`, "Two categories of kind — ownership, not mechanism"). A
/// `[kind.<name>]` registration in the assembly whose name is one of these is a
/// **built-in layer** (adopt-and-layer its shipped package); any other name
/// registers a **custom** kind, whose definition is authored under
/// `.temper/kinds/<name>/KIND.md` (`specs/40-composition.md`, "Decision: a custom
/// kind is an authored `.temper/` artifact, registered in the assembly").
pub const BUILTIN_KINDS: &[&str] = &["skill", "rule"];

/// The **file locus** a custom kind reads (`specs/40-composition.md`, "Registering a
/// custom kind"): the root directory its units live under, and the filename glob that
/// selects them. `import` discovers a custom kind's units by scanning `root` for files
/// matching `glob`; file placement is itself an extraction primitive, so the locus is
/// part of the authored definition, not external config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Governs {
    /// The root directory the kind's units sit under (`specs`, `docs/adr`), a path
    /// relative to the harness the assembly governs.
    pub root: String,
    /// The filename glob that selects the kind's units under `root` (`*.md`,
    /// `[0-9][0-9]-*.md`), stored verbatim.
    pub glob: String,
}

/// A custom kind's **authored definition**, loaded from `.temper/kinds/<name>/KIND.md`
/// (`specs/20-surface.md`, "Decision: a kind definition is `KIND.md` — one document,
/// same medium"; `specs/40-composition.md`, "Decision: a custom kind is an authored
/// `.temper/` artifact, registered in the assembly"). The `+++`-fenced header carries
/// the composed definition — the [`governs`](CustomKind::governs) locus, the composed
/// [`Extraction`], and the declared [`relationships`](CustomKind::relationships) — and
/// the body is the kind's own prose (what this class of artifact *is*), read by no
/// check.
///
/// A custom kind is **purely declare-side**: it carries **no clauses**. Its
/// require-side is a **package** bound in the assembly (`[kind.<name>] package =
/// "<name>"`), resolved under `.temper/packages/` exactly as a built-in kind binds its
/// shipped one — every kind refers to a declared package, uniformly.
///
/// Not `Eq` — its [`extraction`](CustomKind::extraction) is `Eq`, but keeping the
/// derive `PartialEq` leaves room for future `f64`-bearing fields without a churn, as
/// it does for [`Clause`](crate::contract::Clause).
#[derive(Debug, Clone, PartialEq)]
pub struct CustomKind {
    /// The kind's name — the `[kind.<name>]` registration key and the
    /// `<name>` directory its `KIND.md` lives under.
    pub name: String,
    /// The file locus the kind reads.
    pub governs: Governs,
    /// The composed extractor over the closed algebra (`specs/15-kinds.md`), parsed
    /// from the header's `[[extraction]]` array by the shared [`Extraction::from_table`]
    /// — so an out-of-vocabulary primitive is rejected at load exactly as it is for a
    /// standalone extraction declaration. Absent ⇒ the vacuous extractor (only the
    /// intrinsic id).
    pub extraction: Extraction,
    /// The declared **relationships** — which of the kind's references are edges
    /// (`specs/15-kinds.md`, "The entity graph is a kind capability"), each an
    /// [`Edge`] whose `from` is this kind. Parsed from the header's
    /// `[[relationships]]` array. Absent ⇒ empty (the kind declares no edges).
    pub relationships: Vec<Edge>,
}

impl CustomKind {
    /// Load a custom kind's authored definition from `<kinds_dir>/<name>/KIND.md` — the
    /// one home for a project's own artifact kind (`specs/20-surface.md`). A missing
    /// document is a [`KindError::MissingDefinition`]: the assembly registered the kind,
    /// so its definition is required, never silently skipped. A malformed fenced
    /// document, an out-of-vocabulary extraction primitive, a missing/mistyped
    /// `governs`, or a stray header key each surface as a precise [`KindError`].
    pub fn load(kinds_dir: &Path, name: &str) -> Result<Self, KindError> {
        let path = kinds_dir.join(name).join("KIND.md");
        let raw = match std::fs::read_to_string(&path) {
            Ok(raw) => raw,
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(KindError::MissingDefinition {
                    path,
                    kind: name.to_string(),
                });
            }
            Err(source) => return Err(KindError::Io { path, source }),
        };
        let document = Document::parse(&raw).map_err(|source| KindError::Document {
            path: path.clone(),
            source,
        })?;
        Self::from_header(document.header().as_table(), name, &path)
    }

    /// Parse a custom kind's definition off a `KIND.md` header table — the composed
    /// [`governs`](CustomKind::governs) locus, the `[[extraction]]` extractor (via the
    /// shared [`Extraction::from_table`]), and the `[[relationships]]` edges. The seam
    /// tests drive without touching disk. A header carries only `governs`,
    /// `extraction`, and `relationships`; a stray key — a leftover `clause` (a custom
    /// kind carries no clauses; its contract is the bound package), an `entities` table
    /// (nodes derive from `features.id`), or a typo — is a [`KindError::UnknownKey`],
    /// rejected rather than silently dropped (`specs/10-contracts.md`, "Decision:
    /// unknown keys are rejected, not ignored").
    pub fn from_header(table: &Table, name: &str, path: &Path) -> Result<Self, KindError> {
        for (key, _) in table.iter() {
            if !matches!(key, "governs" | "extraction" | "relationships") {
                return Err(KindError::UnknownKey {
                    path: path.to_path_buf(),
                    kind: name.to_string(),
                    key: key.to_string(),
                });
            }
        }
        let governs = parse_governs(table, name, path)?;
        let extraction = Extraction::from_table(table, path)?;
        let relationships = parse_relationships(table, name, path)?;
        Ok(Self {
            name: name.to_string(),
            governs,
            extraction,
            relationships,
        })
    }
}

/// Parse a `KIND.md` header's required `governs = { root, glob }` locus: absent ⇒
/// [`KindError::MissingGoverns`] (a custom kind that reads no files is meaningless);
/// not a table, or a missing/mistyped `root`/`glob` string ⇒ [`KindError::BadGoverns`],
/// folding every malformation into one error.
fn parse_governs(table: &Table, kind: &str, path: &Path) -> Result<Governs, KindError> {
    let item = table
        .get("governs")
        .ok_or_else(|| KindError::MissingGoverns {
            path: path.to_path_buf(),
            kind: kind.to_string(),
        })?;
    let bad = || KindError::BadGoverns {
        path: path.to_path_buf(),
        kind: kind.to_string(),
    };
    let governs = item.as_table_like().ok_or_else(bad)?;
    let root = governs
        .get("root")
        .and_then(Item::as_str)
        .ok_or_else(bad)?
        .to_string();
    let glob = governs
        .get("glob")
        .and_then(Item::as_str)
        .ok_or_else(bad)?
        .to_string();
    Ok(Governs { root, glob })
}

/// Parse a `KIND.md` header's `[[relationships]]` array into typed [`Edge`]s, in
/// declaration order — a reference is a **kind capability** declared under its owning
/// kind (`specs/15-kinds.md`, "The entity graph is a kind capability"). The owning
/// `kind` is each edge's source (the implicit `from`); each relationship names its
/// reference `field` and target `to` kind, both strings. Absent ⇒ an empty vec; not an
/// array-of-tables ⇒ [`KindError::RelationshipsNotArray`]; a missing/mistyped `field`
/// or `to` ⇒ a single folded [`KindError::BadRelationship`] naming its position.
fn parse_relationships(table: &Table, kind: &str, path: &Path) -> Result<Vec<Edge>, KindError> {
    let Some(item) = table.get("relationships") else {
        return Ok(Vec::new());
    };
    let array = item
        .as_array_of_tables()
        .ok_or_else(|| KindError::RelationshipsNotArray {
            path: path.to_path_buf(),
            kind: kind.to_string(),
        })?;
    let mut edges = Vec::with_capacity(array.len());
    for (index, relationship) in array.iter().enumerate() {
        let bad = || KindError::BadRelationship {
            path: path.to_path_buf(),
            kind: kind.to_string(),
            index,
        };
        let field = relationship
            .get("field")
            .and_then(Item::as_str)
            .ok_or_else(bad)?
            .to_string();
        let to = relationship
            .get("to")
            .and_then(Item::as_str)
            .ok_or_else(bad)?
            .to_string();
        edges.push(Edge {
            field,
            from: kind.to_string(),
            to,
        });
    }
    Ok(edges)
}

/// A custom kind's composed extractor: an ordered set of deterministic
/// [`Primitive`]s over the closed algebra. Run over a [`Unit`] with
/// [`Extraction::extract`] it yields the [`Features`] a contract validates —
/// re-running over the same unit is byte-identical, because every primitive is a
/// pure function of the surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extraction {
    /// The composed primitives, in declaration order. An empty set is a valid
    /// (vacuous) extractor — it yields only the intrinsic `id`, everything else
    /// at its default (no fields, zero lines, no headings, no placement).
    primitives: Vec<Primitive>,
}

/// A single extraction primitive from the closed vocabulary. Each names a locus
/// on the surface and the feature it yields — every one *deterministically
/// extractable*, so a clause over its feature is a true positive
/// (`specs/15-kinds.md`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    /// `field` — project the frontmatter value at `key` into the named field
    /// feature (kind-preserving). Absent from the unit ⇒ the feature is not
    /// yielded, mirroring how a skill's optional `version` is omitted when unset.
    Field {
        /// The frontmatter key-path read, and the name the feature is keyed by.
        key: String,
    },
    /// `headings` — the body's ATX headings, in document order
    /// (`Features::headings`).
    Headings,
    /// `line_count` — the body's line count (`Features::body_lines`), the
    /// `max_lines` feature.
    LineCount,
    /// `placement` — the name of the directory the unit sits under
    /// (`Features::source_dir`) — file placement.
    Placement,
    /// `references` — the backtick-filename references in the body, as a list
    /// feature named `feature`. The corpus's declared reference syntax
    /// (`` `NN-name.md` ``), decided at the surface — never grepped prose meaning.
    References {
        /// The name the yielded reference-list feature is keyed by.
        feature: String,
    },
}

impl Primitive {
    /// This primitive's TOML `primitive` discriminator — the key it is parsed
    /// from, reused as the vocabulary name a diagnostic reports.
    #[must_use]
    pub fn key(&self) -> &'static str {
        match self {
            Primitive::Field { .. } => "field",
            Primitive::Headings => "headings",
            Primitive::LineCount => "line_count",
            Primitive::Placement => "placement",
            Primitive::References { .. } => "references",
        }
    }

    /// Apply this primitive to `unit`, folding its one feature into `features`.
    /// Deterministic and side-effect-free over the surface, so the composed
    /// extractor is too.
    fn apply(&self, unit: &Unit, features: &mut Features) {
        match self {
            Primitive::Field { key } => {
                if let Some(value) = unit.frontmatter.get(key) {
                    features
                        .fields
                        .insert(key.clone(), extract::json_to_feature(value));
                }
            }
            Primitive::Headings => features.headings = extract::body_headings(&unit.body),
            Primitive::LineCount => features.body_lines = extract::body_line_count(&unit.body),
            Primitive::Placement => {
                features.source_dir = extract::source_dir_name(&unit.source_path)
            }
            Primitive::References { feature } => {
                let refs = backtick_filename_refs(&unit.body);
                features
                    .fields
                    .insert(feature.clone(), FeatureValue::List(refs));
            }
        }
    }
}

/// A raw markdown unit the composed extractor reads: the intrinsic identity plus
/// the three surface loci the primitives range over (parsed frontmatter, the
/// byte-faithful body, the source placement). Frontmatter is *already parsed* —
/// splitting it is the surface tier's job and varies per harness format
/// (`crate::skill` vs a frontmatter-less spec), so this composer takes the values
/// rather than re-parse. A spec supplies an empty `frontmatter`.
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    /// The artifact id used in diagnostics and as `Features::id` (a file stem, a
    /// skill's `name`). Intrinsic to the unit, never a composed primitive.
    pub id: String,
    /// The parsed frontmatter values by key — the `field` primitive's locus.
    /// Empty for a frontmatter-less kind (a spec, `90-spec-system.md`).
    pub frontmatter: BTreeMap<String, JsonValue>,
    /// The byte-faithful markdown body (frontmatter stripped) — the locus for
    /// `headings`, `line_count`, and `references`.
    pub body: String,
    /// The source path the unit was read from — the `placement` locus.
    pub source_path: PathBuf,
}

impl Unit {
    /// Reload a written custom-unit surface `<root>/<name>/` into a raw [`Unit`]:
    /// the id is the surface directory name, and its lone `.md` sibling is the member
    /// **document** (`specs/20-surface.md`, "Decision: the member is one document") —
    /// a `+++`-fenced header carrying `[provenance]` over the byte-faithful body. The
    /// body is everything below the header, and `source_path` is read back from the
    /// `[provenance]` module `import` wrote (`src/import.rs`, `import_custom_unit`).
    ///
    /// This is the **generic** inverse of that projection — keyed on the surface
    /// shape every custom kind shares (a lone member document), not on any one kind's
    /// IR. It is the sole reader `check`'s custom-kind path uses — there is no bespoke
    /// per-kind loader (the built-in `spec` IR that once had one is retired) — so a
    /// custom kind rooted at *any* `governs.root`, not just `specs/`, is read
    /// (`specs/40-composition.md`, "Declaring a custom kind").
    ///
    /// The document is found by extension, not by the kind's own upper-cased name, so
    /// one reader serves every custom kind. Frontmatter is left empty: a custom unit's
    /// document preserves the *whole* source file (frontmatter included) in its body,
    /// and the composed extractor reads the loci it declares off that body
    /// (`specs/15-kinds.md`). An unreadable or malformed surface is a [`KindError`],
    /// never a silent skip.
    pub fn from_surface_dir(dir: &Path) -> Result<Self, KindError> {
        let id = dir
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_string)
            .ok_or_else(|| KindError::SurfaceMissingField {
                path: dir.to_path_buf(),
                field: "name",
            })?;

        let doc_path = lone_body_file(dir)?;
        let raw = std::fs::read_to_string(&doc_path).map_err(|source| KindError::Io {
            path: doc_path.clone(),
            source,
        })?;
        let document = Document::parse(&raw).map_err(|source| KindError::Document {
            path: doc_path.clone(),
            source,
        })?;
        let (source_path, _import_hash) = crate::document::provenance(document.header())
            .ok_or_else(|| KindError::SurfaceMissingField {
                path: doc_path.clone(),
                field: "provenance",
            })?;

        Ok(Self {
            id,
            frontmatter: BTreeMap::new(),
            body: document.body().to_string(),
            source_path: PathBuf::from(source_path),
        })
    }
}

/// The lone `.md` member document in a custom-unit surface directory — the
/// `+++`-fenced document `import` writes (`<KIND>.md`; `src/import.rs`). Selected by
/// extension rather than by the kind's own upper-cased name, so the reader stays
/// generic over every custom kind. Exactly one is required: zero (no document) or
/// more than one (an ambiguous surface) is a [`KindError::SurfaceBody`].
fn lone_body_file(dir: &Path) -> Result<PathBuf, KindError> {
    let listing = std::fs::read_dir(dir).map_err(|source| KindError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    let mut bodies = Vec::new();
    for entry in listing {
        let entry = entry.map_err(|source| KindError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            bodies.push(path);
        }
    }
    match bodies.len() {
        1 => Ok(bodies.remove(0)),
        found => Err(KindError::SurfaceBody {
            dir: dir.to_path_buf(),
            found,
        }),
    }
}

/// Errors raised while loading an [`Extraction`]. Hard failures (unreadable file,
/// malformed TOML, a primitive outside the closed vocabulary) — distinct from a
/// lint finding, which the check engine collects rather than throws. Mirrors
/// [`crate::contract::ContractError`]: the closed vocabulary is guarded at load
/// the same way on both sides of the boundary.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum KindError {
    /// The extraction file could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::kind::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The extraction file is not valid TOML.
    #[error("failed to parse {path} as TOML")]
    #[diagnostic(code(temper::kind::toml))]
    Toml {
        /// The declaration that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
    },

    /// A written custom-unit surface's member document is not a well-formed
    /// `+++`-fenced document (missing or unterminated fence, or a malformed TOML
    /// header). Reloading parses the document `import` wrote, so a malformed one is a
    /// hard error, never a silent skip.
    #[error("{path}: {source}")]
    #[diagnostic(code(temper::kind::bad_document))]
    Document {
        /// The surface document that failed to parse.
        path: PathBuf,
        /// The underlying fenced-document parse error.
        #[source]
        source: crate::document::DocumentError,
    },

    /// `extraction` is present but is not an array of tables (`[[extraction]]`).
    #[error("{path}: `extraction` must be an array of tables (`[[extraction]]`)")]
    #[diagnostic(code(temper::kind::extraction_not_array))]
    ExtractionNotArray {
        /// The malformed declaration.
        path: PathBuf,
    },

    /// A primitive is missing a key it requires.
    #[error("{path}: extraction primitive {index} is missing required key `{param}`")]
    #[diagnostic(code(temper::kind::missing_param))]
    MissingParam {
        /// The declaration the primitive lives in.
        path: PathBuf,
        /// The zero-based primitive index.
        index: usize,
        /// The absent key.
        param: &'static str,
    },

    /// A primitive key has the wrong TOML type.
    #[error("{path}: extraction primitive {index} key `{param}` must be {expected}")]
    #[diagnostic(code(temper::kind::wrong_type))]
    WrongType {
        /// The declaration the primitive lives in.
        path: PathBuf,
        /// The zero-based primitive index.
        index: usize,
        /// The mistyped key.
        param: &'static str,
        /// The type that was expected, for the message.
        expected: &'static str,
    },

    /// A primitive names an extractor outside the closed vocabulary. This is the
    /// trapdoor the closed algebra exists to keep shut — an unsound extractor is
    /// *unsayable*, so it is rejected at load, never skipped (`specs/15-kinds.md`,
    /// "Decision: extraction is a closed algebra, not author parsing").
    #[error("{path}: extraction primitive {index} names unknown extractor `{primitive}`")]
    #[diagnostic(
        code(temper::kind::unknown_primitive),
        help(
            "extraction is a closed algebra, not an escape hatch — a missing primitive is a deliberate vocabulary addition, never a per-kind hatch"
        )
    )]
    UnknownPrimitive {
        /// The declaration the primitive lives in.
        path: PathBuf,
        /// The zero-based primitive index.
        index: usize,
        /// The unrecognized extractor key.
        primitive: String,
    },

    /// A written custom-unit surface is missing a required part — its directory
    /// name, its `[provenance]` table, or the `source_path` inside that table.
    /// Reloading is the inverse of the projection `import` writes, so a surface
    /// missing what `import` always writes is malformed, never a silent skip.
    #[error("{path}: custom-unit surface is missing required field `{field}`")]
    #[diagnostic(code(temper::kind::surface_missing_field))]
    SurfaceMissingField {
        /// The surface (its directory, or its `meta.toml`) whose part is absent.
        path: PathBuf,
        /// The required field that was absent.
        field: &'static str,
    },

    /// A written custom-unit surface does not carry exactly one `.md` member document
    /// — the `+++`-fenced document the extractor reads (`src/import.rs`,
    /// `import_custom_unit`). Zero (no document) or more than one (an ambiguous
    /// surface) is malformed.
    #[error(
        "{dir}: custom-unit surface must carry exactly one `.md` member document (found {found})"
    )]
    #[diagnostic(code(temper::kind::surface_body))]
    SurfaceBody {
        /// The surface directory whose body is missing or ambiguous.
        dir: PathBuf,
        /// How many `.md` bodies were found (never exactly one).
        found: usize,
    },

    /// The assembly registered a custom kind but its authored definition
    /// `.temper/kinds/<name>/KIND.md` is absent. A registration promises a definition
    /// (`specs/40-composition.md`, "Registering a custom kind"), so a missing one is a
    /// hard error, never a silent skip.
    #[error("{path}: custom kind `{kind}` is registered but its `KIND.md` definition is missing")]
    #[diagnostic(
        code(temper::kind::missing_definition),
        help(
            "author the kind's definition at `.temper/kinds/<name>/KIND.md`, or drop its `[kind.<name>]` registration"
        )
    )]
    MissingDefinition {
        /// The `KIND.md` path that was expected.
        path: PathBuf,
        /// The registered kind whose definition is absent.
        kind: String,
    },

    /// A `KIND.md` header names no `governs` locus — a custom kind that reads no files
    /// is meaningless, so the locus is required (`specs/40-composition.md`).
    #[error("{path}: custom kind `{kind}` is missing required key `governs`")]
    #[diagnostic(
        code(temper::kind::missing_governs),
        help("a custom kind must declare the file locus it reads — a `governs` root and glob")
    )]
    MissingGoverns {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The custom kind missing its locus.
        kind: String,
    },

    /// A `KIND.md` header's `governs` locus is malformed — not a table, or
    /// missing/mistyped one of its `root` and `glob` strings. The locus is a root
    /// directory plus a filename glob; any miss collapses into this one error.
    #[error(
        "{path}: custom kind `{kind}` `governs` must be a table with `root` and `glob` string keys"
    )]
    #[diagnostic(code(temper::kind::bad_governs))]
    BadGoverns {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The custom kind with the malformed locus.
        kind: String,
    },

    /// A `KIND.md` header carries a key outside its closed set (`governs`,
    /// `extraction`, `relationships`). A leftover `clause` (a custom kind carries no
    /// clauses; its contract is the bound package), an `entities` table (nodes derive
    /// from `features.id`), or a typo is rejected at load rather than silently dropped
    /// (`specs/10-contracts.md`, "Decision: unknown keys are rejected, not ignored").
    #[error("{path}: custom kind `{kind}` definition has unknown key `{key}`")]
    #[diagnostic(
        code(temper::kind::unknown_key),
        help(
            "a `KIND.md` definition carries only `governs`, `extraction`, and `relationships` — a custom kind carries no clauses (its contract is the bound package), and there is no `entities` table"
        )
    )]
    UnknownKey {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The custom kind whose definition carries the stray key.
        kind: String,
        /// The unrecognized key.
        key: String,
    },

    /// A `KIND.md` header's `relationships` key is present but is not an array of
    /// `[[relationships]]` reference tables.
    #[error(
        "{path}: custom kind `{kind}` `relationships` must be an array of `[[relationships]]` reference tables"
    )]
    #[diagnostic(code(temper::kind::relationships_not_array))]
    RelationshipsNotArray {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The kind whose relationships array is malformed.
        kind: String,
    },

    /// A `[[relationships]]` declaration is malformed — missing or mistyped one of its
    /// `field`, `to` strings. A declared relationship names a reference field and a
    /// target kind (its owning kind is the source); any miss collapses into this one
    /// error naming its position.
    #[error(
        "{path}: custom kind `{kind}` `[[relationships]]` #{index} must name a reference `field` and a `to` kind, both strings"
    )]
    #[diagnostic(code(temper::kind::bad_relationship))]
    BadRelationship {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The kind that owns the malformed relationship.
        kind: String,
        /// The zero-based position of the malformed relationship in declaration order.
        index: usize,
    },
}

impl Extraction {
    /// Load and parse a kind's extraction declaration from a TOML file on disk.
    pub fn load(path: &Path) -> Result<Self, KindError> {
        let src = std::fs::read_to_string(path).map_err(|source| KindError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        Self::parse(&src, path)
    }

    /// Parse an extraction declaration from TOML source, reading the
    /// `[[extraction]]` array of primitive tables off the root. `path` labels
    /// diagnostics, so this is the seam tests drive without touching disk.
    pub fn parse(src: &str, path: &Path) -> Result<Self, KindError> {
        let doc = src
            .parse::<DocumentMut>()
            .map_err(|source| KindError::Toml {
                path: path.to_path_buf(),
                source,
            })?;
        Self::from_table(doc.as_table(), path)
    }

    /// Parse the composed extractor off a table carrying an `[[extraction]]`
    /// array — the `[kind.<name>.extraction]` declaration (`specs/15-kinds.md`).
    /// Public so the author-composition tier (`crate::compose`) can compose a
    /// custom kind's extractor off a `[kind.<name>]` table without duplicating
    /// this closed-vocabulary parser, exactly as it reuses
    /// [`crate::contract::parse_clauses`] for the predicate side.
    pub fn from_table(table: &Table, path: &Path) -> Result<Self, KindError> {
        let array = match table.get("extraction") {
            None => {
                return Ok(Self {
                    primitives: Vec::new(),
                });
            }
            Some(Item::ArrayOfTables(array)) => array,
            Some(_) => {
                return Err(KindError::ExtractionNotArray {
                    path: path.to_path_buf(),
                });
            }
        };

        let mut primitives = Vec::with_capacity(array.len());
        for (index, primitive) in array.iter().enumerate() {
            primitives.push(parse_primitive(primitive, index, path)?);
        }
        Ok(Self { primitives })
    }

    /// The composed primitives, in declaration order.
    #[must_use]
    pub fn primitives(&self) -> &[Primitive] {
        &self.primitives
    }

    /// Run the composed extractor over a raw markdown `unit`, folding each
    /// primitive's one feature into a [`Features`]. The intrinsic `id` is always
    /// set; every other feature stays at its default until a primitive yields it.
    /// A pure function of the surface — re-running over the same unit is
    /// byte-identical, which is what makes the feature a sound gate input.
    #[must_use]
    pub fn extract(&self, unit: &Unit) -> Features {
        let mut features = Features {
            id: unit.id.clone(),
            fields: BTreeMap::new(),
            body_lines: 0,
            headings: Vec::new(),
            source_dir: None,
            // Custom-kind `satisfies` is a later follow-on: no extraction
            // primitive yields it yet, so a composed Unit's representation
            // bindings stay empty until that lands.
            satisfies: Vec::new(),
        };
        for primitive in &self.primitives {
            primitive.apply(unit, &mut features);
        }
        features
    }
}

/// Parse one `[[extraction]]` table into its typed [`Primitive`], pulling each
/// primitive's own parameters. A discriminator outside the closed vocabulary is
/// rejected, never skipped.
fn parse_primitive(table: &Table, index: usize, path: &Path) -> Result<Primitive, KindError> {
    let kind = str_param(table, "primitive", index, path)?;
    let primitive = match kind.as_str() {
        "field" => Primitive::Field {
            key: str_param(table, "key", index, path)?,
        },
        "headings" => Primitive::Headings,
        "line_count" => Primitive::LineCount,
        "placement" => Primitive::Placement,
        "references" => Primitive::References {
            feature: str_param(table, "feature", index, path)?,
        },
        other => {
            return Err(KindError::UnknownPrimitive {
                path: path.to_path_buf(),
                index,
                primitive: other.to_string(),
            });
        }
    };
    Ok(primitive)
}

/// Read a required string primitive key.
fn str_param(
    table: &Table,
    key: &'static str,
    index: usize,
    path: &Path,
) -> Result<String, KindError> {
    match table.get(key) {
        None => Err(KindError::MissingParam {
            path: path.to_path_buf(),
            index,
            param: key,
        }),
        Some(item) => item
            .as_str()
            .map(str::to_string)
            .ok_or(KindError::WrongType {
                path: path.to_path_buf(),
                index,
                param: key,
                expected: "a string",
            }),
    }
}

/// The backtick-filename references in a byte-faithful markdown body, in document
/// order — the corpus's declared reference syntax (`` `NN-name.md` ``,
/// `specs/15-kinds.md`). A reference is an inline single-backtick span whose
/// content is *filename-shaped* (see [`is_filename_reference`]); a span inside a
/// fenced code block is illustration, not a declared reference, so it is skipped
/// exactly as heading extraction skips fenced `#`. Prose meaning is never read —
/// only the surface-decidable span.
fn backtick_filename_refs(body: &str) -> Vec<String> {
    // A static literal regex: `Regex::new` over a fixed valid pattern is a
    // genuine invariant that cannot fail (`.claude/rules/rust.md`).
    static SPAN: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"`([^`\n]+)`").expect("the inline-code span regex is valid"));

    let mut refs = Vec::new();
    // The open fence's char and run length, while inside a fenced code block —
    // the same tracking `crate::extract::body_headings` uses.
    let mut fence: Option<(char, usize)> = None;
    for line in body.lines() {
        if let Some((fence_char, fence_len)) = extract::fence_marker(line) {
            match fence {
                Some((open_char, open_len)) if fence_char == open_char && fence_len >= open_len => {
                    fence = None;
                }
                Some(_) => {}
                None => fence = Some((fence_char, fence_len)),
            }
            continue;
        }
        if fence.is_some() {
            continue;
        }
        for capture in SPAN.captures_iter(line) {
            let content = &capture[1];
            if is_filename_reference(content) {
                refs.push(content.to_string());
            }
        }
    }
    refs
}

/// Whether an inline-code span's content is filename-shaped — the decidable rule
/// that separates a declared reference (`` `15-kinds.md` ``, `` `src/skill.rs` ``)
/// from ordinary inline code (`` `Features` ``, `` `min_len` ``) or a version
/// (`` `1.2.0` ``): no internal whitespace, and a final `.<ext>` whose extension
/// is alphanumeric and carries at least one letter. The letter requirement is
/// what rejects a numeric version segment, keeping the reference set free of
/// false positives that would dangle at resolution.
fn is_filename_reference(span: &str) -> bool {
    if span.chars().any(char::is_whitespace) {
        return false;
    }
    match span.rsplit_once('.') {
        Some((stem, ext)) => {
            !stem.is_empty()
                && !ext.is_empty()
                && ext.chars().all(|c| c.is_ascii_alphanumeric())
                && ext.chars().any(|c| c.is_ascii_alphabetic())
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::Kind;

    /// The composed `spec`-shaped extractor the worked example needs now
    /// (`specs/15-kinds.md`): line count, ATX headings, file placement, and
    /// backtick-filename references — every primitive the `max_lines`/references
    /// clauses read.
    fn spec_extraction() -> Extraction {
        let toml = r#"
[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "placement"

[[extraction]]
primitive = "references"
feature = "references"
"#;
        Extraction::parse(toml, Path::new("temper.toml")).unwrap()
    }

    /// A raw spec-shaped unit: no frontmatter, a body carrying two headings, two
    /// backtick-filename references, non-reference inline code, a version, and a
    /// filename inside a fenced block (which must be skipped).
    fn spec_unit() -> Unit {
        let body = "# Kinds\n\
\n\
## The extraction algebra\n\
\n\
Composed like `15-kinds.md` over `10-contracts.md`. Not refs: `Features`, `min_len`, or version `1.2.0`.\n\
\n\
```text\n\
`inside-a-fence.md` is illustration, not a reference\n\
```\n";
        Unit {
            id: "15-kinds".to_string(),
            frontmatter: BTreeMap::new(),
            body: body.to_string(),
            source_path: PathBuf::from("specs/15-kinds.md"),
        }
    }

    #[test]
    fn composes_and_extracts_a_raw_unit_into_features() {
        let extraction = spec_extraction();
        let features = extraction.extract(&spec_unit());

        // The intrinsic id is always the unit id.
        assert_eq!(features.id, "15-kinds");

        // `line_count` — the whole body, counted the same way a spec projector does.
        assert_eq!(features.body_lines, 9);

        // `headings` — ATX headings in order (the fenced content is not a heading).
        assert_eq!(
            features.headings,
            vec!["Kinds".to_string(), "The extraction algebra".to_string()]
        );

        // `placement` — the folder the unit sits under.
        assert_eq!(features.source_dir.as_deref(), Some("specs"));

        // `references` — the backtick-filename spans only: the two real filenames,
        // never `Features`/`min_len` (no extension), the version `1.2.0` (numeric
        // extension), or the filename inside the fenced block (skipped).
        assert_eq!(
            features.field("references"),
            Some(&FeatureValue::List(vec![
                "15-kinds.md".to_string(),
                "10-contracts.md".to_string(),
            ]))
        );

        // A frontmatter-less kind composes no `field`, so nothing else is yielded.
        assert_eq!(features.fields.len(), 1);
    }

    #[test]
    fn re_running_the_extractor_is_byte_identical() {
        let extraction = spec_extraction();
        let unit = spec_unit();

        // Extraction is a pure function of the surface — the soundness boundary
        // (`specs/15-kinds.md`): the same unit yields the same features every run.
        let first = extraction.extract(&unit);
        let second = extraction.extract(&unit);
        assert_eq!(first, second);
    }

    #[test]
    fn a_field_primitive_projects_frontmatter_kind_preserving() {
        let toml = r#"
[[extraction]]
primitive = "field"
key = "name"

[[extraction]]
primitive = "field"
key = "priority"
"#;
        let extraction = Extraction::parse(toml, Path::new("temper.toml")).unwrap();

        let mut frontmatter = BTreeMap::new();
        frontmatter.insert("name".to_string(), JsonValue::String("demo".to_string()));
        frontmatter.insert("priority".to_string(), JsonValue::from(7));
        let unit = Unit {
            id: "demo".to_string(),
            frontmatter,
            body: "# Demo\n".to_string(),
            source_path: PathBuf::from("skills/demo/SKILL.md"),
        };

        let features = extraction.extract(&unit);

        // The frontmatter value is projected through the shared kind-preserving
        // projector: a string stays `string`, an integer keeps `integer`.
        assert_eq!(
            features.field("name"),
            Some(&FeatureValue::scalar(Kind::String, "demo"))
        );
        assert_eq!(
            features.field("priority").map(FeatureValue::kind),
            Some(Kind::Integer)
        );
        // The body loci are untouched — this extractor composes only `field`.
        assert_eq!(features.body_lines, 0);
        assert!(features.headings.is_empty());
    }

    #[test]
    fn a_field_absent_from_the_unit_is_not_yielded() {
        let toml = "[[extraction]]\nprimitive = \"field\"\nkey = \"license\"\n";
        let extraction = Extraction::parse(toml, Path::new("temper.toml")).unwrap();
        let unit = Unit {
            id: "demo".to_string(),
            frontmatter: BTreeMap::new(),
            body: String::new(),
            source_path: PathBuf::from("skills/demo/SKILL.md"),
        };
        // A key the unit does not carry yields no feature — never a phantom entry.
        assert!(extraction.extract(&unit).field("license").is_none());
    }

    #[test]
    fn an_empty_declaration_is_a_vacuous_extractor() {
        let extraction = Extraction::parse("# nothing\n", Path::new("temper.toml")).unwrap();
        assert!(extraction.primitives().is_empty());

        let unit = spec_unit();
        let features = extraction.extract(&unit);
        // Only the intrinsic id; every composed feature stays at its default.
        assert_eq!(features.id, "15-kinds");
        assert_eq!(features.body_lines, 0);
        assert!(features.headings.is_empty());
        assert!(features.source_dir.is_none());
        assert!(features.fields.is_empty());
    }

    #[test]
    fn an_unknown_primitive_is_a_load_error_not_a_silent_skip() {
        // The closed vocabulary makes an unsound extractor unsayable: naming one
        // outside it is rejected at load, exactly as an unknown predicate key is.
        let toml = r#"
[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "paragraph_meaning"
"#;
        let err = Extraction::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            KindError::UnknownPrimitive { ref primitive, index: 1, .. }
                if primitive == "paragraph_meaning"
        ));
    }

    #[test]
    fn a_primitive_missing_its_parameter_is_a_load_error() {
        // `references` names the feature it yields; omitting it is a load error.
        let toml = "[[extraction]]\nprimitive = \"references\"\n";
        let err = Extraction::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            KindError::MissingParam {
                param: "feature",
                index: 0,
                ..
            }
        ));
    }

    #[test]
    fn a_mistyped_parameter_is_a_load_error() {
        let toml = "[[extraction]]\nprimitive = \"field\"\nkey = 7\n";
        let err = Extraction::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            KindError::WrongType {
                param: "key",
                index: 0,
                ..
            }
        ));
    }

    #[test]
    fn a_non_array_extraction_key_is_a_load_error() {
        let err = Extraction::parse("extraction = 7\n", Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, KindError::ExtractionNotArray { .. }));
    }

    #[test]
    fn load_reads_a_declaration_from_disk() {
        let dir = std::env::temp_dir().join(format!("author-kind-load-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("spec.extraction.toml");
        std::fs::write(&path, "[[extraction]]\nprimitive = \"line_count\"\n").unwrap();

        let extraction = Extraction::load(&path).unwrap();
        assert_eq!(extraction.primitives(), &[Primitive::LineCount]);
    }

    /// A fresh, empty temp directory unique to this call.
    fn surface_tmpdir(label: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-kind-surface-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Write a `<root>/<name>/<BODY>.md` surface exactly as `import` projects a
    /// custom-kind unit: ONE member document — a provenance-only `+++` header over
    /// the whole body. Returns the surface directory.
    fn write_surface(
        root: &Path,
        name: &str,
        source_path: &str,
        body_name: &str,
        body: &str,
    ) -> PathBuf {
        let dir = root.join(name);
        std::fs::create_dir_all(&dir).unwrap();
        let document = format!(
            "+++\n[provenance]\nsource_path = \"{source_path}\"\nimport_hash = \"deadbeef\"\n+++\n{body}"
        );
        std::fs::write(dir.join(body_name), document).unwrap();
        dir
    }

    #[test]
    fn from_surface_dir_reloads_a_written_unit_for_any_root() {
        // The root is `docs/adr`, not `specs` — the reader keys on the surface
        // shape, never a hardwired `specs` special case, so a kind rooted anywhere
        // reloads the same way.
        let root = surface_tmpdir("adr-root").join("docs").join("adr");
        let body = "# ADR 0001\n\nContext refers to `15-kinds.md`.\n";
        let dir = write_surface(
            &root,
            "0001-use-kinds",
            "docs/adr/0001-use-kinds.md",
            "ADR.md",
            body,
        );

        let unit = Unit::from_surface_dir(&dir).unwrap();

        // id is the surface directory name.
        assert_eq!(unit.id, "0001-use-kinds");
        // body is the lone `.md` sibling, byte-faithful.
        assert_eq!(unit.body, body);
        // source_path is read back from the `[provenance]` table.
        assert_eq!(
            unit.source_path,
            PathBuf::from("docs/adr/0001-use-kinds.md")
        );
        // A generic surface reload carries no frontmatter — the whole file is body.
        assert!(unit.frontmatter.is_empty());
    }

    #[test]
    fn from_surface_dir_feeds_the_composed_extractor() {
        // The reloaded unit is exactly what a kind's composed extractor reads: the
        // spec-shaped extractor over it yields the same features it would over a
        // freshly-parsed unit — the tie between the generic loader and the check path.
        let root = surface_tmpdir("feed-root").join("specs");
        let body = "# Kinds\n\nComposed over `10-contracts.md`.\n";
        let dir = write_surface(&root, "15-kinds", "specs/15-kinds.md", "SPEC.md", body);

        let unit = Unit::from_surface_dir(&dir).unwrap();
        let features = spec_extraction().extract(&unit);

        assert_eq!(features.id, "15-kinds");
        assert_eq!(features.body_lines, 3);
        assert_eq!(features.headings, vec!["Kinds".to_string()]);
        assert_eq!(features.source_dir.as_deref(), Some("specs"));
        assert_eq!(
            features.field("references"),
            Some(&FeatureValue::List(vec!["10-contracts.md".to_string()]))
        );
    }

    #[test]
    fn a_surface_missing_its_provenance_is_a_load_error() {
        let root = surface_tmpdir("no-prov");
        let dir = root.join("00-intent");
        std::fs::create_dir_all(&dir).unwrap();
        // A member document whose header carries no `[provenance]` module.
        std::fs::write(dir.join("SPEC.md"), "+++\n# no provenance\n+++\n# Intent\n").unwrap();

        let err = Unit::from_surface_dir(&dir).unwrap_err();
        assert!(matches!(
            err,
            KindError::SurfaceMissingField {
                field: "provenance",
                ..
            }
        ));
    }

    #[test]
    fn a_surface_with_a_malformed_document_is_a_load_error() {
        let root = surface_tmpdir("bad-doc");
        let dir = root.join("00-intent");
        std::fs::create_dir_all(&dir).unwrap();
        // The lone `.md` is not a `+++`-fenced document — a hard error, never a skip.
        std::fs::write(dir.join("SPEC.md"), "# no fence here\nbody\n").unwrap();

        let err = Unit::from_surface_dir(&dir).unwrap_err();
        assert!(matches!(err, KindError::Document { .. }));
    }

    #[test]
    fn a_surface_without_a_body_file_is_a_load_error() {
        let root = surface_tmpdir("no-body");
        let dir = root.join("00-intent");
        std::fs::create_dir_all(&dir).unwrap();
        // No `.md` member document at all — only a stray non-markdown sibling.
        std::fs::write(dir.join("notes.txt"), "not a document\n").unwrap();

        let err = Unit::from_surface_dir(&dir).unwrap_err();
        assert!(matches!(err, KindError::SurfaceBody { found: 0, .. }));
    }

    #[test]
    fn a_surface_with_two_body_files_is_ambiguous() {
        let root = surface_tmpdir("two-body");
        let dir = root.join("00-intent");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("SPEC.md"), "+++\n+++\n# One\n").unwrap();
        std::fs::write(dir.join("EXTRA.md"), "+++\n+++\n# Two\n").unwrap();

        let err = Unit::from_surface_dir(&dir).unwrap_err();
        assert!(matches!(err, KindError::SurfaceBody { found: 2, .. }));
    }
}
