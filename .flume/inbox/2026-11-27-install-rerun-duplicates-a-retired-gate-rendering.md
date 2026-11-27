# Re-running install after the exit-2 change seats a second gate beside the retired one

Found curating the 0.0.27 changelog. GATE-COMMAND-EXITS-2-WHEN-ENGINE-UNREACHABLE changed `render_gate_command`, so the missing-binary branch exits 2 instead of 127 and names the install command. On an unrepresented harness, `install`'s merge asks `group_runs_gate` whether temper's gate is already wired. `rendered_gate_binary` matches only today's exact bytes, so a `settings.json` holding the 0.0.26 rendering (`… || { echo "temper: command not found" >&2; exit 127; } && temper …`) reads as unwired. A re-run then appends a second group at each gate address. Both guards then run on every tool call, and the old one still fails open.

`specs/model/adoption.md`: "Re-running install converges". A re-run after an upgrade must leave one gate per `(event, matcher)` address, at today's rendering.

Acceptance:
- **new:** an unrepresented harness whose `settings.json` holds the 0.0.26 rendering of each gate hook ends a re-run of `install` with exactly one group per gate address, carrying today's rendering;
- **new:** a represented harness reports the retired rendering as a stale member, with its remedy (`emit` re-renders it), never as a missing one;
- **kept:** a hook that runs anything other than a temper rendering, current or retired, is never rewritten.

Spell the retired rendering once, beside `render_gate_command`, as the one prior form recognized, not as a looser match.
