---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
paths: [".flume/plan/pending.json"]
---
# Pending-entry discipline

Binds every edit to `.flume/plan/pending.json` — the `plan` phase filing or
rewriting an entry, and any interactive session hand-editing the queue.

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
- **Widening a shared enumeration names its other consumers.** An entry
  that adds a variant, row shape, or member class to a shared concept —
  edges, members, template layers, lock rows, deletable things,
  discoverable paths — lists that enumeration's other consumers in its
  own `files[]` (`rg` the type's match sites), per
  `specs/process/engineering.md`, "A shared concept is one type": "who
  else reads this set?" is derivation's question, never the consumer's
  bug report.
- **Disjoint, or serialized — never both `open` over a shared file.** Build
  fans out pickable entries in parallel worktrees; two `open` entries editing
  the same file conflict at merge and revert the wave. If any path appears in
  two entries, serialize with `gate: { kind: "blockedBy", tag: "FIRST-TAG" }`.
- Honor the invariants in `specs/intent.md`: only decidable contract clauses
  become checks; behavior is delegated, never guessed. A derived layer never
  invents intent absent from its source.
- Keep `summary` a terse one-liner — the *what*; mechanics live in
  `files[].description`, `acceptance`, `tests[].asserts`, `notes`.

**Field-length footgun.** `summary` ≤200 chars, `notes` ≤500 chars, enforced
by a gate *after* commit — a violation on any single entry reverts the
**entire** tick, every cursor advance and reconciliation included. A field
near its limit is a smell — push detail into the unbounded fields.
