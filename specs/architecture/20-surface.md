# The surface — the SDK: authoring, prose, emit, the seam, init

The surface is where the harness **lives**: the **SDK**, a typed module
library in which members are typed values, composition is an `import`, and
**emit** compiles the whole into the committed harness artifacts
(`00-intent.md`, laws 5 and 7 and the SDK Decision). Members are the only
source; every artifact is a projection; the engine reads only committed
artifacts plus the lock. Source and output are different media on purpose — a
module compiling to an artifact reads as src→dist, where a same-medium mirror
reads as a copy that isn't.

This spec owns the authoring surface (the member and its prose), emit, the
SDK↔engine seam, and `init`. The assembly value that binds the whole —
`harness()` — is `40-composition.md`'s; kinds, genres, loci, and the three
authoring postures are `15-kinds.md`'s; clauses and requirements are
`10-contracts.md`'s; the graph and its judges are `45-governance.md`'s.

## The port scene

The audience the surface is designed for is the agent authoring under a
human's contract (`00-intent.md`, Positioning) — which means the artifact the
*human* actually handles is a review diff. The face must pass that reading.
An AI co-author ports a harness into the SDK; the reviewer opens:

```ts
// .temper/skills/reviewer.ts
import { skill, file } from "@dtmd/temper/claude-code";

export const reviewer = skill({
  name: "reviewer",
  description: "Review the working diff for correctness bugs before commit.",
  prose: file("./reviewer.md"),
});
```

Recognition is the test: *that's my skill, and that's its intent* — no
framework noise, no record-about-a-record, the prose untouched in the markdown
file it always lived in. Deepening, later, is an additive two-line diff:

```diff
 export const reviewer = skill({
   name: "reviewer",
   description: "Review the working diff for correctness bugs before commit.",
+  satisfies: ["review-coverage"],
+  needs: [bash("git diff")],
   prose: file("./reviewer.md"),
 });
```

Two lines: the member now fills a declared requirement (`40-composition.md`)
and its permission entry is derived at emit. No restructuring, no new file, no
layer to learn before the first line reads.

```
.temper/                    # the authored program — the SDK surface
  harness.ts                #   harness(): members · expect · require · settings · reachability
  skills/reviewer.ts        #   a member module (the face)
  skills/reviewer.md        #   its prose asset — file(), posture 1
  lock.toml                 # tool-written: provenance + emit fingerprints + declaration rows
.claude/**  .mcp.json       # the projection: emitted whole, never hand-edited
```

Two provenance classes, never blurred: **authored** (the modules and their
prose assets) and **generated** (the projection and the lock).

## Two registers — the face and the engine room

- **The face** is what a harness author imports: plain nouns — `skill`,
  `rule`, `hook`, `mcpServer`, … one per built-in kind — from
  `@dtmd/temper/claude-code` (`50-distribution.md`), plus `harness()`, the floors,
  and the prose constructors. Modules are harness-shaped: a file per member, a
  directory per kind if the author likes, ordinary exports. The face is the
  product.
- **The engine room** is the `kind<T>()` and `genre<T>()` constructors
  (`15-kinds.md`): integrator territory, where a provider defines a new kind —
  its five runtime facts and its typed field interface. Most authors never
  open this register.

The registers are one mechanism: the face's nouns are themselves engine-room
output — the built-in kinds are a published SDK module built with the same
constructors every provider uses (`15-kinds.md`, ownership not privilege).

## The member

```
Member = name · prose? · satisfies? · requires? · needs? · <typed fields: scalar | Prose>
```

- **`name`** — identity within its kind.
- **`prose`** — the member's words (next section).
- **`satisfies`** — string keys naming the requirements this member fills;
  keys resolve at emit, and a dangling key is a finding
  (`40-composition.md`, the satisfies-keys Decision).
- **`requires`** — requirements the member itself publishes (a skill that
  only works beside a companion hook says so where it lives); the
  `Requirement` shape is `10-contracts.md`'s.
- **`needs`** — the capabilities the member's behavior uses, declared as
  typed values. Emit derives the settings permission list from their union
  (below), so a permission is never authored twice.
- **typed fields** — the kind's own vocabulary (`description`, an event, a
  path scope…), **flat at the top level**. There is no `fields:` bag: the
  member reads as the artifact, not as a record about one. A kind's
  registration — the world-facing fact reachability stands on
  (`45-governance.md`) — is declared through these fields, not beside them.

Field typing lives in **plain named interfaces**. Hover a member and the type
is `Skill`, its fields TSDoc-documented — the hover channel is where a floor's
guidance meets the author mid-keystroke (`10-contracts.md`, guidance). A
typo'd field errors in domain vocabulary, never derived-type soup
(`Omit<…> & Partial<…>`): the interfaces are hand-named, not derived. tsc is
the keystroke wall; past the wall, gate findings speak the same names via the
compiled debug labels the declarations carry (`15-kinds.md`).

A **genre is a kind at the block locus** (`15-kinds.md`): its members live
inside host documents, extend `Member` — so they carry `satisfies` and
`requires`, enter the graph, and fill requirements cross-kind — and inherit
registration through their host. Their meaning-carrying fields are
Prose-typed; one field concept everywhere: scalar = data, Prose =
law-5-protected words.

## Prose — three constructors, one field type

```
Prose = file() | text`…` | blocks(…)
```

- **`file(path)`** — the document keeps its medium: markdown in a markdown
  file, full tooling, forever legal (posture 1, `15-kinds.md`).
- **`` text`…` ``** — short prose inline, dedented, byte-deterministic; the
  three-line rule that would be silly as a sidecar file.
- **`blocks(…)`** — fully composed genre values (posture 3): typed
  collections whose emitted document is pure render, byte-identical to the
  same values authored as fences in a document (posture 2; the shared fence
  format is `(genre-fence-format)`).

Whatever the constructor, the words land byte-identical to their authored
text (law 5, `00-intent.md`).

### Two reference intents — mention and embed

Prose may interpolate declared values, with two distinct intents:

- **`${x}` is a mention** — it *names* a member: an edge, no content flows.
  Rendered by one corpus-wide display rule; resolution-checked (a mention
  cannot dangle); obligation-free — `explain` reports mentions as citations,
  never fallout. In a posture-2 document a mention spells identically —
  `${address}` in the markdown, not recognized inside code spans or fenced
  blocks (`(mention-marker)` resolved 2026-07-04: one spelling across the
  postures — a second marker would be a synonym for the model's own edge).
- **`${embed(x)}` is a pull** — the target's content flows into the host's
  emitted bytes, so the edge is a content dependency the lock fingerprints.
  Today's `CLAUDE.md` `@path` import is the embed's harness spelling — the
  format's own structure, not prose (law 8, `00-intent.md`).

Two edge species, both declared, both in the graph (`45-governance.md`).
Renaming a declared value flows through every reference — fearless
refactoring (law 6) reaching inside paragraphs.

### Decision: prose is a typed field, never a wrapper

**Chosen:** a member's prose is data the member declares — `file()` for
documents, `` text`…` `` for short inline prose, `blocks()` for composed genre
values — rendered byte-deterministically at emit; the words are the author's
untouched (law 5). **Rejected:** (a) prose-only-inline — template literals are
hostile to long documents (no markdown tooling, escaping noise), and the tax
lands on exactly the artifacts authored most; (b) prose-only-sidecar — two
files for a three-line rule; the author picks per member; (c) a template
*language* over prose (loops, conditionals rendering different text per
emit) — emitted prose must be one authored text, byte-stable; families of
members are generated at the value level, never inside one member's words.

### Decision: two reference intents, opt-in per word — no completeness check, ever

**Chosen:** citation and inclusion are different facts — one moves bytes, one
never does — so they are two declared species, `${x}` and `${embed(x)}`. The
author marks which words are references by interpolating them; every other
word is just a word. Plain prose with zero references is a fully legal member
of every kind, and no clause may quantify over reference completeness ("this
paragraph should have modeled its nouns") — law 8's opt-in bound
(`00-intent.md`). **Rejected:** (a) one reference form for both intents — a
syntax that sometimes moves content makes the difference invisible exactly
where it is reviewed; (b) auto-linking recognized names in prose — mining
with a friendlier name; recognition is authorship, never inference; (c)
mention-completeness or -density clauses — the mining swamp rebuilt from the
declaration side, law 4 at its finest grain.

## Emit — total, byte-reproducible, refusing

**Emit is total: members are the only source.** Every artifact — `.claude/*`,
the settings file, `.mcp.json`, a genre-bearing spec document — is a
projection of members. The interesting cases are many-to-one:

- **Hook members fold into the settings artifact**; **MCP members fold into
  `.mcp.json`**. Folded files are regenerated whole and fingerprinted in the
  lock — there is nothing of the human's in them to lose, so whole-file
  determinism replaces format-preserving caution.
- **The permission list is derived, never authored**: `permissions.allow` is
  the union of the members' declared `needs`. A permission with no member is
  visible as exactly that.
- **Residual harness-level settings** ride `harness()`'s `settings` field — a
  shrinking list (`40-composition.md`, `(settings-residual)`).
- **Genre-bearing documents render by the display rule** — one emit-owned,
  byte-deterministic rendering per genre. Connective tissue (headings,
  labels, ordering) is projection formatting; every meaning-carrying word
  traces to an authored leaf or a reference's rendered form (law 5).

**Emit refuses before it writes.** A dangling `satisfies` key, an
unresolvable mention or embed, an unfilled `required` requirement: each is an
emit refusal in the toolchain lane — the author cannot produce output from a
broken source.

**Emit is byte-reproducible, and checked** (law 5: "byte-reproducible and
mechanically checked"). Same program in, same bytes out, verified by
double-emit comparison at every run — nondeterminism in authoring code (a
timestamp, an unordered map) is a loud emit failure, never silent churn. The
fixpoint holds at the surface: emitting, then lifting the emitted output
(`init`, below), yields the surface back.

### Decision: emit is total — no partial territory

**Chosen:** every harness artifact is a projection; a file is never part
emitted, part hand-maintained. Registrations live on members; permissions are
derived from `needs`; what has no member home is a declared `settings`
residual. **Rejected:** (a) hand territory inside emitted files (emit patching
some keys of a settings file humans also edit) — two writers for one file is
the drift engine's blind spot, and every merge rule is a place authorship
blurs; (b) authored permission lists beside declared `needs` — the same fact
in two homes, guaranteed to diverge; (c) emit-stamped managed-by prose inside
projections — the projection is the member's content, and a stamping projector
breaks law 5 for every downstream byte-comparison.

## The seam — one implementation

The SDK implements **no semantics**: types, constructors, and one JSON pipe to
the engine. Running the authored program produces plain data — **every type
erases at the seam**: kinds, genres, clauses, the assembly are TypeScript
values that compile to declaration rows; the engine never sees a constructor.
The in-flight JSON is internal, versioned in lockstep — **the SDK pins its
engine version** — and is not a designed IR: a stable public interchange is
admitted when its consumer lands (the entry gate), and none exists.

The engine does the semantics: **emit, lock, gate, explain**. It is kind- and
schema-blind — extraction is the generic algebra, a kind's runtime residue is
its five declaration facts, judgment is compiled predicates behind one
admissibility stage (`15-kinds.md`, `45-governance.md`). Checking never needs
a language runtime (law 3's quarantine, `00-intent.md` SDK Decision): the
gate, CI, and `explain` consume **committed artifacts plus the lock** —
that pair is the committed seam. A harness with no SDK program at all is
still gated by the compiled default program embedded in the engine binary
(`50-distribution.md`) — SDK-less checking, the no-toolchain trial and the
visitor mode for repos you don't own.

Source↔artifact integrity is verified where it is honestly verifiable: **CI
re-runs `emit --frozen` and byte-compares** the result against the committed
projection.

The engine's implementation language is deliberately non-normative
(`(engine-language)` resolved 2026-07-04: the engine stays in Rust); this
corpus says "the engine" and "the temper binary or its equivalent",
never a language as a contractual fact.

### Decision: one authored surface, one implementation

**Chosen:** the SDK is the only authoring surface and implements no
semantics; the engine is the only implementation of emit, lock, gate, and
explain; the seam between them is plain data. (Re-cut 2026-07-04; pre-states:
the `mirror-era`, `bound-prose-era`, `manifest-era` tags.) **Rejected:**
(a) script-as-canonical configuration — the engine executing author code to
learn the contract dissolves decidability, determinism, and the offline gate
in one move; Turing-completeness stays quarantined at authoring time;
(b) a committed manifest as the gate's corpus — face↔file integrity is
unverifiable without re-emitting, so the honest check is CI's `--frozen`
byte-compare, and the committed file adds only a review surface nobody reads;
(c) a maintained hand-TOML authoring surface beside the SDK — a permanent
second surface (its own docs, keystroke channel, format-preserving patcher,
an adoption ladder to dignify it) serving no author this product has;
(d) a TypeScript re-implementation of emit beside the engine — two
implementations of one byte-contract drift apart, and the seam exists
precisely so semantics live once;
(e) a designed stable IR ahead of a consumer — the entry gate: a key,
predicate, verb, or grain enters when its consumer lands, and no consumer of
a public interchange exists.

## The lock and drift — one vocabulary

The lock (`.temper/lock.toml`) is tool-written, never composed, and carries
three row families:

- **provenance** — each member's source fingerprint (law 5: "the lock carries
  every member's provenance fingerprints — the drift anchor");
- **emit fingerprints** — each emitted artifact's byte hash;
- **declaration rows** — the program's erased declarations: kind facts,
  clauses, requirements, assembly facts (`40-composition.md`).

One producer writes all three families: **`emit`, compiling the SDK program —
the sole producer**. No verb compiles a hand-written configuration into
declaration rows, and the gate never reads declarations from anywhere but the
lock. A lock with no declaration rows is an **unadopted** harness — still
fully gated, SDK-less, by the embedded default program — never a half-adopted
one. (Resolves `(inplace-lock-producer)`, 2026-07-04: sole producer, clean
cuts.)

Drift is **one comparison in one vocabulary: disk vs lock**. An artifact
whose bytes differ from its lock fingerprint, a source that differs from its
provenance row — each is the same finding shape, naming the member that owns
the bytes and the side that moved. The remedy is named in the finding:
re-emit (the source moved), or edit the owning source and re-emit (the
projection was hand-touched). `emit --dry-run` previews what a run would
change.

### Decision: drift routes to the authored source — no reverse parse

**Chosen:** there is no round-trip from projection back to surface. A direct
edit to emitted output is drift — surfaced by the guard at the write boundary
and by the disk-vs-lock check at the gate — and the remedy is to edit the
owning member and re-emit, or to lift the change deliberately (`init`,
below). **Rejected:** a reconcile-back verb (`re-add`) and its three-state
merge model (desired / last-applied / real) — load-bearing only when source
and output shared a medium and an edit was ambiguous between them; with
src→dist media, patching the projection is patching `dist/`, and a tool that
merges dist-edits back teaches its authors the wrong home. (Resolves
`(surface-authority)`; supersedes the three-state Decision's mechanics.)

### Decision: surface authority is a declared posture, never a baked stance

**Chosen:** how loudly a projection hand-edit is treated is the author's
declaration, mapped onto the severity vocabulary — note = information, warn =
advisory, block = required — so temper never escalates on its own
determination. The enforcement artifacts are install-wired, enumerated,
self-audited, reversible: a managed-by note where the projection's format
tolerates cost-free metadata (never stamped by `emit` — law 5; memory
projections skip it, a comment there costs context every session), and a
guard hook at the provider's write boundary. The limit is stated, not solved:
the hook binds one provider's writes, so authority is only as strong as the
weakest uninstrumented consumer — the note is the only universal layer and CI
the backstop wall. The guard is the `temper guard` subcommand, blocking per
this declared posture (`50-distribution.md`, the guard Decision —
`(guard-posture)` resolved). **The lock is what names a path a projection**:
the enforcement artifacts bind only where the lock's declaration rows say
emit owns the bytes. An in-place member — its `file()` source and its
projected path are the same file — is authored territory: the note is never
stamped there and the guard never claims it as output, because a hand edit
to it is an edit to the source (drift then reads as a stale fingerprint, not
a violation). On a harness with no lock nothing is a projection and the
enforcement artifacts have nothing to bind (the bare binary checks; it never
adopts). (Resolves `(carriage-aware-placements)` — cascade field evidence,
ruled 2026-07-06.) **Rejected:** (a) baked-in blocking —
the tool determining invasiveness on a surface it was invited onto; (b)
`emit`-stamped notes — a stamping projector breaks law 5 for every downstream
byte-comparison; (c) framing the hook as a wall — multi-consumer loci
(`docs/market-formats.md`) make that a false promise. (Ratified 2026-07-03.)

## init — the lift, once

`init` is the one-time on-ramp, and its lift is **the posture move at scale**
(the three equal postures: `15-kinds.md`). For each discovered artifact it
scaffolds a member module whose prose is `file()` over the original text —
zero rewording, zero file moves for the words: recognition of the port scene
is the acceptance test. Where a document carries genre fences, the lift is
the posture-2 ⇄ posture-3 round-trip mechanized: parse to typed values,
render back to the fence, **byte-stable** — so the lift is reviewable as a
no-op on content. Law 5 fixes the license: "free to normalize framing, never
to alter content; after it the surface is the single authored home."

The lift's output is the SDK program and nothing else: member modules whose
prose is `file()` over the original text, and the `harness.ts` skeleton.
`init` writes no lock and compiles no configuration — the first `emit` after
the lift is the moment a harness becomes adopted ("The lock and drift",
above: emit is the sole producer). The bare binary **checks; it never
adopts**.

Members arrive shallow and fully functional; deepening (`satisfies`,
`needs`, `requires`) accrues member by member, and the pressure to deepen
comes from the author's own `require` coverage failing — the right
instrument — never from on-ramp ceremony. Before any lift, the harness is
already gated SDK-less (the embedded default program, above): the
no-commitment trial state.

### Decision: init is the lift; the postures stay equal

**Chosen:** one verb, one-time, per-member, byte-stable on content. Movement
between postures is author-initiated and free in both directions; the system
is not opinionated about where you author — no upgrade advisory, no adoption
metric, no lint counting fences. **Rejected:** (a) import-as-a-verb — copying
the whole harness into a second tree before the first finding; the on-ramp
cost is what the wedge lives or dies on; (b) a carriage gradient — an
adoption ladder of ranked authoring forms (in-place → document → module)
dignifies a permanent second surface and makes the SDK a rank instead of a
posture; (c) recognition demanded up front — intent-encoding accrues under
coverage pressure, not ceremony; (d) a binary-side declaration compiler — a
hand-authored `temper.toml` compiled into lock rows as an adoption rung —
which is the hand-TOML second surface (the seam Decision) re-entering
through the on-ramp: the postures are prose media, never config dialects
(`(inplace-lock-producer)` resolved 2026-07-04).

### Decision: discovery respects ignore rules; the backing set reads raw disk

**Chosen:** member **discovery** — `init`'s scan and the engine's walks —
always excludes `.git/` and honors the repository's ignore rules
(`.gitignore`): a member is authored content, and an ignored file is by
declaration not authored here (an any-depth memory glob must not import a
dependency tree's `AGENTS.md` files as the project's own members). The
**directive-backing file set** is the opposite case and stays **raw disk**:
whether an `@path` target is backed is a fact about the filesystem the
harness reads, and law 3 fixes the safe direction — an extra file in the
backing set can only *suppress* a finding, pruning it can *forge* one. Two
sets, two rules, never merged. **Rejected:** (a) raw-disk discovery —
strangers' files as members; (b) ignore-filtered backing — forged unbacked
findings on targets that exist; (c) a temper-specific ignore file — the
repository already declares what it considers authored, and a second
vocabulary would drift.

### Decision: the workspace is per-project

**Chosen:** the surface targets a per-project harness — the `.claude/` and
co-located artifacts of one project, at the explicit path the verbs take.
**Rejected (for now):** managing `~/.claude`, or both at once — the
per-project harness is the unit a contract gates and a session loads; global
config is a later extension the same engine handles as another landscape root
(`30-landscapes.md` — a landscape is just more kinds). (Resolves
`(workspace-scope)`.)

## CLI surface

`init · check · emit · install · bundle · explain · schema`

- **`temper init [<path>]`** — the on-ramp and the lift (above).
- **`temper check [<path>]`** — the gate: the engine's judge stage over the
  committed harness plus the lock (`45-governance.md`), including freshness
  (disk vs lock). Exit posture per law 1: hard where blocking is cheap,
  advisory at session start — **session-start is a reporter of `check`, not a
  verb**. Reporters: terminal, SARIF, session-start payload
  (`50-distribution.md`).
- **`temper emit [--frozen] [--dry-run]`** — the compile: members to their
  artifacts, declarations and fingerprints to the lock; refuses on
  declare-side failures; double-emit verified; `--frozen` refuses network and
  is CI's byte-compare posture.
- **`temper install`** — project the gate's wiring (session-start hook, CI
  job, guard, schema modeline) into the harness, drift-synced
  (`50-distribution.md`).
- **`temper bundle`** — compose into a publishable plugin
  (`50-distribution.md`).
- **`temper explain <target>`** — the one read verb (Decision below).
- **`temper schema [--kind <kind>]`** — emit the declarations as an editor
  JSON Schema for keystroke validation off the SDK path
  (`50-distribution.md`).

### Decision: one read verb — `explain`

**Chosen:** every read question is one graph walked from a different corner,
so one verb answers them over data `check` already computes: a member (what
holds it in place — the requirements it fills with their authored `means`,
the clauses it owes, its edges in and out), a requirement (its satisfier set,
coverage, and what a removal would strand — blast radius, law 6 made a verb),
an address at leaf grain (impact, with citations reported separately from
fallout), a neighborhood (the pre-edit context bundle for the primary
author). Projections, never gates: no new engine semantics, no non-zero exit
on findings. Output is a teaching surface in the corpus's vocabulary, and it
**discloses coverage**: mixed adoption is the standing state, so every
leaf-grain answer names what it cannot see — an incomplete answer wearing
complete clothes erodes the read verb exactly as a false block erodes the
gate (law 1). Offline, tier-1, committed artifacts plus lock only.
**Rejected:** (a) growing `check` flags into a query surface — the gate stays
a gate whose exit code CI trusts; (b) a general query verb — a query language
for a fixed set of walks; (c) a verb per question (`why` / `requirements` /
`impact` / `context`) — four spellings of one traversal, restating the
graph's shape in the CLI's top level. (Resolves `(read-verbs)` and folds in
the earlier read-family Decisions.)

## Scope boundary

This spec owns the member and its prose, the two registers, emit, the seam
and the lock, drift, `init`, and the CLI's shape. The assembly's five fields
are `40-composition.md`'s; kinds, genres, postures, extraction, and loci are
`15-kinds.md`'s; clauses, requirements, and `verifiedBy` are
`10-contracts.md`'s; the graph, registration, and reachability are
`45-governance.md`'s; publishing, the default program, and the gate's wiring
are `50-distribution.md`'s.
