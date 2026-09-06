<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at bf4b4168 (session; correction to GH #48 and to pending entry
  SECTION-CONTAINS-LABEL-MARKER-COLLISION, verified by cascade-integrations
  at github.com/duct-tape-and-markdown/temper/issues/48#issuecomment-5561237527)
  — the entry's files[] claims `clause_label` folds a section_contains
  clause's *heading* in as the field segment and drops only the marker.
  Wrong: the label is stamped at emit from the row's `field` column
  (`drift.rs` `stamp_clause_label` → `contract::clause_label(owner,
  predicate, row.field)`), and the SDK lowers `sectionContains(heading,
  marker)` with heading and marker under `section` and `field` undefined
  (`contract.ts` ~197, `declarations.ts` ~103), so every section_contains
  on a kind labels `<kind>.section_contains` — heading AND marker dropped;
  `require_sections` has the same gap. `contract.rs`'s
  `Predicate::SectionContains { heading, .. } => Some(heading)` is a
  different consumer's "constrained field", not the label. If build takes
  the entry as written it folds in the marker and leaves the heading
  collision. Amend: both heading and marker fold into the label segment
  (and `require_sections`'s section list), and the regression test is two
  `sectionContains` compiled THROUGH THE SDK — `tests/lock_declaration_rows.rs`
  ~2907 proves the refusal only on a hand-written lock, which is why this
  was never seen. Blast radius: label values re-spell on section_contains /
  require_sections rows only.

- observed at bf4b4168 (cascade-integrations, for fork
  `(containment-selection-family)`) — consumer evidence the Decision must
  cover or reject explicitly: cascade's actual clause is "every rule body
  carries ≥1 directive OR consult" — a floor over a *union* of admitted
  kinds. One derived field per admitted kind (`contains:directive`) plus
  `degree` spells a per-kind floor, not a union; the candidate needs either
  a field-set filter on `degree` (a degree bound over `contains:directive`
  ∪ `contains:consult`) or a union spelling in the incidence family. Source
  facts for the drafter: `Predicate::Degree` carries no field filter today
  though `contract.md` "selection" defines by-incidence as filtered by
  field and direction; every `ResolvedEdge` already carries `field`;
  `graph::degree` builds adjacency from resolved reference and mention
  edges only; hosts reach the graph only through
  `gate::embedded_hosts_by_source`, no containment arc exists. Minimal
  composition (inferred, none of it exists): gate synthesizes one
  `ResolvedEdge` per `nested_member` row (host → (kind,key), field
  `contains:<kind>`); `Degree` gains an optional field filter accepting a
  set; `graph::degree` filters adjacency by it.

- observed at bf4b4168 (cascade-integrations, verified by the session;
  corrections to pending entry NESTED-MEMBER-DUPLICATE-KEY-ADMISSIBILITY,
  GH #51) — four claims in the entry mis-build as written. (1) Mechanism:
  not "whichever composed last". `compose::embedded_features_by_kind`
  pushes every host's rows into one Vec; `graph::resolves` is a boolean
  `.any()`, so a duplicate `(kind, key)` resolves an edge to an ambiguous
  identity with no finding; only `explain` picks (read.rs ~522 collapses
  same-kind matches to the first). (2) Neighbour: not
  `satisfies_label_admissibility` — it reads `by_kind`, whose Features
  carry no host (compose.rs ~1212 lifts `row.key` as id and drops
  `row.host`), so a mirror cannot name both hosts; the neighbour is
  `nested_member_admissibility` (admissibility.rs 60), which iterates
  `declarations.nested_members` where `host` is present. (3) Cites:
  `graph::member_at` serves the mention path only; resolution is
  `resolves` (~1609) from `resolved_edges` (~1264). (4) Scope — the
  substantive one: refusing every cross-host duplicate hard-codes the
  alternative 0049 rejects by name. Under 0049's own grammar
  `spec:alpha/invariant/x` and `spec:beta/invariant/x` are distinct
  addresses, not coincident; 0049's Consequences sentence ("refuses
  duplicate (kind, key) across hosts as coincident") contradicts its
  Decision and is being amended (session draft, John rules). Corrected
  entry: coincident = same host, same kind, same key (malformed lock at
  admissibility); a *bare* target or bare `explain` name matching more
  than one nested member of the kind is refused at resolution (check
  finding under `graph.route`; explain refusal) naming every host, which
  requires host to ride into Features or the graph Node; the full
  address always resolves. Same fix family as the 0049-narrowing note
  above — one entry, or two chained on graph.rs.

- observed at bf4b4168 (cascade-integrations, verified by the session;
  corrections to pending entry SDK-MEMBER-TABLE-NESTED-EDGE-TARGET, GH
  #50) — line cites and mechanism hold; the prescription is incomplete in
  three places and a literal build hits a second throw. (5) Index both
  spellings — bare `key` within the target kind and the full
  `<host-address>/<kind>/<key>` — never `kind:key` alone (0049 Rejected);
  refuse a duplicate bare key (memberTable ~706 is last-wins for
  registration members while `uniqueMap` refuses elsewhere — use the
  refusing path); stop treating any `:`-bearing leaf as a whole key at
  ~270 when it contains `/`. (6) After the lookup, `isProjected(target)`
  is false for an embedded locus (~604) and `projectionPath` (~206)
  throws on it: branch before both, derive `path` from
  `projectionPath(host)`, and widen the table's value type — a nested
  value is an `EmbeddedMemberValue`, not a `Member`. (7) Check-side: the
  full nested address does not resolve at check (`graph::target_identity`
  has no `/` grammar; `embedded_member_features` is host-blind), so an
  SDK-only entry is end-to-end for bare-unique keys only — scope it so,
  or add graph.rs/compose.rs to files[] and chain on the #51 entry.
  Tests: `sdk/test/refusals.test.ts` ~292 asserts the current throw and
  narrows to a truly absent target. Not lazy resolution: 0049 says
  resolvable, and a lazy path lets a hook that reads `v.targets` render a
  fabricated reference (0048). For GH #52 (no entry): the honest split is
  by the citing field's grain — a top-level member's edge field lowers to
  an assembly edge fact judged at check under `graph.route` (an error,
  not advisory); an embedded value's resolves at emit. Candidate, not
  ruled: `edgeTargetFacts` returns a dangling marker; `recordingView`
  already observes whether the hook selected `v.targets.<field>`;
  selected + dangling refuses (pipeline.md "Refusing" holds), unselected
  rides the row and check fires `graph.route`.

- observed at bf4b4168 (cascade harness pass, GH #57; verified by the
  session) — `emit` places the managed-projection banner on a
  frontmatterless markdown projection (`drift.rs` emit_one, the banner
  branch bccf42c0 moved from install) but never the frontmatter `#` note;
  only `install` writes the note (`install.rs` evaluate_placements →
  project_note), while `gate_installed` tallies a missing note per modeled
  artifact at warn. So every freshly emitted projection with frontmatter
  reports `install.gate-installed` until install runs — on a built-in
  skill (delete SKILL.md, emit, check) and on a custom spec kind alike.
  The banner move left the note behind. Fix: emit places the note the way
  it places the banner — emit owns the projection's bytes (invariant 7:
  never part-authored, part-emitted), so a placement inside a projection
  is emit's; install keeps converging wording on projections it did not
  just write. Subtraction candidate ride-along: once emit places both,
  install's note placement over emit-owned projections is a duplicate
  surface to retire, not keep.

- observed at bf4b4168 (cascade-integrations audit of eight open entries;
  spot-verified by the session) — corrections before build picks them.
  (a) LAYOUT-LOUD-READ-SWALLOWED-REGION contradicts 0048: it asks an
  *advisory* finding; 0048 Rejected says a read that dropped authored
  structure is an error, and its Consequences name the layout seam's
  refusal. Recut: a new `LayoutError` variant in `layout.rs`, error, firing
  when a field or collection region binds a heading with children while a
  later heading-bound region stays unbound, naming heading, consuming
  region, starved region; `per` 0048 + invariant 6; drop `compose.rs`
  ("paralleling derive_layout_rows" — that lives in drift.rs) and
  `builtin_lock.rs` (derived, built-ins only; a layout kind is custom);
  `tests/layout_kind.rs` `intent_layout` is the fixture, no H1 case
  exists. (b) LAYOUT-DUPLICATE-PROSE-REGION-REFUSAL: cite 0048, not 0019
  (which says nothing about prose position); range over
  `overlaid_builtin_kinds` as `governs_collision` does — a relocated
  built-in can carry a layout; the vacuous-clause checks it names live in
  engine.rs/roster.rs, admissibility.rs stays the home. (c)
  LAYOUT-COLLECTION-MEMBER-OWN-SPAN-LEAF: "the composed half already names
  it `prose`" is false — composed leaves are author-named
  (`NestedMemberRow.leaves` keyed by field name); the SDK reserves no leaf
  name (`FRAMEWORK_KEYS` `prose` is member-level). A reserved own-span
  leaf name is a Decision, John's — fold into
  `(layout-title-heading-admission)` or open `(layout-own-span-leaf)`;
  hazards: `node.body` includes sub-heading text, so the own span cuts at
  the first child heading; a sub-heading slugged to the reserved name
  refuses. (d) REFUSAL-MESSAGE-NAMES-THE-EDIT: the read.rs item targets
  narration that does not exist (`narrate_kind` prints guidance/cite
  only); under (a) it collapses to "the new LayoutError's message meets
  the bar"; the admissibility half stands (`clause_collision` names only
  the label). (e) EXPLAIN-KIND-ADOPTER-ENTRY-POINT: the unchecked `kind:`
  path is read.rs ~221, not the bare-name fallthrough; the layout/embedded
  narration needs `narrate_kind` widened to declarations.kinds (regions,
  templates) — `Contract` carries neither; define "composed kind" or drop
  the term. (f) EXPLAIN-KIND-QUALIFIED-ADDRESS-FORM conflicts with 0049
  Rejected if it adds `<kind>:<name>` as a fifth qualifier beside the
  four: the grammar replaces the namespace; an undeclared kind in the
  form refuses (match `graph::target_identity`), a bare cross-kind
  collision refuses as ambiguous rather than narrating every kind
  (`why_impl` ~508); there is no shared address resolver to reuse
  (`address.rs` is FieldPath; kind:name parsed ad hoc at graph.rs ~1338,
  ~1596, drift.rs ~889) — "one grammar" means one tokenizer, and the
  two explain entries should merge as "explain accepts the 0049 grammar"
  or ENTRY-POINT is blockedBy this one. (g)
  BUILTIN-KIND-RELOCATION-EDGE-FIELDS: `compose.rs` "extend
  overlay_builtin_kind to carry edgeFields" is unnecessary — edges never
  live on a kind row; the gate reads assembly `edge` rows by `from` kind
  and resolves over built-in features already; once the SDK emits the
  row the engine gates it. Reword to "verify no engine change; Rust
  regression". `graph.route` is not assertable from an SDK test: split
  into an SDK payload assertion plus a Rust check test. `relocate(base,
  delta)` belongs in `kind.ts` beside `kind()`. (h)
  KINDS-IN-PLAY-NAME-COLLISION-REFUSAL: the engine already refuses a
  diverging same-name row (`kind.admissibility`, tests/coverage.rs
  ~281); the SDK's first-wins pre-empts it, so the fix is a refusal in
  `admit` on facts-object mismatch only — no engine logic in TS; test in
  refusals/emit tests via `compileDeclarations`; the blockedBy coupling
  must be stated: the refusal can recognize the relocation form only if
  (g) defines a marker on extended facts (e.g. `facts.relocates`), and
  landing (h) first breaks the only working (g) workaround (spreading
  `rule.facts`, order-dependent).

- observed at bf4b4168 (cascade-integrations, last two audits; verified by
  the session) — INSTALL-HOOK-PLACEMENT-FALSE-APPLIED needs four
  amendments. (1) Trigger mis-scoped: not "a hook kind claims the
  placement's `hooks.<Event>`". Any registration member whose kind targets
  `.claude/settings.json` (hook on any event, installed-plugin,
  known-marketplace) makes the file a represented manifest, and
  `emit_manifest` regenerates it whole, dropping every placed group
  regardless of event; an authored hook with byte-identical fields
  survives, which is why this repo's `.temper/hooks.ts` hand-mirrors the
  three commands. Predicate: "settings.json is a represented manifest in
  this lock" → Unchanged when the desired group is present after emit,
  else the new conflict outcome. (2) "naming the SDK constant to author
  instead" cannot be met: no SDK export of the three commands exists
  (`TAP_COMMAND` in declarations.ts is the tap only); they are Rust-side
  (`install.rs` SESSION_START_COMMAND, GUARD_COMMAND,
  POST_TOOL_USE_COMMAND). The outcome names the hook kind and the command
  string; exporting them from the SDK is the consolidation half the
  original note already named — file it as its own entry or fold in.
  (3) Missing file: `gate_installed` runs the same `evaluate_placements`
  dry-run and folds anything not Unchanged into `install.gate-installed`;
  unless the new outcome is recognized there, check flags the hook
  forever. Add to files[]. (4) Tests: nothing drives `install::run` with
  a hook member; patterns at tests/install.rs ~581 and ~825; ~1276 is
  guard-only. `ApplyOutcome::Conflicted` means baseline drift, so a new
  variant is warranted; 0021 ("no self-healing placement") supports
  report-over-replace. KIND-REGISTRATION-EMPTY-EMBEDDED-TYPE is safe to
  build as written; add tests/registration_locus.rs as the retained
  runtime backstop. HOOK-COLLECTION-ADDRESS-DUPLICATE-REFUSAL's park
  reason cites e4ff7bc6, unresolvable at HEAD (a gate-reverted worktree
  commit); cite 080311b9 if the reason should dereference.
