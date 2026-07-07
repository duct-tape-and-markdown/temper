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

- `(json-projection-format)` — OPEN. The JSON-manifest built-in kinds
  (settings, MCP, plugin/marketplace) need a generic JSON adapter (a peer to
  `src/frontmatter.rs`) reading nested-key fields into the generic extraction
  path; a kind's on-disk shape is its `format` (`specs/model/representation.md`,
  "kind"). The kernel now rules hooks/permissions/MCP servers are embedded
  members with kinds and default contracts (`specs/model/representation.md`,
  "Reach"), and `specs/builtins.md` names them forward work ("The named
  expansion") — so the adapter is unblocked engine work, but its shape and the
  `format`-fact spelling for a JSON kind are an open design fork needing John
  before it can be filed as a pending entry.

- `(agents-md-builtin-kind)` — OPEN (registered 2026-07-06). The engine's
  hand-written std-lib ships an `agents-md.memory` built-in kind (glob
  `**/AGENTS.md`), but the SDK module and the derived built-in lock export only
  the `CLAUDE.md` `memory` kind (`specs/builtins.md`, "The shipped kinds":
  memory is a `CLAUDE.md`-family file; `AGENTS.md` at root sources `CLAUDE.md` —
  `specs/distribution.md`, "The offering"). The spec-faithful default drops the
  engine's agents-md.memory to match the lock. Open: should temper ship an
  AGENTS.md built-in kind — and if so under a **distinct label** (never a
  provider-qualified `memory`; identity travels by import, not string —
  `specs/builtins.md`)? A feature addition, not a chain blocker.

- `(local-overrides)` — OPEN. The committed-plus-gitignored personal-override
  layer has no stated spelling in the assembly model (`specs/model/pipeline.md`,
  "The SDK" — the harness is one composed value). Candidates: a local harness
  module composed by convention, or an engine-side severity overlay. Blocks
  nothing until someone needs a personal override.

- `(eval-capability)` — OPEN, strategic, parked past launch. Harness evals: a
  requirement carries prose intent and a verifier edge
  (`specs/model/contract.md`, "requirement"), and the graph gives eval
  selection for free (impact → which evals re-run). If ever built: a verifier
  type and/or the behavioral remainder made concrete — probabilistic, NEVER a
  well-formedness check or the hard gate (`specs/intent.md`, invariant 2 / "The
  honest bound"). Do not let it near the launch wedge.

- `(multi-harness-projection)` — OPEN, strategic. One member projecting to N
  harnesses (`.claude/rules/` and `.cursor/rules/` from one document) —
  rulesync's portability as an architecture side effect (`specs/intent.md`,
  "Positioning"). The engine is corpus-generic (`specs/model/representation.md`,
  "Reach"), but the write face of foreign formats is open: per-harness
  capability mismatch, which harness is authoritative, whether a lossy
  projection is a verdict or an error. No dependents.

- `(json-write-fidelity)` — OPEN (field report 2026-07-07). install's hook
  merge into an existing `.claude/settings.json` is semantically correct but
  re-serializes the whole file (keys reordered, EOL churn). The round-trip
  discipline the corpus states for TOML/markdown (`toml_edit`, the
  format-preserving keystone) has no stated JSON equivalent
  (`specs/model/pipeline.md`, "Install" — the one settings write). Candidate
  ruling: key-order-preserving insertion. Peer to `(json-projection-format)` —
  that fork models JSON *kinds*; this is install's write fidelity into a file it
  does not own. Blocks a clean install footprint on a hand-authored
  settings.json.

- `(orphaned-projection)` — OPEN (field report 2026-07-07). Deleting a member
  from the program and re-emitting reports "N unchanged" and leaves the emitted
  `.claude/` file on disk — unowned, still loading; the lock forgets the path
  with no removed-state, no reap, no report (`specs/model/pipeline.md`, "Emit":
  "Total, and write-only" does not say whether a now-ownerless projection is
  reaped or reported). Detection is decidable (a lock-known projection with no
  current owner); the remedy — reap the file vs. report it as drift — is the
  open fork. Live hazard: a stale projection keeps loading into the agent.

- `(file-edge-resolution)` — OPEN (field report 2026-07-07). `file()` edge
  resolution disagrees between doc and engine: the SDK's `prose.ts` documents
  module-relative, emit resolves against `baseDir ?? cwd`
  (workspace-relative). Scaffold's `../.claude` paths work only because
  `.temper/` sits one directory deep. One of the two must change; scaffold
  output, resolution, and docs move together in one entry. Fork: which locus is
  authoritative (module-relative or workspace-relative)?
  (`specs/model/pipeline.md`, "The SDK" — an include pulls the target's
  content; the resolution locus is unstated.)

- `(emit-eol-policy)` — OPEN (field report 2026-07-07). On Windows, emitted
  projections mix LF (emit's connective tissue) with CRLF (a verbatim body
  copied from a CRLF source), tripping git's LF/CRLF warnings and breaking
  byte-reproducibility (`specs/intent.md`, invariant 3). A deterministic EOL
  policy is needed; candidate: always LF. The tension is real — normalizing the
  body's EOL touches verbatim prose (invariant 3, "never rewords"), so "always
  LF" is a ruling about whether EOL is meaning-carrying, not a free default.
  Fork before an entry can encode it.

- `(surface-vocabulary)` — OPEN (field report 2026-07-07, needs John). The
  remaining Claude Code surfaces need declared vocabulary before they can be
  modeled or gated: slash commands need a registration value for
  **user-invoked** (the existing registration vocabulary is always /
  description-trigger / paths-match / event / connection); hooks and settings
  need kinds. Until ruled, two coverage advisories stay permanent — including a
  decidable "an entry in `.claude/` matching no kind's `governs` and no known
  surface" advisory (a bogus `.clauignore` currently sails through). Joins the
  `(json-projection-format)` family (`specs/builtins.md`, "The named
  expansion" — commands/agents/hooks/permissions are forward work).

## Kept on purpose — deliberate asymmetries (re-read every tick)

Every asymmetry below is a **choice with a condition**, not a fact. When its
condition arrives, it is the next break. If work touches one, surface it.

- **Default-contract auto-adoption** (a bare harness gets the built-in kinds
  checked with no assembly declaration) — kept for the zero-config front door;
  the engine embeds a built-in lock, the default contract in declaration shape,
  so a lockless harness is still fully gated (`specs/model/pipeline.md`, "The
  lock"). Data, not code.

- **Format implementations are engine code** (the generic frontmatter adapter)
  — kept because an external format's mechanics are temper's to implement once;
  the kind that selects them is data (`specs/model/representation.md`, "kind":
  a kind is data, its extractor composed from that data). Grows only by
  deliberate addition.

- **`kinds/` + `packages/` curated trees — RETIRED.** The engine retirement
  drained and the physical trees were deleted (`chore(harness)` 68f187d). One
  standing behavioral debt survives: `tests/session_start.rs` still writes
  `+++`-format `.temper/kinds/spec/KIND.md` + `.temper/packages/spec/PACKAGE.md`
  fixtures — live test code asserting stray old-format files are ignored — to
  be reconciled when the session-start path is next touched (accepted debt,
  verify in the next ship audit).

- **`.flume/` is ungoverned by temper** — the machine that builds temper is not
  yet under its gate; a candidate governed corpus once the custom-kind story
  proves end to end (`specs/model/representation.md`, "Reach").

- **`docs/` is candidate intent, not intent** — human territory,
  fence-excluded; plan never reads a horizon entry as intent.
