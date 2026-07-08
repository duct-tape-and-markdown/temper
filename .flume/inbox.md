<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Windows field report (observed at baac085): `cargo test` (the full suite —
  `cargo build --release` and `cargo test --lib` are clean, 229/229) fails to
  *compile* on Windows, zero tests run. Three call sites use
  `std::os::unix::fs::symlink` unconditionally, no `cfg(windows)` arm:
  `tests/emit.rs:699` (`wire_sdk_harness_program`), `tests/install.rs:84`
  (`vendor_sdk`), `tests/builtin_lock_frozen.rs:105`
  (`wire_memberless_harness`) — each stands in for `npm install` by
  symlinking `sdk_root()` (a directory) into a scratch `node_modules/@dtmd/`.
  The Windows counterpart is `std::os::windows::fs::symlink_dir` (a
  directory target — not `symlink_file`). `tests/install.rs:582`'s
  `PermissionsExt` use is already correctly `#[cfg(unix)]`-gated; not part
  of the break. Dates to the 07-06 SDK-integration wave (`911cc45`,
  `7102391`, `7decfd6` — `BUILTIN-LOCK-FROZEN-LANE` and siblings first added
  the symlink-vendoring helper), not to any 07-08 work. CI is
  `ubuntu-latest`-only, so this is currently invisible to every gate — the
  `cargo test` afterMerge gate cannot run on a Windows box at all until
  fixed. UNVERIFIED, flag only: creating a directory symlink on Windows has
  historically needed Developer Mode or `SeCreateSymbolicLinkPrivilege`; a
  `cfg(windows)` fix may compile clean but still need that privilege at
  runtime — confirm on the actual CI runner if a Windows lane is ever added.
  Separately confirmed clean, no action needed: both previously-known
  Windows workarounds are upstream and load-bearing as designed —
  `install.rs`'s `npm_program()` (npm.cmd naming) and `drift.rs`'s
  `strip_verbatim_prefix` (`\\?\` stripping); a stock build now works
  without a patched worktree binary.

