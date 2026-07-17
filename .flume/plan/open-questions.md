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

- **`kinds/` + `packages/` curated trees — RETIRED.** The engine retirement
  drained and the physical trees were deleted (`chore(harness)` 68f187d).
  **One debt survives**, accepted: `tests/session_start.rs:122/141` still
  writes `+++`-format `.temper/kinds/spec/KIND.md` +
  `.temper/packages/spec/PACKAGE.md` fixtures. **Reclassified 07-16 — this
  record had it wrong, and the misfiling is why it never discharged.** It was
  filed as narration staleness riding a reconcile; it is not. Read on disk at
  8913b59: the fixtures sit inside
  `stray_custom_kind_shaped_fixtures_never_disturb_a_clean_session_start`
  (113), whose *subject* is that files in the retired format are inert — the
  vocabulary is the assertion, not a comment beside it. So no hygiene pass can
  "reconcile" it: the live question is whether temper still wants a test
  pinning retired-format inertness at all, and that is a value call
  (subtraction before addition, CLAUDE.md) no build tick may invent. **The
  reclassification is now proven, not predicted:** CHECK-RUNNER-REMAINDER
  shipped (a9a21a9), edited this very file at 49, and left the fixtures
  standing — the third entry to open it and correctly not touch it (after
  664a522 and CHECK-ARG-HALF-GATE 4256274). CHECK-ANNOUNCES (dab85aa) is now
  the **fourth**, touching only lines 17 and 394 to thread `Announcement`
  through `session_start` — the fixtures untouched, and unshifted with them.
  The numbers above are re-read on disk at b85df4a (fn 113, cites 122/141),
  not carried forward. Not a rider awaiting a carrier; a
  question awaiting a human.
  **The `sdk/src/builtins.ts` half is discharged.** SKILL-NESTED-REFERENCE-DOCS
  (a7a8cc1) carried it named and cut both doc-comment cites to the deleted
  `packages/{rule,memory}.anthropic/PACKAGE.md` files; `rg` over the file finds
  neither. Nine entries had opened builtins.ts and left them — the same lesson
  the record below spent two entries learning, proven a third time: the rider
  discharges when an entry names it, and not when a file is merely opened.

- **One stale cite, ride-only, never an entry.**
  Comment and citation staleness never files a standalone entry; it rides
  whichever entry next opens the file — and the rule has now paid out four
  times running, which is why this record keeps shrinking rather than growing.
  The
  `sdk/src/prose.ts` half retired at 9409a6c (SDK-BLOCKS-FILE-REFUSAL,
  42a2dd1, cut all ten pre-recut narration lines). The `Cargo.toml` half
  **retires here, discharged on disk at 385c429**:
  FIELD-ADDRESSING-RFC-9535-SUBSET (aaf70f1) opened the file to add
  `serde_json_path`, carried both riders it was given, and landed them — `regex`
  now reads "the charset mechanics behind `allowed_chars` — hidden, never an
  author-facing `pattern` clause" (36-38), and the schemars/ts-rs note cites
  `src/schema.rs` with ts-rs holding its live `sdk/src/generated/` seam role
  (48-55). Sweeping the class across `src/` + `sdk/src/` at 0c3cbcb still
  leaves exactly one orphan line:
  - `src/roster.rs`:475 — **the last orphan of its class**: the doc comment on
    the `membership_roster` test helper cites `` `10-contracts.md` ``, a file
    0001 deleted. The sentence's claim is live — a `target` names a declared
    requirement — so the cite comes out, never gets re-pointed at a surviving
    file. A rider discharges when an entry NAMES it, never when a file is
    merely opened, which is why the carrier must carry it in its scope.
    **The carrier has now arrived**: VERIFIER-TYPED (unblocked this tick as
    EXTENT shipped 4bdb796) opens `src/roster.rs` for its clause-(b) verifier
    dispatch and NAMES this cite-cut in its roster.rs scope — so the rider now
    rides it and discharges when VERIFIER-TYPED ships, the ride-only rule's
    predicted resolution. Cite re-read on disk at HEAD, now 475 (drifted
    469→473→475 under CLOSED-KEYS-CLAUSE's and later edits).
  The rule's **fourth** payout — its first on a cite the sweep itself
  surfaced — **landed**: `src/main.rs`:1047 called the selection loop "The
  second and last dial site" while four `dial.apply` sites existed. It was
  handed to CHECK-ANNOUNCES in scope, and dab85aa carried it: the comment now
  reads "The last of the dial's four sites, and the only one over selections
  rather than contracts" (`src/main.rs`:1050-1051, read at b85df4a) — the
  loose literal count replaced by the one axis that makes the site last. Named
  in scope, so it discharged; it never became an entry. That is four payouts
  running, and the rule's condition has never once failed: **every** discharge
  came from an entry that NAMED the cite, and none from a file merely opened.
  The `sdk/src/builtins.ts` cite this record carried last tick **discharged
  exactly as the rule predicts**: 7fae62e falsified `marketplaceDefaultContract`'s
  header sentence, SHAPE-PREDICATE was given it in scope, and 0927979 landed it —
  the header at 960 now names its own widening rather than a sibling's retired
  hold, verified on disk at a2e48aa. It never became an entry.
  Fixture body text inside tests stays a separate class, excluded — it is
  `.to_string()` test data, not cites: `src/kind.rs`'s `15-kinds.md` strings,
  `src/read.rs`'s `20-surface` member ids, `tests/display_rule.rs`'s "law 5"
  and "law 7" rejected-entry bodies, and `src/extract.rs`'s two `"…law 5"`
  decision-fixture strings.

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
  stale-but-harmless (neither trigger mechanism reads the prose). Kept — a
  cosmetic residual, not the drift risk originally logged here.

- **`docs/` is candidate intent, not intent** — human territory,
  fence-excluded; plan never reads a horizon entry as intent.
