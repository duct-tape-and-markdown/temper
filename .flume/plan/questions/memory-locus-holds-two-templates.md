# Does a file locus hold more than one template, and what row carries AGENTS.md's load condition?

7b99c08c rules `AGENTS.md` a memory member by position (`**/AGENTS.md`, beside
`**/CLAUDE.md`), its registration conditioned on no `CLAUDE.md` at or above,
judged by reachability (`specs/builtins.md`, "The shipped kinds").

Two seams the corpus does not reach:

1. `specs/model/representation.md`, "locus": a file locus is **a** template
   (one string, `governs_glob` in the lock; `Locus.glob` in `sdk/src/kind.ts`).
   Memory now needs two. Does the locus grammar widen to a template list, or
   does `AGENTS.md` sit under some other spelling (a subkind can't: it governs
   a locus within its base's)?
2. The registration channels the lock declares name no "absent sibling or
   ancestor document" condition. Is this a new channel kind in the
   registration grammar, or a fact of the reachability predicate that the
   memory kind's registration row names?

Blocks MEMORY-CLAIMS-AGENTS-MD-WHERE-THE-RUNTIME-LOADS-IT.
