<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->








- `root.locus-declared`'s finding text asserts "yet Claude Code loads it" for every governed locus, including user kinds Claude Code never loads (reproduce: `temper check` in `examples/base-harness` → `src/*.js` under the `source` kind). The claim holds only for harness-surface kinds; a user kind's finding should not assert Claude loads the file. Observed at f9d1b962.
- The PostToolUse locus edge fired on a read-only Bash call (`temper check` in `examples/base-harness`), claiming "this call left a document at a represented kind's governed locus" — the tree was clean. Because shell writes name no path, it enumerates every undeclared document at the lock's loci and attributes pre-existing ones to the call; it should not assert the call wrote them. Observed at f9d1b962.
- An engine older than the lock's `engine` stamp fails at load with a generic `gate.load-fault` ("predicate `engine-matches` … not in the closed vocabulary") instead of saying the lock was written by a newer temper and naming both versions. `root.engine-matches` is advisory only for the newer-reads-older direction. The older-reads-newer direction can't reach that clause, so the load path should read the stamp first and refuse with an upgrade remedy. Observed at f9d1b962 (0.0.20 binary vs 0.0.21 lock).
