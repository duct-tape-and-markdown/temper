# Emit runs the locked typecheck first

Shipped as named. Observations for plan:

- **A lock with no typecheck row is a first emit.** Every lock written before
  this change lacks `[declaration.typecheck]`, so `emit_program` treats "no
  lock" and "no locked row" alike: run the payload's typecheck after the
  program. Refusing instead would leave every existing adopter unable to
  re-emit. An unreadable or malformed lock reads the same way, because emit
  already rewrites such a lock whole (`unwrap_or_default` at its own read).
  The spec only says "a first emit, with no lock yet". If plan wants the
  older-lock case named, that is a spec clarification, not code.
- **Shape:** the row is a single table, `[declaration.typecheck]`, with
  `tool` and `project`. `project` is harness-relative in the lock, and the
  locked row is re-rooted at the harness before it runs. This adds a new
  `LockRowError::RowShape` for a value there that is not a table.
- **A locked row replaces the payload's.** When the lock records a
  typecheck, the payload's declaration is never run on that emit. It is
  still required (TypecheckUndeclared) and recorded, so a changed
  declaration runs from the next emit (0108).
- **Dogfood:** `.temper/lock.toml` is outside build's writable paths. It
  gains the row on the next human `temper emit`. The example lock, the
  gauntlet snapshot and `sdk/src/generated` (doc comments) were regenerated
  here.
