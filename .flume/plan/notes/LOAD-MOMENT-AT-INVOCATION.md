# LOAD-MOMENT-AT-INVOCATION

`channel_load_moment` (src/gate.rs) now returns on-demand for `user-invoked`,
`description-trigger` and `paths-listing`; `None` is left to event, connection,
enablement and registry. Skill, command and agent members carry a moment, so a
`when(loads("on-demand"), ...)` body judges them and a `launch` guard stays silent.
`imported_file_moment` picks this up unchanged: a file only skills import is now
on-demand, and any launch-loaded importer still wins.

Not touched: `explain`'s per-channel narration (src/read.rs) still words the invoking
channels by hand, never through `channel_load_moment`. The wording already says the
body loads on invocation, so nothing disagrees today. But it is a second spelling of
the same fact. The refusal half (a guard on a kind whose content never enters context)
still waits on question load-moment-refusal-class.
