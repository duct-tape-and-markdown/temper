import { file, rule } from "@dtmd/temper/claude-code";

export const rule_docsDiscipline = rule({
  name: "docs-discipline",
  paths: ["docs/**"],
  prose: file(import.meta.url, "./docs-discipline.md"),
});
