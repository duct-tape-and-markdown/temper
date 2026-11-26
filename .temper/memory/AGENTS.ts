import { file, memory } from "@dtmd/temper/claude-code";

/** The pointer other runtimes read in place of CLAUDE.md; Claude Code never loads it here. */
export const memory_AGENTS = memory({
  name: "AGENTS",
  prose: file(import.meta.url, "./AGENTS.md"),
});
