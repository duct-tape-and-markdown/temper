import { emit, harness } from "@dtmd/temper";
import { rule } from "@dtmd/temper/claude-code";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_collaboration } from "./rules/collaboration.ts";
import { rule_pendingEntry } from "./rules/pending-entry.ts";
import { rule_rust } from "./rules/rust.ts";
import { rule_sdk } from "./rules/sdk.ts";
import { skill_captureFriction } from "./skills/capture-friction.ts";

// The two requirements below are load-bearing, not documentation: emit
// refuses if either loses its satisfier (sdk/test/refusals.test.ts, "an
// unfilled required requirement"). .flume/prompts/plan.md's/build.md's
// prose still names each member by identifier outside any gate — that
// residual is cosmetic (neither's trigger mechanism reads the prose) and
// tracked as evidence on the ".flume/ is ungoverned by temper" entry in
// .flume/plan/open-questions.md, not re-litigated here.
//
// friction-capture-procedure carries no `kind:` constraint: Requirement.kind
// is KindDefinition<object>, and skill's own Skill.description is required,
// so KindDefinition<Skill> fails that assignment on real variance grounds
// (a KindDefinition<object> slot must be callable with no T fields at all).
// Only all-optional-field kinds (rule, memory) can fill `kind:` today.
const program = harness({
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
  },
  members: [
    memory_CLAUDE,
    rule_collaboration,
    rule_pendingEntry,
    rule_rust,
    rule_sdk,
    skill_captureFriction,
  ],
});

process.stdout.write(emit(program).seam);
