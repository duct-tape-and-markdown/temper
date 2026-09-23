## Surface

Two surfaces now author `.claude/settings.json`, and the later one silently
erases the earlier one's bytes:

- `install::evaluate_placements` (src/install.rs:531) splices the
  `SessionStart`, `PreToolUse` and `PostToolUse` groups in through
  `json_splice`, format-preservingly;
- the second `drift::emit_program` immediately after it (src/install.rs:545)
  — there to re-stamp the lock over the placement-inclusive bytes —
  re-renders the whole file from the `settings` member, which carries only
  the keys the lift hoisted. The three hook groups are gone.

Reachable only since this tick: before it, the lift read `settings.json` as
frontmatter and emit refused `drift::body_has_no_home`, so the yes-path over
a project carrying committed settings never got this far. Measured on a
fixture carrying `.claude/settings.json` + `.claude-plugin/plugin.json`: all
three placements report `ApplyOutcome::Applied`, and the file on disk
afterwards carries no `hooks` key at all — so the reported outcome is also
wrong, not just the bytes.

Out of this entry's scope: it names the read dispatch, not settings'
write ownership, and which surface should yield is a design call
(`specs/model/adoption.md` "Install" says install wires the reporter *and*
the represented path emits — it does not say who owns `hooks` when both
run).

## Observed at

0804029f (HEAD when observed), plus this tick's `src/install.rs` change.

## Suggested consolidation

One writer for the file. Either the lift converts the `hooks` collection
address into real `hook` members (the kind exists and is `Content::Fields`,
which the lift skips today) so the program authors the placement and emit
renders it, or the placement runs after the last emit and the lock re-stamp
reads the spliced bytes. The first keeps emit's codomain whole; the second
keeps install's gate wiring outside the program.
