//! The embedded built-in kind std-lib.
//!
//! `temper` ships the read-side definitions of the known-harness kinds as plain Rust
//! data below — the compiled default program the engine carries for SDK-less checking.
//!
//! A built-in kind's definition is a [`CustomKind`] like any other — assembled with
//! [`CustomKind::new`] — and validated as any kind is; this module only sources its
//! facts from Rust literals instead of a parsed header. Identity is flat: a kind's bare
//! name is its whole identity, so the kinds below never collide.

use std::collections::BTreeMap;

use serde_json::Value as JsonValue;

use crate::compose::Edge;
use crate::drift::NestedMemberRow;
use crate::extract::{self, Features};
use crate::kind::{
    CollectionAddress, CollectionKeyPath, Content, CustomKind, Extraction, Format, Governs,
    Primitive, Registration, Template, Unit,
};
use crate::tap::TapEvent;

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
/// 2026-07-16).
///
/// Its one template layer names the bundled reference documents a skill's directory
/// carries — `supporting-doc` children at the directory's own markdown, the documented
/// placement (same source, "Add supporting files"). The pattern is the host's fact, so
/// it is what discovery classifies a skill's companion files through; a supporting file
/// of another type matches nothing and stays unmodeled.
fn claude_code_skill() -> CustomKind {
    CustomKind {
        format: Some(Format::YamlFrontmatter),
        unit_shape: Some(crate::kind::UnitShape::Directory),
        registration: skill_surface_registration(),
        templates: vec![Template {
            kind: "supporting-doc".to_string(),
            path: Some("*.md".to_string()),
        }],
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

/// Anthropic's documented supporting-file kind: a skill's bundled reference document,
/// at the nested-file locus — its path composes from its host skill's unit and the
/// host's template pattern, so it governs no glob and nothing discovers it at one
/// (code.claude.com/docs/en/skills, "Add supporting files", retrieved 2026-07-16). A
/// lone file, frontmatterless (no `format` — the whole file is body), identity from the
/// filename, and channel-less: it reaches the world only through the skill whose body
/// references it.
fn claude_code_supporting_doc() -> CustomKind {
    CustomKind {
        unit_shape: Some(crate::kind::UnitShape::File),
        ..CustomKind::nested_file(
            "supporting-doc",
            Extraction::new(vec![
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]),
        )
    }
}

/// Anthropic's documented `.claude/commands/*.md` kind: the skill surface's legacy
/// file placement (Claude Code merged commands into skills), a lone file whose
/// identity is the filename stem, the skill's field schema by import, registered on
/// the same two documented invocation channels as `skill`
/// (code.claude.com/docs/en/skills, retrieved 2026-07-16).
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
/// 2026-07-16).
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

/// Anthropic's documented `settings.json` `hooks.<Event>` kind: a hook is a fields-only
/// registration member surfacing inside the project settings manifest, keyed under its
/// lifecycle event (`code.claude.com/docs/en/hooks`, retrieved 2026-07-16). It owns no
/// file of its own — the manifest is discovered off the `.claude/settings.json` locus and
/// each `hooks.<Event>` entry read as a member — carries no body (`Content::Fields`), and
/// registers on the `event` channel, its event surfaced as a field off the collection key.
fn claude_code_hook() -> CustomKind {
    CustomKind {
        unit_shape: Some(crate::kind::UnitShape::File),
        registration: vec![Registration::Event {
            field: "event".to_string(),
        }],
        content: Content::Fields,
        collection_address: Some(CollectionAddress {
            manifest: "settings.json".to_string(),
            key_path: CollectionKeyPath::HooksEvent,
        }),
        ..CustomKind::new(
            "hook",
            Governs {
                root: ".claude".to_string(),
                glob: "settings.json".to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }
}

/// Anthropic's documented `.mcp.json` `mcpServers.*` kind: an MCP server is a fields-only
/// registration member surfacing inside the project MCP manifest, keyed by name
/// (`code.claude.com/docs/en/mcp`, retrieved 2026-07-16). It owns no file of its own — the
/// manifest is discovered off the `.mcp.json` locus and each `mcpServers.*` entry read as a
/// member — carries no body (`Content::Fields`), and registers on the `connection` channel.
/// Unlike a hook (whose event value is an array), a server entry is an object, so its
/// fields fold into the member the read surfaces.
fn claude_code_mcp_server() -> CustomKind {
    CustomKind {
        unit_shape: Some(crate::kind::UnitShape::File),
        registration: vec![Registration::Connection],
        content: Content::Fields,
        collection_address: Some(CollectionAddress {
            manifest: ".mcp.json".to_string(),
            key_path: CollectionKeyPath::McpServers,
        }),
        ..CustomKind::new(
            "mcp-server",
            Governs {
                root: ".".to_string(),
                glob: ".mcp.json".to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }
}

/// Anthropic's documented `settings.json` `enabledPlugins` kind: an installed plugin is a
/// fields-only registration member surfacing inside the project settings manifest, keyed by
/// its `<plugin>@<marketplace>` identity (`code.claude.com/docs/en/plugins-reference`,
/// retrieved 2026-07-16). It owns no file of its own — the manifest is discovered off the
/// `.claude/settings.json` locus and each `enabledPlugins` entry read as a member — carries
/// no body (`Content::Fields`), and registers on the `enablement` channel: the entry's own
/// presence is the registration.
///
/// Unlike a hook (array value) or an MCP server (object value), an entry's value is a bare
/// scalar, so the member carries exactly one declared field (`enabled`) and folds no object.
/// The marketplace half of its `<plugin>@<marketplace>` key is a declared edge to the
/// `known-marketplace` member it names — split off the composite key at read and resolved on
/// the reference graph, so an enablement naming a marketplace no registration declares dangles.
///
/// The members a plugin *contributes* — its skills, agents, hooks, MCP servers — live in the
/// plugin cache, outside the corpus. Their reach is unmodeled and named as such: this kind
/// types the enablement entry, never the plugin's own surface.
fn claude_code_installed_plugin() -> CustomKind {
    CustomKind {
        unit_shape: Some(crate::kind::UnitShape::File),
        registration: vec![Registration::Enablement],
        content: Content::Fields,
        collection_address: Some(CollectionAddress {
            manifest: "settings.json".to_string(),
            key_path: CollectionKeyPath::EnabledPlugins,
        }),
        // The marketplace half of the `<plugin>@<marketplace>` key is an edge to the
        // `known-marketplace` member it names (decision 0039); the read splits it off the
        // composite key onto the `marketplace` field the reference graph resolves.
        relationships: vec![Edge {
            field: crate::kind::MARKETPLACE_FIELD.to_string(),
            from: "installed-plugin".to_string(),
            to: vec!["known-marketplace".to_string()],
        }],
        ..CustomKind::new(
            "installed-plugin",
            Governs {
                root: ".claude".to_string(),
                glob: "settings.json".to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }
}

/// Anthropic's documented `settings.json` `extraKnownMarketplaces` kind: a known
/// marketplace is a fields-only registration member surfacing inside the project settings
/// manifest, keyed by the marketplace name a user has registered
/// (`code.claude.com/docs/en/plugin-marketplaces`, retrieved 2026-07-17). It owns no file of
/// its own — the manifest is discovered off the `.claude/settings.json` locus and each
/// `extraKnownMarketplaces` entry read as a member — carries no body (`Content::Fields`), and
/// registers on the `registry` channel: the entry's own presence is the registration.
///
/// The consumer half of the plugin-distribution graph, distinct from the publisher-side
/// `marketplace` catalog: that document is a marketplace's own `marketplace.json`, this entry
/// is one consumer's record that they have added it. Its value is an object (unlike an
/// installed plugin's bare boolean), so the object's fields — the `source` union and
/// `autoUpdate` — fold into the member the read surfaces.
fn claude_code_known_marketplace() -> CustomKind {
    CustomKind {
        unit_shape: Some(crate::kind::UnitShape::File),
        registration: vec![Registration::Registry],
        content: Content::Fields,
        collection_address: Some(CollectionAddress {
            manifest: "settings.json".to_string(),
            key_path: CollectionKeyPath::ExtraKnownMarketplaces,
        }),
        ..CustomKind::new(
            "known-marketplace",
            Governs {
                root: ".claude".to_string(),
                glob: "settings.json".to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }
}

/// Anthropic's documented `.claude-plugin/plugin.json` kind: a plugin pack's identity and
/// metadata, the whole file one JSON document rather than frontmatter over a body
/// (`code.claude.com/docs/en/plugins-reference`, retrieved 2026-07-16). Identity reads from
/// the top-level `name` — the file's stem is `plugin` for every manifest ever written, so
/// the named-field mode is the only one that distinguishes two.
///
/// It owns its file, so it carries no collection address — unlike the registration members
/// that surface *inside* a manifest, this kind is the manifest. Channel-less: it carries
/// distribution metadata rather than session content, so it reaches the model on no channel
/// of its own; what reads it is the installer.
fn claude_code_plugin_manifest() -> CustomKind {
    CustomKind {
        format: Some(Format::JsonDocument),
        unit_shape: Some(crate::kind::UnitShape::NamedField {
            field: "name".to_string(),
        }),
        ..CustomKind::new(
            "plugin-manifest",
            Governs {
                root: ".claude-plugin".to_string(),
                glob: "plugin.json".to_string(),
            },
            Extraction::new(vec![Primitive::Field {
                key: "name".to_string(),
            }]),
        )
    }
}

/// Anthropic's documented `.claude-plugin/marketplace.json` kind: the catalog a
/// marketplace distributes its plugins through, the whole file one JSON document
/// (`code.claude.com/docs/en/plugin-marketplaces`, retrieved 2026-07-16). Identity reads
/// from the top-level `name` for the same reason its `plugin-manifest` sibling's does —
/// the stem is `marketplace` for every catalog ever written.
///
/// Its glob is distinct from `plugin-manifest`'s, so the two kinds share the
/// `.claude-plugin` root and never contend for a file.
///
/// It owns its file, so it carries no collection address. Channel-less: a catalog is
/// distribution metadata read by the installer, never surfaced to the model.
fn claude_code_marketplace() -> CustomKind {
    CustomKind {
        format: Some(Format::JsonDocument),
        unit_shape: Some(crate::kind::UnitShape::NamedField {
            field: "name".to_string(),
        }),
        ..CustomKind::new(
            "marketplace",
            Governs {
                root: ".claude-plugin".to_string(),
                glob: "marketplace.json".to_string(),
            },
            Extraction::new(vec![Primitive::Field {
                key: "name".to_string(),
            }]),
        )
    }
}

/// Anthropic's documented `.claude/settings.local.json` kind: the machine's own per-project
/// settings overlay, the whole file one JSON document at the **local** commitment class
/// (`code.claude.com/docs/en/settings`, retrieved 2026-07-16). Read in place at check and
/// gated, never an emit input or target, and no row of its members' ever lands in a lock.
///
/// Identity is the fixed singleton stem `settings.local` (the `file` unit shape): every
/// machine's overlay is the one file at this path, so no declared key names it. Its
/// documented top-level keys are its fields and the unschematized residue stays opaque —
/// a locally-registered hook or plugin enablement is one such opaque field, never a modeled
/// member (those kinds read the committed `settings.json`). Channel-less: machine
/// configuration read by the harness, never surfaced to the model.
fn claude_code_settings_local() -> CustomKind {
    CustomKind {
        format: Some(Format::JsonDocument),
        unit_shape: Some(crate::kind::UnitShape::File),
        ..CustomKind::new(
            "settings-local",
            Governs {
                root: ".claude".to_string(),
                glob: "settings.local.json".to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }
    .local()
}

/// temper's own **dial** kind: `.temper/dial.toml`, a local file locus whose entries name
/// a clause by its compiled address and declare the severity this machine reads it at.
///
/// The one kind here that is not a claude-code kind — Claude Code never reads this
/// document; temper's own gate does — so it is the SDK's root export rather than its
/// provider subpath's, and its facts are the corpus's rather than an external source's.
///
/// Identity reads from the top-level `name` for its `marketplace` sibling's reason: the
/// stem is `dial` on every machine that has one. Its `clause` entries reach the member's
/// fields the way every other top-level key of a document member does — the whole table
/// is the member's own fields — so [`crate::dial`] reads them off the extracted
/// [`Features`] rather than re-parsing the document behind the contract's back.
///
/// Channel-less, and [`Commitment::Local`](crate::kind::Commitment::Local): read in place
/// at check, never an emit input or target, and no row of its members' ever enters the
/// lock.
fn temper_dial() -> CustomKind {
    CustomKind {
        format: Some(Format::TomlDocument),
        unit_shape: Some(crate::kind::UnitShape::NamedField {
            field: "name".to_string(),
        }),
        ..CustomKind::new(
            crate::dial::KIND,
            Governs {
                root: crate::WORKSPACE_DIR.to_string(),
                glob: crate::dial::DOCUMENT.to_string(),
            },
            Extraction::new(vec![Primitive::Field {
                key: "name".to_string(),
            }]),
        )
    }
    .local()
}

/// Every embedded built-in kind, freshly constructed — the compiled default program's
/// whole kind set, in no particular order (callers key by [`CustomKind::name`]).
fn all_kinds() -> Vec<CustomKind> {
    vec![
        temper_dial(),
        claude_code_agent(),
        claude_code_command(),
        claude_code_hook(),
        claude_code_installed_plugin(),
        claude_code_known_marketplace(),
        claude_code_marketplace(),
        claude_code_mcp_server(),
        claude_code_plugin_manifest(),
        claude_code_settings_local(),
        claude_code_skill(),
        claude_code_supporting_doc(),
        claude_code_rule(),
        claude_code_memory(),
    ]
}

/// The built-in kind a bare `name` resolves to, or `None` if none carries it. Bare
/// name is the whole identity now, so this is a plain lookup.
pub fn definition(name: &str) -> Option<CustomKind> {
    all_kinds().into_iter().find(|kind| kind.name == name)
}

/// Every embedded built-in kind, keyed by its bare name — the compiled default
/// program's kind roster. Infallible — every entry is Rust data.
pub fn definitions() -> BTreeMap<String, CustomKind> {
    all_kinds()
        .into_iter()
        .map(|kind| (kind.name.clone(), kind))
        .collect()
}

/// Extract a built-in skill's [`Features`] by running the embedded `skill` kind's
/// extraction over a generic [`Unit`] — the same composed path
/// every kind reads, with
/// **no IR→Unit adapter on the check read**: the caller builds the `Unit` straight
/// off the imported [`crate::frontmatter::Member`], exactly as any other kind's
/// members load. `skill`'s one template is a *file* layer, whose children own units of
/// their own — no embedded layer, so no lock row addresses a nested member of one.
#[must_use]
pub fn skill_features(unit: &Unit) -> Features {
    features(&claude_code_skill(), unit, &[])
}

/// Extract a built-in rule's [`Features`] the same way [`skill_features`] does — the
/// embedded `rule` kind's extraction over the rule's generically-loaded surface [`Unit`].
#[must_use]
pub fn rule_features(unit: &Unit) -> Features {
    features(&claude_code_rule(), unit, &[])
}

/// Run a built-in `kind`'s embedded extraction over `unit`, fold every preserved
/// frontmatter key the composed primitives did not name into the feature map, and
/// resolve `unit`'s own nested members off `nested_members` — the lock's declared
/// [`NestedMemberRow`] family, matched by this member's `kind:name` address
/// ([`crate::drift::nested_members_from_rows`]). The **permissive extraction**: an unknown
/// key on a known artifact is already extracted, so a clause (a `forbidden_keys`) can
/// range over it. The closed algebra cannot enumerate unknown keys, so this bulk
/// preservation is the adapter's, while each documented field is the composed
/// extraction's. `or_insert` leaves each field the composed extractor already yielded
/// untouched.
///
/// The **sole choke point** every custom/built-in member's [`Features`] is built
/// through — nested-member facts are declared, never re-derived by re-parsing a
/// rendered fence (0018, "the projection is not the database"). Takes the resolved
/// [`CustomKind`] rather than a name (the `check` gate holds it from [`definitions`]),
/// so it is total — the extraction cannot fail once the definition is in hand.
/// [`skill_features`]/[`rule_features`] stay the thin callers over `skill`/`rule`,
/// neither of which any lock row can address a nested member of.
#[must_use]
pub fn features(kind: &CustomKind, unit: &Unit, nested_members: &[NestedMemberRow]) -> Features {
    let mut features = kind.extract(unit);
    features.nested_members = crate::drift::nested_members_from_rows(
        &extract::host_address(&kind.name, &unit.id),
        nested_members,
    );
    for (key, value) in &unit.frontmatter {
        features
            .fields
            .entry(key.clone())
            .or_insert_with(|| value.clone());
    }
    features
}

/// Classify a Claude Code hook payload into its lifecycle event, identity, and optional reason.
///
/// The payload shapes are Claude Code's hook contract, an external fact:
/// code.claude.com/docs/en/hooks (retrieved 2026-07-17). InstructionsLoaded carries
/// {file_path, load_reason, content}; UserPromptExpansion {command_name, expanded_prompt};
/// PostToolUse {tool_name, tool_input, tool_response}; a skill invocation rides PostToolUse
/// with tool_name="Skill" and the skill name under tool_input.skill. The prose fields —
/// content, expanded_prompt, tool_response — are never included in the returned classification.
///
/// Returns `None` if the payload does not parse, names no recognized event, or lacks the
/// identity field its event needs.
#[must_use]
pub(crate) fn classify_claude_code_hook_payload(
    value: &JsonValue,
) -> Option<(TapEvent, String, Option<String>)> {
    let string = |key: &str| value.get(key).and_then(JsonValue::as_str);

    match string("hook_event_name")? {
        "InstructionsLoaded" => {
            let identity = string("file_path")?.to_string();
            let reason = string("load_reason").map(str::to_string);
            Some((TapEvent::InstructionsLoaded, identity, reason))
        }
        "UserPromptExpansion" => {
            let identity = string("command_name")?.to_string();
            Some((TapEvent::UserPromptExpansion, identity, None))
        }
        "PostToolUse" => {
            let tool = string("tool_name")?;
            if tool == "Skill" {
                let skill = value
                    .get("tool_input")
                    .and_then(|input| input.get("skill"))
                    .and_then(JsonValue::as_str)?;
                Some((TapEvent::SkillInvoked, skill.to_string(), None))
            } else {
                Some((TapEvent::ToolUse, tool.to_string(), None))
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::Edge;
    use crate::kind::Governs;
    use crate::test_support::tmpdir;

    #[test]
    fn skill_definition_matches_the_hand_authored_kind() {
        let skill = definition("skill").expect("skill is embedded");

        assert_eq!(skill.name, "skill");
        assert_eq!(
            skill.governs,
            Some(Governs {
                root: ".claude/skills".to_string(),
                glob: "*/SKILL.md".to_string(),
            })
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
        // One file-child layer: its bundled reference documents, at the directory's own
        // markdown. The pattern's one home is the host, so the child governs no glob.
        assert_eq!(
            skill.templates,
            vec![Template {
                kind: "supporting-doc".to_string(),
                path: Some("*.md".to_string()),
            }]
        );
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
        let rule = definition("rule").expect("rule is embedded");

        assert_eq!(rule.name, "rule");
        assert_eq!(
            rule.governs,
            Some(Governs {
                root: ".claude/rules".to_string(),
                glob: "*.md".to_string(),
            })
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
        assert!(definition("spec").is_none());
    }

    #[test]
    fn definitions_enumerates_the_embedded_kind_set_by_bare_name() {
        let all = definitions();
        assert_eq!(
            all.keys().map(String::as_str).collect::<Vec<_>>(),
            vec![
                "agent",
                "command",
                "dial",
                "hook",
                "installed-plugin",
                "known-marketplace",
                "marketplace",
                "mcp-server",
                "memory",
                "plugin-manifest",
                "rule",
                "settings-local",
                "skill",
                "supporting-doc"
            ]
        );
    }

    #[test]
    fn supporting_doc_is_a_prose_only_nested_file_kind_governing_no_glob() {
        let doc = definition("supporting-doc").expect("supporting-doc is embedded");

        assert_eq!(doc.name, "supporting-doc");
        // The nested-file locus: neither half is the child's — its path composes from
        // its host skill's unit and `skill`'s own template pattern, so nothing discovers
        // it at a glob and it owns no surface subdirectory.
        assert_eq!(doc.governs, None);
        assert_eq!(doc.surface_subdir(), None);
        // A lone file, frontmatterless — the whole file is body.
        assert_eq!(doc.unit_shape, Some(crate::kind::UnitShape::File));
        assert_eq!(doc.format, None);
        // Channel-less: it reaches the world only through the skill referencing it.
        assert_eq!(doc.registration, Vec::<Registration>::new());
        // Fields-free: the markdown-structure primitives and not one declared field —
        // the format documents no frontmatter schema for a supporting file.
        assert_eq!(
            doc.extraction.primitives(),
            &[
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]
        );
        // Prose-only, and body-bearing — never the fields-only registration shape.
        assert_eq!(doc.content, Content::File);
        assert_eq!(doc.collection_address, None);
        // It hosts nothing: the nesting stops one layer down.
        assert!(doc.templates.is_empty());
    }

    #[test]
    fn agent_definition_matches_the_hand_authored_kind() {
        let agent = definition("agent").expect("agent is embedded");

        assert_eq!(agent.name, "agent");
        assert_eq!(
            agent.governs,
            Some(Governs {
                root: ".claude/agents".to_string(),
                glob: "**/*.md".to_string(),
            })
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
        let command = definition("command").expect("command is embedded");

        assert_eq!(command.name, "command");
        assert_eq!(
            command.governs,
            Some(Governs {
                root: ".claude/commands".to_string(),
                glob: "*.md".to_string(),
            })
        );
        // The skill's field schema, reused verbatim — command is a second placement of
        // the skill surface, not a second schema.
        assert_eq!(
            command.extraction.primitives(),
            definition("skill").unwrap().extraction.primitives()
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

    #[test]
    fn hook_definition_is_a_fields_only_manifest_kind_at_the_hooks_collection_address() {
        use crate::kind::{CollectionAddress, CollectionKeyPath, Content};

        let hook = definition("hook").expect("hook is embedded");

        assert_eq!(hook.name, "hook");
        // Discovered off the `.claude/settings.json` manifest locus, never a file tree of
        // its own.
        assert_eq!(
            hook.governs,
            Some(Governs {
                root: ".claude".to_string(),
                glob: "settings.json".to_string(),
            })
        );
        // Fields-only: no body slot, distinct from every file-content built-in.
        assert_eq!(hook.content, Content::Fields);
        // The manifest fence: which manifest, which key path its registration keys at.
        assert_eq!(
            hook.collection_address,
            Some(CollectionAddress {
                manifest: "settings.json".to_string(),
                key_path: CollectionKeyPath::HooksEvent,
            })
        );
        // Registers on the `event` channel — the lifecycle event it fires at, surfaced
        // as a field off the collection key.
        assert_eq!(
            hook.registration,
            vec![Registration::Event {
                field: "event".to_string()
            }]
        );
        // A registration member is fields-and-edges only — no declared frontmatter fields
        // (its event reads off the manifest key, folded in at read time).
        assert_eq!(hook.extraction.primitives(), &[]);
    }

    #[test]
    fn mcp_server_definition_is_a_fields_only_manifest_kind_at_the_mcp_servers_collection_address()
    {
        use crate::kind::{CollectionAddress, CollectionKeyPath, Content};

        let mcp = definition("mcp-server").expect("mcp-server is embedded");

        assert_eq!(mcp.name, "mcp-server");
        // Discovered off the root `.mcp.json` manifest locus, never a file tree of its own.
        assert_eq!(
            mcp.governs,
            Some(Governs {
                root: ".".to_string(),
                glob: ".mcp.json".to_string(),
            })
        );
        assert_eq!(mcp.content, Content::Fields);
        assert_eq!(
            mcp.collection_address,
            Some(CollectionAddress {
                manifest: ".mcp.json".to_string(),
                key_path: CollectionKeyPath::McpServers,
            })
        );
        // Registers on the `connection` channel — the harness connects to it, a runtime
        // fact no repo criterion decides dead.
        assert_eq!(mcp.registration, vec![Registration::Connection]);
        // `mcpServers.*` names no key field, so a server carries only its own object
        // fields, folded in at read time — no declared frontmatter primitives.
        assert_eq!(mcp.extraction.primitives(), &[]);
    }

    /// Lift an imported [`crate::frontmatter::Member`] straight into the raw [`Unit`]
    /// the composed extractor reads — the same fields a built-in kind's member carries
    /// into `check`, with no disk round trip.
    fn surface_unit(member: &crate::frontmatter::Member) -> Unit {
        Unit {
            id: member.id.clone(),
            frontmatter: member.fields.iter().cloned().collect(),
            body: member.body.clone(),
            source_path: member.provenance.source_path.clone(),
            satisfies: member
                .satisfies
                .iter()
                .map(|s| s.requirement.clone())
                .collect(),
            satisfies_clauses: member.satisfies.clone(),
        }
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
        let skill = definition("skill").unwrap();
        let mut member =
            crate::frontmatter::Member::from_source(&skill, &src.join("SKILL.md")).unwrap();
        // The authored representation edge — surfaced by the driver, kept out of `fields`.
        member.satisfies = vec![crate::document::Satisfies {
            requirement: "req.one".to_string(),
            rationale: Some("The human why, never a decidable feature.".to_string()),
        }];

        // Read the extracted features off the written surface, not a typed IR.
        let unit = surface_unit(&member);
        let features = skill_features(&unit);

        // The documented fields come off the composed `field` primitives.
        assert_eq!(
            features.field("name"),
            Some(FeatureValue::scalar(ValueType::String, "demo"))
        );
        // Permissive extraction: the unknown keys ride into the same feature map, so a
        // `forbidden_keys` clause can range over a project convention on a known artifact.
        assert_eq!(
            features.field("allowed-tools"),
            Some(FeatureValue::List(vec![
                "Bash".to_string(),
                "Read".to_string()
            ]))
        );
        assert_eq!(
            features
                .field("priority")
                .as_ref()
                .and_then(FeatureValue::as_scalar),
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
        let skill = definition("skill").unwrap();
        let member =
            crate::frontmatter::Member::from_source(&skill, &src.join("SKILL.md")).unwrap();
        let unit = surface_unit(&member);
        let features = skill_features(&unit);

        // `disable-model-invocation`/`user-invocable` are ordinary declared fields — a
        // clause can range over them exactly like `name`/`description`.
        assert_eq!(
            features.field("disable-model-invocation"),
            Some(FeatureValue::scalar(ValueType::Boolean, "true"))
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
        let rule = definition("rule").unwrap();

        std::fs::write(
            rules.join("rust.md"),
            "---\npaths:\n  - \"src/**/*.rs\"\n---\n# Rust\n\nBody.\n",
        )
        .unwrap();
        let member =
            crate::frontmatter::Member::from_source(&rule, &rules.join("rust.md")).unwrap();
        let unit = surface_unit(&member);
        let features = rule_features(&unit);
        assert_eq!(
            features.field("paths"),
            Some(FeatureValue::List(vec!["src/**/*.rs".to_string()]))
        );
        // `placement` reads the imported source directory off provenance, carried
        // through the surface — `rules`, not the projected surface directory.
        assert_eq!(features.source_dir.as_deref(), Some("rules"));

        // A rule with no frontmatter carries no fields at all — the whole file is body.
        std::fs::write(rules.join("collab.md"), "# Collaboration\n\nPushback.\n").unwrap();
        let bare =
            crate::frontmatter::Member::from_source(&rule, &rules.join("collab.md")).unwrap();
        let bare_unit = surface_unit(&bare);
        let bare_features = rule_features(&bare_unit);
        assert!(bare_features.fields.is_empty());
        assert_eq!(bare_features.body_lines, 3);
    }

    #[test]
    fn features_resolves_nested_members_off_the_lock_row_matching_this_members_address() {
        // The choke point every custom/built-in member's `Features` builds through:
        // a `NestedMemberRow` addressed to this exact `kind:id` folds in, one for a
        // different host is left out — never a re-parse of the rendered body.
        let parent = tmpdir("rule-nested-members");
        let rules = parent.join("rules");
        std::fs::create_dir_all(&rules).unwrap();
        let rule = definition("rule").unwrap();
        std::fs::write(rules.join("uses-directive.md"), "# Rule\n\nBody.\n").unwrap();
        let member =
            crate::frontmatter::Member::from_source(&rule, &rules.join("uses-directive.md"))
                .unwrap();
        let unit = surface_unit(&member);

        let rows = vec![
            crate::drift::NestedMemberRow {
                host: "rule:uses-directive".to_string(),
                kind: "directive".to_string(),
                key: "at-import".to_string(),
                leaves: BTreeMap::from([("target".to_string(), "some/path.md".to_string())]),
                collections: Vec::new(),
                placed_edges: None,
                rendered_lines: None,
                rendered_chars: None,
            },
            crate::drift::NestedMemberRow {
                host: "rule:some-other-rule".to_string(),
                kind: "directive".to_string(),
                key: "unrelated".to_string(),
                leaves: BTreeMap::new(),
                collections: Vec::new(),
                placed_edges: None,
                rendered_lines: None,
                rendered_chars: None,
            },
        ];

        let features = features(&rule, &unit, &rows);
        assert_eq!(features.nested_members.len(), 1);
        let nested = &features.nested_members[0];
        assert_eq!(nested.kind, "directive");
        assert_eq!(nested.key, "at-import");
        assert_eq!(
            nested.leaves.get("target").map(String::as_str),
            Some("some/path.md")
        );
    }
}
