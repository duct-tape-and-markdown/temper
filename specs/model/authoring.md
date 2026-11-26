# Authoring — SDK · layers

How the harness value is written: one typed program, composed in ordinary
code, stacked in layers under one merge algebra.

## The SDK

The typed authoring surface: an npm module in which members are typed values,
composition is an import, and `harness()` constructs the root member
(`representation.md`) — nested members, contract bindings, and its own
fields, settings keys among them, the unschematized ones opaque and named as
such. Composing is ordinary code, typed at the keystroke: a default contract
is an exported clause array — adoption is the import, overriding is composing
the array. The SDK implements no semantics — types, constructors, one pipe to
the engine; Turing-completeness is quarantined at authoring time, running the
program is the author's act, and checking never needs Node.

**One fact, one spelling.** Every fact the author declares has exactly one
place to declare it, and what declared facts already determine is derived,
never asked for again. A restatement the type system binds to its source —
an exhaustive witness the compiler refuses out of step — is that one
spelling carried across the erasure; an unbound restatement is two
spellings, and two spellings of one fact are one defect, never a choice.
A default the SDK lowers is the corpus's default: the model states it once
and the surface inherits it, never re-decides it.

A member's prose keeps its authored medium — a document in its own file, a
three-line rule inline — and either way lands byte-identical in the
projection. A composed body may interleave verbatim prose spans with embedded
member values in authored order — the write-side mirror of a layout's ordered
regions (`representation.md`, "layout"); a narrative span is prose, never a
wrapper member minted to carry it. Prose may declare references, two intents
apart: a mention names a member and moves no content; an include pulls the
target's content into the host's emitted bytes, a dependency the lock
fingerprints. Both are declared edges (`contract.md`), a path reference
resolves relative to the module that states it, never the workspace, and
every other word is just a word. The program's own **module closure** —
every module its run loaded that the repository tracks — is fingerprinted
at emit; the toolchain is held by its version and packages by their
lockfile, so neither is the program's own. An import is
a reference the authoring language executes, declared by being written, so
a value computed from an imported module is never fresher than that
module. A member may also declare **inputs**: non-code files its claims
rest on, fingerprinted by the lock like an include's target and moved
nowhere — no bytes reach the projection. A committed file temper does not
write joins the roster this way: a member whose record temper writes,
declaring the file as its input.

A **command** the harness runs is one typed reference, a member of the
shipped `tool` kind (`../builtins.md`), stated once and pointed at by what
runs it: a package's bin (`{ package, bin }`, its version the lockfile's,
its path the anchored `.bin` the program's package manager lays out), a
committed file (`{ file }`, a source joining the roster as an input), or a
system tool (`{ system }`, claiming presence and no version). Its runners (a
gate hook, a tap, the typecheck, a script verifier) reach it by declared
edges, and a command nothing runs is an advisory finding. the program's
`package.json` is a source temper scaffolds once and never writes: its
`packageManager` field is the declared manager, which install's scaffold
writes as the repository root's own `packageManager` where that is declared,
else the manager install ran, with its version, and every other writer (the
manager on every add, update bots) stays free.

## Layers

The harness value composes from an ordered stack of layers under the same
merge the SDK already speaks — later layers win by array surgery, one
algebra for every layer. Which slots exist and their order is the provider's
documented fact, carried as cited data on the provider face; the claude-code
face emulates the product's own precedence, with the user slot named and
reserved, ungoverned. Temper adopts the provider's ordering, not its
override semantics: the bounds here are this gate's own rules.

A layer is a locus fact, never a mechanism: a member of a kind whose locus is
local, or a lock the invocation joins. Two rules attach to anything
uncommitted — content review never saw — never to a layer's name. Check-side,
structurally: a local member (the read-side-only class, `representation.md`,
locus §) contributes no emit output, so the lock captures the committed
harness alone and committed bytes are layer-invariant by construction; its
rows derive at read time under the kind the committed lock declares — review
fixes what a local file may say, the gate reads what it says. And review is
the price of softening: the committed harness may soften anything, a reviewed
diff, while uncommitted content hardens without bound in every mode and
softens only through the shipped dial kind (`contract.md`, "clause"), whose
softening is inert in block mode — so a block-mode pass on any machine
implies the shared gate's pass.

A policy layer arrives with the invocation, as a lock: `check` joins the
declaration rows of locks its invocation names over this corpus — kinds
travel by name, the joined clauses range over the host's selections, and the
lock stays the one interchange. Whoever owns the invocation owns the top of
the stack; org authority is the org's pipeline definition. A joined lock that
fails admissibility fails the check, fail-closed. `check` announces every
active local member, every dialed clause, every joined lock, and the
harness root it resolved where that differs from the path it was given.
