import { clause, count, emit, harness, maxLines, uniqueName } from "@dtmd/temper";
import { skill, skillDefaultContract } from "@dtmd/temper/claude-code";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_conduct } from "./rules/conduct.ts";
import { rule_docsDiscipline } from "./rules/docs-discipline.ts";
import { skill_growHarness } from "./skills/grow-harness.ts";
import { skill_verifySummary } from "./skills/verify-summary.ts";
import { hook_guard, hook_sessionStart } from "./hooks.ts";
import { alternative, decision, flow, glossary, invariant, source, stepKind, system } from "./kinds.ts";
import { system_scanner } from "./docs/systems/scanner.ts";
import { system_renderer } from "./docs/systems/renderer.ts";
import { flow_summarize } from "./docs/flows/summarize.ts";
import { decision_authorityArrow } from "./docs/decisions/authority-arrow.ts";
import { decision_perChangeDocDuty } from "./docs/decisions/per-change-doc-duty.ts";

const program = harness({
  require: {
    // The five domains — the whole skeleton, declared on day one. Growth is
    // additive: a new member names the domain it fills; no domain is ever
    // added later. Conduct, orientation, and governance are floored
    // (`required`) — no repo is exempt from how-to-behave, what-this-is,
    // and the-harness-maintains-itself. Standards and operations are
    // declared with exemplary satisfiers an adopter replaces: their content
    // is earned from the project's own history, and a required empty slot
    // would force filler.
    conduct: {
      prose:
        "how any agent behaves here — epistemics, gap handling, escalation — is declared, portable, and free of project facts",
      required: true,
    },
    orientation: {
      prose:
        "what this project is and where truth lives is stated ambiently — the map every session loads",
      required: true,
    },
    standards: {
      prose: "what correct code and change look like, scoped to the trees the conventions govern",
    },
    operations: {
      prose:
        "doing work to this project is a stated procedure — build, verify, diagnose live as invocable skills, never as chat folklore",
      kind: skill,
    },
    governance: {
      prose:
        "the harness maintains itself: the gate placements are wired, and growing the harness is itself a stated procedure",
      required: true,
    },
    "documented-spine": {
      prose:
        "every load-bearing area of this repository's behavior is documented as a system that declares spine membership",
      kind: system,
      required: true,
      clauses: [
        clause(count({ min: 2 }), {
          severity: "required",
          guidance:
            "the spine is plural by definition — a one-system corpus is an undocumented repository with one essay",
        }),
      ],
    },
  },
  members: [
    memory_CLAUDE,
    rule_conduct,
    rule_docsDiscipline,
    skill_verifySummary,
    skill_growHarness,
    system_scanner,
    system_renderer,
    flow_summarize,
    decision_authorityArrow,
    decision_perChangeDocDuty,
    glossary({ name: "glossary" }),
    hook_sessionStart,
    hook_guard,
  ],
  // Admission is the corpus's declaration, not the type's: an embedded kind
  // declares no host, and each host names what its composed body admits.
  admit: [
    { host: system, admits: [invariant] },
    { host: flow, admits: [stepKind] },
    { host: decision, admits: [alternative] },
  ],
  expect: [
    {
      kind: system,
      clauses: [
        clause(uniqueName(), { severity: "required" }),
        clause(maxLines(120), {
          severity: "advisory",
          guidance:
            "a system doc is navigational, not exhaustive — past ~120 lines, promote a section to a flow or split the system",
        }),
      ],
    },
    { kind: flow, clauses: [clause(uniqueName(), { severity: "required" })] },
    { kind: decision, clauses: [clause(uniqueName(), { severity: "required" })] },
    // The shipped default contract, adopted whole — adoption is a choice,
    // extension is a spread (`specs/builtins.md`, "Default contracts").
    { kind: skill, clauses: [...skillDefaultContract] },
    // In-play bindings, no clauses of their own: the embedded kinds so their
    // values are admitted nesting, and `source` so `implemented-by` has a
    // target kind to resolve within.
    { kind: invariant, clauses: [] },
    { kind: stepKind, clauses: [] },
    { kind: alternative, clauses: [] },
    { kind: source, clauses: [] },
  ],
});

process.stdout.write(emit(program).seam);
