<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Field (centercode, first contact with the enablement fix): **nested-file
  discovery loses every member when the harness root is spelled relatively —
  `check .` and bare `check` find `supporting-doc (0)` while
  `check <absolute-path>` finds all 12 on the same tree**. Field-proven on
  centercode at bfe147c1: relative spelling → 36 members, 12 `graph.route`
  failures (`` `conventions` `cite` routes to
  `supporting-doc:csharp-conventions`, which resolves to no `rule` or
  `supporting-doc` artifact ``), exit 1; absolute spelling → 48 members,
  `supporting-doc (12)`, exit 0. The docs' own wired spelling —
  `temper check . --reporter session-start` — is the broken one, so every
  real session-open report on a harness with nested members goes red while
  CI stays green (fixture tests build absolute tempdir roots and never see
  it). Suspected mechanism, worth verifying: the shared discovery index keys
  entries through `address::normalize_path`, which drops the `CurDir`
  component, while `discover_nested_file`'s host-unit derivation
  (`src/import.rs`, `unit_dir`) strips entries against the raw
  `disc.harness().join(&governs.root)` — for a `.`-rooted harness that
  prefix still carries `CurDir`, `strip_prefix` mismatches on components,
  every host resolves `None`, and the nested roster is silently empty. Flat
  kinds survive because their id derivation tolerates the same mismatch —
  which is its own smell: the harness-root spelling should be normalized
  once, at `Discovery` construction, not tolerated per consumer. Likely
  regressed in the shared-index rewrite (73d5757/0bc0ee9 window); the
  pre-index walk returned entries carrying the same `./` prefix as the
  root. Emit is unaffected (composes from the program, `0 emitted, 39
  unchanged` throughout). Ask beyond the fix: a fixture that runs the gate
  with a *relative* root — the spelling the product itself wires — so
  path-spelling regressions can't pass again. observed at bfe147c1

- Field (temper repo housekeeping, same window): fe58bc2 committed
  `src/main.rs.backup` (2,822 lines) into the tree — a stray editor/refactor
  artifact riding a `build:` commit. Worth reaping and worth a posture-sweep
  rule if the sweep doesn't already flag `*.backup`. observed at bfe147c1

- Field (centercode follow-up on 55b8539): the new round-trip gate in
  `tests/builtin_lock_frozen.rs` covers one row, not the class. It decodes
  exactly one row of the derived lock (`find(|k| k.name == "installed-plugin")`)
  through `CustomKind::from_kind_fact_row`; the other builtin kind rows are
  parsed structurally but never decoded, so a one-sided respell of any *other*
  wire vocabulary — an `event(f)` label, a `paths-match` form, a shape or
  collection-address spelling — would still pass CI exactly the way bare
  `enablement` did. Fix shape: replace the single `find` with a loop decoding
  *every* row through `from_kind_fact_row` (expect all to succeed), keeping the
  targeted field-carrying-Enablement assertion as the regression pin. The gate
  already pays the expensive part (node build + emit); decoding the remaining
  parsed rows is microseconds and converts "this bug can't recur" into "this
  class can't recur" — the original ask. Low priority: no other channel is
  mid-respell. observed at 9a7d7d1
