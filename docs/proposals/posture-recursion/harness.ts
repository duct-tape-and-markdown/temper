// The assembly. Admission is the Element B gap made visible: each built-in
// kind declares which house postures its body may carry. Semantics bind per
// posture type — budgets on push, degrees on edges — and the requirements
// are unchanged from the live program: the posture model changes how bodies
// are written, not what the harness owes.
import { clause, count, degree, emit, harness, maxLines, requirement, uniqueName } from "@dtmd/temper";
import {
  memory,
  memoryAnthropicDefaultContract,
  rule,
  ruleDefaultContract,
  skill,
  skillDefaultContract,
} from "@dtmd/temper/claude-code";
import { consult, directive, harnessMetaDoc, orientation, platformDoc, reference, step } from "./kinds.ts";
import { memory_CLAUDE } from "./memory.ts";
import {
  rule_cls,
  rule_connect_asp,
  rule_connect_optimus,
  rule_connectdb,
  rule_csharp,
  rule_feature_flags,
  rule_frontend,
  rule_omegaone,
  rule_protocol,
  rule_routing,
  rule_web_ui,
} from "./rules.ts";
import { doc_economy, doc_reference, skill_check_logs, skill_harness_meta, skill_platform } from "./skills.ts";
import { standards } from "./standards.ts";

const program = harness({
  require: {
    orientation: requirement({
      prose: "Repo orientation every session opens with — the platform map and the operating contract (CLAUDE.md).",
      kind: memory,
      required: true,
      clauses: [clause(count({ min: 1, max: 1 }), { severity: "required" })],
    }),
    "always-on-directives": requirement({
      prose: "The thin always-on tier — universal operating directives only; the single home of the source-authority doctrine.",
      kind: rule,
      required: true,
      clauses: [clause(count({ min: 1, max: 1 }), { severity: "advisory" })],
    }),
    "area-directives": requirement({
      prose: "Path-scoped directives firing deterministically on file surfaces; detail behind consult edges.",
      kind: rule,
      required: true,
    }),
    "knowledge-routing": requirement({
      prose: "Behavioral knowledge routes to runner, structure to cartograph; descriptive content never enters the harness.",
      required: true,
    }),
    "dev-standards": requirement({
      prose: "Code-convention references, paths-gated to the trees that write them; every one reached from the firing tier.",
      kind: skill,
      required: true,
      clauses: [clause(degree({ incoming: { min: 1 } }), { severity: "required" })],
    }),
    "operational-standards": requirement({
      prose: "Deploy/restart policy, task-shaped; the platform skill executes.",
      kind: skill,
      required: true,
      clauses: [clause(degree({ incoming: { min: 1 } }), { severity: "advisory" })],
    }),
    "product-standards": requirement({
      prose: "Product policy references, task-shaped; a binding product invariant graduates to a tenet.",
      kind: skill,
      required: true,
      clauses: [clause(degree({ incoming: { min: 1 } }), { severity: "advisory" })],
    }),
    "ops-execution": requirement({ prose: "Build/publish/deploy/services/IIS execution.", kind: skill }),
    "ops-diagnostics": requirement({ prose: "Application-log diagnostics on the remote server.", kind: skill }),
    "harness-governance": requirement({
      prose: "Harness changes route through the intake procedure; the gate holds the structure.",
      kind: skill,
      required: true,
    }),
  },
  members: [
    memory_CLAUDE,
    rule_cls,
    rule_connect_asp,
    rule_connect_optimus,
    rule_connectdb,
    rule_csharp,
    rule_feature_flags,
    rule_frontend,
    rule_omegaone,
    rule_protocol,
    rule_routing,
    rule_web_ui,
    skill_check_logs,
    skill_harness_meta,
    doc_economy,
    skill_platform,
    doc_reference,
    ...standards,
  ],
  expect: [
    // ── admission: the Element B gap — built-in bodies range over house postures ──
    { kind: memory, content: [orientation, directive, consult, reference] },
    { kind: rule, content: [orientation, directive, consult, reference] },
    { kind: skill, content: [orientation, directive, consult, reference, step] },

    // ── built-in contracts, unchanged in spirit from the live program ──
    {
      kind: memory,
      clauses: [
        ...memoryAnthropicDefaultContract,
        clause(maxLines(70), { severity: "advisory" }),
      ],
    },
    {
      kind: rule,
      clauses: [
        ...ruleDefaultContract,
        clause(uniqueName(), { severity: "required" }),
        clause(count({ of: directive, max: 5 }), {
          severity: "advisory",
          guidance: "a rule is a handful of invariants, not an essay — past five directives, something is a convention that belongs in a standards skill.",
        }),
      ],
    },
    {
      kind: skill,
      clauses: [...skillDefaultContract, clause(uniqueName(), { severity: "required" })],
    },

    // ── posture semantics: budgets on push, teeth on edges ──
    { kind: orientation, clauses: [clause(maxLines(12), { severity: "advisory" })] },
    { kind: directive, clauses: [clause(maxLines(4), { severity: "advisory" })] },
    { kind: step, clauses: [clause(maxLines(3), { severity: "advisory" })] },
    { kind: consult, clauses: [] },   // in play; resolution and degree are the gate's
    { kind: reference, clauses: [] },

    // ── supporting docs: reached through their skill or dead ──
    ...[harnessMetaDoc, platformDoc].map((docKind) => ({
      kind: docKind,
      clauses: [
        clause(uniqueName(), { severity: "required" as const }),
        clause(degree({ incoming: { min: 1 } }), { severity: "required" as const }),
      ],
    })),
  ],
});

process.stdout.write(emit(program).seam);
