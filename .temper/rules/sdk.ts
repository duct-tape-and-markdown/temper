import { file, rule } from "@dtmd/temper/claude-code";

export const rule_sdk = rule({
  name: "sdk",
  paths: ["sdk/**/*.ts"],
  prose: file(import.meta.url, "./sdk.md"),
});
