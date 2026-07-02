//! The extraction algebra — a custom kind's read side, composed from data.
//!
//! specs/15-kinds.md. Where `crate::contract` is the engine's predicate half
//! (what an artifact must satisfy), this is the extraction half (what it *is*,
//! and how it is read). Extraction is the soundness boundary: a clause is sound
//! only if its feature is deterministically extractable, so a custom kind carries
//! no code of its own — its extractor is composed from a closed algebra of
//! deterministic [`Primitive`]s. That closed vocabulary makes unsound extraction
//! unsayable: an out-of-vocabulary primitive is a load error, never a per-kind
//! escape hatch.
//!
//! Every primitive delegates to the same surface extractor the built-in
//! projectors use (`crate::extract`), so the soundness boundary is one boundary,
//! not a forked implementation that can drift.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;
use toml_edit::{DocumentMut, Item, Table};

use crate::compose::Edge;
use crate::document::{Document, PublishedRequirement};
use crate::extract::{self, Features};

/// The built-in harness kinds temper ships an engine-code extractor for. A
/// `[kind.<name>]` registration naming one of these is a built-in layer; any
/// other name registers a custom kind, defined under
/// `.temper/kinds/<name>/KIND.md` (`specs/15-kinds.md`; `specs/40-composition.md`).
pub const BUILTIN_KINDS: &[&str] = &["skill", "rule"];

/// The file locus a custom kind reads (`specs/40-composition.md`): the root
/// directory its units live under, and the filename glob that selects them.
/// `import` scans `root` for files matching `glob`. File placement is itself an
/// extraction primitive, so the locus is part of the authored definition, not
/// external config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Governs {
    /// The root directory the kind's units sit under (`specs`, `docs/adr`), a path
    /// relative to the harness the assembly governs.
    pub root: String,
    /// The filename glob that selects the kind's units under `root` (`*.md`,
    /// `[0-9][0-9]-*.md`), stored verbatim.
    pub glob: String,
}

/// A custom kind's authored definition, loaded from `.temper/kinds/<name>/KIND.md`
/// (`specs/20-surface.md`; `specs/40-composition.md`). The `+++`-fenced header
/// carries the [`governs`](CustomKind::governs) locus, the composed
/// [`Extraction`], and the declared [`relationships`](CustomKind::relationships);
/// the body is the kind's own prose, read by no check.
///
/// A custom kind is purely declare-side — it carries no clauses. Its require-side
/// is a package bound in the assembly, resolved under `.temper/packages/` exactly
/// as a built-in kind binds its shipped one.
///
/// Not `Eq`: keeping the derive `PartialEq` leaves room for future `f64`-bearing
/// fields without churn, as it does for [`Clause`](crate::contract::Clause).
#[derive(Debug, Clone, PartialEq)]
pub struct CustomKind {
    /// The kind's name — the `[kind.<name>]` registration key and the
    /// `<name>` directory its `KIND.md` lives under.
    pub name: String,
    /// The file locus the kind reads.
    pub governs: Governs,
    /// The composed extractor over the closed algebra (`specs/15-kinds.md`), parsed
    /// from the header's `[[extraction]]` array by [`Extraction::from_table`].
    /// Absent ⇒ the vacuous extractor (only the intrinsic id).
    pub extraction: Extraction,
    /// The declared relationships — which of the kind's references are edges
    /// (`specs/15-kinds.md`), each an [`Edge`] whose `from` is this kind. Parsed
    /// from the header's `[[relationships]]` array. Absent ⇒ empty.
    pub relationships: Vec<Edge>,
}

impl CustomKind {
    /// Load a custom kind's authored definition from `<kinds_dir>/<name>/KIND.md`
    /// (`specs/20-surface.md`). A missing document is a
    /// [`KindError::MissingDefinition`] — the assembly registered the kind, so its
    /// definition is required, never silently skipped; a malformed document, an
    /// out-of-vocabulary primitive, a bad `governs`, or a stray key each surface as
    /// a precise [`KindError`].
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

    /// Parse a custom kind's definition off a `KIND.md` header table — the
    /// [`governs`](CustomKind::governs) locus, the `[[extraction]]` extractor (via
    /// [`Extraction::from_table`]), and the `[[relationships]]` edges. The seam
    /// tests drive without touching disk. A header carries only `governs`,
    /// `extraction`, and `relationships`; any stray key is a
    /// [`KindError::UnknownKey`], rejected rather than silently dropped
    /// (`specs/10-contracts.md`).
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
/// declaration order (`specs/15-kinds.md`). The owning `kind` is each edge's source
/// (the implicit `from`); each relationship names its reference `field` and target
/// `to` kind, both strings. Absent ⇒ an empty vec; not an array-of-tables ⇒
/// [`KindError::RelationshipsNotArray`]; a missing/mistyped `field` or `to` ⇒ a
/// folded [`KindError::BadRelationship`] naming its position.
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
    /// `sections` — the body's ATX sections (each heading + the body span beneath
    /// it), in document order (`Features::sections`) — the `## Decision`-block
    /// feature a `section_contains` clause decides over (`specs/10-contracts.md`).
    Sections,
    /// `line_count` — the body's line count (`Features::body_lines`), the
    /// `max_lines` feature.
    LineCount,
    /// `placement` — the name of the directory the unit sits under
    /// (`Features::source_dir`) — file placement.
    Placement,
}

impl Primitive {
    /// This primitive's TOML `primitive` discriminator — the key it is parsed
    /// from, reused as the vocabulary name a diagnostic reports.
    #[must_use]
    pub fn key(&self) -> &'static str {
        match self {
            Primitive::Field { .. } => "field",
            Primitive::Headings => "headings",
            Primitive::Sections => "sections",
            Primitive::LineCount => "line_count",
            Primitive::Placement => "placement",
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
            Primitive::Sections => features.sections = extract::body_sections(&unit.body),
            Primitive::LineCount => features.body_lines = extract::body_line_count(&unit.body),
            Primitive::Placement => {
                features.source_dir = extract::source_dir_name(&unit.source_path)
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
    /// `headings`, `sections`, and `line_count`.
    pub body: String,
    /// The source path the unit was read from — the `placement` locus.
    pub source_path: PathBuf,
    /// The requirements this unit opts into filling — the authored
    /// `[satisfies.<requirement>]` header modules (`specs/20-surface.md`). A
    /// representation edge the coverage check resolves, not a composed feature:
    /// intrinsic to the surface, threaded through unchanged so a custom-kind member
    /// joins coverage exactly as a skill/rule does (`specs/10-contracts.md`). Empty
    /// when the member authors none.
    pub satisfies: Vec<String>,
    /// The same `[satisfies.<requirement>]` opt-ins **with their authored rationale**
    /// (`specs/20-surface.md`, "The member document") — the whole [`Satisfies`] clause,
    /// not just the name coverage reads. The read family (`why`/`requirements`) narrates
    /// the *why* a custom member fills a requirement (READ-CUSTOM-SATISFIERS), so it
    /// needs the rationale the decidable [`satisfies`](Unit::satisfies) name-vec drops.
    /// Populated from the same header parse (`crate::document::satisfies`); empty when
    /// the member authors none.
    pub satisfies_clauses: Vec<crate::document::Satisfies>,
    /// The requirements this unit **publishes** — the authored `[requirement.<name>]`
    /// header modules (`specs/10-contracts.md`, "Decision: a requirement's publisher is
    /// any authored surface document"). The demand side of the fill edge, threaded
    /// through unchanged so a custom-kind member (an intent `spec`) publishes into the
    /// one requirement namespace exactly as the assembly does. Empty when the member
    /// publishes none.
    pub published_requirements: Vec<PublishedRequirement>,
}

impl Unit {
    /// Reload a written custom-unit surface `<root>/<name>/` into a raw [`Unit`]:
    /// the id is the surface directory name, and its lone `.md` sibling is the
    /// member document (`specs/20-surface.md`) — a `+++`-fenced `[provenance]`
    /// header over the byte-faithful body, whose `source_path` `import` wrote
    /// (`src/import.rs`, `import_custom_unit`).
    ///
    /// The generic inverse of that projection: keyed on the surface shape every
    /// custom kind shares (a lone member document found by extension), not on any
    /// one kind's IR, so it is the sole reader `check`'s custom-kind path uses and a
    /// kind rooted at any `governs.root` — not just `specs/` — is read
    /// (`specs/40-composition.md`). The `[clause.<field>]` header values are lifted
    /// into `frontmatter`, so the `field` primitive ranges over a custom member's
    /// declared fields exactly as it does a built-in's parsed frontmatter
    /// (`specs/20-surface.md`); a member carrying no clause tables reloads with empty
    /// frontmatter, its whole source file preserved in the body. An unreadable or
    /// malformed surface is a [`KindError`], never a silent skip.
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

        // The rationale-carrying clauses are read whole: coverage feeds off the
        // requirement name alone (the per-clause `rationale` is the human *why*, never
        // a decidable feature), while the read family narrates the rationale too
        // (READ-CUSTOM-SATISFIERS). One parse, both consumers (`specs/20-surface.md`).
        let satisfies_clauses = crate::document::satisfies(document.header());
        let satisfies = satisfies_clauses
            .iter()
            .map(|s| s.requirement.clone())
            .collect();

        // The demand side: `[requirement.*]` modules the member publishes, carried
        // through unchanged into the one namespace the gate unions (`specs/10-contracts.md`).
        let published_requirements =
            crate::document::requirements(document.header()).map_err(|source| {
                KindError::Document {
                    path: doc_path.clone(),
                    source,
                }
            })?;

        // The `[clause.<field>]` header values are the member's typed fields — lift
        // each into `frontmatter` so the `field` primitive ranges over a custom member
        // exactly as it does a built-in's parsed frontmatter (`specs/20-surface.md`). A
        // clause whose `value` is JSON-null-unrepresentable is dropped, never invented.
        let frontmatter = crate::document::clauses(document.header())
            .into_iter()
            .filter_map(|(field, value)| {
                crate::document::item_to_json(value).map(|json| (field, json))
            })
            .collect();

        Ok(Self {
            id,
            frontmatter,
            body: document.body().to_string(),
            source_path: PathBuf::from(source_path),
            satisfies,
            satisfies_clauses,
            published_requirements,
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

    /// A primitive names an extractor outside the closed vocabulary — the trapdoor
    /// the closed algebra exists to keep shut, so it is rejected at load, never
    /// skipped (`specs/15-kinds.md`).
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
    /// `.temper/kinds/<name>/KIND.md` is absent. A registration promises a
    /// definition (`specs/40-composition.md`), so a missing one is a hard error,
    /// never a silent skip.
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
    /// `extraction`, `relationships`) — a leftover `clause`, an `entities` table, or
    /// a typo — rejected at load rather than silently dropped (`specs/10-contracts.md`).
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
            sections: Vec::new(),
            source_dir: None,
            // `satisfies` is a surface edge threaded through unchanged, not a
            // composed primitive, so a custom-kind member joins coverage exactly as
            // a built-in kind's does (`specs/10-contracts.md`).
            satisfies: unit.satisfies.clone(),
            // The demand side rides through the same way — a published `[requirement.*]`
            // is authored surface state, never a composed feature.
            published_requirements: unit.published_requirements.clone(),
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
        "sections" => Primitive::Sections,
        "line_count" => Primitive::LineCount,
        "placement" => Primitive::Placement,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::{FeatureValue, Kind};

    /// The composed `spec`-shaped extractor the worked example needs
    /// (`specs/15-kinds.md`): line count, ATX headings, and file placement —
    /// markdown structure only, no body-mined references (the `references`
    /// primitive is retired; the corpus's edges are declared in member headers).
    fn spec_extraction() -> Extraction {
        let toml = r#"
[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "placement"
"#;
        Extraction::parse(toml, Path::new("temper.toml")).unwrap()
    }

    /// A raw spec-shaped unit: no frontmatter, a body carrying two headings and a
    /// filename inside a fenced block (which heading/line-count extraction skips).
    fn spec_unit() -> Unit {
        let body = "# Kinds\n\
\n\
## The extraction algebra\n\
\n\
Composed like `15-kinds.md` over `10-contracts.md`.\n\
\n\
```text\n\
`inside-a-fence.md` is illustration, not a heading\n\
```\n";
        Unit {
            id: "15-kinds".to_string(),
            frontmatter: BTreeMap::new(),
            body: body.to_string(),
            source_path: PathBuf::from("specs/15-kinds.md"),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
            published_requirements: Vec::new(),
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

        // A frontmatter-less kind composes no `field`, and body-mined references are
        // retired — nothing lands in `fields`.
        assert!(features.fields.is_empty());
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
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
            published_requirements: Vec::new(),
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
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
            published_requirements: Vec::new(),
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
    fn the_retired_references_primitive_is_now_an_unknown_extractor() {
        // Law 8 (`specs/00-intent.md`; `specs/15-kinds.md`, "Decision: no
        // body-mined references"): `references` grepped prose and called the result
        // structure. Retired from the vocabulary, a KIND.md still naming it is
        // rejected exactly as any other out-of-vocabulary primitive is — a mined
        // edge no longer wears tier-1 clothes.
        let toml = "[[extraction]]\nprimitive = \"references\"\nfeature = \"references\"\n";
        let err = Extraction::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            KindError::UnknownPrimitive { ref primitive, index: 0, .. }
                if primitive == "references"
        ));
    }

    #[test]
    fn a_primitive_missing_its_parameter_is_a_load_error() {
        // `field` names the frontmatter key it reads; omitting it is a load error.
        let toml = "[[extraction]]\nprimitive = \"field\"\n";
        let err = Extraction::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            KindError::MissingParam {
                param: "key",
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
        let body = "# Kinds\n\nComposed over the predicate half.\n";
        let dir = write_surface(&root, "15-kinds", "specs/15-kinds.md", "SPEC.md", body);

        let unit = Unit::from_surface_dir(&dir).unwrap();
        let features = spec_extraction().extract(&unit);

        assert_eq!(features.id, "15-kinds");
        assert_eq!(features.body_lines, 3);
        assert_eq!(features.headings, vec!["Kinds".to_string()]);
        assert_eq!(features.source_dir.as_deref(), Some("specs"));
        // The composed `spec` extractor mines no references — `fields` stays empty.
        assert!(features.fields.is_empty());
    }

    #[test]
    fn from_surface_dir_lifts_clause_fields_into_frontmatter() {
        // A custom member carrying `[clause.<field>]` header tables reloads with those
        // fields in `frontmatter` — the generic reader that closes the built-in/custom
        // asymmetry (`specs/20-surface.md`): a custom member's declared fields are the
        // `field` primitive's locus, like a built-in's parsed frontmatter.
        let root = surface_tmpdir("clause-fields").join("specs");
        let dir = root.join("15-kinds");
        std::fs::create_dir_all(&dir).unwrap();
        let document = "+++\n\
[clause.name]\n\
value = \"15-kinds\"\n\
[clause.priority]\n\
value = 7\n\
[provenance]\n\
source_path = \"specs/15-kinds.md\"\n\
import_hash = \"deadbeef\"\n\
+++\n\
# Kinds\n\nBody.\n";
        std::fs::write(dir.join("SPEC.md"), document).unwrap();

        let unit = Unit::from_surface_dir(&dir).unwrap();

        // The clause values land in `frontmatter`, JSON-kind-faithful: a string stays
        // a string, a bare integer stays an integer.
        assert_eq!(
            unit.frontmatter.get("name"),
            Some(&JsonValue::String("15-kinds".to_string()))
        );
        assert_eq!(unit.frontmatter.get("priority"), Some(&JsonValue::from(7)));

        // And they resolve through the composed `field` primitive exactly as a
        // built-in's parsed frontmatter does — the asymmetry closed.
        let toml = "[[extraction]]\nprimitive = \"field\"\nkey = \"name\"\n\
[[extraction]]\nprimitive = \"field\"\nkey = \"priority\"\n";
        let extraction = Extraction::parse(toml, Path::new("temper.toml")).unwrap();
        let features = extraction.extract(&unit);
        assert_eq!(
            features.field("name"),
            Some(&FeatureValue::scalar(Kind::String, "15-kinds"))
        );
        assert_eq!(
            features.field("priority").map(FeatureValue::kind),
            Some(Kind::Integer)
        );
    }

    #[test]
    fn from_surface_dir_with_no_clause_tables_yields_empty_frontmatter() {
        // A member document with no `[clause.<field>]` tables (only provenance) reloads
        // with empty frontmatter — the built-in floor's default, unchanged from before
        // this reader existed.
        let root = surface_tmpdir("no-clauses").join("specs");
        let dir = write_surface(
            &root,
            "00-intent",
            "specs/00-intent.md",
            "SPEC.md",
            "# Intent\n\nBody.\n",
        );

        let unit = Unit::from_surface_dir(&dir).unwrap();
        assert!(unit.frontmatter.is_empty());
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
