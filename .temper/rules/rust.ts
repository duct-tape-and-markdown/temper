import { file, text } from "@dtmd/temper/claude-code";
import { rule } from "../kinds.ts";

export const rule_rust = rule({
  name: "rust",
  paths: ["src/**/*.rs","tests/**/*.rs","benches/**/*.rs"],
  prose: file(import.meta.url, "./rust.md"),
});
