# The memory kind claims `AGENTS.md` where Claude Code loads it as memory (0101)

An instance of class 0101 (a built-in reads as the runtime reads). code.claude.com/docs/en/memory (retrieved 2026-11-26): "Claude can also read a repository's `AGENTS.md` files in place of CLAUDE.md … By default, Claude reads `AGENTS.md` only when you have no `CLAUDE.md` in your working directory or above it" (v2.1.277 or later). The page also recommends "an `@AGENTS.md` import in a `CLAUDE.md` next to it" to share one file across tools. temper's memory kind claims only the `CLAUDE.md` family, so a harness that keeps its instructions in `AGENTS.md` alone has its memory ungoverned.

Memory claims an `AGENTS.md` exactly where the runtime loads one as memory: in a directory with no `CLAUDE.md` in it or above it. An `AGENTS.md` that a `CLAUDE.md` imports stays reached through the import edge (shipped), and is not a second memory member. Codex's own rules (`AGENTS.override.md`, nested overrides) belong to a future Codex provider face, not to claude-code. Acceptance:
- **new:** a harness with only `AGENTS.md` at root governs it as memory;
- **new:** an `AGENTS.md` beside a `CLAUDE.md` is not a memory member;
- **kept:** a `CLAUDE.md` that imports `@AGENTS.md` reaches it through the import edge.
