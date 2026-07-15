import { file, rule } from "@dtmd/temper/claude-code";

/**
 * The Conduct floor: how any agent behaves — epistemics, gap handling,
 * escalation. Path-less, so it registers unconditionally; zero project
 * facts, so it is the one member an adopter keeps verbatim.
 */
export const rule_conduct = rule({
  name: "conduct",
  satisfies: ["conduct"],
  prose: file(import.meta.url, "./conduct.md"),
});
