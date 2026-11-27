import { relocate } from "@dtmd/temper";
import { rule as builtinRule, skill as builtinSkill, type Rule, type Skill } from "@dtmd/temper/claude-code";

/**
 * The built-in skill kind, less the test fixtures: a fixture tree holds
 * harness files as test data, never as this harness's members (0090).
 */
export const skill = relocate<Skill>(builtinSkill, { exclude: ["tests/fixtures"] });

/**
 * The built-in rule kind, less the trees that hold rules as data: the test
 * fixtures and the demo harness the README renders. Rules nest at any depth
 * (0118), so the claim reaches them unless excluded here.
 */
export const rule = relocate<Rule>(builtinRule, { exclude: ["tests/fixtures", "examples/demo"] });
