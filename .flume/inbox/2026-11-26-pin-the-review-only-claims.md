# Pin the review-only claims that the census found cheap to test

The 2026-11-26 census found 87 claims held only by prose. These are the important, cheap ones. Each becomes a pin-only entry, where the behavior is kept so the line is a `pins[]` line or an SDK test (PROTOCOL, Tests):
- `check` and `explain` run over an emitted harness with Node absent from the PATH (`authoring.md`, "checking never needs Node");
- `explain` exits 0 over a harness with an error-severity finding (`contract.md`, "Read verbs": a read verb never gates);
- `check`, `guard` and `explain` leave the lock byte-identical (`pipeline.md`: emit is the lock's sole producer);
- all four reporters render the same non-empty finding set (`distribution.md`: one diagnostic source);
- the release version equals the tag: a `release.yml` step compares `sdk/package.json` with `GITHUB_REF_NAME`;
- a nested skill's leaf `name` passes `skill.name-matches-dir` (`builtins.md`, skill);
- the package root exports no built-in kind (`builtins.md`: the `claude-code` subpath only);
- a keyless retitle of a collection heading is a delete plus a create (`representation.md`, "layout");
- a nested guard (`when` in a `when` body) is refused (`contract.md`, "clause").

Plan files each as its own entry, with the claim named in the entry so the pin traces to the sentence.
