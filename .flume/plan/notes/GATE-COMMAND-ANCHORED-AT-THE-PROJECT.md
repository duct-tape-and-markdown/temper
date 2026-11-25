# Gate command anchored at the project root

- Shipped spelling: `cd "${CLAUDE_PROJECT_DIR:-.}" || exit 1; command -v ... && <bin> <verb>`. The `cd` is not chained with `&&`: a failed `cd` would fall into the probe's `||` branch and report "command not found", exit 127. The prefix lives once, as `PROJECT_ANCHOR` in `src/install.rs`.
- Why the prior attempt read green-on-base: its `tests[]` carrier for the unanchored line was a unit test in `src/install.rs`. The judge overlays a carrier's file onto base, so the fix came along with it. Both `tests[]` carriers now live in `tests/it/install.rs`; each was checked failing with the base `src/`. Plan: a `tests[]` line whose behavior lives in a private fn needs an integration-level carrier.
- `it__install__a_fresh_install_settings_document_is_stable.snap` was orphaned (no test names it), so it was deleted rather than re-blessed.
- Tests that run a gate command through `sh -c` now clear `CLAUDE_PROJECT_DIR`, since gates run inside Claude Code and would otherwise `cd` into the trunk checkout. The not-on-PATH guard test pipes its payload into `{ cmd; }`; a bare pipe would feed only the `cd`.
- Human half: re-emit the dogfood `.claude/settings.json` and `.temper` lock (`chore(harness)`). Until then, check names this repo's own gate as stale.
