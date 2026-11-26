# With no lock, the guard calls nothing a projection (0103)

`specs/model/pipeline.md`, enforcement: "with no lock nothing is one". The guard binds only paths the lock names as projections. With no lock, `src/install.rs:1947-1953` falls back to guarding every `.claude/` path as a "temper-managed projection" (`tests/it/install.rs::guard_defaults_to_warn_when_the_lock_is_absent`). On a harness with no lock that claim is false, because those files are authored sources. That makes it an instance of class 0103 (a finding is true for its reader). With no lock the guard reports no projection drift. At most it gives one session-scoped note that the harness is unrepresented. Acceptance:
- **new:** a no-lock write to `.claude/skills/x/SKILL.md` draws no projection warning;
- **kept:** with a lock, a write to a lock-named projection is still guarded.
