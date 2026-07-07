## Symptom
The plan loop is parked on human design forks (4 open questions gate the
kernel-recut arc), and the one pickable entry (VACATE-KIND-NOUN, `gate: open`,
disjoint file set) cannot reach build. The dispatcher runs build only on
`Plan continues: no`, but `planHonestyGate` reverts any `no` while specs/
commits sit past the spec cursor — and that cursor can't advance because the
recut (e842a32) is one giant commit that is "fully derived" only once every
fork resolves. So the marker is pinned to `yes`, the loop re-wakes plan each
tick to re-confirm "still parked," and no work moves.

## Cost this tick
The prior attempt tried to escape the spin by declaring `Plan continues: no`;
planHonestyGate reverted the whole commit (spec cursor trails). This tick spent
a full investigation re-deriving that every autonomous input is dead-ended —
spec-delta slices fork-blocked, residue sweep all fork-blocked or
citation-staleness-excepted — the loop's steady state until John acts, at ~one
fresh `claude -p` per tick.

## Suggested fix
A dispatch affordance for "parked on human, not done" — distinct from both
`no` (done, hand off) and `yes` (more autonomous work now). E.g. a
`Plan continues: parked` marker that neither reverts under a trailing cursor
nor re-wakes plan immediately: it hibernates until inbox/specs change, and/or
routes disjoint pickable entries to build when piecemeal shipping is
acceptable. Alternatively, if piecemeal recut shipping is NOT wanted (VACATE's
files overlap the un-derived requirement/nesting arc), state that rule so
VACATE is marked `blockedBy` the recut rather than advertised `open` — the
queue should not advertise pickable work the dispatcher can never reach.
