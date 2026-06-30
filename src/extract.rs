//! Extraction — an artifact's surface-decidable feature set.
//!
//! Models the "Extraction is the soundness boundary" section of
//! `specs/30-landscapes.md` (generalized by `specs/20-surface.md`, "The IR"): a
//! per-kind extractor projects a parsed artifact into a [`Features`] map the
//! generic contract engine reads. A contract clause is sound only because the
//! feature it names is **deterministically extractable** — so [`Features`]
//! admits *only* surface-decidable facts (a field's value, a key's presence, a
//! body's line count, the directory a unit sits under) and never inferred
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

use serde_json::Value as JsonValue;

use crate::skill::Skill;

/// One extracted feature value: a scalar field, or a list field (e.g. a YAML
/// sequence like `allowed-tools`). The two shapes the decidable predicates need
/// — scalar predicates (`min_len`, `enum`, `deny`, `allowed_chars`) read the
/// scalar; presence predicates (`required`, `forbidden_keys`) need only the key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureValue {
    /// A single scalar value, stringified (the YAML/JSON scalar as text).
    Scalar(String),
    /// A sequence of scalar values, stringified element-wise.
    List(Vec<String>),
}

impl FeatureValue {
    /// The scalar text of this value, or `None` if it is a list. Lets a
    /// scalar-oriented clause (`min_len`, `enum`, …) read the value generically.
    #[must_use]
    pub fn as_scalar(&self) -> Option<&str> {
        match self {
            FeatureValue::Scalar(s) => Some(s),
            FeatureValue::List(_) => None,
        }
    }
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
    /// The name of the directory the unit was imported from, off provenance
    /// (for `name-matches-dir`). `None` when the source path has no parent.
    pub source_dir: Option<String>,
    /// Companion paths relative to the artifact, forward-slash normalized so the
    /// comparison is platform-stable.
    pub companions: Vec<String>,
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
    fields.insert("name".to_string(), FeatureValue::Scalar(skill.name.clone()));
    fields.insert(
        "description".to_string(),
        FeatureValue::Scalar(skill.description.clone()),
    );
    if let Some(version) = &skill.version {
        fields.insert("version".to_string(), FeatureValue::Scalar(version.clone()));
    }
    if let Some(license) = &skill.license {
        fields.insert("license".to_string(), FeatureValue::Scalar(license.clone()));
    }
    // Unknown frontmatter keys join the same name-keyed map, so `forbidden_keys`
    // and value predicates see them exactly as they see the typed fields.
    for (key, value) in &skill.extra {
        fields.insert(key.clone(), json_to_feature(value));
    }

    Features {
        id: skill.name.clone(),
        fields,
        body_lines: skill.body.lines().count(),
        source_dir: source_dir_name(skill),
        companions: skill
            .companions
            .iter()
            .map(|path| path.to_string_lossy().replace('\\', "/"))
            .collect(),
    }
}

/// The name of the directory the skill was imported from (the folder Claude Code
/// discovers it under), off `provenance.source_path`.
fn source_dir_name(skill: &Skill) -> Option<String> {
    skill
        .provenance
        .source_path
        .parent()
        .and_then(std::path::Path::file_name)
        .and_then(|name| name.to_str())
        .map(str::to_string)
}

/// Convert an `extra` frontmatter value into a [`FeatureValue`]: arrays become a
/// list of stringified scalars, everything else a single stringified scalar.
fn json_to_feature(value: &JsonValue) -> FeatureValue {
    match value {
        JsonValue::Array(items) => {
            FeatureValue::List(items.iter().map(json_scalar_string).collect())
        }
        other => FeatureValue::Scalar(json_scalar_string(other)),
    }
}

/// Stringify a JSON scalar to its plain text form (no surrounding quotes for
/// strings); non-scalars fall back to their JSON text so the feature stays a
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
        // Companions are forward-slash normalized (none here, but the shape holds).
        assert!(features.companions.is_empty());
    }
}
