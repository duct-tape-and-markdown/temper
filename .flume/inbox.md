<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


## cascade's diff-audit of the seven re-cuts: two defects a builder would resolve by fabricating, one scope statement, cosmetic cites — observed at 7ba8e74f

Full reports at `~/.cache/cascade-integrations/audits/recut-*.md`. Plan's
re-verification held: every corrected cite checked out; what remains is
untouched or newly introduced. Four claims checked here at HEAD and
confirmed: `TomlDocumentError::Malformed` carries
`temper::toml_document::malformed` (src/toml_document.rs:129); NESTED's
tests[].asserts still names `why_impl`; drift.rs:892 and graph.rs:1703
both split a host address on `:`.

- **SESSION-START-LOAD-ERROR-BYPASSES-REPORTER** — one correctness defect
  the re-cut introduced: the tests[] row for tests/toml_document.rs says
  the finding is `gate.load-fault`, but under the entry's own mechanic (i)
  the rule is the report's code, `temper::toml_document::malformed`. A
  builder writing that assertion goes red and the likely "fix" fabricates
  the id, unwinding the amendment. files[] for the same file is right
  (count-based). Fix the row. Also: "library-level at that" about
  tests/reporters.rs:169 is false (it drives the binary at :177, it just
  never sees a load fault) — strike it, it is the template for the new
  SARIF case; name the SARIF test's home out loud (tests/reporters.rs is
  natural and is not in the fence); mechanic (ii), the source chain, has no
  test pinning it — a non-UTF-8 settings.json case in tests/session_start.rs
  would; and the eight tripwires all assert substrings living in the
  top-level `#[error]`, so they pass on Display alone and do not tie to the
  chain as acceptance says. Six of the eight sit outside the fence.
- **NESTED-MEMBER-DUPLICATE-KEY-ADMISSIBILITY** — tests[].asserts names
  "explain's why_impl" where files[] and acceptance now say `resolve` (:209);
  the regression gets written against the assertion contract, so the entry
  contradicts itself from the inside. Two inventory errors: "drift.rs:892
  splits a different string" is false — it splits a host address into
  (host_kind, host_name) with an error naming the kind:name grammar, a sixth
  site; and graph.rs:1703 (the host-segment split inside
  `parse_nested_address`) is missing, and is the one site that literally
  becomes the reader half. Seven sites: gate.rs:627, graph.rs:333, :1348,
  :1624, :1703, read.rs:238, drift.rs:892. Handoff gap: the gate.rs item
  hands the inventory forward to MEMBER-ADDRESS, whose files[] never
  receives it.
- **GUARD-UNDECLARED-LOCUS-WRITE** — mechanism, locus sourcing, the :1364
  inversion, owns_source, memory's `.` root, blockedBy relabel: all
  resolved. Scope statement missing: the entry frames everything in
  `.claude/` terms, but the binding is every non-`.`-rooted governed locus.
  On cascade (block mode) that includes specs/intent, specs/domain,
  specs/features, specs/surfaces — a hand Write creating
  specs/intent/north-star.md is denied at exit 2 after this lands. Correct
  under cascade's own law (specs are projections emitted via Bash, which the
  guard does not instrument) and it closes a real hole, but it is the
  largest behavior change for the first adopter and acceptance must say so.
  Cosmetic: install.rs:857 is :858 (`is_claude_path` :886);
  `guarded_manifests` spans :556-602.
- **CHECK-UNDECLARED-MEMBER-AT-A-GOVERNED-LOCUS** — safe as written. The
  `local`-remedy divergence is now principled (no built-in carries
  Content::Layout). Cosmetic: gate.rs:73-76 is :74-77; the three exclusions
  and "only Content::File kinds join" are not the same predicate (a
  Content::Fields kind without a collection_address passes one and fails
  the other) — state the one predicate `content == File &&
  governs.is_some() && commitment != Local` and drop the redundant
  exclusion; add a clause that check keeps `.`-rooted loci on purpose while
  the guard excludes them.
- **EXPLAIN-KIND-REGISTRATION-LOCUS-AND-ADDRESS-FORM** — drift.rs
  manifest/key_path are :3479/:3482, the three-strand range :534-582, the
  early return :554-567 (the first two propagated from cascade's original
  audit); the drift.rs cites sit inside the read.rs files[] item with no
  file named — add "in src/drift.rs"; the registration channel label is
  `event(event)` on the wire; the nested-file child (supporting-doc) is a
  third case the address strand's title invites folding into the embedding
  inverse — narrow the title to embedding hosts or add the branch;
  "byte-for-byte" for tests/read_verbs.rs:1065 holds only if the reword
  keeps the opening clause "No authoring guidance is declared" and changes
  the tail after the em dash.
- **MEMBER-ADDRESS-GRAMMAR-ONE-HOME** — tests/read_verbs.rs:257 is :258; the
  leaf-tail doc range is graph.rs:1688-1694; "intra-doc links from four
  files" is two (graph.rs, compose.rs — compose.rs:1212 is a plain comment);
  call-site cites regressed to definition lines (:331, :1608, :1650) where
  the calls are :332, :1617, :1653; engineering.md:15-16's commit-body
  obligation is still instructed nowhere.
- **SDK-MEMBER-ADDRESS-GRAMMAR-ONE-HOME** — fully corrected; the
  sdk/test/member-address.test.ts description still says "twenty" where
  everything else says twenty-one.

On the settings.local.json debt line: plan's four clauses hold for a
represented harness. With no lock, `targets` is None and the fallback at
install.rs:858 uses `is_claude_path` (:886), a substring match on
`.claude/` that does match settings.local.json — bound at default warn
mode, still exit 0. "Never bound" is true only under a lock; the debt line
should say so.
