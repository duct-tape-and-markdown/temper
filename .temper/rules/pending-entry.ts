import { file, rule } from "@dtmd/temper/claude-code";

export const rule_pendingEntry = rule({
  name: "pending-entry",
  paths: [".flume/plan/pending.json"],
  prose: file(import.meta.url, "./pending-entry.md"),
});
