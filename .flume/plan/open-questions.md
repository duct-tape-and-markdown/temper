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

- `(committed-settings-kind)` — OPEN, live driver. The harness-authored
  `settings` residue (`sdk/src/assembly.ts`'s `settings: Record<string,
  unknown>`) folds untyped into the committed `.claude/settings.json` at
  emit (`declarations.ts`'s `settingsRows`), and the read side
  (`json_manifest.rs`'s `Manifest::opaque_fields`) catches the same keys
  whenever no collection address (`hook`/`installed-plugin`/
  `known-marketplace`) claims them — no kind governs the committed file as a
  whole, so no clause can type any of its residue. Two now-documented keys
  sit there unschematized: `autoMemoryEnabled` (bool) and
  `autoMemoryDirectory` (absolute or `~/`-prefixed path; honored at any
  settings scope; project-scope gated by the workspace trust dialog)
  (code.claude.com/docs/en/memory, retrieved 2026-07-26) — and this repo's
  own `.temper/harness.ts` already authors `autoMemoryEnabled: false` this
  way: a live consumer today, not a hypothetical one. 0036 shipped exactly
  this fix for `.claude/settings.local.json` (a fields-only kind, documented
  keys typed, residue opaque and named) but ruled only on the **local**,
  read-only file; whether the same posture extends to the **committed** file
  is silent, not decided. The committed file is materially harder: it is
  already an emit target sharing its top level with the three
  registration-member collection addresses, so a new kind here must not
  duplicate what those already model (0036's own "Rejected: a local
  registration manifest" concern, at committed-file stakes). What's missing
  is the human ruling on the mechanism, session-argued as 0036 was — not a
  waiting-for-demand fork, the demand already shipped.
  **Sharpened 09-07** (cascade-integrations, GH #40, observed at 913dfc2e;
  re-verified on disk this tick): the missing container kind is also why
  **the guard can be switched off through the manifest it protects,
  silently**. Replacing `temper guard .` with `true` in the `PreToolUse`
  command exits 0 at the boundary and at the next `check`. Two mechanisms,
  one cause. (i) `emit_manifest` (`drift.rs:1655`) computes the SHA of every
  represented manifest's bytes, but `drift::emit` pushes the rollup row
  carrying it **only** `if let Some((kind, name)) = &build.container`
  (`:1505`) — an ownerless manifest is emit-owned by path (`:1401`) and
  fingerprint-less, so `config_stale` (`:2713`, which walks rows carrying
  `source_path` + `emit_hash`) can never see the hand-edit. This repo's own
  `.temper/lock.toml` proves it: six `[[declaration.registration]]` rows name
  `manifest = "settings.json"` and no row anywhere carries its `source_path`.
  (ii) `RegistrationRow.fields` are seam-inbound and dropped from the lock by
  design (`drift.rs:3823`, 0018 "the projection is not the database"), so the
  hook's command exists in no committed row to compare — which is why the
  boundary half cascade also asks for (a bytes digest `manifest_write_findings`
  compares) is 0018's question, not a fixup. `pipeline.md` "Drift" promises a
  finding for "a **committed projection** differing from its byte fingerprint";
  a represented manifest with no container member has no fingerprint to
  differ from. The cheapest answer needing no new mechanism is this fork's
  own subject: a kind governing the committed file makes `build.container`
  `Some`, the rollup row appear under a real kind, and `config_stale` fire —
  where a synthetic `[[manifest]]` rollup row under a non-kind label would
  mint a phantom kind the by-kind corpus, `coverage.checked`, and
  admissibility must each be taught to ignore (the disease the layout-prose
  refactor capture documented before f07d4f84 drained it into a real
  `layout_prose` family). No entry filed: both faces rest on this ruling.
  No dependents.

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
  interim if (a)'s grammar change is too wide for 0.0.16. Dependents:
  HOOK-COLLECTION-ADDRESS-DUPLICATE-REFUSAL.

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

- `(containment-selection-family)` — OPEN, live driver (cascade spec
  use-case, 09-05). The one contract a real adopter wants that the
  vocabulary cannot spell today: "every rule body carries ≥1 directive," a
  per-host floor over the embedded values a member's body composes.
  Selections are by kind, by opt-in, and by incidence (`contract.md`,
  "selection"); containment — host member → its embedded values — is not an
  edge in the relation graph (`graph.rs` knows hosts only through
  `embedded_hosts` for scope judgments), so `count` over an embedded kind is
  corpus-wide and `degree` sees no containment arc. 0004 makes nested
  members members and says the contract layer ranges over them "exactly as
  over top-level ones," and a host's containment of its values is a
  relationship the program declares (`blocks()`), so it fits "declared,
  never mined." Candidate shape: containment joins the resolved edge set as
  a derived incidence family (one field per admitted kind, e.g.
  `contains:directive`), so the existing `degree` algebra spells the floor
  with no new predicate. `contract.md`'s own bar: "adding one is a
  deliberate language change" — a Decision is needed before any entry, not
  inferred here. **Sharpened 09-06** (cascade-integrations, live consumer
  evidence the Decision must cover or reject explicitly): cascade's actual
  clause is "every rule body carries ≥1 directive OR consult" — a floor
  over a *union* of admitted kinds, not one. One field per admitted kind
  plus `degree` spells a per-kind floor only; needs either a field-set
  filter on `degree` (a bound over `contains:directive` ∪
  `contains:consult`) or a union spelling in the incidence family.
  `Predicate::Degree` carries no field filter today though `contract.md`
  "selection" defines by-incidence as filtered by field and direction;
  every `ResolvedEdge` already carries `field`. No dependents.

- `(layout-own-span-leaf)` — OPEN, live driver (cascade-integrations audit
  of LAYOUT-COLLECTION-MEMBER-OWN-SPAN-LEAF, 09-06). A layout collection
  member's own paragraph span (the text directly under its heading, before
  any sub-heading) needs a leaf key to land under, but no reserved own-span
  leaf name exists anywhere in the corpus today: composed leaves are
  author-named (`NestedMemberRow.leaves` keyed by field name), and the
  SDK's `FRAMEWORK_KEYS` (`kind.ts:245`) reserves `prose` only as a
  member-level value, not a leaf key. Reserving one (`prose`, or another
  name) is a Decision — it collides with any author who slugs a
  sub-heading to that same name, and `node.body` includes sub-heading text
  so the own span must be cut at the first child heading regardless of the
  name chosen. **Third candidate, shipped 09-07** (f07d4f84, verified on
  disk this tick): `layout_prose` gives content with no authored identity
  its own emit-derived family, keyed by the host's address and the
  region's *position* rather than by a name (`LayoutProseRow`,
  `drift.rs:3023`). An own span could ride that shape — a row keyed by
  (host, kind, key) — reserving no leaf name at all, so no author's
  sub-heading can collide with one. Its cost is exactly what the leaf
  spelling buys: leaf predicates, leaf addresses and `explain`'s leaf
  narration would range over two sources where today every leaf is a
  `NestedMemberRow.leaves` entry. Session recommendation: still the
  reserved leaf key, with the sub-heading collision refused loud — one
  home for leaves outweighs one reserved name — but the ruling is the
  human's. Dependents: LAYOUT-COLLECTION-MEMBER-OWN-SPAN-LEAF.

- `(post-tool-use-placement)` — OPEN, live driver (GH #42 (ii),
  cascade-integrations, confirmed on disk at a13f6bf2). `install.rs:173`'s
  `POST_TOOL_USE_COMMAND` is byte-identical to `SESSION_START_COMMAND`
  (`:96`) — both `temper check . --reporter session-start` — and its group
  binds `BASH_MATCHER` (`:163`) under the same constituency test as the
  `PreToolUse` guard, so `reporter::context`'s pass-time disclosure (the
  `Checked:` block, `reporter.rs:158`, every `Severity::Note` plus the
  announcement, ~960 bytes of `additionalContext`) replays on EVERY Bash
  tool call rather than once at session open. The corpus owns no such
  surface: `distribution.md` "The placements and their enforcement modes"
  enumerates five — Keystroke, Session start, CI, the author's terminal, and
  Per tool call (`PreToolUse` = `temper guard`, three enforcement modes) —
  and `PostToolUse` appears **nowhere** in the evergreen corpus. It arrived
  via `cf67f291` (09-03, a `build:` commit), a placement build minted with
  no spec section owning it. Three candidate rulings, none derivable from
  the corpus as it stands: (a) unsanctioned — install stops wiring it, and
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
  dependents — the fixture entry ships without it.

- `(kind-declared-leaf-schema)` — OPEN, live driver (cascade-integrations
  assessment of EXPLAIN-KIND-ADOPTER-ENTRY-POINT, GH #47, 09-07). `explain
  kind:<x>` on a kind with **no member yet** — the adopter's actual moment —
  can teach nothing about the leaves an embedded child carries, because no
  kind declares a field set anywhere: `read.rs`'s `corpus_leaves` (:817)
  unions what existing members carry, and the SDK's `kind()` declaration
  (`sdk/src/kind.ts`) has locus, identity field, edge fields, content, shape,
  collection address, templates and guidance — no leaf schema. A member's
  fields are inferred at emit from the value it composes (`kind.ts:330`,
  `fields: ReadonlyArray<readonly [string, unknown]>`), so `NestedMemberRow`
  carries `leaves` as a bare `Record<string, string>` and `KindFactRow` has no
  leaf column to render from. Two candidate shapes: (a) the kind declares its
  embedded children's leaf names — a new declaration surface, so a deliberate
  language change under `contract.md`'s own bar, and one that must not become a
  second schema competing with the member type it would duplicate; (b) the SDK
  lowers `keyof T` off the embedded value type at emit, so the lock's kind row
  carries the leaf set as a *derived* fact and no author writes it twice
  (`engineering.md`, "Derived state is computed, never stored beside its
  source" cuts both ways here — the leaf set is derived from the type, and the
  question is whether the lock may record what only the compiler knows).
  Session recommendation: (b) — it adds no author-facing surface and the
  type already is the declaration; (a)'s cost is a second place to be wrong.
  Dependents: none — EXPLAIN-KIND-REGISTRATION-LOCUS-AND-ADDRESS-FORM ships
  the other two lines of #47's assessment without it.

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
