//! The embedded built-in kind std-lib.
//!
//! specs/architecture/15-kinds.md, "A built-in kind is an adapter". `temper` ships the read-side
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

use crate::document::Document;
use crate::extract::{self, Features};
use crate::kind::{CustomKind, KindError, Unit};

// The generated `pub static BUILTIN_KINDS: &[(&str, &str)]` — every embedded kind as
// `(name, KIND.md source)`, sorted by name. `build.rs` writes it into `$OUT_DIR` from
// the `kinds/` tree; including it here re-exports it as
// `crate::builtin_kind::BUILTIN_KINDS`.
include!(concat!(env!("OUT_DIR"), "/builtin_kinds.rs"));

/// The embedded `KIND.md` source for the built-in kind a **bare** `name` resolves to,
/// or `None` if none carries it. A bare name resolves to its unique carrier whether the
/// embedded table keys it bare (`skill`, today's flat tree) or `<provider>.<name>`
/// qualified (`claude-code.skill`, post file-move) — the same bare→unique-or-collision
/// resolution the assembly binds through (`specs/architecture/15-kinds.md`, "Decision: kind identity
/// carries a provider axis").
///
/// # Errors
///
/// Returns [`KindError::AmbiguousKind`] when two providers carry the bare `name`, or a
/// [`KindError`] if any embedded `KIND.md` fails to parse.
pub fn source(name: &str) -> Result<Option<&'static str>, KindError> {
    Ok(resolve(name)?.map(|(src, _)| src))
}

/// Parse the built-in kind a **bare** `name` resolves to into its [`CustomKind`], or
/// `None` if none carries it. The `+++`-fenced header is parsed through the same
/// [`CustomKind::from_header`] a project-authored definition is; the bare→unique
/// resolution routes `skill` to its unique carrier across the flat or nested layout.
///
/// # Errors
///
/// Returns a [`KindError`] when the embedded `KIND.md` is not a well-formed fenced
/// document, its header is not an admissible kind definition (a bad `governs`, an
/// out-of-vocabulary extraction primitive, a stray key), or two providers collide under
/// the bare `name` ([`KindError::AmbiguousKind`]).
pub fn definition(name: &str) -> Result<Option<CustomKind>, KindError> {
    Ok(resolve(name)?.map(|(_, kind)| kind))
}

/// The **qualified identity** of the built-in kind a bare `name` resolves to —
/// `<provider>.<name>` (`specs/architecture/15-kinds.md`, "Decision: kind identity carries a
/// provider axis"), or `None` if no embedded kind carries the bare name. The one
/// resolution the qualified-binding consumers route through: a bare `skill` resolves
/// to the unique `claude-code.skill`, and a two-provider collision surfaces as a load
/// error rather than a silent wrong identity.
///
/// # Errors
///
/// Returns a [`KindError`] when an embedded `KIND.md` fails to parse, or two providers
/// collide under the bare `name` ([`KindError::AmbiguousKind`]).
pub fn qualified(name: &str) -> Result<Option<String>, KindError> {
    Ok(definition(name)?.map(|kind| kind.qualified_name()))
}

/// Parse every embedded built-in kind into a `name → CustomKind` map — the built-in
/// read-side set, the mirror of [`crate::builtin::contracts`] on the require-side. The
/// map is keyed by each kind's **bare** name (`skill`, `rule`), so the flat and nested
/// layouts key identically; a two-provider collision under one bare name is surfaced
/// through [`CustomKind::resolve_bare`] rather than silently overwriting a map entry.
///
/// # Errors
///
/// Returns a [`KindError`] if any embedded `KIND.md` fails to parse into an admissible
/// [`CustomKind`], or two providers collide under one bare name.
pub fn definitions() -> Result<BTreeMap<String, CustomKind>, KindError> {
    let kinds = parsed_kinds()?;
    let set: Vec<CustomKind> = kinds.iter().map(|(_, kind)| kind.clone()).collect();
    let mut map = BTreeMap::new();
    for (_, kind) in &kinds {
        // Route the keying through bare→unique resolution: two providers meeting under
        // one bare name is a load error, never a silent last-writer-wins overwrite.
        CustomKind::resolve_bare(&kind.name, &set)?;
        map.insert(kind.name.clone(), kind.clone());
    }
    Ok(map)
}

/// Parse every embedded built-in kind, pairing each with the embedded source it parsed
/// from. The bare `name` is taken from the (possibly qualified) table key, so
/// [`CustomKind::resolve_bare`] ranges over the same identity the assembly does.
fn parsed_kinds() -> Result<Vec<(&'static str, CustomKind)>, KindError> {
    BUILTIN_KINDS
        .iter()
        .map(|(key, src)| parse(key, src).map(|kind| (*src, kind)))
        .collect()
}

/// Resolve a **bare** kind name against the embedded set to its `(source, kind)` pair —
/// unique-or-collision per [`CustomKind::resolve_bare`]. `None` when no embedded kind
/// carries the name; the pair is re-selected by the resolved bare name, unique once
/// resolution succeeds.
fn resolve(name: &str) -> Result<Option<(&'static str, CustomKind)>, KindError> {
    let kinds = parsed_kinds()?;
    let set: Vec<CustomKind> = kinds.iter().map(|(_, kind)| kind.clone()).collect();
    match CustomKind::resolve_bare(name, &set)? {
        None => Ok(None),
        Some(_) => Ok(kinds.into_iter().find(|(_, kind)| kind.name == name)),
    }
}

/// Parse one embedded `KIND.md` `src` into a [`CustomKind`], its bare name taken from
/// the possibly-qualified table `key` (`claude-code.skill` → `skill`; `skill` → `skill`).
/// The synthetic `kinds/[<provider>/]<name>/KIND.md` path gives diagnostics the same
/// shape the on-disk loader's carry.
fn parse(key: &str, src: &str) -> Result<CustomKind, KindError> {
    let name = key.rsplit('.').next().unwrap_or(key);
    let path = PathBuf::from("kinds")
        .join(key.replace('.', "/"))
        .join("KIND.md");
    let document = Document::parse(src).map_err(|source| KindError::Document {
        path: path.clone(),
        source,
    })?;
    CustomKind::from_header(document.header().as_table(), name, &path)
}

/// Extract a built-in skill's [`Features`] by running the embedded `skill`
/// `KIND.md` extraction over a generically-loaded surface [`Unit`] — the same
/// composed path every kind reads (`specs/architecture/15-kinds.md`, "The extraction algebra"),
/// with **no IR→Unit adapter on the check read**: the caller loads the surface
/// member document through [`Unit::from_member_document`](crate::kind::Unit::from_member_document),
/// exactly as a custom kind's members load, so built-in and custom kinds read the
/// surface through one loader. The per-field feature mapping is the composed
/// `kinds/skill/KIND.md`.
///
/// # Errors
///
/// Returns a [`KindError`] if the embedded `skill` `KIND.md` is not an admissible
/// kind definition — a genuine invariant, as it is compiled-in product source
/// (`build.rs`).
pub fn skill_features(unit: &Unit) -> Result<Features, KindError> {
    features_from("skill", unit)
}

/// Extract a built-in rule's [`Features`] the same way [`skill_features`] does — the
/// embedded `rule` `KIND.md` extraction over the rule's generically-loaded surface
/// [`Unit`].
///
/// # Errors
///
/// Returns a [`KindError`] if the embedded `rule` `KIND.md` is not an admissible
/// kind definition (a compiled-in invariant).
pub fn rule_features(unit: &Unit) -> Result<Features, KindError> {
    features_from("rule", unit)
}

/// Run the named built-in kind's embedded extraction over `unit`, then fold every
/// preserved frontmatter key the composed primitives did not name into the feature
/// map — the built-in adapter's **permissive extraction** (`specs/architecture/15-kinds.md`,
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
        assert!(source("spec").unwrap().is_none());
    }

    #[test]
    fn definitions_carries_exactly_the_two_built_in_kinds() {
        let all = definitions().unwrap();
        assert_eq!(all.keys().collect::<Vec<_>>(), vec!["rule", "skill"]);
    }

    #[test]
    fn resolve_bare_over_a_qualified_set_finds_the_unique_carrier_and_errors_on_collision() {
        use toml_edit::DocumentMut;

        // A synthetic `<provider>`-qualified `skill` kind — the shape the embedded table
        // carries post-file-move (`kinds/<provider>/skill/KIND.md` with a `provider`
        // line), proving the resolution the built-in lookups route through finds the
        // qualified kind exactly as it finds today's bare one.
        fn skill_of(provider: &str) -> CustomKind {
            let src = format!(
                "governs = {{ root = \".claude/skills\", glob = \"*/SKILL.md\" }}\nprovider = \"{provider}\"\n"
            );
            let doc = src.parse::<DocumentMut>().unwrap();
            CustomKind::from_header(
                doc.as_table(),
                "skill",
                std::path::Path::new("kinds/claude-code/skill/KIND.md"),
            )
            .unwrap()
        }

        // One carrier: a bare `skill` resolves to its unique `claude-code.skill`.
        let one = vec![skill_of("claude-code")];
        assert_eq!(
            CustomKind::resolve_bare("skill", &one)
                .unwrap()
                .map(CustomKind::qualified_name)
                .as_deref(),
            Some("claude-code.skill")
        );

        // Two providers meeting under one bare name is a load error naming the
        // candidates — the collision the Decision requires, never a silent wrong kind.
        let two = vec![skill_of("claude-code"), skill_of("agent-skills")];
        let err = CustomKind::resolve_bare("skill", &two).unwrap_err();
        assert!(matches!(err, KindError::AmbiguousKind { .. }));
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

    /// Write a member's authored surface member document `<dir>/<member_doc>` exactly
    /// as `import`/`apply` project it ([`crate::frontmatter::Member::to_document`]),
    /// then reload it through the generic surface loader `check` reads — the built-in
    /// kind's member-document read (`specs/architecture/15-kinds.md`, "A built-in kind is an
    /// adapter"), one generic adapter, no per-kind IR.
    fn surface_unit(
        member: &crate::frontmatter::Member,
        member_doc: &str,
        dir: &std::path::Path,
    ) -> Unit {
        std::fs::create_dir_all(dir).unwrap();
        let doc_path = dir.join(member_doc);
        std::fs::write(&doc_path, member.to_document().emit()).unwrap();
        Unit::from_member_document(dir, &doc_path).unwrap()
    }

    #[test]
    fn skill_features_fold_unknown_keys_and_surface_satisfies_off_the_surface() {
        use crate::extract::{FeatureValue, Kind};

        let parent = tmpdir("skill-driver");
        let src = parent.join("demo");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("SKILL.md"),
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
        let skill = definition("skill").unwrap().unwrap();
        let mut member =
            crate::frontmatter::Member::from_source(&skill, &src.join("SKILL.md")).unwrap();
        // The authored representation edge — surfaced by the driver, kept out of `fields`.
        member.satisfies = vec![crate::document::Satisfies {
            requirement: "req.one".to_string(),
            rationale: Some("The human why, never a decidable feature.".to_string()),
        }];

        // Read the extracted features off the written surface, not a typed IR.
        let unit = surface_unit(&member, "SKILL.md", &parent.join("surface-demo"));
        let features = skill_features(&unit).unwrap();

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
        let rule = definition("rule").unwrap().unwrap();

        std::fs::write(
            rules.join("rust.md"),
            "---\npaths:\n  - \"src/**/*.rs\"\n---\n# Rust\n\nBody.\n",
        )
        .unwrap();
        let member =
            crate::frontmatter::Member::from_source(&rule, &rules.join("rust.md")).unwrap();
        let unit = surface_unit(&member, "RULE.md", &parent.join("surface-rust"));
        let features = rule_features(&unit).unwrap();
        assert_eq!(
            features.field("paths"),
            Some(&FeatureValue::List(vec!["src/**/*.rs".to_string()]))
        );
        // `placement` reads the imported source directory off provenance, carried
        // through the surface — `rules`, not the projected surface directory.
        assert_eq!(features.source_dir.as_deref(), Some("rules"));

        // A rule with no frontmatter carries no fields at all — the whole file is body.
        std::fs::write(rules.join("collab.md"), "# Collaboration\n\nPushback.\n").unwrap();
        let bare =
            crate::frontmatter::Member::from_source(&rule, &rules.join("collab.md")).unwrap();
        let bare_unit = surface_unit(&bare, "RULE.md", &parent.join("surface-collab"));
        let bare_features = rule_features(&bare_unit).unwrap();
        assert!(bare_features.fields.is_empty());
        assert_eq!(bare_features.body_lines, 3);
    }
}
