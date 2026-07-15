import { blocks } from "@dtmd/temper";
import { flow, participantsLine, span, step } from "../../kinds.ts";
import { system_scanner } from "../systems/scanner.ts";
import { system_renderer } from "../systems/renderer.ts";

// The steps are the one authored surface: each carries its system as an
// import, and the participants line below is rendered from them.
const steps = [
  step({
    key: "scan",
    title: "Scan the lines",
    in: system_scanner,
    body:
      "Every line of the document is classified. Lines carrying a box " +
      "become items; everything else drops out here and is never seen " +
      "again downstream.",
  }),
  step({
    key: "summarize",
    title: "Render the summary",
    in: system_renderer,
    body:
      "The surviving items tally into the summary line: done over total. " +
      "The document itself is out of reach by design.",
  }),
];

export const flow_summarize = flow({
  name: "summarize",
  prose: blocks(
    span(
      "Summarize is the toy's one behavior: read a checklist, report how " +
        "much of it is done (`node src/main.js TODO.md`).\n\n" +
        `${participantsLine(steps)}\n\n` +
        "## Steps",
    ),
    ...steps.map((entry) => entry.value),
  ),
});
