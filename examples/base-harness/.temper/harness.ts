import { clause, count, emit, harness, maxLines, uniqueName } from "@dtmd/temper";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_docsDiscipline } from "./rules/docs-discipline.ts";
import { hook_guard, hook_sessionStart } from "./hooks.ts";
import { alternative, decision, flow, glossary, invariant, source, stepKind, system } from "./kinds.ts";
import { system_scanner } from "./docs/systems/scanner.ts";
import { system_renderer } from "./docs/systems/renderer.ts";
import { flow_summarize } from "./docs/flows/summarize.ts";
import { decision_authorityArrow } from "./docs/decisions/authority-arrow.ts";
import { decision_perChangeDocDuty } from "./docs/decisions/per-change-doc-duty.ts";

const program = harness({
  require: {
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
    rule_docsDiscipline,
    system_scanner,
    system_renderer,
    flow_summarize,
    decision_authorityArrow,
    decision_perChangeDocDuty,
    glossary({ name: "glossary" }),
    hook_sessionStart,
    hook_guard,
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
