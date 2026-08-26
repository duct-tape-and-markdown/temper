# ASSIGNED ENTRY

<entry>
{{ENTRY_JSON}}
</entry>

# THE WHY

Find the section named `{{PER_SECTION}}` (or the nearest equivalent heading) in
the file below. The rest of the spec is context for intent, not scope.

<spec path="{{PER_PATH}}">
!`cat {{PER_PATH}} 2>/dev/null || echo "(spec not found: {{PER_PATH}})"`
</spec>

# CONTEXT

<src-tree>
!`{ find src tests -name '*.rs'; find sdk/src sdk/test -name '*.ts'; } 2>/dev/null | sort`
</src-tree>

<recent-commits>
!`git log -n 5 --oneline`
</recent-commits>

<premise-delta>
{{SCOPED_DELTA}}
</premise-delta>

# TASK

Execute entry `{{TAG}}` completely: `entry.acceptance` must hold, with tests
alongside the code (`entry.tests[]` names what must turn green).

- The writable paths in the `<harness>` block are exact — `entry.files` plus
  the capture dirs. One path outside them reverts the entire commit.
- **If reaching green needs a file `entry.files` didn't list — almost always
  an existing test your change breaks — do NOT ship doomed work.** File the
  exact path(s) and why as a `build-<slug>.md` capture in `.flume/refactor/`
  (plan drains it and re-scopes the entry), commit the capture alone, and
  end the tick.
- `<premise-delta>` shows what already landed on the entry's files since it
  was scoped: an already-landed fix narrows the entry to its remainder, or
  empties it (leave it uncommitted and say so in the report).
- If the entry's `per` cite is ambiguous or rests on an unsettled decision,
  do NOT guess — leave it and surface the question.

# OUTPUT

One commit on this worktree's branch, prefixed `build:`. Imperative subject;
the body explains *why*. Never commit anywhere else — merging onto the trunk
is the dispatcher's job, and a commit made off this branch is lost to ship
bookkeeping.

Gates run automatically against your commit and are the definition of done —
run their commands and reach green before committing:

{{GATES}}

A gate failure reverts the commit and the entry returns to pending. If you
genuinely cannot reach green, do not bail silently: state in your final
message exactly what blocked you and what you tried — it is recorded and
programs this entry's next attempt.
