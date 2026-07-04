//! The extraction algebra — a custom kind's read side, composed from data.
//!
//! specs/architecture/15-kinds.md. Where `crate::contract` is the engine's predicate half
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
/// `.temper/kinds/<name>/KIND.md` (`specs/architecture/15-kinds.md`; `specs/architecture/40-composition.md`).
pub const BUILTIN_KINDS: &[&str] = &["skill", "rule"];

/// The file locus a custom kind reads (`specs/architecture/40-composition.md`): the root
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
/// (`specs/architecture/20-surface.md`; `specs/architecture/40-composition.md`). The `+++`-fenced header
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
    /// The composed extractor over the closed algebra (`specs/architecture/15-kinds.md`), parsed
    /// from the header's `[[extraction]]` array by [`Extraction::from_table`].
    /// Absent ⇒ the vacuous extractor (only the intrinsic id).
    pub extraction: Extraction,
    /// The declared relationships — which of the kind's references are edges
    /// (`specs/architecture/15-kinds.md`), each an [`Edge`] whose `from` is this kind. Parsed
    /// from the header's `[[relationships]]` array. Absent ⇒ empty.
    pub relationships: Vec<Edge>,
    /// The declared projection format — how a member's on-disk artifact is shaped
    /// (`specs/architecture/15-kinds.md`, "the adapter faces are declared"). A closed vocabulary;
    /// absent ⇒ `None` (today's built-in KIND.md declare none). Inert until
    /// DECLARED-FRONTMATTER-ADAPTER: parsed and typed, consumed by nothing yet.
    pub format: Option<Format>,
    /// The declared unit shape — whether a member is a lone file (id from the stem)
    /// or a directory with companions (id from the directory name)
    /// (`specs/architecture/15-kinds.md`). A closed enum; absent ⇒ `None`. Inert alongside
    /// [`format`](CustomKind::format).
    pub unit_shape: Option<UnitShape>,
    /// The declared activation — the kind's inherent world-edges (`specs/architecture/15-kinds.md`,
    /// "Activation — a kind's inherent world-edges"): how the harness reaches a member,
    /// and over which declared field. A closed vocabulary; absent ⇒ `None` (today's
    /// built-in KIND.md declare none). Stored inert — REACHABILITY reads it to decide a
    /// member's declared activation edge is dead; nothing consumes it yet.
    pub activation: Option<Activation>,
    /// The declared **provider** — the authority that *defines* the external format
    /// this kind mirrors (`specs/architecture/15-kinds.md`, "Decision: kind identity carries a
    /// provider axis"): a tool (`claude-code`) or a standard (`agents-md`). *Any*
    /// string — the vocabulary is the market's, not the parser's — so a present value
    /// is admissible verbatim and only a non-string is a load error. Absent ⇒ `None`:
    /// a project's own kind (`spec`) mirrors nothing external and stays bare. Feeds
    /// [`qualified_name`](CustomKind::qualified_name); the bare→unique-or-collision
    /// wiring into the assembly-binding/`satisfies`-typing consumers is BINDING-QUALIFY.
    pub provider: Option<String>,
    /// The kind's declared **genres** — typed shapes for its members' recurring prose
    /// forms (`specs/architecture/15-kinds.md`, "genres (optional)"; `specs/architecture/20-surface.md`,
    /// "Genre values"), parsed from the header's `[[genres]]` array. Extraction folds a
    /// member's genre fences into typed values against this set ([`CustomKind::extract`]);
    /// the shape is the kind's, the predicates the bound package's. Absent ⇒ empty.
    pub genres: Vec<Genre>,
}

/// A kind's declared **projection format** — the closed vocabulary naming how a
/// member's on-disk artifact is shaped (`specs/architecture/15-kinds.md`, "Decision: the adapter
/// faces are declared"). The engine implements each format once, generically; the
/// first and only harvested entry is [`YamlFrontmatter`](Format::YamlFrontmatter).
/// Any other value is a load error, the same closed-vocabulary guard the extraction
/// primitives carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// `yaml-frontmatter` — YAML frontmatter over a markdown body, the Claude Code
    /// family's shape.
    YamlFrontmatter,
}

/// A kind's declared **unit shape** — the format fact that varies per kind
/// (`specs/architecture/15-kinds.md`): whether a member's on-disk artifact is a lone file, its
/// identity the filename stem, or a directory with companions, its identity the
/// directory name. A closed enum; any other value is a load error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitShape {
    /// `file` — a lone file; the member's id is its filename stem (a rule's
    /// `.claude/rules/rust.md`).
    File,
    /// `directory` — a directory with companions; the member's id is the directory
    /// name (a skill's `.claude/skills/<name>/SKILL.md`).
    Directory,
}

/// A kind's declared **activation** — its inherent world-edges, the inbound
/// boundary edges of the relation graph (`specs/architecture/15-kinds.md`, "Activation — a
/// kind's inherent world-edges"): how the harness reaches a member, per-kind
/// mechanics over per-member data. A closed vocabulary harvested from the kinds
/// temper ships; any other value is a load error, the same closed-vocabulary guard
/// [`Format`] and [`UnitShape`] carry. The three field-carrying variants name the
/// declared frontmatter field they range over, never a value — the glob/description
/// *values* stay the member's ordinary clauses. Inert until REACHABILITY reads it to
/// decide a member's declared activation edge is dead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Activation {
    /// `always` — loaded at launch, unconditionally (a rule without `paths`;
    /// `CLAUDE.md` itself). Carries no field: the edge is unconditional.
    Always,
    /// `description-trigger(field)` — the named field is always in context, the body
    /// loading on invocation (a skill's `description`). The field names the declared
    /// frontmatter field the trigger ranges over.
    DescriptionTrigger {
        /// The declared frontmatter field always kept in context.
        field: String,
    },
    /// `paths-match(field)` — the member activates when the agent reads files matching
    /// the named glob field (a path-scoped rule's `paths`).
    PathsMatch {
        /// The declared frontmatter field carrying the activation glob.
        field: String,
    },
    /// `event(field)` — the member executes at a named lifecycle event (carried for the
    /// future `hook` kind). The field names the declared lifecycle-event field.
    Event {
        /// The declared frontmatter field naming the lifecycle event.
        field: String,
    },
}

/// A **genre** a kind declares — a typed shape for one of its members' recurring prose
/// forms (`specs/architecture/15-kinds.md`, "genres (optional)"; `specs/architecture/20-surface.md`,
/// "Genre values — prose that declares its own anatomy"): named fields over prose
/// **leaves** plus keyed **collections**, serialized whole into the manifest. The shape
/// is the kind's; any *predicate* over it (a decision names at least one rejected
/// alternative) is the bound package's, **out of the kind object** — the same ownership
/// line extraction and contract split on everywhere. So a `Genre` carries the vocabulary,
/// never a clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Genre {
    /// The genre name — the `genre.<name>` a fence info string carries
    /// (`genre.decision surface-authority` → `decision`), the token extraction matches a
    /// fence against to fold it into a typed [`GenreValue`](crate::extract::GenreValue).
    pub name: String,
    /// The declared **prose-leaf** field names — the genre value's top-level authored
    /// strings. The declared schema a genre-value predicate (the bound package, out of
    /// the kind object) ranges over; extraction classifies a fence's interior
    /// structurally (a string is a leaf, a table a collection), so this list is inert
    /// until that predicate lands — the same declared-and-inert posture
    /// [`format`](CustomKind::format)/[`activation`](CustomKind::activation) carry.
    pub leaves: Vec<String>,
    /// The declared **keyed-collection** names — the genre value's sibling collections
    /// (`rejected`). Declared schema like [`leaves`](Genre::leaves), inert until the
    /// genre-value predicate reads it.
    pub collections: Vec<String>,
}

impl CustomKind {
    /// Load a custom kind's authored definition from `<kinds_dir>/<name>/KIND.md`
    /// (`specs/architecture/20-surface.md`). A missing document is a
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
    /// [`Extraction::from_table`]), the `[[relationships]]` edges, and the declared
    /// adapter faces ([`format`](CustomKind::format), [`unit_shape`](CustomKind::unit_shape),
    /// [`activation`](CustomKind::activation)). The seam tests drive without touching disk.
    /// A header carries only `governs`, `extraction`, `relationships`, `format`,
    /// `unit_shape`, `activation`, and `provider`; any stray key is a
    /// [`KindError::UnknownKey`], rejected rather than silently dropped
    /// (`specs/architecture/10-contracts.md`).
    pub fn from_header(table: &Table, name: &str, path: &Path) -> Result<Self, KindError> {
        for (key, _) in table.iter() {
            if !matches!(
                key,
                "governs"
                    | "extraction"
                    | "relationships"
                    | "format"
                    | "unit_shape"
                    | "activation"
                    | "provider"
                    | "genres"
            ) {
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
        let format = parse_format(table, name, path)?;
        let unit_shape = parse_unit_shape(table, name, path)?;
        let activation = parse_activation(table, name, path)?;
        let provider = parse_provider(table, name, path)?;
        let genres = parse_genres(table, name, path)?;
        Ok(Self {
            name: name.to_string(),
            governs,
            extraction,
            relationships,
            format,
            unit_shape,
            activation,
            provider,
            genres,
        })
    }

    /// Run the kind's composed extractor over `unit`, then fold its declared genres
    /// (`specs/architecture/20-surface.md`, "Genre values"): each fenced block whose info string
    /// names a declared genre (`genre.<genre> <key>`) has its interior TOML parsed into a
    /// typed [`GenreValue`](crate::extract::GenreValue) and folded into `Features::genres`,
    /// beside its raw form in `fenced_blocks`. This composes the `Fenced` primitive with a
    /// TOML parse — the typed genre layer over the raw-block algebra (`specs/architecture/15-kinds.md`).
    /// The single entry point every extract call site routes through, so genre folding
    /// never forks from the primitive extraction. A kind declaring no genres (every
    /// built-in), or a body with no matching fence, folds nothing.
    #[must_use]
    pub fn extract(&self, unit: &Unit) -> Features {
        let mut features = self.extraction.extract(unit);
        self.fold_genres(&mut features);
        features
    }

    /// Fold this kind's declared genres out of the already-extracted `fenced_blocks`
    /// (`specs/architecture/20-surface.md`). A block whose info string parses as
    /// `genre.<genre> <key>` for a **declared** genre and whose interior is well-formed
    /// TOML becomes a [`GenreValue`](crate::extract::GenreValue); a fence naming an
    /// undeclared genre, or any non-genre block, stays raw-only — genre adoption is opt-in
    /// per block. A pure function of `fenced_blocks` and the declared genre set, so
    /// re-running is byte-identical, the property that keeps a genre value a sound gate
    /// input.
    fn fold_genres(&self, features: &mut Features) {
        if self.genres.is_empty() {
            return;
        }
        let mut genres = Vec::new();
        for block in &features.fenced_blocks {
            let Some((genre, key)) = extract::parse_genre_info(&block.info) else {
                continue;
            };
            if !self.genres.iter().any(|declared| declared.name == genre) {
                continue;
            }
            if let Some(value) = extract::parse_genre_value(&genre, &key, &block.content) {
                genres.push(value);
            }
        }
        features.genres = genres;
    }

    /// The kind's **qualified identity** — `<provider>.<name>` when a provider is
    /// declared, the bare `name` otherwise (`specs/architecture/15-kinds.md`, "Decision: kind
    /// identity carries a provider axis"). A kind that mirrors an external format
    /// qualifies by the authority defining it; a project's own kind mirrors nothing
    /// and stays bare, paying no qualification tax until two providers actually meet
    /// under one bare name (see [`resolve_bare`](CustomKind::resolve_bare)).
    #[must_use]
    pub fn qualified_name(&self) -> String {
        match &self.provider {
            Some(provider) => format!("{provider}.{}", self.name),
            None => self.name.clone(),
        }
    }

    /// Resolve a **bare** kind reference against a kind set (`specs/architecture/15-kinds.md`,
    /// "Decision: kind identity carries a provider axis"): a bare `name` resolves iff
    /// exactly one kind in `kinds` carries it, returning that unique kind; two
    /// providers meeting under one bare name is a collision, a
    /// [`KindError::AmbiguousKind`] naming the qualified candidates so the author
    /// qualifies the reference. No kind carrying the name resolves to `Ok(None)` — an
    /// unknown-reference is the consuming binding's concern (BINDING-QUALIFY), not this
    /// pure identity mechanism's.
    ///
    /// # Errors
    ///
    /// Returns [`KindError::AmbiguousKind`] when more than one kind in `kinds` carries
    /// the bare `name`.
    pub fn resolve_bare<'a>(
        name: &str,
        kinds: &'a [CustomKind],
    ) -> Result<Option<&'a CustomKind>, KindError> {
        let mut matches = kinds.iter().filter(|kind| kind.name == name);
        let Some(first) = matches.next() else {
            return Ok(None);
        };
        if matches.next().is_none() {
            return Ok(Some(first));
        }
        // A collision: name the qualified candidates so the author knows what to
        // disambiguate against. Re-scan for the full set — the message is worth the
        // second pass over a handful of kinds (this tool is I/O-bound over tiny files).
        let candidates = kinds
            .iter()
            .filter(|kind| kind.name == name)
            .map(CustomKind::qualified_name)
            .collect::<Vec<_>>()
            .join(", ");
        Err(KindError::AmbiguousKind {
            name: name.to_string(),
            candidates,
        })
    }

    /// The kind's declared frontmatter fields, in declaration order — the `field`
    /// extraction primitives' keys (`specs/architecture/15-kinds.md`, "the adapter faces are
    /// declared"). The generic frontmatter adapter (`crate::frontmatter`) lifts these
    /// into the leading `[clause.<field>]` tables, before the preserved unknown keys.
    #[must_use]
    pub fn declared_fields(&self) -> Vec<&str> {
        self.extraction
            .primitives()
            .iter()
            .filter_map(|primitive| match primitive {
                Primitive::Field { key } => Some(key.as_str()),
                _ => None,
            })
            .collect()
    }

    /// The surface member-document filename for this kind — the kind name upper-cased
    /// with a `.md` suffix (`skill` → `SKILL.md`, `rule` → `RULE.md`), the name both
    /// the emit face writes and the reload face reads (`src/frontmatter.rs`,
    /// `src/import.rs`).
    #[must_use]
    pub fn member_document(&self) -> String {
        format!("{}.md", self.name.to_uppercase())
    }

    /// The surface subdirectory a member of this kind lands under — the leaf of the
    /// `governs.root` locus (`.claude/skills` → `skills`, `.claude/rules` → `rules`).
    /// The read face's scan root and the emit face's write root share this leaf, so a
    /// built-in kind's surface tree is derived from its declaration, not hardwired.
    #[must_use]
    pub fn surface_subdir(&self) -> &str {
        self.governs
            .root
            .rsplit('/')
            .next()
            .unwrap_or(&self.governs.root)
    }

    /// Whether a surface member imported from `source_path` belongs to this kind — its
    /// source filename matches the kind's `governs` glob leaf. The discriminator for kinds
    /// that **share a surface locus**: the two `memory` providers both project their member
    /// to `./MEMORY.md` (`claude-code.memory` from `CLAUDE.md`, `agents-md.memory` from
    /// `AGENTS.md`), so the projected document alone cannot say which package governs it —
    /// the provenance source name does (`specs/architecture/20-surface.md`). A kind at a unique locus
    /// (skill's `SKILL.md`, rule's `*.md`) matches its own members, so the filter is a no-op
    /// there. A member with no readable source name belongs to nothing rather than
    /// mis-dispatching.
    #[must_use]
    pub fn owns_source(&self, source_path: &Path) -> bool {
        source_path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| glob_matches(self.governs.glob_leaf(), name))
    }
}

impl Governs {
    /// The glob's file-matching **leaf** — its final `/`-separated segment
    /// (`*/SKILL.md` → `SKILL.md`, `CLAUDE.md` → `CLAUDE.md`). Earlier segments select
    /// subdirectories to descend; the leaf selects the member files under the locus, so it
    /// is the segment a per-member membership test ([`CustomKind::owns_source`]) matches
    /// against a source filename.
    #[must_use]
    pub fn glob_leaf(&self) -> &str {
        self.glob.rsplit('/').next().unwrap_or(&self.glob)
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
/// declaration order (`specs/architecture/15-kinds.md`). The owning `kind` is each edge's source
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

/// Parse a `KIND.md` header's optional `[[genres]]` array into typed [`Genre`] shapes,
/// in declaration order (`specs/architecture/15-kinds.md`, "genres (optional)";
/// `specs/architecture/20-surface.md`, "Genre values"). Each genre names itself and its declared
/// prose-leaf and keyed-collection field vocabularies (both optional string arrays).
/// Absent ⇒ an empty vec; not an array-of-tables ⇒ [`KindError::GenresNotArray`]; a
/// missing/mistyped `name`, or a non-string-array `leaves`/`collections`, ⇒ a folded
/// [`KindError::BadGenre`] naming its position, exactly as [`parse_relationships`] folds
/// its own.
fn parse_genres(table: &Table, kind: &str, path: &Path) -> Result<Vec<Genre>, KindError> {
    let Some(item) = table.get("genres") else {
        return Ok(Vec::new());
    };
    let array = item
        .as_array_of_tables()
        .ok_or_else(|| KindError::GenresNotArray {
            path: path.to_path_buf(),
            kind: kind.to_string(),
        })?;
    let mut genres = Vec::with_capacity(array.len());
    for (index, genre) in array.iter().enumerate() {
        let bad = || KindError::BadGenre {
            path: path.to_path_buf(),
            kind: kind.to_string(),
            index,
        };
        let name = genre
            .get("name")
            .and_then(Item::as_str)
            .ok_or_else(bad)?
            .to_string();
        let leaves = genre_str_array(genre, "leaves", &bad)?;
        let collections = genre_str_array(genre, "collections", &bad)?;
        genres.push(Genre {
            name,
            leaves,
            collections,
        });
    }
    Ok(genres)
}

/// Read an optional string-array field off a `[[genres]]` table (`leaves`,
/// `collections`): absent ⇒ an empty vec; present-but-not-an-array-of-strings ⇒ the
/// folded [`KindError::BadGenre`] its caller supplies.
fn genre_str_array(
    table: &Table,
    key: &str,
    bad: &impl Fn() -> KindError,
) -> Result<Vec<String>, KindError> {
    match table.get(key) {
        None => Ok(Vec::new()),
        Some(item) => {
            let array = item.as_array().ok_or_else(bad)?;
            let mut out = Vec::with_capacity(array.len());
            for element in array.iter() {
                out.push(element.as_str().ok_or_else(bad)?.to_string());
            }
            Ok(out)
        }
    }
}

/// Parse a `KIND.md` header's optional `format` key into a typed [`Format`]
/// (`specs/architecture/15-kinds.md`, "the adapter faces are declared"). Absent ⇒ `Ok(None)`,
/// still valid — today's built-in KIND.md declare none. A non-string or an
/// out-of-vocabulary string is a [`KindError::OutOfVocab`], the closed-vocabulary
/// guard the extraction primitives carry applied to the projection-format face.
fn parse_format(table: &Table, kind: &str, path: &Path) -> Result<Option<Format>, KindError> {
    let Some(item) = table.get("format") else {
        return Ok(None);
    };
    match item.as_str() {
        Some("yaml-frontmatter") => Ok(Some(Format::YamlFrontmatter)),
        other => Err(KindError::OutOfVocab {
            path: path.to_path_buf(),
            kind: kind.to_string(),
            key: "format",
            value: other
                .map(str::to_string)
                .unwrap_or_else(|| render_item(item)),
            expected: "`yaml-frontmatter`",
        }),
    }
}

/// Parse a `KIND.md` header's optional unit-shape key into a typed [`UnitShape`]
/// (`specs/architecture/15-kinds.md`). Absent ⇒ `Ok(None)`; a non-string or an out-of-vocabulary
/// string is a [`KindError::OutOfVocab`], folding every malformation into one error
/// as [`parse_format`] does for the projection face.
fn parse_unit_shape(
    table: &Table,
    kind: &str,
    path: &Path,
) -> Result<Option<UnitShape>, KindError> {
    let Some(item) = table.get("unit_shape") else {
        return Ok(None);
    };
    match item.as_str() {
        Some("file") => Ok(Some(UnitShape::File)),
        Some("directory") => Ok(Some(UnitShape::Directory)),
        other => Err(KindError::OutOfVocab {
            path: path.to_path_buf(),
            kind: kind.to_string(),
            key: "unit_shape",
            value: other
                .map(str::to_string)
                .unwrap_or_else(|| render_item(item)),
            expected: "`file` or `directory`",
        }),
    }
}

/// Parse a `KIND.md` header's optional `activation` key into a typed [`Activation`]
/// (`specs/architecture/15-kinds.md`, "Activation — a kind's inherent world-edges"). Absent ⇒
/// `Ok(None)` — today's built-in KIND.md declare none. The value is an inline table
/// naming its vocab entry in `via`, plus (for the three field-carrying variants) the
/// declared frontmatter `field` it ranges over. A `via` outside the closed vocabulary
/// is a [`KindError::OutOfVocab`], the same guard [`parse_format`] carries; a structural
/// malformation — not a table, a missing/mistyped `via`, or a field-carrying variant
/// missing its `field` string — folds into [`KindError::BadActivation`], exactly as
/// [`parse_governs`] folds its locus malformations.
fn parse_activation(
    table: &Table,
    kind: &str,
    path: &Path,
) -> Result<Option<Activation>, KindError> {
    let Some(item) = table.get("activation") else {
        return Ok(None);
    };
    let bad = || KindError::BadActivation {
        path: path.to_path_buf(),
        kind: kind.to_string(),
    };
    let declaration = item.as_table_like().ok_or_else(bad)?;
    let via = declaration
        .get("via")
        .and_then(Item::as_str)
        .ok_or_else(bad)?;
    // The field-carrying variants name a declared frontmatter field, never a value.
    let field = || {
        declaration
            .get("field")
            .and_then(Item::as_str)
            .map(str::to_string)
            .ok_or_else(bad)
    };
    let activation = match via {
        "always" => Activation::Always,
        "description-trigger" => Activation::DescriptionTrigger { field: field()? },
        "paths-match" => Activation::PathsMatch { field: field()? },
        "event" => Activation::Event { field: field()? },
        other => {
            return Err(KindError::OutOfVocab {
                path: path.to_path_buf(),
                kind: kind.to_string(),
                key: "activation",
                value: other.to_string(),
                expected: "`always`, `description-trigger`, `paths-match`, or `event`",
            });
        }
    };
    Ok(Some(activation))
}

/// Parse a `KIND.md` header's optional `provider` key into the declared authority
/// (`specs/architecture/15-kinds.md`, "Decision: kind identity carries a provider axis"). Absent ⇒
/// `Ok(None)` — a project's own kind mirrors nothing external and stays bare. A present
/// value is *any string* (the vocabulary is the market's, not the parser's, so there is
/// no closed set to guard against, unlike [`parse_format`]); only a non-string folds
/// into a [`KindError::BadProvider`], as [`parse_governs`] folds its locus malformations.
fn parse_provider(table: &Table, kind: &str, path: &Path) -> Result<Option<String>, KindError> {
    let Some(item) = table.get("provider") else {
        return Ok(None);
    };
    item.as_str()
        .map(|provider| Some(provider.to_string()))
        .ok_or_else(|| KindError::BadProvider {
            path: path.to_path_buf(),
            kind: kind.to_string(),
        })
}

/// Render a non-string header value for an [`KindError::OutOfVocab`] diagnostic — its
/// TOML text with surrounding decor trimmed, so `format = 7` reports `7`.
fn render_item(item: &Item) -> String {
    item.as_value()
        .map(|value| value.to_string().trim().to_string())
        .unwrap_or_else(|| "a table".to_string())
}

/// Whether `glob` matches `name`, treating `*` as "any run of characters (including
/// empty)" and every other character literally — the minimal in-crate wildcard a
/// `governs` glob segment needs (`*.md`), short of pulling in a glob crate for one
/// metacharacter. Lives beside [`Governs`], the glob's home, so both `import`'s discovery
/// scan and a kind's own [`CustomKind::owns_source`] membership test share one matcher
/// (`.claude/rules/rust.md`). A standard linear matcher with single-star backtracking: on
/// a mismatch it falls back to the most recent `*`, extending what that star consumed by
/// one character. Matches one glob *segment*, not a `/`-path — the caller splits a
/// multi-segment glob and matches each part.
pub(crate) fn glob_matches(glob: &str, name: &str) -> bool {
    let pattern: Vec<char> = glob.chars().collect();
    let text: Vec<char> = name.chars().collect();
    let mut pi = 0;
    let mut ti = 0;
    // The position of the last `*` in `pattern`, and how much of `text` it had
    // consumed when we matched it — the backtrack point.
    let mut star: Option<usize> = None;
    let mut star_ti = 0;
    while ti < text.len() {
        if pi < pattern.len() && pattern[pi] == text[ti] {
            pi += 1;
            ti += 1;
        } else if pi < pattern.len() && pattern[pi] == '*' {
            star = Some(pi);
            star_ti = ti;
            pi += 1;
        } else if let Some(star_pi) = star {
            // Mismatch under an open `*`: let the star swallow one more character.
            pi = star_pi + 1;
            star_ti += 1;
            ti = star_ti;
        } else {
            return false;
        }
    }
    // Trailing `*`s match the empty remainder.
    while pi < pattern.len() && pattern[pi] == '*' {
        pi += 1;
    }
    pi == pattern.len()
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
/// (`specs/architecture/15-kinds.md`).
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
    /// feature a `section_contains` clause decides over (`specs/architecture/10-contracts.md`).
    Sections,
    /// `line_count` — the body's line count (`Features::body_lines`), the
    /// `max_lines` feature.
    LineCount,
    /// `placement` — the name of the directory the unit sits under
    /// (`Features::source_dir`) — file placement.
    Placement,
    /// `directives` — the body's format-executed directive occurrences for the
    /// named [`syntax`](DirectiveSyntax) (`specs/architecture/15-kinds.md`, "Directives
    /// — format-executed body syntax"), folded into `Features::directives` in
    /// document order. Unlike the mining the `references` retirement bans, a directive
    /// is grammar the format authority documents as *executed*, so its occurrences are
    /// observed structure, not typography.
    Directives {
        /// The directive syntax extracted — the closed per-syntax vocabulary, sole
        /// member `at-import`.
        syntax: DirectiveSyntax,
    },
    /// `fenced` — the body's fenced code blocks (`Features::fenced_blocks`), in
    /// document order, each block's info string paired with its interior content
    /// (`specs/architecture/15-kinds.md`, "a fenced block — whose first consumer is
    /// the genre fence"). Markdown structure, deterministically extractable like
    /// `headings`/`sections`: the same fence boundaries, surfaced whole. Its first
    /// consumer is the genre fence — fenced extraction composed with a TOML parse
    /// (GENRE-MANIFEST-LEAF); this primitive yields the raw blocks only.
    Fenced,
}

/// A directive's format-executed body syntax — the closed per-syntax vocabulary the
/// [`Directives`](Primitive::Directives) primitive ranges over
/// (`specs/architecture/15-kinds.md`, "Directives — format-executed body syntax"). The
/// sole harvested member is [`AtImport`](DirectiveSyntax::AtImport); any other value
/// is a load error, the closed-vocabulary guard the primitive discriminator carries
/// applied to the per-syntax face.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectiveSyntax {
    /// `at-import` — an `@path/to/file` occurrence imports the target file into
    /// context (documented for Claude Code memory files, resolved relative to the
    /// importing file, absolute allowed; code.claude.com/docs/en/memory, retrieved
    /// 2026-07-02).
    AtImport,
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
            Primitive::Directives { .. } => "directives",
            Primitive::Fenced => "fenced",
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
            Primitive::Directives { syntax } => match syntax {
                DirectiveSyntax::AtImport => {
                    features.directives = extract::body_at_imports(&unit.body)
                }
            },
            Primitive::Fenced => features.fenced_blocks = extract::body_fenced_blocks(&unit.body),
        }
    }
}

/// A raw markdown unit the composed extractor reads: the intrinsic identity plus
/// the three surface loci the primitives range over (parsed frontmatter, the
/// byte-faithful body, the source placement). Frontmatter is *already parsed* —
/// splitting it is the surface tier's job and varies per harness format
/// (`crate::frontmatter` vs a frontmatter-less spec), so this composer takes the
/// values rather than re-parse. A spec supplies an empty `frontmatter`.
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
    /// `[satisfies.<requirement>]` header modules (`specs/architecture/20-surface.md`). A
    /// representation edge the coverage check resolves, not a composed feature:
    /// intrinsic to the surface, threaded through unchanged so a custom-kind member
    /// joins coverage exactly as a skill/rule does (`specs/architecture/10-contracts.md`). Empty
    /// when the member authors none.
    pub satisfies: Vec<String>,
    /// The same `[satisfies.<requirement>]` opt-ins **with their authored rationale**
    /// (`specs/architecture/20-surface.md`, "The member document") — the whole [`Satisfies`] clause,
    /// not just the name coverage reads. The read family (`why`/`requirements`) narrates
    /// the *why* a custom member fills a requirement (READ-CUSTOM-SATISFIERS), so it
    /// needs the rationale the decidable [`satisfies`](Unit::satisfies) name-vec drops.
    /// Populated from the same header parse (`crate::document::satisfies`); empty when
    /// the member authors none.
    pub satisfies_clauses: Vec<crate::document::Satisfies>,
    /// The requirements this unit **publishes** — the authored `[requirement.<name>]`
    /// header modules (`specs/architecture/10-contracts.md`, "Decision: a requirement's publisher is
    /// any authored surface document"). The demand side of the fill edge, threaded
    /// through unchanged so a custom-kind member (an intent `spec`) publishes into the
    /// one requirement namespace exactly as the assembly does. Empty when the member
    /// publishes none.
    pub published_requirements: Vec<PublishedRequirement>,
}

impl Unit {
    /// Reload a written custom-unit surface `<root>/<name>/` into a raw [`Unit`]:
    /// the id is the surface directory name, and its lone `.md` sibling is the
    /// member document (`specs/architecture/20-surface.md`) — a `+++`-fenced `[provenance]`
    /// header over the byte-faithful body, whose `source_path` `import` wrote
    /// (`src/import.rs`, `import_custom_unit`).
    ///
    /// The generic inverse of that projection: keyed on the surface shape every
    /// custom kind shares (a lone member document found by extension), not on any
    /// one kind's IR, so it is the sole reader `check`'s custom-kind path uses and a
    /// kind rooted at any `governs.root` — not just `specs/` — is read
    /// (`specs/architecture/40-composition.md`). The `[clause.<field>]` header values are lifted
    /// into `frontmatter`, so the `field` primitive ranges over a custom member's
    /// declared fields exactly as it does a built-in's parsed frontmatter
    /// (`specs/architecture/20-surface.md`); a member carrying no clause tables reloads with empty
    /// frontmatter, its whole source file preserved in the body. An unreadable or
    /// malformed surface is a [`KindError`], never a silent skip.
    pub fn from_surface_dir(dir: &Path) -> Result<Self, KindError> {
        let doc_path = lone_body_file(dir)?;
        Self::from_member_document(dir, &doc_path)
    }

    /// Reload a surface member from an explicit member document `doc_path` under the
    /// surface directory `dir`, sharing the whole parse [`from_surface_dir`] runs.
    ///
    /// [`from_surface_dir`](Unit::from_surface_dir) finds the member document by the
    /// lone-`.md` convention every custom kind's surface shares; a **built-in** kind
    /// whose surface may carry markdown companions (a skill's `PLAYBOOK.md`) names its
    /// own member document instead — `SKILL.md`, `RULE.md` — so the companion never
    /// confuses the read. Both faces then read the surface through this one path
    /// (`specs/architecture/15-kinds.md`, "A built-in kind is an adapter"): the `[clause.*]` header
    /// lifts into `frontmatter`, `[satisfies.*]`/`[requirement.*]` into the edge sets,
    /// the body byte-faithful. The id is the surface directory name — the member's
    /// home, never a field it sets (`specs/architecture/15-kinds.md`, "the emit face owns the
    /// locus").
    ///
    /// # Errors
    ///
    /// Returns a [`KindError`] when the document is unreadable, is not a well-formed
    /// `+++`-fenced document, or carries no `[provenance]` — the same hard failures
    /// [`from_surface_dir`](Unit::from_surface_dir) raises, never a silent skip.
    pub fn from_member_document(dir: &Path, doc_path: &Path) -> Result<Self, KindError> {
        let id = dir
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_string)
            .ok_or_else(|| KindError::SurfaceMissingField {
                path: dir.to_path_buf(),
                field: "name",
            })?;

        let raw = std::fs::read_to_string(doc_path).map_err(|source| KindError::Io {
            path: doc_path.to_path_buf(),
            source,
        })?;
        let document = Document::parse(&raw).map_err(|source| KindError::Document {
            path: doc_path.to_path_buf(),
            source,
        })?;
        let (source_path, _source_hash) = crate::document::provenance(document.header())
            .ok_or_else(|| KindError::SurfaceMissingField {
                path: doc_path.to_path_buf(),
                field: "provenance",
            })?;

        // The rationale-carrying clauses are read whole: coverage feeds off the
        // requirement name alone (the per-clause `rationale` is the human *why*, never
        // a decidable feature), while the read family narrates the rationale too
        // (READ-CUSTOM-SATISFIERS). One parse, both consumers (`specs/architecture/20-surface.md`).
        let satisfies_clauses = crate::document::satisfies(document.header());
        let satisfies = satisfies_clauses
            .iter()
            .map(|s| s.requirement.clone())
            .collect();

        // The demand side: `[requirement.*]` modules the member publishes, carried
        // through unchanged into the one namespace the gate unions (`specs/architecture/10-contracts.md`).
        let published_requirements =
            crate::document::requirements(document.header()).map_err(|source| {
                KindError::Document {
                    path: doc_path.to_path_buf(),
                    source,
                }
            })?;

        // The `[clause.<field>]` header values are the member's typed fields — lift
        // each into `frontmatter` so the `field` primitive ranges over a custom member
        // exactly as it does a built-in's parsed frontmatter (`specs/architecture/20-surface.md`). A
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
    /// skipped (`specs/architecture/15-kinds.md`).
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

    /// A `directives` primitive names a `syntax` outside the closed per-syntax
    /// vocabulary — the same closed-algebra guard the primitive discriminator carries
    /// (`specs/architecture/15-kinds.md`, "Directives — format-executed body syntax"),
    /// applied to the per-syntax face. The sole member is `at-import`; any other value
    /// is rejected at load, never a per-kind escape hatch into arbitrary body syntax.
    #[error("{path}: extraction primitive {index} names unknown directive syntax `{syntax}`")]
    #[diagnostic(
        code(temper::kind::unknown_directive_syntax),
        help(
            "the directive vocabulary is closed — a syntax is admitted only where the format authority documents it as executed, never as a per-kind hatch; the sole member is `at-import`"
        )
    )]
    UnknownDirectiveSyntax {
        /// The declaration the primitive lives in.
        path: PathBuf,
        /// The zero-based primitive index.
        index: usize,
        /// The unrecognized directive syntax value.
        syntax: String,
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
    /// definition (`specs/architecture/40-composition.md`), so a missing one is a hard error,
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
    /// is meaningless, so the locus is required (`specs/architecture/40-composition.md`).
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
    /// `extraction`, `relationships`, `format`, `unit_shape`, `activation`, `provider`,
    /// `genres`) — a leftover `clause`, an `entities` table, or a typo — rejected at load
    /// rather than silently dropped (`specs/architecture/10-contracts.md`).
    #[error("{path}: custom kind `{kind}` definition has unknown key `{key}`")]
    #[diagnostic(
        code(temper::kind::unknown_key),
        help(
            "a `KIND.md` definition carries only `governs`, `extraction`, `relationships`, `format`, `unit_shape`, `activation`, `provider`, and `genres` — a custom kind carries no clauses (its contract is the bound package), and there is no `entities` table"
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

    /// A `KIND.md` header's `genres` key is present but is not an array of `[[genres]]`
    /// shape tables (`specs/architecture/15-kinds.md`, "genres (optional)").
    #[error("{path}: custom kind `{kind}` `genres` must be an array of `[[genres]]` shape tables")]
    #[diagnostic(code(temper::kind::genres_not_array))]
    GenresNotArray {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The kind whose genres array is malformed.
        kind: String,
    },

    /// A `[[genres]]` declaration is malformed — missing or mistyped its `name` string,
    /// or a non-string-array `leaves`/`collections`. A genre shape names itself and its
    /// declared prose-leaf and keyed-collection field vocabularies; any miss collapses
    /// into this one error naming its position, as [`BadRelationship`] does for an edge.
    ///
    /// [`BadRelationship`]: KindError::BadRelationship
    #[error(
        "{path}: custom kind `{kind}` `[[genres]]` #{index} must name a string `name` and, if present, string-array `leaves`/`collections`"
    )]
    #[diagnostic(code(temper::kind::bad_genre))]
    BadGenre {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The kind that owns the malformed genre shape.
        kind: String,
        /// The zero-based position of the malformed genre in declaration order.
        index: usize,
    },

    /// A `KIND.md` header's `format` or `unit_shape` key names a value outside its
    /// closed vocabulary — the same closed-algebra guard the extraction primitives
    /// carry (`specs/architecture/15-kinds.md`, "the adapter faces are declared"), applied to the
    /// declared adapter faces. Rejected at load, never silently coerced; a non-string
    /// value folds into this error too.
    #[error(
        "{path}: custom kind `{kind}` key `{key}` has out-of-vocabulary value `{value}` (expected {expected})"
    )]
    #[diagnostic(code(temper::kind::out_of_vocab))]
    OutOfVocab {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The kind whose adapter-face declaration is out of vocabulary.
        kind: String,
        /// The offending key — `format` or `unit_shape`.
        key: &'static str,
        /// The rejected value, as authored.
        value: String,
        /// The closed vocabulary that was expected, for the message.
        expected: &'static str,
    },

    /// A `KIND.md` header's `activation` declaration is structurally malformed — not an
    /// inline table, missing its `via` vocab entry, or a field-carrying variant
    /// (`description-trigger`/`paths-match`/`event`) missing the declared `field` string
    /// it ranges over. A `via` *value* outside the vocabulary is an [`OutOfVocab`] instead
    /// (`specs/architecture/15-kinds.md`); any structural miss collapses into this one error, as
    /// [`BadGoverns`] does for the locus.
    ///
    /// [`OutOfVocab`]: KindError::OutOfVocab
    /// [`BadGoverns`]: KindError::BadGoverns
    #[error(
        "{path}: custom kind `{kind}` `activation` must be a table naming a `via` vocab entry and, for the field-carrying variants, the declared `field` it ranges over"
    )]
    #[diagnostic(code(temper::kind::bad_activation))]
    BadActivation {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The custom kind with the malformed activation declaration.
        kind: String,
    },

    /// A `KIND.md` header's `provider` is present but is not a string. The provider
    /// names the authority defining the mirrored format (`specs/architecture/15-kinds.md`, "Decision:
    /// kind identity carries a provider axis"); *any* string is admissible — the
    /// vocabulary is the market's, not the parser's — so there is no closed set to guard,
    /// only the type, and a non-string folds into this one error.
    #[error("{path}: custom kind `{kind}` `provider` must be a string")]
    #[diagnostic(code(temper::kind::bad_provider))]
    BadProvider {
        /// The malformed `KIND.md`.
        path: PathBuf,
        /// The custom kind with the mistyped provider.
        kind: String,
    },

    /// A **bare** kind reference resolves to more than one kind — a provider collision
    /// (`specs/architecture/15-kinds.md`, "Decision: kind identity carries a provider axis"). A bare
    /// name resolves iff exactly one kind in the assembly carries it; two providers
    /// meeting under one bare name is a load error naming the qualified candidates, so
    /// the author qualifies the reference as `<provider>.<name>`. Nobody pays the
    /// qualification tax until two providers actually meet. Carries no `path` — the
    /// collision is over the assembly's whole kind set, not one `KIND.md`; the consuming
    /// binding (BINDING-QUALIFY) adds its own locus.
    #[error("bare kind reference `{name}` is ambiguous — candidates: {candidates}")]
    #[diagnostic(
        code(temper::kind::ambiguous_kind),
        help("qualify the reference as `<provider>.<name>` to name the kind you mean")
    )]
    AmbiguousKind {
        /// The bare name that resolves to more than one kind.
        name: String,
        /// The qualified candidates it collides across, comma-joined for the message.
        candidates: String,
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
    /// array — the `[kind.<name>.extraction]` declaration (`specs/architecture/15-kinds.md`).
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
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            // Genres are folded by [`CustomKind::extract`] after the primitives run — a
            // typed layer over `fenced_blocks`, needing the kind's declared genre set the
            // primitive-only `Extraction` does not hold. Empty here on purpose.
            genres: Vec::new(),
            // `satisfies` is a surface edge threaded through unchanged, not a
            // composed primitive, so a custom-kind member joins coverage exactly as
            // a built-in kind's does (`specs/architecture/10-contracts.md`).
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
        "directives" => Primitive::Directives {
            syntax: parse_directive_syntax(&str_param(table, "syntax", index, path)?, index, path)?,
        },
        "fenced" => Primitive::Fenced,
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

/// Resolve a `directives` primitive's `syntax` value against the closed per-syntax
/// vocabulary (`specs/architecture/15-kinds.md`). The sole member is `at-import`; any
/// other value is a [`KindError::UnknownDirectiveSyntax`], the closed-vocabulary
/// reject [`parse_primitive`] makes over the primitive discriminator applied to the
/// per-syntax face.
fn parse_directive_syntax(
    syntax: &str,
    index: usize,
    path: &Path,
) -> Result<DirectiveSyntax, KindError> {
    match syntax {
        "at-import" => Ok(DirectiveSyntax::AtImport),
        other => Err(KindError::UnknownDirectiveSyntax {
            path: path.to_path_buf(),
            index,
            syntax: other.to_string(),
        }),
    }
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
    /// (`specs/architecture/15-kinds.md`): line count, ATX headings, and file placement —
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
            source_path: PathBuf::from("specs/architecture/15-kinds.md"),
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

        // `placement` — the folder the unit sits under (the class directory).
        assert_eq!(features.source_dir.as_deref(), Some("architecture"));

        // A frontmatter-less kind composes no `field`, and body-mined references are
        // retired — nothing lands in `fields`.
        assert!(features.fields.is_empty());
    }

    #[test]
    fn re_running_the_extractor_is_byte_identical() {
        let extraction = spec_extraction();
        let unit = spec_unit();

        // Extraction is a pure function of the surface — the soundness boundary
        // (`specs/architecture/15-kinds.md`): the same unit yields the same features every run.
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
    fn a_fenced_primitive_parses_and_folds_block_interiors_into_features() {
        // `fenced` is a closed-vocab, parameterless primitive — it parses into
        // `Primitive::Fenced` and folds the body's fenced blocks into `fenced_blocks`,
        // each interior paired with its info string, surrounding prose skipped
        // (`specs/architecture/15-kinds.md`, "a fenced block — whose first consumer is
        // the genre fence").
        let extraction = Extraction::parse(
            "[[extraction]]\nprimitive = \"fenced\"\n",
            Path::new("temper.toml"),
        )
        .unwrap();
        assert_eq!(extraction.primitives(), &[Primitive::Fenced]);

        let body = "# Doc\n\nprose\n\n```toml genre.manifest\nname = \"x\"\n```\n";
        let unit = Unit {
            id: "doc".to_string(),
            frontmatter: BTreeMap::new(),
            body: body.to_string(),
            source_path: PathBuf::from("specs/architecture/15-kinds.md"),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
            published_requirements: Vec::new(),
        };
        let features = extraction.extract(&unit);
        assert_eq!(features.fenced_blocks.len(), 1);
        assert_eq!(features.fenced_blocks[0].info, "toml genre.manifest");
        assert_eq!(features.fenced_blocks[0].content, "name = \"x\"");
        // This extractor composes only `fenced` — every other locus stays at its
        // default (no headings extracted, no fields), the vacuous-composition floor.
        assert!(features.headings.is_empty());
        assert!(features.fields.is_empty());
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
        // Law 8 (`specs/intent/00-intent.md`; `specs/architecture/15-kinds.md`, "Decision: no
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
            "+++\n[provenance]\nsource_path = \"{source_path}\"\nsource_hash = \"deadbeef\"\n+++\n{body}"
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
        let dir = write_surface(
            &root,
            "15-kinds",
            "specs/architecture/15-kinds.md",
            "SPEC.md",
            body,
        );

        let unit = Unit::from_surface_dir(&dir).unwrap();
        let features = spec_extraction().extract(&unit);

        assert_eq!(features.id, "15-kinds");
        assert_eq!(features.body_lines, 3);
        assert_eq!(features.headings, vec!["Kinds".to_string()]);
        assert_eq!(features.source_dir.as_deref(), Some("architecture"));
        // The composed `spec` extractor mines no references — `fields` stays empty.
        assert!(features.fields.is_empty());
    }

    #[test]
    fn from_surface_dir_lifts_clause_fields_into_frontmatter() {
        // A custom member carrying `[clause.<field>]` header tables reloads with those
        // fields in `frontmatter` — the generic reader that closes the built-in/custom
        // asymmetry (`specs/architecture/20-surface.md`): a custom member's declared fields are the
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
source_path = \"specs/architecture/15-kinds.md\"\n\
import_hash = \"deadbeef\"\n\
+++\n\
# Kinds\n\nBody.\n";
        // The pre-rename `import_hash` key is deliberate: it exercises the provenance
        // reader's legacy-key fallback (LOCK-FRESHNESS-FACTS), the path the committed
        // `.temper/` dogfood still travels until a human `chore(harness):` re-emits it.
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
            "specs/intent/00-intent.md",
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

    /// Build a `CustomKind` off a `KIND.md`-shaped header, `governs` inline so the
    /// adapter-face keys sit at the top level regardless of order. Drives
    /// [`CustomKind::from_header`] without touching disk.
    fn kind_from_header(extra: &str) -> Result<CustomKind, KindError> {
        let src = format!("governs = {{ root = \"specs\", glob = \"*.md\" }}\n{extra}");
        let doc = src.parse::<DocumentMut>().unwrap();
        CustomKind::from_header(doc.as_table(), "spec", Path::new("kinds/spec/KIND.md"))
    }

    #[test]
    fn absent_format_and_unit_shape_keys_parse_as_none() {
        // The built-in-kind default — today's KIND.md carry neither, and absence is
        // valid: the fields stay `None`, nothing invented.
        let kind = kind_from_header("").unwrap();
        assert_eq!(kind.format, None);
        assert_eq!(kind.unit_shape, None);
    }

    #[test]
    fn a_declared_format_and_each_unit_shape_variant_parse_into_typed_fields() {
        let lone =
            kind_from_header("format = \"yaml-frontmatter\"\nunit_shape = \"file\"\n").unwrap();
        assert_eq!(lone.format, Some(Format::YamlFrontmatter));
        assert_eq!(lone.unit_shape, Some(UnitShape::File));

        // The other unit-shape variant — a directory with companions, id from the dir.
        let dir = kind_from_header("unit_shape = \"directory\"\n").unwrap();
        assert_eq!(dir.unit_shape, Some(UnitShape::Directory));
    }

    #[test]
    fn an_out_of_vocab_format_is_a_load_error() {
        // The closed vocabulary guards the projection face exactly as it guards the
        // extraction primitives: an unknown format is rejected at load, never coerced.
        let err = kind_from_header("format = \"toml-frontmatter\"\n").unwrap_err();
        assert!(matches!(
            err,
            KindError::OutOfVocab { key: "format", ref value, .. } if value == "toml-frontmatter"
        ));
    }

    #[test]
    fn an_out_of_vocab_unit_shape_is_a_load_error() {
        let err = kind_from_header("unit_shape = \"symlink\"\n").unwrap_err();
        assert!(matches!(
            err,
            KindError::OutOfVocab { key: "unit_shape", ref value, .. } if value == "symlink"
        ));
    }

    #[test]
    fn a_header_carrying_both_adapter_face_keys_no_longer_trips_unknown_key() {
        // Inbox sequencing (`FORMAT-KEY-PARSE`): before this entry a KIND.md carrying
        // `format`/`unit_shape` lines tripped `UnknownKey`. They now parse into typed
        // fields, so a human can add the two curated lines without turning checks red.
        let kind = kind_from_header("format = \"yaml-frontmatter\"\nunit_shape = \"directory\"\n")
            .unwrap();
        assert_eq!(kind.format, Some(Format::YamlFrontmatter));
        assert_eq!(kind.unit_shape, Some(UnitShape::Directory));
    }

    #[test]
    fn absent_activation_parses_as_none() {
        // Today's built-in KIND.md declare none — absence is valid, nothing invented.
        assert_eq!(kind_from_header("").unwrap().activation, None);
    }

    #[test]
    fn each_activation_vocab_entry_parses_into_its_typed_value() {
        // `always` carries no field — the edge is unconditional.
        let always = kind_from_header("activation = { via = \"always\" }\n").unwrap();
        assert_eq!(always.activation, Some(Activation::Always));

        // The three field-carrying variants name the declared frontmatter field they
        // range over, never a value.
        let desc = kind_from_header(
            "activation = { via = \"description-trigger\", field = \"description\" }\n",
        )
        .unwrap();
        assert_eq!(
            desc.activation,
            Some(Activation::DescriptionTrigger {
                field: "description".to_string()
            })
        );

        let paths = kind_from_header("activation = { via = \"paths-match\", field = \"paths\" }\n")
            .unwrap();
        assert_eq!(
            paths.activation,
            Some(Activation::PathsMatch {
                field: "paths".to_string()
            })
        );

        let event =
            kind_from_header("activation = { via = \"event\", field = \"event\" }\n").unwrap();
        assert_eq!(
            event.activation,
            Some(Activation::Event {
                field: "event".to_string()
            })
        );
    }

    #[test]
    fn an_out_of_vocab_activation_is_a_load_error() {
        // The closed vocabulary guards the activation face exactly as it guards format:
        // an unknown `via` is rejected at load, never silently dropped.
        let err = kind_from_header("activation = { via = \"cron\" }\n").unwrap_err();
        assert!(matches!(
            err,
            KindError::OutOfVocab { key: "activation", ref value, .. } if value == "cron"
        ));
    }

    #[test]
    fn a_field_carrying_activation_missing_its_field_is_a_load_error() {
        // `paths-match` names the declared field it ranges over — omitting it is
        // structurally malformed, never silently defaulted.
        let err = kind_from_header("activation = { via = \"paths-match\" }\n").unwrap_err();
        assert!(matches!(err, KindError::BadActivation { .. }));
    }

    #[test]
    fn a_header_carrying_activation_alongside_the_adapter_faces_no_longer_trips_unknown_key() {
        // Inbox sequencing (`ACTIVATION-KEY-PARSE`): before this entry a KIND.md carrying
        // an `activation` line tripped `UnknownKey`. It now parses into a typed field
        // beside format/unit_shape, so a human can add the curated activation line
        // without turning checks red.
        let kind = kind_from_header(
            "format = \"yaml-frontmatter\"\nunit_shape = \"directory\"\n\
activation = { via = \"paths-match\", field = \"paths\" }\n",
        )
        .unwrap();
        assert_eq!(kind.format, Some(Format::YamlFrontmatter));
        assert_eq!(kind.unit_shape, Some(UnitShape::Directory));
        assert_eq!(
            kind.activation,
            Some(Activation::PathsMatch {
                field: "paths".to_string()
            })
        );
    }

    #[test]
    fn a_provider_line_parses_inert_and_absence_is_none() {
        // Inbox sequencing (`PROVIDER-KEY-PARSE`): a KIND.md that mirrors an external
        // format declares the authority defining it — `provider = "claude-code"` parses
        // into the field rather than tripping `UnknownKey`, so a human can add the curated
        // line. A project's own kind mirrors nothing and stays bare.
        let claude = kind_from_header("provider = \"claude-code\"\n").unwrap();
        assert_eq!(claude.provider.as_deref(), Some("claude-code"));

        assert_eq!(kind_from_header("").unwrap().provider, None);
    }

    #[test]
    fn a_provider_is_any_string_not_a_closed_vocabulary() {
        // The vocabulary is the market's, not the parser's (`specs/architecture/15-kinds.md`): a
        // standard-authority provider is as admissible as a tool one, no closed set to
        // guard against.
        assert_eq!(
            kind_from_header("provider = \"agents-md\"\n")
                .unwrap()
                .provider
                .as_deref(),
            Some("agents-md")
        );
    }

    #[test]
    fn a_non_string_provider_is_a_load_error() {
        // Only the type is guarded — a non-string provider names no authority, folded
        // into `BadProvider` as `governs` folds its locus malformations.
        let err = kind_from_header("provider = 7\n").unwrap_err();
        assert!(matches!(err, KindError::BadProvider { .. }));
    }

    #[test]
    fn qualified_name_is_provider_dot_name_or_the_bare_name() {
        // A provider qualifies identity as `<provider>.<name>`; absent, identity is the
        // bare `name` — the qualification tax nobody pays until two providers meet.
        let qualified = kind_from_header("provider = \"claude-code\"\n").unwrap();
        assert_eq!(qualified.qualified_name(), "claude-code.spec");

        let bare = kind_from_header("").unwrap();
        assert_eq!(bare.qualified_name(), "spec");
    }

    #[test]
    fn resolve_bare_returns_the_unique_match_or_none() {
        // A bare name resolves iff exactly one kind carries it. The set here holds two
        // distinct bare names, so `spec` resolves to its lone `claude-code`-qualified kind.
        let claude_spec = kind_from_header("provider = \"claude-code\"\n").unwrap();
        let mut cursor_rule = kind_from_header("provider = \"cursor\"\n").unwrap();
        cursor_rule.name = "rule".to_string();
        let kinds = vec![claude_spec, cursor_rule];

        let resolved = CustomKind::resolve_bare("spec", &kinds).unwrap();
        assert_eq!(
            resolved.map(CustomKind::qualified_name).as_deref(),
            Some("claude-code.spec")
        );

        // A bare name no kind carries resolves to `None`, not an error — an
        // unknown-reference is the consuming binding's concern (BINDING-QUALIFY).
        assert!(CustomKind::resolve_bare("hook", &kinds).unwrap().is_none());
    }

    #[test]
    fn resolve_bare_reports_a_collision_naming_the_qualified_candidates() {
        // Two providers meeting under one bare name is the collision the Decision requires
        // as a load error — `AmbiguousKind` names the qualified candidates so the author
        // knows what to qualify against.
        let claude_skill = kind_from_header("provider = \"claude-code\"\n").unwrap();
        let cursor_skill = kind_from_header("provider = \"cursor\"\n").unwrap();
        // Both are named `spec` by the shared header helper — the collision under test.
        let kinds = vec![claude_skill, cursor_skill];

        let err = CustomKind::resolve_bare("spec", &kinds).unwrap_err();
        match err {
            KindError::AmbiguousKind { name, candidates } => {
                assert_eq!(name, "spec");
                assert!(candidates.contains("claude-code.spec"));
                assert!(candidates.contains("cursor.spec"));
            }
            other => panic!("expected AmbiguousKind, got {other:?}"),
        }
    }
}
