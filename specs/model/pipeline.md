# Pipeline — SDK · emit · lock · drift · install

How the model becomes files and stays true: the harness is authored as one
typed value, compiled deterministically into the committed artifacts plus the
lock, and every later question — is the gate green, has anything moved — is
answered from that committed pair, offline.

## The SDK

The typed authoring surface: an npm module in which members are typed values
and composition is an import. The author composes the whole harness as one
program; `harness()` constructs the root member (`representation.md`) — its
nested members, its contract bindings (clauses over selections, requirement
members), and its own fields, settings keys among them, the unschematized
ones opaque and named as such. Composing is ordinary code, typed at the
keystroke: a default contract is an exported clause array — adoption is the
import, overriding is composing the array — and harness families cost
nothing.

The SDK implements no semantics: types, constructors, and one pipe to the
engine. Turing-completeness is quarantined at authoring time — running the
program is the author's act; checking never needs Node. A member's prose
keeps its authored medium — a document stays in its own file, a three-line
rule may live inline — and either way the words land byte-identical in the
projection. A composed body may interleave verbatim prose spans with
embedded member values in authored order — the write-side mirror of a
layout's ordered regions (`representation.md`, "kind"); a narrative span is
prose, never a wrapper member minted to carry it. Prose may declare references, two intents apart: a mention names
a member and moves no content; an include pulls the target's content into
the host's emitted bytes, a dependency the lock fingerprints. Both are
declared edges (`contract.md`), and a path reference resolves relative to
the module that states it, never the workspace; every other word is just a
word.

## Layers

The harness value composes from an **ordered stack of layers** under the
same merge the SDK already speaks — later layers win by array surgery, one
algebra for every layer, no layer a special species (decision 0030). Which
slots exist and their order is the **provider's documented fact**, carried
as cited data on the provider face, never chosen here; the claude-code
face emulates the product's own precedence, with the user slot named and
reserved, ungoverned.
Temper adopts the provider's ordering, not its override semantics: the
bounds below are this gate's own rules.

A layer is a **locus fact, never a mechanism** (decision 0032): a member
of a kind whose locus is local, or a lock the invocation joins. Two rules
attach to anything **uncommitted** — content review never saw — never to
a layer's name:

- **Check-side, structurally.** A local member (the read-side-only class,
  `representation.md`, locus §) contributes no emit output, so the lock
  captures the committed harness alone and committed bytes are
  layer-invariant by construction. Its rows derive at read time under the
  kind the committed lock declares: review fixes what a local file may
  say; the gate reads what it says.
- **Review is the price of softening.** The committed harness may soften
  anything — that is a reviewed diff. Uncommitted content hardens
  without bound, in every mode; it softens only through the shipped dial
  kind (`contract.md`, "clause"), whose softening is inert in block
  mode, so a block-mode pass on any machine implies the shared gate's
  pass.

A policy layer arrives with the invocation, **as a lock**: `check` joins
the declaration rows of locks its invocation names over this corpus —
kinds travel by name, the joined clauses range over the host's
selections, and the lock stays the one interchange. Whoever owns the
invocation owns the top of the stack; org authority is the org's
pipeline definition — no trust model here. A joined lock that fails
admissibility fails the check, fail-closed. `check` announces every
active local member, every dialed clause, and every joined lock.

## Telemetry

A telemetry declaration projects as tap hook registrations in the emitted
manifest — the same rows any hook rides. The tap appends event records to
its own log — the lock's category, never a member's: machine-written,
bespoke-parsed, internal and versioned in lockstep with the one binary
that both writes and reads it (the emit payload's precedent; the format
vocabulary stays the members'). The log is per-machine, uncommitted, and
never an emit input or target — the local class's guarantees by
placement, not by membership — and an append is a single record:
parallel sessions interleave lines, never rewrite the file. A reader
meeting records an older tap wrote tolerates them out loud — a count in
the narration, never a silent skip. A record is an event's
identity and its minimal discriminant — the member or path the event
names, the load reason, the session id — and never captured prose: no
tool output, no prompt text, no message bodies. The bound is the tap's
contract, not its configuration. Interpretation happens at read time
alone: the reader joins raw events to members through the lock's own
declarations, so the tap stays dumb and the record stays honest — a fact
about what fired, read for narration, never mined for model structure.

## Emit

The deterministic compile of the harness value into artifacts plus the lock.
Its codomain is the committed tree: temper is an authoring tool — a
projection is an iterative update to a source-controlled artifact, and
carrying bytes to machines is source control's job. Emit writes no
uncommitted path; a local member is a check-side input, never a target
(decision 0034).

- **Total, and write-only.** Members are the only source; every artifact is
  its member's format evaluated over its values; no file is part emitted,
  part hand-maintained. Nothing ever parses a projection back — the read
  side stays on the formats the engine knows, standard and declared-layout
  alike, and a hand-edited projection is drift by hash, answered by editing
  the owning source. An
  embedded member's facts are declaration rows, captured the same emit pass
  that renders it; its serialized form is the artifact only, regenerated
  whole, never a second copy the engine reads back. A layout kind's document
  is the opposite case and the one governed source: emit reads it under the
  declared layout, derives its declaration rows in the same pass, and writes
  nothing at its path — never regenerated, never reaped. Derived facts are
  computed, never authored twice: the permission list is the union of the
  members' declared capability needs, so a permission no member needs is
  visible as exactly that. Total runs in reverse too: emit reaps a
  projection whose owning member is gone when the file is byte-identical to
  its lock fingerprint — temper wrote every byte, so nothing authored is
  lost; a drifted ownerless file is a finding, never a silent delete.
- **Verbatim.** Every meaning-carrying word in an emitted artifact traces to
  an authored prose leaf or a reference's rendered form; connective tissue —
  headings, labels, ordering — is projection formatting, and emit stamps
  nothing of its own into a projection. Line endings are layout, never
  content: projections are written LF uniformly, whatever the source's
  convention.
- **Byte-reproducible, mechanically double-checked.** Same program in, same
  bytes out, verified by double-emit comparison on every run; nondeterminism
  in authoring code is a loud emit failure, never silent churn.
- **Refusing.** A dangling edge, an unresolvable mention or include, an
  unfilled requirement whose fill clause errors — each refuses before a byte
  is written; the author cannot produce output from a broken source.
  An unfilled edge field refuses nothing: a field the kind's schema marks
  optional is no edge when absent — requiredness is the kind's own schema,
  failing in the author's program at compose time — and refusal reaches
  only a reference that is filled yet unresolvable.
  Refusal reaches exactly as far as the program's own universe: a mention
  addressing a kind the program declares whose member is not a composed
  value defers to the gate — the row rides the lock and `check` owns the
  verdict at the same table `implemented-by` resolves on — while a mention
  addressing no declared kind is dangling here and refuses. The deferral's
  signal is the declared kind, never a guess over the address.

The **emit payload** is the versioned JSON the SDK program prints and the
engine consumes. Every type erases into it: the engine never sees a
constructor, only plain data. It is internal, versioned in lockstep — the SDK
pins its engine version — and is not a designed public interchange; one is
admitted when a consumer exists, and none does. Source↔artifact integrity is
verified where it is honestly verifiable: CI re-emits and byte-compares.

## The lock

The committed anchor: tool-written whole, never patched, and emit is its sole
producer. The gate reads declarations from nowhere but the **lock family**:
the committed lock, the read-time rows of local-locus members whose
kinds it declares, and the locks the invocation joins (decisions 0032,
0034) — no
verb compiles a committed declaration row anywhere else. Two row families: **provenance**
— per member, source path plus content hash, and the byte hash of each
emitted artifact: the fingerprints drift compares — and **declaration rows**
— the program's erased declarations and the rows emit derives from layout
sources: kinds, clauses, requirement members,
nested-member facts, the root member's bindings. In declaration rows,
identity is a compiled label written once at emit; the engine treats labels
as opaque and never resolves a collision — two rows wearing one label is a
malformed lock, rejected at admissibility.

An upgraded engine owes a committed lock a robust read and a canonical
rewrite (decision 0024): joins over its rows normalize spellings on both
sides at read time, and a bare label an older engine wrote qualifies
against the corpus where unambiguous — the file itself is never patched,
and the next emit rewrites it whole in canonical form. A true collision
stays a malformed lock, refused loud. And no upgrade is silent about
scale: a reap wave that would delete every live projection while emitting
nothing, or a re-read that drops a whole declared layer the lock still
carries, refuses with the finding stated — a full teardown is an explicit
flag the author spells, never a side effect.

The gate and every read verb consume committed artifacts plus the lock
family and nothing else: offline, no language runtime. A harness with no lock is still
fully gated — the engine embeds a built-in lock, the default contract in the
same declaration shape, receipt-less because nothing was emitted. One input
shape, two sources; there is no third.

## Drift

One comparison in one vocabulary: disk versus lock. Two freshness facts, one
finding shape — an **authored source** differing from its provenance row (the
source moved; re-emit) and a **committed projection** differing from its byte
fingerprint (the projection was hand-touched; edit the owning source and
re-emit). Each finding names the member that owns the bytes, the side that
moved, and the remedy.

A mismatch is never silently reconciled: no reverse parse from projection to
source, no merge model — a projection edit routes to the owning member. How
loudly a projection hand-edit is treated is the author's declared severity,
never the tool's own determination; the enforcement artifacts bind only paths
the lock names as projections, so with no lock nothing is a projection.

## Install

The single adoption verb. It opens with the discovery report — findings
first, ceremony after: what the walk finds, members by kind, and what the
built-in default contract says about them. Discovery honors the repository's
ignore rules — an ignored file is by declaration not authored here, with
one scoped override: a committed local-locus kind's `governs` declaration
is itself the authorship claim over its per-machine documents, so
discovery sees them despite ignore rules or the workspace skip, there and
only there (decision 0034) — and
stops at a nested governed root: a directory carrying its own
`.temper/lock.toml` is its own corpus, and its members are never the
parent's. Reaching into one is a verb aimed at it on purpose, never the
ambient walk. Then one
question: represent this harness as a typed program? A harness is represented
or it is not — one genuine fork, so exactly one question.

- **No** — install wires the session-start reporter, with consent for the one
  settings write, and stops: a footprint of one entry, Node-free forever. The
  settings write is format-preserving — existing keys, order, and formatting
  survive the insertion; install never re-serializes a file it does not own.
- **Yes** — this path requires Node and the workspace, checked up front and
  refused with instructions when absent; no half-scaffolded state. Install
  installs the tool whole: it ensures the SDK dependency and converts each
  discovered artifact into a member module — every schema-declared field a
  typed property, prose module-side and byte-faithful (inline for short
  bodies, a module-adjacent file for documents) — then runs the first emit
  (the adoption moment: the lock, plus each artifact regenerated as a
  canonical projection, the one reviewable adoption diff), and places what
  that lock justifies.

The conversion writes no lock and compiles nothing: adoption is the first
emit. There is no intermediate depth — unrepresented, every artifact is a
source; represented, every composed kind's artifact is a projection, and a
layout kind's document is a source at either depth: its authored home never
moves, so the lift never converts it. Re-running
install converges, placements following the lock's current contents. The
verbs target one project's harness at an explicit path.

## Read verbs

`explain` is the one read verb — narration and removal fallout (its impact
strand) over the same resolved edges the gate uses (`contract.md`); it reads
the committed pair, never gates.
