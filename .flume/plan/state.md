# Plan state

- **Phase:** demolition-wave draining. HEAD 9a250ed (+ this plan commit). Inbox
  empty; no corpus change since last reconcile; no fork resolved.
- **Last shipped:** CLI-COLLAPSE (build da84b69 / chore 9a250ed). Verified on
  disk: `src/main.rs` now carries the collapsed face
  init·check·emit·guard·install·bundle·schema — `Command::Guard` reads the
  `PreToolUse` stdin at the author's declared posture, and the four read verbs
  (why/requirements/impact/context) are gone from the CLI (EXPLAIN-UNIFY re-homes
  their traversals under `explain`).
- **This tick:** CLI-COLLAPSE shipping unblocks its successor — **MANIFEST-
  MACHINERY-RETIRE** flips `blockedBy → open`. Verified unshipped: `gate()` still
  loads `assembly_artifacts`, calls `merge_assembly`, and reads `roster` + the
  `temper.toml` layer. One-line reconcile; nothing else moved.
- **In flight:** MANIFEST-MACHINERY-RETIRE is the one pickable `open` head; the
  tail stays serialized behind it (each defines the seam the next consumes) —
  KIND-PACKAGE-PARSE-RETIRE → EXPLAIN-UNIFY (terminal, fork-gated on
  `(explain-target-disambiguation)`). PACKAGING-CHANNELS parked on human creds.
- **What's next:** build drains from MANIFEST-MACHINERY-RETIRE; plan re-reconciles
  per green tick and unblocks the next link. Human halves: explain's target
  resolution; PACKAGING-CHANNELS creds; the session's fence-side migration.

Plan continues: no — the queue is reconciled to the shipped CLI-COLLAPSE entry, a
single pickable `open` head exists, inbox is empty. Building drains the queue now.
