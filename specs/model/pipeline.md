# Pipeline — emit · lock · drift

How the model becomes files and stays true: the harness value (`authoring.md`)
is compiled deterministically into the committed artifacts plus the lock, and
every later question — is the gate green, has anything moved — is answered
from that committed pair, offline.

## Emit

The deterministic compile of the harness value into artifacts plus the lock.
Its codomain is the committed tree: temper is an authoring tool — a
projection is an iterative update to a source-controlled artifact, and
carrying bytes to machines is source control's job. Emit writes no
uncommitted path; a local member is a check-side input, never a target.

- **Total, and write-only.** Members are the only source; every artifact is
  its member's format evaluated over its values; no file is part emitted,
  part hand-maintained. Nothing ever parses a projection back — a hand-edited
  projection is drift by hash, answered by editing the owning source. An
  embedded member's serialized form is the artifact only, regenerated whole,
  its facts captured as declaration rows in the same pass, never a second
  copy the engine reads back. A layout kind's document is the opposite case
  and the one governed source: emit reads it under the declared layout,
  derives its rows in the same pass, and writes nothing at its path — never
  regenerated, never reaped. Derived facts are computed, never authored
  twice. Total runs in reverse too: emit reaps a projection whose owning
  member is gone when the file is byte-identical to its lock fingerprint —
  temper wrote every byte, so nothing authored is lost; a drifted ownerless
  file is a finding, never a silent delete.
- **Verbatim.** Every meaning-carrying word in an emitted artifact traces to
  an authored prose leaf or a reference's rendered form; connective tissue —
  headings, labels, ordering — is projection formatting, and emit stamps
  nothing of its own. Line endings are layout, never content: projections are
  written LF uniformly, whatever the source's convention.
- **Byte-reproducible.** Same program in, same bytes out, verified by
  double-emit comparison on every run; nondeterminism in authoring code is a
  loud emit failure, never silent churn. The program reaches the engine as a
  versioned internal payload — every type erases, the engine sees only plain
  data — and integrity is verified where it is honestly verifiable: CI
  re-emits and byte-compares.
- **Refusing.** A dangling edge, an unresolvable mention or include, an
  unfilled requirement whose fill clause errors — each refuses before a byte
  is written. Refusal reaches exactly as far as the program's own universe: a
  field the kind's schema marks optional is no edge when absent; a mention
  addressing a declared kind whose member is not a composed value defers to
  the gate — the row rides the lock, `check` owns the verdict — and a mention
  addressing no declared kind refuses.

## The lock

The committed anchor: tool-written whole, never patched, and emit is its sole
producer. The gate reads declarations from nowhere but the lock family — the
committed lock, the read-time rows of local-locus members whose kinds it
declares, and the locks the invocation joins; no verb compiles a committed
declaration row anywhere else. Two row families: **provenance** — per member,
source path plus content hash, and the byte hash of each emitted artifact:
the fingerprints drift compares — and **declaration rows** — the program's
erased declarations and the rows emit derives from layout sources. In
declaration rows, identity is a compiled label written once at emit; the
engine treats labels as opaque and never resolves a collision — two rows
wearing one label is a malformed lock, rejected at admissibility.

An upgraded engine owes a committed lock a robust read and a canonical
rewrite: older spellings normalize at read time, the file itself is never
patched, and the next emit rewrites it whole in canonical form; a true
collision stays a malformed lock, refused loud. No upgrade is silent about
scale: a reap wave that would delete every live projection while emitting
nothing, or a re-read that drops a whole declared layer the lock still
carries, refuses with the finding stated — a full teardown is an explicit
flag the author spells, never a side effect.

The gate and every read verb consume committed artifacts plus the lock family
and nothing else: offline, no language runtime. A harness with no lock is
still fully gated — the engine embeds a built-in lock, the default contract
in the same declaration shape, receipt-less because nothing was emitted. One
input shape, two sources; there is no third.

## Drift

One comparison in one vocabulary: disk versus lock. Two freshness facts, one
finding shape — an **authored source** differing from its provenance row (the
source moved; re-emit) and a **committed projection** differing from its byte
fingerprint (the projection was hand-touched; edit the owning source and
re-emit). Each finding names the member that owns the bytes, the side that
moved, and the remedy. The comparison is line-ending-blind — an EOL-only
difference is layout (**Verbatim**, above), so the comparator canonicalizes
EOL before it hashes; every non-EOL difference remains the hand-edit.

A mismatch is never silently reconciled: no reverse parse from projection to
source, no merge model — a projection edit routes to the owning member. How
loudly a hand-edit is treated is the author's declared severity, never the
tool's own determination; the enforcement artifacts bind only paths the lock
names as projections, so with no lock nothing is a projection.

## Telemetry

A telemetry declaration projects as tap hook registrations in the emitted
manifest — the same rows any hook rides. The tap appends event records to its
own log — the lock's category, never a member's: machine-written,
bespoke-parsed, versioned in lockstep with the one binary that both writes
and reads it. The log is per-machine, uncommitted, never an emit input or
target, and an append is a single record: parallel sessions interleave lines,
never rewrite the file. A reader meeting records an older tap wrote tolerates
them out loud — a count in the narration, never a silent skip — and a reader
meeting no records where the lock declares tap registrations states that
absence the same way: the declared wiring against the empty log, evidence
never verdict. A record is an event's identity and its minimal discriminant —
the member or path the event names, the load reason, the session id — never
captured prose: no tool output, no prompt text, no message bodies; the bound
is the tap's contract, not its configuration. Interpretation happens at read
time alone: the reader joins raw events to members through the lock's own
declarations, so the tap stays dumb and the record stays honest — a fact
about what fired, read for narration, never mined for model structure.

## Read verbs

`explain` is the one read verb — narration and removal fallout (its impact
strand) over the same resolved edges the gate uses (`contract.md`); it reads
the committed pair, never gates.
