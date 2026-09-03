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
  waiting-for-demand fork, the demand already shipped. No dependents.

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
