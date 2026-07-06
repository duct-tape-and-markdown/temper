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
!`for f in $(find specs -name '*.md' 2>/dev/null | sort); do echo "===== $f ====="; cat "$f"; echo; done || echo "(no specs)"`
</spec-corpus>

<spec-delta>
!`ANCHOR=$(git log -1 --format=%h --grep='^plan:' 2>/dev/null); if [ -n "$ANCHOR" ]; then echo "specs/ commits since the last plan tick ($ANCHOR):"; git log --format='%h %s' "$ANCHOR"..HEAD -- specs/; echo; git diff --stat "$ANCHOR"..HEAD -- specs/ | tail -15; else echo "(no prior plan commit — treat the whole corpus as the delta)"; fi`
</spec-delta>

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

`specs/` is the evergreen source of truth (see `specs/process/90-spec-system.md`). It is
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

   **Walk the spec delta first.** `<spec-delta>` lists every `specs/` commit
   since the last plan tick. Read each one's diff (`git show <sha> -- specs/`)
   before reconciling anything else: ratified intent changes there before it
   changes anywhere, and a delta nobody operationalizes is how residue
   accumulates unseen. (Real failure: a day of ratified demolitions — the
   package noun, the reachability dial, the requirement recut — sat un-derived
   while the queue read as quiescent.) **An empty delta licenses nothing**: it
   means no *new* intent this window, not no un-derived intent — the residue
   sweep below runs every tick regardless, because accumulated debt predates
   any window by definition. (Real failure: the very next tick after the delta
   block landed reasoned "delta empty → intent unmoved → idle" and skipped the
   sweep over a corpus carrying three un-derived demolitions.)

   **Re-test every gate, every tick.** A `parked` reason, a `blockedBy`, an
   open-question's "rides X" routing, and prior state's "human-gated" labels
   are notes from a past tick, never standing law — each names a condition,
   and you verify the condition on disk NOW. If the blocker has shipped (the
   front door is built, the fork is resolved, the foundation landed), the
   classification is stale and the work behind it is derivable this tick.
   (Real failure: the package-noun and dial cuts sat classified "behind the
   SDK-primary front door" for a full day after that front door shipped.)

   **Residue is a gap.** A Decision's rejected alternative, a retired noun, or
   a residue paragraph that names code still present in `src/`, `tests/`, or
   `sdk/` — with no pending entry operationalizing the cut — is a fileable gap
   exactly like missing behavior. Cite the residue text in `per`, name the
   living symbols in `files[].description`. The corpus naming a demolition is
   intent; an entry is what makes it work.

   **Comments are residue carriers too.** The sweep's greps cover comment
   text, not just symbols: a retired noun living in a doc comment, and a
   comment quoting a spec section title that no longer exists in `specs/`
   (grep the quoted title against the corpus's headings), are the same
   fileable staleness — route them into whichever entry already opens that
   file (`.claude/rules/rust.md`, the exit clause), or the standing
   comment-stock sweep if none does. Comments are the one surface no cargo
   gate re-tests; this sweep is their only reconcile moment.

   **One entry = one gate-sized commit, comfortably under 200k tokens of build
   work.** If a scope needs lettered sub-parts, an internal task list, or
   bundles an implementation with its consumers and a re-target, it is not one
   entry — it is a `blockedBy` chain. Split until each link is a single idea
   build can implement, gate, and commit in one contained session. (Real
   failures: INSTALL-FRONT-DOOR filed whole with (a)(b)(c) sub-slices ran a
   45-minute tick; TEMPER-TOML-ZERO shipped under-scoped and forced a
   mid-chain split — file the split up front.)
   Scope `files` to the truthful **blast radius** — include existing tests/snapshots
   a change will break — so build reaches green inside the planned scope instead
   of discovering the ripple mid-tick.
   Every path in `files` — `new[].path`, `edit[].path`, and `retire` (bare
   strings) — is a **repo-relative file path**; the fence gate glob-matches all
   three against build's writable paths. `retire` means "this FILE is deleted."
   Retiring a symbol *within* a surviving file (a function, an enum variant) is
   an `edit` to that file, described in its `description` — never a `retire`
   entry.
   **Disjoint, or serialized — never both `open` over a shared file.** Build fans
   out every pickable entry *in parallel worktrees* and merges the wave together;
   if two `open` entries edit the **same file** (even different regions — touching
   the same struct/enum/`match` is enough), the merge **conflicts**, the whole wave
   reverts, and the queue spins forever re-filing work that can never land. Before
   you leave entries `open`, diff their `files` blast radii: if any path appears in
   two of them, they are **not** parallel-safe. Make them genuinely disjoint, or
   **serialize** them — give the later one `gate: { kind: "blockedBy", tag:
   "FIRST-TAG" }` so build picks them one at a time. A shared file is the signal;
   `blockedBy` is the mechanism. (Real failure: GOV-RANGE and GOV-COUNT both edited
   `src/compose.rs` and spun the loop to a standstill.)
   Honor the law in `specs/intent/00-intent.md`: only decidable contract clauses become
   checks; behavior is delegated, never guessed. Do not re-introduce heuristic
   rules the corpus rejected.
   Keep `summary` a **terse one-liner** — the *what*, not the *how*. It is hard-
   capped (see the footgun below); the file-by-file mechanics belong in
   `files[].description`, `acceptance`, and `notes`, never crammed into `summary`.

3. **Drain `.flume/inbox.md`.** Route each line into pending (with a `per` cite),
   open-questions (no clean cite, or a product fork), or accepted debt (noted in
   the commit body). Remove drained lines.

4. **Re-derive `state.md` from scratch** (~5 lines: phase, last shipped, in
   flight, what's next). Never carry forward.

5. **Open questions** belong in `open-questions.md`, never in pending. If a
   candidate entry can't carry a clean `per` cite, it is an open question. Key
   each with a `(slug)` so entries can declare `dependsOnForks: ["slug"]`.
   **The file holds OPEN forks only — prune on reconcile:** when a fork
   resolves, encode the ruling (corpus Decision or your commit body) and
   DELETE the record; never append reconciliation DATUMs to a resolved
   record — narrate those in the plan commit body. Git is the archive. The
   file is inlined whole into your own prompt: its length is your context
   tax, and a stale record is a latent misroute.

6. **Continuation marker.** End `state.md` with `Plan continues: yes — <reason>`
   ONLY if there is concrete *additional plan work* left this turn (an undrained
   inbox, unreconciled drift, entries still to rewrite, or a derivation cursor
   below). Otherwise `Plan continues: no`. **Bias hard toward `no`:** once the
   queue is reconciled and any `open` entry exists, hand to build — building is
   how the queue drains. Never re-emit a queue identical to the last `plan:`
   commit and continue; if your reconciliation produced no change and pickable
   work exists, you are done — write `Plan continues: no` and let build run.
   Re-planning an unchanged queue is the failure mode, not diligence.

   **Derivation may span ticks — like the work you slice for build, yours is
   sliced too.** A large sweep (a ceremony's worth of spec delta, a many-file
   reconcile) does not have to land in one tick: file what this tick derived,
   record a cursor in `state.md` (`sweep at: <spec file / commit sha>`), and
   write `Plan continues: yes — sweep resumes at <cursor>`. Progress is filing
   entries or advancing the cursor; an identical queue with an unmoved cursor
   is the spin the bias-toward-`no` exists to kill.

You do NOT write `specs/` — intent is human-authored. Surface spec ambiguity as
an open question; never silently fill it.

# OUTPUT

One commit prefixed `plan:`. Write `.flume/plan/{pending.json,state.md,open-questions.md}`
and drain `.flume/inbox.md`. The harness rejects the commit if `pending.json`
doesn't parse or you modify anything outside the phase's writable paths.

**Field-length footgun — this reverts the whole tick.** Two fields have a hard
upper bound, enforced *after* you commit: **`summary` ≤200 chars** and **`notes`
≤500 chars**. If **any** entry violates either, the gate reverts the **entire**
commit and **all** your reconciliation work this tick is lost — not just the
offending entry, and not just the offending field. Before you finish, re-read
**every `summary` (≤200) AND every `notes` (≤500)** and confirm each is under its
cap. A field near its limit is a smell — shorten it and push detail into the
*unbounded* fields (`files[].description`, `acceptance`, `tests[].asserts`). One
long string should never cost the tick. (Real failures: a 200+ `summary`, then
later a 500+ `notes`, each silently nuked a full plan tick.)

<schema>
{{PENDING_SCHEMA}}
</schema>
