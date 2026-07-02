//! Extraction — an artifact's surface-decidable feature set.
//!
//! Models the "Extraction is the soundness boundary" section of
//! `specs/30-landscapes.md` (generalized by `specs/20-surface.md`, "The IR"): a
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

use crate::rule::Rule;
use crate::skill::Skill;

/// A field's parsed source kind — the closed scalar/container lattice the `type`
/// primitive ranges over (`specs/10-contracts.md`, "Decision: the `type`
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
    /// name table (`specs/10-contracts.md`, "Decision: the `type` vocabulary is
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
/// feature a `section_contains` clause decides over (`specs/10-contracts.md`, the
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
    /// (`specs/10-contracts.md`, the `section_contains` structural primitive). A
    /// superset of [`headings`](Features::headings): where `headings` carries only
    /// each heading's text, a section pairs it with its body span so a marker check
    /// has prose to search.
    pub sections: Vec<Section>,
    /// The name of the directory the unit was imported from, off provenance
    /// (for `name-matches-dir`). `None` when the source path has no parent.
    pub source_dir: Option<String>,
    /// The requirements this artifact opts into filling — the authored
    /// `[representation].satisfies` bindings, surfaced for the coverage check
    /// (`specs/20-surface.md`, "Each artifact directory is a representation, not
    /// a copy"). This is a *representation* edge the coverage resolver reads, NOT
    /// a contract-checkable frontmatter field — so it lives here, distinct from
    /// `fields`, and never resolves through [`Features::field`]. The authored
    /// `rationale` is deliberately absent: it is the human *why*, never a
    /// decidable feature.
    pub satisfies: Vec<String>,
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

/// Project a [`Skill`] into its [`Features`]. Lives here, not on `Skill`, so the
/// artifact IR (`crate::skill`) stays untouched: extraction is a separate,
/// engine-facing view of the same parsed value.
#[must_use]
pub fn skill_features(skill: &Skill) -> Features {
    let mut fields = BTreeMap::new();
    // The typed fields are always parsed as strings (the IR stringifies them at
    // load), so their source kind is `string`.
    fields.insert(
        "name".to_string(),
        FeatureValue::scalar(Kind::String, skill.name.clone()),
    );
    fields.insert(
        "description".to_string(),
        FeatureValue::scalar(Kind::String, skill.description.clone()),
    );
    if let Some(version) = &skill.version {
        fields.insert(
            "version".to_string(),
            FeatureValue::scalar(Kind::String, version.clone()),
        );
    }
    if let Some(license) = &skill.license {
        fields.insert(
            "license".to_string(),
            FeatureValue::scalar(Kind::String, license.clone()),
        );
    }
    // Unknown frontmatter keys join the same name-keyed map, so `forbidden_keys`
    // and value predicates see them exactly as they see the typed fields.
    for (key, value) in &skill.extra {
        fields.insert(key.clone(), json_to_feature(value));
    }

    Features {
        id: skill.name.clone(),
        fields,
        body_lines: body_line_count(&skill.body),
        headings: body_headings(&skill.body),
        sections: body_sections(&skill.body),
        source_dir: source_dir_name(&skill.provenance.source_path),
        // The authored representation binding the coverage check resolves — the
        // requirement names off the `[satisfies.*]` clause modules, distinct from
        // the frontmatter `fields` above (the per-clause `rationale` is the human
        // *why*, never a decidable feature, so it is dropped here).
        satisfies: skill
            .satisfies
            .iter()
            .map(|s| s.requirement.clone())
            .collect(),
    }
}

/// Project a [`Rule`] into its [`Features`]. Mirrors [`skill_features`]: `paths`
/// is exposed as a list, every `extra` frontmatter key is folded into the same
/// name-keyed map (so a `forbidden_keys` clause resolves `description`/`globs`/
/// `alwaysApply` exactly as it does for a skill), and `body_lines` counts the
/// byte-faithful body. `source_dir` is the folder it was discovered under
/// (uniform with skills, even though the rule contract names neither).
#[must_use]
pub fn rule_features(rule: &Rule) -> Features {
    let mut fields = BTreeMap::new();
    if let Some(paths) = &rule.paths {
        fields.insert("paths".to_string(), FeatureValue::List(paths.clone()));
    }
    // Unknown frontmatter keys join the same name-keyed map, so `forbidden_keys`
    // and value predicates see them exactly as they see the typed `paths`.
    for (key, value) in &rule.extra {
        fields.insert(key.clone(), json_to_feature(value));
    }

    Features {
        id: rule.name.clone(),
        fields,
        body_lines: body_line_count(&rule.body),
        headings: body_headings(&rule.body),
        sections: body_sections(&rule.body),
        source_dir: source_dir_name(&rule.provenance.source_path),
        // The authored representation binding the coverage check resolves — the
        // requirement names off the `[satisfies.*]` clause modules, distinct from
        // the frontmatter `fields` above.
        satisfies: rule
            .satisfies
            .iter()
            .map(|s| s.requirement.clone())
            .collect(),
    }
}

/// The line count of a byte-faithful markdown body — the `max_lines` feature.
/// A single home for the count so the per-kind projectors and the data-driven
/// [`crate::kind`] composer read it the identical way rather than each writing
/// `body.lines().count()` inline.
///
/// `pub(crate)` so the closed extraction algebra (`specs/15-kinds.md`, "The
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
/// ATX/fence logic rather than reimplementing it (`specs/15-kinds.md`).
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
/// feature a `section_contains` clause reads (`specs/10-contracts.md`, the
/// `section_contains` structural primitive). A heading (and any `#` line) inside a
/// fenced code block opens no section — the same exclusion [`body_headings`] makes,
/// tracked the identical way — so a fenced marker never splits the prose. Heading
/// text is stripped exactly as [`body_headings`] strips it; the body is the
/// intervening lines rejoined with `\n`, the span a marker check searches.
///
/// `pub(crate)` so the data-driven [`crate::kind`] `sections` primitive composes
/// this exact splitter rather than a second one that could drift from the heading
/// logic (`specs/15-kinds.md`).
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
/// placement feature the identical way (`specs/15-kinds.md`).
pub(crate) fn source_dir_name(source_path: &Path) -> Option<String> {
    source_path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .map(str::to_string)
}

/// Project an `extra` frontmatter value into a [`FeatureValue`], preserving its
/// parsed source [`Kind`]: arrays become a list, objects a map, and each scalar
/// keeps the kind it parsed as (`string`/`integer`/`number`/`boolean`/`null`)
/// alongside its text. Stringifying every scalar to a bare string — the slice-1
/// shortcut — would make a `type` check undecidable; recording the kind here is
/// the precondition that check needs (`specs/10-contracts.md`, the `type`
/// lattice Decision).
///
/// `pub(crate)` so the [`crate::kind`] `field` extraction primitive projects a
/// declared frontmatter value into a [`FeatureValue`] through the same
/// kind-preserving path, never a second projector (`specs/15-kinds.md`).
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
    use crate::skill::Skill;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-extract-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    const FIXTURE: &str = "---\n\
name: demo\n\
description: Use when demonstrating feature extraction.\n\
version: \"1.2.0\"\n\
allowed-tools: [\"Bash\", \"Read\"]\n\
priority: 7\n\
---\n\
# Demo\n\
\n\
Body line two.\n\
Body line three.";

    /// Parse a skill from a directory named `dir_name` so `source_dir` is
    /// predictable (it reads the directory off provenance).
    fn skill_in(parent: &std::path::Path, dir_name: &str, skill_md: &str) -> Skill {
        let dir = parent.join(dir_name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), skill_md).unwrap();
        Skill::from_source_dir(&dir).unwrap()
    }

    #[test]
    fn exposes_typed_and_unknown_fields_lines_and_source_dir() {
        let parent = tmpdir("expose");
        let skill = skill_in(&parent, "demo", FIXTURE);

        let features = skill_features(&skill);

        // The artifact id used in diagnostics is the skill name.
        assert_eq!(features.id, "demo");

        // Typed fields are name-keyed alongside the unknown frontmatter keys.
        assert_eq!(
            features.field("name").and_then(FeatureValue::as_scalar),
            Some("demo")
        );
        assert_eq!(
            features
                .field("description")
                .and_then(FeatureValue::as_scalar),
            Some("Use when demonstrating feature extraction.")
        );
        assert_eq!(
            features.field("version").and_then(FeatureValue::as_scalar),
            Some("1.2.0")
        );
        assert!(features.field("license").is_none());

        // Unknown keys ride the same map: a list stays a list, a scalar a scalar.
        assert_eq!(
            features.field("allowed-tools"),
            Some(&FeatureValue::List(vec![
                "Bash".to_string(),
                "Read".to_string()
            ]))
        );
        assert_eq!(
            features.field("priority").and_then(FeatureValue::as_scalar),
            Some("7")
        );

        // Body line count and the imported directory name, off provenance.
        assert_eq!(features.body_lines, 4);
        assert_eq!(features.source_dir.as_deref(), Some("demo"));

        // The body's ATX heading is exposed deterministically.
        assert_eq!(features.headings, vec!["Demo".to_string()]);
    }

    #[test]
    fn each_field_preserves_its_parsed_source_kind_while_as_scalar_still_reads_text() {
        let parent = tmpdir("kinds");
        // A field of each parsed kind: a string, an integer, a float, a boolean,
        // and a list — the precondition a sound `type` check needs.
        let skill = skill_in(
            &parent,
            "kinds",
            "---\n\
name: kinds\n\
description: Use when checking that source kinds survive projection.\n\
count: 7\n\
ratio: 1.5\n\
enabled: true\n\
tags: [\"a\", \"b\"]\n\
---\nbody\n",
        );
        let features = skill_features(&skill);

        // The typed `name` field is parsed as a string.
        assert_eq!(
            features.field("name").map(FeatureValue::kind),
            Some(Kind::String)
        );

        // An integer keeps `integer`, a float `number`, a boolean `boolean` —
        // none is flattened to `string` (the slice-1 shortcut this entry kills).
        assert_eq!(
            features.field("count").map(FeatureValue::kind),
            Some(Kind::Integer)
        );
        assert_eq!(
            features.field("ratio").map(FeatureValue::kind),
            Some(Kind::Number)
        );
        assert_eq!(
            features.field("enabled").map(FeatureValue::kind),
            Some(Kind::Boolean)
        );
        // A list keeps `list`.
        assert_eq!(
            features.field("tags").map(FeatureValue::kind),
            Some(Kind::List)
        );

        // Yet `as_scalar` still yields each scalar's comparison text unchanged,
        // so the existing scalar predicates (`min_len`/`enum`/…) read on as before.
        assert_eq!(
            features.field("name").and_then(FeatureValue::as_scalar),
            Some("kinds")
        );
        assert_eq!(
            features.field("count").and_then(FeatureValue::as_scalar),
            Some("7")
        );
        assert_eq!(
            features.field("ratio").and_then(FeatureValue::as_scalar),
            Some("1.5")
        );
        assert_eq!(
            features.field("enabled").and_then(FeatureValue::as_scalar),
            Some("true")
        );
        // A list is a container, not a scalar — `as_scalar` stays `None`.
        assert_eq!(
            features.field("tags").and_then(FeatureValue::as_scalar),
            None
        );
    }

    #[test]
    fn unknown_keys_let_a_forbidden_keys_clause_resolve_presence() {
        let parent = tmpdir("forbidden");
        let skill = skill_in(
            &parent,
            "legacy",
            "---\n\
name: legacy\n\
description: Use when porting a Cursor rule.\n\
globs: \"**/*.rs\"\n\
alwaysApply: true\n\
---\nbody\n",
        );

        let features = skill_features(&skill);

        // The keys Claude Code ignores are present by name, so a generic
        // `forbidden_keys` clause can resolve them without any skill opinion.
        assert!(features.has_field("globs"));
        assert!(features.has_field("alwaysApply"));
        assert!(!features.has_field("nonexistent"));
    }

    #[test]
    fn field_lookup_is_generic_by_name() {
        let parent = tmpdir("generic");
        let skill = skill_in(&parent, "demo", FIXTURE);
        let features = skill_features(&skill);

        // A clause carries only a field *name*; lookup resolves it the same way
        // for any field, which is what keeps the engine free of `skill.name`.
        for name in ["name", "description", "version"] {
            assert!(features.field(name).is_some(), "field `{name}` resolves");
        }
    }

    /// Parse a rule from a file `<parent>/rules/<stem>.md`, so the rule name is
    /// the stem and `source_dir` is the discovered `rules` folder.
    fn rule_in(parent: &std::path::Path, stem: &str, rule_md: &str) -> Rule {
        let rules_dir = parent.join("rules");
        fs::create_dir_all(&rules_dir).unwrap();
        let path = rules_dir.join(format!("{stem}.md"));
        fs::write(&path, rule_md).unwrap();
        Rule::from_source_file(&path).unwrap()
    }

    #[test]
    fn rule_features_expose_paths_unknown_keys_and_body_lines() {
        let parent = tmpdir("rule-paths");
        let rule = rule_in(
            &parent,
            "rust",
            "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
  - \"tests/**/*.rs\"\n\
globs: \"**/*.rs\"\n\
---\n\
# Rust\n\
\n\
Body line two.\n\
Body line three.",
        );

        let features = rule_features(&rule);

        // The artifact id is the rule name (file stem).
        assert_eq!(features.id, "rust");

        // `paths` is exposed as a list.
        assert_eq!(
            features.field("paths"),
            Some(&FeatureValue::List(vec![
                "src/**/*.rs".to_string(),
                "tests/**/*.rs".to_string(),
            ]))
        );

        // An `extra` key is folded into `fields`, so a `forbidden_keys` clause can
        // resolve the Cursor keys Claude Code ignores.
        assert!(features.has_field("globs"));
        assert!(!features.has_field("alwaysApply"));

        // Body line count, and the folder the rule was discovered under.
        assert_eq!(features.body_lines, 4);
        assert_eq!(features.source_dir.as_deref(), Some("rules"));
        // The rule body's heading is exposed the same way a skill's is.
        assert_eq!(features.headings, vec!["Rust".to_string()]);
    }

    #[test]
    fn headings_skip_fenced_code_and_yield_empty_for_a_bodyless_artifact() {
        let parent = tmpdir("headings");

        // Multiple ATX levels are captured in order; a `#` inside a fenced block
        // (and a `#tag` with no separating space) is not a heading.
        let skill = skill_in(
            &parent,
            "fenced",
            "---\n\
name: fenced\n\
description: Use when checking heading extraction.\n\
---\n\
# Title\n\
\n\
## Usage\n\
\n\
```sh\n\
# not a heading, just a shell comment\n\
```\n\
\n\
#tag is not a heading either\n\
\n\
### Examples ###\n",
        );
        let features = skill_features(&skill);
        assert_eq!(
            features.headings,
            vec![
                "Title".to_string(),
                "Usage".to_string(),
                "Examples".to_string(),
            ]
        );

        // A body with no headings yields an empty list, not a phantom entry.
        let plain = skill_in(
            &parent,
            "plain",
            "---\n\
name: plain\n\
description: Use when there is nothing but prose.\n\
---\n\
Just a paragraph, no headings at all.\n",
        );
        assert!(skill_features(&plain).headings.is_empty());
    }

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

    #[test]
    fn skill_features_expose_satisfies_off_the_ir_and_keep_it_out_of_fields() {
        let parent = tmpdir("satisfies");
        let mut skill = skill_in(&parent, "demo", FIXTURE);
        // The authored binding — set on the IR the way the surface reload would (via
        // the `[satisfies.*]` clause modules, each with its optional `rationale`).
        skill.satisfies = vec![
            crate::document::Satisfies {
                requirement: "req.one".to_string(),
                rationale: Some("Why this skill exists.".to_string()),
            },
            crate::document::Satisfies::new("req.two"),
        ];

        let features = skill_features(&skill);

        // `satisfies` is surfaced as requirement names for the coverage check.
        assert_eq!(features.satisfies, vec!["req.one", "req.two"]);
        // It is NOT a frontmatter field — it never resolves through `field`, so a
        // contract clause can't range over it.
        assert!(features.field("satisfies").is_none());
        assert!(!features.has_field("satisfies"));
        // `rationale` is the human *why*, never extracted as a decidable feature.
        assert!(features.field("rationale").is_none());
        assert!(!features.has_field("rationale"));
    }

    #[test]
    fn rule_features_expose_satisfies_off_the_ir_and_keep_it_out_of_fields() {
        let parent = tmpdir("rule-satisfies");
        let mut rule = rule_in(
            &parent,
            "rust",
            "---\npaths:\n  - \"src/**/*.rs\"\n---\n# Rust\n\nBody.\n",
        );
        rule.satisfies = vec![crate::document::Satisfies {
            requirement: "req.rust-style".to_string(),
            rationale: Some("The conventions the gate enforces.".to_string()),
        }];

        let features = rule_features(&rule);

        assert_eq!(features.satisfies, vec!["req.rust-style"]);
        assert!(features.field("satisfies").is_none());
        assert!(!features.has_field("satisfies"));
        assert!(!features.has_field("rationale"));
    }

    #[test]
    fn features_default_to_no_satisfies_when_unauthored() {
        let parent = tmpdir("no-satisfies");
        let skill = skill_in(&parent, "demo", FIXTURE);
        assert!(skill_features(&skill).satisfies.is_empty());
    }

    #[test]
    fn rule_features_handle_a_no_frontmatter_rule() {
        let parent = tmpdir("rule-nofm");
        let rule = rule_in(&parent, "collaboration", "# Collaboration\n\nPushback.\n");

        let features = rule_features(&rule);

        assert_eq!(features.id, "collaboration");
        // No frontmatter: no `paths`, no fields at all.
        assert!(features.field("paths").is_none());
        assert!(features.fields.is_empty());
        assert_eq!(features.body_lines, 3);
    }
}
