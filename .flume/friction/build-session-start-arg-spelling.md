## Symptom

The repo's own SessionStart hook is wired with the wrong `check` argument. Both
`.claude/settings.json` (`temper check .temper --reporter session-start`) and the
`.flume/chain.ts` comment (line ~193, "its gate — `temper check .temper`") use the
**surface** spelling `.temper`, not the **harness-root** spelling `.`.

With the CHECK-ARG-HALF-GATE fix, the path argument is a harness root for every
reporter: `check .temper` resolves `.temper/.temper` (absent) → gates the raw
`.temper` dir → walks members off `.temper/` (empty) while reading the lock's
requirements from `.temper/lock.toml`. Every declared requirement therefore reads
unfilled. That is exactly what fired at this session's start:

    [requirement.unfilled] friction-capture-procedure ...
    [requirement.unfilled] pending-entry-discipline ...

Both are false — a member does satisfy each; the members were just walked off the
wrong root. `temper check . --reporter session-start` on the same repo is clean
(empty payload). Note this was *already* broken before this tick (session-start
has resolved the arg as a harness root for a while); the fix did not introduce it,
it just makes the terminal reporter agree, so the misspelling is now unambiguous.

## Cost this tick

~0 min to the entry (the fix is orthogonal), but the false findings mislead every
session that opens against this repo — they demand notify-and-approve on a
contract that is actually clean, training the reader to ignore the gate.

## Suggested fix

Human edit (both are outside build's writable paths — `.claude/` and `.flume/`
are human territory):

- `.claude/settings.json`: change the SessionStart command from
  `temper check .temper --reporter session-start` to
  `temper check . --reporter session-start` (matches `install.rs`'s
  `SESSION_START_COMMAND`, which already ships `.`).
- `.flume/chain.ts` line ~193 comment: `temper check .temper` → `temper check .`.

The prior hand-fix (notes: aba7e47, 549969f) picked `.temper` believing it was the
correct spelling; it is the broken one.
