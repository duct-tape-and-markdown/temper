//! The embedded built-in kind std-lib.
//!
//! Each built-in [`CustomKind`] is lifted off the embedded lock's kind rows
//! ([`crate::builtin_lock`]) — the same lift a committed lock's custom rows take — with the
//! few facts no row carries (declared field keys, the at-import directive, `skill`'s
//! bare-root file, `mcp-server`'s documented `type` default) layered on as named per-kind exceptions. Identity is flat: a kind's
//! bare name is its whole identity.
//!
//! This module is also the home for known Claude Code surfaces — the curated registry
//! of documented `.claude/` paths the coverage note's stray-entry scan passes over, so a
//! surface Claude Code itself owns never reads as an unclaimed entry.

use std::collections::BTreeMap;

use serde_json::Value as JsonValue;

use crate::compose::Edge;
use crate::drift::{KindFactRow, NestedMemberRow};
use crate::extract::Features;
use crate::kind::{Content, CustomKind, Extraction, Format, Primitive, Unit};
use crate::member_address;
use crate::tap::TapEvent;

/// The Claude Code harness root directory.
pub const CLAUDE_ROOT: &str = ".claude";

/// The maximum import-recursion depth reachability propagates a live importer's
/// liveness across — the `at-import` grammar is recursion-capped at four hops
/// (code.claude.com/docs/en/memory, retrieved 2026-07-20), so an import chain
/// deeper than this loads nothing at runtime and cannot carry liveness either.
pub const MAX_IMPORT_HOPS: usize = 4;

/// The kinds whose members the runtime also loads from scopes a check of the project does
/// not read, each with those scopes: a name such a kind's member does not carry in the tree
/// is undecidable there, never dangling. External facts: output styles load from
/// `~/.claude/output-styles`, the managed settings directory and plugins as well as the
/// project (code.claude.com/docs/en/output-styles, retrieved 2026-11-26); a marketplace a
/// user adds is "declared in user settings" (code.claude.com/docs/en/plugin-marketplaces,
/// retrieved 2026-11-26).
pub const OFF_TREE_SCOPES: &[(&str, &[&str])] = &[
    (
        "known-marketplace",
        &["marketplaces declared in user settings"],
    ),
    (
        "output-style",
        &[
            "user styles (`~/.claude/output-styles`)",
            "managed-policy styles",
            "plugin-provided styles",
        ],
    ),
];

/// The scopes outside the tree that may carry a member of `kind` — empty for a kind the
/// runtime loads from the tree alone ([`OFF_TREE_SCOPES`]).
#[must_use]
pub fn off_tree_scopes(kind: &str) -> &'static [&'static str] {
    OFF_TREE_SCOPES
        .iter()
        .find(|(name, _)| *name == kind)
        .map_or(&[], |(_, scopes)| scopes)
}

/// The curated exclusion list for the coverage note's stray-entry scan: each entry is a
/// documented Claude Code path directly under `.claude/` (verified against the settings
/// docs, code.claude.com/docs/en/settings, retrieved 2026-07-16 — an external fact cited
/// at its point of claim), so a surface Claude Code itself owns is never reported as an
/// unclaimed entry even where no in-scope kind governs it.
///
/// A locus a kind always governs is deliberately absent — skills under
/// `.claude/skills/`, commands under `.claude/commands/`, subagents under
/// `.claude/agents/`, rules under `.claude/rules/`: governance already excludes them.
/// So is any path outside `.claude/` (`.mcp.json`, `CLAUDE.md`): the scan never walks
/// there, so a row for one would exclude nothing. Hooks are **not** a directory — they
/// are configured inside `settings.json`, so no invented `.claude/hooks/` locus appears
/// (a false locus would be the exact uncited guess collaboration.md forbids).
pub const KNOWN_SURFACES: &[&str] = &[".claude/settings.json"];

/// The frontmatter (or document) keys each built-in kind extracts as fields, in declaration
/// order — the one extraction fact no kind-fact row carries, so it stays a named per-kind
/// exception layered onto the lift ([`lift`]). The keys are the documented ones:
/// `skill`/`command` share the skill surface's table (code.claude.com/docs/en/skills,
/// "Frontmatter reference", retrieved 2026-07-16), `agent` its identity and trigger
/// (code.claude.com/docs/en/sub-agents, retrieved 2026-07-16), `output-style` its four
/// optional fields (code.claude.com/docs/en/output-styles, retrieved 2026-11-25), `rule` its activation glob
/// (code.claude.com/docs/en/memory, retrieved 2026-11-24), and the named-field document
/// kinds the `name` key their identity reads from.
fn declared_field_keys(kind: &str) -> &'static [&'static str] {
    match kind {
        "skill" | "command" => &[
            "name",
            "description",
            "license",
            "disable-model-invocation",
            "user-invocable",
        ],
        "agent" => &["name", "description"],
        "output-style" => &[
            "name",
            "description",
            "keep-coding-instructions",
            "force-for-plugin",
        ],
        "rule" => &["paths"],
        "plugin-manifest" | "marketplace" | crate::dial::KIND => &["name"],
        _ => &[],
    }
}

/// The directive grammar a built-in kind's body declares, the second row-less extraction
/// fact: `memory` alone reads `@path` imports
/// (code.claude.com/docs/en/memory, retrieved 2026-07-20).
fn declared_directives(kind: &str) -> Option<crate::kind::DirectiveSyntax> {
    (kind == "memory").then_some(crate::kind::DirectiveSyntax::AtImport)
}

/// The bare-root file a built-in kind additionally recognizes at the harness root, the
/// third row-less fact: `skill`'s `<harness>/SKILL.md`, the documented bare-skill-repo
/// convention (code.claude.com/docs/en/skills, retrieved 2026-07-16).
fn declared_bare_root_file(kind: &str) -> Option<&'static str> {
    (kind == "skill").then_some("SKILL.md")
}

/// The documented defaults a built-in kind's top-level fields read as where a member leaves
/// them unauthored, the fourth row-less fact: an `mcp-server` entry with no `type` is a
/// stdio server (code.claude.com/docs/en/mcp, retrieved 2026-07-15 — the default
/// `McpServer.type` cites in `sdk/src/builtins.ts`). A `when` guard reads the field at this
/// value, so the stdio transport's `command` requirement binds a `type`-less entry exactly
/// as Claude Code does (decision 0101).
fn declared_field_defaults(kind: &str) -> &'static [(&'static str, &'static str)] {
    match kind {
        "mcp-server" => &[("type", "stdio")],
        _ => &[],
    }
}

/// Lift one embedded kind-fact row into its built-in [`CustomKind`]: every fact the row
/// carries comes off [`CustomKind::from_kind_fact_row`], the relationships off the lock's
/// own `edge` assembly facts, and the row-less extraction, bare-root and field-default
/// facts off the named exceptions above. The markdown-structure primitives the lift
/// composes stay only where a markdown body exists — never on a fields-only kind or a whole-file document.
/// A subkind's row names none of these exceptions: it takes the extraction and the
/// relationships off this lifted base when its base binds (`compose::resolve_subkinds`).
fn lift(row: &KindFactRow, edges: &[Edge]) -> CustomKind {
    let mut kind = CustomKind::from_kind_fact_row(row).expect(
        "src/builtin_lock.toml is compiled-in data produced by this crate's own emit — \
         a kind row that fails its lift is a build-time bug, never a runtime condition",
    );
    let markdown_body = kind.content != Content::Fields
        && !matches!(
            kind.format,
            Some(Format::JsonDocument | Format::TomlDocument)
        );
    let mut primitives: Vec<Primitive> = declared_field_keys(&kind.name)
        .iter()
        .map(|key| Primitive::Field {
            key: (*key).to_string(),
        })
        .collect();
    primitives
        .extend(declared_directives(&kind.name).map(|syntax| Primitive::Directives { syntax }));
    if markdown_body {
        primitives.extend(kind.extraction.primitives().iter().cloned());
    }
    kind.extraction = Extraction::new(primitives);
    kind.relationships = edges
        .iter()
        .filter(|edge| edge.from == kind.name)
        .cloned()
        .collect();
    kind.bare_root_file = declared_bare_root_file(&kind.name).map(str::to_string);
    kind.field_defaults = declared_field_defaults(&kind.name)
        .iter()
        .map(|(field, value)| ((*field).to_string(), (*value).to_string()))
        .collect();
    kind
}

/// The embedded lock's declared edges — compiled-in data, so a malformed edge fact is a
/// build-time bug rather than a runtime condition.
fn builtin_edges() -> Vec<Edge> {
    crate::drift::edges_from_declarations(crate::builtin_lock::declarations()).expect(
        "src/builtin_lock.toml is compiled-in data produced by this crate's own emit — \
         a malformed edge fact is a build-time bug, never a runtime condition",
    )
}

/// The built-in kind a bare `name` resolves to, or `None` if none carries it. Bare
/// name is the whole identity now, so this is a plain lookup.
pub fn definition(name: &str) -> Option<CustomKind> {
    let edges = builtin_edges();
    crate::builtin_lock::declarations()
        .kinds
        .iter()
        .find(|row| row.name == name)
        .map(|row| lift(row, &edges))
}

/// Every embedded built-in kind, keyed by its bare name — the compiled default
/// program's kind roster, lifted off the embedded lock's kind rows.
pub fn definitions() -> BTreeMap<String, CustomKind> {
    let edges = builtin_edges();
    crate::builtin_lock::declarations()
        .kinds
        .iter()
        .map(|row| (row.name.clone(), lift(row, &edges)))
        .collect()
}

/// The built-in kind `name` names, for a caller that names one the embedded lock always
/// carries.
fn shipped(name: &str) -> CustomKind {
    definition(name).expect("the embedded lock ships every kind this module names")
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
    features(&shipped("skill"), unit, &[])
}

/// Extract a built-in rule's [`Features`] the same way [`skill_features`] does — the
/// embedded `rule` kind's extraction over the rule's generically-loaded surface [`Unit`].
#[must_use]
pub fn rule_features(unit: &Unit) -> Features {
    features(&shipped("rule"), unit, &[])
}

/// Run a built-in `kind`'s embedded extraction over `unit`, fold every preserved
/// frontmatter key the composed primitives did not name into the feature map, and
/// resolve `unit`'s own nested members off `nested_members` — the lock's declared
/// [`NestedMemberRow`] family, matched by this member's own **address**
/// ([`crate::drift::nested_members_from_rows`]), read through
/// [`member_address::address_of`] so a nested **file** child whose id is already its whole
/// address still finds its rows. The **permissive extraction**: an unknown
/// key on a known artifact is already extracted, so a clause (a `forbidden-keys`) can
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
        &member_address::address_of(&kind.name, &unit.id),
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

/// One documented tap event paired with the `settings.json` registration its tap hook
/// takes: the `hooks.<Event>` key it registers under, and the matcher scoping the fire.
pub struct TapHookBinding {
    /// The author-facing event name a telemetry verifier declares — the token
    /// `crate::tap::documented_name` spells for the variant.
    pub name: &'static str,
    /// The `hooks.<Event>` lifecycle key the tap hook registers under.
    pub event: &'static str,
    /// The matcher scoping the fire to the telemetry-relevant subset.
    pub matcher: &'static str,
}

/// The lifecycle event and matcher one [`TapEvent`]'s tap hook registers under — the
/// write-side twin of the payload shapes `classify_claude_code_hook_payload` reads below,
/// and the same external fact (code.claude.com/docs/en/hooks, retrieved 2026-08-26):
///
/// - `InstructionsLoaded` fires on a rule/memory load, under a lifecycle event of its own;
///   its matcher filters the load reason across every documented reason (`session_start`,
///   `nested_traversal`, `path_glob_match`, `include`, `compact`), so always-on members
///   that load at session start are recorded.
/// - `SkillInvoked` is a skill invocation, surfaced under `PostToolUse` with the tool-name
///   matcher `Skill`.
/// - `UserPromptExpansion` fires on a command expansion, under its own event; its matcher
///   filters the command name, `.*` capturing every one.
/// - `ToolUse` fires after any tool call, under `PostToolUse`; its matcher filters the tool
///   name, `.*` capturing every one.
///
/// An exhaustive match, so a new [`TapEvent`] variant cannot compile without declaring the
/// hook its tap fires under: the alternative is a variant the roster admits while the SDK
/// synthesizes no registration for it, exactly the silent no-op the admissibility finding
/// exists to refuse.
const fn tap_hook_binding(event: TapEvent) -> TapHookBinding {
    let (hook_event, matcher) = match event {
        TapEvent::InstructionsLoaded => ("InstructionsLoaded", ".*"),
        TapEvent::SkillInvoked => ("PostToolUse", "Skill"),
        TapEvent::UserPromptExpansion => ("UserPromptExpansion", ".*"),
        TapEvent::ToolUse => ("PostToolUse", ".*"),
    };
    TapHookBinding {
        name: crate::tap::documented_name(event),
        event: hook_event,
        matcher,
    }
}

/// Every documented tap event paired with the hook registration it takes, in [`TapEvent`]'s
/// own declaration order — the one home for that pairing.
///
/// The seam gate (`tests/it/seam_bindings_current.rs`) writes
/// `sdk/src/generated/tap-hooks.ts` from this table and the SDK's `TELEMETRY_EVENT_HOOKS`
/// reads that module, so the registration vocabulary cannot be respelled on one side of
/// the seam; [`crate::tap::documented_event_names`] reads the same table for the
/// admissibility set the roster judges a telemetry verifier's event names against. So the
/// set `check` admits and the set the SDK synthesizes a hook for are one enumeration.
pub const TAP_HOOK_BINDINGS: [TapHookBinding; 4] = [
    tap_hook_binding(TapEvent::InstructionsLoaded),
    tap_hook_binding(TapEvent::SkillInvoked),
    tap_hook_binding(TapEvent::UserPromptExpansion),
    tap_hook_binding(TapEvent::ToolUse),
];

/// Classify a Claude Code hook payload into its lifecycle event, identity, and the
/// optional discriminants an `InstructionsLoaded` load carries: its reason, the path
/// whose match triggered a lazy load, and the file an `include` load was pulled from.
///
/// The payload shapes are Claude Code's hook contract, an external fact:
/// code.claude.com/docs/en/hooks (retrieved 2026-07-17). InstructionsLoaded carries
/// {file_path, load_reason, content}; UserPromptExpansion {command_name, expanded_prompt};
/// PostToolUse {tool_name, tool_input, tool_response}; a skill invocation rides PostToolUse
/// with tool_name="Skill" and the skill name under tool_input.skill. The prose fields —
/// content, expanded_prompt, tool_response — are never included in the returned classification.
///
/// An `InstructionsLoaded` payload also carries `trigger_file_path` on a lazy load and
/// `parent_file_path` on an `include` load, an external fact: code.claude.com/docs/en/hooks,
/// "InstructionsLoaded input" (retrieved 2026-09-24). Both are read; the same payload's
/// `globs` and `memory_type` are not (0062 — the lock already holds the first, and the
/// second follows from the path).
///
/// Returns `None` if the payload does not parse, names no recognized event, or lacks the
/// identity field its event needs.
#[must_use]
pub(crate) fn classify_claude_code_hook_payload(
    value: &JsonValue,
) -> Option<HookPayloadClassification> {
    let string = |key: &str| value.get(key).and_then(JsonValue::as_str);

    match string("hook_event_name")? {
        "InstructionsLoaded" => Some(HookPayloadClassification {
            event: TapEvent::InstructionsLoaded,
            identity: string("file_path")?.to_string(),
            reason: string("load_reason").map(str::to_string),
            trigger_path: string("trigger_file_path").map(str::to_string),
            parent_path: string("parent_file_path").map(str::to_string),
        }),
        "UserPromptExpansion" => Some(HookPayloadClassification::bare(
            TapEvent::UserPromptExpansion,
            string("command_name")?,
        )),
        "PostToolUse" => {
            let tool = string("tool_name")?;
            if tool == "Skill" {
                let skill = value
                    .get("tool_input")
                    .and_then(|input| input.get("skill"))
                    .and_then(JsonValue::as_str)?;
                Some(HookPayloadClassification::bare(
                    TapEvent::SkillInvoked,
                    skill,
                ))
            } else {
                Some(HookPayloadClassification::bare(TapEvent::ToolUse, tool))
            }
        }
        _ => None,
    }
}

/// What [`classify_claude_code_hook_payload`] reads off one hook payload: the lifecycle
/// event, the identity it names, and the discriminants only an `InstructionsLoaded` load
/// carries. Named rather than a wide tuple so each path is unmistakable at the one call
/// site that relativizes them ([`crate::tap::record_from_payload`]).
pub(crate) struct HookPayloadClassification {
    /// Which lifecycle event the payload named.
    pub(crate) event: TapEvent,
    /// The member, path, skill, command or tool name the event names.
    pub(crate) identity: String,
    /// The load reason an `InstructionsLoaded` payload carries.
    pub(crate) reason: Option<String>,
    /// The path whose match triggered a lazy load.
    pub(crate) trigger_path: Option<String>,
    /// The file an `include` load was pulled from.
    pub(crate) parent_path: Option<String>,
}

impl HookPayloadClassification {
    /// An event carrying identity alone — every event but `InstructionsLoaded`, none of
    /// which has a reason or a path to relativize.
    fn bare(event: TapEvent, identity: &str) -> Self {
        Self {
            event,
            identity: identity.to_string(),
            reason: None,
            trigger_path: None,
            parent_path: None,
        }
    }
}

/// Extract the session ID from a Claude Code hook payload.
///
/// The session_id field is part of Claude Code's hook contract, an external fact:
/// code.claude.com/docs/en/hooks (retrieved 2026-07-17).
///
/// Returns the session ID as a string, or an empty string if the field is absent or invalid.
#[must_use]
pub(crate) fn hook_payload_session_id(value: &JsonValue) -> String {
    value
        .get("session_id")
        .and_then(JsonValue::as_str)
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::Edge;
    use crate::kind::{Governs, Registration, Template};
    use crate::test_support::{surface_unit, tmpdir};

    #[test]
    fn every_tap_event_binds_a_non_empty_hook_registration() {
        // Vacuity pin: the table is the whole `TapEvent` enumeration, and the admissibility
        // set `check` judges against is read off these same rows.
        assert_eq!(
            TAP_HOOK_BINDINGS.len(),
            crate::tap::documented_event_names().len()
        );
        assert!(!TAP_HOOK_BINDINGS.is_empty());
        for TapHookBinding {
            name,
            event,
            matcher,
        } in &TAP_HOOK_BINDINGS
        {
            assert!(!name.is_empty());
            assert!(!event.is_empty(), "`{name}` binds an empty event");
            assert!(!matcher.is_empty(), "`{name}` binds an empty matcher");
            assert!(crate::tap::documented_event_names().contains(name));
        }
    }

    #[test]
    fn skill_definition_matches_the_hand_authored_kind() {
        let skill = definition("skill").expect("skill is embedded");

        assert_eq!(skill.name, "skill");
        assert_eq!(
            skill.governs,
            Some(Governs {
                root: ".".to_string(),
                globs: vec!["**/.claude/skills/*/SKILL.md".to_string()],
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
                globs: vec!["**/*.md".to_string()],
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
                "handler",
                "hook",
                "installed-plugin",
                "known-marketplace",
                "marketplace",
                "mcp",
                "mcp-server",
                "memory",
                "output-style",
                "plugin-manifest",
                "requirement",
                "rule",
                "settings",
                "settings-local",
                "skill",
                "supporting-doc",
                "tool"
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
                globs: vec!["**/*.md".to_string()],
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
                globs: vec!["**/*.md".to_string()],
            })
        );
        // The skill's field schema, reused verbatim — command is a second placement of
        // the skill surface, not a second schema.
        assert_eq!(
            command.extraction.primitives(),
            definition("skill").unwrap().extraction.primitives()
        );
        assert_eq!(command.relationships, Vec::<Edge>::new());
        // Path-shaped — identity is the path under `commands/`, each `/` a `:`
        // (`frontend:component`), no `name` field required for identity.
        assert_eq!(command.unit_shape, Some(crate::kind::UnitShape::Path));
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
                globs: vec!["settings.json".to_string()],
            })
        );
        // Fields-only: no body slot, distinct from every file-content built-in.
        assert_eq!(hook.content, Content::Fields);
        // The manifest fence: which manifest, which key path its registration keys at.
        assert_eq!(
            hook.collection_address,
            Some(CollectionAddress {
                key_path: CollectionKeyPath::HooksEvent,
                entry_shape: crate::kind::EntryShape::GroupArray {
                    member_key: "hooks".to_string(),
                    lifted_fields: vec!["matcher".to_string()],
                },
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
                globs: vec![".mcp.json".to_string()],
            })
        );
        assert_eq!(mcp.content, Content::Fields);
        assert_eq!(
            mcp.collection_address,
            Some(CollectionAddress {
                key_path: CollectionKeyPath::McpServers,
                entry_shape: crate::kind::EntryShape::Object,
            })
        );
        // Registers on the `connection` channel — the harness connects to it, a runtime
        // fact no repo criterion decides dead.
        assert_eq!(mcp.registration, vec![Registration::Connection]);
        // `mcpServers.*` names no key field, so a server carries only its own object
        // fields, folded in at read time — no declared frontmatter primitives.
        assert_eq!(mcp.extraction.primitives(), &[]);
    }

    #[test]
    fn handler_definition_is_a_fields_only_embedded_kind_hook_templates() {
        use crate::kind::{Content, Template};

        let handler = definition("handler").expect("handler is embedded");

        assert_eq!(handler.name, "handler");
        // The host-composed locus: a handler lives inside its host hook member, so nothing
        // discovers it at a glob and it owns no surface subdirectory of its own.
        assert_eq!(handler.governs, None);
        assert_eq!(handler.surface_subdir(), None);
        // A lone entry in its host's `hooks` array — fields-only, no body slot, and no
        // artifact format: it is a JSON object inside the host's manifest, never a document.
        assert_eq!(handler.content, Content::Fields);
        assert_eq!(handler.format, None);
        assert_eq!(handler.unit_shape, Some(crate::kind::UnitShape::File));
        // Channel-less: it reaches the world only through the hook whose event fires it,
        // and it is no collection address of its own — the group's `hooks` key is its host's.
        assert_eq!(handler.registration, Vec::new());
        assert_eq!(handler.collection_address, None);
        // No declared frontmatter primitives: its fields fold in off the host's group array.
        assert_eq!(handler.extraction.primitives(), &[]);

        // And the path fact is the host's. `hook` templates it with no `path` — the
        // spelling that parts an embedded layer from `skill`'s file-child one.
        let hook = definition("hook").expect("hook is embedded");
        assert_eq!(
            hook.templates,
            vec![Template {
                kind: "handler".to_string(),
                path: None,
            }]
        );
        let skill = definition("skill").expect("skill is embedded");
        assert_eq!(
            skill.templates.first().map(|t| t.path.as_deref()),
            Some(Some("*.md"))
        );
    }

    #[test]
    fn the_mcp_definition_is_a_committed_json_document_governing_the_whole_manifest() {
        let mcp = definition("mcp").expect("mcp is embedded");

        assert_eq!(mcp.name, "mcp");
        // The whole file at the documented root path, one JSON document
        // (code.claude.com/docs/en/mcp, retrieved 2026-09-29).
        assert_eq!(
            mcp.governs,
            Some(Governs {
                root: ".".to_string(),
                globs: vec![".mcp.json".to_string()],
            })
        );
        assert_eq!(mcp.format, Some(Format::JsonDocument));
        // Singleton identity: the file itself, so no declared key names it.
        assert_eq!(mcp.unit_shape, Some(crate::kind::UnitShape::File));
        // Committed, unlike `settings-local`/`dial` — no declared class is the committed
        // one, so emit writes the file and its rows land in the lock.
        assert_eq!(mcp.commitment, None);
        // Channel-less: the harness reads the manifest, it never reaches the model. The
        // connections do, as `mcp-server` members.
        assert!(mcp.registration.is_empty());
        // The container of `mcpServers`, never a member at it: the address belongs to
        // `mcp-server` alone, so the container declares none.
        assert_eq!(mcp.collection_address, None);
        // Every key is a field of the member; nothing is lifted by a declared primitive.
        assert_eq!(mcp.extraction.primitives(), &[]);
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
        // `forbidden-keys` clause can range over a project convention on a known artifact.
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
