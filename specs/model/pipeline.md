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
members), and a residual of settings fields with no member home yet, a list
that only shrinks. Composing is ordinary code, typed at the keystroke: a
default contract is an exported clause array — adoption is the import,
overriding is composing the array — and harness families cost nothing.

The SDK implements no semantics: types, constructors, and one pipe to the
engine. Turing-completeness is quarantined at authoring time — running the
program is the author's act; checking never needs Node. A member's prose
keeps its authored medium — a document stays in its own file, a three-line
rule may live inline — and either way the words land byte-identical in the
projection. Prose may declare references, two intents apart: a mention names
a member and moves no content; an include pulls the target's content into
the host's emitted bytes, a dependency the lock fingerprints. Both are
declared edges (`contract.md`); every other word is just a word.

## Emit

The deterministic compile of the harness value into artifacts plus the lock.

- **Total, and write-only.** Members are the only source; every artifact is
  its member's format evaluated over its values; no file is part emitted,
  part hand-maintained. Nothing ever parses a projection back — the read
  side stays on the standard formats the engine knows, and a hand-edited
  projection is drift by hash, answered by editing the owning source. An
  embedded member serializes into its parent's artifact, and such files are
  regenerated whole. Derived
  facts are computed, never authored twice: the permission list is the union
  of the members' declared capability needs, so a permission no member needs
  is visible as exactly that.
- **Verbatim.** Every meaning-carrying word in an emitted artifact traces to
  an authored prose leaf or a reference's rendered form; connective tissue —
  headings, labels, ordering — is projection formatting, and emit stamps
  nothing of its own into a projection.
- **Byte-reproducible, mechanically double-checked.** Same program in, same
  bytes out, verified by double-emit comparison on every run; nondeterminism
  in authoring code is a loud emit failure, never silent churn.
- **Refusing.** A dangling edge, an unresolvable mention or embed, an
  unfilled requirement whose fill clause errors — each refuses before a byte
  is written; the author cannot produce output from a broken source.

The **emit payload** is the versioned JSON the SDK program prints and the
engine consumes. Every type erases into it: the engine never sees a
constructor, only plain data. It is internal, versioned in lockstep — the SDK
pins its engine version — and is not a designed public interchange; one is
admitted when a consumer exists, and none does. Source↔artifact integrity is
verified where it is honestly verifiable: CI re-emits and byte-compares.

## The lock

The committed anchor: tool-written whole, never patched, and emit is its sole
producer — no verb compiles anything else into declaration rows, and the gate
reads declarations from nowhere but the lock. Two row families: **provenance**
— per member, source path plus content hash, and the byte hash of each
emitted artifact: the fingerprints drift compares — and **declaration rows**
— the program's erased declarations: kinds, clauses, requirement members, the
root member's bindings. In declaration rows, identity is a compiled label
written once at emit; the engine treats labels as opaque and never resolves a
collision — two rows wearing one label is a malformed lock, rejected at
admissibility.

The gate and every read verb consume committed artifacts plus the lock and
nothing else: offline, no language runtime. A harness with no lock is still
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
ignore rules — an ignored file is by declaration not authored here. Then one
question: represent this harness as a typed program? A harness is represented
or it is not — one genuine fork, so exactly one question.

- **No** — install wires the session-start reporter, with consent for the one
  settings write, and stops: a footprint of one entry, Node-free forever.
- **Yes** — this path requires Node and the workspace, checked up front and
  refused with instructions when absent; no half-scaffolded state. Install
  ensures the SDK dependency, lifts each discovered artifact into a member
  module whose prose stays in its original file — zero rewording, byte-stable
  on content — runs the first emit (the adoption moment, producing the lock),
  and places what that lock justifies.

The lift writes no lock and compiles nothing: adoption is the first emit.
Members arrive shallow and fully functional; deepening accrues member by
member under the author's own requirement coverage failing, never under
on-ramp ceremony. Depth is emergent, never declared — no depth selector, no
recorded preference: re-running install converges on whatever the program has
become, placements following the lock's current contents. The verbs target
one project's harness at an explicit path.

## Read verbs

`explain` narrates and `impact` reports removal fallout over the same resolved
edges the gate uses (`contract.md`); both read the committed pair, never gate.
