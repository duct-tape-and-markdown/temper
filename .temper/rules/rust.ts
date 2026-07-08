import { file, rule } from "@dtmd/temper/claude-code";

export const rule_rust = rule({
  name: "rust",
  prose: file(import.meta.url, "../../.claude/rules/rust.md"),
});
