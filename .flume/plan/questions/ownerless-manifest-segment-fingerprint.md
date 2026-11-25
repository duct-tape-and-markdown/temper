# How does emit see a hand edit to its own segment of a settings file no member owns?

Driver: build note of EMIT-NEVER-REAPS-A-HAND-REGISTRATION-FROM-AN-UNDECLARED-SETTINGS (cf012af1). `src/drift.rs` `unrecorded_registration_entries` refuses only when a key's on-disk hook groups outnumber the lock's registration rows. Registration rows carry no bytes. So when a hand edit changes an emitted group, or a hand group replaces one, the count stays the same and emit overwrites it with no finding. `specs/model/pipeline.md` "Emit" says "nothing authored is lost" and "a drifted ownerless file is a finding, never a silent delete", and "Drift" says a hand-edited projection is drift by hash. Here that hash does not exist: per 0050, emit fingerprints only a manifest with a container member.

The corpus does not decide the lock shape. 0050's Rejected list turns down "a synthetic rollup row under a non-kind label" and "lock rows carrying hook commands", and those are the two fixes the note proposes.

Options: (1) each registration row carries a byte fingerprint of the group it rendered. That is a hash, not a command, so it may fall outside 0050's second rejection. (2) A container-less manifest fails closed: emit refuses any unmatched group. Today it refuses on a count. (3) Accept the gap: an adopter who hand-edits declares `settings` (0050), and the ownerless case stays a known hole.

Recommendation: 1. It brings back drift-by-hash, which "Drift" promises for every projection, at the grain emit already writes. (2) refuses every emit after any rename of a hook member. Sections: `specs/model/pipeline.md` "Emit", "The lock"; 0050; 0007.
