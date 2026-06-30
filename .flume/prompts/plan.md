# CURRENT STATE

<pending-json>
!`cat .flume/plan/pending.json 2>/dev/null || echo "[]"`
</pending-json>

<state>
!`cat .flume/plan/state.md 2>/dev/null || echo "(no prior state)"`
</state>

<open-questions>
!`cat .flume/plan/open-questions.md 2>/dev/null || echo "(none)"`
</open-questions>

<inbox>
!`cat .flume/inbox.md 2>/dev/null || echo "(empty)"`
</inbox>

<spec-corpus>
!`for f in $(ls specs/*.md 2>/dev/null | sort); do echo "===== $f ====="; cat "$f"; echo; done || echo "(no specs)"`
</spec-corpus>

<src-tree>
!`find src tests -name '*.rs' 2>/dev/null | sort`
</src-tree>

<cargo-check>
!`cargo clippy --all-targets -- -D warnings 2>&1 | tail -20 || true`
</cargo-check>

<recent-commits>
!`git log -n 10 --oneline`
</recent-commits>

# TASK

`specs/` is the evergreen source of truth (see `specs/90-spec-system.md`). It is
not a release target — it is the current intent, and your job is to reconcile the
code against it. The spec is law: if code and spec disagree on intent, the spec
wins.

1. **Reconcile** every existing pending entry against the spec section named in
   its `per` cite and the files named in `files`. A stale entry gets a full
   rewrite, never a patch. Drop entries whose work has shipped (verify on disk —
   read `src/`, never grep the git log).

2. **File the gap between the corpus and current `src/`** as new `open` entries,
   each with a truthful `per` cite into the spec section that owns the intent and
   truthful `files` (the partition reads `files.edit[].path` — keep entries small
   and disjoint; a Rust entry that creates a new module should also add its
   `pub mod` line in the foundation entry, not co-edit `lib.rs` from two entries).
   Scope `files` to the truthful **blast radius** — include existing tests/snapshots
   a change will break — so build reaches green inside the planned scope instead
   of discovering the ripple mid-tick.
   Honor the law in `specs/00-intent.md`: only decidable contract clauses become
   checks; behavior is delegated, never guessed. Do not re-introduce heuristic
   rules the corpus rejected.

3. **Drain `.flume/inbox.md`.** Route each line into pending (with a `per` cite),
   open-questions (no clean cite, or a product fork), or accepted debt (noted in
   the commit body). Remove drained lines.

4. **Re-derive `state.md` from scratch** (~5 lines: phase, last shipped, in
   flight, what's next). Never carry forward.

5. **Open questions** belong in `open-questions.md`, never in pending. If a
   candidate entry can't carry a clean `per` cite, it is an open question. Key
   each with a `(slug)` so entries can declare `dependsOnForks: ["slug"]`.

6. **Continuation marker.** End `state.md` with `Plan continues: yes — <reason>`
   ONLY if there is concrete *additional plan work* left this turn (an undrained
   inbox, unreconciled drift, entries still to rewrite). Otherwise `Plan
   continues: no`. **Bias hard toward `no`:** once the queue is reconciled and any
   `open` entry exists, hand to build — building is how the queue drains. Never
   re-emit a queue identical to the last `plan:` commit and continue; if your
   reconciliation produced no change and pickable work exists, you are done —
   write `Plan continues: no` and let build run. Re-planning an unchanged queue
   is the failure mode, not diligence.

You do NOT write `specs/` — intent is human-authored. Surface spec ambiguity as
an open question; never silently fill it.

# OUTPUT

One commit prefixed `plan:`. Write `.flume/plan/{pending.json,state.md,open-questions.md}`
and drain `.flume/inbox.md`. The harness rejects the commit if `pending.json`
doesn't parse or you modify anything outside the phase's writable paths.

<schema>
{{PENDING_SCHEMA}}
</schema>
