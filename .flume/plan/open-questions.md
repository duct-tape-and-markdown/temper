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

- `(cross-source-edge-correlation)` — OPEN, live driver (GH #59). No
  predicate relates a property of one edge's source to a property of
  another edge's source when both edges meet at one member. The shape is
  `writer --writes--> target <--presents-- presenter --gatedBy-->
  condition`, and the defect it would catch is a gated presenter whose
  target has an ungated writer. Today's vocabulary misses it by
  construction: `degree` counts edges at one member, `membership` tests one
  field against a fixed set, and `reached-from` (0.0.19) follows
  reachability, never a correlation between two sources. The ruling is a
  `contract.md` "clause" addition with no spec section behind it, so it is
  human-authored, not derivable. The objection it must answer: a general
  path predicate is a query language, and the kernel grows one only when a
  narrower shape cannot cover the demand — for example "every sibling
  source into X over field F carries the edge G that the source over field
  H carries", which stays decidable and bounded. **Stance, John 2026-09-29:** a general path predicate is rejected — it
  makes the contract language a query language. The narrow sibling form
  clears the evidence bar only when the driver authors it and has to
  delete it; the driver is a second corpus, a feature and never a founding
  assumption (`representation.md`, "Reach"). Stays open for that
  evidence. No dependents.

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
  item) stays advisory, unchanged. **Its scale, measured 2026-09-28 (this
  tick, both sides run on disk): 98 before `cc6eb974`, 103 after** — the
  record has carried no number until now, and the choice reads differently at
  103 than at the single digits a reader would assume. The growth is
  *structural*, not drift: narrowing an unearned `pub` (`engineering.md`, "An
  export earns its consumer") mints one warning per public doc that linked the
  name, so the visibility campaign and this asymmetry are coupled — every
  future narrow adds to the count, and `cc6eb974`'s own body mis-stated the
  baseline as seven. The gate is unaffected (advisory, `exit 0`) and the
  links stay navigable, which is the choice; what is new is that its cost
  grows monotonically with work the queue actively wants done. The condition
  for the next break: a reader who cannot find a real broken link among the
  advisory ones, or a decision to spell these links `crate::…`-qualified at
  the narrow rather than leave them. Prose staleness no linter can check — a
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
