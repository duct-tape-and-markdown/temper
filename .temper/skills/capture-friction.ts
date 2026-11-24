import { file, skill } from "@dtmd/temper/claude-code";

export const skill_captureFriction = skill({
  name: "capture-friction",
  description:
    "Use when this tick or session hit real, disproportionately costly friction with the harness (a pitfall it could have warned about, a slow gate, missing operational knowledge), or touched structural debt it can't fix right now (a duplicate surface, a hand-roll a sanctioned crate already covers). Files one terse note to .flume/friction/ or a build note under .flume/plan/notes/ in the documented format — exceptional, never a duty; most ticks file nothing.",
  prose: file(import.meta.url, "./capture-friction.md"),
  satisfies: ["friction-capture-procedure"],
});
