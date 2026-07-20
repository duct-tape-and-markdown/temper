//! Extraction — an artifact's surface-decidable feature set.
//!
//! Extraction is the soundness boundary temper's decidability spine rests on:
//! a per-kind extractor projects a parsed artifact into a
//! [`Features`] map the generic contract engine reads. A contract clause is
//! sound only because the feature it names is **deterministically
//! extractable** — so [`Features`]
//! admits *only* surface-decidable facts (a field's value, a key's presence, a
//! body's line count, the body's ATX headings, the directory a unit sits under)
//! and never inferred
//! prose meaning ("is this fact duplicated," "does this paragraph mean X"). That
//! restraint is what makes a violation a true positive, which is what earns the
//! hard gate.
//!
//! ## Generic by field name (the whole point)
//!
//! Frontmatter fields are keyed by **name**, so a clause referencing `name` or
//! `description` resolves through [`Features::field`] without the engine baking
//! in any `skill.name` opinion. The same lookup serves every artifact kind: the
//! engine carries the predicate vocabulary (`crate::contract`), the extractor
//! carries the facts, and the two meet only at the field name. This module
//! deliberately takes no dependency on [`crate::contract`] — features are facts,
//! not clauses.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value as JsonValue;

/// A field's parsed source kind — the closed scalar/container lattice a kind's
/// field schema ranges over. Taken from the *parsed*
/// YAML/JSON value, not its stringified form: a sound `type` check needs the
/// extractor to preserve the source kind rather than collapse every scalar to a
/// bare string. The five scalar kinds
/// answer [`FeatureValue::as_scalar`]; the two container kinds do not.
///
/// The binding renames to lowercase so the TS spelling is the one
/// [`ValueType::from_name`] decodes — the lattice has one name across the seam,
/// and a `type` clause's declared kind crosses the lock in that spelling.
///
/// `Ord` is the declaration order above, and it is what gives a declared *set* of
/// kinds one canonical spelling: a `type` clause holds its kinds in a `BTreeSet`, so
/// the order an author wrote them in never reaches the lock, the diagnostic, or the
/// emitted schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, schemars::JsonSchema, ts_rs::TS)]
#[ts(rename_all = "lowercase")]
pub enum ValueType {
    /// A textual scalar.
    String,
    /// A whole-number scalar (no fractional part).
    Integer,
    /// A fractional/floating-point scalar.
    Number,
    /// A boolean scalar.
    Boolean,
    /// A null scalar.
    Null,
    /// A sequence/array container.
    List,
    /// A mapping/object container.
    Map,
}

impl ValueType {
    /// The lattice name of this kind — the declared-type spelling a `type`
    /// clause uses and the form diagnostics render. The inverse of
    /// [`ValueType::from_name`].
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            ValueType::String => "string",
            ValueType::Integer => "integer",
            ValueType::Number => "number",
            ValueType::Boolean => "boolean",
            ValueType::Null => "null",
            ValueType::List => "list",
            ValueType::Map => "map",
        }
    }

    /// Parse a declared type name into its [`ValueType`], or `None` if it is not one
    /// of the closed lattice's names. This is the single home of the lattice's
    /// name table; the contract parser maps a
    /// declared `type` through here rather than duplicating the spelling.
    #[must_use]
    pub fn from_name(name: &str) -> Option<ValueType> {
        match name {
            "string" => Some(ValueType::String),
            "integer" => Some(ValueType::Integer),
            "number" => Some(ValueType::Number),
            "boolean" => Some(ValueType::Boolean),
            "null" => Some(ValueType::Null),
            "list" => Some(ValueType::List),
            "map" => Some(ValueType::Map),
            _ => None,
        }
    }
}

/// One extracted feature value: a scalar field (carrying its parsed source
/// [`ValueType`] alongside its comparison text), a list field (e.g. a YAML sequence
/// like `allowed-tools`), or a map field. Scalar predicates (`min_len`, `enum`,
/// `deny`, `allowed_chars`) read the scalar text; presence predicates
/// (`required`, `forbidden_keys`) need only the key; the `type` primitive reads
/// [`FeatureValue::kind`].
#[derive(Debug, Clone, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub enum FeatureValue {
    /// A single scalar value: its parsed source kind (one of the scalar kinds —
    /// `string`/`integer`/`number`/`boolean`/`null`) and its stringified text,
    /// the comparison text the scalar predicates read.
    Scalar {
        /// The parsed source kind of the scalar.
        kind: ValueType,
        /// The scalar as text (the YAML/JSON scalar stringified).
        text: String,
    },
    /// A sequence of scalar values, stringified element-wise (kind `list`).
    List(Vec<String>),
    /// A mapping/object value (kind `map`). Only its kind is projected — no
    /// predicate reads a map's contents — so it carries no payload.
    Map,
}

impl FeatureValue {
    /// A scalar feature of the given kind and text — the construction helper the
    /// extractor and tests share.
    #[must_use]
    pub fn scalar(kind: ValueType, text: impl Into<String>) -> Self {
        FeatureValue::Scalar {
            kind,
            text: text.into(),
        }
    }

    /// The scalar text of this value, or `None` if it is a container (list or
    /// map). Lets a scalar-oriented clause (`min_len`, `enum`, …) read the value
    /// generically — unchanged by the kind now riding alongside the text.
    #[must_use]
    pub fn as_scalar(&self) -> Option<&str> {
        match self {
            FeatureValue::Scalar { text, .. } => Some(text),
            FeatureValue::List(_) | FeatureValue::Map => None,
        }
    }

    /// This value's parsed source kind — the fact the `type` primitive decides
    /// over. A list is always [`ValueType::List`] and a map [`ValueType::Map`]; a scalar
    /// reports the kind it was parsed as.
    #[must_use]
    pub fn kind(&self) -> ValueType {
        match self {
            FeatureValue::Scalar { kind, .. } => *kind,
            FeatureValue::List(_) => ValueType::List,
            FeatureValue::Map => ValueType::Map,
        }
    }
}

/// One ATX **section** of a markdown body: a heading paired with the body span
/// beneath it, up to the next heading of the same or a shallower level. The
/// feature a `section_contains` clause decides over — its [`heading`](Section::heading) is
/// matched by the clause's declared prefix, its [`body`](Section::body) searched
/// for the declared marker. Surface-decidable like every other feature: a heading
/// inside a fenced code block opens no section (the same exclusion
/// [`body_headings`] makes), so a section is never a guess.
#[derive(Debug, Clone, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub struct Section {
    /// The heading text, with its `#` markers stripped exactly as
    /// [`body_headings`] strips them.
    pub heading: String,
    /// The body span beneath the heading — the intervening lines rejoined with
    /// `\n`, the text a `section_contains` marker check searches.
    pub body: String,
}

/// One fenced code block of a markdown body: its **info string** (the text after
/// the opening fence — `sh`, `toml`, `toml member.foo` — trimmed) paired with the
/// block's **interior content** (the lines between the fences, rejoined with `\n`).
/// The feature a `fenced` primitive yields: fenced extraction
/// composed with a TOML parse yields an embedded member's features, declared data
/// at body position. Surface-decidable like every other feature — the fence
/// boundaries are the ones [`body_headings`] already tracks, so a block is never a
/// guess. The info string is available so the embedded-member consumer can key on
/// `member.<name>`; this generic primitive yields the raw blocks only, the TOML
/// composition a later slice.
#[derive(Debug, Clone, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub struct FencedBlock {
    /// The opening fence's info string, trimmed — `sh`, `toml`, or empty for a bare
    /// fence. The declared kind the embedded-member consumer keys on.
    pub info: String,
    /// The block's interior content — the lines between the fences, rejoined with
    /// `\n`, byte-faithful to the body span exactly as a [`Section`]'s body is.
    pub content: String,
}

/// A **nested member** folded from its parent's body at the **embedded locus**,
/// extracted from a member fence
/// at the floor. It carries the child kind it instantiates (`decision`) and the fence
/// key that names this instance among its embedded siblings (`surface-authority`),
/// plus its own **prose leaves** — top-level authored strings — and its own **nested
/// members**, one layer deeper: a collection's entries, each itself an
/// [`EmbeddedMember`] (`rejected.baked-projection`), in authored order. Every leaf is
/// addressed structurally (member + kind + key + child path) so drift, `impact`, and
/// citations survive rewording ([`EmbeddedMember::addressed_leaves`]).
///
/// Floor leaves carry no mentions: an edge is declared at a position, never matched as
/// a pattern within prose, so no floor mention syntax can exist and a leaf is
/// permanently a plain [`String`], never a mention-bearing span.
#[derive(Debug, Clone, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub struct EmbeddedMember {
    /// The child kind this member instantiates — the fence info string's
    /// `member.<kind>` (`decision`), one of the host kind's declared templates.
    pub kind: String,
    /// The fence key naming this instance among its embedded siblings in the same
    /// host member — the info string's second token (`surface-authority`). Part of a
    /// leaf's address, so it is keyed, never positional.
    pub key: String,
    /// This member's own top-level **prose leaves** — field name → authored string,
    /// in stable (sorted) key order so serialization is deterministic.
    pub leaves: BTreeMap<String, String>,
    /// This member's own **nested members**, one layer deeper — one entry per
    /// collection member, in authored order, so a collection leaf addresses as
    /// `<collection>.<entry>.<field>` (`rejected.baked-projection.because`). Each
    /// entry's collection name and key are keyed identity, never positional — an
    /// address that survives insertion and reorder even though the list itself is
    /// ordered.
    pub members: Vec<EmbeddedMemberCollectionEntry>,
}

/// One entry in an [`EmbeddedMember`]'s sibling collection: the collection name it
/// belongs to, its own key among that collection's siblings, and its own nested
/// member — the same identity a [`CollectionEntryRow`](crate::drift::CollectionEntryRow)
/// carries, expanded one layer.
#[derive(Debug, Clone, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub struct EmbeddedMemberCollectionEntry {
    /// The collection this entry belongs to.
    pub collection: String,
    /// The entry's key among its collection's siblings.
    pub key: String,
    /// The entry's own nested member.
    pub member: EmbeddedMember,
}

impl EmbeddedMember {
    /// Every leaf's **structural child path** paired with its authored value, in
    /// stable order: a top-level leaf's
    /// bare field name (`chosen`), a nested member's
    /// leaf `<collection>.<entry>.<field>` (`rejected.baked-projection.because`). The
    /// path rides the structure the author already wrote, so it is stable under
    /// content edits — the property drift routing and `impact` stand on. Recurses
    /// through [`members`](EmbeddedMember::members) to arbitrary depth,
    /// though today's fold populates one
    /// layer.
    #[must_use]
    pub fn addressed_leaves(&self) -> Vec<(String, &str)> {
        let mut out = Vec::new();
        for (field, value) in &self.leaves {
            out.push((field.clone(), value.as_str()));
        }
        for entry in &self.members {
            for (path, value) in entry.member.addressed_leaves() {
                out.push((format!("{}.{}.{path}", entry.collection, entry.key), value));
            }
        }
        out
    }
}

/// The **structural address** of a nested member's leaf: the member it lives in, the nested member's
/// identity (child kind + fence key), and the child path within it. Keyed at
/// every level and stable under content edits, so a citation targeting a leaf
/// and `impact` at leaf grain survive rewording —
/// only a key *rename* breaks it, and then to the resolution check, which tells the citer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberAddress {
    /// The (outer) member the nested member lives in (`Features::id`).
    pub member: String,
    /// The child kind the nested member instantiates (`decision`).
    pub kind: String,
    /// The fence key naming the nested member among its embedded siblings
    /// (`surface-authority`).
    pub key: String,
    /// The child path within the nested member — a bare leaf name or a
    /// `<collection>.<entry>.<field>` path.
    pub child_path: String,
}

/// An artifact's deterministically-extracted features, keyed for generic clause
/// lookup. Everything here is surface-decidable; nothing is inferred meaning.
#[derive(Debug, Clone, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub struct Features {
    /// The artifact id used in diagnostics (for a skill, its `name`).
    pub id: String,
    /// Frontmatter fields by name — the typed fields *and* the `extra` keys, so
    /// a clause resolves `name`/`description`/`version` or any unknown key
    /// (e.g. for `forbidden_keys`) through one generic lookup.
    ///
    /// Each value is retained **as parsed**, never flattened: a clause addresses a field
    /// by a path that may step into an object or grain over an array's elements
    /// ([`Features::locate`]), so the nesting has to survive extraction. Projection to
    /// the [`FeatureValue`] the predicates read happens at the read edge
    /// ([`json_to_feature`]), on the node the path located rather than on the top-level
    /// value alone.
    ///
    /// A retained value is arbitrary parsed JSON, and the binding says exactly that: the
    /// TS face narrows before it reads, because typing the value is the *contract*'s job
    /// and no consumer of this map knows the kind yet.
    #[ts(type = "{ [key in string]: unknown }")]
    pub fields: BTreeMap<String, JsonValue>,
    /// The artifact body's line count — the `line_count` primitive's feature, populated
    /// only when a kind composes it.
    pub body_lines: usize,
    /// The member's **rendered extent** in lines — the projection's line count, measured
    /// intrinsically off every unit rather than gated behind a composed primitive, since
    /// `extent` is node-scope and must decide over any kind's members. Distinct from
    /// [`body_lines`](Features::body_lines): that reads only where `line_count` is
    /// composed; this carries the projected body's size.
    ///
    /// `None` where nothing rendered a projection to measure — an embedded member read off
    /// a layout host's source, never a format's output. A file member always carries
    /// `Some` (its committed body is the projection), so an `extent` clause over it stays
    /// decidable; a `None` member's `extent` is undecidable rather than a zero read as a
    /// pass.
    pub rendered_lines: Option<usize>,
    /// The member's **rendered extent** in characters — the projection's character count,
    /// the second unit an `extent` clause measures in. `None` on the same terms as
    /// [`rendered_lines`](Features::rendered_lines).
    pub rendered_chars: Option<usize>,
    /// The ATX headings (`#`..`######`) in the body, in document order, with the
    /// `#` run and any closing `#` run trimmed (for `require_sections`). A `#`
    /// inside a fenced code block is not a heading.
    pub headings: Vec<String>,
    /// The body's ATX [`Section`]s (heading + the body span beneath it), in
    /// document order — the feature a `section_contains` clause decides over.
    /// A
    /// superset of [`headings`](Features::headings): where `headings` carries only
    /// each heading's text, a section pairs it with its body span so a marker check
    /// has prose to search.
    pub sections: Vec<Section>,
    /// The name of the directory the unit was imported from, off provenance
    /// (for `name-matches-dir`). `None` when the source path has no parent.
    pub source_dir: Option<String>,
    /// The body's format-executed directive occurrences, in document order — the
    /// `at-import` `@path` targets a `directives` primitive extracts.
    /// A body-derived feature like [`headings`](Features::headings)/[`sections`](Features::sections):
    /// the raw occurrence strings only, resolution/classing a later slice. Empty
    /// when the kind composes no `directives` primitive.
    pub directives: Vec<String>,
    /// The body's fenced code blocks, in document order — each block's info string
    /// paired with its interior content, the feature a `fenced` primitive yields.
    /// A body-derived feature like
    /// [`headings`](Features::headings)/[`sections`](Features::sections)/[`directives`](Features::directives):
    /// the same fence boundaries the heading extractor tracks, surfaced whole. Empty
    /// when the kind composes no `fenced` primitive.
    pub fenced_blocks: Vec<FencedBlock>,
    /// The host member's own **nested members** — its declared [`EmbeddedMember`]s,
    /// read off the lock's own `Declarations::nested_members` rows by this member's
    /// `kind:name` address ([`nested_members_from_rows`]), never mined from
    /// [`fenced_blocks`](Features::fenced_blocks) (0018, "the projection is not the
    /// database"). Empty when the lock carries no row for this member.
    pub nested_members: Vec<EmbeddedMember>,
    /// The requirements this artifact opts into filling — the authored
    /// `[representation].satisfies` bindings, surfaced for the coverage check.
    /// This is a *representation* edge
    /// the coverage resolver reads, NOT
    /// a contract-checkable frontmatter field — so it lives here, distinct from
    /// `fields`, and never resolves through [`Features::field`]. The authored
    /// `rationale` is deliberately absent: it is the human *why*, never a
    /// decidable feature.
    pub satisfies: Vec<String>,
    /// Each edge this member *carries*, paired with whether the format that renders the
    /// member placed it — the feature a `format-places-edges` clause decides over. The
    /// carried set is the lock's `assembly` `edge` facts for this member's kind, narrowed
    /// to the fields its own row fills; the placed set is its
    /// [`NestedMemberRow::placed_edges`](crate::drift::NestedMemberRow::placed_edges),
    /// which `emit` captured while rendering. It arrives as a declaration row because
    /// the engine never sees the `render` hook and never reads a projection back.
    ///
    /// The two ways a member offers nothing to indict stay apart, because an empty map
    /// standing for both is what would leave the clause undecidable: `None` is no format
    /// at all (a layout host's document is source, so no rendering happened, and a kind
    /// declaring no edge places nothing either way), `Some` over an empty map is a format
    /// that ran with no carried edge to place. Both hold; neither is a fabricated pass.
    pub edge_placements: Option<BTreeMap<String, bool>>,
}

impl Features {
    /// Resolve a frontmatter field by name — the generic accessor a clause's
    /// `field` reference goes through, so the engine holds no per-kind opinion.
    ///
    /// Flat by name, never a path: this is the top-level lookup the extraction face and
    /// the read family ask for. A clause's addressing goes through [`Features::locate`],
    /// which reaches the same value for a bare name and walks nesting for a path.
    #[must_use]
    pub fn field(&self, name: &str) -> Option<FeatureValue> {
        self.fields.get(name).map(json_to_feature)
    }

    /// Whether a frontmatter field/key by this name is present (for `required`
    /// and `forbidden_keys`).
    #[must_use]
    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }

    /// The member's fields as the one JSON object an addressing path is rooted at.
    ///
    /// Built per call rather than held: the fields are the retained parse, and this tool
    /// reads kilobyte files — a clone per clause costs nothing a reader should pay for in
    /// indirection.
    #[must_use]
    fn root(&self) -> JsonValue {
        JsonValue::Object(
            self.fields
                .iter()
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect(),
        )
    }

    /// Every value `path` addresses on this member, each paired with the concrete address
    /// it resolved to — one for a path of name segments, one per element under each
    /// `[*]`. Empty when the path resolves nowhere: absent, never errored.
    #[must_use]
    pub fn locate(&self, path: &crate::address::FieldPath) -> Vec<(String, FeatureValue)> {
        let root = self.root();
        path.locate(&root)
            .into_iter()
            .map(|(address, value)| (address, json_to_feature(value)))
            .collect()
    }

    /// Whether each node `path`'s parent addresses carries `path`'s trailing key — the
    /// presence face, paired with the concrete address the key would have had.
    ///
    /// An absent key locates no node, so presence cannot be read off [`Features::locate`]:
    /// it is the *parent* that must be located and then asked. A path ending in `[*]`
    /// names elements rather than a key and yields nothing here; admissibility refuses a
    /// presence clause spelled that way, so no live clause reaches it.
    #[must_use]
    pub fn locate_presence(&self, path: &crate::address::FieldPath) -> Vec<(String, bool)> {
        let Some((parent, leaf)) = path.split_leaf() else {
            return Vec::new();
        };
        let root = self.root();
        parent
            .locate(&root)
            .into_iter()
            .map(|(address, value)| {
                let address = if address.is_empty() {
                    leaf.to_string()
                } else {
                    format!("{address}.{leaf}")
                };
                // Only an object carries a key to be present: an array or a scalar met
                // where the path expects one answers "absent", never a forged hit.
                (address, value.get(leaf).is_some())
            })
            .collect()
    }

    /// Every nested member's leaf as a fully-qualified [`MemberAddress`] paired with
    /// its authored value — the leaf-grain surface the read family (`impact`,
    /// `context`) consumes. Each
    /// address carries this member's id, so a citation resolving to a leaf resolves to a
    /// unique point across the corpus.
    #[must_use]
    pub fn embedded_leaves(&self) -> Vec<(MemberAddress, &str)> {
        let mut out = Vec::new();
        for member in &self.nested_members {
            for (child_path, leaf) in member.addressed_leaves() {
                out.push((
                    MemberAddress {
                        member: self.id.clone(),
                        kind: member.kind.clone(),
                        key: member.key.clone(),
                        child_path,
                    },
                    leaf,
                ));
            }
        }
        out
    }
}

/// One node of a markdown body's **heading tree**: an ATX heading, the body span
/// beneath it (byte-faithful, exactly as a [`Section`]'s body — the deeper
/// subsections stay part of the span as text), and the immediate deeper headings
/// nested under it as their own nodes. A [`Section`] is this same heading+span
/// pair flattened; the tree adds the parent→child nesting a member collection reads
/// (a collection heading's child headings are each one member), tracked off the same
/// ATX/fence primitives so the two views never disagree on what a heading is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingNode {
    /// The heading text, `#` markers stripped exactly as [`body_headings`] strips them.
    pub heading: String,
    /// The heading's ATX level (1..=6) — the nesting depth the tree is built by.
    pub level: usize,
    /// The body span beneath the heading, up to the next heading of the same or a
    /// shallower level — the intervening lines rejoined with `\n`, deeper subsections
    /// included as text, exactly as [`Section::body`].
    pub body: String,
    /// The immediate deeper headings nested under this one, in document order.
    pub children: Vec<HeadingNode>,
}

/// The heading lines of a byte-faithful markdown body *outside* fenced code — each
/// with its line index, ATX level, and stripped text, in document order. The section
/// boundaries [`body_sections`] and [`body_heading_tree`] both partition, collected
/// once off the shared [`atx_heading`]/[`track_fence`] primitives so neither view
/// forks a second heading scan.
fn collect_heads(lines: &[&str]) -> Vec<(usize, usize, String)> {
    let mut heads = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    for (index, line) in lines.iter().enumerate() {
        if track_fence(line, &mut fence) {
            continue;
        }
        if fence.is_none()
            && let Some((level, text)) = atx_heading(line)
        {
            heads.push((index, level, text));
        }
    }
    heads
}

/// Build the byte-faithful markdown body's **heading tree**: the top-level headings
/// (the shallowest ATX level present), each carrying the immediate deeper headings
/// nested under it, to arbitrary depth. The span and fence semantics are
/// [`body_sections`]'s exactly — the tree is that flat section list re-partitioned by
/// level, off the one [`collect_heads`] scan — so a member collection reads a
/// collection heading's children the same way `section_contains` reads a section's
/// body. A body with no heading yields no nodes (its whole text is preamble, the
/// reader's to place).
///
/// `pub(crate)` so the [`crate::kind`] layout reader stands on this exact heading
/// substrate rather than a second parser that could drift from the section/fence logic.
pub(crate) fn body_heading_tree(body: &str) -> Vec<HeadingNode> {
    let lines: Vec<&str> = body.lines().collect();
    let heads = collect_heads(&lines);
    build_heading_nodes(&heads, &lines, 0, heads.len(), lines.len())
}

/// Recursively partition the flat `heads` slice `[lo, hi)` into nested [`HeadingNode`]s.
/// Each node at this level owns the contiguous run of strictly-deeper headings that
/// follow it (its descendants) until the next heading of the same or a shallower level;
/// that run recurses into the node's own children. `range_end` is the line the
/// enclosing span ends at, so the final node's body runs to the parent's end rather
/// than the whole document.
fn build_heading_nodes(
    heads: &[(usize, usize, String)],
    lines: &[&str],
    lo: usize,
    hi: usize,
    range_end: usize,
) -> Vec<HeadingNode> {
    let mut nodes = Vec::new();
    let mut i = lo;
    while i < hi {
        let (start, level, ref text) = heads[i];
        // Advance past every strictly-deeper heading — the node's descendants — to the
        // next same-or-shallower sibling (or the range end).
        let mut j = i + 1;
        while j < hi && heads[j].1 > level {
            j += 1;
        }
        let child_end = if j < hi { heads[j].0 } else { range_end };
        let body = lines[start + 1..child_end].join("\n");
        let children = build_heading_nodes(heads, lines, i + 1, j, child_end);
        nodes.push(HeadingNode {
            heading: text.clone(),
            level,
            body,
            children,
        });
        i = j;
    }
    nodes
}

/// The verbatim **preamble** of a byte-faithful markdown body — the text before its
/// first ATX heading (fence-aware, off the shared [`collect_heads`] scan), rejoined
/// with `\n`. The whole body when it carries no heading. The span a layout's leading
/// prose region lands.
///
/// `pub(crate)` so the [`crate::kind`] layout reader places a verbatim prose region off
/// the same heading boundaries [`body_heading_tree`] partitions.
pub(crate) fn body_preamble(body: &str) -> String {
    let lines: Vec<&str> = body.lines().collect();
    let heads = collect_heads(&lines);
    let end = heads.first().map_or(lines.len(), |head| head.0);
    lines[..end].join("\n")
}

/// The line count of a byte-faithful markdown body — the `line_count` primitive's feature.
/// A single home for the count so the per-kind projectors and the data-driven
/// [`crate::kind`] composer read it the identical way rather than each writing
/// `body.lines().count()` inline.
///
/// `pub(crate)` so the closed extraction algebra composes the *same* deterministic extractor a built-in
/// kind's engine code uses, never a second implementation that could drift.
pub(crate) fn body_line_count(body: &str) -> usize {
    body.lines().count()
}

/// Extract the ATX headings (`#`..`######`) from a byte-faithful markdown body,
/// in document order. A `#` inside a fenced code block (```` ``` ```` or `~~~`)
/// is not a heading — that exclusion is what keeps the feature deterministic
/// rather than a guess. Each returned string is the heading text with its
/// leading `#` run, the required separating space, and any closing `#` run
/// trimmed off.
///
/// `pub(crate)` so the data-driven [`crate::kind`] composer reuses this exact
/// ATX/fence logic rather than reimplementing it.
pub(crate) fn body_headings(body: &str) -> Vec<String> {
    let lines: Vec<&str> = body.lines().collect();
    let heads = collect_heads(&lines);
    heads.into_iter().map(|(_, _, text)| text).collect()
}

/// Extract the ATX **sections** of a byte-faithful markdown body: each heading
/// paired with the body span beneath it, up to the next heading of the same or a
/// shallower level (a deeper subsection stays part of its parent's span). The
/// feature a `section_contains` clause reads. A heading (and any `#` line) inside a
/// fenced code block opens no section — the same exclusion [`body_headings`] makes,
/// tracked the identical way — so a fenced marker never splits the prose. Heading
/// text is stripped exactly as [`body_headings`] strips it; the body is the
/// intervening lines rejoined with `\n`, the span a marker check searches.
///
/// `pub(crate)` so the data-driven [`crate::kind`] `sections` primitive composes
/// this exact splitter rather than a second one that could drift from the heading
/// logic.
pub(crate) fn body_sections(body: &str) -> Vec<Section> {
    let lines: Vec<&str> = body.lines().collect();
    // First pass: the heading lines *outside* fenced code, each with its line
    // index, level, and stripped text — the section boundaries.
    let heads = collect_heads(&lines);

    // Second pass: each heading's body runs to the next heading of the same or a
    // shallower level (`next_level <= level`), so a subsection nests inside its
    // parent's span rather than truncating it.
    let mut sections = Vec::with_capacity(heads.len());
    for (position, (start, level, text)) in heads.iter().enumerate() {
        let end = heads[position + 1..]
            .iter()
            .find(|head| head.1 <= *level)
            .map_or(lines.len(), |head| head.0);
        let body = lines[*start + 1..end].join("\n");
        sections.push(Section {
            heading: text.clone(),
            body,
        });
    }
    sections
}

/// Extract the **fenced code blocks** of a byte-faithful markdown body, in document
/// order: each block's info string (the text after the opening fence, trimmed)
/// paired with its interior content (the lines between the fences, rejoined with
/// `\n`). The feature a `fenced` primitive yields. A block opens on a
/// fence marker and closes on the next marker of the **same char and at least the
/// opening length** — the identical fence tracking [`body_headings`] runs, so a
/// heading or a shorter/different marker *inside* a block is interior content, never
/// a nested open. An unterminated fence runs to the end of the body (CommonMark), so
/// its block is still yielded rather than silently dropped. Surrounding prose is
/// skipped — only the interior is captured — and a body with no fence yields none.
///
/// `pub(crate)` so the data-driven [`crate::kind`] `fenced` primitive composes this
/// exact reader rather than a second one that could drift from the heading/section
/// fence logic.
pub(crate) fn body_fenced_blocks(body: &str) -> Vec<FencedBlock> {
    let mut blocks = Vec::new();
    // The open fence's char, run length, info string, and the interior lines
    // gathered so far — `Some` while inside a fenced block.
    let mut open: Option<(char, usize, String, Vec<String>)> = None;
    for line in body.lines() {
        if let Some((fence_char, fence_len)) = fence_marker(line) {
            match open.take() {
                None => {
                    // A fence opens: its info string is the text after the fence run.
                    open = Some((
                        fence_char,
                        fence_len,
                        fence_info(line, fence_char),
                        Vec::new(),
                    ));
                }
                Some((open_char, open_len, info, content))
                    if fence_char == open_char && fence_len >= open_len =>
                {
                    // A matching closing fence — the block is complete. `open` stays
                    // `None` (taken above).
                    blocks.push(FencedBlock {
                        info,
                        content: content.join("\n"),
                    });
                }
                Some((open_char, open_len, info, mut content)) => {
                    // A marker of a different char or shorter length inside an open
                    // block is interior content, not a close (the `Some(_) => {}` case
                    // `body_headings` treats as non-heading, captured here).
                    content.push(line.to_string());
                    open = Some((open_char, open_len, info, content));
                }
            }
            continue;
        }
        if let Some((.., content)) = &mut open {
            content.push(line.to_string());
        }
    }
    // An unterminated fence extends to the end of the body (CommonMark) — yield its
    // block rather than lose the captured content.
    if let Some((.., info, content)) = open {
        blocks.push(FencedBlock {
            info,
            content: content.join("\n"),
        });
    }
    blocks
}

/// The info string of a fenced-block opening `line` — the text after the fence
/// character run, trimmed (`` ```sh `` → `sh`, a bare `` ``` `` → empty). The
/// caller has already confirmed via [`fence_marker`] that `line` (leading spaces
/// aside) opens with a run of `fence_char`, so trimming that run then the
/// surrounding whitespace leaves the declared kind the embedded-member consumer
/// keys on.
fn fence_info(line: &str, fence_char: char) -> String {
    line.trim_start_matches(' ')
        .trim_start_matches(fence_char)
        .trim()
        .to_string()
}

/// This host member's own `kind:name` lock address — the key
/// [`crate::drift::NestedMemberRow::host`] carries, the identical `${kind}:${name}`
/// form `sdk/src/declarations.ts`'s `nestedMemberRows` writes it in.
#[must_use]
pub fn host_address(kind: &str, id: &str) -> String {
    format!("{kind}:{id}")
}

/// The fence marker a line carries, if any: the fence character (`` ` `` or
/// `~`) and its run length (≥3). Up to three leading spaces are allowed before
/// the run; four or more is an indented code block, not a fence. Heading and
/// section extraction use it to skip fenced code — a `#` inside a fence is
/// illustration, not an ATX heading.
fn fence_marker(line: &str) -> Option<(char, usize)> {
    let rest = line.trim_start_matches(' ');
    if line.len() - rest.len() >= 4 {
        return None;
    }
    let fence_char = rest.chars().next().filter(|&c| c == '`' || c == '~')?;
    let len = rest.chars().take_while(|&c| c == fence_char).count();
    (len >= 3).then_some((fence_char, len))
}

/// Advance a fence-tracking scan by one `line`, updating `fence` in place;
/// returns whether `line` was a fence marker, so callers `continue` past it.
/// A closing fence matches the opener's char and is at least as long;
/// anything else inside a fence is just content. Shared by [`body_headings`],
/// [`body_sections`], and [`body_at_imports`], whose fence exclusion is
/// otherwise a byte-identical match on [`fence_marker`]'s result.
fn track_fence(line: &str, fence: &mut Option<(char, usize)>) -> bool {
    let Some((fence_char, fence_len)) = fence_marker(line) else {
        return false;
    };
    match *fence {
        Some((open_char, open_len)) if fence_char == open_char && fence_len >= open_len => {
            *fence = None;
        }
        Some(_) => {}
        None => *fence = Some((fence_char, fence_len)),
    }
    true
}

/// The **level and text** of an ATX heading on this line, or `None` if it is not
/// one. A heading is up to three leading spaces, a `#`..`######` run (the level),
/// then a space/tab (or end of line); the returned text has the markers and an
/// optional closing `#` run stripped. [`body_headings`] reads only the text;
/// [`body_sections`] also reads the level to nest subsections inside their parent.
fn atx_heading(line: &str) -> Option<(usize, String)> {
    let rest = line.trim_start_matches(' ');
    if line.len() - rest.len() >= 4 {
        return None;
    }
    let level = rest.chars().take_while(|&c| c == '#').count();
    if level == 0 || level > 6 {
        return None;
    }
    let after = &rest[level..];
    // The `#` run must be followed by whitespace or end the line, else the `#`s
    // are content (e.g. `#tag`), not a heading marker.
    if !after.is_empty() && !after.starts_with([' ', '\t']) {
        return None;
    }
    let text = after.trim();
    // A trailing `#` run is a closing sequence only when whitespace separates it
    // from the text (CommonMark); a `#` glued to a word stays content.
    let stripped = text.trim_end_matches('#');
    let text = if stripped.is_empty() {
        ""
    } else if stripped.len() != text.len() && stripped.ends_with([' ', '\t']) {
        stripped.trim_end()
    } else {
        text
    };
    Some((level, text.to_string()))
}

/// The name of the directory the artifact was imported from (the folder Claude
/// Code discovers it under), off its `provenance.source_path`.
///
/// `pub(crate)` so the data-driven [`crate::kind`] composer reads the file
/// placement feature the identical way.
pub(crate) fn source_dir_name(source_path: &Path) -> Option<String> {
    source_path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .map(str::to_string)
}

/// Extract the `at-import` directive occurrences (`@path/to/file`) from a
/// byte-faithful markdown body, in document order — the raw path strings, one per
/// occurrence. An `@` opens an import only at a word boundary (start of line or after
/// whitespace), so an email `user@host` and a bare `@` in prose yield nothing; the
/// occurrence is the run of non-whitespace after the `@` ("Both relative and absolute
/// paths are allowed"; code.claude.com/docs/en/memory, retrieved 2026-07-16). A `@path`
/// inside a fenced code block or an inline code span is illustration the harness does
/// not execute ("Import parsing skips Markdown code spans and fenced code blocks", same
/// retrieval), so it is skipped — the fence exclusion [`body_headings`] makes,
/// extended to inline spans, is what keeps the extraction sound rather than a guess.
/// Resolution/classing is a later slice; this yields the raw occurrence strings only.
///
/// `pub(crate)` so the data-driven [`crate::kind`] `directives` primitive composes
/// this exact reader rather than a second one that could drift.
pub(crate) fn body_at_imports(body: &str) -> Vec<String> {
    let mut imports = Vec::new();
    // The open fence's char and run length, while inside a fenced code block.
    let mut fence: Option<(char, usize)> = None;
    for line in body.lines() {
        if track_fence(line, &mut fence) {
            continue;
        }
        if fence.is_none() {
            line_at_imports(line, &mut imports);
        }
    }
    imports
}

/// Collect the `at-import` occurrences on a single non-fenced line into `imports`,
/// skipping inline code spans so a `` `@path` `` mention is typography, not an edge.
/// An `@` opens an import only at a word boundary; the path is the run of
/// non-whitespace after it.
fn line_at_imports(line: &str, imports: &mut Vec<String>) {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    // Whether the previous character was a word boundary — start of line counts, so a
    // leading `@` opens an import, while an `@` glued to a preceding word (`user@host`)
    // does not.
    let mut boundary = true;
    while i < chars.len() {
        let c = chars[i];
        if c == '`' {
            i = skip_code_span(&chars, i);
            boundary = false;
            continue;
        }
        if c == '@' && boundary {
            let start = i + 1;
            let mut end = start;
            while end < chars.len() && !chars[end].is_whitespace() && chars[end] != '`' {
                end += 1;
            }
            if end > start {
                imports.push(chars[start..end].iter().collect());
            }
            i = end;
            boundary = false;
            continue;
        }
        boundary = c.is_whitespace();
        i += 1;
    }
}

/// The index just past the inline code span opening at `start` (a run of backticks):
/// its matching closing run of the same length, or — when nothing closes it — just
/// past the opening run, leaving the stray backticks as literal text. Mirrors the
/// CommonMark rule that a run of N backticks is closed only by a run of exactly N.
fn skip_code_span(chars: &[char], start: usize) -> usize {
    let mut i = start;
    while i < chars.len() && chars[i] == '`' {
        i += 1;
    }
    let open_len = i - start;
    let after_open = i;
    while i < chars.len() {
        if chars[i] == '`' {
            let run_start = i;
            while i < chars.len() && chars[i] == '`' {
                i += 1;
            }
            if i - run_start == open_len {
                return i;
            }
        } else {
            i += 1;
        }
    }
    after_open
}

/// Project an `extra` frontmatter value into a [`FeatureValue`], preserving its
/// parsed source [`ValueType`]: arrays become a list, objects a map, and each scalar
/// keeps the kind it parsed as (`string`/`integer`/`number`/`boolean`/`null`)
/// alongside its text. Stringifying every scalar to a bare string — the slice-1
/// shortcut — would make a `type` check undecidable; recording the kind here is
/// the precondition that check needs.
///
/// `pub(crate)` so the [`crate::kind`] `field` extraction primitive projects a
/// declared frontmatter value into a [`FeatureValue`] through the same
/// kind-preserving path, never a second projector.
pub(crate) fn json_to_feature(value: &JsonValue) -> FeatureValue {
    match value {
        JsonValue::Array(items) => {
            FeatureValue::List(items.iter().map(json_scalar_string).collect())
        }
        JsonValue::Object(_) => FeatureValue::Map,
        JsonValue::Null => FeatureValue::scalar(ValueType::Null, "null"),
        JsonValue::Bool(b) => FeatureValue::scalar(ValueType::Boolean, b.to_string()),
        JsonValue::Number(n) => FeatureValue::scalar(number_kind(n), n.to_string()),
        JsonValue::String(s) => FeatureValue::scalar(ValueType::String, s.clone()),
    }
}

/// The source kind of a JSON number: `integer` when it parsed as a whole number
/// (`i64`/`u64`), else `number` (a floating-point value).
fn number_kind(n: &serde_json::Number) -> ValueType {
    if n.is_i64() || n.is_u64() {
        ValueType::Integer
    } else {
        ValueType::Number
    }
}

/// Stringify a JSON scalar to its plain text form (no surrounding quotes for
/// strings); non-scalars fall back to their JSON text so a list element stays a
/// deterministic, comparable string.
fn json_scalar_string(value: &JsonValue) -> String {
    match value {
        JsonValue::String(s) => s.clone(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_sections_pair_each_heading_with_its_nested_span_and_skip_fences() {
        // Two `##` sections; the first nests a `###` subsection (which stays part of
        // the parent's span, `level <= parent`), the second's body is a fenced block
        // whose `#` line is not a heading and so opens no phantom section.
        let body = "# Title\n\
\n\
## Decision: one\n\
Chosen: A. Rejected: B.\n\
\n\
### Sub\n\
detail\n\
\n\
## Decision: two\n\
```sh\n\
# not a heading\n\
```\n\
tail\n";
        let sections = body_sections(body);

        let headings: Vec<&str> = sections.iter().map(|s| s.heading.as_str()).collect();
        assert_eq!(
            headings,
            vec!["Title", "Decision: one", "Sub", "Decision: two"]
        );

        // `Decision: one` runs to the next same-or-shallower heading (`## Decision:
        // two`), so its body absorbs the nested `### Sub` subsection.
        let one = &sections[1];
        assert!(one.body.contains("Chosen: A. Rejected: B."));
        assert!(one.body.contains("### Sub"));
        assert!(one.body.contains("detail"));

        // `Decision: two`'s body carries the fenced block verbatim (the fenced `#`
        // never split off a section) and the trailing line.
        let two = &sections[3];
        assert!(two.body.contains("# not a heading"));
        assert!(two.body.contains("tail"));
    }

    #[test]
    fn at_imports_are_extracted_in_document_order_and_bare_at_is_skipped() {
        // Two `@path` occurrences (relative then absolute), a bare `@` in prose, and an
        // email `user@host` — only the two real imports are edges, in document order.
        let body = "# Memory\n\
\n\
Bring in @config/base.md and @/abs/shared.md here.\n\
\n\
Ping me @ the standup, or at user@example.com.\n";
        assert_eq!(
            body_at_imports(body),
            vec!["config/base.md".to_string(), "/abs/shared.md".to_string()]
        );
    }

    #[test]
    fn at_imports_inside_code_are_typography_not_edges() {
        // A `@path` inside a fenced block or an inline code span is illustration the
        // harness never executes (code.claude.com/docs/en/memory) — skipped, so the
        // extractor stays sound. The lone real import outside code is the only edge.
        let body = "The `@path` syntax is documented; see @real/import.md in context.\n\
\n\
```text\n\
@fenced/not-an-edge.md\n\
```\n";
        assert_eq!(body_at_imports(body), vec!["real/import.md".to_string()]);
    }

    #[test]
    fn at_imports_are_order_stable_across_re_extraction() {
        // A pure function of the body — the same body yields the same occurrences,
        // the property that makes the directive a sound edge input.
        let body = "@a/one.md and @b/two.md and @c/three.md\n";
        assert_eq!(body_at_imports(body), body_at_imports(body));
        assert_eq!(
            body_at_imports(body),
            vec![
                "a/one.md".to_string(),
                "b/two.md".to_string(),
                "c/three.md".to_string()
            ]
        );
    }

    #[test]
    fn fenced_blocks_capture_interiors_and_info_strings_in_document_order() {
        // Two blocks with info strings, prose around and between — only the interiors
        // are captured, in document order, each with its trimmed info string.
        let body = "# Title\n\
\n\
prose above\n\
\n\
```sh\n\
cargo test\n\
```\n\
\n\
prose between\n\
\n\
```toml member.manifest\n\
name = \"x\"\n\
count = 2\n\
```\n\
\n\
prose below\n";
        let blocks = body_fenced_blocks(body);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].info, "sh");
        assert_eq!(blocks[0].content, "cargo test");
        assert_eq!(blocks[1].info, "toml member.manifest");
        assert_eq!(blocks[1].content, "name = \"x\"\ncount = 2");
    }

    #[test]
    fn a_body_with_no_fence_yields_no_blocks() {
        // Absent, never errored — the default a `fenced` primitive lands on.
        assert!(body_fenced_blocks("# Only prose\n\nno fence here at all.\n").is_empty());
        assert!(body_fenced_blocks("").is_empty());
    }

    #[test]
    fn a_bare_fence_and_an_empty_block_are_captured() {
        // A bare ``` opens a block with an empty info string; an immediately-closed
        // fence yields an empty content span — never dropped.
        let blocks = body_fenced_blocks("```\nplain\n```\n\n```\n```\n");
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].info, "");
        assert_eq!(blocks[0].content, "plain");
        assert_eq!(blocks[1].info, "");
        assert_eq!(blocks[1].content, "");
    }

    #[test]
    fn a_heading_or_inner_marker_inside_a_fence_is_interior_content() {
        // The same fence tracking `body_headings` runs: a `#` line and a shorter/other
        // marker inside the block are interior content, not a heading or a nested open.
        let body = "```text\n\
# not a heading\n\
~~~ not a close\n\
`` short\n\
```\n";
        let blocks = body_fenced_blocks(body);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].info, "text");
        assert_eq!(
            blocks[0].content,
            "# not a heading\n~~~ not a close\n`` short"
        );
        // And the fenced `#` opens no section — the two extractors agree on the fence.
        assert!(body_headings(body).is_empty());
    }

    #[test]
    fn an_unterminated_fence_runs_to_the_end_of_the_body() {
        // CommonMark: an unclosed fence extends to end of document — the block is still
        // yielded (its content not silently lost), the same view `body_headings` takes
        // when a trailing fence swallows the remainder.
        let blocks = body_fenced_blocks("intro\n\n```sh\ncargo build\nno closing fence\n");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].info, "sh");
        assert_eq!(blocks[0].content, "cargo build\nno closing fence");
    }

    #[test]
    fn re_running_fenced_extraction_is_byte_identical() {
        // A pure function of the body — the property that makes a fenced block a sound
        // gate input.
        let body = "```toml\nk = 1\n```\n";
        assert_eq!(body_fenced_blocks(body), body_fenced_blocks(body));
    }

    #[test]
    fn each_lattice_name_round_trips_and_an_unknown_name_is_rejected() {
        // Every name in the closed lattice maps to its `ValueType` and renders back to
        // the same spelling — the single name table a `type` clause goes through.
        for kind in [
            ValueType::String,
            ValueType::Integer,
            ValueType::Number,
            ValueType::Boolean,
            ValueType::Null,
            ValueType::List,
            ValueType::Map,
        ] {
            assert_eq!(ValueType::from_name(kind.name()), Some(kind));
        }
        // The spellings are exactly the lattice's, nothing more.
        assert_eq!(ValueType::String.name(), "string");
        assert_eq!(ValueType::from_name("map"), Some(ValueType::Map));

        // A name outside the lattice yields `None` (the load-error signal a
        // `type` clause rejects on), never a silent default.
        assert_eq!(ValueType::from_name("array"), None);
        assert_eq!(ValueType::from_name("int"), None);
        assert_eq!(ValueType::from_name(""), None);
    }

    #[test]
    fn host_address_is_kind_colon_id() {
        assert_eq!(host_address("rule", "collaboration"), "rule:collaboration");
    }

    #[test]
    fn heading_tree_nests_children_under_their_parent_and_carries_spans() {
        // Two top-level `#` headings; the second nests two `##` children (a member
        // collection's members), each with its own `###` sub-heading. The tree pairs
        // each heading with its span and its immediate deeper children, off the same
        // ATX/fence scan `body_sections` runs.
        let body = "preamble line\n\
\n\
# Intent\n\
the intent span\n\
\n\
# Invariants\n\
\n\
## Determinism\n\
### key\n\
det-core\n\
\n\
## Idempotence\n\
### key\n\
idem-core\n";
        let tree = body_heading_tree(body);

        let top: Vec<&str> = tree.iter().map(|node| node.heading.as_str()).collect();
        assert_eq!(top, vec!["Intent", "Invariants"]);

        // The field-section heading's span is its body, verbatim.
        assert!(tree[0].body.contains("the intent span"));
        assert!(tree[0].children.is_empty());

        // The collection heading's immediate children are the two members.
        let members: Vec<&str> = tree[1]
            .children
            .iter()
            .map(|node| node.heading.as_str())
            .collect();
        assert_eq!(members, vec!["Determinism", "Idempotence"]);

        // Each member carries its own `### key` sub-heading, one layer deeper.
        let first = &tree[1].children[0];
        assert_eq!(first.level, 2);
        assert_eq!(first.children.len(), 1);
        assert_eq!(first.children[0].heading, "key");
        assert!(first.children[0].body.contains("det-core"));
    }

    #[test]
    fn preamble_is_the_text_before_the_first_heading() {
        assert_eq!(
            body_preamble("lead one\nlead two\n\n# First\nunder\n"),
            "lead one\nlead two\n"
        );
        // A body with no heading is all preamble.
        assert_eq!(
            body_preamble("just prose\nno heading\n"),
            "just prose\nno heading"
        );
    }
}
