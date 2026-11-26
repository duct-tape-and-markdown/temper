# FRONTMATTER-LOADS-AS-THE-RUNTIME-LOADS shipped

- Leniency is a kind fact: SDK `leniency?: readonly Leniency[]` (`"malformed-yaml"`)
  on the `at` and `nested-file` KindFacts variants, row column `leniency = [...]`
  (empty omitted), `CustomKind.leniency: Vec<Leniency>` lifted with the closed-vocab
  guard. skill and command declare it; agent does not. OUTPUT-STYLE-KIND-SHIPS only
  needs `leniency: ["malformed-yaml"]` on its kind.
- Forgiven scope: only a block YAML cannot parse (gray_matter's Null, which also
  covers an explicit `null`). A block that parses to a sequence or bare scalar stays a
  `member.load-fault` even on a lenient kind — the docs speak of YAML that does not
  parse, and nothing cites what the runtime does with a non-mapping block.
- The forgiven member carries `frontmatter::Member::forgiven` (the detail); compose
  lowers it to one `member.non-portable-yaml` warn with the docs-vs-lab guidance, on
  both the frontmatter and layout read paths.
- Not done, not owed by the entry: `explain` (src/read.rs narrate_kind) does not
  narrate the leniency column; the row reads through drift.rs/kind.rs, not read.rs.
- Human half: the dogfood `.temper/lock.toml` gains `leniency` on its skill/command
  rows at the next `temper emit` (not a build-writable path). The example lock
  (examples/base-harness) was re-emitted here.
