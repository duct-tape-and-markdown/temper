# Load-bearing verdicts pin their subject

All six tests now pin a populated subject before their verdict.

- install.rs `guard_places_no_memory_local_member_inside_an_ignored_directory...`:
  the scaffold had no lock, so the guard bound nothing and every path was
  silent. The test now writes a warn lock declaring `memory-local` at the root
  `CLAUDE.local.md` and asserts that a write there names its owner. Probed
  while building: under that lock, an *unignored* `sub/CLAUDE.local.md` also
  draws nothing, in warn or block mode. So the guard places no undeclared
  memory-local anywhere, and the ignored-dir property this test names is not
  isolated from the general one. To isolate it, the test needs a case where
  an unignored undeclared memory-local draws a placement. If the guard should
  place one, that is a product question.
- locus_collision.rs: the nameless `.claude/agents/reviewer.md` load-faults
  and counts as `agent (0)`, so its pin checks that the path is named, not a
  tally. memory counts 3 and memory-local counts 1.
- cli.rs self-host: parses the `checked N members` census from the output,
  since no shared helper exists. coverage_note/check_cost repeat the same
  substring probing; a `CheckRun::census()` helper would be the one home.
