# Plan state

- **Phase:** reconcile. HEAD e38cb59.
- **Last shipped (trunk):** EMBED-BUILTIN-KINDS — `kinds/{skill,rule}/KIND.md`
  embedded by `build.rs`, parsed via `builtin_kind::definition` into a `CustomKind`
  through the project-authored `KIND.md` path (verified on disk: `kinds/` tree,
  `build.rs` walks it, `src/builtin_kind.rs` present).
- **This tick:** verified BUILTIN-EXTRACT-GENERIC unbuilt — `skill_features`/
  `rule_features` still at `extract.rs:238/296`, called at `main.rs` 405–408/541–545;
  `kind::Unit` + `Extraction::extract` exist for the adapter face. Sole change: its
  gate flipped `blockedBy EMBED-BUILTIN-KINDS` → `open`. Inbox empty; no forks moved.
- **Pickable now:** BUILTIN-EXTRACT-GENERIC (open), the last entry of the
  extraction-unification wave. AGENT-KIND deferred; PACKAGING-CHANNELS /
  COMMUNITY-DOCS parked. Sole live OPEN fork: (edge-representation-unify).

Plan continues: no — BUILTIN-EXTRACT-GENERIC is pickable and sole-open (nothing
shares its files); hand to build.
