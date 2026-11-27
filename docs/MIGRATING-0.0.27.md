# Migrating to 0.0.27

**This note covers `0.0.26` to `0.0.27` and nothing earlier.** For the
upgrade before it, read [`MIGRATING-0.0.26.md`](MIGRATING-0.0.26.md).

## 1. Re-emit, or re-run install

The gate command changed: when the engine cannot be reached it now exits
2, which Claude Code treats as blocking, instead of 127, which it let
through.

- **A harness with a `.temper/` program:** run `temper emit .temper` and
  commit the settings and the lock together. Until then `check` reports
  `root.engine-matches`, and `install.gate-installed` names each gate hook
  as stale.
- **A harness without a program:** run `temper install` again. It
  recognizes the 0.0.26 command and replaces it in place.

Before (0.0.26):

```sh
… command -v temper >/dev/null 2>&1 || { echo "temper: command not found" >&2; exit 127; } && temper guard .
```

After (0.0.27):

```sh
… command -v temper >/dev/null 2>&1 || { echo "temper: command not found; install temper with npm install -g @dtmd/temper, or npm -C .temper install for a represented harness" >&2; exit 2; } && temper guard .
```

On a machine without the engine, a session now stops at the guard with
those instructions rather than writing unguarded.

## 2. Re-key an override of the rule line budget

Only if your contract dials or overrides the rule's 200-line advisory.
The clause now binds only rules that load at launch, so its label moved.

Before: `rule.extent`. After: `rule.when.loads=launch.extent`.

A path-scoped rule no longer draws this advisory at all. It is judged by
the on-demand 40,000-character limit instead.
