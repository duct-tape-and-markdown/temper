# posture-sweep — administering the engineering postures

When running the posture sweep (plan's job 4), this discipline binds:

- **The pages are the authority as they read this tick** — every section
  of `specs/process/engineering.md`, the subsystem roster from
  `specs/process/architecture.md`'s codemap. Nothing is swept from a
  remembered list.
- One subsystem per tick. On a subsystem untouched since its last
  sweep, skip forward; **quiet-on-clean is the normal verdict**,
  recorded by advancing the rotation alone.
- A violation counts only when **verified on disk this tick**, cited by
  symbol and line. Beyond the pages' own sections, cohesion (a module
  carrying jobs that want separate homes) and dead plumbing
  (unconstructable variants, vacuous Result paths) are sweep lenses.
- Routing: purely mechanical shape → a pending entry; needs a design
  decision → a `.flume/refactor/` capture. Either cites the owning
  section in `per`. Never against a "Kept on purpose" asymmetry.
- When the rotation closes, stamp `Posture swept through: <HEAD sha>`;
  the job re-arms when commits past it touch `src/`, `sdk/src/`, or
  `tests/`.
