# A bare edge target in a one-element `to` set aborts emit where its `kind:name` spelling defers to check (GH #52)

Observed on 0.0.24, triaging GH #52. An embedded member's edge field `cite` with `to: ["skill"]`:

- `cite: "skill:no-such-skill"` — `emit` exits 0, and `check` reports `graph.route` (resolves to no `skill` artifact);
- `cite: "no-such-skill"` — `emit` exits 1 with `drift::sdk_program_failed` ("resolves to no composed member").

`specs/model/contract.md`, "edge": a one-element set resolves a bare name within its one kind, so these are one address. `specs/model/pipeline.md`, "Emit", "Refusing", defers that address to the gate. `edgeTargetFacts` (`sdk/src/emit.ts`) judges `defersToGate` on the written text, not on the address after `edgeLookupKey`'s rewrite. Judge it on the rewritten address when `to` has one element. A bare name in a multi-element set still refuses, because the contract requires the qualified spelling there. Pin both spellings to the same class beside `sdk/test/refusals.test.ts` "emit defers an edge field naming a declared at-locus kind's uncomposed member".
