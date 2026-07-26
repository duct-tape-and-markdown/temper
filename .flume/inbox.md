<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

`temper tap` resolves its write destination cwd-relative. A hook firing
inside a flume fanout worktree therefore writes telemetry into a directory
flume deletes after each wave — the events are destroyed, and tap exits
zero, so nothing reports the loss. Proven destructive, not latent: in the
asp-lens job (centercode-platform, branch job/asp-lens) a mint agent's
friction note on `gate.kind` ambiguity was written to the worktree's
friction dir and destroyed; the agent truthfully reported having logged
it. Two victim classes, one mechanism: tap events and friction files.
Direction: durable writes must resolve against the git common dir (or an
explicit root env var flume sets), never cwd. observed at 05586093
