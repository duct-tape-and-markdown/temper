//! The embedded built-in kind std-lib.
//!
//! specs/15-kinds.md, "A built-in kind is an adapter". `temper` ships the read-side
//! definitions of the known-harness kinds (`skill`, `rule`) as first-party product
//! source in a `kinds/<name>/KIND.md` tree at the repo root — the same medium and
//! schema as a project's own `.temper/kinds/<name>/KIND.md`, differing only in where
//! it sources from (temper-maintained vs author-maintained; ownership, not
//! mechanism). `build.rs` walks that tree and generates the [`BUILTIN_KINDS`] table
//! this module re-exports, so a built-in kind resolves from the embedded set with no
//! on-disk configuration.
//!
//! A built-in kind's definition parses into a [`CustomKind`] through the very same
//! [`CustomKind::from_header`] path a project-authored `KIND.md` does, and is
//! validated as any kind is — this only sources the header from embedded product
//! data instead of `.temper/kinds/`.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::Value as JsonValue;

use crate::document::Document;
use crate::extract::{self, Features};
use crate::kind::{CustomKind, KindError, Unit};
use crate::rule::Rule;
use crate::skill::Skill;

// The generated `pub static BUILTIN_KINDS: &[(&str, &str)]` — every embedded kind as
// `(name, KIND.md source)`, sorted by name. `build.rs` writes it into `$OUT_DIR` from
// the `kinds/` tree; including it here re-exports it as
// `crate::builtin_kind::BUILTIN_KINDS`.
include!(concat!(env!("OUT_DIR"), "/builtin_kinds.rs"));

/// The embedded `KIND.md` source for a built-in kind, or `None` if no kind of that
/// name is embedded.
#[must_use]
pub fn source(name: &str) -> Option<&'static str> {
    BUILTIN_KINDS
        .iter()
        .find(|(embedded, _)| *embedded == name)
        .map(|(_, src)| *src)
}

/// Parse the named built-in kind's embedded `KIND.md` into its [`CustomKind`], or
/// `None` if no kind of that name is embedded. The `+++`-fenced header is parsed
/// through the same [`CustomKind::from_header`] a project-authored definition is; the
/// synthetic `kinds/<name>/KIND.md` path labels any diagnostic.
///
/// # Errors
///
/// Returns a [`KindError`] when the embedded `KIND.md` is not a well-formed fenced
/// document, or its header is not an admissible kind definition (a bad `governs`, an
/// out-of-vocabulary extraction primitive, a stray key).
pub fn definition(name: &str) -> Result<Option<CustomKind>, KindError> {
    match source(name) {
        None => Ok(None),
        Some(src) => Ok(Some(parse(name, src)?)),
    }
}

/// Parse every embedded built-in kind into a `name → CustomKind` map — the built-in
/// read-side set, the mirror of [`crate::builtin::contracts`] on the require-side.
///
/// # Errors
///
/// Returns a [`KindError`] if any embedded `KIND.md` fails to parse into an
/// admissible [`CustomKind`].
pub fn definitions() -> Result<BTreeMap<String, CustomKind>, KindError> {
    let mut map = BTreeMap::new();
    for (name, src) in BUILTIN_KINDS {
        map.insert((*name).to_string(), parse(name, src)?);
    }
    Ok(map)
}

/// Parse one embedded `KIND.md` source into a [`CustomKind`] named `name`. The
/// synthetic `kinds/<name>/KIND.md` path gives diagnostics the same shape the on-disk
/// loader's carry.
fn parse(name: &str, src: &str) -> Result<CustomKind, KindError> {
    let path = PathBuf::from("kinds").join(name).join("KIND.md");
    let document = Document::parse(src).map_err(|source| KindError::Document {
        path: path.clone(),
        source,
    })?;
    CustomKind::from_header(document.header().as_table(), name, &path)
}

/// Extract a built-in skill's [`Features`] by running the embedded `skill`
/// `KIND.md` extraction over an IR-derived [`Unit`] — the generic composed path
/// every kind reads (`specs/15-kinds.md`, "The extraction algebra"), in place of a
/// hand-coded per-field projector. The `Skill` IR → `Unit` conversion is the
/// sanctioned engine-code adapter face (the frontmatter format is Claude Code's);
/// the per-field feature mapping is the composed `kinds/skill/KIND.md`.
///
/// # Errors
///
/// Returns a [`KindError`] if the embedded `skill` `KIND.md` is not an admissible
/// kind definition — a genuine invariant, as it is compiled-in product source
/// (`build.rs`).
pub fn skill_features(skill: &Skill) -> Result<Features, KindError> {
    features_from("skill", &skill_unit(skill))
}

/// Extract a built-in rule's [`Features`] the same way [`skill_features`] does — the
/// embedded `rule` `KIND.md` extraction over the rule's IR-derived [`Unit`].
///
/// # Errors
///
/// Returns a [`KindError`] if the embedded `rule` `KIND.md` is not an admissible
/// kind definition (a compiled-in invariant).
pub fn rule_features(rule: &Rule) -> Result<Features, KindError> {
    features_from("rule", &rule_unit(rule))
}

/// Run the named built-in kind's embedded extraction over `unit`, then fold every
/// preserved frontmatter key the composed primitives did not name into the feature
/// map — the built-in adapter's **permissive extraction** (`specs/15-kinds.md`,
/// "Extending a built-in kind"): an unknown key on a known artifact is already
/// extracted, so a clause (a `forbidden_keys`) can range over it. The closed algebra
/// cannot enumerate unknown keys, so this bulk preservation is the adapter's, while
/// each documented field is the composed `KIND.md`'s. `or_insert` leaves each field
/// the composed extractor already yielded untouched.
fn features_from(name: &str, unit: &Unit) -> Result<Features, KindError> {
    let kind = definition(name)?.expect("a built-in kind is embedded for every built-in driver");
    let mut features = kind.extraction.extract(unit);
    for (key, value) in &unit.frontmatter {
        features
            .fields
            .entry(key.clone())
            .or_insert_with(|| extract::json_to_feature(value));
    }
    Ok(features)
}

/// Adapt a parsed [`Skill`] IR into the generic [`Unit`] the composed extractor
/// reads (`specs/15-kinds.md`, "A built-in kind is an adapter"): the typed
/// frontmatter fields and every preserved `extra` key as parsed frontmatter, the
/// byte-faithful body, the provenance source path (the `placement` locus), and the
/// representation edges (`satisfies`, published requirements) carried through
/// unchanged. The typed fields are stringified at load, so each is a JSON string —
/// exactly what the composed `field` primitive projects kind-preserving.
fn skill_unit(skill: &Skill) -> Unit {
    let mut frontmatter = BTreeMap::new();
    frontmatter.insert("name".to_string(), JsonValue::String(skill.name.clone()));
    frontmatter.insert(
        "description".to_string(),
        JsonValue::String(skill.description.clone()),
    );
    if let Some(version) = &skill.version {
        frontmatter.insert("version".to_string(), JsonValue::String(version.clone()));
    }
    if let Some(license) = &skill.license {
        frontmatter.insert("license".to_string(), JsonValue::String(license.clone()));
    }
    for (key, value) in &skill.extra {
        frontmatter.insert(key.clone(), value.clone());
    }
    Unit {
        id: skill.name.clone(),
        frontmatter,
        body: skill.body.clone(),
        source_path: skill.provenance.source_path.clone(),
        satisfies: skill
            .satisfies
            .iter()
            .map(|s| s.requirement.clone())
            .collect(),
        satisfies_clauses: skill.satisfies.clone(),
        published_requirements: skill.published_requirements.clone(),
    }
}

/// Adapt a parsed [`Rule`] IR into its generic [`Unit`], mirroring [`skill_unit`]:
/// the optional `paths` sequence as a JSON array of strings (the `field` primitive
/// projects it back to a list feature) plus every preserved `extra` key, the
/// byte-faithful body, provenance placement, and the representation edges.
fn rule_unit(rule: &Rule) -> Unit {
    let mut frontmatter = BTreeMap::new();
    if let Some(paths) = &rule.paths {
        frontmatter.insert(
            "paths".to_string(),
            JsonValue::Array(paths.iter().map(|p| JsonValue::String(p.clone())).collect()),
        );
    }
    for (key, value) in &rule.extra {
        frontmatter.insert(key.clone(), value.clone());
    }
    Unit {
        id: rule.name.clone(),
        frontmatter,
        body: rule.body.clone(),
        source_path: rule.provenance.source_path.clone(),
        satisfies: rule
            .satisfies
            .iter()
            .map(|s| s.requirement.clone())
            .collect(),
        satisfies_clauses: rule.satisfies.clone(),
        published_requirements: rule.published_requirements.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::Edge;
    use crate::kind::{Governs, Primitive};

    #[test]
    fn skill_definition_matches_the_hand_authored_kind() {
        let skill = definition("skill").unwrap().expect("skill is embedded");

        assert_eq!(skill.name, "skill");
        assert_eq!(
            skill.governs,
            Governs {
                root: ".claude/skills".to_string(),
                glob: "*/SKILL.md".to_string(),
            }
        );
        // The composed extractor mirrors `kinds/skill/KIND.md`: the four documented
        // frontmatter fields, then the markdown-structure primitives, in order.
        assert_eq!(
            skill.extraction.primitives(),
            &[
                Primitive::Field {
                    key: "name".to_string()
                },
                Primitive::Field {
                    key: "description".to_string()
                },
                Primitive::Field {
                    key: "version".to_string()
                },
                Primitive::Field {
                    key: "license".to_string()
                },
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]
        );
        // The built-in `skill` kind declares no relationships.
        assert_eq!(skill.relationships, Vec::<Edge>::new());
    }

    #[test]
    fn rule_definition_matches_the_hand_authored_kind() {
        let rule = definition("rule").unwrap().expect("rule is embedded");

        assert_eq!(rule.name, "rule");
        assert_eq!(
            rule.governs,
            Governs {
                root: ".claude/rules".to_string(),
                glob: "*.md".to_string(),
            }
        );
        assert_eq!(
            rule.extraction.primitives(),
            &[
                Primitive::Field {
                    key: "paths".to_string()
                },
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]
        );
        assert_eq!(rule.relationships, Vec::<Edge>::new());
    }

    #[test]
    fn an_unknown_kind_name_is_none() {
        assert!(definition("spec").unwrap().is_none());
        assert!(source("spec").is_none());
    }

    #[test]
    fn definitions_carries_exactly_the_two_built_in_kinds() {
        let all = definitions().unwrap();
        assert_eq!(all.keys().collect::<Vec<_>>(), vec!["rule", "skill"]);
    }

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "builtin-kind-driver-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn skill_features_fold_unknown_keys_and_surface_satisfies_off_the_ir() {
        use crate::extract::{FeatureValue, Kind};

        let parent = tmpdir("skill-driver");
        let dir = parent.join("demo");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("SKILL.md"),
            "---\n\
name: demo\n\
description: Use when exercising the composed built-in driver.\n\
allowed-tools: [\"Bash\", \"Read\"]\n\
priority: 7\n\
---\n\
# Demo\n\
\n\
Body line two.\n",
        )
        .unwrap();
        let mut skill = Skill::from_source_dir(&dir).unwrap();
        // The authored representation edge — surfaced by the driver, kept out of `fields`.
        skill.satisfies = vec![crate::document::Satisfies {
            requirement: "req.one".to_string(),
            rationale: Some("The human why, never a decidable feature.".to_string()),
        }];

        let features = skill_features(&skill).unwrap();

        // The documented fields come off the composed `field` primitives.
        assert_eq!(
            features.field("name"),
            Some(&FeatureValue::scalar(Kind::String, "demo"))
        );
        // Permissive extraction: the unknown keys ride into the same feature map, so a
        // `forbidden_keys` clause can range over a project convention on a known artifact.
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

        // `satisfies` is surfaced as requirement names, never as a frontmatter field.
        assert_eq!(features.satisfies, vec!["req.one"]);
        assert!(!features.has_field("satisfies"));
        assert!(!features.has_field("rationale"));
    }

    #[test]
    fn rule_features_expose_paths_and_a_no_frontmatter_rule() {
        use crate::extract::FeatureValue;

        let parent = tmpdir("rule-driver");
        let rules = parent.join("rules");
        std::fs::create_dir_all(&rules).unwrap();

        std::fs::write(
            rules.join("rust.md"),
            "---\npaths:\n  - \"src/**/*.rs\"\n---\n# Rust\n\nBody.\n",
        )
        .unwrap();
        let rule = Rule::from_source_file(&rules.join("rust.md")).unwrap();
        let features = rule_features(&rule).unwrap();
        assert_eq!(
            features.field("paths"),
            Some(&FeatureValue::List(vec!["src/**/*.rs".to_string()]))
        );
        assert_eq!(features.source_dir.as_deref(), Some("rules"));

        // A rule with no frontmatter carries no fields at all — the whole file is body.
        std::fs::write(rules.join("collab.md"), "# Collaboration\n\nPushback.\n").unwrap();
        let bare = Rule::from_source_file(&rules.join("collab.md")).unwrap();
        let bare_features = rule_features(&bare).unwrap();
        assert!(bare_features.fields.is_empty());
        assert_eq!(bare_features.body_lines, 3);
    }
}
