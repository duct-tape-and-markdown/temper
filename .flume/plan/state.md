# Plan state

- Spec derived through: 5945405
- Audited through: 2b82ebd
- Residue swept through: 2b82ebd
- This tick: Residue sweep 9148eb3..HEAD. Verified the prior sweep's rider landed:
  e6c7f60 retargeted emit.ts's two stale `specs/architecture/20-surface.md` cites to
  pipeline.md "Emit" — resolved. New build code (mention edge, explain resolver,
  scaffold) corpus-consistent. One residue class found, non-routable: drift.rs:262/269
  docstrings still say "five families" while the mention edge shipped `mentions` as the
  sixth declaration family (Declarations struct + lock `mention` table + SDK
  declarations.ts already say "six"). Pure comment staleness — never standalone, never
  the queue's only new work; no pending entry opens drift.rs, so it awaits the next
  drift.rs touch (rust.md exit-clause / next sweep). Ship audit quiet: 0fa1cab, 2b82ebd
  touch no code — audit cursor advanced to HEAD. kind.rs/read.rs `15-kinds`/`20-surface`
  are fixture source_paths, not cites — left, per prior sweep.
- Queue: 3 — PATH-SEP-NORMALIZE open (install.rs/document.rs), GUARD-OWNPATH blockedBy
  PATH-SEP-NORMALIZE (shared install.rs), PACKAGING-CHANNELS parked (human release creds).

Plan continues: no — every input current (inbox empty, no spec delta past 5945405,
ship audit + residue sweep both at HEAD). PATH-SEP-NORMALIZE is open and pickable;
build takes over.
