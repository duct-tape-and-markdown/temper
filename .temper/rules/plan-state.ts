import { file, rule } from "@dtmd/temper/claude-code";

export const rule_planState = rule({
  name: "plan-state",
  paths: [".flume/plan/state.md"],
  prose: file(import.meta.url, "./plan-state.md"),
  satisfies: ["plan-state-discipline"],
});
