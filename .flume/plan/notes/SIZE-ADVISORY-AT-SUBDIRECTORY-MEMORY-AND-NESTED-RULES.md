# SIZE-ADVISORY-AT-SUBDIRECTORY-MEMORY-AND-NESTED-RULES

Shipped as written: memory gains `when(loads("on-demand"), [extent(characters, 40000)])`.
`rule` now registers `on-read-beneath` beside `paths-match`, and its size clause guards
on the load moment instead of on `paths`.

Three engine seams the entry did not predict:

- `kind::member_home` folded only a direct `.claude/` parent, so a root rule was homed
  at `.claude/rules` and read as on demand. It now folds `.claude/` and a
  `.claude/rules/` tree at any depth into the directory that holds it. Other `.claude/`
  subtrees keep their own directory.
- Reachability OR's a kind's channels, and `on-read-beneath` is never dead, so it would
  have hidden a rule whose `paths` match nothing. Beside a sibling channel it now only
  registers, and the siblings decide liveness. `explain` says "registered ...; its
  `paths` hold the load until a match" for a scoped rule. This conjunctive reading is
  implied by 0120 ("register"), but no spec line states it. Worth a sentence in
  `builtins.md` if plan agrees.
- Stale built-in rows (0117) used to be dropped whole. That dropped an authored
  `relocate` exclude, so the self-host check failed hard on `examples/demo`. Now the
  stale row's format, unit shape, registration, guidance and cite are cleared, the
  author's locus stays, and the re-emit finding still fires. 0117 says "reads the
  embedded built-in", and an authored relocation is not a snapshotted built-in fact.
  Flag it if the corpus means otherwise.

Owed: a `chore(harness)` re-emit of the dogfood `.temper/lock.toml`. The rule row and
the digest stamp both moved, and the path is outside build's writable set. The example
lock was re-emitted here.
