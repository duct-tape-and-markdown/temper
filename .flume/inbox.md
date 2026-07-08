<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Windows field report, two findings (observed at 2285821):
  1. `tests/common/mod.rs:58`'s `ensure_sdk_built()` spawns bare
     `Command::new("npm")` — `NotFound` on every Windows box (no bare
     `npm.exe`; Windows ships `npm.cmd`, and `Command`'s launch doesn't
     consult `PATHEXT`). A regression from today's tmpdir-consolidation
     wave: the correct, already-shipped fix is `src/install.rs:1311`'s
     `npm_program() -> &'static str { if cfg!(windows) { "npm.cmd" } else
     { "npm" } }`, but it's private (`fn`, not `pub`) and `tests/` is a
     separate integration-test crate that can't reach it — so this isn't a
     one-line copy-the-conditional fix without recreating the exact
     duplication class this repo just spent a wave consolidating out
     (`specs/process/engineering.md`, "one job, one home"). Correct shape:
     make `npm_program()` `pub` in `install.rs` and have
     `tests/common/mod.rs` call `temper::install::npm_program()`, or lift
     it to a shared home both sides reuse.
  2. `tests/builtin_lock_frozen.rs`'s
     `the_embedded_builtin_lock_byte_equals_the_sdk_modules_own_memberless_emit`
     false-fails on a Windows clone: content is identical, but the live SDK
     emit (`drift::emit_program`, LF-normalized via `normalize_lf`) gets
     byte-compared against `src/builtin_lock.toml` read straight off disk,
     and the repo carries **no `.gitattributes`** — so a Windows clone with
     `core.autocrlf=true` (git's own installer-recommended default) rewrites
     the checked-in LF fixture to CRLF on checkout, and the test's strict
     byte-equality (correctly enforcing decision 0010, "line endings are
     layout") then flags a false drift. `src/builtin_lock.toml` is the
     confirmed direct case; the risk class is broader — 20 `.snap` files,
     `tests/fixtures/{rules,extract_equivalence,coordinate}`, and several
     other byte-fidelity round-trip tests (`tests/extract_equivalence.rs`,
     `tests/lock_declaration_rows.rs`, `src/document.rs`/`src/extract.rs`
     round-trip paths) read committed files with the same strict-equality
     assumption — worth an audit for which need pinning, not just the one
     confirmed file. Fix is a `.gitattributes` pinning the frozen/
     round-tripped fixture set to `text eol=lf` (or `-text` if any carry
     deliberate CRLF as data) — the whole point of decision 0010 is that
     these bytes are data, never something git checkout should touch.
