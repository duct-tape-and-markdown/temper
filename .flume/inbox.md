<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


## guard: an unparseable Write to a file-unit JSON document exits 0 — observed at 12bca4b3

Observed by cascade and reproduced here against 12bca4b3 (`guard .`, absolute paths): a Write
of `{ not json` to `.claude/settings.local.json` exits 0. The same bytes
to `.claude/settings.json` now exit 2 with `guard.manifest-unparseable`
(GUARD-UNPARSEABLE-MANIFEST-WRITE-REFUSAL). settings.local.json is a
represented member — `settings-local`, the file-unit JSON document at the
local commitment class (`specs/builtins.md`) — so the guard reads it as a
document, not a manifest, and the manifest refusal does not reach it.

Consequence today is loud, not silent: the next `check` fails to load the
member. The spec keeps a hook registered in settings.local.json an opaque
field (decisions 0032/0034/0036), so no reporter placement rests on that
file and this is not the settings.json exposure. The question is the
class, not the file: `specs/distribution.md`'s fail-loud invariant is now
held at the boundary for collection-address manifests; is it held for
every represented document whose Write the guard reconstructs — a
file-unit JSON document, a frontmatter file that no longer parses — or does
a parse fault on a document read still fall through to the next
placement? Verify at HEAD what the guard's document read does with a parse
fault (search `manifest_write_findings` and the document-read path in
`src/install.rs`) before filing; if the fall-through is the same class
GUARD-MANIFEST-EDIT-CONTENTLESS-FALLTHROUGH closed, it is one entry
naming the document rule, otherwise a fork on whether the boundary holds
per document or per manifest.

## cascade's audit of seven pending entries: every one needs amendment — observed at ab61fea4, source identical through ee3b9bd3

cascade-integrations audited the entries cut from its filings plus the three
address-grammar entries, claim by claim against the source. None contradicts
a decision; every one carries wrong cites, an omitted in-fence file, or a
mechanism claim that does not hold at HEAD. The full per-claim reports —
exact-vs-wrong cites, corrected instructions, omitted files — are on this
machine at `~/.cache/cascade-integrations/audits/` (README.md indexes them;
read the report for an entry before re-cutting it). Headlines, one per
entry; each is an amendment to the entry's notes/files/acceptance, not a
new entry.

- **CHECK-UNDECLARED-MEMBER-AT-A-GOVERNED-LOCUS** — cites exact, but the
  provenance-row join is wrong for hook, installed-plugin, known-marketplace
  (and mcp-server): they carry `[[declaration.registration]]` rows, never a
  `[[<kind>]]` rollup, so the generalized walk forges three false findings on
  the gauntlet and moves `gauntlet__check_diagnostics.snap`, the entry's own
  hard constraint. Skip kinds with `collection_address.is_some()` and kinds
  with `governs` None; only `Content::File` kinds with a file locus join.
  Normalize both sides through `normalize_lock_path` as the reap-diff does;
  the site walk must sit above the `coverage_note::check` call
  (gate.rs:544); the lock-presence gate widens the layout branch too; the
  `local` remedy caveat contradicts the shipped layout message at
  drift.rs:2832. Omitted from files[]: tests/check_cost.rs:388 (a five-arg
  `coverage_note::check` call — compile error), tests/coverage_note.rs:410
  and :335 (fixtures with an undeclared skill that gain a finding). A
  warn-severity finding trips `--deny-advisories` on every harness with an
  undeclared document — say so in acceptance.
- **GUARD-UNDECLARED-LOCUS-WRITE** — hole real, but sourcing loci from
  `declarations.kinds` does not close #58: neither the gauntlet lock nor
  cascade's carries a `[[declaration.kind]]` row for agent, so
  `.claude/agents/stray.md` stays allowed. Build the locus set the way
  `guarded_manifests` already does (main.rs:556-600: `builtin_kind::
  definitions()` overlaid, plus `partition_kind_rows` customs), skipping
  local commitment, `governs` None, and `collection_address` Some. The "no
  arm changes verdict" acceptance is false: tests/install.rs:1364 inverts,
  and without the collection_address exclusion the settings.json
  Unrecognized path changes verdict. `CustomKind::owns_source` is leaf-only;
  use `compile_glob` over root/glob with `scan_locus` semantics. memory
  governs `.` with `**/CLAUDE.md`, so the guard would judge every CLAUDE.md
  in the repo with no ignore reader — name the asymmetry or bound it.
  blockedBy GUARD-UNPARSEABLE is a real dependency (the early return at
  main.rs:359-374); blockedBy CHECK-UNDECLARED is textual only — say so.
- **SESSION-START-LOAD-ERROR-BYPASSES-REPORTER** — root cause exact and
  reproduced. cascade counts five assertions that break where build's
  second capture (e1adf17d) counts two: tests/gate_fail_loud.rs:534 and
  tests/toml_document.rs:254 unconditionally, plus tests/toml_document.rs:249,
  tests/marketplace_kind.rs:195, tests/plugin_manifest_kind.rs:143 which
  assert the miette code. The two counts agree once the rule id derives
  from the report's `code()` with `gate.load-fault` as fallback
  (`IdentityMapError` has no code, so duplicate-key lands there) — build's
  working fix already does this and reports those three green; verify,
  and keep the derivation in the entry's instruction so a re-cut cannot
  drop it. Render the full source chain, not bare Display, or
  `TomlDocumentError::Io/NotUtf8` lose their detail. src/reporter.rs needs no
  edit (context at :94 already renders error severity into the blocking
  block) — listing it under files.edit will make the builder invent one.
  `Announcement::default()` is the constructor. Precedent lowering:
  `frontmatter_fault_diagnostic` at compose.rs:633-661. Wrong cites:
  session_start is reporter.rs:66 not :118, cap constant :42 not :39,
  json_manifest.rs:267 is the variant not the raise (:334, via
  `Manifest::read` at :364). Nine abort-path prose assertions to verify-only
  are in the report. The entry's note that Claude Code never invokes
  SessionStart when settings.json will not parse carries no retrieved cite —
  keep it out of the shipped message until cascade supplies one.
- **GUARD-UNPARSEABLE-MANIFEST-WRITE-REFUSAL** — landed. cascade reported the
  render header falling to the generic "a member of this write violates its
  contract" on the new rule; re-run here at 12bca4b3 (`guard .`, Write of
  `{ not json` to settings.json) renders the unparseable header. Nothing to
  file; cascade is told.
- **EXPLAIN-KIND-REGISTRATION-LOCUS-AND-ADDRESS-FORM** — locus data is where
  the entry says (`KindFactRow` collection_address, registration,
  governs_root/glob; `narrate_kind` binds the row). But the locus is not
  either/or — hook carries both governs and a collection address, render
  both; commitment is omitted; the early return at read.rs:551-566 swallows
  the new strand for a row with locus but no guidance; the two prescribed
  tests contradict each other on the address form (unconditional rendering
  makes tests/read_verbs.rs:1065 red), so the address form rides the same
  guard; and the dropped "hosting kinds" inverse is a prerequisite — the row
  cannot say whether a kind is top-level or embedded (orientation has
  unit_shape file and no governs), and the inverse scan over `kind_facts`
  at read.rs:537 is the oracle for the address form. `narrate_kind` is :534
  not :530. The tests/common/mod.rs:614-631 builder sets both governs fields,
  so four tests' output grows (contains-asserts, stay green). Deferring the
  leaf schema is right; no 0024/0049 conflict.
- **MEMBER-ADDRESS-GRAMMAR-ONE-HOME** — module choice right; every src/read.rs
  cite is stale by ~+294 (`ParsedLeaf` :1129, `parse_leaf_address` :1139,
  `resolve_leaf` :1161, `impact_leaf` :1193); drift.rs call sites :1292,
  :1909, :1976, :2170, :2413. Missing edits that break build or acceptance:
  `resolve_leaf` must match `parsed.member` against both `features.id` and
  `host_address(outer_kind, id)` or the host-qualified leaf resolves nowhere;
  graph.rs:25's `use crate::read::{parse_leaf_address, resolve_leaf}` follows
  the move; `NestedAddress` and its fields become pub (graph.rs :333, :1618,
  :1620, :1653); the moved doc comment carries three graph-private intra-doc
  links that break under the crate-wide deny; `context_leaf` at read.rs:1372
  is a second caller the entry omits. Commit body carries the "why not
  src/address.rs" reasoning (engineering.md:15-16).
- **SDK-MEMBER-ADDRESS-GRAMMAR-ONE-HOME** — consistent with c60ddd3f, a pure
  writer extraction. Every declarations.ts cite is 26 low (host address
  :480, :502, :570, :725, :869; qualified :873; leaf :534; the cross-seam
  comment :529). Missing producer sites: emit.ts:295 (the bare-form lift, a
  grammar decision) and :815 (the host on payload members);
  sdk/test/refusals.test.ts:361 pins the refusal string byte-for-byte and is
  the test the "byte-identical" claim rides on. 21 sites across three
  modules, not nine. The bare spelling at emit.ts:799-800 is `kind:key`,
  which 0049 lists under Rejected — expose it as a lookup key, not an
  address writer.
- **NESTED-MEMBER-DUPLICATE-KEY-ADMISSIBILITY** — the refusal rule matches
  amended 0049 on every branch. Two mechanism claims wrong:
  `parse_nested_address` demands three segments so it returns None on a
  bare `kind:name` host, there is no host-address parser anywhere
  (`extract::host_address` is a writer), and the hand-parse inventory is
  five sites not four; and the read.rs claim is wrong twice — `why_impl` is
  :832 not ~538 and is not where a bare name resolves, `resolve` at :209 is,
  and since ab500f80 a bare nested key resolves to nothing in explain, so
  the work is an addition at resolve, not a refusal grafted onto why_impl.
  `member_named` returns Option and `resolves` returns bool, so the refusal
  is produced where dangling diagnostics are built, not at the lookup.
  Omitted, in-fence: tests/nested_member.rs:886
  (`a_bare_key_still_names_an_embedded_member`) inverts under this rule.
  Ordering NESTED → MEMBER-ADDRESS → SDK is right, but MEMBER-ADDRESS's
  gate.rs "import-path only" and its "member_named already keeps a bare key
  resolving" justification both go stale once NESTED lands — re-cut it
  after NESTED.

cascade will re-audit the re-cut entries as diffs; the session relays them
when plan has landed them.
