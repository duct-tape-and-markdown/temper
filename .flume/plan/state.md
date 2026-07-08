# Plan state

- Spec derived through: bdc938d
- Audited through: 1e5d7ec
- Residue swept through: 31cd09a
- This tick: Spec delta (job 2). Derived bdc938d (0014, command+agent kinds,
  registration as channels) into three serialized entries downstream of
  EMBEDDED-FENCE-MEMBER-NOUN: REGISTRATION-CHANNELS (scalar→channel-set recut,
  `user-invoked` joins, modulating booleans become declared fields, skill
  profile re-verify), COMMAND-KIND (skill schema's second placement), AGENT-KIND
  (new kind + named-field identity, the third mode + name charset). All share
  builtin_kind.rs/builtins.ts/builtin_lock.toml → blockedBy chain. Cursor
  fa7f87c → bdc938d (0012 already routed, 0013 fork-routed, 0014 now derived).
- Queue: 5 — EMBEDDED-FENCE-MEMBER-NOUN open/pickable; REGISTRATION-CHANNELS→
  COMMAND-KIND→AGENT-KIND serialized behind it; PACKAGING-CHANNELS parked. Only
  one open entry, so the fence stays disjoint.

Plan continues: yes — residue sweep (31cd09a..HEAD): corpus moved (0012/0014)
while code held still; verify each residue class is filed/riding/fork-routed and
advance the cursor to HEAD.
