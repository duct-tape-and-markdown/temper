# Representation — member · kind · layout · locus · nesting

Layer 1 of the model: the harness as a typed forest of nested members,
rooted at the harness itself. The value is the contract over it
(`contract.md`), but everything the contract can reach, this layer models.

## member

One authored unit: an instance of a kind. A member carries

- an **identity**, spelled as an **address**: `<kind>:<name>` for a
  top-level member, `<host-address>/<kind>/<key>` for a nested one, as deep
  as the model nests, and `/<leaf>` beneath it; every name, key and leaf is
  one segment (a leaf's child path joins with `.`), so a member address has
  an odd segment count and a leaf's an even one — one grammar every row
  spells and every verb accepts; resolution is total, and coincident
  addresses are a malformed lock,
- typed **fields**, projected to frontmatter or structured config,
- **prose** — its authored words, copied verbatim, byte-for-byte,
- **edges** — its declared references (`contract.md`),
- and, when its kind nests, **nested members**.

## kind

The type of a member. A kind declares

- the **field schema**,
- the **content** — `file`: the body is one verbatim prose value (the
  default; what install hoists), or a **layout** (below),
- the **edge fields** — which fields are references, and to what,
- a **format** — a template literal rendering the member's values into its
  artifact: constant text around interpolated fields, prose, and child
  layers. The kind's format is the default, a member's own overriding it. A
  **file**-locus kind's format derives two one-way faces — the canonical
  writer for projections, the lenient reader for sources — never a round
  trip; admissible only when unambiguous (adjacent slots split by constant
  text neither can absorb; the prose slot terminal or fenced), holding no
  logic and no derived values, and declaring the leniency its reads forgive;
  a structured sublanguage is a slot naming a schema. An **embedded**-locus
  kind's format, under a composed host, is writer-only with no admissibility
  bar — a projection whose facts are declared (`pipeline.md`, "The lock"),
  never read back. It may place a **closed, engine-derived set of facts
  about its edge fields' targets** — name, address, kind, projection path
  relative to the host's; so a rendered reference is true by construction. A
  format omitting a declared edge is a clause's finding.
- and, when it nests, a **template** per inner layer: the child kind, plus
  the path pattern (relative to the parent's unit) when children are files.

A kind is data, never code, its extractor composed from that data; its
identity travels by import, never by string. Built-in and user-declared
kinds are the same construct — ownership, not privilege (`../builtins.md`) —
and a kind may **extend** another. A subkind keeps its base's facts, adds
fields, edge fields and counsel, and governs a locus within its base's;
every selection of the base binds its document, and its fields live where
the base's format admits them, never as an undocumented key. Keeping the
base's name it is a relocation, and any corpus kind of a built-in's name
replaces the built-in there.

## layout

A kind's declared template over its body's heading tree. It structures the
body and nothing else: the format's other slots, frontmatter among them,
read as on a `file`-content kind, and the document's identity follows its
kind's locus as any file member's does. Exactly three primitives:

- **prose** — a verbatim span; a region may be an import, a declared
  reference resolving to a file's contents, fingerprinted and refusing
  when dangling,
- a **field section** — a heading whose span fills a named slot; a slot the
  kind marks as an edge field declares the member's edges (`satisfies`
  among them), its entries addresses,
- a **member collection** — a heading whose child headings are each one
  member of a named kind, identity the slugged heading, so a retitle
  without an explicit key is a delete and a create. An entry is a member
  in every respect — selected, judged, addressed as any of its kind — read
  off the host's layout, source, never projection, with no format of its own.

A lone leading heading is the document's title: its span is the document's
own `prose`, and the regions bind to its child headings. A layout admits no
syntax beyond markdown's own and has one face, the reader: the document is
the authored home, read, never regenerated. What does not fit the three
primitives is two kinds, or it is prose. Regions state what may appear,
never what must: a document conforms with a region empty, and any floor —
a required section, a minimum member count — is a dialable clause.

## locus

Where a member serializes. Three spellings:

- **file** — the member owns a file at a path its kind governs, a
  **template**: literal segments with at most one `*`, optionally under a
  leading `**/`, less the paths the kind declares **excluded**, a directory
  excluding what lies beneath it. Any other pattern syntax refuses at load;
  names narrow by clause, never by pattern. A document's kind is declared by
  its position alone, so two kinds never govern one document (a subkind's is
  its base's), judged per document and answered by excluding it from the
  kind that does not own it; a registration kind's starless locus names its
  manifest and governs no document. A file locus may declare a commitment
  class of **local**: per-machine and uncommitted, the kind reviewed and its
  documents not. A local member is read-side only — the governed source,
  read at check under its kind's format, never an emit input or target
  (`pipeline.md`) — and its rows derive at read time, never in the lock,
- **embedded** — the member lives inside its parent's body, loading through
  its host, never on its own, so it registers nothing — a non-empty
  registration is a well-formedness fault, or
- **nested file** — the member's path composes from its host's unit and
  the host template's path pattern; the child kind governs no path itself.

## nesting

A kind may template inner layers of members, to arbitrary depth; a nested
member is a full member with its own kind — one member type, never a
parallel value shape. Nesting is **model containment** and locus is
**serialization**, and the two are orthogonal: a skill's bundled reference
documents are nested in the model yet own their files; a spec invariant is
nested and embedded in its document. An embedded member's facts reach the
lock one of two declared ways: carried from the composing program — a
projection is never mined for them (`pipeline.md`, "Emit") — or read off a
layout host's document under its declared layout. A prose **leaf** — one
addressable authored string — is a nested member at the finest grain; every
nested member's own span is its reserved `prose` leaf, cut at the first
child heading. An embedded member type declares no host: which types may
compose a kind's body is the adopting corpus's **admission** — a contract
declaration over the host kind — so one type means the same in every body
that admits it, a shipped kind's included (`../builtins.md`).

## The root member

The harness itself is a member — the root of the forest. Its kind's template
names the top layers (`.claude/`, the memory files, settings), the contract
attaches to it as to any member, and harness-wide declarations are its
fields — the enforcement mode among them — overridable per member.

## Reach

- Structured config trees (`settings.json`, `.mcp.json`, a plugin's
  manifests) are **manifests**: projections of a controlled segment of their
  container member — its fields, its members' registration facts, derived
  aggregates like the permission list. A registration member (a hook, an MCP
  server) is a file-less kind at a collection address within the manifest
  its locus names; unschematized keys remain as opaque fields, named.
- Claude Code's artifact levels — user, project, project-local, enterprise —
  are peer forests of one shape, merged at runtime by the surface per
  documented, cited per-kind precedence. temper governs the project forest,
  including the local-locus members its committed kinds declare
  (`adoption.md`, the discovery override); another level is another target
  path, never a model change.
- The engine is corpus-generic — any corpus of authored artifacts can be
  modeled as members and gated — but exactly one governed corpus ships: the
  harness. A second corpus is a feature, never a founding assumption.
