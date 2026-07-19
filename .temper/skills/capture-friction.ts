import { file, skill } from "@dtmd/temper/claude-code";

export const skill_captureFriction = skill({
  name: "capture-friction",
  description:
    "Use when this tick or session hit real, disproportionately costly friction with the harness (a pitfall it could have warned about, a slow gate, missing operational knowledge), touched structural debt it can't fix right now (a duplicate surface, a hand-roll a sanctioned crate already covers), or can ground an apply-ready diff — to the harness's operating layer, or advocating a process-phrase re-cut — in a named invariant. Files one terse capture to .flume/friction/, .flume/refactor/, or .flume/amendments/ in the documented format — exceptional, never a duty; most ticks file nothing.",
  prose: file(import.meta.url, "./capture-friction.md"),
  satisfies: ["friction-capture-procedure"],
});
