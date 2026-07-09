# Plan state

- Spec derived through: f87cc0c
- Audited through: dd7517a
- Residue swept through: 2f1c259
- This tick: Residue sweep (job 4). `git diff --stat 8dfceee..HEAD` (six
  commits: 09fee9f, 8a2459f, a32a45f, 942d28a, cb18a56, 2f1c259) touches only
  `.flume/**`, `.temper/lock.toml`, `.temper/memory/CLAUDE.md`, `CLAUDE.md` —
  zero changes to src/, tests/, or sdk/ since the prior sweep, so no new
  residue can exist there. Both standing accepted debts reread and
  reconfirmed live verbatim: tests/session_start.rs:121-146 still writes
  `+++`-format `.temper/kinds/spec/KIND.md` + `.temper/packages/spec/PACKAGE.md`
  fixtures; sdk/src/builtins.ts:295-395 still doc-comment-cites the deleted
  `packages/{rule,memory}.anthropic/PACKAGE.md` files (agents-md.memory's own
  citation rides the still-open `(agents-md-builtin-kind)` fork, not this
  debt). Grepped `published_requirements` (no hits) and `skill.anthropic`
  (only its legitimate shipped-kind-label uses) — no other retired
  vocabulary found live. No new fileable gap; pending.json unchanged.
- Queue: INSTALL-HOOK-APPEND-COVERAGE (open, next), PACKAGING-CHANNELS
  (parked). Disjoint — open entry touches only tests/install.rs.

Plan continues: yes — quiet closing pass (job 5) is next; residue cursor now
at HEAD, all four prior inputs current.
