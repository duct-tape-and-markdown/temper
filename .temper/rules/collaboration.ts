import { file, text, rule } from "@dtmd/temper/claude-code";

export const rule_collaboration = rule({
  name: "collaboration",
  prose: file(import.meta.url, "./collaboration.md"),
});
