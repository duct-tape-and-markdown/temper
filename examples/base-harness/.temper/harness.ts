import { clause, count, emit, extent, harness, input, uniqueName } from "@dtmd/temper";
import { settings, skill, skillDefaultContract } from "@dtmd/temper/claude-code";
import { starter } from "@dtmd/temper/starter";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_conduct } from "./rules/conduct.ts";
import { rule_docsDiscipline } from "./rules/docs-discipline.ts";
import { skill_growHarness } from "./skills/grow-harness.ts";
import { skill_verifySummary } from "./skills/verify-summary.ts";
import { alternative, decision, flow, glossary, invariant, stepKind, system } from "./kinds.ts";
import { system_scanner } from "./docs/systems/scanner.ts";
import { system_renderer } from "./docs/systems/renderer.ts";
import { flow_summarize } from "./docs/flows/summarize.ts";
import { decision_authorityArrow } from "./docs/decisions/authority-arrow.ts";
import { decision_perChangeDocDuty } from "./docs/decisions/per-change-doc-duty.ts";

// The starter supplies the mechanics of using temper well: the typecheck emit
// runs before the program (the workspace's own `typescript` over the project
// that checks every module here), the three gate hooks, and the emitted harness
// reference at `.claude/skills/temper/`. The project file is the one required
// argument, since the path is ours; everything else is a defaulted option.
//
// The worked customization: the reference's description is hydrated with this
// project's own trigger, and the reference declares the domain it fills. Its
// body and topics still derive from what this program declares.
const base = starter({
  typecheck: { project: input(import.meta.url, "./tsconfig.json") },
  reference: {
    description:
      "How to operate the temper gate over this repository's harness and documentation corpus. Use when a temper finding appears, before editing anything under .claude/ or docs/, or when adding a member to .temper/.",
    satisfies: ["governance"],
  },
});

const program = harness({
  ...base,
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
      required: false,
    },
    operations: {
      prose:
        "doing work to this project is a stated procedure — build, verify, diagnose live as invocable skills, never as chat folklore",
      kind: skill,
      required: false,
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
    ...base.members,
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
    // The committed `.claude/settings.json` as one member: it carries no keys
    // of its own, so the file renders whole from the hook registrations.
    settings({ name: "settings" }),
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
        clause(extent("lines", 120), {
          severity: "advisory",
          guidance:
            "a system doc is navigational, not exhaustive — past ~120 rendered lines, promote a section to a flow or split the system. Measured render-side, off the projected artifact rather than the source body.",
        }),
      ],
    },
    { kind: flow, clauses: [clause(uniqueName(), { severity: "required" })] },
    { kind: decision, clauses: [clause(uniqueName(), { severity: "required" })] },
    // The shipped default contract, adopted whole — adoption is a choice,
    // extension is a spread (`specs/builtins.md`, "Default contracts").
    { kind: skill, clauses: [...skillDefaultContract] },
    // In-play bindings, no clauses of their own: the embedded kinds so their
    // values are admitted nesting.
    { kind: invariant, clauses: [] },
    { kind: stepKind, clauses: [] },
    { kind: alternative, clauses: [] },
  ],
});

process.stdout.write(emit(program).seam);
