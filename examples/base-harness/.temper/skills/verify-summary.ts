import { blocks, input, span } from "@dtmd/temper";
import { skill } from "@dtmd/temper/claude-code";
import { GATE_COMMAND, VERIFY_COMMAND, VERIFY_EXPECTED } from "../facts.ts";

/**
 * The Operations exemplar: the end-to-end check for the governed toy, as an
 * on-demand procedure. Path-scoped, so it registers only when the work
 * touches what it knows how to verify. Every command it speaks is
 * interpolated from `facts.ts` — the memory's map states the same commands
 * from the same constants, so the two surfaces cannot disagree.
 */
export const skill_verifySummary = skill({
  name: "verify-summary",
  description:
    "Verifies the checklist summarizer end to end: runs it against TODO.md and compares the printed count with the file's items. Use after changing anything under src/ or TODO.md.",
  paths: ["src/**", "TODO.md"],
  satisfies: ["operations"],
  // The procedure's claims rest on the entrypoint it runs: declared as an
  // input, the lock fingerprints it, so a moved or deleted `src/main.js`
  // routes the author back to this skill before the next emit.
  inputs: [input(import.meta.url, "../../src/main.js")],
  prose: blocks(
    span(
      `# Verify the summary

The implementation under \`src/\` is a checklist summarizer and \`TODO.md\`
is its input. To confirm a change actually works:

1. Run \`${VERIFY_COMMAND}\`.
2. Against the committed \`TODO.md\` it prints \`${VERIFY_EXPECTED}\` — a
   \`done/total\` count over the checklist items. If \`TODO.md\` changed,
   count its \`[x]\` items yourself and compare.
3. On a mismatch, the corpus is authoritative: read
   \`docs/systems/scanner.md\` and \`docs/systems/renderer.md\` and decide
   whether the code or the document has the bug — never adjust the expected
   output to match broken behavior.
4. Finish with \`${GATE_COMMAND}\`: the structural gate must be green
   before the change is done.

The entrypoint under test is \`src/main.js\`.
`,
    ),
  ),
});
