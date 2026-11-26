# Which AGENTS.md load facts does memory's registration carry?

`specs/builtins.md`, "memory" and registration: an `AGENTS.md` opens only where no `CLAUDE.md` sits "in its directory or above", and memory claims `**/AGENTS.md`. The memory docs (code.claude.com/docs/en/memory, retrieved 2026-11-26, "When Claude Code reads AGENTS.md" and "Choose which instruction files load") state three facts the spec does not:

1. `.claude/AGENTS.md` loads at launch beside `AGENTS.md`. Memory does not claim it, so `check` never sees one a repo carries.
2. A subdirectory's `AGENTS.md` loads on Read when "that subdirectory has none of the three `CLAUDE.md` files of its own". The page never says whether a `CLAUDE.md` above that subdirectory suppresses it. The spec and the shipped gate (`gate::absence_gated`) read "or above", which is one of two readings the docs allow. A probe would settle it.
3. The **Project instructions** setting (`claude-md-and-agents-md`, `managed-only`) changes the gate. Under `claude-md-and-agents-md`, a shadowed `AGENTS.md` loads, and the `root.reachable` advisory on it is false.

Needed: a ruling on each, to be written into `builtins.md` with its cite. Plan files no entry before then. Raised by build note MEMORY-CLAIMS-AGENTS-MD-WHERE-THE-RUNTIME-LOADS-IT.
