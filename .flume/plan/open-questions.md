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
  whichever entry next opens the file, and discharges only when that entry
  NAMES the cite — never when a file is merely opened. The rule's condition
  has never once failed across every payout git records.
  **Ten live orphans** (six re-verified at HEAD df57610, the seventh at
  8415088, the eighth at 721cab6, the ninth at 11ab0ab, the tenth new this
  tick, one — extract.rs's own, a different one — discharged at 2a6e488). `src/json_splice.rs`'s
  module header (surfaced 5af93d9, sweeping foundation) claims install.rs as
  "the sole consumer," but json_manifest.rs now also calls apply_edits/
  object_shape/insert_member/pretty_at — no pending entry currently opens
  json_splice.rs, so per the rule it rides the next one that does rather
  than filing here. `src/drift.rs`'s `RawLockRow` doc comment (now
  2074-2079, re-read at 126c264) still names `[read_prior_provenance]` in
  its intra-doc link list; DRIFT-EMIT-LOCK-PARSE-HOIST (112b188) renamed
  that fn to `read_prior_provenance_from_doc`. DRIFT-SOURCE-DEP-PARSE-HOIST
  (2df42a0), IMPORT-ROLLUP-WRITER-PLACEMENT (ab2e822),
  MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT (e97fc81), DRIFT-COLLECTION-ADDRESS-ENTRY-SHAPE-DEDUP
  (a96d0b0), DRIFT-CONFIG-STALE-FROM-DOC-DELEGATE (f0880ce),
  DRIFT-WRITE-ROLLUP-CUSTOM-PARAM-PRUNE (c52a1db), and
  WRITE-CREATING-PARENTS-CONSOLIDATE (700b588) have all since shipped
  touching drift.rs, none reaching the region — so it still rides
  whichever entry first does.
  `src/document.rs`'s
  `item_to_json` doc comment (surfaced the formats posture sweep) cites
  "the built-in adapters' `json_to_toml_value`" as the function's
  inverse-of; that function was cut in 664a522, before 6618b47 even wrote
  the citing sentence — stale from the moment it was authored. No pending
  entry currently opens document.rs, so it rides whichever one first does.
  `src/install.rs`'s orphaned `placement_lines` doc comment (now
  1685-1691, re-read at 126c264) is dead prose the extraction commit
  (8704036, PLACEMENT-MODULE-EXTRACTION) left behind: it moved
  `placement_lines`/`is_placement_comment` to `src/placement.rs` verbatim
  but not their preceding doc comment, which still sits glued — no blank
  line — directly above `render`'s own doc comment, reading as render's
  opening paragraph though it describes a function that no longer lives
  in this file. INSTALL-PROJECTION-MATCH-CONSOLIDATE,
  INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE, INSTALL-PLACEMENT-KIND-ENUM,
  BUNDLE-INSTALL-SESSION-START-SHAPE-CONSOLIDATE,
  INSTALL-ERROR-ZERO-CONSUMER-PRUNE (2c037ba), and
  GUARD-DECLARED-LOCUS-FILTER (a5e154f) — plus INSTALL-PACKAGE-JSON-ANCESTOR-SHORT-CIRCUIT
  (9bf9ebb, the ensure_package_json/spawn_npm_install split at lines
  7-13/443-452/1509-1546) and, new since the last read,
  WRITE-CREATING-PARENTS-CONSOLIDATE (700b588, write_scaffold_file's body
  at 1454) — have all shipped touching install.rs, none reaching this
  range; no entry currently open chains onto it, so it rides whichever
  next does.
  `src/json_manifest.rs`'s `Manifest::read` doc comment (now line 352,
  re-read at f88e96d) still names `[extract::manifest_members]`, though
  the function it points at moved into this same file when
  `manifest_members` was extracted from extract.rs to json_manifest.rs
  (404b73a, EXTRACT-FOUNDATION-BOUNDARY-RESTORE) — the module prefix is
  self-referential and wrong. JSON-MANIFEST-DISCOVERY-BOUNDARY-RESTORE
  (89cfc64), GATE-MANIFEST-SHARED-READ-HOIST (83fbdd5), and
  MANIFEST-CACHE-READ-COUNT-PIN (c27b411) have all since shipped
  touching json_manifest.rs, none reaching line 352 — the last edited
  only the `manifest_read_count` doc comment at 40-43; no entry
  currently open chains onto it, so it rides whichever next does.
  **A seventh, surfaced this tick's posture sweep of contract.rs.**
  `src/contract.rs`'s `SHAPE_PATTERNS` doc comment (884-885, re-read at
  8415088) claims compilation "cannot fail" is "covered by
  [`crate::contract::tests`]" — a module this file no longer carries (a
  `mod tests` block was present through e1de1f0 and is gone by HEAD; the
  three shape patterns' only exercise today is the standalone
  `tests/shape_predicate.rs`). No pending entry currently opens
  contract.rs for edit — SHAPE-LEADING-DOT-SLASH-UNTESTED, filed this
  tick, only touches tests/shape_predicate.rs — so it rides whichever one
  first does.
  **An eighth, surfaced the drift.rs posture sweep, re-read at 126c264.**
  `src/drift.rs`'s `source_dep_stale_from_doc` doc comment (now 2769-2788)
  is two glued doc blocks: the first paragraph and its `# Errors`
  (2769-2778) claims "Returns a [`DriftError`] if the lock cannot be
  read/parsed or a present row is malformed" — that is `source_dep_stale`'s
  (2821-2843) contract, the sibling that reads and parses the lock file
  itself; `source_dep_stale_from_doc` (2789-2819) takes an already-parsed
  `doc: &DocumentMut` and can only fail on a malformed row, correctly
  stated by the second, glued-on paragraph (2779-2788). `source_dep_stale`
  carries no doc comment of its own — the split evidently pasted the
  pre-split doc onto the new `_from_doc` function without trimming it to
  match, then never gave the surviving read+parse wrapper its own.
  DRIFT-COLLECTION-ADDRESS-ENTRY-SHAPE-DEDUP, DRIFT-CONFIG-STALE-FROM-DOC-DELEGATE,
  DRIFT-WRITE-ROLLUP-CUSTOM-PARAM-PRUNE, and WRITE-CREATING-PARENTS-CONSOLIDATE
  have all since shipped (a96d0b0, f0880ce, c52a1db, 700b588), each
  confirmed on disk touching 1140-1148, the `config_stale` body,
  `write_rollup`'s custom-param call site, and `write_rollup`'s own
  create-parent-dirs-then-write body respectively — none reaching
  2769-2843. No entry is currently open on drift.rs at all, so it rides
  whichever one next does.
  **A ninth, surfaced this tick's posture sweep of src/layout.rs's immediate
  import.** `src/extract.rs`'s `body_heading_tree` (538) and `body_preamble`
  (588) doc comments each justify their `pub(crate)` visibility as "so the
  [`crate::kind`] layout reader" stands on/places prose off this substrate —
  but the layout reader moved out of kind.rs into its own module at cfa545e
  (build: extract layout-document reader to new layout module); `rg` confirms
  both functions' only present caller is `src/layout.rs` (124, 143, 157), not
  kind.rs. No pending entry currently opens extract.rs, so per the ride-only
  rule it rides whichever one first does.
  **A tenth, surfaced this tick's posture sweep of src/roster.rs.**
  `tests/contract_template.rs`'s doc comment (255-258, authored 94ac5f1,
  2026-07-07) and `tests/read_verbs.rs`'s inline comment (215-216, authored
  7c66611, same day) each still cite `roster::check` — the function
  `src/roster.rs` carried under that name until 28ad61f (build: judge the
  set predicates over any declared selection, 2026-07-16) split it into
  [`selections`] (opt-in resolution) and [`admissibility`] (definition
  validation); neither symbol has existed in this module since.
  `tests/requirement_roster.rs`'s neighboring `roster::candidates(by_kind)`
  cite (877) names the private helper that still carries that exact name
  today, so it is not part of this orphan. No pending entry currently opens
  either citing file, so each rides whichever one first does.
  **An eleventh, surfaced this tick's post-ship reconciliation over
  a00e14a..HEAD.** `sdk/src/contract.ts`'s `telemetry` doc comment (338)
  reads "See `TELEMETRY_EVENT_HOOKS` in `declarations.ts` for the
  vocabulary" — but DECLARATIONS-TELEMETRY-HOOK-PROVIDER-FACE-MOVE
  (19258b7/a6db2b5), shipped inside this very window, relocated
  `TELEMETRY_EVENT_HOOKS` to `sdk/src/builtins.ts` (provider-face data
  belongs beside `hookDefaultContract`); the cite in contract.ts was not
  updated with it, so it now names the wrong module. No pending entry
  currently opens contract.ts (CONTRACT-FORMAT-PLACES-EDGES-ZERO-CONSUMER
  already shipped, a00e14a), so it rides whichever one first does.
  The prior orphan, `src/roster.rs`'s `membership_roster` doc comment citing
  the 0001-deleted `10-contracts.md`, discharged at 2fc2291 — VERIFIER-TYPED
  opened roster.rs for its verifier dispatch and cut the cite in scope,
  exactly the ride-only rule's predicted resolution. A second orphan,
  `src/extract.rs`'s orphaned `manifest_members` doc-comment fragment
  (954-964, the same 404b73a extraction that left the json_manifest.rs
  companion cite above), discharged at 2a6e488 — EXTRACT-BODY-HEADINGS-COLLECT-HEADS-DEDUP
  opened extract.rs for its `body_headings`/`collect_heads` consolidation
  and excised the dead prose in scope, the same predicted resolution a
  second time.
  Fixture body text inside tests stays a separate class, excluded — it is
  `.to_string()` test data, not cites: `src/kind.rs`'s `15-kinds.md` /
  `10-contracts.md` strings, `src/read.rs`'s `20-surface` member ids,
  `tests/section_contains.rs`'s `10-contracts` fixture, `tests/display_rule.rs`'s
  "law 5" and "law 7" rejected-entry bodies, and `src/extract.rs`'s two
  `"…law 5"` decision-fixture strings.

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
