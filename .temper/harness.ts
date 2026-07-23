import { emit, harness } from "@dtmd/temper";
import { rule } from "@dtmd/temper/claude-code";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_collaboration } from "./rules/collaboration.ts";
import { rule_forkLifecycle } from "./rules/fork-lifecycle.ts";
import { rule_pendingEntry } from "./rules/pending-entry.ts";
import { rule_planState } from "./rules/plan-state.ts";
import { rule_postureSweep } from "./rules/posture-sweep.ts";
import { rule_publicProse } from "./rules/public-prose.ts";
import { rule_release } from "./rules/release.ts";
import { rule_rust } from "./rules/rust.ts";
import { rule_sdk } from "./rules/sdk.ts";
import { skill_captureFriction } from "./skills/capture-friction.ts";
import { hook_fmtOnWrite, hook_guard, hook_sessionStart } from "./hooks.ts";

// The requirements below are load-bearing, not documentation: emit
// refuses if any loses its satisfier (sdk/test/refusals.test.ts, "an
// unfilled required requirement"). The flume prompts are thin by design
// — doctrine lives in these paths-scoped rules, loading exactly when the
// governed file is touched (the pending-entry precedent, generalized
// 07-17); a prompt paragraph that duplicates a rule is drift. Prompt
// prose still names members by identifier outside any gate — that
// residual is cosmetic (no trigger mechanism reads the prose) and
// tracked on the ".flume/ is ungoverned by temper" record, not
// re-litigated here.
//
// friction-capture-procedure carries no `kind:` constraint: Requirement.kind
// is KindDefinition<object>, and skill's own Skill.description is required,
// so KindDefinition<Skill> fails that assignment on real variance grounds
// (a KindDefinition<object> slot must be callable with no T fields at all).
// Only all-optional-field kinds (rule, memory) can fill `kind:` today.
const program = harness({
  // The write-boundary guard blocks: a hand-edit to a managed projection is
  // denied (exit 2), not merely surfaced. `warn`'s PreToolUse exit-0 stdout is
  // not read back to the model, so an advisory nudge is inert for an agent —
  // only a deny reaches it (specs/distribution.md, "Per tool call").
  mode: "block",
  require: {
    "pending-entry-discipline": {
      prose: "flume's plan phase (and any interactive session) needs pending.json entry-filing constraints available as a rule scoped to .flume/plan/pending.json",
      kind: rule,
      required: true,
    },
    "friction-capture-procedure": {
      prose: "an agent that hits harness friction or touches structural debt it can't fix now needs a description-triggered procedure (a skill) for filing the capture",
      required: true,
    },
    "plan-state-discipline": {
      prose: "flume's plan phase needs state.md's schema, cursor copy-forward, and marker mechanics as a rule scoped to .flume/plan/state.md — the prompt stays thin",
      kind: rule,
      required: true,
    },
    "fork-lifecycle-discipline": {
      prose: "any actor touching open-questions.md needs the open-forks-only lifecycle (encode + delete, DATUMs to commit bodies, kept-on-purpose semantics) as a rule scoped to that file",
      kind: rule,
      required: true,
    },
    "posture-sweep-discipline": {
      prose: "the posture sweep needs its administering discipline (pages-as-authority, verified-on-disk, routing, rotation) as a rule scoped to the posture pages it reads",
      kind: rule,
      required: true,
    },
  },
  members: [
    memory_CLAUDE,
    rule_collaboration,
    rule_forkLifecycle,
    rule_pendingEntry,
    rule_planState,
    rule_postureSweep,
    rule_publicProse,
    rule_release,
    rule_rust,
    rule_sdk,
    skill_captureFriction,
    hook_sessionStart,
    hook_guard,
    hook_fmtOnWrite,
  ],
  // Residual settings with no member home yet. The permission allowlist is
  // authored residue until members declare capability needs and the derived
  // union takes over (specs/model/pipeline.md, "Emit" — derived facts are
  // computed, never authored twice; today nothing declares a need).
  settings: {
    autoMemoryEnabled: false,
    permissions: {
      allow: [
        "Bash(cargo build:*)",
        "Bash(cargo test:*)",
        "Bash(cargo clippy:*)",
        "Bash(cargo fmt:*)",
      ],
    },
    worktree: { bgIsolation: "none" },
  },
});

process.stdout.write(emit(program).seam);
