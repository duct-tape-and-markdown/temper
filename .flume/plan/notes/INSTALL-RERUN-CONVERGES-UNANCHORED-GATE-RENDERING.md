# Unanchored gate rendering converges on install re-run

Shipped. `render_retired_gate_command` became `retired_gate_renderings` in
src/install.rs: it returns both prior byte forms, the 0.0.26 anchored
exit-127 form and the <=0.0.24 bare form (the same body with no `cd`
prefix, checked against `git show v0.0.24:src/install.rs`). `gate_rendering`
now treats the anchor as optional before the probe, and both forms map to
`GateRendering::Retired`. So the unrepresented splice, the represented
`Retired` outcome and the stale report cover the old form with no new
branch.

Tests: the two retired-rendering tests became helpers parameterized by the
respelling, and the unanchored titles call them. The pin's
`gate-foreign-unanchored` case was exactly the v0.0.24 bytes, so it now
recognizably converges. It was replaced with two near misses: today's body
with no anchor (never shipped) and the unanchored form at exit 1. The unit
test's negative list dropped the unanchored exit-127 guard for the same
reason.

Observation: the recognized set grows by one per respelling. If a third
retired form arrives, a table of (version, renderer) would read better than
an array literal.
