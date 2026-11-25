import { emit, harness, requirement, telemetry } from "@dtmd/temper";
import { settings } from "@dtmd/temper/claude-code";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_collaboration } from "./rules/collaboration.ts";
import { rule_publicProse } from "./rules/public-prose.ts";
import { rule_release } from "./rules/release.ts";
import { rule_rust } from "./rules/rust.ts";
import { rule_sdk } from "./rules/sdk.ts";
import { skill_captureFriction } from "./skills/capture-friction.ts";
import {
  hook_fmtOnWrite,
  hook_guard,
  hook_postToolUseBash,
  hook_sessionStart,
} from "./hooks.ts";

// The requirements below are load-bearing, not documentation: emit
// refuses if any loses its satisfier (sdk/test/refusals.test.ts, "an
// unfilled required requirement"). The flume pipeline's own discipline
// ships with its harness package (.flume/declaration.ts); this harness
// carries only what the package leaves to the project.
//
// friction-capture-procedure carries no `kind:` constraint: Requirement.kind
// is KindDefinition<object>, and skill's own Skill.description is required,
// so KindDefinition<Skill> fails that assignment on real variance grounds
// (a KindDefinition<object> slot must be callable with no T fields at all).
// Only all-optional-field kinds (rule, memory) can fill `kind:` today.
// The committed `.claude/settings.json` as one member (0050): every key is
// authored here and the file renders whole from it. The permission allowlist
// stays authored until members declare capability needs and the derived union
// takes over (specs/model/pipeline.md, "Emit").
const settings_project = settings({
  name: "settings",
  autoMemoryEnabled: false,
  permissions: {
    allow: [
      "Bash(cargo build:*)",
      "Bash(cargo test:*)",
      "Bash(cargo clippy:*)",
      "Bash(cargo fmt:*)",
    ],
  },
  // Undocumented in the settings reference (retrieved 2026-09-22): opaque,
  // named residue rather than a typed key.
  residue: { worktree: { bgIsolation: "none" } },
});

const program = harness({
  // The write-boundary guard blocks: a hand-edit to a managed projection is
  // denied (exit 2), not merely surfaced. `warn`'s PreToolUse exit-0 stdout is
  // not read back to the model, so an advisory nudge is inert for an agent —
  // only a deny reaches it (specs/distribution.md, "Per tool call").
  mode: "block",
  require: {
    "friction-capture-procedure": {
      prose: "an agent that hits harness friction or touches structural debt it can't fix now needs a description-triggered procedure (a skill) for filing the capture",
    },
    // The behavioral remainder the gate cannot decide (intent.md invariant 2):
    // whether a declared member ever reaches the model. Its verifier is the
    // tap — declaring it here is what synthesizes the tap hook registrations
    // into settings.json (pipeline.md, "Telemetry"), so this requirement is
    // also the harness's only tap wiring. Two events, not four: this harness
    // has no command members for UserPromptExpansion to see, and ToolUse
    // records every call for a question about members that only
    // InstructionsLoaded and SkillInvoked answer.
    "context-arrives": requirement({
      prose:
        "every always-on member this harness declares should actually reach the model, and every skill should be reachable by its description — the failure intent.md names (a rule that fails to load, a skill that never triggers), judged by reading the field record, never by check",
      verifier: telemetry(["InstructionsLoaded", "SkillInvoked"]),
      // Advisory by design: the field record judges it, so nothing satisfies
      // it and the shipped fill default would refuse emit.
      required: false,
    }),
  },
  members: [
    memory_CLAUDE,
    rule_collaboration,
    rule_publicProse,
    rule_release,
    rule_rust,
    rule_sdk,
    skill_captureFriction,
    hook_sessionStart,
    hook_guard,
    hook_postToolUseBash,
    hook_fmtOnWrite,
    settings_project,
  ],
});

process.stdout.write(emit(program).seam);
