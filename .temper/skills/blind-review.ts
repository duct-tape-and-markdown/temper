import { file } from "@dtmd/temper/claude-code";
import { skill } from "../kinds.ts";

export const skill_blindReview = skill({
  name: "blind-review",
  description:
    "Use when a design, a set of names, or a proposal needs a fresh read not anchored on this session's reasoning — before encoding something the session argued itself into, or to test how a stranger reads it. Briefs a fresh agent from primary sources and a self-written definition, withholds the session's own answer, and reads the divergence as the finding.",
  prose: file(import.meta.url, "./blind-review.md"),
});
