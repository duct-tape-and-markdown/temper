import { span } from "@dtmd/temper";
import { file } from "@dtmd/temper/claude-code";
import { playbook } from "@dtmd/temper/patterns";

/**
 * The engineering posture: how code enters this repository, as one playbook. The rule
 * carries the invariants as standing facts and loads when a session reads a file in the
 * trees they govern; an edit always reads first, so the facts are in context before any
 * write. Each invariant's full text (its tell and its holder) is a topic the skill routes
 * to, loaded on demand. The topics are the sweep's posture pages.
 */
const topic = (name: string, when: string) => ({
  name,
  when,
  prose: file(import.meta.url, `./engineering/${name}.md`),
});

export const playbook_engineering = playbook({
  name: "engineering",
  description:
    "The engineering posture of this repository: each invariant code must hold, with its tell and its holder. Use before adding a function, module, helper, walk, read, cache, type, export, test or comment under src/, tests/ or sdk/, and when a sweep or review cites one of its sections.",
  trigger: { paths: ["src/**", "tests/**", "benches/**", "sdk/**", "build.rs"] },
  facts: span(`# Engineering posture

How code enters this repository. Each line below is an invariant, and the
engineering skill holds its full section: its **tell** (the observable sign of
a break at one place) and its **holder** (the strongest thing that holds it
today, on the ladder: a type, then a test or pin, then a clause, then prose and
review). A **misfit** is code the next holder up would refuse. A **downgrade**
is a fix that holds the property lower than before, and is residue.

- One job, one home: extend the existing surface before adding one.
- Libraries before hand-rolls.
- A shared concept is one type.
- Derived state is computed, never stored beside its source.
- The fix lands at the mechanism.
- The gauntlet corpus holds every composition the model claims legal.
- Cost scale is hoisted, and pinned by count: a run reads each input once, through one owner.
- A green verdict is proven non-vacuous.
- A reader refuses or reports what it consumes.
- A fix ships the test that would have caught it.
- A seam gate reads what the real writer wrote.
- An export earns its consumer.
- Narration is the ladder's bottom rung.
- The tree stays flat.
- The provider face is data.`),
  topics: [
    topic("one-job-one-home", "adding a function, module, helper or test fixture"),
    topic("libraries-first", "about to hand-roll a glob, walk, parse or diff"),
    topic("shared-concept", "widening a shared enumeration or adding a parallel struct"),
    topic("derived-state", "storing a value another value already determines"),
    topic("fix-at-the-mechanism", "fixing a defect one kind or one path exposed"),
    topic("gauntlet", "shipping a surface that composes with existing ones"),
    topic("cost-scale", "adding a walk, a file read, a parse or a glob compile"),
    topic("non-vacuous", "writing a test that asserts a verdict"),
    topic("reader-reports", "adding a reader that consumes a key"),
    topic("fix-ships-its-test", "fixing a bug"),
    topic("seam-gate", "testing agreement across the SDK and engine seam"),
    topic("export-earns-consumer", "adding a pub item or an SDK export"),
    topic("narration", "writing a comment, a doc header or other prose"),
    topic("flat-tree", "adding a module or a directory under src/"),
    topic("provider-face", "naming a Claude Code fact outside the provider face"),
  ],
});
