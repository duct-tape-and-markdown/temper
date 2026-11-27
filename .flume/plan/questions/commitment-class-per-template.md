# Does a commitment class bind a whole file locus, or one template inside it?

`specs/builtins.md` (memory, 0116) makes `CLAUDE.local.md` "a local-locus template" of memory, whose other templates (`CLAUDE.md`, `.claude/CLAUDE.md`, `AGENTS.md`) are committed. `specs/model/representation.md`, "locus", states the class per file locus: "A file locus may declare a commitment class of **local** … the kind reviewed and its documents not", and its local member is read-side only with rows out of the lock. The code agrees: `commitment` is one field of the kind's single `at` locus (`sdk/src/kind.ts`, `src/kind.rs` `CustomKind::commitment`), so a kind is all-local or all-committed.

Two readings, each consistent with both sections: (a) a template may carry its own class, widening the locus grammar to per-template commitment; (b) `CLAUDE.local.md` is a separate local-locus kind (a memory subkind), keeping commitment a whole-locus fact. Nothing in intent ranks them. A sentence in "locus" (or in memory's row) closes it.

Dependents: MEMORY-CLAIMS-CLAUDE-LOCAL-MD and the local-memory half of the size advisory (0116) rest on this; derive files them once it is answered. Raised by the reverted plan-derive tick bf57dea3 (inbox finding a plan slice may not write).
