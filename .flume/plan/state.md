# Plan state

- **Phase:** reconcile. HEAD b862bf0.
- **Last shipped:** DIRECTIVE-BACKING-BASE-DIR (build 590b3e0 / chore e6be469).
- **This tick:** **drained the inbox — WALK-IGNORE-DISCIPLINE un-parks to `open`.**
  Both parked blockers are closed on disk: the ignore discipline is now a written
  spec Decision (`specs/architecture/20-surface.md:317`, "discovery respects ignore
  rules; the backing set reads raw disk" — two sets, two rules, never merged) and
  the `ignore` crate is sanctioned in CLAUDE.md's tech stack. Re-cited the entry
  off the stale per-project Decision onto the ignore-rules Decision that actually
  owns the intent. Verified on disk: `collect_glob` (src/import.rs:417) is the
  discovery walk to prune; `repo_file_set` (src/main.rs:1001) is the backing set
  to leave raw; `ignore` is NOT yet in Cargo.toml (build adds it, now sanctioned).
  Other 3 entries unchanged (re-verified accurate last tick).
- **Pickable now:** **WALK-IGNORE-DISCIPLINE** (the sole `open` entry — build can
  ship it). Remaining 3 stay human-gated: PACKAGING-CHANNELS parked (release
  creds); EXTRACTION-VOCAB-GAPS + AGENT-KIND deferred (no consumer).
- **After it drains:** the human flips both memory kinds' `governs` to the
  any-depth glob (standing ruled ceremony, curated embeds — human territory).

Plan continues: no — inbox drained, the un-parked entry is `open` and pickable;
hand to build.
