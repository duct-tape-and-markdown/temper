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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    /// The heading text, with its `#` markers stripped exactly as
    /// [`body_headings`] strips them.
    pub heading: String,
    /// The body span beneath the heading — the intervening lines rejoined with
    /// `\n`, the text a `section_contains` marker check searches.
    pub body: String,
}

/// An artifact's deterministically-extracted features, keyed for generic clause
/// lookup. Everything here is surface-decidable; nothing is inferred meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
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
