# posture-sweep — administering the engineering postures

When running the posture sweep (plan's job 4), this discipline binds:

- **The pages are the authority as they read this tick** — every section
  of `specs/process/engineering.md`, the subsystem roster from
  `specs/process/architecture.md`'s codemap. Nothing is swept from a
  remembered list.
- At most one **touched** subsystem is read and swept per tick — that
  is the context bound. Untouched-since-last-sweep subsystems (the
  forward `git log` window is the test, no file reads) **skip forward
  in bulk within the same tick**; a rotation over a quiet tree closes
  in one tick, never one tick per skip. **Quiet-on-clean is the normal
  verdict**, recorded by advancing the rotation alone.
- A violation counts only when **verified on disk this tick**, cited by
  symbol and line. Beyond the pages' own sections, cohesion (a module
  carrying jobs that want separate homes), dead plumbing
  (unconstructable variants, vacuous Result paths), and **embedded
  provider knowledge** (documented external facts — kind names, loci,
  payload schemas — as literals outside the provider face: the
  knowledge-form of an upward edge, invisible to the import graph)
  are sweep lenses.
- Routing: purely mechanical shape → a pending entry; needs a design
  decision → a `.flume/refactor/` capture. Either cites the owning
  section in `per`. Never against a "Kept on purpose" asymmetry.
- When the rotation closes, stamp `Posture swept through: <HEAD sha>`;
  the job re-arms when commits past it touch `src/`, `sdk/src/`, or
  `tests/`.
