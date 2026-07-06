# Plan state

- **Phase:** reconciled the S6–S7 demolition tail against disk after CODEC-RETIRE
  (S5) shipped. Promoted **INSTALL-FRONT-DOOR (S6)** `blockedBy → open` and
  refreshed its drifted main.rs line ref (Guard authority arm now 350-372, the
  `AuthorLayer::load` at 356 — the entry said 375-381).
- **Last shipped:** CODEC-RETIRE / S5 (build 4033374 / chore d654ead). Verified on
  disk: `Command::Init`/`Command::Lift`/`import::init` are gone from main.rs, and
  the `[[member]]` *write* codec (`member_to_table`/`write_members`) is gone from
  compose.rs. Only the in-place *read* path (`parse_inplace_member`,
  `inplace_members`, `InPlaceMember`) + `AuthorLayer` survive — S7 (TEMPER-TOML-ZERO)
  retires them.
- **This tick:** confirmed S5 landed on disk (not the git log); refreshed the new
  head's line refs against `src/`; left S7's refs approximate (reconcile-when-head).
  Inbox empty — nothing to drain; no fork moved.
- **In flight:** one `open` head — INSTALL-FRONT-DOOR (S6). TEMPER-TOML-ZERO (S7)
  waits behind it (`blockedBy` — both touch the shared main.rs spine, so serialized,
  not parallel). PACKAGING-CHANNELS parked on human release setup.
- **What's next:** build drains S6; an S6 ship unblocks S7 on the following
  reconcile (its main.rs line refs need a refresh then). Human owns
  PACKAGING-CHANNELS release setup, the USPTO name screen, and the
  genre-fence-format workshop (cascade pilot).

Plan continues: no — S5 shipped, S6 is promoted to `open` with its line refs
refreshed truthfully, S7 stays serialized behind it on the shared main.rs, and the
inbox is drained. Building is how the queue drains from here.
