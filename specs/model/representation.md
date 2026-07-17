# Representation — member · kind · locus · nesting

Layer 1 of the model: a faithful, typed representation of the harness as a
forest of nested members, rooted at the harness itself. Representation is
ground level — the value is the contract over it (`contract.md`) — but
everything the contract can reach, this layer must model.

## member

One authored unit: an instance of a kind. A member carries

- an **identity**,
- typed **fields**, projected to frontmatter or structured config,
- **prose** — its authored words, copied verbatim, byte-for-byte,
- **edges** — its declared references (`contract.md`),
- and, when its kind nests, **nested members**.

Members are what `emit` projects into artifacts (`pipeline.md`).

## kind

The type of a member. A kind declares

- the **field schema**,
- the **content** — `file`: the body is one verbatim prose value (the
  default; what install hoists), or a **layout**: a declared template over
  the body's heading tree in exactly three primitives — prose (verbatim; a
  region may be an import, a declared reference resolving to a file's
  contents, fingerprinted and refusing when dangling), a field section (a
  heading whose span fills a named slot — intent among them), and a member
  collection (a heading whose child headings are each one member of a named
  kind; identity is the slugged heading, an explicit key survives
  retitling). A field section the kind marks as an edge field declares the
  member's edges — `satisfies` among them — and its entries are addresses.
  A layout admits no syntax beyond markdown's own and has one
  face — the reader: the document is the authored home, read, never
  regenerated. What does not fit the three primitives is two kinds, or it
  is prose. A layout's regions state what may appear, never what must: a
  document conforms with a region empty, so one kind serves a tree whose
  documents range from prose-only to fully membered, and any floor — a
  required section, a minimum member count — is a clause over the
  selection, dialable per corpus. A document's kind is declared by its
  position alone: two kinds never share a `governs` glob, because routing
  a document by its content would be mining.
- the **edge fields** — which fields are references, and to what,
- a **format** — a template literal rendering the member's values into its
  artifact: constant text around interpolated fields, prose, and child
  layers. The kind's format is the default; a member may dictate its own.
  A **file**-locus kind's format derives two one-way faces — the canonical
  writer for projections, the lenient reader for sources — never a round
  trip; admissible only when unambiguous (adjacent slots split by constant
  text neither can absorb; the prose slot terminal or fenced), holding no
  logic and no derived values, and declaring the leniency its source-reads
  forgive; a structured sublanguage is a slot naming a schema, never syntax
  spelled as constant text. An **embedded**-locus kind's format is
  writer-only and unconstrained — no admissibility bar — when its host is
  composed: the rendering is a projection, its facts declared (`pipeline.md`,
  "The lock"), never read back. Beside the member's own values, an embedded
  format may place a **closed, engine-derived set of facts about its edge
  fields' targets** — name, address, kind, and the target's projection path
  relative to the host member's own projection — as data the template
  selects, never authored at the instance and never fabricated: a rendered
  reference is true by construction, and instance prose never spells its
  target. A format that omits an edge its kind declares renders a contract
  the prose does not represent; that check is a clause. A member embedded
  in a layout document has
  no format of its own: it is read off the host's declared layout — source,
  never projection.
- and, when it nests, a **template** per inner layer: the child kind, plus
  the path pattern (relative to the parent's unit) when children are files.

A kind is data, never code; its extractor is composed from that data. Kind
identity travels by import, never by string. Built-in and user-declared kinds
are the same construct — ownership, not privilege (`../builtins.md`).

## locus

Where a member serializes. Three spellings:

- **file** — the member owns a file at a path its kind governs. A file
  locus may declare a commitment class of **local**: per-machine and
  uncommitted — the kind is declared and reviewed, its members' documents
  are not. A local locus is layout-only (the document is the governed
  source, read at check under the declared layout; emit writes nothing
  there) and its members' rows never enter the lock, deriving at read
  time instead (decision 0032),
- **embedded** — the member lives inside its parent's body, addressed per
  the parent's format, or
- **nested file** — the member owns a file whose path composes from its
  host's unit and the host template's path pattern (the template fact
  above): the pattern is the host's declared fact, the child kind governs
  no glob of its own, and position stays decidable — two kinds still never
  share a governs glob.

## nesting

A kind may template inner layers of members, to arbitrary depth; a nested
member is a full member with its own kind — one member type in the model;
nested content never lives in a parallel value shape. Nesting is **model
containment** and locus is **serialization**, and the two are orthogonal: a
skill's bundled reference documents are nested in the model yet own their
files; a hook is nested and embedded in `settings.json`; a spec invariant is
nested and embedded in its document. An embedded member's facts reach the
lock one of two declared ways: carried from the composing program — a
projection is never mined for them (`pipeline.md`, "Emit") — or read off a
layout host's document, whose declared layout is the typed surface they are
declared on. A prose **leaf** — one addressable authored string — is a
nested member at the finest grain. An embedded member type declares no
host: which types may compose a kind's body is the adopting corpus's
**admission** — a contract declaration over the host kind — so one type
means the same thing in every body that admits it, and a shipped kind's
composed body admits corpus-declared types by the same declaration
(`../builtins.md`).

## The root member

The harness itself is a member — the root of the forest. Its kind's template
names the top layers (`.claude/`, the memory files, settings), and the
contract attaches to it the way a contract attaches to any member. Harness-
wide declarations are its fields — the enforcement mode among them —
overridable per member. There is no container above the forest; it is
members all the way down.

## Reach

- Structured config trees (`settings.json`, `.mcp.json`, a plugin's
  manifests) are **manifests**: projections representing a controlled
  segment of their container member — its fields, its members' registration
  facts, and derived aggregates like the permission list. A registration
  member (a hook, an MCP server, an installed plugin) is a fields-only kind
  surfacing at its declared collection address; a small residue of genuinely
  unschematized keys remains as opaque fields, named as such.
- Claude Code's artifact levels — user, project, project-local, enterprise —
  are peer forests of one shape, merged at runtime by the surface per
  documented, cited per-kind precedence. temper governs the project forest;
  an ignored local file is by declaration not authored here; another level
  is another target path, never a model change.
- The engine is corpus-generic — any corpus of authored artifacts can be
  modeled as members and gated — but exactly one governed corpus ships: the
  harness. A second corpus is a feature, never a founding assumption.
