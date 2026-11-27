# LOAD-MOMENT-ONE-DERIVATION shipped

`gate::channel_load_moment` is now the one per-channel moment derivation;
`gate::member_load_moment` folds it over a kind's channels (on demand if any
loading channel is) for the `loads` guard, and `read::narrate_channel` reads
the same function for its `always`, `on-read-beneath` and `paths-match`
sentences. `Placement` moved from read.rs to gate.rs so both carry one type.
`gate::load_moments` now reads each member's absence-gate state through a new
`kind_absence_gate_states` (which `absence_gate_states` also folds onto), so
an `always` channel behind an on-demand gate counts on demand.

`read::member_globs` is deleted for `graph::declared_globs`: the two were
equivalent (`extract_globs` already returns nothing for a map).

Observed, not taken: `narrate_channel` still picks its sentence per channel
while the guard reads the folded member moment. They cannot disagree on any
built-in kind (each registers at most one loading channel), but a custom kind
registering `always` beside `paths-match` would narrate one channel at launch
while the guard counts the member on demand. Whether explain should also
state the member's folded moment is a design question for the spec.
