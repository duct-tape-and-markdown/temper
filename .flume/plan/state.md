# Plan state

- **Phase:** reconcile. Verified on disk: **RE-ADD shipped** — `drift::re_add` +
  `Command::ReAdd` (`main.rs:362`), the third drift direction (on-disk→surface)
  over `tests/readd.rs`. All three drift directions (`apply` / `diff` / `re-add`)
  now on disk.
- **Last shipped:** RE-ADD (`62380c8`). Its dependent SESSION-START-GATE unblocked.
- **Filed / pickable (1):** **SESSION-START-GATE** (`open`) — the advisory
  session-start gate: a new `src/reporter.rs` (`claude-session-start` payload) + a
  one-shot check verb in `main.rs` that imports a harness path internally, runs the
  same by-kind gate `check` runs, and emits the SessionStart JSON (advisory, exit 0).
  Verified unshipped: no `src/reporter.rs`, no `tests/session_start.rs`.
- **Frontier:** the SARIF + GitHub-annotation reporters (sibling on the new reporter
  family), `bundle`/`install`, and more built-in harness kinds
  (agent/hook/command/MCP/settings/plugin) are the unbuilt areas — each touches
  `main.rs` (+ `reporter.rs` or `import.rs`), so all serialize on that shared surface,
  not filed `open` now. Spec-kind `references-resolve` waits on
  `(reference-id-normalization)`; `decisions-name-alternatives` on `(decision-marker-predicate)`.
- **Inbox:** empty (nothing to drain). Open questions unchanged (no fork resolved this tick).

Plan continues: no — queue reconciled, RE-ADD dropped as shipped, SESSION-START-GATE
unblocked to `open` and pickable, inbox empty; hand to build.
