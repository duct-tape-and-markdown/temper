# Gate commands exit 2 on a missing binary; upgrade over the 127 form duplicates

Shipped: `render_gate_command`'s missing-binary branch exits 2 and its stderr
appends an install hint (`INSTALL_HINT`: `npm install -g @dtmd/temper`, or
`npm -C .temper install` for a represented harness). SESSION_START_COMMAND /
GUARD_COMMAND and the tap command follow; `examples/base-harness` re-emitted.

Upgrade path, unstated by the spec, not invented here: `rendered_gate_binary`
recognizes only today's bytes, so a settings.json carrying the prior exit-127
rendering reads as no installed gate. Re-running `install` appends the new
command beside the old one (the same shape the pre-anchor test pins:
`commands == [old, new]`), so the old non-blocking hook keeps running next to
the new one. Whether install should recognize and replace a prior rendering
(a recognized-legacy-forms list? a version key?) is a spec question for
`specs/distribution.md` (install convergence), not a build call.

The hint text itself is a build choice the spec leaves open ("the install
instructions"); a represented harness whose tool is some other path still
gets the npm hint.

Dogfood: `.claude/settings.json` drifts until a human re-emits `.temper`.
