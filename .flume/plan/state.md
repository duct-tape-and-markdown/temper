# Plan state

- **Phase:** reconcile — realign tick. The corpus was reconciled to the
  package/assembly/kind model (`5b06eae`) *after* the last plan commit
  (`8117825`), so the inherited queue spoke the retired vocabulary. This tick
  realigns it. Verified on disk: `lock.toml` rename already shipped
  (import.rs:57), but code still uses `template`/`contract`-bundle (no `package`
  concept), embeds `contracts/*.toml` bound by hardcoded kind name (main.rs:59/65),
  types requirements by `contract`, and declares custom kinds *inline* in
  `[kind.*]`. No `.temper/`/`temper.toml` on disk — the self-application surface
  was parked at `cb52cc3` "pending code reconciliation." Inbox empty.
- **Last shipped:** `lock.toml` rollup rename (RENAME-ROLLUP-LOCK, on disk).
- **In flight:** none.
- **Filed this tick:** PACKAGE-MODEL-RECONCILE (parked) — the code↔spec migration
  (`template`→`package`, embed built-in packages from `.temper/packages/`, retire
  `contracts/`, custom kinds under `.temper/kinds/`), blocked on the human
  un-parking `.temper/` + the new `(package-surface-sequencing)` fork. Reconciled
  the three carried entries to the new model (fixed AGENT-KIND's stale cite +
  `contracts/agent.toml` shape; noted COVERAGE-CUSTOM-KIND downstream of
  `.temper/kinds/`).
- **Pickable now (0):** every entry is parked/deferred. PACKAGE-MODEL-RECONCILE
  parked (human un-park + sequencing), COVERAGE-CUSTOM-KIND deferred (priority +
  downstream), PACKAGING-CHANNELS parked (release creds), AGENT-KIND deferred
  (reframe).
- **Blocked frontier (forks, unchanged):** `(package-surface-sequencing)` (new)
  gates the whole migration; `(decision-marker-predicate)` + `(reference-id-normalization)`
  gate the spec-kind decisions-name / references-resolve clauses; `(read-verbs)`
  gates `why`/`requirements` CLI verbs — all await a human decision.

Plan continues: no — queue realigned to the reconciled spec model, inbox empty,
and the frontier (the package/assembly migration) is human-held: it rests on
un-parking `.temper/` and settling `(package-surface-sequencing)`. Nothing for
build to pick and nothing more to plan until that decision lands.
