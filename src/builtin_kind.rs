//! The embedded built-in kind std-lib.
//!
//! `temper` ships the read-side definitions of the known-harness kinds (`agent`,
//! `command`, `skill`, `rule`, `memory`) as plain Rust data below — the compiled
//! default program the engine carries for SDK-less checking.
//!
//! A built-in kind's definition is a [`CustomKind`] like any other — assembled with
//! [`CustomKind::new`] — and validated as any kind is; this module only sources the
//! five facts from Rust literals instead of a parsed header. Identity is flat: a
//! kind's bare name is its whole identity, so the five kinds below never
//! collide.

use std::collections::BTreeMap;

use crate::extract::{self, Features};
use crate::kind::{
    CustomKind, Extraction, Format, Governs, KindError, Primitive, Registration, Unit,
};

/// The skill surface's field schema — the documented frontmatter fields plus the
/// markdown-structure primitives, shared verbatim by `skill` and `command`
/// (`specs/builtins.md`, "The shipped kinds": command is a second placement of the
/// skill surface, not a second schema).
fn skill_extraction() -> Extraction {
    Extraction::new(vec![
        Primitive::Field {
            key: "name".to_string(),
        },
        Primitive::Field {
            key: "description".to_string(),
        },
        Primitive::Field {
            key: "license".to_string(),
        },
        Primitive::Field {
            key: "disable-model-invocation".to_string(),
        },
        Primitive::Field {
            key: "user-invocable".to_string(),
        },
        Primitive::LineCount,
        Primitive::Headings,
        Primitive::Sections,
        Primitive::Placement,
    ])
}

/// Both `skill` and `command` register on both documented invocation channels —
/// user-invoked (`/name`) and description-trigger.
fn skill_surface_registration() -> Vec<Registration> {
    vec![
        Registration::UserInvoked,
        Registration::DescriptionTrigger {
            field: "description".to_string(),
        },
    ]
}

/// Anthropic's documented `.claude/skills/<name>/SKILL.md` kind: a directory whose
/// identity is the `name` field, registered on both documented invocation channels —
/// user-invoked (`/name`) and description-trigger — modulated per member by the
/// `disable-model-invocation`/`user-invocable` fields
/// (code.claude.com/docs/en/skills, "Control who invokes a skill", retrieved
/// 2026-07-07).
fn claude_code_skill() -> CustomKind {
    CustomKind {
        format: Some(Format::YamlFrontmatter),
        unit_shape: Some(crate::kind::UnitShape::Directory),
        registration: skill_surface_registration(),
        ..CustomKind::new(
            "skill",
            Governs {
                root: ".claude/skills".to_string(),
                glob: "*/SKILL.md".to_string(),
            },
            skill_extraction(),
        )
    }
}

/// Anthropic's documented `.claude/commands/*.md` kind: the skill surface's legacy
/// file placement (Claude Code merged commands into skills), a lone file whose
/// identity is the filename stem, the skill's field schema by import, registered on
/// the same two documented invocation channels as `skill`
/// (code.claude.com/docs/en/skills, retrieved 2026-07-07).
fn claude_code_command() -> CustomKind {
    CustomKind {
        format: Some(Format::YamlFrontmatter),
        unit_shape: Some(crate::kind::UnitShape::File),
        registration: skill_surface_registration(),
        ..CustomKind::new(
            "command",
            Governs {
                root: ".claude/commands".to_string(),
                glob: "*.md".to_string(),
            },
            skill_extraction(),
        )
    }
}

/// Anthropic's documented `.claude/agents/**/*.md` kind: a subagent definition,
/// identity from its frontmatter `name` field (never the filename), discovered
/// recursively — any containing subdirectory is purely organizational, per the docs'
/// own `agents/review/`, `agents/research/` example — registering only on the
/// description-trigger channel (code.claude.com/docs/en/sub-agents, retrieved
/// 2026-07-07).
fn claude_code_agent() -> CustomKind {
    CustomKind {
        format: Some(Format::YamlFrontmatter),
        unit_shape: Some(crate::kind::UnitShape::NamedField {
            field: "name".to_string(),
        }),
        registration: vec![Registration::DescriptionTrigger {
            field: "description".to_string(),
        }],
        ..CustomKind::new(
            "agent",
            Governs {
                root: ".claude/agents".to_string(),
                glob: "**/*.md".to_string(),
            },
            Extraction::new(vec![
                Primitive::Field {
                    key: "name".to_string(),
                },
                Primitive::Field {
                    key: "description".to_string(),
                },
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]),
        )
    }
}

/// Anthropic's documented `.claude/rules/*.md` kind: a lone file whose identity is
/// the filename stem, activated by its `paths` glob (or unconditionally, when absent).
fn claude_code_rule() -> CustomKind {
    CustomKind {
        format: Some(Format::YamlFrontmatter),
        unit_shape: Some(crate::kind::UnitShape::File),
        registration: vec![Registration::PathsMatch {
            field: "paths".to_string(),
        }],
        ..CustomKind::new(
            "rule",
            Governs {
                root: ".claude/rules".to_string(),
                glob: "*.md".to_string(),
            },
            Extraction::new(vec![
                Primitive::Field {
                    key: "paths".to_string(),
                },
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]),
        )
    }
}

/// Anthropic's documented `CLAUDE.md` memory kind: every `CLAUDE.md` in the
/// repository, frontmatter-less (no `format`), loaded unconditionally at launch.
fn claude_code_memory() -> CustomKind {
    CustomKind {
        unit_shape: Some(crate::kind::UnitShape::File),
        registration: vec![Registration::Always],
        ..CustomKind::new(
            "memory",
            Governs {
                root: ".".to_string(),
                glob: "**/CLAUDE.md".to_string(),
            },
            Extraction::new(vec![
                Primitive::Directives {
                    syntax: crate::kind::DirectiveSyntax::AtImport,
                },
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]),
        )
    }
}

/// Every embedded built-in kind, freshly constructed — the compiled default program's
/// whole kind set, in no particular order (callers key by [`CustomKind::name`]).
fn all_kinds() -> Vec<CustomKind> {
    vec![
        claude_code_agent(),
        claude_code_command(),
        claude_code_skill(),
        claude_code_rule(),
        claude_code_memory(),
    ]
}

/// The built-in kind a bare `name` resolves to, or `None` if none carries it. Bare
/// name is the whole identity now, so this is a plain lookup.
///
/// # Errors
///
/// Never fails; the `Result` is kept for API stability (every call site already
/// threads `?` through it).
pub fn definition(name: &str) -> Result<Option<CustomKind>, KindError> {
    Ok(all_kinds().into_iter().find(|kind| kind.name == name))
}

/// The built-in kind a bare `name` resolves to, named by its own bare label — kept for
/// call sites that ask for a kind's identity rather than its full definition. Always
/// equal to `name` itself when the kind is embedded, since there is no provider axis
/// to resolve through.
///
/// # Errors
///
/// Never fails; the `Result` is kept for API stability.
pub fn qualified(name: &str) -> Result<Option<String>, KindError> {
    Ok(definition(name)?.map(|kind| kind.name))
}

/// Every embedded built-in kind, keyed by its bare name — the compiled default
/// program's kind roster. Infallible — every entry is Rust data.
///
/// # Errors
///
/// Never fails; the `Result` is kept for API stability (every call site already
/// threads `?` through it).
pub fn definitions() -> Result<BTreeMap<String, CustomKind>, KindError> {
    Ok(all_kinds()
        .into_iter()
        .map(|kind| (kind.name.clone(), kind))
        .collect())
}

/// Extract a built-in skill's [`Features`] by running the embedded `skill` kind's
/// extraction over a generically-loaded surface [`Unit`] — the same composed path
/// every kind reads, with
/// **no IR→Unit adapter on the check read**: the caller loads the surface member
/// document through [`Unit::from_member_document`](crate::kind::Unit::from_member_document),
/// exactly as any other kind's members load.
#[must_use]
pub fn skill_features(unit: &Unit) -> Features {
    features(&claude_code_skill(), unit)
}

/// Extract a built-in rule's [`Features`] the same way [`skill_features`] does — the
/// embedded `rule` kind's extraction over the rule's generically-loaded surface [`Unit`].
#[must_use]
pub fn rule_features(unit: &Unit) -> Features {
    features(&claude_code_rule(), unit)
}

/// Run a built-in `kind`'s embedded extraction over `unit`, then fold every preserved
/// frontmatter key the composed primitives did not name into the feature map — the
/// built-in adapter's **permissive extraction**: an unknown key on a known artifact is already
/// extracted, so a clause (a `forbidden_keys`) can range over it. The closed algebra
/// cannot enumerate unknown keys, so this bulk preservation is the adapter's, while
/// each documented field is the composed extraction's. `or_insert` leaves each field
/// the composed extractor already yielded untouched.
///
/// Takes the resolved [`CustomKind`] rather than a name (the `check` gate holds it from
/// [`definitions`]), so it is total — the extraction cannot fail once the definition is
/// in hand. [`skill_features`]/[`rule_features`] stay the thin callers over `skill`/`rule`.
#[must_use]
pub fn features(kind: &CustomKind, unit: &Unit) -> Features {
    let mut features = kind.extract(unit);
    for (key, value) in &unit.frontmatter {
        features
            .fields
            .entry(key.clone())
            .or_insert_with(|| extract::json_to_feature(value));
    }
    features
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::Edge;
    use crate::kind::Governs;

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
        // The composed extractor: the documented frontmatter fields (`version` is
        // in neither the agentskills.io spec nor Claude Code's table — dropped), then the
        // markdown-structure primitives, in order.
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
                    key: "license".to_string()
                },
                Primitive::Field {
                    key: "disable-model-invocation".to_string()
                },
                Primitive::Field {
                    key: "user-invocable".to_string()
                },
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]
        );
        // The built-in `skill` kind declares no relationships.
        assert_eq!(skill.relationships, Vec::<Edge>::new());
        // Registers on both documented invocation channels — a set, not a scalar.
        assert_eq!(
            skill.registration,
            vec![
                Registration::UserInvoked,
                Registration::DescriptionTrigger {
                    field: "description".to_string()
                },
            ]
        );
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
        // A singleton channel set.
        assert_eq!(
            rule.registration,
            vec![Registration::PathsMatch {
                field: "paths".to_string()
            }]
        );
    }

    #[test]
    fn an_unknown_kind_name_is_none() {
        assert!(definition("spec").unwrap().is_none());
    }

    #[test]
    fn definitions_enumerates_the_embedded_kind_set_by_bare_name() {
        let all = definitions().unwrap();
        assert_eq!(
            all.keys().map(String::as_str).collect::<Vec<_>>(),
            vec!["agent", "command", "memory", "rule", "skill"]
        );
    }

    #[test]
    fn qualified_names_every_embedded_kind_by_its_own_bare_name() {
        // No provider axis left to resolve through — a bare name's qualified identity
        // is always itself.
        for bare in ["agent", "command", "skill", "rule", "memory"] {
            assert_eq!(qualified(bare).unwrap().as_deref(), Some(bare));
        }
        assert!(qualified("spec").unwrap().is_none());
    }

    #[test]
    fn agent_definition_matches_the_hand_authored_kind() {
        let agent = definition("agent").unwrap().expect("agent is embedded");

        assert_eq!(agent.name, "agent");
        assert_eq!(
            agent.governs,
            Governs {
                root: ".claude/agents".to_string(),
                glob: "**/*.md".to_string(),
            }
        );
        assert_eq!(
            agent.extraction.primitives(),
            &[
                Primitive::Field {
                    key: "name".to_string()
                },
                Primitive::Field {
                    key: "description".to_string()
                },
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]
        );
        assert_eq!(agent.relationships, Vec::<Edge>::new());
        // Named-field identity — the third mode, distinct from `skill`'s directory
        // shape and `rule`/`command`'s file-stem shape.
        assert_eq!(
            agent.unit_shape,
            Some(crate::kind::UnitShape::NamedField {
                field: "name".to_string()
            })
        );
        // Registers on the description-trigger channel only — no user-invoked slash
        // command, unlike `skill`/`command`.
        assert_eq!(
            agent.registration,
            vec![Registration::DescriptionTrigger {
                field: "description".to_string()
            }]
        );
    }

    #[test]
    fn command_definition_matches_the_hand_authored_kind() {
        let command = definition("command").unwrap().expect("command is embedded");

        assert_eq!(command.name, "command");
        assert_eq!(
            command.governs,
            Governs {
                root: ".claude/commands".to_string(),
                glob: "*.md".to_string(),
            }
        );
        // The skill's field schema, reused verbatim — command is a second placement of
        // the skill surface, not a second schema.
        assert_eq!(
            command.extraction.primitives(),
            definition("skill")
                .unwrap()
                .unwrap()
                .extraction
                .primitives()
        );
        assert_eq!(command.relationships, Vec::<Edge>::new());
        // File-shaped, like `rule` — identity is the filename stem, no `name` field
        // required for identity.
        assert_eq!(command.unit_shape, Some(crate::kind::UnitShape::File));
        // Registers on both documented invocation channels, exactly like `skill`.
        assert_eq!(
            command.registration,
            vec![
                Registration::UserInvoked,
                Registration::DescriptionTrigger {
                    field: "description".to_string()
                },
            ]
        );
    }

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> std::path::PathBuf {
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
    /// as `import`/`emit` project it ([`crate::frontmatter::Member::to_document`]),
    /// then reload it through the generic surface loader `check` reads — the built-in
    /// kind's member-document read, one generic adapter, no per-kind IR.
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
        use crate::extract::{FeatureValue, ValueType};

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
        let features = skill_features(&unit);

        // The documented fields come off the composed `field` primitives.
        assert_eq!(
            features.field("name"),
            Some(&FeatureValue::scalar(ValueType::String, "demo"))
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
    fn skill_features_extract_the_invocation_modulating_fields() {
        use crate::extract::{FeatureValue, ValueType};

        let parent = tmpdir("skill-modulators");
        let src = parent.join("deploy");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("SKILL.md"),
            "---\n\
name: deploy\n\
description: Deploy the application to production.\n\
disable-model-invocation: true\n\
---\n\
# Deploy\n",
        )
        .unwrap();
        let skill = definition("skill").unwrap().unwrap();
        let member =
            crate::frontmatter::Member::from_source(&skill, &src.join("SKILL.md")).unwrap();
        let unit = surface_unit(&member, "SKILL.md", &parent.join("surface-deploy"));
        let features = skill_features(&unit);

        // `disable-model-invocation`/`user-invocable` are ordinary declared fields — a
        // clause can range over them exactly like `name`/`description`.
        assert_eq!(
            features.field("disable-model-invocation"),
            Some(&FeatureValue::scalar(ValueType::Boolean, "true"))
        );
        // Absent when the author never sets it — never a phantom default.
        assert!(!features.has_field("user-invocable"));
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
        let features = rule_features(&unit);
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
        let bare_features = rule_features(&bare_unit);
        assert!(bare_features.fields.is_empty());
        assert_eq!(bare_features.body_lines, 3);
    }
}
