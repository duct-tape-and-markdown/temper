# Assertions that cannot fail: one hidden defect surfaced

All ten assertions now judge a populated, correctly named subject.
Spot mutations went red on agent `agent.unique-name`, rule
`rule.glob-valid.paths`, installed-plugin (SETTINGS in place of
SETTINGS_NO_PLUGINS), the emit CRLF contrast, and kind_extension
(dropping the satisfies row fires `requirement.count`).

Hidden defect (src/ untouched, per acceptance): when another engine's
lock holds a stale built-in `memory` row, `explain` narrates a stripped
kind. It drops the guidance line and shows the lock's globs
(`**/CLAUDE.md`) where the embedded built-in has `**/CLAUDE.md` and
`**/AGENTS.md`, plus its registration line. The old compare searched for
the word "guidance", which never appears (guidance renders as a `‣`
line), so it compared None to None. The corrected compare lives in
tests/it/read_verbs.rs as
`explain_narrates_a_stale_built_in_rows_kind_with_the_embedded_guidance`,
marked `#[ignore]`. It is red under `--ignored`. The fix entry
un-ignores it.

Also seen, not chased: in kind_extension, narrowing `review-coverage` to
`skill` while a `decision` (a rule) satisfies it fires no
`requirement.count`. That may be the narrowing judge's own label, or a
gap. Worth one look.

local_locus: the "always-true alternative" was the only live arm. The
error's Debug form carries `AtLocusUnderWorkspace`, never the snake-case
code. It now downcasts to the variant.
