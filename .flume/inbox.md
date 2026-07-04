<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- **toml_edit output style is version-unstable — treat bumps as contract
  events** (interactive session, 07-03, web-verified: toml_edit CHANGELOG
  documents a breaking default-output change at 0.22.25, 2025-04-25 —
  "Reduced escaping in strings"; three earlier style-churn precedents). The
  SDK's byte-parity emitter (272b4f4) mirrors 0.22.27 behavior, so any
  future toml_edit bump is a latent silent parity break. Standing rule: a
  toml_edit version bump entry must re-run the SDK byte-parity fixtures and
  reconcile both sides in the same entry — never bump-and-ship. (The
  structural retirement of this tax — single-writer-per-format — rides the
  TS-primary ceremony, human-gated.)
