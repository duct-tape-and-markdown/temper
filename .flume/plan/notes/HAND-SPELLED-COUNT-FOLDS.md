# HAND-SPELLED-COUNT-FOLDS

Shipped every named site. The tree's line numbers had drifted from the entry's (the sites
were found by phrase, not by line).

- Any edit to `src/builtin_lock.toml`, its comment header included, moves the built-in
  digest in an unreleased engine stamp, so `examples/base-harness/.temper/lock.toml` has
  to be re-emitted in the same commit. The fix only touched the stamp line. A
  comment-only fold of that header therefore costs a lock churn. If the digest hashed
  only the declaration rows, comments could be edited freely; that design call is plan's.
- cli.rs: a `listed_subcommands()` helper now reads the subcommand set from the binary's
  `--help` Commands section. The surface test (renamed
  `the_cli_surface_is_exactly_the_shipped_verbs`) asserts exact set equality including
  `tap`, which is stronger than the old per-verb `contains`. The help-text sweep iterates
  the derived set.
- shape_predicate: added the missing `LeadingDotSlash` name assertion. `Shape` has no
  `ALL` iterator, so a closed-set walk was not available.
- Swept in passing: `builtin_lock_frozen.rs:41` "six handler-level clauses".
- Not touched: the "documented five" handler-type guidance strings in the built-in
  lock's rows. They come from the SDK's `builtins.ts`, and correcting them is an SDK
  edit plus a regeneration.
