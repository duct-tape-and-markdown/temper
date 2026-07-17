//! The extraction algebra — a kind's read side, composed from data.
//!
//! Where `crate::contract` is the engine's predicate half
//! (what an artifact must satisfy), this is the extraction half (what it *is*,
//! and how it is read). Extraction is the soundness boundary: a clause is sound
//! only if its feature is deterministically extractable, so a kind carries no
//! code of its own — its extractor is composed from a closed algebra of
//! deterministic [`Primitive`]s, authored as plain Rust data (there is no kind
//! file format to parse it from — "Decision: field typing lives in the SDK").
//!
//! Every primitive delegates to the same surface extractor the built-in
//! projectors use (`crate::extract`), so the soundness boundary is one boundary,
//! not a forked implementation that can drift.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;

use crate::compose::Edge;
use crate::drift::{KindFactRow, LayoutRow, LockRowError, TemplateRow};
use crate::extract::{self, Features};

/// The file locus a custom kind reads: the root
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

/// A file locus's declared **commitment class** — whether the documents its members own
/// are committed beside the kind that declares them. Absent from a kind is the default
/// and only other class: committed, the whole shipped set's posture, where the kind and
/// its members' documents are reviewed together and `emit` owns the bytes.
///
/// A closed vocabulary, the same load-time guard [`Format`] and [`UnitShape`] carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Commitment {
    /// `local` — per-machine and uncommitted: the kind is declared and reviewed, its
    /// members' documents are not. Two consequences ride the class:
    ///
    /// - the locus is **read-side only** — the document is the governed source, read at
    ///   check in place under whatever format the kind declares, and never an `emit`
    ///   input or target, because emit's codomain is the committed tree;
    /// - its members' rows never enter the lock, deriving at read time instead, so the
    ///   committed bytes stay layer-invariant by construction rather than by a rule
    ///   something has to remember to enforce.
    Local,
}

impl Commitment {
    /// The declared label this class lifts from — the inverse of
    /// [`commitment_from_label`], so a diagnostic names the class in the same vocabulary
    /// the author declared it in.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Commitment::Local => "local",
        }
    }
}

/// A kind's declared definition — a constructor plus its runtime residue: the
/// [`governs`](CustomKind::governs) locus, the composed [`Extraction`], the declared
/// [`relationships`](CustomKind::relationships), and the declared shape facts
/// ([`format`](CustomKind::format), [`unit_shape`](CustomKind::unit_shape),
/// [`registration`](CustomKind::registration), [`templates`](CustomKind::templates),
/// [`content`](CustomKind::content)). Every kind is Rust data — a built-in is authored
/// directly in [`crate::builtin_kind`]; there is no `KIND.md` file format to parse it from.
///
/// A custom kind is purely declare-side — it carries no clauses. Predicates over
/// its members ride the assembly's `expect`/`require` clauses.
///
/// Not `Eq`: keeping the derive `PartialEq` leaves room for future `f64`-bearing
/// fields without churn, as it does for [`Clause`](crate::contract::Clause).
#[derive(Debug, Clone, PartialEq)]
pub struct CustomKind {
    /// The kind's bare name — the `[kind.<name>]` registration key, and the
    /// surface subdirectory/member-document convention key
    /// ([`member_document`](CustomKind::member_document)).
    pub name: String,
    /// The file locus the kind reads. [`None`] for a **nested file** kind: its members'
    /// paths compose from their host's unit and the host template's pattern, so it governs
    /// no glob of its own — nothing discovers it at one, and it contends with no other
    /// kind's locus.
    pub governs: Option<Governs>,
    /// The composed extractor over the closed algebra,
    /// authored via [`Extraction::new`]. An empty primitive set is the vacuous
    /// extractor (only the intrinsic id).
    pub extraction: Extraction,
    /// The declared relationships — which of the kind's references are edges,
    /// each an [`Edge`] whose `from` is this kind.
    /// Absent ⇒ empty (the default [`CustomKind::new`] leaves it at).
    pub relationships: Vec<Edge>,
    /// The declared projection format — how a member's on-disk artifact is shaped.
    /// A closed vocabulary; absent ⇒ `None`. Each built-in frontmatter kind declares
    /// [`Format::YamlFrontmatter`]; a relocation row that diverges on it is a namespace
    /// collision, decided by `row_relocates_builtin`.
    pub format: Option<Format>,
    /// The declared unit shape — whether a member is a lone file (id from the stem),
    /// a directory with companions (id from the directory name), or a lone file whose
    /// id is read from a declared field (an agent's `name`).
    /// A closed enum; absent ⇒ `None`. Read by the surface loaders
    /// ([`crate::frontmatter`], [`crate::json_manifest`]) to derive a member's id from
    /// its unit.
    pub unit_shape: Option<UnitShape>,
    /// The declared registration — the kind's world fact: the **set** of documented
    /// channels a member reaches the world over (user invocation and description
    /// trigger are channels, not rivals; `builtins.md`, "The shipped kinds"). A closed
    /// per-channel vocabulary; empty ⇒ no declared registration (today's built-in kinds
    /// each declare at least one). Read by the reachability walk ([`crate::graph`]) to
    /// decide a member's world edge is live iff any one channel is.
    pub registration: Vec<Registration>,
    /// The kind's declared **templates** — one per inner layer of nested members it
    /// hosts: the child kind it nests, plus the path pattern that layer's children sit
    /// at when they are files, a declared fact carried
    /// through the lock's own [`KindFactRow::templates`]. A host's *actual* embedded
    /// members are resolved independently, off `Declarations::nested_members` by
    /// address (`builtin_kind::features`); any predicate over a nested member's
    /// interior rides the assembly's `expect`/`require` clauses. Absent ⇒ empty.
    pub templates: Vec<Template>,
    /// The kind's declared **content** — whether a member's body is one verbatim prose
    /// value ([`Content::File`], the default), a declared [`Layout`] over the body's
    /// heading tree, or [`Content::Fields`] (no body slot at all). Absent from the row
    /// reads as [`Content::File`].
    pub content: Content,
    /// The kind's declared **collection address** — for a registration member surfacing
    /// inside a host manifest (a hook, an MCP server), which manifest and which key path
    /// it registers at. `None` for a kind that owns its own file locus. Read back off the
    /// row's `collection_address` column.
    pub collection_address: Option<CollectionAddress>,
    /// The file locus's declared **commitment class** — [`Commitment::Local`] for a
    /// per-machine, uncommitted locus; absent ⇒ committed, the class every shipped kind
    /// takes. Meaningless without a [`governs`](CustomKind::governs) locus, the one fence
    /// [`local_locus_fault`](CustomKind::local_locus_fault) states; admissible over any
    /// declared content and format. Read back off the row's `commitment` column.
    pub commitment: Option<Commitment>,
}

/// A kind's declared **content** — how a member's authored body is shaped. The body is
/// one verbatim prose value ([`File`](Content::File), the default every built-in takes),
/// a declared [`Layout`] over the body's heading tree, or nothing at all
/// ([`Fields`](Content::Fields) — a registration member has fields and edges but no prose
/// to fill). An absent row column reads as `File`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    /// `file` — the body is one verbatim prose value, copied byte-for-byte.
    File,
    /// `layout` — the body is a declared template over its heading tree.
    Layout(Layout),
    /// `fields` — no body slot: the member is its typed fields and edges, nothing more
    /// (a hook, an MCP server). Distinct from [`File`](Content::File), which still carries
    /// a verbatim prose body.
    Fields,
}

impl Content {
    /// The declared label this content shape lifts from, so a diagnostic names it in the
    /// same vocabulary the author declared it in — [`Format::label`]'s peer.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Content::File => "file",
            Content::Layout(_) => "layout",
            Content::Fields => "fields",
        }
    }
}

/// A registration member's declared **collection address** — where inside a host manifest
/// its registration surfaces. A hook registers under its lifecycle event in
/// `settings.json`'s `hooks`; an MCP server registers by name under `.mcp.json`'s
/// `mcpServers`. The manifest is a free path; the [`key_path`](CollectionAddress::key_path)
/// is a closed vocabulary, the same load-time reject [`Format`]/[`UnitShape`]/
/// [`Registration`] carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionAddress {
    /// The host manifest the registration surfaces in (`settings.json`, `.mcp.json`), a
    /// path relative to the harness.
    pub manifest: String,
    /// The key path within the manifest the member's registration keys at.
    pub key_path: CollectionKeyPath,
}

/// A collection address's **key path** — the closed vocabulary of manifest key paths a
/// registration member surfaces at. Any other wire label is a load error, the same
/// closed-vocabulary guard [`Format`]/[`UnitShape`]/[`Registration`] carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionKeyPath {
    /// `hooks.<Event>` — a hook keys under its lifecycle event in the manifest's `hooks`
    /// map (`settings.json`).
    HooksEvent,
    /// `mcpServers.*` — an MCP server keys by name under the manifest's `mcpServers` map
    /// (`.mcp.json`).
    McpServers,
    /// `enabledPlugins.*` — an installed plugin keys by its `<plugin>@<marketplace>`
    /// identity under the manifest's `enabledPlugins` map (`settings.json`).
    EnabledPlugins,
}

/// The declared field an `enabledPlugins` entry's **scalar value** surfaces under — the
/// one home the synthesized name lives at, so the read face that names it
/// ([`crate::extract::manifest_members`]) and the liveness gate that reads it
/// ([`crate::graph`]) cannot drift apart. The wire carries the value bare (`"foo@bar":
/// true`), so unlike a hook's `event` this field names no key of the manifest's own; it
/// is the member's one declared field, and its documented semantics — `false` is a plugin
/// the harness does not load — gate the member's channel outright
/// (`code.claude.com/docs/en/plugins-reference`, retrieved 2026-07-16).
pub(crate) const ENABLEMENT_FIELD: &str = "enabled";

impl CollectionKeyPath {
    /// The manifest's **top-level collection key** this key path walks into — the object
    /// whose entries are the registration members. `hooks.<Event>` reads the `hooks`
    /// object; `mcpServers.*` the `mcpServers` object. The one place the wire key paths
    /// map to their manifest object, so the adapter's walk (`crate::json_manifest`) names
    /// no literal of its own.
    #[must_use]
    pub fn collection_key(self) -> &'static str {
        match self {
            CollectionKeyPath::HooksEvent => "hooks",
            CollectionKeyPath::McpServers => "mcpServers",
            CollectionKeyPath::EnabledPlugins => "enabledPlugins",
        }
    }

    /// The field a member's own **collection key** surfaces under, when the key path names
    /// one — the `<Event>` in `hooks.<Event>` is the member's lifecycle-event *field*
    /// (`event`), so a member read at that address carries its event as a checkable field a
    /// clause can range over. `mcpServers.*` and `enabledPlugins.*` name no such field:
    /// each key is its member's identity — a server's name, a plugin's
    /// `<plugin>@<marketplace>` — not a field the member carries alongside one. `None`
    /// leaves the read surfacing only the entry's own fields.
    #[must_use]
    pub fn key_field(self) -> Option<&'static str> {
        match self {
            CollectionKeyPath::HooksEvent => Some("event"),
            CollectionKeyPath::McpServers | CollectionKeyPath::EnabledPlugins => None,
        }
    }

    /// Whether the manifest this collection canonically owns carries **nothing but** this
    /// collection — so modelling it covers the whole file. `.mcp.json` is wholly its
    /// `mcpServers` map (code.claude.com/docs/en/mcp, retrieved 2026-07-10), so an
    /// `mcp-server` kind governs the file outright; `settings.json` carries permissions,
    /// env, and more alongside its `hooks`, so a `hook` kind covers only that one segment
    /// and the container stays unmodeled until every segment is (code.claude.com/docs/en/settings,
    /// retrieved 2026-07-10). The coverage note reads this to decide whether a manifest
    /// kind retires its host file's `coverage.unmodeled-surface` finding.
    #[must_use]
    pub fn spans_whole_manifest(self) -> bool {
        match self {
            CollectionKeyPath::McpServers => true,
            CollectionKeyPath::HooksEvent | CollectionKeyPath::EnabledPlugins => false,
        }
    }
}

/// A declared **layout** — the ordered regions a `layout`-content kind's body is read as,
/// each one of the three corpus primitives ([`LayoutRegion`]). The regions are the
/// declared template; matching them against a member's actual heading tree is the
/// reader's job, not this fact's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    /// The layout's regions, in declared document order.
    pub regions: Vec<LayoutRegion>,
}

/// One region of a [`Layout`] — a single corpus primitive over the body's heading tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutRegion {
    /// **prose** — a verbatim span, or, when `import` is declared, a reference resolving
    /// to a file's contents (fingerprinted, refusing when dangling — the resolver's job).
    Prose {
        /// The import reference this region resolves from, when it imports a file's
        /// contents rather than carrying its own verbatim words.
        import: Option<String>,
    },
    /// **field section** — a heading whose span fills the named field `slot`.
    Field {
        /// The field slot the heading's span fills.
        slot: String,
    },
    /// **member collection** — a heading whose child headings are each one member of the
    /// named kind; a member's identity is its slugged child heading, or `key` when an
    /// explicit one is declared (surviving a retitle).
    Collection {
        /// The child member kind each child heading instantiates.
        member_kind: String,
        /// An explicit identity key overriding the slugged heading, when declared.
        key: Option<String>,
    },
}

/// The typed reading of a `layout`-content document: what emit derives its
/// declaration rows from and the gate reads its fields off, all off the one authored
/// source. Field sections fill named [`fields`](LayoutReading::fields); member
/// collections yield [`members`](LayoutReading::members) carrying slugged-heading (or
/// explicit-key) identity; verbatim prose regions land in
/// [`prose`](LayoutReading::prose), in document order. The document is never written
/// back — it is the source, read (`pipeline.md`, "Emit").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LayoutReading {
    /// Each field section's named slot filled by its heading's verbatim span.
    pub fields: BTreeMap<String, String>,
    /// Each **edge** section's slot mapped to its parsed address entries, in document
    /// order. A field section whose slot the kind marks as an edge field carries
    /// addresses, not a verbatim span, so its entries land here rather than in
    /// [`fields`](LayoutReading::fields) — `satisfies` among them.
    pub edges: BTreeMap<String, Vec<String>>,
    /// Each member collection's members, in document order — one per child heading of
    /// a collection heading.
    pub members: Vec<LayoutMember>,
    /// Each verbatim prose region's span, in document order (an importing prose region
    /// is out of this reader's scope and lands empty).
    pub prose: Vec<String>,
}

/// One member read off a layout document's member collection: its child kind, its
/// identity (the slugged heading, or the explicit key when the collection declares
/// one — surviving a retitle of the heading), the authored heading it was read from,
/// and its own leaves (the immediate deeper sub-headings' spans, keyed by slug).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMember {
    /// The child kind this member instantiates — the collection region's `member_kind`.
    pub member_kind: String,
    /// The member's identity: the slugged child heading, or the explicit key's value
    /// when the collection declares a `key` (stable across a heading retitle).
    pub key: String,
    /// The authored child heading this member was read from — its identity source when
    /// no explicit key overrides, kept so a rename surfaces as a move.
    pub heading: String,
    /// The member's own prose leaves — its immediate deeper sub-headings' spans, keyed
    /// by the slug of each sub-heading.
    pub leaves: BTreeMap<String, String>,
}

/// The failures a layout read surfaces loud — each naming the file and the heading at
/// fault, never a degraded empty reading (invariant 6: loud or nothing). An unfilled
/// region is not among them: a region states what may appear, never what must, so it
/// reads empty. What refuses is malformed structure the reader cannot place.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum LayoutError {
    /// A collection declares an explicit identity `key`, but a member carries no
    /// sub-heading of that name to read the key from.
    #[error(
        "{path}: collection member `{heading}` has no `{key}` sub-heading for its explicit key"
    )]
    #[diagnostic(code(temper::layout::missing_key))]
    MissingKey {
        /// The layout document at fault.
        path: PathBuf,
        /// The member heading missing the key sub-heading.
        heading: String,
        /// The declared key sub-heading name.
        key: String,
    },

    /// The document carries a top-level heading the layout's regions never admit —
    /// structure no primitive covers (`representation.md`: "two kinds, or it is prose").
    #[error("{path}: heading `{heading}` fits no declared layout region")]
    #[diagnostic(code(temper::layout::unadmitted))]
    Unadmitted {
        /// The layout document at fault.
        path: PathBuf,
        /// The unadmitted heading.
        heading: String,
    },
}

/// The framework edge field every kind carries — the fill-edge key whose entries name
/// requirements. A layout field section on this slot is always an edge slot, alongside
/// any the kind's own declared [`relationships`](CustomKind::relationships) name.
pub const SATISFIES_EDGE_FIELD: &str = "satisfies";

impl Layout {
    /// Read `body` under this declared layout, off the document at `source_path` (named
    /// only in diagnostics). The regions map in document order onto the body's top-level
    /// headings ([`extract::body_heading_tree`]): a field section fills its slot with
    /// the heading's verbatim span — unless the slot is one of `edge_fields`, when its
    /// entries are parsed as addresses into [`edges`](LayoutReading::edges) instead — a
    /// member collection turns each child heading into a member, and a verbatim prose
    /// region takes the document preamble. A region the document has no heading for reads
    /// empty — a region states what may appear, never what must — so one layout serves a
    /// tree ranging from prose-only to fully membered. An explicit key with no sub-heading,
    /// or a top-level heading no region admits, still refuses loud ([`LayoutError`]).
    ///
    /// # Errors
    ///
    /// Returns a [`LayoutError`] naming the file and heading when the document carries
    /// structure the declared layout cannot place.
    pub fn read(
        &self,
        body: &str,
        source_path: &Path,
        edge_fields: &BTreeSet<String>,
    ) -> Result<LayoutReading, LayoutError> {
        let tree = extract::body_heading_tree(body);
        let mut reading = LayoutReading::default();
        // The cursor into the document's top-level headings the field/collection regions
        // consume in order; a preamble taken once by the first verbatim prose region.
        let mut cursor = 0;
        let mut preamble_taken = false;
        for region in &self.regions {
            match region {
                LayoutRegion::Prose { import } => {
                    // An importing prose region resolves elsewhere (LAYOUT-PROSE-IMPORT);
                    // a verbatim one takes the document preamble, once. The blank lines
                    // that separate regions are structure, trimmed off the captured span.
                    let span = if import.is_none() && !preamble_taken {
                        preamble_taken = true;
                        extract::body_preamble(body).trim().to_string()
                    } else {
                        String::new()
                    };
                    reading.prose.push(span);
                }
                LayoutRegion::Field { slot } => {
                    // No heading left for the slot: it reads absent, not loud.
                    let Some(node) = next_heading(&tree, &mut cursor) else {
                        continue;
                    };
                    if edge_fields.contains(slot) {
                        reading
                            .edges
                            .insert(slot.clone(), parse_edge_entries(&node.body));
                    } else {
                        reading
                            .fields
                            .insert(slot.clone(), node.body.trim().to_string());
                    }
                }
                LayoutRegion::Collection { member_kind, key } => {
                    // No heading left for the collection: it reads with zero members.
                    let Some(node) = next_heading(&tree, &mut cursor) else {
                        continue;
                    };
                    for child in &node.children {
                        reading.members.push(read_collection_member(
                            child,
                            member_kind,
                            key,
                            source_path,
                        )?);
                    }
                }
            }
        }
        // Every top-level heading the regions did not consume is structure no primitive
        // admits — loud, never silently dropped.
        if let Some(extra) = tree.get(cursor) {
            return Err(LayoutError::Unadmitted {
                path: source_path.to_path_buf(),
                heading: extra.heading.clone(),
            });
        }
        Ok(reading)
    }
}

/// The next unconsumed top-level heading a field/collection region binds to, advancing
/// the cursor — or `None` when the document has run out of headings, leaving the region
/// to read empty.
fn next_heading<'a>(
    tree: &'a [extract::HeadingNode],
    cursor: &mut usize,
) -> Option<&'a extract::HeadingNode> {
    let node = tree.get(*cursor)?;
    *cursor += 1;
    Some(node)
}

/// Read one collection member off its child heading `node`: its identity is the
/// slugged heading, or — when the collection declares an explicit `key` — the slug of
/// the value under the member's `key` sub-heading, which a heading retitle leaves
/// untouched. Its leaves are the member's immediate sub-headings' spans, keyed by slug.
fn read_collection_member(
    node: &extract::HeadingNode,
    member_kind: &str,
    key: &Option<String>,
    source_path: &Path,
) -> Result<LayoutMember, LayoutError> {
    let mut leaves = BTreeMap::new();
    for child in &node.children {
        leaves.insert(slugify(&child.heading), child.body.trim().to_string());
    }
    let identity = match key {
        None => slugify(&node.heading),
        Some(key) => {
            let slug = slugify(key);
            let value = leaves.get(&slug).ok_or_else(|| LayoutError::MissingKey {
                path: source_path.to_path_buf(),
                heading: node.heading.clone(),
                key: key.clone(),
            })?;
            slugify(value)
        }
    };
    Ok(LayoutMember {
        member_kind: member_kind.to_string(),
        key: identity,
        heading: node.heading.clone(),
        leaves,
    })
}

/// Parse an edge slot's heading span into its ordered address entries: one entry per
/// non-empty line, a leading markdown list marker (`-`/`*`/`+`) and surrounding
/// whitespace stripped. The authored form a `satisfies` (or other edge-field) section
/// lists its targets as — bare requirement names for `satisfies`, `kind:name` addresses
/// for a declared relationship.
fn parse_edge_entries(span: &str) -> Vec<String> {
    span.lines()
        .map(|line| line.trim().trim_start_matches(['-', '*', '+']).trim())
        .filter(|entry| !entry.is_empty())
        .map(str::to_string)
        .collect()
}

/// Slugify a heading (or explicit-key value) into a stable identity token: ASCII
/// alphanumerics lower-cased, every other run collapsed to a single `-`, leading and
/// trailing `-` trimmed. The identity a member collection keys on when no explicit key
/// overrides — `## Baked projection` → `baked-projection`.
fn slugify(text: &str) -> String {
    let mut out = String::new();
    let mut pending_sep = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_sep && !out.is_empty() {
                out.push('-');
            }
            pending_sep = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            pending_sep = true;
        }
    }
    out
}

/// A kind's declared **projection format** — the closed vocabulary naming how a
/// member's on-disk artifact is shaped. The engine implements each format once,
/// generically, and a file kind's declared entry decides which adapter reads its
/// artifact. Any value outside the vocabulary is a load error, the same
/// closed-vocabulary guard the extraction primitives carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// `yaml-frontmatter` — YAML frontmatter over a markdown body, the Claude Code
    /// family's shape.
    YamlFrontmatter,
    /// `json-document` — the whole artifact is one JSON object, its top-level keys the
    /// member's own fields and its identity a declared key among them. A member owns the
    /// document rather than surfacing in one collection of it, which is what separates
    /// this format from a [`CollectionAddress`]-carrying manifest kind.
    JsonDocument,
    /// `toml-document` — the whole artifact is one TOML table, its top-level keys the
    /// member's own fields and its identity a declared key among them: the
    /// [`JsonDocument`](Format::JsonDocument) read over a second grammar. A **read face
    /// only** — no write twin exists, so a member of a kind declaring it is read and gated
    /// in place and never projected.
    TomlDocument,
}

impl Format {
    /// The declared label this format lifts from — the inverse of
    /// [`format_from_label`], so a diagnostic names the format in the same
    /// vocabulary the author declared it in.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Format::YamlFrontmatter => "yaml-frontmatter",
            Format::JsonDocument => "json-document",
            Format::TomlDocument => "toml-document",
        }
    }
}

/// A kind's declared **unit shape** — the format fact that varies per kind:
/// whether a member's on-disk artifact is a lone file, its
/// identity the filename stem; a directory with companions, its identity the
/// directory name; or a lone file whose identity is read from a declared field
/// rather than derived from the path. A closed enum; any other
/// value is a load error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitShape {
    /// `file` — a lone file; the member's id is its filename stem (a rule's
    /// `.claude/rules/rust.md`).
    File,
    /// `directory` — a directory with companions; the member's id is the directory
    /// name (a skill's `.claude/skills/<name>/SKILL.md`).
    Directory,
    /// `named-field` — a lone file whose id is read from a declared field, not the
    /// filename (an agent's frontmatter `name`, a JSON document's top-level `name`; any
    /// containing subdirectory is purely organizational). The field is read from whichever
    /// surface the kind's [`Format`] carries its fields on.
    NamedField {
        /// The declared field the id is read from.
        field: String,
    },
    /// `starred-segment` — a lone file whose id is the directory segment its `*/<file>`
    /// glob stars, not its filename stem (a `conventions.md` keyed by its parent folder).
    /// Distinct from [`Directory`](UnitShape::Directory): that shape owns the directory and
    /// hosts its templated companions, whereas this file borrows the segment for identity
    /// only, so it coexists inside another kind's directory (a `conventions.md` sitting in a
    /// skill's directory, which `skill` owns).
    StarredSegment,
}

/// A kind's declared registration — one **channel** among the inbound boundary edges
/// of the relation graph: one documented way the harness reaches a
/// member, per-kind mechanics over per-member data. A closed vocabulary harvested from the kinds
/// temper ships; any other value is a load error, the same closed-vocabulary guard
/// [`Format`] and [`UnitShape`] carry. The three field-carrying variants name the
/// declared frontmatter field they range over, never a value — the glob/description
/// *values* stay the member's ordinary clauses. A kind declares a **set** of these
/// ([`CustomKind::registration`]) — user invocation and description trigger are
/// channels, not rivals (`builtins.md`, "The shipped kinds"). Inert until REACHABILITY
/// reads the set to decide a member's world edge is live iff any one channel is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Registration {
    /// `always` — loaded at launch, unconditionally (a rule without `paths`;
    /// `CLAUDE.md` itself). Carries no field: the edge is unconditional.
    Always,
    /// `user-invoked` — the member is directly invocable by name (a skill's `/name`).
    /// Carries no field: no repo-decidable criterion names this channel dead, mirroring
    /// [`Always`](Registration::Always) — a member's `user-invocable` modulating field
    /// is an ordinary declared field, not part of this channel's identity.
    UserInvoked,
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
        /// The declared frontmatter field carrying the registration glob.
        field: String,
    },
    /// `event(field)` — the member executes at a named lifecycle event (carried for the
    /// future `hook` kind). The field names the declared lifecycle-event field.
    Event {
        /// The declared frontmatter field naming the lifecycle event.
        field: String,
    },
    /// `connection` — the member reaches the world by the harness connecting to it (an MCP
    /// server temper models off `.mcp.json`). Carries no field: whether a connection
    /// succeeds is a runtime fact temper cannot decide, so like [`Always`](Registration::Always)
    /// the channel is never provably dead.
    Connection,
    /// `enablement` — the member reaches the world by the harness *enabling* it: an entry
    /// in `settings.json`'s `enabledPlugins` map (an installed plugin). Carries no field —
    /// the entry's own presence IS the channel, exactly as [`Connection`](Registration::Connection)'s
    /// is. Unlike a connection, this channel is repo-decidable: the member's declared
    /// [`ENABLEMENT_FIELD`] carries the entry's documented `false`, which gates the
    /// member outright (`builtins.md`, "The shipped kinds"). The gate rides that field,
    /// never a second channel entry.
    Enablement,
}

/// A **template** a kind declares for one inner layer of nested members it hosts: the
/// child kind, plus the path pattern its children sit at when they are files
/// (`specs/model/representation.md`, "kind"), serialized whole into the lock. Any
/// *predicate* over a nested member's interior rides the assembly's `expect`/`require`
/// clauses, **out of the kind object** — the same ownership line extraction and contract
/// split on everywhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    /// The child kind — for an embedded layer the `member.<kind>` a fence info string
    /// carries (`member.decision surface-authority` → `decision`); for a file layer the
    /// kind each child's own unit is read as. Either way the kind's own declared nesting
    /// fact. A host's actual [`EmbeddedMember`](crate::extract::EmbeddedMember)s are
    /// resolved independently, off `Declarations::nested_members` by address.
    pub kind: String,
    /// The path pattern a file child's unit sits at, relative to the parent's unit.
    /// [`None`] for an embedded layer, whose children live in the host's body and own no
    /// unit. Both faces compose against it: emit writes a child's projection under its
    /// host's unit at this pattern, and discovery classifies an adopted harness's matching
    /// files there as that host's children — the pattern's one home is the host, so the
    /// child kind governs no glob of its own.
    pub path: Option<String>,
}

impl CustomKind {
    /// Construct a kind's declared definition directly — the constructor a built-in
    /// (`crate::builtin_kind`) or a future SDK-authored custom kind supplies its five
    /// facts through. There is no file format to load it from:
    /// every field here is plain Rust data, set by
    /// the caller rather than parsed.
    #[must_use]
    pub fn new(name: impl Into<String>, governs: Governs, extraction: Extraction) -> Self {
        Self::with_locus(name, Some(governs), extraction)
    }

    /// Construct a **nested file** kind's declared definition — the host-composed spelling
    /// of the locus: its members own files whose paths compose from their host member's
    /// unit and the host kind's template pattern, so the kind governs no glob and nothing
    /// discovers it at one.
    #[must_use]
    pub fn nested_file(name: impl Into<String>, extraction: Extraction) -> Self {
        Self::with_locus(name, None, extraction)
    }

    /// The shared body of the two constructors — a kind's definition at either locus, its
    /// remaining facts at their declare-nothing defaults.
    fn with_locus(
        name: impl Into<String>,
        governs: Option<Governs>,
        extraction: Extraction,
    ) -> Self {
        Self {
            name: name.into(),
            governs,
            extraction,
            relationships: Vec::new(),
            format: None,
            unit_shape: None,
            registration: Vec::new(),
            templates: Vec::new(),
            content: Content::File,
            collection_address: None,
            commitment: None,
        }
    }

    /// Declare this kind's file locus **local** — per-machine and uncommitted
    /// ([`Commitment::Local`]). The class is the locus's fact, so a kind that governs no
    /// glob has none to declare; [`local_locus_fault`](CustomKind::local_locus_fault)
    /// states that fence rather than this setter refusing it, so a malformed declaration
    /// surfaces as one loud finding at the gate instead of two different failures
    /// depending on which door it came through.
    #[must_use]
    pub fn local(mut self) -> Self {
        self.commitment = Some(Commitment::Local);
        self
    }

    /// Why this kind's declared commitment class is inadmissible, or [`None`] when it is
    /// sound — the class is a *file* locus's fact, and a nested-file kind governs no glob
    /// of its own, so it has no locus to class.
    ///
    /// That locus fence is the whole of it. The class rules the *read* side — the document
    /// is the governed source, read in place under whatever format the kind declares, and
    /// never an emit input or target — so no content or format is inadmissible under it:
    /// the property carrying the trust story is read-never-written, and every declared
    /// read face serves it.
    #[must_use]
    pub fn local_locus_fault(&self) -> Option<String> {
        if self.commitment != Some(Commitment::Local) {
            return None;
        }
        if self.governs.is_none() {
            return Some(
                "it declares no `governs` locus — a commitment class is a file locus's \
                 fact, and a nested file kind's members compose their paths from their \
                 host's unit instead"
                    .to_string(),
            );
        }
        None
    }

    /// Reconstruct a kind's declared definition from the committed lock's own
    /// [`KindFactRow`]: the row's five-fact residue lifts into `governs`/`format`/
    /// `unit_shape`/`registration` directly. The reconstructed extractor stays the
    /// same generic markdown-structure set every built-in composes
    /// (`headings`/`sections`/`line_count`/`placement`); a floor clause's own `field`
    /// column, plus the permissive frontmatter fold every custom member's extraction
    /// already runs through (`crate::builtin_kind::features`), is what actually ranges
    /// over a custom member's declared fields — never a per-kind `Field` primitive list.
    ///
    /// The row's `templates` column lifts into one [`Template`] per declared inner layer
    /// — child kind plus a file layer's path pattern; the kind's own declared nesting
    /// fact. A host's actual
    /// embedded members are resolved independently, off `Declarations::nested_members`
    /// by address (`crate::builtin_kind::features`), so the reconstructed extraction
    /// needs no `Fenced` primitive of its own to serve them.
    ///
    /// # Errors
    ///
    /// Returns a [`LockRowError`] when a `format`/`unit_shape`/`registration`/`shape`/
    /// `collection_address` label or a layout region falls outside its closed vocabulary —
    /// the tool-written lock carries only labels the SDK could emit, so an unknown one is
    /// corruption rejected at load.
    pub fn from_kind_fact_row(row: &KindFactRow) -> Result<Self, LockRowError> {
        Ok(CustomKind {
            format: format_from_row(row)?,
            unit_shape: match &row.unit_shape {
                Some(label) => Some(kind_vocab(
                    label,
                    "unit_shape",
                    unit_shape_from_label(label),
                )?),
                None => None,
            },
            registration: row
                .registration
                .iter()
                .map(|label| kind_vocab(label, "registration", registration_from_label(label)))
                .collect::<Result<Vec<_>, _>>()?,
            templates: row.templates.iter().map(template_from_row).collect(),
            content: content_from_row(row)?,
            collection_address: collection_address_from_row(row)?,
            commitment: commitment_from_row(row)?,
            ..CustomKind::with_locus(
                row.name.clone(),
                // The two columns are one spelling: an `at` locus writes both, a nested file
                // kind neither — so the row is never mined for a root+glob it does not carry.
                row.governs_root
                    .clone()
                    .zip(row.governs_glob.clone())
                    .map(|(root, glob)| Governs { root, glob }),
                Extraction::new(vec![
                    Primitive::LineCount,
                    Primitive::Headings,
                    Primitive::Sections,
                    Primitive::Placement,
                ]),
            )
        })
    }

    /// Run the kind's composed extractor over `unit` — the primitive algebra only.
    /// Nested-member facts are never derived here: [`builtin_kind::features`](
    /// crate::builtin_kind::features), the entry point every extract call site routes
    /// through, folds them in afterward off the lock's own declared
    /// `Declarations::nested_members` rows (0018, "the projection is not the
    /// database").
    #[must_use]
    pub fn extract(&self, unit: &Unit) -> Features {
        self.extraction.extract(unit)
    }

    /// Overlay a lock row's declared `templates` onto this kind: each row becomes a
    /// [`Template`] — the kind's own declared nesting fact, read back off
    /// the lock. A host's actual embedded members are resolved independently, off
    /// `Declarations::nested_members` by address, so this overlay needs no primitive
    /// of its own to serve them.
    #[must_use]
    pub fn overlay_templates(mut self, templates: &[TemplateRow]) -> Self {
        self.templates = templates.iter().map(template_from_row).collect();
        self
    }

    /// Overlay a lock row's declared `content` onto this kind: when the row declares a
    /// layout, the kind's body shape becomes that [`Layout`] — the same LayoutRow→[`Content`]
    /// lift emit reads by ([`content_from_row`]). A row with no `content` column defers to
    /// the kind's own shape, never blanking a built-in's default, mirroring
    /// [`overlay_templates`](CustomKind::overlay_templates).
    ///
    /// # Errors
    ///
    /// Returns a [`LockRowError`] when a declared region names a primitive outside the
    /// closed vocabulary or omits the column that primitive requires.
    pub fn overlay_content(mut self, content: Option<&LayoutRow>) -> Result<Self, LockRowError> {
        if let Some(layout) = content {
            self.content = Content::Layout(layout_from_row(layout)?);
        }
        Ok(self)
    }

    /// The kind's **edge-field slot** names — its declared [`relationships`](
    /// CustomKind::relationships) fields plus the framework
    /// [`satisfies`](SATISFIES_EDGE_FIELD) key every kind carries. A layout field
    /// section whose slot is one of these is an edge slot: its entries are addresses,
    /// not a verbatim span.
    #[must_use]
    pub fn edge_field_slots(&self) -> BTreeSet<String> {
        let mut slots: BTreeSet<String> = self
            .relationships
            .iter()
            .map(|edge| edge.field.clone())
            .collect();
        slots.insert(SATISFIES_EDGE_FIELD.to_string());
        slots
    }

    /// The kind's **identity** — its bare `name`. Kept as its own
    /// method rather than inlining `.name.clone()` at each call site.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        self.name.clone()
    }

    /// The kind's declared frontmatter fields, in declaration order — the `field`
    /// extraction primitives' keys. The generic frontmatter
    /// adapter (`crate::frontmatter`) lifts these
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
    /// [`None`] for a nested file kind — its members land under their host's unit, never
    /// under a surface root of their own.
    #[must_use]
    pub fn surface_subdir(&self) -> Option<&str> {
        let root = &self.governs.as_ref()?.root;
        Some(root.rsplit('/').next().unwrap_or(root))
    }

    /// Whether a surface member imported from `source_path` belongs to this kind — its
    /// source filename matches the kind's `governs` glob leaf. The discriminator for two
    /// kinds that **share a surface locus**. A kind at a unique locus
    /// (skill's `SKILL.md`, rule's `*.md`) matches its own members, so the filter is a
    /// no-op there. A member with no readable source name belongs to nothing rather than
    /// mis-dispatching, as does every member of a kind that governs no glob at all.
    #[must_use]
    pub fn owns_source(&self, source_path: &Path) -> bool {
        let Some(matcher) = self
            .governs
            .as_ref()
            .and_then(|governs| compile_glob(governs.glob_leaf()))
        else {
            return false;
        };
        source_path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| matcher.is_match(name))
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

/// Surface a `kind`-fact column's label as an out-of-vocabulary [`LockRowError`] when its
/// closed-vocabulary lookup came back empty — the shared reject the label lifts route a
/// present-but-unrecognized value through.
fn kind_vocab<T>(label: &str, column: &'static str, parsed: Option<T>) -> Result<T, LockRowError> {
    parsed.ok_or_else(|| LockRowError::Vocabulary {
        family: "kind".to_string(),
        column: column.to_string(),
        value: label.to_string(),
    })
}

/// Parse a [`KindFactRow::format`] label into its typed [`Format`] — `None` for any
/// label outside the closed vocabulary.
fn format_from_label(label: &str) -> Option<Format> {
    match label {
        "yaml-frontmatter" => Some(Format::YamlFrontmatter),
        "json-document" => Some(Format::JsonDocument),
        "toml-document" => Some(Format::TomlDocument),
        _ => None,
    }
}

/// Parse a [`KindFactRow::unit_shape`] label into its typed [`UnitShape`] — `None`
/// outside the closed vocabulary. `named-field(<field>)` is the third mode's wire
/// form, the same `<name>(<field>)` call syntax [`registration_from_label`]'s
/// field-carrying variants use.
fn unit_shape_from_label(label: &str) -> Option<UnitShape> {
    match label {
        "file" => return Some(UnitShape::File),
        "directory" => return Some(UnitShape::Directory),
        "starred-segment" => return Some(UnitShape::StarredSegment),
        _ => {}
    }
    let (name, field) = label.strip_suffix(')')?.split_once('(')?;
    (name == "named-field").then(|| UnitShape::NamedField {
        field: field.to_string(),
    })
}

/// Parse a [`KindFactRow::commitment`] label into its typed [`Commitment`] — `None` for
/// any label outside the closed vocabulary. The committed class has no label of its own:
/// it is the absent column, so every shipped kind's row stays byte-identical.
fn commitment_from_label(label: &str) -> Option<Commitment> {
    match label {
        "local" => Some(Commitment::Local),
        _ => None,
    }
}

/// Lift a [`KindFactRow`]'s declared commitment label into its typed [`Commitment`] —
/// `None` for a kind whose file locus is committed (the absent column).
///
/// # Errors
///
/// Returns a [`LockRowError`] when the label falls outside the closed vocabulary.
pub(crate) fn commitment_from_row(row: &KindFactRow) -> Result<Option<Commitment>, LockRowError> {
    match &row.commitment {
        Some(label) => Ok(Some(kind_vocab(
            label,
            "commitment",
            commitment_from_label(label),
        )?)),
        None => Ok(None),
    }
}

/// Lift a [`KindFactRow`]'s declared format label into its typed [`Format`] — `None` for
/// a kind that declares none. [`content_from_row`]'s peer, for a caller holding a row
/// rather than a whole [`CustomKind`].
///
/// # Errors
///
/// Returns a [`LockRowError`] when the label falls outside the closed vocabulary.
pub(crate) fn format_from_row(row: &KindFactRow) -> Result<Option<Format>, LockRowError> {
    match &row.format {
        Some(label) => Ok(Some(kind_vocab(label, "format", format_from_label(label))?)),
        None => Ok(None),
    }
}

/// Lift a [`KindFactRow`]'s content facts into typed [`Content`]. The `shape` marker wins
/// when present — its sole recognized value is `fields` ([`Content::Fields`], a
/// no-body-slot kind); absent, an absent `content` column is [`Content::File`] (the
/// default every built-in takes) and a present one a [`Layout`] whose region rows lift
/// through [`layout_region_from_row`].
///
/// # Errors
///
/// Returns a [`LockRowError`] when the `shape` marker carries a label outside its closed
/// vocabulary, or a present region names a primitive outside the closed vocabulary or
/// omits the column that primitive requires.
pub(crate) fn content_from_row(row: &KindFactRow) -> Result<Content, LockRowError> {
    if let Some(label) = &row.shape {
        return kind_vocab(label, "shape", content_shape_from_label(label));
    }
    match &row.content {
        None => Ok(Content::File),
        Some(layout) => Ok(Content::Layout(layout_from_row(layout)?)),
    }
}

/// Lift one [`TemplateRow`] into a typed [`Template`] — the row→value step shared by
/// [`CustomKind::from_kind_fact_row`] and [`CustomKind::overlay_templates`], so both read
/// a declared template by the one lift. Every column is free-form data (a child kind
/// name, a path pattern), so the lift is total: there is no vocabulary to reject against.
fn template_from_row(row: &TemplateRow) -> Template {
    Template {
        kind: row.kind.clone(),
        path: row.path.clone(),
    }
}

/// Lift a [`LayoutRow`]'s region rows into a typed [`Layout`] — the LayoutRow→[`Layout`]
/// step shared by [`content_from_row`]'s layout branch and the relocation overlay
/// ([`CustomKind::overlay_content`]), so both read a declared layout by the one lift.
///
/// # Errors
///
/// Returns a [`LockRowError`] when a region names a primitive outside the closed
/// vocabulary or omits the column that primitive requires.
fn layout_from_row(layout: &LayoutRow) -> Result<Layout, LockRowError> {
    Ok(Layout {
        regions: layout
            .regions
            .iter()
            .map(layout_region_from_row)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

/// Parse a [`KindFactRow::shape`] marker label into its typed [`Content`] — `None` for any
/// label outside the closed vocabulary, whose sole member is `fields`.
fn content_shape_from_label(label: &str) -> Option<Content> {
    match label {
        "fields" => Some(Content::Fields),
        _ => None,
    }
}

/// Lift a [`KindFactRow`]'s optional `collection_address` column into a typed
/// [`CollectionAddress`]: absent for a file-locus kind, present for a registration member
/// whose key path lifts through the closed [`CollectionKeyPath`] vocabulary.
///
/// # Errors
///
/// Returns a [`LockRowError`] when the recorded `key_path` label falls outside the closed
/// vocabulary — the tool-written lock carries only labels the SDK could emit, so an
/// unknown one is corruption rejected at load.
pub(crate) fn collection_address_from_row(
    row: &KindFactRow,
) -> Result<Option<CollectionAddress>, LockRowError> {
    match &row.collection_address {
        None => Ok(None),
        Some(address) => Ok(Some(CollectionAddress {
            manifest: address.manifest.clone(),
            key_path: kind_vocab(
                &address.key_path,
                "collection_address",
                collection_key_path_from_label(&address.key_path),
            )?,
        })),
    }
}

/// Parse a [`CollectionAddressRow::key_path`](crate::drift::CollectionAddressRow::key_path)
/// label into its typed [`CollectionKeyPath`] — `None` for any label outside the closed
/// vocabulary (`hooks.<Event>`, `mcpServers.*`, `enabledPlugins.*`).
fn collection_key_path_from_label(label: &str) -> Option<CollectionKeyPath> {
    match label {
        "hooks.<Event>" => Some(CollectionKeyPath::HooksEvent),
        "mcpServers.*" => Some(CollectionKeyPath::McpServers),
        "enabledPlugins.*" => Some(CollectionKeyPath::EnabledPlugins),
        _ => None,
    }
}

/// Lift one [`LayoutRegionRow`](crate::drift::LayoutRegionRow) into a typed
/// [`LayoutRegion`].
///
/// # Errors
///
/// Returns a [`LockRowError`] when the `region` discriminator is outside the closed
/// three-primitive vocabulary, or the row omits the column that primitive requires (a
/// `field` with no `slot`, a `collection` with no `member_kind`).
fn layout_region_from_row(
    row: &crate::drift::LayoutRegionRow,
) -> Result<LayoutRegion, LockRowError> {
    match row.region.as_str() {
        "prose" => Ok(LayoutRegion::Prose {
            import: row.import.clone(),
        }),
        "field" => {
            let slot = row
                .slot
                .clone()
                .ok_or_else(|| LockRowError::MissingColumn {
                    family: "kind".to_string(),
                    column: "slot".to_string(),
                })?;
            Ok(LayoutRegion::Field { slot })
        }
        "collection" => {
            let member_kind =
                row.member_kind
                    .clone()
                    .ok_or_else(|| LockRowError::MissingColumn {
                        family: "kind".to_string(),
                        column: "member_kind".to_string(),
                    })?;
            Ok(LayoutRegion::Collection {
                member_kind,
                key: row.key.clone(),
            })
        }
        other => Err(LockRowError::Vocabulary {
            family: "kind".to_string(),
            column: "content".to_string(),
            value: other.to_string(),
        }),
    }
}

/// Parse one [`KindFactRow::registration`] wire label into its typed [`Registration`]
/// channel — the closed vocabulary's compact wire form (`always`/`user-invoked`, or a
/// `<name>(<field>)` call for the three field-carrying variants). `None` for a bare
/// unrecognized name or a malformed `(field)` suffix. The row carries one label per
/// declared channel; the caller folds each label of the set through this.
fn registration_from_label(label: &str) -> Option<Registration> {
    match label {
        "always" => return Some(Registration::Always),
        "user-invoked" => return Some(Registration::UserInvoked),
        "connection" => return Some(Registration::Connection),
        "enablement" => return Some(Registration::Enablement),
        _ => {}
    }
    let (name, field) = label.strip_suffix(')')?.split_once('(')?;
    let field = field.to_string();
    match name {
        "description-trigger" => Some(Registration::DescriptionTrigger { field }),
        "paths-match" => Some(Registration::PathsMatch { field }),
        "event" => Some(Registration::Event { field }),
        _ => None,
    }
}

/// Compile `glob` into a `globset` matcher — the one glob-matching surface every
/// caller shares, in this module or across the crate (a kind's own
/// [`CustomKind::owns_source`] membership test, `import`'s per-segment discovery
/// walk, `coverage_note`'s `governs` leaf test, `graph`'s `paths-match` liveness
/// test). `literal_separator` is on: `*`/`?` stay within one `/`-separated segment,
/// `**` crosses segments (a leading `**/` matching zero or more, per `globset`'s
/// documented three-position grammar) — the one semantics every call site needs,
/// whether the candidate it tests is a bare filename (no `/` to cross) or a full
/// repo-relative path. `None` for a glob `globset` cannot compile (a malformed
/// character class); the caller decides what an uncompilable pattern means for its
/// own match (`graph`'s liveness test treats it as matching, never a false
/// negative on a pattern it failed to understand).
#[must_use]
pub(crate) fn compile_glob(glob: &str) -> Option<globset::GlobMatcher> {
    globset::GlobBuilder::new(glob)
        .literal_separator(true)
        .build()
        .ok()
        .map(|compiled| compiled.compile_matcher())
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
/// extractable*, so a clause over its feature is a true positive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    /// `field` — retain the frontmatter value at `key` as the named field feature,
    /// exactly as parsed. An absent key ⇒ the feature is not yielded — absent, never
    /// errored — mirroring how a skill's optional `version` is omitted when unset.
    ///
    /// Retention, never traversal: the value keeps its nesting whole, and reaching into
    /// it is the *clause*'s to spell — a `field` addressing path walks an object and
    /// grains over an array's elements (`crate::address`). One home for path walking, and
    /// it is the one the RFC engine backs.
    Field {
        /// The frontmatter key read, and the name the feature is keyed by.
        key: String,
    },
    /// `headings` — the body's ATX headings, in document order
    /// (`Features::headings`).
    Headings,
    /// `sections` — the body's ATX sections (each heading + the body span beneath
    /// it), in document order (`Features::sections`) — the `## Decision`-block
    /// feature a `section_contains` clause decides over.
    Sections,
    /// `line_count` — the body's line count (`Features::body_lines`).
    LineCount,
    /// `placement` — the name of the directory the unit sits under
    /// (`Features::source_dir`) — file placement.
    Placement,
    /// `directives` — the body's format-executed directive occurrences for the
    /// named [`syntax`](DirectiveSyntax), folded into `Features::directives` in
    /// document order. Unlike the mining the `references` retirement bans, a directive
    /// is grammar the format authority documents as *executed*, so its occurrences are
    /// observed structure, not typography.
    Directives {
        /// The directive syntax extracted — the closed per-syntax vocabulary, sole
        /// member `at-import`.
        syntax: DirectiveSyntax,
    },
    /// `fenced` — the body's fenced code blocks (`Features::fenced_blocks`), in
    /// document order, each block's info string paired with its interior content.
    /// Markdown structure, deterministically extractable like
    /// `headings`/`sections`: the same fence boundaries, surfaced whole. Its first
    /// consumer is the member fence — fenced extraction composed with a TOML parse;
    /// this primitive yields the raw blocks only.
    Fenced,
}

/// A directive's format-executed body syntax — the closed per-syntax vocabulary the
/// [`Directives`](Primitive::Directives) primitive ranges over.
/// The
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
                    features.fields.insert(key.clone(), value.clone());
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
    /// Empty for a frontmatter-less kind.
    pub frontmatter: BTreeMap<String, JsonValue>,
    /// The byte-faithful markdown body (frontmatter stripped) — the locus for
    /// `headings`, `sections`, and `line_count`.
    pub body: String,
    /// The source path the unit was read from — the `placement` locus.
    pub source_path: PathBuf,
    /// The requirements this unit opts into filling — the authored
    /// `[satisfies.<requirement>]` header modules. A
    /// representation edge the coverage check resolves, not a composed feature:
    /// intrinsic to the surface, threaded through unchanged so a custom-kind member
    /// joins coverage exactly as a skill/rule does. Empty
    /// when the member authors none.
    pub satisfies: Vec<String>,
    /// The same `[satisfies.<requirement>]` opt-ins **with their authored rationale**
    /// — the whole [`Satisfies`] clause,
    /// not just the name coverage reads. The read family (`why`/`requirements`) narrates
    /// the *why* a custom member fills a requirement (READ-CUSTOM-SATISFIERS), so it
    /// needs the rationale the decidable [`satisfies`](Unit::satisfies) name-vec drops.
    /// Empty when the member authors none.
    pub satisfies_clauses: Vec<crate::document::Satisfies>,
}

/// The error type [`crate::builtin_kind`]'s embedded-kind lookups return, kept for API
/// stability even though every embedded kind is plain Rust data and none of those
/// lookups can fail: an empty enum, so a `Result::Err` here is statically
/// unreachable.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum KindError {}

impl Extraction {
    /// Compose an extractor directly from its ordered [`Primitive`]s — the
    /// constructor a kind's declared definition supplies (`crate::builtin_kind`, a
    /// future SDK-authored custom kind). There is no `[[extraction]]` file grammar
    /// to parse it from; an empty vec is the valid
    /// vacuous extractor.
    #[must_use]
    pub fn new(primitives: Vec<Primitive>) -> Self {
        Self { primitives }
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
            // Render-side extent is intrinsic to every unit, never gated behind the
            // `line_count` primitive — an `extent` clause is node-scope and decides over
            // any kind's members. The body a file member carries is read off its
            // committed projection, so its size is the rendered extent directly.
            rendered_lines: Some(extract::body_line_count(&unit.body)),
            rendered_chars: Some(unit.body.chars().count()),
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: None,
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            // Nested members are folded by [`CustomKind::extract`] after the primitives
            // run — a typed layer over `fenced_blocks`, needing the kind's declared
            // template set the primitive-only `Extraction` does not hold. Empty here on
            // purpose.
            nested_members: Vec::new(),
            // `satisfies` is a surface edge threaded through unchanged, not a
            // composed primitive, so a custom-kind member joins coverage exactly as
            // a built-in kind's does.
            satisfies: unit.satisfies.clone(),
            // A unit's own format is the file format the extraction just read it
            // back through, so nothing here observed a placement. The fact belongs to
            // an embedded value, whose format `emit` rendered ([`crate::main`]).
            edge_placements: None,
        };
        for primitive in &self.primitives {
            primitive.apply(unit, &mut features);
        }
        features
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::{FeatureValue, ValueType};

    /// Whether `glob` matches `candidate` through the shared `compile_glob` surface —
    /// `None` (an uncompilable glob) reported as no match, the polarity every
    /// segment-level caller (`owns_source`, `import`, `coverage_note`) wants.
    fn matches(glob: &str, candidate: &str) -> bool {
        compile_glob(glob).is_some_and(|matcher| matcher.is_match(candidate))
    }

    #[test]
    fn compile_glob_matches_common_path_globs_within_and_across_segments() {
        // `**/` matches any number of leading segments including none, a flat `*`
        // stays within one segment — the semantics every caller through
        // `compile_glob` leans on, whether matching a bare filename or a full
        // repo-relative path.
        assert!(matches("**/*.rs", "foo.rs"));
        assert!(matches("**/*.rs", "src/a/foo.rs"));
        assert!(!matches("**/*.rs", "foo.md"));

        assert!(matches("src/**", "src/graph.rs"));
        assert!(matches("src/**", "src/a/b.rs"));
        assert!(!matches("src/**", "tests/graph.rs"));

        // A single `*` does not cross a `/`.
        assert!(matches("*.md", "README.md"));
        assert!(!matches("*.md", "docs/README.md"));

        // A `?` matches exactly one character, never a `/`.
        assert!(matches("SKILL.md", "SKILL.md"));
        assert!(matches("SKILL.m?", "SKILL.md"));
        assert!(!matches("SKILL.m?", "SKILL.mkd"));
        assert!(!matches("*/SKILL.md", "SKILL.md"));
        assert!(matches("[0-9][0-9]-*.md", "07-kinds.md"));
        assert!(!matches("[0-9][0-9]-*.md", "ab-kinds.md"));
    }

    #[test]
    fn compile_glob_is_none_for_an_uncompilable_pattern() {
        // An unterminated character class is a `globset` compile error — `None`,
        // never a panic.
        assert!(compile_glob("[abc").is_none());
    }

    /// The composed `spec`-shaped extractor the worked example needs:
    /// line count, ATX headings, and file placement —
    /// markdown structure only, no body-mined references (the `references`
    /// primitive is retired; the corpus's edges are declared in member headers).
    fn spec_extraction() -> Extraction {
        Extraction::new(vec![
            Primitive::LineCount,
            Primitive::Headings,
            Primitive::Placement,
        ])
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

        // Extraction is a pure function of the surface — the soundness boundary:
        // the same unit yields the same features every run.
        let first = extraction.extract(&unit);
        let second = extraction.extract(&unit);
        assert_eq!(first, second);
    }

    #[test]
    fn a_field_primitive_projects_frontmatter_kind_preserving() {
        let extraction = Extraction::new(vec![
            Primitive::Field {
                key: "name".to_string(),
            },
            Primitive::Field {
                key: "priority".to_string(),
            },
        ]);

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
        };

        let features = extraction.extract(&unit);

        // The frontmatter value is projected through the shared kind-preserving
        // projector: a string stays `string`, an integer keeps `integer`.
        assert_eq!(
            features.field("name"),
            Some(FeatureValue::scalar(ValueType::String, "demo"))
        );
        assert_eq!(
            features.field("priority").map(|value| value.kind()),
            Some(ValueType::Integer)
        );
        // The body loci are untouched — this extractor composes only `field`.
        assert_eq!(features.body_lines, 0);
        assert!(features.headings.is_empty());
    }

    #[test]
    fn a_fenced_primitive_parses_and_folds_block_interiors_into_features() {
        // `fenced` is a closed-vocab, parameterless primitive — it parses into
        // `Primitive::Fenced` and folds the body's fenced blocks into `fenced_blocks`,
        // each interior paired with its info string, surrounding prose skipped.
        let extraction = Extraction::new(vec![Primitive::Fenced]);
        assert_eq!(extraction.primitives(), &[Primitive::Fenced]);

        let body = "# Doc\n\nprose\n\n```toml member.manifest\nname = \"x\"\n```\n";
        let unit = Unit {
            id: "doc".to_string(),
            frontmatter: BTreeMap::new(),
            body: body.to_string(),
            source_path: PathBuf::from("specs/architecture/15-kinds.md"),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
        };
        let features = extraction.extract(&unit);
        assert_eq!(features.fenced_blocks.len(), 1);
        assert_eq!(features.fenced_blocks[0].info, "toml member.manifest");
        assert_eq!(features.fenced_blocks[0].content, "name = \"x\"");
        // This extractor composes only `fenced` — every other locus stays at its
        // default (no headings extracted, no fields), the vacuous-composition floor.
        assert!(features.headings.is_empty());
        assert!(features.fields.is_empty());
    }

    #[test]
    fn a_field_absent_from_the_unit_is_not_yielded() {
        let extraction = Extraction::new(vec![Primitive::Field {
            key: "license".to_string(),
        }]);
        let unit = Unit {
            id: "demo".to_string(),
            frontmatter: BTreeMap::new(),
            body: String::new(),
            source_path: PathBuf::from("skills/demo/SKILL.md"),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
        };
        // A key the unit does not carry yields no feature — never a phantom entry.
        assert!(extraction.extract(&unit).field("license").is_none());
    }

    #[test]
    fn an_empty_declaration_is_a_vacuous_extractor() {
        let extraction = Extraction::new(Vec::new());
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

    /// A bare `spec` [`KindFactRow`] with every optional fact absent — the base the
    /// `from_kind_fact_row` content tests override the one column they exercise onto.
    fn spec_row() -> KindFactRow {
        KindFactRow {
            name: "spec".to_string(),
            provider: None,
            governs_root: Some("specs".to_string()),
            governs_glob: Some("*.md".to_string()),
            commitment: None,
            format: None,
            unit_shape: None,
            registration: Vec::new(),
            templates: Vec::new(),
            content: None,
            shape: None,
            collection_address: None,
        }
    }

    /// A bare `spec` kind — the shape a built-in or SDK-authored custom kind
    /// constructs ([`CustomKind::new`]), no file format involved.
    fn spec_kind() -> CustomKind {
        CustomKind::new(
            "spec",
            Governs {
                root: "specs".to_string(),
                glob: "*.md".to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }

    #[test]
    fn new_constructs_a_kind_with_every_optional_fact_absent() {
        // The constructor's defaults — nothing invented until the caller sets it.
        let kind = spec_kind();
        assert_eq!(kind.format, None);
        assert_eq!(kind.unit_shape, None);
        assert!(kind.registration.is_empty());
        assert!(kind.relationships.is_empty());
        assert!(kind.templates.is_empty());
        assert_eq!(kind.content, Content::File);
    }

    #[test]
    fn qualified_name_is_the_bare_name() {
        // Identity travels by import, never by string — a kind's qualified identity
        // is always its own bare name.
        assert_eq!(spec_kind().qualified_name(), "spec");
    }

    #[test]
    fn from_kind_fact_row_lifts_every_declared_fact() {
        let row = KindFactRow {
            name: "spec".to_string(),
            provider: None,
            governs_root: Some("specs".to_string()),
            governs_glob: Some("*.md".to_string()),
            commitment: None,
            format: Some("yaml-frontmatter".to_string()),
            unit_shape: Some("directory".to_string()),
            registration: vec!["description-trigger(description)".to_string()],
            templates: Vec::new(),
            content: None,
            shape: None,
            collection_address: None,
        };
        let kind = CustomKind::from_kind_fact_row(&row).unwrap();

        assert_eq!(kind.name, "spec");
        assert_eq!(
            kind.governs,
            Some(Governs {
                root: "specs".to_string(),
                glob: "*.md".to_string(),
            })
        );
        assert_eq!(kind.format, Some(Format::YamlFrontmatter));
        assert_eq!(kind.unit_shape, Some(UnitShape::Directory));
        assert_eq!(
            kind.registration,
            vec![Registration::DescriptionTrigger {
                field: "description".to_string()
            }]
        );
        // The generic markdown-structure set every built-in composes — never a
        // per-kind `Field` primitive, since the row carries no field-level facts.
        assert_eq!(
            kind.extraction.primitives(),
            &[
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]
        );
    }

    #[test]
    fn from_kind_fact_row_rejects_an_out_of_vocabulary_label() {
        // The lock is tool-written, so a label the closed vocabulary cannot admit is a
        // corrupt lock rejected loud at load, never a channel silently dropped.
        let row = KindFactRow {
            name: "spec".to_string(),
            provider: None,
            governs_root: Some("specs".to_string()),
            governs_glob: Some("*.md".to_string()),
            commitment: None,
            format: Some("xml".to_string()),
            unit_shape: Some("directory".to_string()),
            registration: vec!["bogus".to_string()],
            templates: Vec::new(),
            content: None,
            shape: None,
            collection_address: None,
        };
        let err = CustomKind::from_kind_fact_row(&row).unwrap_err();
        assert!(
            matches!(&err, LockRowError::Vocabulary { family, column, value }
                if family == "kind" && column == "format" && value == "xml"),
            "expected an out-of-vocabulary `format` reject, got: {err:?}"
        );
    }

    #[test]
    fn from_kind_fact_row_rejects_an_out_of_vocabulary_registration_channel() {
        // A registration set carrying one unrecognized label is corruption — the whole
        // lift rejects loud rather than dropping the bad channel and keeping the rest.
        let row = KindFactRow {
            name: "spec".to_string(),
            provider: None,
            governs_root: Some("specs".to_string()),
            governs_glob: Some("*.md".to_string()),
            commitment: None,
            format: None,
            unit_shape: None,
            registration: vec!["user-invoked".to_string(), "bogus".to_string()],
            templates: Vec::new(),
            content: None,
            shape: None,
            collection_address: None,
        };
        let err = CustomKind::from_kind_fact_row(&row).unwrap_err();
        assert!(
            matches!(&err, LockRowError::Vocabulary { column, value, .. }
                if column == "registration" && value == "bogus"),
            "expected an out-of-vocabulary `registration` reject, got: {err:?}"
        );
    }

    #[test]
    fn from_kind_fact_row_lifts_a_multi_channel_registration_set_in_order() {
        // `skill`'s own two-channel set — both labels lift, order preserved.
        let row = KindFactRow {
            name: "skill".to_string(),
            provider: None,
            governs_root: Some(".claude/skills".to_string()),
            governs_glob: Some("*/SKILL.md".to_string()),
            commitment: None,
            format: None,
            unit_shape: None,
            registration: vec![
                "user-invoked".to_string(),
                "description-trigger(description)".to_string(),
            ],
            templates: Vec::new(),
            content: None,
            shape: None,
            collection_address: None,
        };
        let kind = CustomKind::from_kind_fact_row(&row).unwrap();
        assert_eq!(
            kind.registration,
            vec![
                Registration::UserInvoked,
                Registration::DescriptionTrigger {
                    field: "description".to_string()
                },
            ]
        );
    }

    #[test]
    fn from_kind_fact_row_with_no_optional_facts_yields_the_generic_defaults() {
        let row = KindFactRow {
            name: "adr".to_string(),
            provider: None,
            governs_root: Some("adr".to_string()),
            governs_glob: Some("*.md".to_string()),
            commitment: None,
            format: None,
            unit_shape: None,
            registration: Vec::new(),
            templates: Vec::new(),
            content: None,
            shape: None,
            collection_address: None,
        };
        let kind = CustomKind::from_kind_fact_row(&row).unwrap();
        assert_eq!(kind.format, None);
        assert_eq!(kind.unit_shape, None);
        assert!(kind.registration.is_empty());
    }

    #[test]
    fn from_kind_fact_row_with_no_content_column_is_file_content() {
        // Every built-in kind is file-content: an absent `content` column lifts to
        // `Content::File`, never a phantom empty layout.
        let row = KindFactRow {
            content: None,
            ..spec_row()
        };
        assert_eq!(
            CustomKind::from_kind_fact_row(&row).unwrap().content,
            Content::File
        );
    }

    #[test]
    fn from_kind_fact_row_lifts_a_declared_layout_in_all_three_primitives() {
        // A `layout`-content kind's row carries its ordered regions — an importing
        // prose region, a field section filling a slot, and a member collection of a
        // named kind — each lifting into its typed `LayoutRegion`.
        let row = KindFactRow {
            content: Some(crate::drift::LayoutRow {
                regions: vec![
                    crate::drift::LayoutRegionRow {
                        region: "prose".to_string(),
                        import: Some("specs/intent.md".to_string()),
                        slot: None,
                        member_kind: None,
                        key: None,
                    },
                    crate::drift::LayoutRegionRow {
                        region: "field".to_string(),
                        import: None,
                        slot: Some("intent".to_string()),
                        member_kind: None,
                        key: None,
                    },
                    crate::drift::LayoutRegionRow {
                        region: "collection".to_string(),
                        import: None,
                        slot: None,
                        member_kind: Some("invariant".to_string()),
                        key: None,
                    },
                ],
            }),
            ..spec_row()
        };
        assert_eq!(
            CustomKind::from_kind_fact_row(&row).unwrap().content,
            Content::Layout(Layout {
                regions: vec![
                    LayoutRegion::Prose {
                        import: Some("specs/intent.md".to_string()),
                    },
                    LayoutRegion::Field {
                        slot: "intent".to_string(),
                    },
                    LayoutRegion::Collection {
                        member_kind: "invariant".to_string(),
                        key: None,
                    },
                ],
            }),
        );
    }

    #[test]
    fn from_kind_fact_row_rejects_an_out_of_vocabulary_layout_region() {
        // A region whose discriminator is outside the closed three-primitive vocabulary
        // is a corrupt lock — rejected loud, never dropped from an otherwise-valid layout.
        let row = KindFactRow {
            content: Some(crate::drift::LayoutRow {
                regions: vec![
                    crate::drift::LayoutRegionRow {
                        region: "bogus".to_string(),
                        import: None,
                        slot: None,
                        member_kind: None,
                        key: None,
                    },
                    crate::drift::LayoutRegionRow {
                        region: "prose".to_string(),
                        import: None,
                        slot: None,
                        member_kind: None,
                        key: None,
                    },
                ],
            }),
            ..spec_row()
        };
        let err = CustomKind::from_kind_fact_row(&row).unwrap_err();
        assert!(
            matches!(&err, LockRowError::Vocabulary { column, value, .. }
                if column == "content" && value == "bogus"),
            "expected an out-of-vocabulary region reject, got: {err:?}"
        );
    }

    #[test]
    fn from_kind_fact_row_rejects_a_layout_region_missing_its_required_column() {
        // A `field` region carries no `slot`: the primitive's required column is absent,
        // so the present region is malformed and rejected loud, never silently dropped.
        let row = KindFactRow {
            content: Some(crate::drift::LayoutRow {
                regions: vec![crate::drift::LayoutRegionRow {
                    region: "field".to_string(),
                    import: None,
                    slot: None,
                    member_kind: None,
                    key: None,
                }],
            }),
            ..spec_row()
        };
        let err = CustomKind::from_kind_fact_row(&row).unwrap_err();
        assert!(
            matches!(&err, LockRowError::MissingColumn { family, column }
                if family == "kind" && column == "slot"),
            "expected a missing-`slot` reject, got: {err:?}"
        );
    }

    #[test]
    fn from_kind_fact_row_lifts_declared_templates_by_child_kind() {
        // Each recorded template lifts into a `Template`: an embedded layer by its child
        // kind alone, a file layer carrying the path pattern its children sit at.
        let row = KindFactRow {
            name: "spec".to_string(),
            provider: None,
            governs_root: Some("specs".to_string()),
            governs_glob: Some("*.md".to_string()),
            commitment: None,
            format: None,
            unit_shape: None,
            registration: Vec::new(),
            templates: vec![
                TemplateRow {
                    kind: "decision".to_string(),
                    path: None,
                },
                TemplateRow {
                    kind: "note".to_string(),
                    path: Some("notes/*.md".to_string()),
                },
            ],
            content: None,
            shape: None,
            collection_address: None,
        };
        let kind = CustomKind::from_kind_fact_row(&row).unwrap();
        assert_eq!(
            kind.templates,
            vec![
                Template {
                    kind: "decision".to_string(),
                    path: None,
                },
                Template {
                    kind: "note".to_string(),
                    path: Some("notes/*.md".to_string()),
                },
            ]
        );
    }

    #[test]
    fn overlay_templates_sets_the_declared_template_set_and_leaves_extraction_untouched() {
        // Overlaying a declared template records the kind's own nesting fact only — a
        // host's actual embedded members are resolved off `Declarations::nested_members`
        // by address, never by anything this kind's own composed extraction yields.
        let kind = CustomKind::new(
            "rule",
            Governs {
                root: ".claude/rules".to_string(),
                glob: "*.md".to_string(),
            },
            Extraction::new(vec![Primitive::LineCount, Primitive::Headings]),
        )
        .overlay_templates(&[TemplateRow {
            kind: "directive".to_string(),
            path: None,
        }]);

        assert_eq!(
            kind.templates,
            vec![Template {
                kind: "directive".to_string(),
                path: None,
            }]
        );
        assert_eq!(
            kind.extraction.primitives(),
            &[Primitive::LineCount, Primitive::Headings]
        );
    }
}
