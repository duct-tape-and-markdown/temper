import { file, rule } from "@dtmd/temper/claude-code";

export const rule_postureSweep = rule({
  name: "posture-sweep",
  paths: ["specs/process/engineering.md", "specs/process/architecture.md"],
  prose: file(import.meta.url, "./posture-sweep.md"),
  satisfies: ["posture-sweep-discipline"],
});
