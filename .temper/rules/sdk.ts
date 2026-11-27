import { file } from "@dtmd/temper/claude-code";
import { rule } from "../kinds.ts";

export const rule_sdk = rule({
  name: "sdk",
  paths: ["sdk/**/*.ts"],
  prose: file(import.meta.url, "./sdk.md"),
});
