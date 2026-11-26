# A tap's tool edge leaves the requirement node, beside the typecheck's

Shipped: `runsRows` appends `{ member = "root/requirement/<name>", tool }` for
each telemetry requirement that names a tool, in name order. The lock carries
it as an ordinary `[[declaration.runs]]` row. `graph::root_runs_edges` now
takes (runner, tool) pairs. It lifts the root runner (typecheck) and
root-hosted requirement runners. A requirement runner leaves the
`(requirement, <name>)` node, the same node a mention of the requirement
binds, so `explain` and `degree` agree. Member runners are skipped because
`fold_runs` carries them. As with the typecheck edge, `compose::fold_runs` is
untouched: a requirement is no lock member and has no features to fold onto.

A tap with no declared tool draws no row. It runs the engine's default
temper, which is no composed tool member.

Observed (real adopter friction, not fixed here): a program whose only
hook-kind content is a telemetry tap fails `emit` with `drift::unknown_kind`
("member `PostToolUse` names kind `hook`, which the payload's declarations
carry no kind fact for"). The synthesized tap row needs the hook kind fact,
and only a composed hook member brings it. The new test composes a
do-nothing `SessionStart` hook to get past this
(`tests/it/tool_kind.rs`, `TAP_ONLY_PROGRAM`). Worth an entry: tap synthesis
should bring the hook kind with it.

`root/requirement/<name>` still does not parse via `parse_nested_address`
because `host_identity` rejects the bare `root` host. `root_runs_edges`
strips the prefix through `nested_address` + `is_one_segment` rather than
reparse it.
