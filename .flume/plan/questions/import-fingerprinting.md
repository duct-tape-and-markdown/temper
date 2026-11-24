# Does emit fingerprint a member's imported modules, or does `input()` stay the only declared dependency?

Driver: GH #61. A member whose value is computed from an imported module passes `root.fresh` after the module changes, until the member declares it with `inputs`. The `fresh` guidance says a "source dependency it imports or includes" moved, which an author reads as a TS import.

Session stance (2026-11-24): emit records the program's local module closure (non-`node_modules` files the program loaded) as fingerprints; a TS import is a reference the target format executes, the shape `specs/model/contract.md` already admits for `@path` imports, so it is declared, not mined. `input()` then narrows to non-code files. The objection to answer: per-member attribution of the closure, and the cost of re-hashing it on check. Ruling is a `specs/model/authoring.md` change. Until then, the derivable half is the guidance text, which must not promise what the engine does not fingerprint.
