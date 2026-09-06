<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


- observed at 6a9a25ec (0.0.16 dogfood, this repo) — `temper install` reports
  `applied  post-tool-use hook  ./.claude/settings.json` on every run and
  the file never changes: `install::run` splices the group, then its own
  `drift::emit_program` (install.rs ~493/518) re-projects `hooks.<Event>`
  from the authored `hook` members and drops it, so the placement is undone
  inside the same run and `check` keeps reporting `install.gate-installed`
  (post-tool-use hook missing). Any harness that authors hook members hits
  this for every synthesized placement — the guard and session-start hooks
  only converge here because `.temper/hooks.ts` mirrors their commands by
  hand (the documented dogfood pattern; now extended to the PostToolUse
  Bash hook). Defect: the "applied" line is a claim install does not hold —
  a silent wrong answer. Remedy candidates: (a) install detects that a hook
  kind claims the `hooks.<Event>` address and reports
  `conflicted: emit-owned — author it as a hook member` naming the SDK
  constant; (b) the SDK exports the three synthesized hook commands so a
  consumer authors them without a hand mirror (removes the byte-identical
  comment in hooks.ts). (a) is the fix; (b) is the consolidation.

- observed at ad10e8cb (cascade adopter probe, 0.0.17; reader reproduced
  locally via `Layout::read`) — the layout reader binds regions to the
  document's *shallowest* heading level (`extract::body_heading_tree`), so a
  document with one leading `# Title` and `##` sections binds the title to the
  first heading-bound region and every later region reads empty: with the
  `prose, field(intent), collection(invariant)` layout, `intent` receives the
  whole document under the title and the collection yields zero members — no
  `nested_member` rows, no addresses, every each-grain clause over the
  embedded kind vacuous, `required("intent")` satisfied by the swallowed
  span. Nothing is reported: a region "reads empty, not loud" and the title
  *was* admitted. Every finding the probe filed as "layout members get
  nothing" reduces to this. Defect against invariant 6. Remedies: (a) refuse
  loud when a field region binds a heading that carries child headings while
  a later heading-bound region stays unbound — the author learns the title
  is being read as a slot; (b) the design fork: does a layout admit a
  document title (a lone leading H1 whose own span is preamble)? 0019's three
  primitives name none, yet the H1-title-over-H2-sections shape is
  markdown's universal one — this repo's own `specs/intent.md` has it. (a)
  is the fix this tick; (b) is an open question, not the fix's to settle.
  (GH #45; #43 and #44 were confounded by the same title.)

- observed at ad10e8cb (same probe) — a layout collection member with no
  explicit `key` that is retitled silently becomes a new member: emit writes
  the new slug, the old row vanishes, and nothing names the change until an
  edge to the old key fails `graph.route`. 0019's consequences promise
  "fingerprints report a rename as a move, loudly"; `representation.md`
  carries only "an explicit key survives retitling"; the engine has no move
  detection over `nested_member` rows. Corpus/code collision to settle in
  the model body: either a same-host, same-kind, same-leaves row whose key
  changed is reported as a move at emit (advisory), or the model states that
  identity without a key is the heading and a retitle is a delete + create.
  Session recommends the first — the row already carries the leaves to
  match on. The composed half has the same silence: renaming an embedded
  value's key in the program emits nothing that names the change (probe F9),
  so whatever the model rules for a layout retitle should bind a composed
  rename identically — one identity story for nested rows, not two.
  (GH #43 retitle half, #51 rename half.)

- observed at ad10e8cb (same probe) — `explain kind:<name>` for a name no
  kind declares prints the same "No authoring guidance is declared for
  `<name>`" narration as a declared kind without guidance: the `kind:`
  qualifier resolves to `Species::Kind` without an existence check
  (`read.rs` `resolve`), and `narrate_kind` treats an absent contract as
  absent guidance. `docs/cli.md`'s "a qualified name is always accepted
  directly" means never re-checked for ambiguity, not never checked for
  existence. Fix: a qualified kind name absent from `contracts` is
  `NotFound`, same as the bare form. (GH #47)

- observed at ad10e8cb (same probe) — `explain` does not accept the engine's
  own member address form. `<kind>:<name>` is what `nested_member.host`
  rows, `graph.route` findings, and mention targets spell, but `resolve`
  knows only `member:`/`requirement:`/`kind:`/`address:` qualifiers, so
  `explain memory:CLAUDE` falls to the bare-name path and reads `NotFound`
  while `explain CLAUDE` resolves. Fix: `resolve` accepts
  `<declared-kind>:<name>` as a kind-scoped member — which also
  disambiguates same-named members across kinds, a case the bare-name
  lookup cannot express today. (GH #49)

- observed at ad10e8cb (same probe) — two `section_contains` clauses on one
  kind with the same heading prefix and different markers reduce to one
  label (`<kind>.section_contains.<heading>`; `contract.rs` folds the
  heading in as the field segment and drops the marker), so the lock is
  malformed at admissibility and the author cannot express "every invariant
  carries a Test" and "... a Standard" together. `clause()` has no label
  override, and `pipeline.md` is right that it should not (the label is
  compiled, never authored). Fix: fold the marker into the label's field
  segment — deterministic, human-legible, spellable in a dial entry — and
  audit the other predicates whose label omits a distinguishing argument
  (`must_define` keys on its marker already; `require_sections` has none).
  (GH #48)

- observed at ad10e8cb (cascade probe F9, composed spec kind) — edge
  resolution to a nested member is host-blind. At check an embedded member's
  id is its bare `key` (`compose::embedded_member_features`), `by_kind`
  pools every host's rows of one kind, `graph::resolves` matches by id, and
  `gate::embedded_hosts_by_source` keys `(kind, key)` with last-write-wins —
  so two specs each carrying an invariant keyed `x` resolve an edge to
  *one* of them silently, and renaming one host's key to a key another host
  holds silently re-points every edge. No admissibility rule covers a
  duplicate `(kind, key)` across hosts, though `pipeline.md` calls two rows
  wearing one identity a malformed lock and `satisfies_label_admissibility`
  already refuses the same ambiguity for bare fill labels. The leaf address
  grammar (`<member>/<kind>/<key>/…`) is host-scoped; the edge identity is
  not — two grains of one noun. Fix: refuse a duplicate `(kind, key)` over
  `nested_member` rows at admissibility, naming both hosts. The adopter's
  mitigation (`uniqueName` at required severity) is a clause standing in
  for a precondition of resolving at all — the boundary `contract.md`
  draws puts this in well-formedness, not in a clause the author must
  remember to write. Host-qualified edge targets are the alternative; the
  session recommends the refusal, since a corpus-unique key is what the
  leaf address already assumes. (GH #51)

- observed at ad10e8cb (cascade probe F9) — a render hook cannot cite a
  nested member. `emit.ts` `edgeTargetFacts` resolves an edge leaf against
  `memberTable(harness)`, which indexes top-level composed members only, so
  an embedded value's edge field naming another host's embedded value
  throws "resolves to no composed member" while the same address resolves
  and gates at check. `representation.md`'s target-fact set (name, address,
  kind, projection path relative to the host's) is derivable for a nested
  target — its projection is its host's — so the bound is the table, not
  the model. Fix: the SDK's member table indexes embedded values under
  their `kind:key` address alongside top-level members (the same closed
  identity the admissibility fix above makes unique), with `path` taken
  from the owning host's projection. Until then the plain edge field
  gates without rendering, which is a working degradation, not a silent one.
  (GH #50)

- observed at ad10e8cb (cascade F10 pre-run) — a verbatim `prose` layout
  region that is not the first one is a silent no-op: `Layout::read` lands
  the document preamble in the first verbatim prose region and every later
  one reads empty, and no prose region ever consumes a heading, so a layout
  declared `…, collection(invariant), prose` accepts fine and the trailing
  region can never carry a byte. The adopter read it as "text between the
  values" and lost a heading to `unadmitted` instead. Two remedies: (a)
  admissibility refuses a second verbatim prose region (same class as a
  vacuous clause — a region that can never fill); (b) the model states
  plainly that a layout's prose is the preamble or an import, never an
  interstitial — which is what 0019's "prose (verbatim)" already means and
  the SDK's `LayoutRegion` doc does not say. (a) is the fix; (b) rides it.

- observed at ad10e8cb (cascade, GH #46) — the SDK's kind type accepts a
  non-empty `registration` on an embedded locus while the engine refuses it
  at `kind.registration-locus`. The engine is right (`representation.md`,
  embedded locus registers nothing); the gap is that `tsc` lets it through
  to a run-time refusal when the SDK's positioning is that the type is the
  first gate. Fix: the locus discriminant narrows `registration` to the
  empty tuple for `embedded` and `nested-file`.

- observed at ad10e8cb (John, 09-05: "effective use" — the redirect this
  round sets) — every adopter failure above shares one shape: temper's model
  was right and lived in `specs/` or a source comment, while the adopter
  stood at the CLI with no path from what they saw to what to edit. Intent
  invariant 8 (guidance delivered at the point of failure) binds the kind's
  members today and not the *program that declares kinds, layouts, and
  contracts* — the surface an adopter authors first. The bar: everything
  needed to recover is in the finding text or `explain` output, never in
  `specs/`. Four entries, in this order, each its own commit: (1) **loud
  reads** — a declared layout region that reads empty is a shipped clause in
  the default contract entering advisory (invariant 5), its finding naming
  the heading that consumed the slot it expected; the reader never says
  nothing (rides the existing algebra — 0019 consequences, "coverage clauses
  ride degree over selections"; no collision with "a region states what may
  appear, never what must", since the floor is a dialable clause). (2)
  **types that refuse** — the SDK narrows `KindFacts` to what admissibility
  admits: `registration` empty under `embedded`/`nested-file`, at most one
  verbatim `prose` region, one heading-bound region per document heading is
  the reader's to say but the type can forbid the shapes that never fill.
  (3) **refusals in the author's vocabulary** — an audit of every
  admissibility and reader message: each names the edit (which two clauses
  collided and the argument that tells them apart; which region bound which
  heading), never the lock state alone. (4) **`explain kind:<x>` as the
  adopter's entry point** — for a layout kind it prints the regions in order
  and a skeleton document that fits; for a composed host it prints the
  admitted embedded kinds and their leaves; for an undeclared kind it says
  so (GH #47). A one-page adopter doc (two halves, when each applies, the
  address grammar) is fifth and is human-authored, not an entry.

- observed at ad10e8cb (cascade F10, GH #43/#44 re-diagnosed) — a
  committed layout document that discovery finds but the program never
  declares yields no declaration rows anywhere, silently. Rows for a layout
  source come from exactly two paths: `emit` over `payload.members` (the
  program's declared members — `drift.rs` derive_layout_rows) and `check`'s
  read-time derivation for **local**-locus kinds only (`compose.rs`
  local_document_rows). A discovered committed member of a layout kind is
  read at check for its field slots (compose.rs layout_unit) and for nothing
  else: coverage prints `invariant (0)`, `explain` says "Nested members:
  none", every leaf address fails, and no finding names the cause. The
  split is spec-stated (pipeline.md: the gate reads declarations from the
  lock family and nowhere else; local locus is the 0032/0034 exception), so
  the fix is not read-time derivation for committed sources — it is the
  finding. Remedy: `check` reports a discovered member of a layout kind
  with no provenance row in the lock as a finding (advisory, the
  `config.stale` posture) naming the edit: declare it in the program
  (`spec({ name })`) and re-emit, or declare the kind local. Two
  refinements ride it: (1) regions bind headings by *position*, so a
  missing section shifts every later binding one heading — the loud-reads
  clause should name the heading each region bound, not only the empty
  ones; (2) the preamble lands in the first verbatim prose region in
  *declaration* order, wherever it sits, and prose spans reach neither the
  lock nor `explain`, so a trailing prose region silently receives the
  document's opening paragraph.

- observed at ad10e8cb (cascade spec use-case, 09-05) — the one contract
  the adopter wants that the vocabulary cannot spell: "every rule body
  carries ≥1 directive", a per-host floor over the embedded values a
  member's body composes. Selections are by kind (an embedded kind's
  corpus-wide population), by opt-in, and by incidence (`contract.md`,
  "selection"); containment — host member → its embedded values — is not
  an edge in the relation graph (`graph.rs` knows hosts only through
  `embedded_hosts` for scope judgments), so `count` over `directive` is
  corpus-wide and `degree` sees no containment arc. 0004 makes nested
  members members and says the contract layer ranges over them "exactly
  as over top-level ones"; the host's containment of its values is a
  relationship the program declares (`blocks()`), so it fits "declared,
  never mined". Session recommends: containment joins the resolved edge
  set as a derived incidence family (one field per admitted kind, e.g.
  `contains:directive`), so the existing `degree` algebra spells the floor
  with no new predicate — "each rule: outgoing ≥ 1 on contains:directive".
  A requirement-attached clause then spells "the satisfier of X holds only
  directives" with the same family and a `kind`-narrowing each-grain
  clause; no admit-by-requirement mechanism is needed (admission is a
  type statement over a host kind; this is a contract over a selection —
  keep the layers apart). A deliberate language change per `contract.md`
  ("adding one is a deliberate language change"): a Decision, then the
  entry.

- observed at ad10e8cb (cascade, idea) — a kind-declared **citable
  handle**: the engine-derived target facts a format may select are the
  closed set name, address, kind, projection path
  (`representation.md`, kind), so a reference posture's `render` branches
  on `target.kind` to choose a spelling. A `handle` declared on the target
  kind ("how a reference to a member of this kind reads") is declared on
  the kind, never authored at the instance, never fabricated — compatible
  with the closed-set rule as a fifth engine-derived fact read off the
  target kind's declaration. Lower priority than the containment family
  above; not a defect.

- observed at ad10e8cb (cascade F11, GH #43 leaves gap) — a layout
  collection member's own span is dropped. `layout.rs`
  `read_collection_member` takes leaves from the member's immediate
  sub-headings only (H4 under an H3 member), keyed by slug; the paragraph
  under the member heading itself lands nowhere — not a leaf, not the lock,
  not `explain`. A spec whose invariants are `### Title` + a paragraph +
  `**Test.** …` materializes rows with `leaves = {}`, so leaf predicates,
  leaf addresses, and leaf `explain` cannot reach layout content at all
  while the composed twin carries full leaves. Nothing tells the author
  that sub-headings are the leaf syntax. Fix: the member's own span lands
  as a `prose` leaf (the name the composed half's embedded values already
  use for the authored body), sub-heading spans stay keyed by slug beside
  it, and `explain kind:<layout-kind>`'s skeleton (redirect entry 4) shows
  the H3/H4 shape.
