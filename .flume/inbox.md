<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- Built-in contract reconciliation against the 2026-07-15 Claude Code docs —
  drift register with per-item detail and source URLs in
  `docs/market-formats.md`, "Claude Code deep audit" section. Load-bearing
  items: commands merged into skills (`command` kind wants a legacy-posture
  note + cite refresh); skill frontmatter grew (new fields incl. `when_to_use`,
  `paths`, `context: fork`, component-scoped `hooks` — review `forbiddenKeys`
  and coverage against them; `paths` is a hard registration gate conditioning
  the other channels, verified empirically 2.1.210 — a composed channel the
  flat registration list can't express today, see the digest); `DOCUMENTED_HOOK_EVENTS` re-verify vs the
  current ~30-event set; rules `paths` + recursive discovery now first-class
  documented (cite refresh); agent `tools`-resolution failure now loud
  (v2.1.208+, candidate clause). Caveat carried in the digest: hooks/settings
  extracts were summarizer-mediated — any encoded `cite` re-fetches the raw
  page first, per the external-facts bar. observed at e8edffa
- `Requirement.kind` (`sdk/src/contract.ts:177-183`) is typed
  `KindDefinition<never>`, so a requirement cannot be keyed to any kind whose
  field type carries required members — `KindDefinition<Skill>`,
  `KindDefinition<Hook>` fail to assign; only all-optional-field kinds (rule,
  memory) work. The repo's own harness hit this: `.temper/harness.ts`'s
  `friction-capture-procedure` requirement documents dropping its `kind:` as
  a workaround. A requirement needs only the kind's identity for coverage
  resolution, never its field type; the collection child-kind slot already
  models this as `string | KindDefinition<any>` (`sdk/src/kind.ts:315`).
  Demand is live (human-ruled 07-15): the base-harness third cut prescribes
  skill/hook-keyed requirements. observed at 3540ebb

- Ruled, needs its docs line (John, 07-10, in session): a host that
  interleaves prose with typed members **is a layout source document** —
  temper builds no composed text-with-members posture, and programmatic
  structure-splicing (fabricated `Text` templates, self-rendered fences
  spliced into `text()` bodies — the centercode testbed's own hack, broken
  by two consecutive SDK shape changes) is territory temper won't go. The
  composed side stays what it is: `blocks()` for all-members bodies,
  `text()`/`file()` for prose. Remainder to route: the consumer guidance
  nowhere written — "if your host mixes prose and members, declare a
  layout and author the document" belongs in the docs' custom-kind path.
  (The note's other half — `render()` output fence-wrapped — resolved
  upstream at f2d73da, EMBED-RENDER-FENCE-FREE.) Observed at 8c00159,
  re-verified at c2f8a2c; trimmed at 0aa9e62.

- A 0019-loud instance the shipped refusals don't cover yet:
  UNTEMPLATED-NESTED-MEMBER-LOUD (4752b06) rejects the blocks-side orphan
  (a declared value no host templates), but a `member.<kind> <key>` fence
  sitting in a `text()`/`file()` body — the entire input class the
  pre-0018 fold used to read — is dead text with no finding on any path.
  Live repro: the centercode harness was green pre-0018 with ~57 embedded
  members resolving at leaf grain; it re-emits and re-checks green today
  with that whole layer silently gone (`explain`: "carries no nested
  member"; coverage: "21 documents carry no nested members"). With
  0019-content ruled, a `member.` fence is meaningful nowhere except as
  `blocks()` default rendering (layouts reject per-entity fences by
  decision), so one appearing in a text body and naming a kind the assembly
  declares is near-certainly an author error. Cheap refusal: fence info
  string parses as `member.` + a declared kind, no `nested_member` row
  matches the host → finding ("a member-shaped fence no declaration
  backs — dead text or a missing declaration"). Observed at 8c00159.

- Layout probe results (centercode testbed, migrated 07-10 off the fence
  hack onto clean prose per John's ruling; a relocation-shaped `rule` kind
  row was then declared with `content: [prose, collection(directive)]`):
  (1) **Good news, a prediction refuted**: emit honors a relocated
  built-in's declared content — `temper::layout::unadmitted` refused loud
  and located ("cls.md: heading `CLS (Shared Libraries)` fits no declared
  layout region") before writing a byte. The 0019-loud posture works
  end-to-end on this path. (2) **Untested divergence to close**: the gate
  path resolves a built-in kind via `overlay_builtin_kind`
  (`src/main.rs:896-919` at c2f8a2c), which lifts exactly `governs` +
  `templates` —
  not `content` — so a document that *fits* its layout would emit rows
  fine while `check`'s `layout_unit` reconstruction may silently fall back
  to the plain frontmatter read for built-in kinds. Same gap class T18 was
  (templates overlay), one fact later; unverified past emit since no
  testbed document fits yet. (3) **The consumer design question**: a
  layout binds the whole kind — every member's document must fit it — but
  a real rule corpus is heterogeneous: directive-carrying documents
  (cls.md, protocol.md) sit beside one-sentence pointer rules
  (omegaone.md, web-ui.md) that have no collection heading at all. 0019's
  own answer is "two kinds" — but both must govern `.claude/rules/*.md`,
  and what happens when two kinds share a governs glob is unspecified
  (collision? first-match? partition by fit?). That's the piece a consumer
  hits immediately on adopting layout for any built-in kind. Observed at
  8c00159.

- Corpus hygiene, small: two decision records wear the number 0019 —
  `0019-loud-or-nothing.md` (07-09, "sixth invariant") and
  `0019-content-is-a-declared-kind-fact.md` (07-09, "invariant 7", cited
  as "0019" by 0020's context). Decisions are addressed by number (status
  lines say "superseded-by 0018"), so a shared number is a live addressing
  ambiguity the first supersession will trip over. One of them renumbers
  to the next free slot — mechanical, but it touches whichever
  cross-references exist, so it's a plan-routed entry, not a hand-fix.
  Observed at 8c00159, still live at c2f8a2c; 0021 has since taken the
  next number, boxing the pair in.

- **Field incident, high-severity class**: the first emit under a
  post-e7b859a engine against a lock written by a pre-e7b859a engine
  **mass-reaps every live projection, silently green**. Live repro
  (centercode testbed, 07-15): lock rows spelled `./CLAUDE.md`-style by
  the c2f8a2c-era engine; one `emit` under 0aa9e62 reported
  `0 emitted, 21 unchanged, 21 reaped, 0 orphan-drift`, exit 0 — and
  deleted all 21 projections (CLAUDE.md, every rule, every SKILL.md)
  while rewriting the lock with unprefixed rows claiming them live and
  unchanged. Mechanism: e7b859a normalizes the workspace path before
  deriving `harness_root`, so the fingerprint pass keys files at the new
  spelling ("unchanged", fresh rows) while the orphan sweep joins the
  *old* rows' raw strings against the new owned-paths set, finds none,
  and deletes their files — the same files. The fix stops future spelling
  forks but never migrates a pre-fix lock, so every existing consumer
  workspace hits this exactly once, on upgrade. Damage is transient
  (sources regenerate; the very next emit re-emits all 21) but the
  window is real — a Claude Code session launched between the two emits
  finds no CLAUDE.md and no rules — and the run *lied* ("unchanged",
  exit 0) while deleting the harness. Two proposals, severable: (a) the
  migration — normalize path spellings when *reading* lock rows too, so
  a pre-fix lock reconciles instead of orphaning (the join key, not just
  the derivation, is what needed normalizing); (b) the 0019-loud guard —
  a reap sweep about to delete **every row the lock carries** while
  emitting nothing is near-certainly a bug or a spelling fork, and is a
  refusal, not a green exit line. Observed at 0aa9e62.

- **The gate's joins key on bare member name, not (kind, name)** — a
  cross-kind name collision produces *wrong findings*, not silence. Live
  repro (centercode testbed, 07-15, custom pack kinds): declaring a
  `dev-pack` member named `csharp` beside the existing `rule` named
  `csharp` made check attribute each member's `satisfies` claim and
  mention edges to the wrong same-named member — false `requirement.kind`
  refusals in both directions ("`area-directives` … satisfier `csharp` is
  kind `dev-pack`" and the mirror), false zero-degree findings on the
  colliding members, and collateral damage on an uninvolved clause (the
  `deploy` command's outgoing-degree broke against a `deploy` pack), exit
  1 on a harness emit had just accepted. The SDK side is already
  kind-qualified (mention addresses are `kind:name`; emit resolved
  `dev-pack:csharp` correctly), so the fix is gate-side: key the
  satisfies/graph joins by (kind, name) — the rows carry both — or refuse
  colliding names loud at emit. Renaming the four colliding members
  cleared every finding (clean bisect). Worse than a 0019-loud gap: this
  path *lies specifically*. Observed at 0aa9e62.

- Pack-kind field trial (centercode, 07-15) — the pass's convention layer
  brought under the gate, and it works: three frontmatterless custom
  kinds (`dev-pack`/`operational-pack`/`product-pack`, slice as a kind
  fact, roots nested inside the standards skill's directory), ten members
  with `file()` prose adopted byte-faithful from the pre-existing on-disk
  packs, every pointer site lifted from inline-code text to a
  `packMention` with identical display bytes (first emit: 21 unchanged),
  and the harness's own "duality invariant" now machine-checked — every
  pack reached by ≥1 pointer (per-slice `degree incoming ≥ 1` advisory
  clauses), every pointer resolves (emit's dangling-mention refusal).
  `registration: []` (a kind reachable only via edges) was accepted.
  Findings for the model: (a) a single kind can't govern a
  directory-sliced corpus — `member_projection_path` substitutes the
  glob's first `*` only, so glob `*/*.md` derives a broken path;
  kind-per-directory is the workaround and it proliferates kinds — a
  consumer wants either nested-glob path derivation or a declared path
  template; (b) frontmatterless projections carry no managed-by note
  (`install`'s note is frontmatter-borne), so a hand-edit to a pack is
  hash-caught as drift but the file itself never warns — the banner wants
  a frontmatterless form (HTML comment) or the gap documented. Observed
  at 0aa9e62.
