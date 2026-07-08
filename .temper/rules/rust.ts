import { file, text, rule } from "@dtmd/temper/claude-code";

export const rule_rust = rule({
  name: "rust",
  paths: ["src/**/*.rs","tests/**/*.rs","benches/**/*.rs"],
  prose: file(import.meta.url, "./rust.md"),
});
