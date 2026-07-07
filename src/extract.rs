//! Extraction — an artifact's surface-decidable feature set.
//!
//! Models the "Extraction is the soundness boundary" section of
//! `specs/architecture/30-landscapes.md` (generalized by `specs/architecture/20-surface.md`, "The IR"): a
//! per-kind extractor projects a parsed artifact into a [`Features`] map the
//! generic contract engine reads. A contract clause is sound only because the
//! feature it names is **deterministically extractable** — so [`Features`]
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

/// A field's parsed source kind — the closed scalar/container lattice the `type`
/// primitive ranges over (`specs/architecture/10-contracts.md`, "Decision: the `type`
/// vocabulary is a closed scalar/container lattice"). Taken from the *parsed*
/// YAML/JSON value, not its stringified form: a sound `type` check needs the
/// extractor to preserve the source kind rather than collapse every scalar to a
/// bare string (the slice-1 shortcut this entry corrects). The five scalar kinds
/// answer [`FeatureValue::as_scalar`]; the two container kinds do not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub enum Kind {
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

impl Kind {
    /// The lattice name of this kind — the declared-type spelling a `type`
    /// clause uses and the form diagnostics render. The inverse of
    /// [`Kind::from_name`].
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Kind::String => "string",
            Kind::Integer => "integer",
            Kind::Number => "number",
            Kind::Boolean => "boolean",
            Kind::Null => "null",
            Kind::List => "list",
            Kind::Map => "map",
        }
    }

    /// Parse a declared type name into its [`Kind`], or `None` if it is not one
    /// of the closed lattice's names. This is the single home of the lattice's
    /// name table (`specs/architecture/10-contracts.md`, "Decision: the `type` vocabulary is
    /// a closed scalar/container lattice"); the contract parser maps a declared
    /// `type` through here rather than duplicating the spelling.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Kind> {
        match name {
            "string" => Some(Kind::String),
            "integer" => Some(Kind::Integer),
            "number" => Some(Kind::Number),
            "boolean" => Some(Kind::Boolean),
            "null" => Some(Kind::Null),
            "list" => Some(Kind::List),
            "map" => Some(Kind::Map),
            _ => None,
        }
    }
}

/// One extracted feature value: a scalar field (carrying its parsed source
/// [`Kind`] alongside its comparison text), a list field (e.g. a YAML sequence
/// like `allowed-tools`), or a map field. Scalar predicates (`min_len`, `enum`,
/// `deny`, `allowed_chars`) read the scalar text; presence predicates
/// (`required`, `forbidden_keys`) need only the key; the `type` primitive
/// (forthcoming) reads [`FeatureValue::kind`].
#[derive(Debug, Clone, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub enum FeatureValue {
    /// A single scalar value: its parsed source kind (one of the scalar kinds —
    /// `string`/`integer`/`number`/`boolean`/`null`) and its stringified text,
    /// the comparison text the scalar predicates read.
    Scalar {
        /// The parsed source kind of the scalar.
        kind: Kind,
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
    pub fn scalar(kind: Kind, text: impl Into<String>) -> Self {
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
    /// over. A list is always [`Kind::List`] and a map [`Kind::Map`]; a scalar
    /// reports the kind it was parsed as.
    #[must_use]
    pub fn kind(&self) -> Kind {
        match self {
            FeatureValue::Scalar { kind, .. } => *kind,
            FeatureValue::List(_) => Kind::List,
            FeatureValue::Map => Kind::Map,
        }
    }
}

/// One ATX **section** of a markdown body: a heading paired with the body span
/// beneath it, up to the next heading of the same or a shallower level. The
/// feature a `section_contains` clause decides over (`specs/architecture/10-contracts.md`, the
/// `section_contains` structural primitive) — its [`heading`](Section::heading) is
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
/// the opening fence — `sh`, `toml`, `toml genre.foo` — trimmed) paired with the
/// block's **interior content** (the lines between the fences, rejoined with `\n`).
/// The feature a `fenced` primitive yields (`specs/architecture/15-kinds.md`, "The
/// engine is kind-blind — extraction is generic"): fenced extraction
/// composed with a TOML parse yields a genre value's features, declared data at
/// body position. Surface-decidable like every other feature — the fence
/// boundaries are the ones [`body_headings`] already tracks, so a block is never a
/// guess. The info string is available so the genre consumer can key on
/// `genre.<name>`; this generic primitive yields the raw blocks only, the TOML
/// composition a later slice.
#[derive(Debug, Clone, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub struct FencedBlock {
    /// The opening fence's info string, trimmed — `sh`, `toml`, or empty for a bare
    /// fence. The declared kind the genre consumer keys on.
    pub info: String,
    /// The block's interior content — the lines between the fences, rejoined with
    /// `\n`, byte-faithful to the body span exactly as a [`Section`]'s body is.
    pub content: String,
}

/// A genre value's **sibling collections** — collection name → (entry key → the entry's
/// own prose leaves), so a collection leaf addresses as `<collection>.<entry>.<field>`
/// (`rejected.baked-projection.because`). Keyed at every level, never positional
/// (`specs/architecture/20-surface.md`, "Decision: one read verb — `explain`"). Named for the
/// three-deep map the read side and the lock serializer share.
pub type GenreCollections = BTreeMap<String, BTreeMap<String, BTreeMap<String, String>>>;

/// A **genre value** — a kind's recurring prose form given typed shape
/// (`specs/architecture/15-kinds.md`, "A genre is a kind at the block locus"),
/// extracted from a genre fence at the floor. It carries the genre it
/// instantiates (`decision`) and the fence key that names this instance among its
/// siblings (`surface-authority`), plus its meaning-carrying **prose leaves**:
/// top-level authored strings, and **sibling collections** — keyed sub-tables
/// (`rejected.baked-projection`), never positional. Every leaf is addressed
/// structurally (member + genre + key + field path) so drift, `impact`, and
/// citations survive rewording ([`GenreValue::addressed_leaves`]).
///
/// Floor leaves carry no mentions — interpolation stays deferred until a floor
/// mention syntax is separately ratified (`specs/architecture/20-surface.md`) — so a
/// leaf is a plain [`String`], not a mention-bearing span.
#[derive(Debug, Clone, PartialEq, Eq, schemars::JsonSchema, ts_rs::TS)]
pub struct GenreValue {
    /// The genre this value instantiates — the fence info string's `genre.<genre>`
    /// (`decision`), one of the kind's declared genres.
    pub genre: String,
    /// The fence key naming this instance among its siblings in the same member —
    /// the info string's second token (`surface-authority`). Part of a leaf's
    /// address, so it is keyed, never positional.
    pub key: String,
    /// The value's top-level **prose leaves** — field name → authored string, in
    /// stable (sorted) key order so serialization is deterministic.
    pub leaves: BTreeMap<String, String>,
    /// The value's **sibling collections** — collection name → (entry key → the
    /// entry's own prose leaves), so a collection leaf addresses as
    /// `<collection>.<entry>.<field>` (`rejected.baked-projection.because`). Keyed at
    /// every level, never positional — an address that survives insertion and reorder
    /// (`specs/architecture/20-surface.md`, "Decision: one read verb — `explain`").
    pub collections: GenreCollections,
}

impl GenreValue {
    /// Every leaf's **structural field path** paired with its authored value, in
    /// stable order (`specs/architecture/20-surface.md`, "Decision: one read verb —
    /// `explain`"): a top-level leaf's bare field name (`chosen`), a collection leaf's
    /// `<collection>.<entry>.<field>` (`rejected.baked-projection.because`). The path
    /// rides the structure the author already wrote, so it is stable under content
    /// edits — the property drift routing and `impact` stand on.
    #[must_use]
    pub fn addressed_leaves(&self) -> Vec<(String, &str)> {
        let mut out = Vec::new();
        for (field, value) in &self.leaves {
            out.push((field.clone(), value.as_str()));
        }
        for (collection, entries) in &self.collections {
            for (entry, leaves) in entries {
                for (field, value) in leaves {
                    out.push((format!("{collection}.{entry}.{field}"), value.as_str()));
                }
            }
        }
        out
    }
}

/// The **structural address** of a genre-value leaf (`specs/architecture/20-surface.md`,
/// "Decision: one read verb — `explain`"): the member it lives in, the genre value's
/// identity (genre name + fence key), and the field path within that value. Keyed at
/// every level and stable under content edits, so a citation targeting a leaf
/// (`specs/architecture/45-governance.md`) and `impact` at leaf grain survive rewording —
/// only a key *rename* breaks it, and then to the resolution check, which tells the citer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafAddress {
    /// The member the leaf lives in (`Features::id`).
    pub member: String,
    /// The genre the value instantiates (`decision`).
    pub genre: String,
    /// The fence key naming the value among its siblings (`surface-authority`).
    pub key: String,
    /// The field path within the value — a bare leaf name or a
    /// `<collection>.<entry>.<field>` path.
    pub field_path: String,
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
    pub fields: BTreeMap<String, FeatureValue>,
    /// The artifact body's line count (for `max_lines`).
    pub body_lines: usize,
    /// The ATX headings (`#`..`######`) in the body, in document order, with the
    /// `#` run and any closing `#` run trimmed (for `require_sections`). A `#`
    /// inside a fenced code block is not a heading.
    pub headings: Vec<String>,
    /// The body's ATX [`Section`]s (heading + the body span beneath it), in
    /// document order — the feature a `section_contains` clause decides over
    /// (`specs/architecture/10-contracts.md`, the `section_contains` structural primitive). A
    /// superset of [`headings`](Features::headings): where `headings` carries only
    /// each heading's text, a section pairs it with its body span so a marker check
    /// has prose to search.
    pub sections: Vec<Section>,
    /// The name of the directory the unit was imported from, off provenance
    /// (for `name-matches-dir`). `None` when the source path has no parent.
    pub source_dir: Option<String>,
    /// The body's format-executed directive occurrences, in document order — the
    /// `at-import` `@path` targets a `directives` primitive extracts
    /// (`specs/architecture/15-kinds.md`, "Directives — format-executed body syntax").
    /// A body-derived feature like [`headings`](Features::headings)/[`sections`](Features::sections):
    /// the raw occurrence strings only, resolution/classing a later slice. Empty
    /// when the kind composes no `directives` primitive.
    pub directives: Vec<String>,
    /// The body's fenced code blocks, in document order — each block's info string
    /// paired with its interior content, the feature a `fenced` primitive yields
    /// (`specs/architecture/15-kinds.md`, "The engine is kind-blind — extraction is
    /// generic"). A body-derived feature like
    /// [`headings`](Features::headings)/[`sections`](Features::sections)/[`directives`](Features::directives):
    /// the same fence boundaries the heading extractor tracks, surfaced whole. Empty
    /// when the kind composes no `fenced` primitive.
    pub fenced_blocks: Vec<FencedBlock>,
    /// The body's **genre values**, in document order — each a genre fence
    /// (`genre.<genre> <key>`) whose interior TOML parsed into a typed
    /// [`GenreValue`] (`specs/architecture/15-kinds.md`, "A genre is a kind at the block
    /// locus"). The typed layer
    /// over [`fenced_blocks`](Features::fenced_blocks): a raw block whose info string
    /// names a genre the kind declares is folded here beside its raw form, keyed by the
    /// fence's genre+key; every other fenced block stays raw-only. Empty when the kind
    /// declares no genres, or no block opts into one — genre adoption is per-block, and
    /// no check quantifies over its completeness.
    pub genres: Vec<GenreValue>,
    /// The requirements this artifact opts into filling — the authored
    /// `[representation].satisfies` bindings, surfaced for the coverage check
    /// (`specs/architecture/20-surface.md`, "Each artifact directory is a representation, not
    /// a copy"). This is a *representation* edge the coverage resolver reads, NOT
    /// a contract-checkable frontmatter field — so it lives here, distinct from
    /// `fields`, and never resolves through [`Features::field`]. The authored
    /// `rationale` is deliberately absent: it is the human *why*, never a
    /// decidable feature.
    pub satisfies: Vec<String>,
    /// The requirements this artifact **publishes** — the authored
    /// `[requirement.<name>]` header modules (`specs/architecture/10-contracts.md`, "Decision: a
    /// requirement's publisher is any authored surface document"). The demand side of
    /// the fill edge, carried beside `satisfies` (the fill side) so the gate gathers
    /// every member's published obligations across every kind and unions them with the
    /// assembly roster into the one requirement namespace. Like `satisfies`, this is a
    /// *representation* fact carried through, never a contract-checkable frontmatter
    /// field. Empty when the member publishes none.
    pub published_requirements: Vec<crate::document::PublishedRequirement>,
}

impl Features {
    /// Resolve a frontmatter field by name — the generic accessor a clause's
    /// `field` reference goes through, so the engine holds no per-kind opinion.
    #[must_use]
    pub fn field(&self, name: &str) -> Option<&FeatureValue> {
        self.fields.get(name)
    }

    /// Whether a frontmatter field/key by this name is present (for `required`
    /// and `forbidden_keys`).
    #[must_use]
    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }

    /// Every genre-value leaf as a fully-qualified [`LeafAddress`] paired with its
    /// authored value — the leaf-grain surface the read family (`impact`, `context`)
    /// consumes (`specs/architecture/20-surface.md`, "Decision: one read verb —
    /// `explain`"). Each
    /// address carries this member's id, so a citation resolving to a leaf resolves to a
    /// unique point across the corpus.
    #[must_use]
    pub fn genre_leaves(&self) -> Vec<(LeafAddress, &str)> {
        let mut out = Vec::new();
        for value in &self.genres {
            for (field_path, leaf) in value.addressed_leaves() {
                out.push((
                    LeafAddress {
                        member: self.id.clone(),
                        genre: value.genre.clone(),
                        key: value.key.clone(),
                        field_path,
                    },
                    leaf,
                ));
            }
        }
        out
    }
}

/// The line count of a byte-faithful markdown body — the `max_lines` feature.
/// A single home for the count so the per-kind projectors and the data-driven
/// [`crate::kind`] composer read it the identical way rather than each writing
/// `body.lines().count()` inline.
///
/// `pub(crate)` so the closed extraction algebra (`specs/architecture/15-kinds.md`, "The
/// extraction algebra") composes the *same* deterministic extractor a built-in
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
/// ATX/fence logic rather than reimplementing it (`specs/architecture/15-kinds.md`).
pub(crate) fn body_headings(body: &str) -> Vec<String> {
    let mut headings = Vec::new();
    // The open fence's char and run length, while inside a fenced code block.
    let mut fence: Option<(char, usize)> = None;
    for line in body.lines() {
        if let Some((fence_char, fence_len)) = fence_marker(line) {
            match fence {
                // A closing fence matches the opener's char and is at least as
                // long; anything else inside a fence is just content.
                Some((open_char, open_len)) if fence_char == open_char && fence_len >= open_len => {
                    fence = None;
                }
                Some(_) => {}
                None => fence = Some((fence_char, fence_len)),
            }
            continue;
        }
        if fence.is_none()
            && let Some((_, text)) = atx_heading(line)
        {
            headings.push(text);
        }
    }
    headings
}

/// Extract the ATX **sections** of a byte-faithful markdown body: each heading
/// paired with the body span beneath it, up to the next heading of the same or a
/// shallower level (a deeper subsection stays part of its parent's span). The
/// feature a `section_contains` clause reads (`specs/architecture/10-contracts.md`, the
/// `section_contains` structural primitive). A heading (and any `#` line) inside a
/// fenced code block opens no section — the same exclusion [`body_headings`] makes,
/// tracked the identical way — so a fenced marker never splits the prose. Heading
/// text is stripped exactly as [`body_headings`] strips it; the body is the
/// intervening lines rejoined with `\n`, the span a marker check searches.
///
/// `pub(crate)` so the data-driven [`crate::kind`] `sections` primitive composes
/// this exact splitter rather than a second one that could drift from the heading
/// logic (`specs/architecture/15-kinds.md`).
pub(crate) fn body_sections(body: &str) -> Vec<Section> {
    let lines: Vec<&str> = body.lines().collect();
    // First pass: the heading lines *outside* fenced code, each with its line
    // index, level, and stripped text — the section boundaries.
    let mut heads: Vec<(usize, usize, String)> = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    for (index, line) in lines.iter().enumerate() {
        if let Some((fence_char, fence_len)) = fence_marker(line) {
            match fence {
                Some((open_char, open_len)) if fence_char == open_char && fence_len >= open_len => {
                    fence = None;
                }
                Some(_) => {}
                None => fence = Some((fence_char, fence_len)),
            }
            continue;
        }
        if fence.is_none()
            && let Some((level, text)) = atx_heading(line)
        {
            heads.push((index, level, text));
        }
    }

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
/// `\n`). The feature a `fenced` primitive yields (`specs/architecture/15-kinds.md`,
/// "The engine is kind-blind — extraction is generic"). A block opens on a
/// fence marker and closes on the next marker of the **same char and at least the
/// opening length** — the identical fence tracking [`body_headings`] runs, so a
/// heading or a shorter/different marker *inside* a block is interior content, never
/// a nested open. An unterminated fence runs to the end of the body (CommonMark), so
/// its block is still yielded rather than silently dropped. Surrounding prose is
/// skipped — only the interior is captured — and a body with no fence yields none.
///
/// `pub(crate)` so the data-driven [`crate::kind`] `fenced` primitive composes this
/// exact reader rather than a second one that could drift from the heading/section
/// fence logic (`specs/architecture/15-kinds.md`).
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
/// surrounding whitespace leaves the declared kind the genre consumer keys on.
fn fence_info(line: &str, fence_char: char) -> String {
    line.trim_start_matches(' ')
        .trim_start_matches(fence_char)
        .trim()
        .to_string()
}

/// Parse a genre fence's **info string** into its `(genre, key)` identity, or `None`
/// when the block is not a genre fence (`specs/architecture/20-surface.md`, "the floor
/// spelling is a genre fence"): a genre fence's info string is `genre.<genre> <key>`
/// (`genre.decision surface-authority`) — the `genre.` prefix, the genre name, then the
/// fence key. Any other info string (a bare `` ``` ``, a `sh`, a `toml`) is a plain
/// fenced block, not a genre value — adoption is opt-in per block, so a non-match is
/// silently *not* a genre, never an error. Exactly two tokens: a stray third token is a
/// malformed info string, not a third address level, so it yields `None`.
pub(crate) fn parse_genre_info(info: &str) -> Option<(String, String)> {
    let rest = info.strip_prefix("genre.")?;
    let mut tokens = rest.split_whitespace();
    let genre = tokens.next()?;
    let key = tokens.next()?;
    if tokens.next().is_some() {
        return None;
    }
    Some((genre.to_string(), key.to_string()))
}

/// Parse a genre fence's **interior TOML** into a [`GenreValue`], or `None` when the
/// interior is not well-formed TOML (`specs/architecture/20-surface.md`, "Extraction composes
/// the algebra's fenced-block primitive with a TOML parse"). A top-level string value is
/// a **prose leaf**; a top-level table is a **sibling collection** whose sub-tables are
/// its keyed entries, each entry's string values its own leaves. Any other TOML type is
/// neither a prose leaf nor a keyed collection, so it is not surfaced — leaves are
/// authored strings (law 5), never inferred from a scalar or array. Malformed interior
/// TOML yields no genre value; the raw [`FencedBlock`] still carries the bytes, so
/// nothing is lost, and extraction stays total (no error channel at this boundary).
pub(crate) fn parse_genre_value(genre: &str, key: &str, interior: &str) -> Option<GenreValue> {
    let doc = interior.parse::<toml_edit::DocumentMut>().ok()?;
    let mut leaves = BTreeMap::new();
    let mut collections = BTreeMap::new();
    for (field, item) in doc.as_table().iter() {
        if let Some(text) = item.as_str() {
            leaves.insert(field.to_string(), text.to_string());
        } else if let Some(entries_table) = item.as_table_like() {
            let mut entries = BTreeMap::new();
            for (entry_key, entry_item) in entries_table.iter() {
                let Some(entry_table) = entry_item.as_table_like() else {
                    continue;
                };
                let mut entry_leaves = BTreeMap::new();
                for (leaf, leaf_item) in entry_table.iter() {
                    if let Some(text) = leaf_item.as_str() {
                        entry_leaves.insert(leaf.to_string(), text.to_string());
                    }
                }
                entries.insert(entry_key.to_string(), entry_leaves);
            }
            collections.insert(field.to_string(), entries);
        }
    }
    Some(GenreValue {
        genre: genre.to_string(),
        key: key.to_string(),
        leaves,
        collections,
    })
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
/// placement feature the identical way (`specs/architecture/15-kinds.md`).
pub(crate) fn source_dir_name(source_path: &Path) -> Option<String> {
    source_path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .map(str::to_string)
}

/// Extract the `at-import` directive occurrences (`@path/to/file`) from a
/// byte-faithful markdown body, in document order — the raw path strings, one per
/// occurrence (`specs/architecture/15-kinds.md`, "Directives — format-executed body
/// syntax"). An `@` opens an import only at a word boundary (start of line or after
/// whitespace), so an email `user@host` and a bare `@` in prose yield nothing; the
/// occurrence is the run of non-whitespace after the `@` (`@path`, absolute allowed;
/// code.claude.com/docs/en/memory, retrieved 2026-07-02). A `@path` inside a fenced
/// code block or an inline code span is illustration the harness does not execute
/// ("imports are not evaluated inside markdown code spans and code blocks", same
/// retrieval), so it is skipped — the fence exclusion [`body_headings`] makes,
/// extended to inline spans, is what keeps the extraction sound rather than a guess.
/// Resolution/classing is a later slice; this yields the raw occurrence strings only.
///
/// `pub(crate)` so the data-driven [`crate::kind`] `directives` primitive composes
/// this exact reader rather than a second one that could drift.
pub(crate) fn body_at_imports(body: &str) -> Vec<String> {
    let mut imports = Vec::new();
    // The open fence's char and run length, while inside a fenced code block — the
    // identical state `body_headings` tracks, for the identical reason.
    let mut fence: Option<(char, usize)> = None;
    for line in body.lines() {
        if let Some((fence_char, fence_len)) = fence_marker(line) {
            match fence {
                Some((open_char, open_len)) if fence_char == open_char && fence_len >= open_len => {
                    fence = None;
                }
                Some(_) => {}
                None => fence = Some((fence_char, fence_len)),
            }
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

/// Resolve a dotted **key-path** (`a.b.c`) against a parsed frontmatter map,
/// walking nested tables to the leaf value — the traversal the `field` extraction
/// primitive promises (`specs/architecture/15-kinds.md`, "structured field — a
/// frontmatter / JSON / TOML value at a key-path"). The first segment resolves in
/// the top-level map; each further segment descends into the value's object, so a
/// settings kind's nested `permissions.defaultMode` reads its leaf. A single-segment
/// path is an ordinary flat lookup, so the common case is unchanged.
///
/// Returns `None` — **absent, never errored** — when any segment fails to resolve:
/// a missing key, or a non-object value met before the leaf (a scalar or list has
/// no sub-key to walk into). Kind-blind: the returned leaf carries its own parsed
/// kind through [`json_to_feature`], so a nested read preserves the source scalar
/// kind exactly as a flat one does.
///
/// `pub(crate)` so the [`crate::kind`] `field` primitive walks the identical path
/// rather than a second traversal that could drift.
pub(crate) fn resolve_key_path<'a>(
    frontmatter: &'a BTreeMap<String, JsonValue>,
    key_path: &str,
) -> Option<&'a JsonValue> {
    let mut segments = key_path.split('.');
    // The first segment resolves in the top-level frontmatter map; a path with no
    // `.` is a single segment, so this is the flat lookup for the common case.
    let mut current = frontmatter.get(segments.next()?)?;
    for segment in segments {
        // Only an object has a sub-key to descend into — a scalar or list met before
        // the leaf leaves the path unresolved (absent), never a forged read.
        current = current.as_object()?.get(segment)?;
    }
    Some(current)
}

/// Project an `extra` frontmatter value into a [`FeatureValue`], preserving its
/// parsed source [`Kind`]: arrays become a list, objects a map, and each scalar
/// keeps the kind it parsed as (`string`/`integer`/`number`/`boolean`/`null`)
/// alongside its text. Stringifying every scalar to a bare string — the slice-1
/// shortcut — would make a `type` check undecidable; recording the kind here is
/// the precondition that check needs (`specs/architecture/10-contracts.md`, the `type`
/// lattice Decision).
///
/// `pub(crate)` so the [`crate::kind`] `field` extraction primitive projects a
/// declared frontmatter value into a [`FeatureValue`] through the same
/// kind-preserving path, never a second projector (`specs/architecture/15-kinds.md`).
pub(crate) fn json_to_feature(value: &JsonValue) -> FeatureValue {
    match value {
        JsonValue::Array(items) => {
            FeatureValue::List(items.iter().map(json_scalar_string).collect())
        }
        JsonValue::Object(_) => FeatureValue::Map,
        JsonValue::Null => FeatureValue::scalar(Kind::Null, "null"),
        JsonValue::Bool(b) => FeatureValue::scalar(Kind::Boolean, b.to_string()),
        JsonValue::Number(n) => FeatureValue::scalar(number_kind(n), n.to_string()),
        JsonValue::String(s) => FeatureValue::scalar(Kind::String, s.clone()),
    }
}

/// The source kind of a JSON number: `integer` when it parsed as a whole number
/// (`i64`/`u64`), else `number` (a floating-point value).
fn number_kind(n: &serde_json::Number) -> Kind {
    if n.is_i64() || n.is_u64() {
        Kind::Integer
    } else {
        Kind::Number
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
```toml genre.manifest\n\
name = \"x\"\n\
count = 2\n\
```\n\
\n\
prose below\n";
        let blocks = body_fenced_blocks(body);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].info, "sh");
        assert_eq!(blocks[0].content, "cargo test");
        assert_eq!(blocks[1].info, "toml genre.manifest");
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
        // gate input (`specs/architecture/15-kinds.md`, the soundness boundary).
        let body = "```toml\nk = 1\n```\n";
        assert_eq!(body_fenced_blocks(body), body_fenced_blocks(body));
    }

    #[test]
    fn each_lattice_name_round_trips_and_an_unknown_name_is_rejected() {
        // Every name in the closed lattice maps to its `Kind` and renders back to
        // the same spelling — the single name table a `type` clause goes through.
        for kind in [
            Kind::String,
            Kind::Integer,
            Kind::Number,
            Kind::Boolean,
            Kind::Null,
            Kind::List,
            Kind::Map,
        ] {
            assert_eq!(Kind::from_name(kind.name()), Some(kind));
        }
        // The spellings are exactly the lattice's, nothing more.
        assert_eq!(Kind::String.name(), "string");
        assert_eq!(Kind::from_name("map"), Some(Kind::Map));

        // A name outside the lattice yields `None` (the load-error signal a
        // `type` clause rejects on), never a silent default.
        assert_eq!(Kind::from_name("array"), None);
        assert_eq!(Kind::from_name("int"), None);
        assert_eq!(Kind::from_name(""), None);
    }
}
