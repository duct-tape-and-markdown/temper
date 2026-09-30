import { file, rule } from "@dtmd/temper/claude-code";

export const rule_release = rule({
  name: "release",
  paths: [
    "Cargo.toml",
    "sdk/package.json",
    "sdk/package-lock.json",
    ".github/workflows/release.yml",
    "CHANGELOG.md",
    "docs/MIGRATING-*.md",
  ],
  prose: file(import.meta.url, "./release.md"),
});
