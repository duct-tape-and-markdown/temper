//! The embedded built-in package std-lib.
//!
//! `temper` ships a set of first-party packages — the curated Anthropic contracts
//! for the built-in artifact kinds (`skill.anthropic`, `rule.anthropic`, the two
//! `memory.*` providers). There is no `PACKAGE.md` file format any more
//! (`specs/architecture/15-kinds.md`, "Decision: field typing lives in the SDK —
//! there is no kind file format"; the same Decision retires a package file): each
//! built-in is authored directly as Rust data below — the compiled default program
//! the engine carries for SDK-less checking (`specs/architecture/50-distribution.md`).
//! The curated `packages/<name>/PACKAGE.md` product tree (human-maintained, cited
//! product source) is the authoring reference this data mirrors; migrating that
//! tree onto a future SDK path is out of this module's scope.

use std::collections::{BTreeMap, BTreeSet};

use crate::contract::{Charset, Clause, Contract, ContractError, Predicate, Severity};

/// The built-in package temper ships as the floor for the `claude-code.skill` kind —
/// Anthropic's documented skill contract (`specs/architecture/10-contracts.md`, "named for its
/// source"). temper's own **published** binding names the *qualified* kind identity
/// (`specs/architecture/15-kinds.md`, "a published package binds a qualified kind name"): publication
/// is where the consumer's assembly is unknowable, so a bare binding would be a latent
/// collision. The package's own name stays short — the kind axis it binds is what
/// qualifies, resolved through the embedded set (`crate::builtin_kind::qualified`).
pub const SKILL_PACKAGE: &str = "skill.anthropic";

/// The built-in package temper ships as the floor for the `claude-code.rule` kind —
/// Anthropic's documented rule contract, bound to the qualified kind identity exactly
/// as [`SKILL_PACKAGE`] is (`specs/architecture/15-kinds.md`). Renamed from the bare `rule`
/// (`specs/architecture/10-contracts.md`, "named for its source": the clauses are equally
/// Anthropic-sourced).
pub const RULE_PACKAGE: &str = "rule.anthropic";

/// The built-in package temper ships as the floor for the `claude-code.memory` kind —
/// the documented `CLAUDE.md` contract (`specs/architecture/10-contracts.md`, "named for its
/// source"). Bound to the **qualified** kind identity in [`QUALIFIED_FLOOR_BINDINGS`]:
/// the bare `memory` name is ambiguous by design — two providers carry it (86d5b70) —
/// so unlike skill/rule it can never resolve through the bare→qualified path, and the
/// floor binds `claude-code.memory` directly.
pub const MEMORY_ANTHROPIC_PACKAGE: &str = "memory.anthropic";

/// The built-in package temper ships as the floor for the `agents-md.memory` kind — the
/// `AGENTS.md` contract, bound to its qualified identity exactly as
/// [`MEMORY_ANTHROPIC_PACKAGE`] is. The AGENTS.md standard constrains almost nothing, so
/// this package is guidance-only (zero clauses); the binding still routes a discovered
/// `AGENTS.md` member here rather than to Anthropic's `CLAUDE.md` floor.
pub const MEMORY_AGENTS_MD_PACKAGE: &str = "memory.agents-md";

/// Each embedded built-in kind's floor package, keyed by the kind's **qualified**
/// identity `<provider>.<name>` — the binding the `check` gate's per-kind loop resolves
/// a discovered member's package through (`specs/architecture/20-surface.md`, "Artifact kinds &
/// package binding"). Qualified, never bare: the two `memory` providers collide on the
/// bare name by design (86d5b70), so a bare key would be ambiguous; skill/rule qualify
/// the same way for one uniform table. This is temper's own **published** binding — it
/// names the qualified kind a consumer's assembly can never mistake for another
/// provider's (`specs/architecture/15-kinds.md`, "a published package binds a qualified kind name").
pub const QUALIFIED_FLOOR_BINDINGS: &[(&str, &str)] = &[
    ("claude-code.skill", SKILL_PACKAGE),
    ("claude-code.rule", RULE_PACKAGE),
    ("claude-code.memory", MEMORY_ANTHROPIC_PACKAGE),
    ("agents-md.memory", MEMORY_AGENTS_MD_PACKAGE),
];

/// The floor package bound to a built-in kind's **qualified** identity
/// (`claude-code.memory` → `memory.anthropic`), or `None` if no embedded kind of that
/// identity ships a floor. The gate's per-kind loop looks each discovered kind's floor
/// up here by [`qualified_name`](crate::kind::CustomKind::qualified_name)
/// ([`QUALIFIED_FLOOR_BINDINGS`]).
#[must_use]
pub fn floor_package(qualified: &str) -> Option<&'static str> {
    QUALIFIED_FLOOR_BINDINGS
        .iter()
        .find(|(id, _)| *id == qualified)
        .map(|(_, package)| *package)
}

/// A convenience constructor for a field-targeted clause carrying both docs channels —
/// most of the built-in vocabulary below pairs a predicate with `guidance` and
/// `source`, so this keeps each entry to its decidable content.
fn clause(severity: Severity, predicate: Predicate, guidance: &str, source: &str) -> Clause {
    Clause {
        severity,
        predicate,
        guidance: Some(guidance.to_string()),
        source: Some(source.to_string()),
    }
}

/// Anthropic's documented contract for a skill, as decidable clauses — sourced from
/// the Agent Skills open standard (agentskills.io), Anthropic's platform validation,
/// and Claude Code's docs (`packages/skill.anthropic/PACKAGE.md`, the curated
/// authoring reference this mirrors). All sources retrieved 2026-07-01.
fn skill_anthropic() -> Contract {
    Contract {
        name: SKILL_PACKAGE.to_string(),
        clauses: vec![
            clause(
                Severity::Required,
                Predicate::Required {
                    field: "name".to_string(),
                },
                "Every skill declares a `name` — the slug the harness binds to. Claude Code alone would default it from the directory name, but a nameless skill is not portable: the spec and Anthropic's upload validation both require it.",
                "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::MinLen {
                    field: "name".to_string(),
                    min: 1,
                },
                "A present-but-empty name fails the spec's 1-64 character bound.",
                "https://agentskills.io/specification#name-field (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::AllowedChars {
                    field: "name".to_string(),
                    charset: Charset {
                        ranges: vec![('a', 'z'), ('0', '9')],
                        chars: BTreeSet::from(['-']),
                    },
                },
                "Lowercase letters, digits, and hyphens only — `PDF-Processing` is the spec's own counter-example. The charset also keeps XML out of the name, which Anthropic's upload validation separately forbids.",
                "https://agentskills.io/specification#name-field (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::MaxLen {
                    field: "name".to_string(),
                    max: 64,
                },
                "Keep the name short and slug-like; it becomes a directory and an id.",
                "https://agentskills.io/specification#name-field (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::Deny {
                    field: "name".to_string(),
                    values: vec!["anthropic".to_string(), "claude".to_string()],
                },
                "Reserved words, enforced by Anthropic's platform upload validation (not by the open spec, and not by Claude Code's runtime — which itself ships a `claude-api` skill). Keep them out if the skill will ever travel through the API or claude.ai.",
                "https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview#skill-structure (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::NameMatchesDir,
                "The spec requires the name to match its parent directory. Claude Code decouples the two (the frontmatter name is a display label; the directory names the slash command, except for a plugin-root SKILL.md) — but a mismatch is a portability trap and a reader trap even where it loads.",
                "https://agentskills.io/specification#name-field (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::Required {
                    field: "description".to_string(),
                },
                "The description is how the model chooses this skill from potentially 100+ available — it is the skill's API. Claude Code would fall back to the body's first paragraph; the spec and upload validation require it declared.",
                "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::MinLen {
                    field: "description".to_string(),
                    min: 1,
                },
                "Say both what the skill does and when to use it, with the keywords a user would naturally say. Write in third person — the text is injected into the system prompt, and inconsistent point-of-view causes discovery problems.",
                "https://agentskills.io/specification#description-field (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::MaxLen {
                    field: "description".to_string(),
                    max: 1024,
                },
                "The spec's cap. Claude Code additionally truncates the skill listing at 1,536 combined characters (description + when_to_use) — truncation, not rejection, but text past the fold cannot help the model choose.",
                "https://agentskills.io/specification#description-field (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::MaxLen {
                    field: "compatibility".to_string(),
                    max: 500,
                },
                "Optional field; when present the spec caps it at 500 characters. Most skills do not need it.",
                "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Advisory,
                Predicate::MaxLines { max: 500 },
                "Progressive disclosure: keep SKILL.md under 500 lines and move detailed reference material to separate files, one level deep. Once a skill loads, its body stays in context across turns — every line is a recurring token cost. The context window is a public good.",
                "https://agentskills.io/specification#progressive-disclosure (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::ForbiddenKeys {
                    keys: vec!["globs".to_string(), "alwaysApply".to_string()],
                },
                "Cursor `.mdc` keys. Nothing in the Agent Skills spec or Claude Code's documented frontmatter accepts them — a skill authored with them is carrying dead configuration that another tool's semantics silently fail to apply.",
                "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)",
            ),
        ],
        guidance: Some(SKILL_ANTHROPIC_GUIDANCE.to_string()),
    }
}

/// `skill.anthropic`'s package-level guidance — the always-on prose the clauses
/// cannot encode, carried verbatim from `packages/skill.anthropic/PACKAGE.md`.
const SKILL_ANTHROPIC_GUIDANCE: &str = "\
Anthropic's documented contract for a skill, as decidable clauses — sourced from\n\
the Agent Skills open standard (agentskills.io), Anthropic's platform validation,\n\
and Claude Code's docs. Adopt it, extend it, fork it, or ignore it: it is data,\n\
never a hardcoded check.\n\
\n\
**Enforcement profiles differ, and this package checks the strictest documented\n\
one.** The spec and Anthropic's upload validation are hard; Claude Code's runtime\n\
is deliberately forgiving (\"All fields are optional\"). The clauses hold the\n\
portable contract — a skill that passes here loads everywhere the format is\n\
honored, not merely on the machine it was written on. Where the runtime diverges,\n\
the clause's guidance says so.\n\
\n\
**Deliberately absent — undecidable, so never gate clauses** (they live here as\n\
guidance instead): whether the description *actually* triggers well or reads\n\
third-person (semantic); vagueness / no-op detection (semantic); gerund naming\n\
(judgment). Two *decidable* spec rules are also absent, pending a vocabulary\n\
addition (a narrow shape predicate): the name must not start/end with a hyphen\n\
and must not contain consecutive hyphens; likewise the platform's \"no XML tags\n\
in the description.\"\n\
\n\
Authoring notes the clauses cannot carry: prefer gerund or noun-phrase names\n\
(`processing-pdfs`, `pdf-processing`) over vague ones (`helper`, `utils`);\n\
`disable-model-invocation: true` for side-effectful workflows you want to time\n\
yourself; `user-invocable: false` for background knowledge that is not a command;\n\
`metadata` is the sanctioned home for versioning — there is no top-level\n\
`version` field.";

/// Anthropic's documented contract for a Claude Code rules file, as decidable
/// clauses — sourced from the memory docs (`packages/rule.anthropic/PACKAGE.md`).
/// All sources retrieved 2026-07-01.
fn rule_anthropic() -> Contract {
    Contract {
        name: RULE_PACKAGE.to_string(),
        clauses: vec![
            clause(
                Severity::Advisory,
                Predicate::Optional {
                    field: "paths".to_string(),
                },
                "`paths` is the one documented frontmatter key for rules: glob patterns (brace expansion supported) that scope the rule to matching files. Rules without it load at launch with the same priority as CLAUDE.md; path-scoped rules load when Claude reads a matching file. Note skills now take a `paths` key too — the two schemas are separate.",
                "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Required,
                Predicate::ForbiddenKeys {
                    keys: vec![
                        "description".to_string(),
                        "globs".to_string(),
                        "alwaysApply".to_string(),
                    ],
                },
                "Cursor `.mdc` keys. Claude Code's documented rules schema is `paths`-only; a rule authored with Cursor frontmatter is configuration another tool's semantics silently fail to honor — the rule loads, the scoping you meant does not. (That Claude Code ignores unknown keys is observed behavior, not documented contract — the documented schema is the citation.)",
                "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-01)",
            ),
            clause(
                Severity::Advisory,
                Predicate::MaxLines { max: 200 },
                "Unconditional rules are always-on context, paid every session: the docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' (Distinct from the hard 200-line/25KB cutoff, which applies only to auto-memory MEMORY.md; rules load in full regardless of length.) For each line ask: would removing it cause Claude to make mistakes? If not, cut it.",
                "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-01)",
            ),
        ],
        guidance: Some(RULE_ANTHROPIC_GUIDANCE.to_string()),
    }
}

/// `rule.anthropic`'s package-level guidance, carried verbatim from
/// `packages/rule.anthropic/PACKAGE.md`.
const RULE_ANTHROPIC_GUIDANCE: &str = "\
Anthropic's documented contract for a Claude Code rules file, as decidable\n\
clauses — sourced from the memory docs (`.claude/rules/` landed in v2.0.64).\n\
Adopt, extend, fork, or ignore: data, never a hardcoded check.\n\
\n\
What the clauses cannot carry, as guidance: keep a rule to facts Claude should\n\
hold whenever the rule is in scope — concrete enough to verify (\"use 2-space\n\
indentation\", not \"format code properly\"). If an entry is a multi-step procedure\n\
or only matters occasionally, it belongs in a skill (on-demand) rather than a\n\
rule (always-on). Prefer path-scoped rules when one convention governs scattered\n\
paths; prefer per-directory CLAUDE.md when directory owners maintain their own.\n\
Treat rules like code: prune them when behavior drifts, and test a change by\n\
watching whether Claude's behavior actually shifts.";

/// Anthropic's documented contract for a project `CLAUDE.md`, as decidable clauses
/// (`packages/memory.anthropic/PACKAGE.md`). Binds the qualified kind
/// `claude-code.memory` — a bare `memory` binding collides with `agents-md.memory`.
/// All sources retrieved 2026-07-02.
fn memory_anthropic() -> Contract {
    Contract {
        name: MEMORY_ANTHROPIC_PACKAGE.to_string(),
        clauses: vec![clause(
            Severity::Advisory,
            Predicate::MaxLines { max: 200 },
            "CLAUDE.md is always-on context, paid every session. The memory docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' For each line ask: would removing it cause Claude to make mistakes? If not, cut it. (Advisory: Claude Code loads the file in full regardless of length; this is a context-cost budget, not a hard cutoff.)",
            "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-02)",
        )],
        guidance: Some(MEMORY_ANTHROPIC_GUIDANCE.to_string()),
    }
}

/// `memory.anthropic`'s package-level guidance, carried verbatim from
/// `packages/memory.anthropic/PACKAGE.md`.
const MEMORY_ANTHROPIC_GUIDANCE: &str = "\
Anthropic's documented contract for a project `CLAUDE.md`, as decidable clauses —\n\
sourced from the Claude Code memory docs. Adopt, extend, fork, or ignore: data,\n\
never a hardcoded check.\n\
\n\
**Deliberately near-empty, because the format is.** `CLAUDE.md` is plain markdown\n\
with no documented frontmatter and no required fields\n\
(https://code.claude.com/docs/en/memory, retrieved 2026-07-02), so there is no\n\
schema to gate — manufacturing a required field or a forbidden-key list would\n\
fake a check the format does not carry (`specs/intent/00-intent.md`, law 3:\n\
decidable clauses only — a gate that guesses cries wolf). The single clause is a\n\
context-cost budget; everything else the contract could say is guidance.\n\
\n\
What the clauses cannot carry, as guidance: a `paths:` frontmatter block belongs\n\
on a `.claude/rules/*.md` file, not on `CLAUDE.md` — the memory docs document\n\
`paths` only for rules, so a rules-style header on `CLAUDE.md` is dead\n\
configuration. Split a large file with `@path` imports (resolved relative to the\n\
importing file, absolute allowed, recursion capped at four hops; wrap a path in\n\
backticks to mention it without importing). If the repo already ships an\n\
`AGENTS.md` for other agents, don't duplicate it — create a `CLAUDE.md` that\n\
`@AGENTS.md`-imports it (or symlink, except on Windows where the import is the\n\
recommended bridge). Mind the loading asymmetry: every *ancestor* `CLAUDE.md`\n\
loads in full at launch, while files in *subdirectories* load only when Claude\n\
reads a file there — so a rule that must always hold belongs above the working\n\
directory, not below it. Personal, un-shared notes go in `CLAUDE.local.md`\n\
(gitignored), which is appended after `CLAUDE.md` at its level.";

/// The AGENTS.md standard's contract for a memory file — which is that there is
/// almost none (`packages/memory.agents-md/PACKAGE.md`). Zero clauses: the
/// standard defines no schema to gate. Binds the qualified kind
/// `agents-md.memory`. All sources retrieved 2026-07-02.
fn memory_agents_md() -> Contract {
    Contract {
        name: MEMORY_AGENTS_MD_PACKAGE.to_string(),
        clauses: Vec::new(),
        guidance: Some(MEMORY_AGENTS_MD_GUIDANCE.to_string()),
    }
}

/// `memory.agents-md`'s package-level guidance, carried verbatim from
/// `packages/memory.agents-md/PACKAGE.md`.
const MEMORY_AGENTS_MD_GUIDANCE: &str = "\
The AGENTS.md standard's contract for a memory file — which is that there is\n\
almost none. Adopt, extend, fork, or ignore: data, never a hardcoded check.\n\
\n\
**Guidance-only, and that is the honest encoding.** `AGENTS.md` \"is just standard\n\
Markdown\" with no required fields, no sections, and no frontmatter\n\
(https://agents.md/, retrieved 2026-07-02); the format deliberately constrains\n\
nothing. A package that manufactured a required field, a size gate, or a\n\
forbidden-key list would assert a contract the standard disclaims\n\
(`specs/intent/00-intent.md`, law 3: decidable clauses only — never fake a gate).\n\
So this package carries zero clauses and speaks only in guidance. Even the\n\
tempting size number is a *tool's* rule, not the format's.\n\
\n\
Real-world reading behavior worth knowing — none of it a clause you can honestly\n\
write over a single file: agents read the closest `AGENTS.md` in the tree\n\
(nested, nearest-wins); Codex concatenates the chain root-to-cwd and stops once\n\
combined size hits a byte budget, not a per-file line count; Gemini CLI reads\n\
`GEMINI.md` by default and only treats `AGENTS.md` as an alias when configured;\n\
Claude Code does not read `AGENTS.md` natively — bridge it with a `CLAUDE.md`\n\
that `@AGENTS.md`-imports it.";

/// The embedded built-in package a `name` resolves to, or `None` if none carries it.
fn by_name(name: &str) -> Option<Contract> {
    match name {
        SKILL_PACKAGE => Some(skill_anthropic()),
        RULE_PACKAGE => Some(rule_anthropic()),
        MEMORY_ANTHROPIC_PACKAGE => Some(memory_anthropic()),
        MEMORY_AGENTS_MD_PACKAGE => Some(memory_agents_md()),
        _ => None,
    }
}

/// Every embedded built-in package's name, sorted — the compiled default program's
/// package roster.
const BUILTIN_PACKAGE_NAMES: &[&str] = &[
    MEMORY_AGENTS_MD_PACKAGE,
    MEMORY_ANTHROPIC_PACKAGE,
    RULE_PACKAGE,
    SKILL_PACKAGE,
];

/// Resolve the named built-in package into its [`Contract`], or `None` if no package
/// of that name is embedded. Infallible today (the data is Rust, not a parsed
/// document) — the `Result` stays so callers that once handled a load failure need
/// no change.
///
/// # Errors
///
/// Never fails; the `Result` is kept for API stability.
pub fn contract(name: &str) -> Result<Option<Contract>, ContractError> {
    Ok(by_name(name))
}

/// Every embedded built-in package as a `name → Contract` map — the built-in set a
/// by-name package binding resolves against (`specs/architecture/20-surface.md`, "Decision:
/// package binding is by artifact kind"; the resolution order [`crate::compose::PackageResolver`]
/// runs).
///
/// # Errors
///
/// Never fails; the `Result` is kept for API stability.
pub fn contracts() -> Result<BTreeMap<String, Contract>, ContractError> {
    Ok(BUILTIN_PACKAGE_NAMES
        .iter()
        .map(|name| {
            (
                (*name).to_string(),
                by_name(name).expect("name drawn from BUILTIN_PACKAGE_NAMES resolves"),
            )
        })
        .collect())
}
