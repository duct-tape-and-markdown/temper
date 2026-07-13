import { clause, count, emit, harness, maxLines, uniqueName } from "@dtmd/temper";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_docsDiscipline } from "./rules/docs-discipline.ts";
import { hook_guard, hook_sessionStart } from "./hooks.ts";
import { decision, flow, glossary, supersededDecision, system } from "./kinds.ts";

// Layout members carry no prose here: each document is the authored home,
// read under its kind's declared layout — emit derives rows from it and
// writes nothing at its path.
const program = harness({
  require: {
    // `required: false` is deliberate: the fills live in the documents'
    // own `Satisfies` sections, which the engine derives at emit — the SDK's
    // fill check cannot see them and would refuse a `required` posture.
    "documented-spine": {
      prose:
        "every load-bearing area of this repository's behavior is documented as a system whose document claims spine membership",
      kind: system,
      required: false,
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
    system({ name: "corpus" }),
    system({ name: "gate" }),
    flow({ name: "change" }),
    flow({ name: "drift-repair" }),
    decision({ name: "authority-arrow" }),
    supersededDecision({ name: "per-change-doc-duty" }),
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
  ],
});

process.stdout.write(emit(program).seam);
