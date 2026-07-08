import { file, rule } from "@dtmd/temper/claude-code";

export const rule_collaboration = rule({
  name: "collaboration",
  prose: file(import.meta.url, "../../.claude/rules/collaboration.md"),
});
