# Built-ins — what ships

temper ships the built-in kinds of one harness, Claude Code, as ordinary SDK
values in a provider module — the `@dtmd/temper/claude-code` subpath, never
the root export. Every built-in is made with the same constructor any author
uses: ownership, not privilege. temper maintains them because the formats are
external and evolving — a skill's shape is the harness's truth, not the
author's to invent — and the author adopts them by import. Kind identity
travels by import, never by string: two providers are two modules, so
collision is impossible and no name-qualification scheme exists.

## The shipped kinds

Seven kinds ship. Five are file members:

- **skill** — its entry file carries YAML frontmatter over a body; identity
  from its directory's name; registers on both invocation channels. Its
  template names **supporting-doc**, the shipped kind of its bundled
  reference documents: fields-free, prose-only, channel-less, identity from
  the filename — the documented shape of a supporting file
  (code.claude.com/docs/en/skills, "Add supporting files", retrieved
  2026-07-16). Like `requirement`, it ships without joining this surface
  enumeration; an adopting corpus may override the template's child kind by
  admission where richer typing is wanted.
- **command** — the skill surface's legacy file placement (Claude Code
  merged commands into skills; code.claude.com/docs/en/skills, retrieved
  2026-07-07): a lone markdown file, the skill's field schema by import,
  identity from the file stem, the same two channels.
- **agent** — a subagent definition; identity from its frontmatter `name`
  field, never the filename; registers by description delegation
  (code.claude.com/docs/en/sub-agents, retrieved 2026-07-07).
- **rule** — a lone markdown file with an optional path scope; a rule with no
  scope registers unconditionally.
- **memory** — a `CLAUDE.md`-family file, loaded whole at launch, with no
  frontmatter: the entire file is prose.

Two are registration members — fields-only entries a manifest carries at a
collection address, never files of their own (`model/representation.md`,
"Reach"):

- **hook** — one handler registration under `settings.json`'s
  `hooks.<Event>`; its channel is the documented event.
- **mcp-server** — one connection under `.mcp.json`'s `mcpServers`; its
  channel is the connection.

A kind's registration names the set of documented channels a member reaches
the world over — user invocation and description trigger are channels, not
rivals — and the documented fields that modulate them per member are
ordinary declared fields. A declared field may also gate the member's
channels outright: a skill's path scope removes it from every channel — the
listing, model invocation, user invocation — until a matching file is in
play (code.claude.com/docs/en/skills, retrieved 2026-07-15; verified against
2.1.210). The gate is the field's documented semantics, carried with the
field, never a channel entry.

Each kind's format facts are external facts about the harness, cited at the
point of claim in the kind's own source.

A shipped kind's composed body admits corpus-declared member types by the
same admission any kind's does (`model/representation.md`, "nesting"): the
body a consumer's program composes may grain into their own embedded types,
and the projection stays the format the harness documents.

## The domain partition

Each shipped kind owns a semantic domain and an output posture, and the
partition is the harness's own documented guidance, not temper's invention
(code.claude.com/docs/en/features-overview, retrieved 2026-07-15):

- **memory** owns always-true facts; its output is ambient context, loaded
  at launch.
- **rule** owns scoped conventions; its output is context injected when a
  matching file is read.
- **skill** owns procedures; its output is an invoked procedure, loaded
  into the turn on activation. (**command** is the same domain at its
  legacy placement.)
- **agent** owns delegated work; its output is an isolated subagent run.
- **hook** owns zero-exception enforcement; its output is a deterministic
  action at its event — the harness's own docs draw the line: an
  instruction is advisory, a hook is a guarantee.
- **mcp-server** owns external capability; its output is a connection's
  tools.

One fact, one owner: the partition is what makes "which kind carries this"
decidable for an author, and content straddling two domains is two members.
The harness documents no mandatory baseline — adoption is trigger-driven,
per surface (same source) — so any prescribed composition is an authored
contract, never a vendor fact.

## The coverage bar

The vocabulary covers documented surface capability: every capability a
built-in surface documents as real — a user-invoked command, an event hook,
a connection — gets its registration value and, where it is an artifact,
its kind, cited to the documentation that settles it. The vocabulary grows
by documented capability, never by invention. Installed plugins are
registration members surfacing in the harness's manifests
(`model/representation.md`); the permission list is a derived aggregate
(`model/pipeline.md`).

## Default contracts

Each shipped kind carries a **default contract**: an exported, overridable
clause set adopted by the same import that adopts the kind. Adoption is a
choice, extension is a spread, overriding is array surgery in the language
the author already writes — no layering rules, no precedence table. A
project's clause array is the same type as the shipped one; the built-ins are
first-party instances of it, never a privileged form.

The stance every shipped default contract holds:

- **Strictest documented profile.** The clauses check the external format's
  strictest documented profile — the open spec and platform validation, even
  where a runtime is deliberately forgiving — so a member that passes travels
  everywhere the format is honored, not merely on the machine it was written
  on. Where the runtime diverges from the spec, the clause's guidance says so.
- **Undecidable properties are deliberately absent.** Whether a description
  triggers well, reads in the right voice, or names its skill aptly is
  semantic judgment — never a gate clause. What the clauses cannot decide,
  their guidance carries as teaching. A format that documents almost no
  contract gets an almost-empty default contract: the honest encoding, not a
  gap.

## The clauses live in code

Equal representation applies: the clause instances, their guidance, and their
external-fact citations live in code at the point of enforcement, and this
corpus never enumerates them. What it binds instead is the bar every shipped
clause must clear:

- **decidable** — unambiguously true or false of the surface, so a violation
  is always a true positive;
- **severity-declared** — the clause's author dials error versus advisory,
  never the tool;
- **guided** — the teaching prose rides the clause value itself, so it cannot
  dangle from the check it explains;
- **cited and dated** — the source is a doc URL plus retrieved date, carried
  as data, so when upstream docs move the update ritual is to walk the
  clauses and re-check their citations, never to re-derive from memory.

## Requirement

The requirement is itself a shipped kind — an embedded member whose template
and default cardinality clause `model/contract.md` owns.
