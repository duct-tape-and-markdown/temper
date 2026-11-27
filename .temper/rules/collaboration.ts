import { file, text } from "@dtmd/temper/claude-code";
import { rule } from "../kinds.ts";

export const rule_collaboration = rule({
  name: "collaboration",
  prose: file(import.meta.url, "./collaboration.md"),
});
