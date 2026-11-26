# Memory claims AGENTS.md behind an absence gate: three seams

Shipped: lock column `absence_gate = [{ template, absent = [...] }]`, the SDK
`GatedTemplate` (`{ glob, absent }` inside a `Templates` list), the lifted
`CustomKind.absence_gates`, and `graph::reachable` reading a closed gate as a
dead own edge (`gate::absence_gated`). Imports still carry liveness. `explain`
threads the same set. The memory docs (retrieved 2026-11-26) say three files
shadow an `AGENTS.md`: `CLAUDE.md`, `.claude/CLAUDE.md`, `CLAUDE.local.md`.
The gate lists all three, not only the `CLAUDE.md` the spec names.

- **The dogfood will flag its own root `AGENTS.md`.** `.temper/lock.toml`
  (not writable here) still overlays memory with one template. After the
  human re-emit, the root `AGENTS.md` beside `CLAUDE.md`, which imports
  nothing, draws a `root.reachable` advisory. Claude Code really does not load
  it, so the advisory is correct.
- **Unmodeled AGENTS.md facts.** The docs also load `.claude/AGENTS.md` at
  session start, and a subdirectory's `AGENTS.md` on Read when "that
  subdirectory has none of the three `CLAUDE.md` files of its own". That
  wording does not say whether a root `CLAUDE.md` suppresses a nested
  `AGENTS.md`. The gate follows the spec ("in its directory or above"). If
  the docs mean own-directory only, that is a spec question. `.claude/AGENTS.md`
  is not claimed.
- **Signature ripple.** `reachable`, `reachability_orphaned` and `explain`
  each took one more parameter. Their argument lists are long now, and a
  reach-inputs struct (`repo_files` and `gated` together) would cut that.
