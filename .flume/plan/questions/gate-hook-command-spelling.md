# How does a gate hook name the temper binary it runs?

Driver: GH #65. `GUARD_COMMAND` calls a bare `temper` behind `command -v temper || exit 127`, and `install.gate-installed` compares hook commands by exact string (`src/install.rs` `group_has_command`). An adopter with temper in a workspace package, off PATH, cannot both run a working guard and pass the check. 0073 gave the commands one home, the engine's.

Session stance (2026-11-24): the gate hook is a typed member with one declared fact, the binary path (default `temper` on PATH); the engine renders the command from it and `gate-installed` judges the typed record, never a string. The fact is stated once (0078). Ruling amends 0073's consequence in `specs/distribution.md`.
