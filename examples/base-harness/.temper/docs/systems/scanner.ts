import { blocks } from "@dtmd/temper";
import { invariantOf, passageOf, system } from "../../kinds.ts";

export const system_scanner = system({
  name: "scanner",
  "implemented-by": ["scan"],
  satisfies: ["documented-spine"],
  prose: blocks(
    passageOf(
      "overview",
      "The scanner classifies the lines of a checklist document. A line is " +
        "an item only when it carries a box: `- [ ]` open, `- [x]` done. " +
        "The scan yields the items in document order and nothing else.\n\n" +
        "## Invariants",
    ),
    invariantOf(
      "ignored-never-guessed",
      "Unrecognized lines are ignored, never guessed",
      "A line without a box is not an item: the scanner drops it and does " +
        "not repair near-misses. Loosening the pattern is a corpus " +
        "amendment first, code second.",
    ),
    invariantOf(
      "done-is-exact",
      "A done box is exactly `[x]`",
      "Lowercase `x`, nothing else. `[X]` and `[~]` are unrecognized lines " +
        "under the invariant above.",
    ),
  ),
});
