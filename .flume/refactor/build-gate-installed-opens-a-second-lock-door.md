## Surface

`gate::gate` reads and parses `.temper/lock.toml` once, then hands the
parsed document down its tiers — and then calls `install::gate_installed`
(`src/gate.rs:729`), which takes no document and re-reads the lock twice on
every represented harness:

- `src/install.rs:1092` — `evaluate_placements` → `drift::emit_owned_targets`
- `src/drift.rs:3745` — `walk_lock_rows` → `read_lock_document_for_emit` (read 2)
- `src/drift.rs:3762` — `read_declarations` → `read_lock_document` (read 3)

`tests/it/check_cost.rs:368` pins the run at exactly 1 read / 1 parse,
"shared across every tier that reads off it", and calls it out as a per-tick
session-open cost. The pin is green only because its fixture writes a lock
and no `.temper/harness.ts`, so `gate_installed` takes the unrepresented
branch and never reaches those two doors. Every actually-represented harness
— this repo included — pays 3/3 per tick, and the pin cannot see it.

## Observed at

1f666731 (HEAD when observed) — surfaced while widening `represented_by`
would have made the pin's own fixture reach the represented branch; the
entry left the reading narrow rather than re-baseline a test outside its
fence.

## Suggested consolidation

Thread the already-parsed lock into `gate_installed` the way the other tiers
take it: `emit_owned_targets` has a `*_from_doc` shape available for both of
its reads (`walk_lock_rows_from_doc`, `declarations_from_doc`), so one
document argument collapses 3/3 back to 1/1. Re-point the `check_cost`
fixture at a harness carrying a `harness.ts` in the same change, so the pin
counts the door it exists to count.
