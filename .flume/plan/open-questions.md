# Open questions

Product/architecture forks not yet settled. Each is keyed with a `(slug)` so a
pending entry can declare `dependsOnForks: ["slug"]` and be held until resolved.

**Lifecycle (the anti-accumulation rule, John 07-06): this file holds OPEN
forks only.** Resolution = encode the ruling (corpus Decision, or the resolving
commit body) and **delete the record** — git history is the archive; "kept as
the decision record" is retired as a category. Reconciliation evidence (DATUMs)
goes in the plan commit body, never appended here. Rationale: this file is
inlined whole into every plan prompt — every dead line is a per-tick context
tax.

## Open forks

- `(multi-harness-projection)` — OPEN, strategic. Split 07-23 into two
  faces. The **read face** — `check` on a foreign environment's harness —
  is correctness downstream (`specs/intent.md`, "Positioning") and
  architecturally a pure data package: kinds are data
  (`specs/model/representation.md`, "Reach"), formats are shared engine
  code, kind rows carry `provider`. Its next probe is a falsification
  spike, not a feature (parked in `docs/ledger.md`): declare a
  `cursor-rule` custom kind in a testbed, point `check` at a real Cursor
  repo — zero `src/` changes proves the thesis; any engine change it
  forces is a custom-kind gap wanted found pre-0.1.0. First provider when
  demand shows: AGENTS.md (ruled 07-15: not a claude-code kind — Claude
  Code does not read it, docs retrieved 2026-07-15 — and the converging
  cross-tool surface). The **write face** — one member → N harnesses —
  stays parked under the 0035 evidence bar with its four open faces:
  per-harness capability mismatch, which harness is authoritative, lossy
  projection as verdict or error, and the counterpart-drift check (07-16
  war game, simulated: 2/8 personas rate it an adoption-blocker) —
  designed only against a real two-tool adopter, never speculatively.
  Watch condition: a portability tool (rulesync or kin) growing a checker
  re-times this fork. No dependents.

- `(lazy-grounds)` — OPEN, no live driver. Field demand (centercode, observed
  at 4cc3081): an eager read-only ground (`src`, `**/*.{cs,vb}`) materialized
  2250 members to resolve seven mention addresses (+45s). The wants: **lazy
  grounds** (on-demand address resolution — a stat per cited address, not a
  full materialization) and an optional content **needle** the gate asserts
  the resolved file still contains (the citation's meaning, where a content
  hash is alarm-fatigue and line numbers rot). Driver withdrawn in the same
  report (the consumer ruled their standards exemplar-free — no live-tree
  citations), so it waits under the 0035 evidence bar: lazy grounds change
  coverage/narration semantics (2250 members vs 7 resolved addresses is a
  model choice, not an optimization) — ratified against a real driver or it
  waits. Latent driver: a base-harness-style implemented-by mapping. The
  needle's design taste rides this record for that day. No dependents.

- `(external-commitment)` — OPEN, live driver (GH #29, human-ruled 09-03,
  PARK). No locus shape expresses "committed, not an emit target, still a
  roster member": every shape tried fails one of the three — a **file**
  locus's `local` commitment is read-side only and never enters the lock
  (drops "roster member" — no lock row to address or drift against); a bare
  `fields` (registration) locus has no file identity of its own to be
  "committed"; an **embedded** locus loads only through its host, so it
  can't stand as an independently-committed artifact. Proposed: a
  `commitment: "external"` class on the **file** locus (sibling to `local`,
  which 0032 ruled as a locus property, not a layer) — committed by the
  author, never written by `emit`, still takes a lock row, still
  addressable as an edge target, with drift defined as the file's bytes vs
  the lock's recorded hash (the same shape `local` denies itself by staying
  read-side-only). Needs a Decision before any entry: this is 0032's
  unresolved sibling case, session-argued, not inferred here. No
  dependents.
  **Field evidence 09-22** (a consumer's Classic ASP model on 0.0.18,
  observed): a read-only `source` kind over a committed code snapshot
  draws one `locus.undeclared-member` note per file (7 of 7). `local`
  would be a lie (the snapshot is committed), and a content fingerprint is
  reachable only by `include()`-ing the bytes into a projection
  (~209KB for five files). That is this fork's shape exactly. The
  fingerprint half may be better carried on the dependent pin
  (`docs/horizons.md`, `(declared-input)`). The roster-member half stays
  here.

- `(hook-member-identity)` — OPEN, live driver (GH #32). The `hook` kind is
  fields-shape at `hooks.<Event>` with entry shape
  `group-array(hooks;matcher)` and registers on `event`, so a member's
  address is `hook:<Event>`. Claude Code allows N matcher groups per event
  (code.claude.com/docs/en/hooks, retrieved 2026-07-15) and this repo's own
  harness carries two on `PostToolUse`, so the 09-03 ruling "refuse a
  duplicate address at declaration" was built (e4ff7bc6) and failed the
  self-host test at afterMerge — it would reject every real harness. The
  defect stands: an edge to `hook:SessionStart` resolves through the
  `kind:name` map to whichever member composed last, silently. The fork is
  what discriminates hook members. Candidates: (a) identity = `event` +
  `matcher` — the group-array's own key, so two members on one (event,
  matcher) ARE one Claude Code group and collide honestly; a matcher-less
  hook keeps the bare event; the address grammar gains the discriminator
  segment. (b) an author-supplied name — but a fields-shape member has no
  side channel in `settings.json`, so the name could only be derived,
  never stored (rejected on that ground unless the lock carries it). (c)
  keep event-only identity and refuse only an *edge* whose target address
  is ambiguous (more than one member), leaving unaddressed duplicates
  legal. Session recommendation: (a), with (c)'s edge refusal as the
  interim if (a)'s grammar change is too wide for 0.0.16.
  **Sharpened 09-07** (post-ship audit of 9c5c7a71, measured on disk this
  tick): (c) is no longer a mechanism to build — it is a branch to widen.
  `graph::member_lookup` now answers a three-way `Membership` (`One`,
  `Ambiguous(carriers)`, `Missing`) and both reference families already
  raise the ambiguity as a finding naming every carrier — `resolved_edges`
  for declared edges, `route_mentions` for mentions — with `explain`
  refusing the same spelling. But it is scoped to a **bare nested key**
  several hosts carry; the equality branch above it still answers
  `find(|f| f.id == identity)`, so two top-level members sharing one
  identity resolve to whichever the scan reaches first, silently. This
  repo's own harness is that case at three: `.claude/settings.json`
  carries three `PostToolUse` groups, which is why `check` reports
  `hook (6)` over four distinct addresses. So the cost of leaving this
  unruled is now visible inside one function — resolution total for a
  nested key, first-wins for a top-level one, both cited to the same
  `representation.md` ("member") sentence. Dependents:
  HOOK-COLLECTION-ADDRESS-DUPLICATE-REFUSAL,
  INSTALL-LIFTS-A-REGISTRATION-MEMBER — the lift must name each hook group
  it converts, so the discriminator is that entry's precondition too.

- `(layout-title-heading-admission)` — OPEN, live driver (GH #45(b)). Does a
  layout admit a document title — a lone leading H1 whose own span is
  preamble — at all? 0019's three primitives (prose, field, collection) name
  none, yet the H1-title-over-H2-sections shape is markdown's universal one
  and this repo's own `specs/intent.md` carries it. `Layout::read` binds
  regions to the document's *shallowest* heading level
  (`extract::body_heading_tree`), so with no title primitive a title-bearing
  layout document has its title silently admitted into whatever region binds
  first (a field or collection), swallowing every later region.
  LAYOUT-LOUD-READ-SWALLOWED-REGION makes that swallow loud (a finding names
  the consumed heading) but does not decide whether the model should instead
  give a title its own preamble-like treatment — that is this fork's
  question, unresolved. No dependents.

- `(nested-member-rename-identity)` — OPEN, live driver (GH #51 rename half).
  A layout collection member with no explicit `key` that is retitled
  silently becomes a new member: emit writes the new slug, the old row
  vanishes, and nothing names the change until an edge to the old key fails
  `graph.route`. 0019's consequences promise "fingerprints report a rename
  as a move, loudly"; `representation.md` carries only "an explicit key
  survives retitling"; the engine has no move detection over `nested_member`
  rows. Corpus/code collision to settle in the model body: either a
  same-host, same-kind, same-leaves row whose key changed is reported as a
  move at emit (advisory), or the model states that identity without a key
  is the heading and a retitle is a delete + create. The composed half
  shares the silence — renaming an embedded value's key in the program emits
  nothing that names the change (cascade probe F9) — so whatever the model
  rules for a layout retitle should bind a composed rename identically, one
  identity story for nested rows, not two. No dependents.

- `(post-tool-use-placement)` — OPEN, live driver (GH #42 (ii),
  cascade-integrations, re-measured on disk this tick after 842884f4 rewrote
  the module). `install.rs:176`'s `POST_TOOL_USE_COMMAND` is byte-identical
  to `SESSION_START_COMMAND` (`:105`) — both
  `temper check . --reporter session-start` — and its group binds
  `BASH_MATCHER` (`:166`) under the same constituency test as the
  `PreToolUse` guard, so `reporter::context`'s pass-time disclosure (the
  `Checked:` block, `reporter.rs:158`, every `Severity::Note` plus the
  announcement, ~960 bytes of `additionalContext`) replays on EVERY Bash
  tool call rather than once at session open. The corpus owns no such
  surface: `distribution.md` "The placements and their enforcement modes"
  enumerates five — Keystroke, Session start, CI, the author's terminal, and
  Per tool call (`PreToolUse` = `temper guard`, three enforcement modes) —
  and `PostToolUse` appears **nowhere** in the evergreen corpus. It arrived
  via `cf67f291` (09-03, a `build:` commit), a placement build minted with
  no spec section owning it, and 842884f4 has since **entrenched** it: the
  three gate hooks are now `hook` members the lift mints from one
  `GATE_HOOKS` table (`install.rs:204`), so the unsanctioned placement is a
  row in temper's own scaffolded program rather than a splice — which
  narrows (a) below to deleting a table row and widens the blast radius of
  leaving it unruled, since every newly-adopted harness now carries it as an
  authored member. Three candidate rulings, none derivable from
  the corpus as it stands: (a) unsanctioned — install stops minting it, and
  Bash-mediated writes stay CI's, the backstop the guard's own message
  already names verbatim to the author; (b) sanctioned, and
  `distribution.md` gains a sixth bullet: PostToolUse is the Bash-write
  drift check, its reporter carries **findings only** and is silent on pass
  (a `--reporter post-tool-use`, or `--quiet-on-pass` on the session-start
  one) — the never-silently-pass guarantee belongs to the *session-start*
  bullet, scoped to session open, while "Per tool call"'s three modes route
  findings, never disclosures; (c) sanctioned but folded into the existing
  "Per tool call" bullet as the guard's Bash half, taking the author's
  declared block/warn/note mode instead of a reporter. Session
  recommendation: (b) — the hole is real, CI-only leaves a Bash-written
  projection drifted for a whole session, and the fix is a reporter, not a
  new concept; (c) is unavailable because `PostToolUse` cannot deny a call,
  so it cannot honour the mode contract's `block` value. The objection (b)
  must answer: a full `check` per Bash call is a tree-scale cost on every
  command. No entry filed — the three rulings produce three incompatible
  entries (delete the wiring / add a quiet reporter / rebind to the guard's
  mode) with no common shippable core, and plan does not pick among them.
  No dependents.

- `(re-rooted-harness-disclosure)` — OPEN, live driver (GH note observed at
  a5101a9d, re-diagnosed on disk this tick). `check --harness <dir>` can gate
  `<dir>/..` and say nothing about it. `resolve_harness_path`
  (`main.rs:661`) answers `HarnessPath::Workspace { enclosing }` for any
  directory holding a file named `lock.toml`, and `harness_diagnostics`
  (`:708`) then discovers the corpus from the **parent** — deliberate, and
  documented: rooting a workspace at itself would read the lock from
  `<path>` while walking `<path>` for a corpus that lives beside it, so
  every declared requirement false-fires `requirement.unfilled`. The defect
  is not the derivation; it is that **nothing in the run names the root it
  resolved**. Measured this tick: `check --harness <tmpdir-with-a-bare-lock>`
  walked a 12.8k-entry `/tmp` and reported only `coverage.checked … harness:
  checked 0 members`, no announcement — which is why the field's 400s-per-spawn
  fanout stall (HOOK-COLLISION-FIXTURE-ROOTS-THE-RUN-AT-TMP) went undiagnosed.
  `adoption.md` "Install" carries the intent — "The verbs target one project's
  harness at an explicit path" — but `authoring.md` enumerates exactly what
  `check` announces ("every active local member, every dialed clause, and
  every joined lock"), and a re-rooted harness root is not on it, so the fix
  is a corpus change, not an inference. Three candidates. (a) Narrow the
  `Workspace` branch to a path literally named `.temper` — the `Root` branch
  already requires that name, so the two branches disagree on what a
  workspace is. **Rejected**: `emit --into <path>` (`main.rs:106`) takes an
  arbitrary directory, so a relocated workspace is a spelling temper already
  sanctions. (b) Disclose the re-root: when the resolved root differs from
  the path argument, `check` says so, as a fourth clause on `authoring.md`'s
  announcement enumeration — an input that judged the run beyond the path the
  author named. (c) Make `--harness` refuse a workspace spelling exactly as
  `install` already does (`main.rs:426` errors, naming the enclosing root).
  **Rejected**: it breaks `resolve_harness_path`'s own stated invariant, that
  a workspace and the harness root it governs always name the same harness,
  by making one flag disagree with the positional. Session recommendation:
  **(b)** — it changes no verdict and costs no capability, it is the only
  candidate that survives both objections, and it is the one change that
  would have turned this incident into a one-line answer. The objection (b)
  must answer: on the common path the resolved root and the argument agree,
  so the line must fire only on divergence or it is noise on every run. No
  dependents — the fixture entry shipped without it (76e29dc1, verified on
  disk this tick: its lock moved under `.temper/` and the case now pins
  `rule (0)` to prove which root the run walked). The gap the fixture's own
  diagnosis exposed is untouched: nothing in that run names the root.

- `(embedded-edge-dangling-judgment)` — OPEN, candidate not yet ruled
  (cascade-integrations, GH #52, 09-06). SDK-MEMBER-TABLE-NESTED-EDGE-TARGET
  fixes an embedded value's edge field resolving another host's embedded
  value, but leaves open how a *dangling* embedded edge target should be
  judged, and by which verb. Candidate split by the citing field's grain: a
  top-level member's edge field lowers to an assembly `edge` row judged at
  `check` under `graph.route` (an error, not advisory); an embedded value's
  edge resolves at `emit` instead. Finer candidate: `edgeTargetFacts`
  returns a dangling marker rather than throwing; `recordingView` already
  observes whether a render hook selected `v.targets.<field>` — selected
  and dangling refuses (pipeline.md "Refusing" already holds this),
  unselected rides the row and `check` fires `graph.route` instead. Not
  lazy resolution: 0049 says every address is resolvable, and a lazy path
  would let a hook that reads `v.targets` render a fabricated reference
  (0048). No dependents yet — no entry currently needs this ruled.

- `(builtin-relocation-unnamed)` — OPEN, live driver (post-ship audit of
  77581333, this tick). The evergreen corpus never names **relocation** — a
  corpus kind reusing a built-in's name under its own facts. `rg relocat
  specs/` returns one hit, in `specs/decisions/0016`, outside the read path.
  Three layers nevertheless judge the same-name case today, by three
  different rules: `compose.rs`'s `row_relocates_builtin` (:1105) decides
  *structurally* (format/unit_shape/registration agree) and its overlay
  carries the row's `governs` **always** (:441), so relocation there MEANS a
  locus change; `kind.ts`'s `relocate` (:512) decides by *provenance* (the
  `facts.relocates` marker) and rides the locus through unchanged; and
  `declarations.ts`'s `kindsInPlay.admit` (:380) decides by *string*,
  first-wins, discarding the second silently. `builtins.md`'s opening reads
  against the whole population — "Kind identity travels by import, never by
  string: two providers are two modules, so collision is impossible and no
  name-qualification scheme exists" — which is true of two providers and
  silent on one corpus redeclaring one provider's name, the case all three
  layers actually handle. What is missing is one corpus sentence: may an
  adopting corpus redeclare a built-in's name under its own locus, and what
  is that called. Session recommendation: sanction it in
  `representation.md` "kind" beside "ownership, not privilege" — the ability
  is already implied there (a kind is data; the author owns its facts) —
  and let provenance be the single authoring answer while the structural
  test stays the row-side one (`row_relocates_builtin`'s doc already
  reconciles the two). The objection it must answer is builtins.md's own: if
  one corpus may reuse a name, the lock is a string-keyed medium and the
  qualification question returns the moment a second provider ships
  (`(multi-harness-projection)`'s read face puts `provider` on kind rows).
  No dependents — RELOCATE-CANNOT-MOVE-A-BUILTINS-LOCUS and
  KINDS-IN-PLAY-NAME-COLLISION-REFUSAL both ship under the implied ability;
  this fork asks the corpus to name it.

- `(build-version-identity)` — OPEN, live driver (cascade-integrations,
  observed at ae74bf49, cites re-derived at ea7625e3). A source build and
  the published binary are indistinguishable: `main.rs:46` declares
  `#[command(name = "temper", version, …)]`, so clap prints
  `CARGO_PKG_VERSION` — `Cargo.toml:3`, `0.0.18` since the 0.0.18 release
  (c60c976a) — for every build between tags, and an adopter probing an
  unreleased engine under pnpm scripts reads back the released string. The report's second half is **not** a defect:
  `sdk/bin/temper.js` resolving the platform `optionalDependencies` package
  whatever PATH says is exactly `distribution.md` "What ships — three
  channels" channel 2 ("pinned by the SDK at an exact version"); identity
  travels by pin there, deliberately, and no PATH may override it. What the
  corpus is silent on is what a build says about *itself*: `distribution.md`
  speaks to the pin and to CI's `emit --frozen` byte-compare, never to build
  provenance, and no other section does. The ruling is load-bearing because
  the version is not display text — `src/lib.rs:17`'s `VERSION` is written
  into two artifacts: the bundled plugin manifest's `version` field
  (`bundle.rs:248`, asserted at `:416`) and the SARIF driver version
  (`reporter.rs:268`). Three candidates. (a) a `build.rs` `git describe`
  suffix (`0.0.18-dev+<sha>`) — the one an adopter reads back, but it makes
  the version a *build-environment* fact, so a tarball build and a git build
  of one tree disagree, and channel 3's bundle stops being reproducible from
  the tag. (b) the shim honours an explicit override (an env var naming a
  binary) — closes the pnpm-script half, moves no artifact byte, and says
  nothing in the output. (c) build provenance on `--version`'s long output
  only, leaving `VERSION` untouched, so both emitted artifacts stay
  tag-identical and the adopter still reads the sha. Session
  recommendation: **(c)**, with (b) as the pnpm-script half and (a)
  rejected — overloading a constant that two artifacts embed trades a
  diagnosis for the reproducibility the offering's byte-compare rests on.
  The objection (c) must answer: a second version surface is a second place
  to be wrong ("One job, one home"), and a script reads the short form. No
  dependents.

- `(directive-relation-scope)` — OPEN, live driver (inbox note observed at
  4b25d0f3, re-verified on disk this tick). `contract.md` "edge" states the
  import-directive locus generically — "a reference the target format itself
  executes (a memory file's `@path` import), resolved by path" — but the
  engine binds it to one kind: `Primitive::Directives` is constructed at
  exactly one site, `builtin_kind.rs:322` (`memory`), no `KindFactRow` path
  spells it (compose's primitive lowering carries no `directives` case, the
  SDK exports none), so an `@`-line in any other kind's body is extracted by
  nobody. Claude Code: "CLAUDE.md files can import additional files using
  `@path/to/import` syntax … Imported files can recursively import other
  files, with a maximum depth of four hops"
  (code.claude.com/docs/en/memory, retrieved 2026-09-22). The docs are
  **silent** on an `@`-line in a rule loaded *as a rule*, and that asymmetry
  is what makes this a model question rather than a bug: a file reached
  *through* an import has its own `@`-lines expanded whatever its kind, so
  `CLAUDE.md → @.claude/rules/r.md → @CLAUDE.md` is a ring the runtime walks
  and temper cannot see. `graph::acyclic` is scoped to the import relation
  (`graph.rs:245`) and shipped on the reachable memory ↔ memory half
  (09ad535a); both it and `reachable` miss the middle hop. The question the
  corpus must answer: is the directive relation **kind-scoped** (only a kind
  declaring a directive primitive carries executed references) or
  **target-scoped** (any file an import reaches carries them, whatever kind
  governs it, to the documented four-hop cap)? Candidates: (a) keep it
  kind-scoped and give the SDK a `directives` primitive an author may compose
  onto any kind, so the ring above becomes expressible by declaring it on
  `rule`; (b) follow the target — a directive edge's target is re-extracted
  for `@`-lines whatever kind governs it, matching the runtime, but an import
  target need not be a member at all, so the relation grows nodes outside the
  roster; (c) rule the middle hop out of scope, since it is undocumented for
  a rule loaded as a rule, and model only what the docs state. Session
  recommendation: **(a)** — `representation.md` "kind" already says a kind is
  data and its extractor is composed from that data, so kind-scoping is the
  standing model and the gap is an SDK one, while (b) puts non-members in the
  graph and (c) leaves a known ring unmodellable. The objection (a) must
  answer: the runtime expands the middle hop whether or not the author
  declared it, so a harness declaring nothing still carries a ring temper
  reports green — the mechanism is right and the *default* is wrong. No
  dependents; nothing is built on it until it is ruled.

- `(unmodeled-surface-registry)` — OPEN, live driver (refactor capture
  `build-unmodeled-surface-dormant`, filed at 2dba7c0c, re-verified on disk
  this tick). `coverage.unmodeled-surface` can no longer fire.
  `coverage_note::check` has exactly one production call site
  (`gate.rs:660`), handed `builtin_kind::definitions()` — the full built-in
  set, never a scope-filtered one, and never the `overlaid_builtin_kinds`
  sitting beside it — and both `KNOWN_SURFACES` rows are governed whole
  under it: `.claude/settings.json` by 0050's `settings` container
  (`builtin_kind.rs:552`), `.mcp.json` by `mcp-server`, whose collection
  spans the whole manifest (`coverage_note.rs:291`). So `whole` is true on
  every real invocation and `segment_coverage` answers `Full` every time.
  Measured at cefac0f0: a tmp harness carrying
  `.claude/settings.json = {permissions, env}` and
  `.mcp.json = {mcpServers, somethingElse}` — four present keys no segment
  kind governs — reported `coverage.checked` alone, zero findings. Both
  branches behind the verdict (`coverage_note.rs:146`, `:161`), the
  `segments`/`Segment` model (`builtin_kind.rs:51`) and
  `manifest_top_level_keys` (`:334`) exist only to decide something nothing
  reaches, and three fixtures withhold a built-in from scope to observe it
  (`tests/coverage_note.rs:93`, `:386`, `tests/check_cost.rs:397`) — two of
  them saying exactly that in their own comments, and a fourth asserts the
  rule's *absence* over the always-empty vec (`tests/hook_kind.rs:195`, an
  `.all()` no ruling can currently falsify). That is the vacuity class
  `engineering.md` "A green verdict is proven non-vacuous" names. The
  capture's subtraction list is narrowed by the re-verify: `with_locked_kinds`
  **survives** — its `governing_kinds` also feeds the
  `coverage.unclaimed-entry` strand (`coverage_note.rs:176`), where a locked
  custom kind's `governs` still suppresses a `.claude/` stray.
  What the corpus does not settle is what the advisory is *for* now.
  `builtins.md` "The coverage bar" speaks only to the vocabulary growing by
  documented capability; the one body sentence naming the posture — the
  supporting-doc bullet's "supporting files of other types remain unmodeled
  and are named as such, the `settings.json` partial-governance posture" —
  cites a precedent 0050 retired. Candidates. (a) **Retire it**: the rule,
  the `segments` column, `Segment`, `manifest_top_level_keys` and both
  branches go, `KNOWN_SURFACES` shrinks to the unclaimed-entry exclusion
  list, and the supporting-doc sentence needs a new precedent. (b) **Keep it
  and fix its classification**, which makes it non-vacuous on the registry it
  already has: `.mcp.json` has no container member at all — no kind
  fingerprints that file — so its `somethingElse` reaches no row, no
  projection input and no finding, where settings.json's residue is at least
  carried by 0050's container. Give `.mcp.json` a segment model (`mcpServers`
  its one governed segment, every other present key residue) and the measured
  case becomes a named gap, with no new external fact. (c) **Keep it and grow
  the registry** with a documented `.claude/` surface no built-in governs —
  needs a cited external fact this tick does not have, and "the vocabulary
  grows by documented capability" points the other way: a documented surface
  temper can model earns a *kind*, not a permanent advisory. Session
  recommendation: **(b)** — it is the only candidate that closes a measured
  silence rather than ratifying one, and it costs no capability. The
  objection (b) must answer: if the Claude Code docs schematize `.mcp.json`
  as exactly `{mcpServers}`, a key beside it is malformed input rather than a
  coverage gap, and (a) wins — that citation is the ruling's first input. No
  entry filed: the three rulings produce incompatible entries (delete the
  rule / re-segment `.mcp.json` / add a registry row) with no common
  shippable core. No dependents.

- `(empty-contract-unspellable)` — OPEN, no adopter driver (refactor capture
  `build-root-default-contract-fence`, filed at ca48168a; generalized on disk
  this tick). An authored **empty** clause array is unspellable: rows-or-default
  reads "no rows for this owner" as "the author declared nothing" and reinstates
  the embedded default, so `expect(kind, [])` and a root `contract: []` both
  compose the full built-in contract. One branch, written twice —
  `compose::builtin_contract` (`compose.rs:1424`) per kind and
  `compose::root_contract` (`:251`) for the root. `builtins.md` "Default
  contracts" promises the opposite: "overriding is array surgery in the language
  the author already writes — no layering rules, no precedence table", and "the
  built-ins are first-party instances of it, never a privileged form". The empty
  array is the one surgery the mechanism reverses, and the reversal is the
  privileged form that sentence disavows. The collision is sharper than a missing
  feature: the branch's own stated rationale is **forward compatibility** — "a
  lock committed before the root contract shipped still gets the shipped default
  rather than silence" (`compose.rs:246`) — and the lock, being row-shaped, gives
  "this lock predates the default" and "this author declared none" a single
  encoding. Candidates. (a) Give the lock an explicit discriminator (a
  contract-declared marker per kind row, and a root one), so absence stays the
  old-lock case and the empty array reaches the engine as itself. (b) Rule it
  intended and say so in one `builtins.md` sentence: a default contract is a
  **floor**, so the empty array is not an override but a no-op — no code, no
  column. (c) A sentinel `none()` clause — **rejected**: a predicate meaning "no
  predicates" is exactly the precedence table the section refuses. Session
  recommendation: **(a)** — the two questions the branch collapses are genuinely
  different, and (b) makes the corpus's own "never a privileged form" false for
  the one array surgery an author is most likely to try. The objection (a) must
  answer: no adopter has asked for the empty contract, so the 0035 evidence bar
  routes it to (b) until one does — and a new lock column is a migration plus a
  fourth thing every writer must set. No entry filed, no dependents:
  ROOT-DEFAULT-CONTRACT-SHIPS ships under today's rule either way.

- `(represented-harness-gate-upgrade)` — OPEN, live driver (842884f4's own
  stated deferral, measured on disk this tick). Since that commit temper's
  gate rides the program: `scaffold` mints one `hook` member per `GATE_HOOKS`
  row (`install.rs:204`) and `emit` is `.claude/settings.json`'s one writer.
  But `run_represented` (`:530`) lifts only when `harness.ts` is absent —
  `let scaffolded = if already_scaffolded { 0 } else { scaffold(…) }`
  (`:556`) — so a harness represented before that change, or one whose author
  deleted a gate hook module, has no member at those events. `gate_outcome`
  (`:747`) then answers `Conflicted` and `gate_installed` names it, forever:
  re-running install re-wires nothing. `adoption.md` "Install" promises
  "Re-running install converges, placements following the lock's current
  contents", and on the represented path a gate hook is no longer a placement
  install performs but a **member the lock carries**, so the sentence's
  subject no longer covers the case — that silence is the fork. Three
  candidates. (a) Install re-lifts: it mints the missing modules and appends
  their import + composition to `harness.ts`. Install is the one verb that
  writes program sources, so this breaks no emit fence — but it means editing
  a TypeScript file the author has since restructured, machinery temper does
  not have and would carry permanently. (b) **Report-only**: `Conflicted`
  grows a remedy — the module path to add and the one import line — and
  install writes nothing. (c) Install writes the missing
  `.temper/hooks/<Event>.ts` modules but never touches `harness.ts`, leaving
  the author one line to add; **rejected** — an unimported module in a
  represented program is exactly the unreached member `reached-from`
  (62d9ac0c) was built to indict, so the remedy would author the defect.
  Session recommendation: **(b)**, and the decisive argument is not the
  migration but the standing case: an author who deliberately deletes the
  PostToolUse gate hook — the live `(post-tool-use-placement)` fork is
  precisely that wish — must not have install silently re-add it on the next
  run. `Conflicted` on a removed member is the *correct* verdict; what is
  missing is only that it says nothing about what to do. The objection (b)
  must answer: a one-time migration for harnesses adopted before 842884f4 is
  then hand-work temper narrates but never performs, and adoption.md's
  "installs the tool whole" reads as a first-run-only promise. No entry
  filed: (a) and (b) produce incompatible entries (TS-editing machinery vs a
  report string) with no common shippable core, and plan does not pick among
  them. No dependents.

- `(unique-over-a-list)` — OPEN, live driver (consumer report on 0.0.18 and
  0.0.19, reproduced at 4756671c and re-verified on disk this tick). A
  `unique` clause over a list-valued field decides nothing and exits 0:
  `duplicates` (`engine.rs:936`) reads `Selection::values` (`:706`), which
  keeps `value.as_scalar()?` only, so a `FeatureValue::List` contributes no
  value to the multiset it counts. That is invariant 6's silent pass, so
  this fork owes a ruling rather than a deferral — the sibling half ships as
  MEMBERSHIP-READS-A-LIST-VALUED-FIELD, whose per-element reading follows
  from "drawn from the satisfiers' values" with no new concept; `unique`'s
  does not. The reporter's proposed remedy — refuse the clause at
  **admissibility** — is unavailable as spelled: that tier runs "before any
  member is read" (`admissibility.rs:6`), and a field's list-ness is member
  data, never a declared fact (`ValueType` is the *parsed value's* kind; only
  a `type` clause declares one). Whatever is ruled fires where the judge
  meets the value. Candidates. (a) **Flatten**: every element across the
  selection must be unique, so two members sharing an element collide and a
  repeat inside one member's list collides with itself. (b) **Flatten across
  members only**: each member's list is deduplicated first, so `unique` asks
  only that two members share no element. (c) **Judge-time refusal**: a list
  under `unique` is a finding naming the member and the field, loud without
  ruling the semantics, and the author narrows with a `type` clause. Session
  recommendation: **(a)** — `unique` names a value that must not repeat
  across the selection, the selection's value multiset is the object it
  counts, and reading a list as a plural feature is the same move membership
  makes; one reading for both set predicates is the "one algebra over
  selections" `contract.md` "selection" already claims, where (b) gives the
  two siblings two flattening rules and (c) leaves a spellable clause
  permanently undecidable. The objection (a) must answer: a list field is
  usually an unordered tag set where a within-member repeat is authoring
  noise and not a contract breach, so (a) fires on a shape `unique` was never
  aimed at — which is (b)'s whole case. `count` is out of scope: it carries
  no field and counts the selection itself. No dependents —
  MEMBERSHIP-READS-A-LIST-VALUED-FIELD ships under today's `unique` reading
  and names the constraint in its own `files[]`.

- `(guard-locus-binding-clause-gated)` — OPEN, live driver (post-ship sweep
  of b0665cae, measured on disk this tick). Since that commit the
  undeclared-member fact is a **clause** on `check`: no `locus-declared`
  clause bound, no finding — pinned by
  `a_root_contract_binding_no_locus_declared_clause_reports_no_undeclared_member_at_all`
  (`tests/acceptance.rs:736`), whose own message rules it, "a stranger at a
  governed locus is not a fact the tool pushes unasked". `temper guard`
  pushes exactly that fact unasked. `guarded_loci` (`main.rs:685`) assembles
  its locus set from the embedded kind data overlaid with the lock's
  relocations and reads **no clause row at all**; its four exclusions
  (`local` commitment, no `governs`, a `collection_address`, a `.`-rooted
  locus) are all kind facts. So `matches_governed_locus`
  (`install.rs:1093`) binds a write into any governed locus the program
  declares no member at, and the mode decides: `block` denies the call. The
  commit body named the omission deliberately ("enforcement mode still
  denies a guarded write to a harness that binds no clause at all") — a
  build-commit ruling, not a corpus one. The corpus is genuinely silent:
  `rg guard specs/` outside `specs/decisions/` returns one line on this
  placement, `distribution.md`'s "Per tool call" bullet, which states only
  the mode vocabulary and never what the guard *binds*; contract.md's three
  `guard` hits are the unrelated clause-guard predicate. The blast radius is
  not the `.claude/` tree: a custom kind over ordinary source — the
  read-only ground shape `(external-commitment)` carries field evidence for,
  `src` / `**/*.cs`, committed so `local` would be a lie — reaches
  `guarded_loci` unexcluded, so every Write/Edit to a `.cs` file that is not
  a declared member binds, at warn by default and denial under `block`. That
  is the "a hostile gate gets disabled" failure `distribution.md` names for
  the session-start placement, arriving at the one placement that can
  actually deny. Three candidates. (a) **The clause decides whether the fact
  binds, the mode decides what happens**: `guarded_loci` drops the locus set
  where no `locus-declared` clause is bound, keeping the two axes the corpus
  already separates (severity at `check`, enforcement mode at `guard`)
  intact. (b) Map the clause's severity onto the verdict — `advisory` softens
  to `warn`/`note`, `required` takes the declared mode. **Rejected as
  spelled**: it collapses the two axes into one and the corpus states no
  mapping from a severity to the `note`/`warn` split, which is about *where
  the finding goes*, not how heavy it is. (c) Rule the guard a write-time
  boundary rather than a contract judge, and say so in `distribution.md`'s
  bullet, so the divergence is declared. Session recommendation: **(a)** —
  the guard is already lock-grounded by construction ("Placements are
  lock-grounded, never assumed"), the clause rows sit in the very
  `declarations` value `guarded_loci` is handed (`main.rs:364`), so the
  filter is a scan of already-parsed rows and the per-tool-call cost bound is
  unmoved; and (c) ratifies a boundary that denies an adopter's ordinary
  source edits. The objection (a) must answer: the guard's *first* binding —
  a direct edit to an emit-owned projection — is not clause-gated either and
  no one wants it to be, so "the guard consults clauses" needs a rule for
  which bindings do, not a blanket one. No dependents; nothing is built on it
  until it is ruled.

## Kept on purpose — deliberate asymmetries (re-read every tick)

Every asymmetry below is a **choice with a condition**, not a fact. When its
condition arrives, it is the next break. If work touches one, surface it.

- **A pack is a skill — no skill-package kind** (human-ruled 07-15, 39a4833;
  reaffirmed by 0025's Rejected list, 82c816e: "a separate skill-package or
  nesting kind for supporting docs — the built-in already owns the shape; a
  parallel kind would be the duplicate-surface disease"). The condition is a
  consumer who *cannot* express a pack with the built-in `skill` plus its
  nested reference documents. The 07-16 datum that looked like demand — the
  centercode `supportingDocs()` factory, minting one nested-root kind per
  skill directory — is **routed, not pending**: it was ergonomics standing in
  for a template fact the spec already declares and the SDK lacks.
  TEMPLATE-FILE-CHILD-FACT shipped that fact (794678f), 0027 (abe5d5d)
  resolved `(nested-file-child)`, and SKILL-NESTED-REFERENCE-DOCS **landed**
  (a7a8cc1): `skill` templates one file-child layer at its directory's
  markdown and `supporting-doc` is that layer's kind, verified on disk. So
  the factory now deletes against `skill` + `supporting-doc`, and this
  record's condition — a consumer who *cannot* express a pack with the two —
  is what a future pack argument must clear.

- **Default-contract auto-adoption** (a bare harness gets the built-in kinds
  checked with no assembly declaration) — kept for the zero-config front door;
  the engine embeds a built-in lock, the default contract in declaration shape,
  so a lockless harness is still fully gated (`specs/model/pipeline.md`, "The
  lock"). Data, not code.

- **Format implementations are engine code** (the frontmatter adapter, the
  `json-document` reader beside it since 3ed8d2b, and `toml-document` since
  09ef5ea) — kept because an external format's mechanics are temper's to
  implement once; the kind that selects them is data
  (`specs/model/representation.md`, "kind": a kind is data, its extractor
  composed from that data). Grows only by deliberate addition, and each of
  the inventory's two additions was exactly that. The third entry sharpened
  the record rather than straining it: `toml-document` is a **read face with
  no write twin**, so `project_bytes` now returns `Option<String>` over an
  exhaustive `Format` match — a format that cannot be written refuses at the
  writer rather than inheriting a fall-through. The next format answers that
  match by construction, which is what keeps "deliberate" mechanical here.

- **Stale cites: intra-doc links are gated, prose rides.** A doc-comment
  cross-reference that drifts is temper's own no-drift thesis turned inward.
  Broken intra-doc links are **gated for public and private items alike**:
  crate-level `#![deny(rustdoc::broken_intra_doc_links)]` plus
  `cargo doc --no-deps --document-private-items --quiet` at afterMerge
  (`.flume/chain.ts`'s `docGate` — re-verified on disk 2026-08-26, this tick:
  24b22045 added the flag and fixed the 8 sites it surfaced, draining
  `.flume/friction/plan-private-item-doc-link-gate.md`). The
  `rustdoc::private_intra_doc_links` lint (a public doc linking to a private
  item) stays advisory, unchanged. Prose staleness no linter can check — a
  "sole consumer" claim, a line-number pointer, a stale invariant paragraph —
  **rides** the next entry that opens the file and discharges when that entry
  names it (never a standalone entry), and is tracked **nowhere**: the
  per-instance ledger was itself the per-tick context tax this rule exists to
  avoid. The 2026-07-23 sweep cleared the standing backlog (23 links, 13 prose
  cites) and set the public-item gate; 24b22045 closed the private-item gap;
  git history holds the rest.

- **`.flume/` is ungoverned by temper** — the machine that builds temper is not
  yet under its gate; a candidate governed corpus once the custom-kind story
  proves end to end (`specs/model/representation.md`, "Reach"). Narrowed
  2026-07-09: the existence half of `.flume/prompts/{plan,build}.md`'s two
  `.claude/` pointers (`pending-entry` rule, `capture-friction` skill) is now
  graph-tracked — `harness.ts` declares both as `required` assembly
  requirements, each member `satisfies`-links to its own (a real
  `requires`/`satisfies` edge needs no `.flume/`-side kind; `emit`/`check`
  now refuse if either loses its satisfier). What remains genuinely
  ungoverned: the prompts' prose *spells the identifier* outside any gate —
  a member rename moves the graph edge with it but leaves the prompt's text
  stale-but-harmless (neither trigger mechanism reads the prose).
  **Re-armed 2026-07-18** (was: kept as cosmetic): the operating layer
  grew past the narrowing's premise — the amendments channel (0044), the
  protocol's slit enumeration, and the sweep-frontier mechanics now span
  prompts, rules, and READMEs as hand-synchronized restatements, the
  drift class temper gates. Organizing it under the dogfood is the
  ledgered next-session focus (interactive-session work, not a pending
  entry — the flume harness is outside build's fence).

- **`docs/` is candidate intent, not intent** — human territory,
  fence-excluded; plan never reads a horizon entry as intent.
