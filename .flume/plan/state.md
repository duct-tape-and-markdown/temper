# Plan state

- **Phase:** demolition-wave draining. HEAD ecf060e (+ this plan commit). Inbox
  empty; no corpus change since last reconcile.
- **Last shipped:** MANIFEST-MACHINERY-RETIRE (build dbf72bb / chore ecf060e) —
  the gate now reads its whole assembly off the lock's declaration rows
  (`drift::read_declarations`, main.rs:116), and the `temper.toml`-manifest-as-corpus
  machinery is retired. Verified on disk: the committed `.temper/lock.toml` now carries
  52 declaration rows (members + `[[declaration.kind]]`), so the earlier zero-row
  session-half is discharged.
- **This tick:** unblocked **KIND-PACKAGE-PARSE-RETIRE** (MANIFEST-MACHINERY-RETIRE
  shipped, its predecessor). Reconciled its blast radius, which was stale: the KIND.md/
  PACKAGE.md parse is still live and on the gate path (compose.rs `PackageResolver`/
  `effective`, main.rs `packages_dir`/`kinds_dir` + `CustomKind::load`, check.rs/import.rs
  kind load), so those four files were added to `files.edit` alongside the six already
  listed. Atomic entry — the parse can't be half-retired in Rust.
- **In flight:** one pickable `open` head — KIND-PACKAGE-PARSE-RETIRE. Serial tail:
  EXPLAIN-UNIFY (terminal leaf, blockedBy it). PACKAGING-CHANNELS parked on human creds.
- **What's next:** build drains KIND-PACKAGE-PARSE-RETIRE; plan unblocks EXPLAIN-UNIFY
  on the following green tick. **Session half still open:** the temper.toml→lock producer
  survives as transitional; retiring it (→ SDK `harness.ts` producing the dogfood lock)
  is the pending fence-side migration the session hand-authors when the producer retires.

Plan continues: no — queue reconciled to the ship, one pickable `open` head exists
(KIND-PACKAGE-PARSE-RETIRE), inbox empty. Building drains the queue now.
