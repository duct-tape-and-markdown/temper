import { relocate } from "@dtmd/temper";
import { skill as builtinSkill, type Skill } from "@dtmd/temper/claude-code";

/**
 * The built-in skill kind, less the test fixtures: a fixture tree holds
 * harness files as test data, never as this harness's members (0090).
 */
export const skill = relocate<Skill>(builtinSkill, { exclude: ["tests/fixtures"] });
