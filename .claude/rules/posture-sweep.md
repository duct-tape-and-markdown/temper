---
paths: ["specs/process/engineering.md","specs/process/architecture.md"]
---
# posture-sweep — administering the engineering postures

When running the posture sweep (plan's job 4), this discipline binds:

- **The pages are the authority as they read this tick** — every section
  of `specs/process/engineering.md`, and the choices in
  `specs/process/architecture.md`. Nothing is swept from a remembered
  list. A ratified phrase change applies from the next rotation
  forward — it never reopens a stamped window.
- **The frontier is decidable; the neighborhood is judged.** Two delta
  kinds arm it, both read off the forward `git log --name-only` (no
  file reads): a **code delta** puts the window's touched modules in
  the frontier; a **phrase delta** — the window touched a posture page
  itself — puts the whole module list in, because a changed phrase has
  been applied to nothing yet. Each tick
  sweeps at most one neighborhood — one frontier module read together
  with its immediate imports; that is the context bound — and records
  every frontier module the neighborhood read as **covered** in the
  mid-rotation cursor. Covered is settled for the window: a later tick
  never re-sweeps or re-draws it, even where fresh judgment would cut
  the boundary differently — the cursor decides coverage, never
  re-derivation.
- **The rotation closes when the frontier empties.** Untouched modules
  never enter the frontier, so a quiet tree closes in one tick, never
  one tick per skip. **Quiet-on-clean is the normal verdict**,
  recorded by advancing the cursor alone.
- **An open rotation is live input.** While the frontier is non-empty,
  the tick's closing marker is `Plan continues: yes` — the rotation
  drives itself to close and is never left waiting on a forced wake.
  Hibernation is the empty frontier's verdict alone.
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
- When the rotation closes, stamp `Posture swept through: <window head>`
  — the sha the frontier was derived from, never a HEAD that moved
  mid-rotation; the job re-arms when commits past the stamp touch
  `src/`, `sdk/src/`, `tests/`, or the posture pages
  (`specs/process/engineering.md`, `specs/process/architecture.md`).
