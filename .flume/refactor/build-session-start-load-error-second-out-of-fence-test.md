# SESSION-START-LOAD-ERROR-BYPASSES-REPORTER needs a SECOND out-of-fence test edit

## Surface

The 09-07 fence widening drained one out-of-fence assertion
(`tests/gate_fail_loud.rs`, now in the fence and updated green). There is a
second one of the identical class, and the fence does not reach it:

- `tests/toml_document.rs:253-257` —
  `a_malformed_document_fails_the_run_rather_than_gating_against_no_fields`
  asserts `run.findings().is_empty()` with the message *"the clause never
  judges a document that would not parse"*.

Same shape as the gate_fail_loud assertion: an empty finding set stands in for
"the load error aborted the run". Once the load failure lowers to one
`error` Diagnostic, the set is no longer empty — it holds exactly one
`gate.load-fault` finding — so the assertion fails and the commit reverts
whole.

The test's own subject is untouched by the fix and must stay: a malformed
document is never gated against invented absent fields, so the `knob.mode`
`required` clause still never fires. What changes is only the stand-in for
"aborted": from *no findings at all* to *one `gate.load-fault` finding and no
clause finding*. Suggested replacement for the one assertion:

```rust
    let findings = run.findings();
    assert_eq!(
        findings.len(),
        1,
        "the clause never judges a document that would not parse — the run has one \
         finding, not a verdict over invented absent fields: {}",
        run.output
    );
    assert_eq!(
        common::findings_for(&findings, "gate.load-fault").len(),
        1,
        "and that one finding is the lowered parse failure: {}",
        run.output
    );
```

Spelled by count rather than by the `knob.mode` clause address, because the
suite never spells that address anywhere (its sibling cases scrape
`output.contains("mode")`, which the lowering now defeats — the TOML parse
error text quotes the offending `mode =` line).

(The sibling assertion at :248 — `run.output.contains(
"temper::toml_document::malformed")` — already passes: the lowering carries
the report's `miette` code into the diagnostic message, conserving the whole
report rather than summarizing it.)

Why the previous capture missed it: that tick ran `cargo test`, which stops at
the first failing target, and `gate_fail_loud` sorts before `toml_document`.
This tick ran `cargo test --no-fail-fast`, so the set is now known to be
complete — with `tests/toml_document.rs` added to the fence, the entry lands
with every gate green (`cargo fmt`, `cargo clippy -D warnings`, `cargo test
--no-fail-fast`, `pnpm --dir sdk test`, `cargo doc` all verified green this
tick against the working fix).

## Observed at

7ad33c00 (HEAD when observed) — the entry's own fence at that commit.

## Suggested consolidation

Widen the entry's fence by exactly one path — `tests/toml_document.rs` — and
name the one assertion above. No other file is needed; the rest of the entry
is unchanged and its work is already validated.
