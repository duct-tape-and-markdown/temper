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
!`ls spec/RELEASE-*.md 2>/dev/null | sort || echo "(no spec)"`
</spec-corpus>

<active-target>
!`cat $(ls spec/RELEASE-*.md 2>/dev/null | sort | tail -1) 2>/dev/null || echo "(no active target)"`
</active-target>

<src-tree>
!`find src -name '*.rs' 2>/dev/null | sort`
</src-tree>

<cargo-check>
!`cargo clippy --all-targets -- -D warnings 2>&1 | tail -20 || true`
</cargo-check>

<recent-commits>
!`git log -n 10 --oneline`
</recent-commits>

# TASK

Re-derive the plan artifacts from current disk reality. The active ship target
is the newest `spec/RELEASE-*.md` (see `<active-target>`); earlier release files
are frozen.

1. **Reconcile** every existing pending entry against the spec section named in
   its `per` cite and the files named in `files`. A stale entry gets a full
   rewrite, never a patch. Drop entries whose work has shipped (verify on disk —
   read `src/`, never grep the git log).

2. **File new entries** for the gap between the active target and current `src/`:
   - A spec requirement with no implementation → a new `open` entry with a `per`
     cite into the active `RELEASE-*.md`.
   - A `cargo clippy`/`cargo test` failure visible above → a `MAINTAIN-*` entry
     at the top of pending, deduped by signature.
   - A gated entry whose blocker has shipped → promote to `gate.kind = "open"`.
   Each entry's `files` must name the Rust modules it creates/edits truthfully —
   the fanout partition reads `files.edit[].path` to run disjoint entries in
   parallel. Keep entries small and disjoint where possible.

3. **Drain `.flume/inbox.md`.** Route each line into pending (with a `per` cite),
   open-questions (no clean cite, or a product/UX fork), or accepted debt (noted
   in the commit body). Remove drained lines.

4. **Re-derive `state.md` from scratch** (~5 lines: phase, last shipped tag,
   in-flight work, what's next). Never carry forward.

5. **Open questions** belong in `open-questions.md`, never in pending. If a
   candidate entry can't carry a clean `per` cite, it is an open question. Key
   each with a `(slug)` so entries can declare `dependsOnForks: ["slug"]`.

6. **Continuation marker.** End `state.md` with `Plan continues: yes — <reason>`
   if the delta exceeds what one tick handled well, else `Plan continues: no`.

# OUTPUT

One commit prefixed `plan:`. Write:

- `.flume/plan/pending.json` — JSON array conforming to the schema below.
- `.flume/plan/state.md` — ~5 line markdown ending with the continuation marker.
- `.flume/plan/open-questions.md` — markdown.

The harness rejects the commit if `pending.json` doesn't parse or you modify
anything outside the phase's writable paths.

<schema>
{{PENDING_SCHEMA}}
</schema>
