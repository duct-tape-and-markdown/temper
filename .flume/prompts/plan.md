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

<refactor-captures>
!`found=0; for f in .flume/refactor/*.md; do [ -e "$f" ] || continue; [ "${f##*/}" = "README.md" ] && continue; echo "===== $f ====="; cat "$f"; echo; found=1; done; [ "$found" -eq 0 ] && echo "(none)"`
</refactor-captures>

<spec-corpus>
!`for f in $(find specs -name '*.md' ! -path 'specs/decisions/*' 2>/dev/null | sort); do echo "===== $f ====="; cat "$f"; echo; done || echo "(no specs)"`
</spec-corpus>

<spec-delta>
!`CURSOR=$(grep -oE '^- Spec derived through: [0-9a-f]+' .flume/plan/state.md 2>/dev/null | grep -oE '[0-9a-f]+$'); [ -z "$CURSOR" ] && CURSOR=$(git log -1 --format=%h --grep='^plan:' 2>/dev/null); if [ -n "$CURSOR" ]; then echo "specs/ commits past the spec cursor ($CURSOR):"; git log --reverse --format='%h %s' "$CURSOR"..HEAD -- specs/; echo; git diff --stat "$CURSOR"..HEAD -- specs/ | tail -15; else echo "(no cursor and no prior plan commit — treat the whole corpus as the delta)"; fi`
</spec-delta>

<src-tree>
!`{ find src tests -name '*.rs'; find sdk/src sdk/test -name '*.ts'; } 2>/dev/null | sort`
</src-tree>

<cargo-check>
!`cargo clippy --all-targets -- -D warnings 2>&1 | tail -20 || true`
</cargo-check>

<recent-commits>
!`git log -n 15 --oneline`
</recent-commits>

# TASK

`specs/` is the evergreen source of truth (`specs/process/spec-system.md`).
The corpus body is the current intent; `specs/decisions/` is history, outside
your read path — a new decision reaches you through the spec delta, once. If
code and spec disagree on intent, the spec wins. You do NOT write `specs/` —
intent is human-authored. Surface ambiguity as an open question; never
silently fill it.

**One tick = one job.** Orient off the state above, take the FIRST live input
in the order below, do that job completely, update the cursors in `state.md`,
set the continuation marker. Never take two jobs in one tick; never leave the
chosen job half-done — the job is the atom.

1. **Inbox** — `<inbox>` has content or `<refactor-captures>` holds live
   captures. Route each inbox line into pending (with a `per` cite),
   open-questions (no clean cite, or a product fork), or accepted debt
   (noted in the commit body); remove drained lines. Drain each refactor
   capture into a pending entry citing `specs/process/engineering.md` and
   DELETE the capture file — a claim that no longer holds at HEAD is deleted
   with a note in the commit body (`.flume/refactor/README.md`). A report's claimed
   gap is re-verified against the current tree before it scopes an entry —
   grep for the claimed-missing surface, run the report's probe; the gap may
   have narrowed or moved since filing. A note stamped `observed at <sha>`
   narrows the re-verify to `git log <sha>..HEAD` — diff forward from what
   the reporter saw. Scope to the verified gap, never the reported one.

2. **Spec delta** — `<spec-delta>` lists `specs/` commits past the cursor.
   Read each commit's diff (`git show <sha> -- specs/`) — ratified intent
   changes there before it changes anywhere. Derive one contained slice into
   entries and advance `Spec derived through:` to the last commit you have
   fully **routed**: every slice either derived into entries or registered as
   a keyed open fork in open-questions. A fork record IS routing — the
   resolution returns through the inbox, a fresh input — so fork-parked
   content never holds the cursor. The cursor is a claim of routed-ness, not
   of having looked. A big delta takes several ticks; that is the design.
   **A ratified Decision's own Consequences section is the derivation
   checklist**: enumerate every bullet by name in the plan commit body — each
   one resolves to a filed entry (name the tag), a verified-already-moot
   claim (say what you checked on disk), or a registered open fork — before
   the decision counts as routed. A bullet with none of the three is not
   routed; "derived into N entries" is not itself a completeness argument,
   and the cursor does not advance past an incompletely-checked decision.

3. **Ship audit** — commits past `Audited through:` touched `src/`, `tests/`,
   or `sdk/`. Verify on disk what shipped (read the files, never the log
   alone), drop pending entries whose work is done, and re-test every stale
   gate: a `parked` reason, a `blockedBy`, an open-question's "rides X"
   routing each name a condition — verify the condition NOW; if the blocker
   shipped, the work behind it is derivable this tick. Advance the cursor.

4. **Residue sweep** — all above quiet, and `Residue swept through:` trails
   HEAD. Sweep code against corpus: a retirement the delta named, body text
   naming a demolition, symbols or vocabulary the corpus no longer sanctions
   still living in `src/`, `tests/`, or `sdk/` — each with no pending entry
   operationalizing it is a fileable gap. A second implementation of one job
   is the structural residue class, same rule
   (`specs/process/engineering.md`, "One job, one home"). Cite the owning spec section in
   `per`, name the living symbols in `files[].description`. Comment and
   citation staleness is the one exception: it only ever rides whichever
   entry next opens that file — never a standalone entry, never the queue's
   only new work. The routing rule from job 2 applies here too: a residue
   class blocked on an open fork is routed by that fork's record. Advance the
   cursor to HEAD when every class is filed, riding, or fork-routed.

5. **Quiet** — every input above is current. One closing pass: the queue is
   disjoint, every gate reason still true, `state.md` re-derived. Write
   `Plan continues: no` and hand off.

**Entry discipline** (binds every job that files or rewrites entries):

- A stale entry gets a full rewrite, never a patch. Every entry carries a
  truthful `per` cite into the spec section that owns the intent and truthful
  `files` (the partition reads `files.edit[].path`).
- **One entry = one gate-sized commit, comfortably under 200k tokens of build
  work.** Lettered sub-parts or an internal task list mean it is not one
  entry — it is a `blockedBy` chain; file the split up front. Scope `files`
  to the honest ripple — include existing tests/snapshots the change will
  break.
- Every path in `files` — `new[].path`, `edit[].path`, `retire` (bare
  strings) — is a repo-relative file path; the fence gate glob-matches all
  three. `retire` means "this FILE is deleted"; retiring a symbol within a
  surviving file is an `edit`.
- **Every surface an entry cites must resolve.** `edit`/`retire` paths exist
  on disk, `new` paths don't, the `per` section is in its file (all gated).
  Symbol-level claims in descriptions — a struct, a lock column, a schema
  surface — either resolve on disk (`rg` before citing) or are written
  "new `X`"; a mechanism you can neither resolve nor mark is an open
  question, never a sub-clause of an entry. Stamp `scoped at <short-sha>`
  (HEAD at scoping) in every routed entry's `notes` — the queue keeps moving
  after scoping, and the stamp lets build diff that range at pick-up instead
  of re-deriving the premise.
- **A "retire mechanism X" entry's blast radius is symbol scope, not path
  scope.** `rg` the retired function/type names across `tests/**`, not just
  `src/**`/`sdk/**` — a shared test helper (`tests/common/*.rs`) that
  round-trips through the retired API fans the edit out to every file
  importing that helper, invisibly to a source-only grep. Include those
  fan-out files in `files.edit` up front rather than letting build discover
  them one `cargo test` failure at a time.
- **Disjoint, or serialized — never both `open` over a shared file.** Build
  fans out pickable entries in parallel worktrees; two `open` entries editing
  the same file conflict at merge and revert the wave. If any path appears in
  two entries, serialize with `gate: { kind: "blockedBy", tag: "FIRST-TAG" }`.
- Honor the invariants in `specs/intent.md`: only decidable contract clauses
  become checks; behavior is delegated, never guessed. A derived layer never
  invents intent absent from its source.
- Keep `summary` a terse one-liner — the *what*; mechanics live in
  `files[].description`, `acceptance`, `tests[].asserts`, `notes`.

**Open questions** live in `open-questions.md`, never in pending; key each
`(slug)` so entries can declare `dependsOnForks: ["slug"]`. The file holds
OPEN forks only — when a fork resolves, encode the ruling and DELETE the
record; reconciliation evidence goes in the plan commit body. The file is
inlined into every tick: a dead line is a per-tick tax, a stale record a
latent misroute.

**state.md** is the scheduler's ledger, re-derived every tick, ~10 lines:

```
# Plan state
- Spec derived through: <sha>
- Audited through: <sha>
- Residue swept through: <sha>
- This tick: <the one job taken and its outcome>
- Queue: <one line — entries, gates>
Plan continues: yes — <the next live input> | no — <why quiet>
```

A cursor you did not advance this tick is copied forward **verbatim** — the
cursor lines must survive every rewrite, or the delta window falls back to
the last `plan:` commit and silently skips past un-derived work.

The marker is mechanical: `yes` iff an input below the one you serviced is
still live, `no` otherwise. With `no` and pickable entries, build takes over;
with `no` and none, the loop hibernates. Never re-emit an unchanged queue
with unmoved cursors under `yes`.

# FRICTION (optional — most ticks file nothing)

If something in THIS tick cost you disproportionate effort — a pitfall the
harness could have warned you about, a lengthy process, missing operational
knowledge — capture it: one new file `.flume/friction/plan-<slug>.md`, terse
(symptom, what it cost this tick, suggested fix). Check the directory first;
never re-file a filed friction. Humans drain it out of band. Never
speculative, never a substitute for the job — see `.flume/friction/README.md`.

# OUTPUT

One commit prefixed `plan:`. Write `.flume/plan/{pending.json,state.md,open-questions.md}`
and drain `.flume/inbox.md` when inbox is the job. The harness rejects the
commit if `pending.json` doesn't parse or you modify anything outside the
phase's writable paths.

**Field-length footgun — this reverts the whole tick.** Two fields have a hard
upper bound, enforced *after* you commit: **`summary` ≤200 chars** and **`notes`
≤500 chars**. If **any** entry violates either, the gate reverts the **entire**
commit — every cursor advance and reconciliation this tick is lost. Before you
finish, re-read every `summary` and `notes` and confirm each is under its cap;
a field near its limit is a smell — push detail into the unbounded fields.

<schema>
{{PENDING_SCHEMA}}
</schema>
