import { file, rule } from "@dtmd/temper/claude-code";

export const rule_forkLifecycle = rule({
  name: "fork-lifecycle",
  paths: [".flume/plan/open-questions.md"],
  prose: file(import.meta.url, "./fork-lifecycle.md"),
  satisfies: ["fork-lifecycle-discipline"],
});
