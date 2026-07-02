# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox empty; no new fork.
- **Last shipped (trunk):** SESSION-START-CUSTOM-KIND (c57342b/8839794) — the one-shot
  session-start gate resolves custom kinds from the harness `.temper`. HEAD 8839794; tree clean.
- **In flight / anomaly:** none. Verified on disk: `contracts/{skill.anthropic,rule}.toml`
  still embedded (main.rs:67/73, bundle.rs:122/126,210/216); `packages/{skill.anthropic,
  rule.anthropic}/PACKAGE.md` authored; `Contract::load_package` at contract.rs:586;
  `Cargo.toml` still `license = "MIT"`; `.github/CONTRIBUTING.md` present; no LICENSE-*/AGENTS.md.
- **This tick:** SESSION-START-CUSTOM-KIND shipped and left the queue, so EMBED-BUILTIN-PACKAGES's
  blocker no longer exists — flipped it `open` and corrected its stale main.rs refs (59/65 → 67/73).
- **Pickable now (3 disjoint / parallel-safe):** OFFERING-LICENSE (Cargo.toml + LICENSE-*),
  AGENTS-MD (AGENTS.md), EMBED-BUILTIN-PACKAGES (now sole main.rs editor: main.rs + build.rs +
  bundle.rs + contract.rs, retires contracts/). Parked: PACKAGING-CHANNELS (human release creds).
  Deferred: AGENT-KIND (priority). Forks: read-verbs + KIND-* remain RESOLVED-but-unfiled records,
  read-verbs still tail-gated behind EMBED (the last surface-language migration step).

Plan continues: no — the only reconciliation this tick was unblocking EMBED-BUILTIN-PACKAGES now
that SESSION-START-CUSTOM-KIND shipped; three disjoint `open` entries are pickable and the inbox is
empty. Building drains it.
