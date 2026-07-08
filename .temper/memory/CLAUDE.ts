import { file, text, memory } from "@dtmd/temper/claude-code";

export const memory_CLAUDE = memory({
  name: "CLAUDE",
  prose: file(import.meta.url, "./CLAUDE.md"),
});
