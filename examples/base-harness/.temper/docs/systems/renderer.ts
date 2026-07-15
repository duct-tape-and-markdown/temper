import { blocks } from "@dtmd/temper";
import { invariantOf, passageOf, system } from "../../kinds.ts";

export const system_renderer = system({
  name: "renderer",
  "implemented-by": ["render"],
  satisfies: ["documented-spine"],
  prose: blocks(
    passageOf(
      "overview",
      "The renderer turns scan results into the one-line summary " +
        "(`2/3 done`): items done over items total.\n\n" +
        "## Invariants",
    ),
    invariantOf(
      "derives-never-rereads",
      "The summary derives from scan results, never the document",
      "The renderer takes the scanned items alone. If a count looks wrong, " +
        "the scan is wrong; the renderer holds no second opinion about the " +
        "document.",
    ),
    invariantOf(
      "one-line",
      "One line out",
      "The summary is a single line with no trailing detail. Anything " +
        "richer is a new declared behavior, not a bigger string.",
    ),
  ),
});
