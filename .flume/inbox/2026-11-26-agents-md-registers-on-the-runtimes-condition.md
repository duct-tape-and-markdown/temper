# `AGENTS.md` is memory by position; its load condition is registration, judged by reachability

This answers MEMORY-CLAIMS-AGENTS-MD's park (4993b871). The park was right that a kind is declared by position alone (`representation.md`, "locus"), so the fallback cannot be a locus condition. `specs/builtins.md`, memory, now rules it instead:
- `AGENTS.md` is a memory member by position (`**/AGENTS.md`, beside memory's `CLAUDE.md` family);
- its registration carries Claude Code's condition: it loads only where no `CLAUDE.md` sits in its directory or above;
- reachability judges the condition. An `AGENTS.md` that a `CLAUDE.md` shadows and does not import draws the not-loaded finding (0103), and one a `CLAUDE.md` imports is reached through the import edge.

Acceptance:
- **new:** a harness with only `AGENTS.md` at root has it as a reachable memory member;
- **new:** an `AGENTS.md` beside a `CLAUDE.md` with no import draws a not-loaded reachability finding;
- **kept:** a `CLAUDE.md` that imports `@AGENTS.md` reaches it.
