import { blocks, text } from "@dtmd/temper";
import { skill } from "@dtmd/temper/claude-code";
import { span } from "../kinds.ts";
import { GATE_COMMAND, VERIFY_COMMAND, VERIFY_EXPECTED } from "../facts.ts";

/**
 * The entrypoint under test, addressed as the discovered `source` member.
 * The SDK defers a discovery-locus mention to the gate (the kind is
 * declared; the member is the engine's to resolve), so this skill's claim
 * about where the toy lives dangles loudly at `check` if `src/main.js`
 * moves.
 */
const srcMain = { address: "source:main", display: "src/main.js" };

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
`,
    ),
    text`The entrypoint under test is ${srcMain}.`,
  ),
});
