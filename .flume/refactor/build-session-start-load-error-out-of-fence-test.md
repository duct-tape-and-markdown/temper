# SESSION-START-LOAD-ERROR-BYPASSES-REPORTER needs one out-of-fence test edit

## Surface

The entry's fix — lower a load failure to one `error` Diagnostic in `main.rs`
and run the *selected* reporter over it — is fully worked out and green on
every suite except one assertion the entry's fence does not reach:

- `tests/gate_fail_loud.rs:531-536` —
  `a_lock_carrying_two_requirement_rows_with_the_same_name_refuses_loud`
  asserts `findings.is_empty()`, with the comment *"a fatal duplicate key
  error produces no findings (the check exits before collecting them)"*.

That assertion **pins the bug the entry removes**. The entry's notes say the
gap "generalizes past settings.json": every load error bypasses every
reporter, so the lowering cannot be narrowed to the manifest path — and it
cannot be narrowed to the session-start reporter either, because
`specs/distribution.md` ("The placements and their enforcement modes") holds
that every reporter renders **one diagnostic source**. Lowering for one
placement and aborting for the others forks that source.

So the duplicate-key case now yields exactly one finding, still exit non-zero:

```
::error title=gate.load-fault::.: duplicate identity key `docs` — the gate did not run, so nothing in this harness was checked.
```

The suite is otherwise whole: `cargo test --no-fail-fast` fails this one test
and nothing else; `cargo fmt --all --check` and
`cargo clippy --all-targets -- -D warnings` are green.

## Observed at

13b5bdaa (HEAD when observed).

## Suggested consolidation

Re-scope the entry with `tests/gate_fail_loud.rs` added to `files.edit`. The
edit is one assertion, inverted to the post-fix contract — the test's subject
(refuse loud, never silently shadow one requirement with the other) is
unchanged and still worth pinning:

- keep `assert!(!success, …)`;
- replace `assert!(findings.is_empty(), …)` with exactly one
  `gate.load-fault` finding naming the colliding key `docs`
  (`common::findings_for(&findings, "gate.load-fault")`);
- update the test's comment: the fatal error is no longer discarded — it is
  the run's single diagnostic, which is what makes the advisory placement
  able to speak it.

The rest of the entry lands unchanged: `check::load_fault` in `src/check.rs`,
the `unwrap_or_else` lowering at `src/main.rs`'s reporter dispatch, a
`session_start` doc note in `src/reporter.rs`, plus the two named regressions
in `tests/session_start.rs` and `tests/cli.rs`.
