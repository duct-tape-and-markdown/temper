# Gated locus-declared finding: guard twin fixed too

The entry's scan (`rg "loads it" src/`) found one more finding with the same
claim: `temper guard`'s pending-write `undeclared_locus_message`
(src/install.rs). It now reads the pending path's absence gate as well, and the
wording lives in one function, `drift::locus_loader_claim`, so check and guard
use the same text. The shut-gate sentence that reachability prints is now
`kind::AbsenceClosure::reason`, shared by `gate::absence_gated` and both
locus-declared findings.

`CustomKind::absence_gate` returns the typed state (`AbsenceGateState::Open(scope)`
or `Shut(closure)`). `absence_closer` is now a thin wrapper over it.

Not modeled: the project-instructions setting that loads both files and bypasses
both gates (builtins.md, "memory"). Nothing in src/ reads it yet, so a harness that
sets it still sees "gate is shut" wording. That is the open half of the same line.
