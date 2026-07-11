import { file, rule } from "@dtmd/temper/claude-code";

export const rule_publicProse = rule({
  name: "public-prose",
  paths: [
    "README.md",
    "CHANGELOG.md",
    "sdk/README.md",
    "docs/**/*.md",
    ".github/**",
  ],
  prose: file(import.meta.url, "./public-prose.md"),
});
