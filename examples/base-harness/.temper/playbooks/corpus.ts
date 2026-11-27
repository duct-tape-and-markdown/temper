import { blocks, mentionOf, span, text } from "@dtmd/temper";
import { playbook } from "@dtmd/temper/patterns";
import { system_renderer } from "../docs/systems/renderer.ts";
import { system_scanner } from "../docs/systems/scanner.ts";
import { GATE_COMMAND, VERIFY_COMMAND, VERIFY_EXPECTED } from "../facts.ts";

/**
 * The worked playbook: one area — the documentation corpus and the code it
 * governs — emitted as a rule holding the area's standing facts and a skill
 * routing to one topic file per procedure. The shape supplies the wiring:
 * the rule's `paths`, its closing pointer to the skill, the router body and
 * the typed links to each topic. This module supplies the content.
 *
 * Path-triggered, so the facts load the first time a session reads a file
 * the area covers; the skill stays listed regardless, so a question asked
 * before any file is read can still find its procedures.
 */
export const playbook_corpus = playbook({
  name: "corpus",
  description:
    "Procedures for the documentation corpus and the code it governs: verifying the checklist summarizer end to end, and superseding a decision. Use after changing anything under src/ or TODO.md, or when a ruling under docs/decisions/ is replaced.",
  trigger: { paths: ["docs/**", "src/**", "TODO.md"] },
  // The area fills two domains: its facts are the standards for these trees,
  // its procedures the operations done to them.
  satisfies: { rule: ["standards"], skill: ["operations"] },
  facts: span(`# The corpus is the program

\`docs/\` is declared intent and \`src/\` reconciles toward it: when the two
disagree, the code has the bug or the corpus needs an authored amendment.

- Everything under \`docs/\` except \`glossary.md\` is a projection: the
  authored home is the owning module under \`.temper/docs/\` (narrative
  passages, invariants, steps, and alternatives are typed values composed in
  the module). Edit the module, then run \`temper emit\`; a direct edit is
  drift.
- \`docs/glossary.md\` is the one layout source: edit it in place. Its terms
  are addressable members, so renaming one is a model change, not a
  find-and-replace.
- The structural discipline — a step's system resolves, a system's
  implementing files (its declared inputs) exist and are fingerprinted, a
  superseded ruling names its successor, spine coverage holds — is enforced
  by the program and \`${GATE_COMMAND}\`, not by this rule.
- Accepted decisions receive corrections, not new meanings.`),
  topics: [
    {
      name: "verify-summary",
      when: "a change under `src/` or `TODO.md` needs confirming",
      prose: blocks(
        span(`# Verify the summary

The implementation under \`src/\` is a checklist summarizer and \`TODO.md\`
is its input; the entrypoint under test is \`src/main.js\`.

1. Run \`${VERIFY_COMMAND}\`.
2. Against the committed \`TODO.md\` it prints \`${VERIFY_EXPECTED}\` — a
   \`done/total\` count over the checklist items. If \`TODO.md\` changed,
   count its \`[x]\` items yourself and compare.
3. Finish with \`${GATE_COMMAND}\`: the structural gate must be green
   before the change is done.`),
        text`
          On a mismatch, the corpus is authoritative: read
          [the scanner](${mentionOf(system_scanner, { as: "path" })}) and
          [the renderer](${mentionOf(system_renderer, { as: "path" })}) and decide
          whether the code or the document has the bug — never adjust the
          expected output to match broken behavior.`,
      ),
    },
    {
      name: "supersede-a-decision",
      when: "a standing ruling under `docs/decisions/` is replaced",
      prose: span(`# Supersede a decision

1. Declare the successor as its own decision module under
   \`.temper/docs/decisions/\`.
2. Rewrite the old ruling's module as \`supersede(successor, { … })\` from
   \`.temper/kinds.ts\`, the successor imported: its record moves to
   \`docs/decisions/superseded/\`, carrying the successor edge.
3. Never edit the accepted text into something new. Evidence against a
   standing ruling enters as a proposed amendment, never as a quiet
   rewrite.
4. Run \`temper emit\`, then \`${GATE_COMMAND}\`.
`),
    },
  ],
});
