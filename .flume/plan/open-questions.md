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

- `(multi-harness-projection)` — OPEN, strategic. One member projecting to N
  harnesses (`.claude/rules/` and `.cursor/rules/` from one document) —
  rulesync's portability as an architecture side effect (`specs/intent.md`,
  "Positioning"). The engine is corpus-generic (`specs/model/representation.md`,
  "Reach"), but the write face of foreign formats is open: per-harness
  capability mismatch, which harness is authoritative, whether a lossy
  projection is a verdict or an error. Inherits the AGENTS.md kind question
  (ruled 07-15: not a claude-code kind — Claude Code does not read
  AGENTS.md, docs retrieved 2026-07-15; its consumer is this fork's
  cross-tool story). Demand side is no longer zero (07-16 war game,
  simulated): 2/8 personas rate one-member→N projections an adoption-blocker
  and want a **counterpart-drift check** — a fourth open face beside the
  three above. Timing unchanged. No dependents.

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

- `(embedded-kind-guidance-channel)` — OPEN, no dependents. 0045 decoupled
  guidance from the clause onto any kind, no locus carve-out named in
  `contract.md`'s guidance-channel prose. The SDK re-coupled delivery to the
  locus-bearing kind-fact row (`sdk/src/declarations.ts` `kindFactRow`
  throws for `locus.kind === "embedded"` yet carries `guidance` on that same
  row), so an embedded kind's guidance never reaches the lock — `schema`
  skips embedded kinds, `explain` has no bare-kind form. Ruled surface-it
  (interactive, 2026-07-23, inbox #12): real gap, not an intended exclusion.
  Undecided: the lock-delivery *shape* for locus-less guidance — a guidance
  row keyed by kind name any kind carries, vs. riding the host's `templates`
  row embedded kinds already reach the lock through. Settle with a short
  `contract.md` note (the 0045 guidance-channel passage) stating delivery is
  decoupled from locus as it is from clause — cf. #10's `pipeline.md`
  precedent, a spec clause landing before the entry routes build-ready.
  Filing the SDK+engine fix now would invent the row shape the note is
  meant to settle. *observed at v0.0.12 (bf4b5cd9); centercode pr-571 @
  efcc1fd175. Decision 0045.*

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

- **Stale cites: links are gated, prose rides.** A doc-comment cross-reference
  that drifts is temper's own no-drift thesis turned inward. Broken intra-doc
  links are now **gated**: crate-level `#![deny(rustdoc::broken_intra_doc_links)]`
  plus `cargo doc --no-deps --document-private-items` at afterMerge, so a rename
  or move that breaks a `[`link`]` fails the build — it never rides. Prose
  staleness no linter can check — a "sole consumer" claim, a line-number
  pointer, a stale invariant paragraph — **rides** the next entry that opens the
  file and discharges when that entry names it (never a standalone entry), and
  is tracked **nowhere**: the per-instance ledger was itself the per-tick context
  tax this rule exists to avoid. The 2026-07-23 sweep cleared the standing
  backlog (23 links, 13 prose cites) and set the gate; git history holds the rest.

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
