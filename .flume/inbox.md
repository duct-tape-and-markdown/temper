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
  `tests/install.rs:582`'s `PermissionsExt` use is already correctly
  `#[cfg(unix)]`-gated; not part of the break. Dates to the 07-06
  SDK-integration wave (`911cc45`, `7102391`, `7decfd6` —
  `BUILTIN-LOCK-FROZEN-LANE` and siblings first added the symlink-vendoring
  helper), not to any 07-08 work. CI is `ubuntu-latest`-only, so this is
  currently invisible to every gate — the `cargo test` afterMerge gate
  cannot run on a Windows box at all until fixed.
  **The Windows fix is a junction, not `symlink_dir`** (verified
  2026-07-08): a true directory symlink on Windows needs
  `SeCreateSymbolicLinkPrivilege` (admin or Developer Mode); a junction
  needs neither — nixhacker.com "Understanding and Exploiting Symbolic
  links in Windows" (retrieved 2026-07-08:
  https://nixhacker.com/understanding-and-exploiting-symbolic-link-in-windows/),
  hinchley.net "Junctions and Symbolic Links" (retrieved 2026-07-08:
  https://hinchley.net/articles/junctions-and-symbolic-links). Confirmed as
  the load-bearing reason npm itself creates junctions (not symlinks) for
  local/workspace deps on Windows — npm/cli#5189 (retrieved 2026-07-08:
  https://github.com/npm/cli/issues/5189) — which is exactly what these
  three helpers stand in for (this repo's own
  `.temper/node_modules/@dtmd/temper` resolves via the same mechanism npm
  itself would use). `std::os::windows::fs::symlink_dir` compiles clean but
  still hits the privilege wall on a stock (non-elevated,
  non-Developer-Mode) runner; `std` exposes no junction constructor, so the
  fix needs a small crate (e.g. `junction`) or a raw
  `DeviceIoControl`/`FSCTL_SET_REPARSE_POINT` call — bigger than a one-line
  `cfg(windows)` swap.
  Separately confirmed clean, no action needed: both previously-known
  Windows workarounds are upstream and load-bearing as designed —
  `install.rs`'s `npm_program()` (npm.cmd naming) and `drift.rs`'s
  `strip_verbatim_prefix` (`\\?\` stripping); a stock build now works
  without a patched worktree binary.

