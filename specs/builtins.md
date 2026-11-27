# Built-ins — what ships

temper ships the built-in kinds of one harness, Claude Code, as ordinary SDK
values in a provider module — the `@dtmd/temper/claude-code` subpath, never
the root export. Every built-in is made with the same constructor any author
uses: ownership, not privilege. temper maintains them because the formats
are external and evolving — a skill's shape is the harness's truth, not the
author's to invent — and the author adopts them by import. A built-in's
locus is the harness's too: it claims exactly the paths the harness loads as
that kind, cited, and where two loaders' documented reach meets, the kind
whose loader the documentation names owns the path and the other excludes
it. A built-in reads its format as the runtime does: the documented behavior
binds, a cited lab observation binds where the docs are silent, and
otherwise the literal bytes bind, with the uncertainty named in guidance. A
document the runtime loads is never a load fault; one it skips is. A value
the runtime rejects is never silence either: a settings key of the wrong
JSON type is skipped alone while the rest of its file applies, and an MCP
config whose server collection is not an object is rejected (probed against
Claude Code 2.1.332, 2026-11-27; the docs are silent), so each draws a
finding naming the key and the type it carries, and a regenerate keeps the
value it never judged. A skill, command or output style whose frontmatter
does not parse loads with no fields set, as the docs say, and draws one
finding naming the non-portable bytes and the fields read as unset; an agent
that does not parse is skipped, so it is a load fault. A relocated built-in
keeps each exclusion its new template still reaches, since moving one
loader's reach moves no other's. Where claims overlap, the most specific
documented claim holds the document, for built-in and custom kinds alike: a
subkind over its base, and a host's template over an any-depth reach (a
`CLAUDE.md` inside any skill's directory is the skill's, not memory's). The
lock records the resolution, and every verb reads it there. An author may
add exclusions to a built-in through its relocation. That is how a collision
with a built-in is answered, and how a fixture tree holding harness files is
left ungoverned. Kind identity travels by import, never by string, and no
name-qualification scheme exists: one harness holds one kind per name, and
two kinds of one name in play are refused. The exception is an extension
under the built-in's own name, a relocation, which replaces the built-in
there.

## The shipped kinds

Twenty kinds ship: eleven file members, four registration members, the
embedded `handler` and `tool`, the nested-file `supporting-doc`, and
temper's own `dial` and `requirement`. The file members:

- **skill** — its entry file carries YAML frontmatter over a body; identity
  from its directory's name; registers on both invocation channels. It
  claims `**/.claude/skills/*/SKILL.md`: a skill nested under a
  subdirectory's own `.claude/skills/` loads once Claude works there,
  invoked `/apps/web:deploy` (code.claude.com/docs/en/skills, "Load skills
  in monorepos and subdirectories", retrieved 2026-11-25). A nested skill's
  name is prefixed by its subdirectory's segments, each joined with `:`
  (`apps:web:deploy`). A skill's own `paths` gate its listing, never its
  body: the skill joins the listing once a matching file is read, and loads
  only when invoked. A supporting document read in a session returns
  verbatim after compaction, re-attached beside the summary (probed against
  Claude Code 2.1.332, 2026-11-26; the docs are silent). The invocation
  spelling is rendered from the placement, never the identity, and the
  frontmatter `name` is the leaf (`deploy`), so the directory and the name
  still match. Its template names **supporting-doc**, the shipped kind of
  its bundled reference documents: fields-free, prose-only, channel-less,
  identity from the filename — the documented shape of a supporting file
  (code.claude.com/docs/en/skills, "Add supporting files", retrieved
  2026-07-16). It ships without joining this surface enumeration; an
  adopting corpus types it further by extending the host, whose template
  then names a subkind (`model/representation.md`, "kind"). The template
  claims the directory's markdown documents — the honest subset the
  prose-only kind can hold; supporting files of other types remain unmodeled
  and are named as such. A supporting doc no typed reference reaches is the
  finding, since a prose link breaks silently; on an unrepresented harness,
  where no edge can be declared, that collapses to one coverage line.
- **command** — the skill surface's legacy file placement (Claude Code
  merged commands into skills; code.claude.com/docs/en/skills, retrieved
  2026-07-07): a lone markdown file, the skill's field schema by import,
  identity from the file stem. All command frontmatter is optional
  (code.claude.com/docs/en/slash-commands, retrieved 2026-07-22), so the
  command kind requires no field — the forgiving legacy placement; the strict
  name/description profile stays the skill kind's, never the command's. The
  same two channels. It claims `.claude/commands/**/*.md`. A command in a
  subdirectory is named by its path under `commands/`, each `/` a `:`
  (`.claude/commands/frontend/component.md` is `/frontend:component`,
  code.claude.com/docs/en/skills, retrieved 2026-11-25), and that is its
  identity: `command:frontend:component`.
- **agent** — a subagent definition; identity from its frontmatter `name`
  field, never the filename; registers by description delegation
  (code.claude.com/docs/en/sub-agents, retrieved 2026-07-07). It claims
  `.claude/agents/**/*.md`: "Claude Code scans `.claude/agents/` …
  recursively, so you can organize definitions into subfolders"
  (code.claude.com/docs/en/sub-agents, retrieved 2026-11-25). A `name` with
  a `:` anywhere or a leading `-` is refused, because Claude Code skips that
  file (code.claude.com/docs/en/sub-agents, retrieved 2026-11-25).
- **output-style** — a markdown file under `.claude/output-styles/`, its
  frontmatter optional (`name`, `description`, `keep-coding-instructions`,
  `force-for-plugin`) and its body the instructions; identity from `name`,
  else the file stem. Settings select it by exact name with `outputStyle`,
  an edge from both `settings` and `settings-local`
  (code.claude.com/docs/en/output-styles, retrieved 2026-11-25). It loads
  from every `.claude/output-styles/` between the working directory and the
  repository root. A nested style's name is prefixed with its subdirectory's
  segments, so a root and a nested style that share a frontmatter `name` are
  two members, and the nested one draws an advisory that it shadows the root
  style in sessions started at or below its directory. A settings file's
  `outputStyle` resolves against the styles at that file's own level.
- **rule** — a lone markdown file with an optional path scope; a rule with
  no scope registers unconditionally. It claims `**/.claude/rules/**`: a
  subdirectory's own rules load on demand once Claude works there, named by
  the nested grammar (code.claude.com/docs/en/memory, "Claude ignores the
  end of my subdirectory CLAUDE.md", retrieved 2026-11-26). A scoped rule
  loads when a matching file is read, never on a search hit: a Grep or Glob
  match on its paths loads nothing (probed against Claude Code 2.1.332,
  2026-11-26; the docs are silent).
- **memory** — a `CLAUDE.md`-family file, loaded whole at launch, with no
  frontmatter: the entire file is prose. `AGENTS.md` and `.claude/AGENTS.md`
  are memory members by position too, and their registration is the
  runtime's condition (code.claude.com/docs/en/memory, retrieved
  2026-11-26): at launch one loads only where no `CLAUDE.md` sits in the
  working directory or above, and a subdirectory's loads on demand only
  where that subdirectory has none of the `CLAUDE.md` files of its own. One
  a `CLAUDE.md` shadows and does not import is never loaded, and
  reachability says so. Which instruction files load is a setting the
  runtime reads only from outside the harness (user, `--settings` and
  managed settings, never project or local), so the gates are judged at its
  documented default and a finding on a gated `AGENTS.md` names that
  assumption. A nested one is named by the same grammar as a nested skill,
  its subdirectory's segments joined with `:` (`memory:apps:web:CLAUDE`), so
  emit places it where it was found.
- **plugin-manifest** — the pack's identity at
  `.claude-plugin/plugin.json`, a JSON document; identity from its `name`
  field, the one required field (kebab-case). Its contract mirrors the
  strictest documented profile — `claude plugin validate --strict` — so a
  passing manifest travels everywhere the format is honored
  (code.claude.com/docs/en/plugins-reference, retrieved 2026-07-16).
- **marketplace** — the distribution catalog at
  `.claude-plugin/marketplace.json`, a JSON document; identity from `name`
  (kebab-case, checked against the documented reserved-names deny list);
  `owner.name` required; each `plugins[]` entry names `name` plus `source`
  (the documented union: relative path, `github`, `url`, `git-subdir`,
  `npm`) (code.claude.com/docs/en/plugin-marketplaces, retrieved
  2026-07-16).

- **settings-local** — the machine's own settings overlay at
  `.claude/settings.local.json`, a JSON document at the **local**
  commitment class: per-machine, uncommitted, read in place and gated,
  never emitted; singleton identity from its
  fixed documented path. Documented keys are typed, the unschematized
  residue stays opaque fields named as such — and it is fields, not
  members: a locally-registered hook or enablement's member-hood and
  reach stay unmodeled and named as such, the installed-plugin cache
  precedent (code.claude.com/docs/en/settings, retrieved 2026-07-16).

- **settings** — the committed `.claude/settings.json` as a whole, a JSON
  document at the committed commitment class, singleton identity from its
  documented path. Documented keys are typed, the unschematized residue
  stays opaque and named; it is the container of the three registration
  collection addresses, which keep their own kinds, and as a member it
  carries the byte fingerprint that makes a hand edit to any part of the
  file `config_stale`: every key is authored in the program, the file
  renders whole (code.claude.com/docs/en/settings, retrieved 2026-09-22).

- **mcp** — the committed `.mcp.json` as a whole, a JSON document at the
  committed commitment class, singleton identity from its documented path.
  `mcpServers`, its one documented key, is the **mcp-server** collection
  address; any other key is residue, opaque and named, and the member's
  byte fingerprint makes a hand edit to any part of the file drift
  (code.claude.com/docs/en/mcp, retrieved 2026-09-29).

The two manifest kinds, `settings`, `mcp`, and `settings-local` sit outside the
domain partition below: distribution metadata and machine configuration,
never authored session content.

Four are registration members — entries a manifest carries at a
collection address, never files of their own (`model/representation.md`,
"Reach"):

- **hook** — one matcher group under `settings.json`'s `hooks.<Event>`;
  its name is the event, then `:` and the matcher's authored bytes when it
  has one (`hook:PostToolUse:Edit|Write`), so two members with one
  identity are a malformed lock; its channel is the documented event. Its
  template names **handler**, the embedded kind of each entry in the
  group's `hooks` array, keyed by position (`…/handler/0`) and carrying
  the handler-level clauses. Matching handlers run in parallel, so the
  position carries no runtime meaning (code.claude.com/docs/en/hooks,
  retrieved 2026-09-29). Like `supporting-doc`, `handler` ships without
  joining this enumeration.
- **mcp-server** — one connection under `.mcp.json`'s `mcpServers`; its
  channel is the connection. An entry with no `type` reads as `stdio`, as
  Claude Code reads it, so a `url`-only entry owes the stdio `command`.
- **installed-plugin** — one enablement entry under `settings.json`'s
  `enabledPlugins`; its channel is the enablement entry itself. The members
  a plugin contributes live in the plugin cache, outside the corpus; their
  reach is unmodeled and named as such — the honest subset. The marketplace
  half of its `plugin@marketplace` key is a declared edge to the
  **known-marketplace** member it names.
- **known-marketplace** — one consumer-side registry entry under
  `settings.json`'s `extraKnownMarketplaces`: name-keyed, carrying the
  documented `source` union and `autoUpdate`; its channel is the registry
  entry itself. Distinct from the publisher-side
  **marketplace** catalog above — two documents, two owners
  (code.claude.com/docs/en/plugin-marketplaces, retrieved 2026-07-17; the
  product stores these per-user in `known_marketplaces.json`).

A kind's registration names the set of documented channels a member reaches
the world over — user invocation and description trigger are channels, not
rivals — and the documented fields that modulate them per member are
ordinary declared fields. A declared field may also gate the member's
channels outright: a skill's path scope removes it from every channel — the
listing, model invocation, user invocation — until a file tool (read, edit,
write) touches a matching path (code.claude.com/docs/en/skills, retrieved
2026-07-15; verified against 2.1.210). A shell read of the same path, or a
search tool's, opens no path gate, a skill's or a rule's (probed on 2.1.281,
2026-09-24), so a reach judgment over a path gate states that it cannot see
reads made any other way. The gate is the field's documented semantics,
carried with the field, never a channel entry. A gate may also be a
document's absence: an `AGENTS.md` opens its channels only where the
`CLAUDE.md` the runtime would load instead is absent, a gate carried with
memory's `AGENTS.md` templates and judged by reach. A finding about a gated
document says whether its gate is open, never that the runtime loads it when
the gate is shut.

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
  file tool touches a matching path.
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
built-in surface documents as real — a user-invocable command, an event
hook, a connection — gets its registration value and, where it is an
artifact, its kind, cited to the documentation that settles it. The
vocabulary grows by documented capability, never by invention. Installed
plugins are registration members surfacing in the harness's manifests
(`model/representation.md`); the permission list is a derived aggregate
(`model/pipeline.md`).

## Default contracts

Each shipped kind carries a **default contract**: an exported, overridable
clause set adopted by the same import that adopts the kind. Adoption is a
choice, extension is a spread, overriding is array surgery in the language
the author already writes — no layering rules, no precedence table. A
project's clause array is the same type as the shipped one; the built-ins are
first-party instances of it, never a privileged form. An empty array is a
declared contract with no clauses: the default applies only where nothing
is declared.

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
- **A documented, decidable rule the vocabulary cannot yet spell is a
  hold**: named in the contract's header at the point of enforcement with
  the widening that closes it, keyed in the plan ledger, test-pinned where
  load-bearing — never silent, never permanent-by-default.

**Context cost is decidable, and the shipped contracts judge it.** What each
surface costs and when it loads is the runtime's documented fact
(code.claude.com/docs/en/skills and /memory, retrieved 2026-11-26): every
model-invocable skill's description rides every request, an entry is cut at
1,536 characters, the listing holds about 1% of the window with the
least-used descriptions dropped first, and an on-demand rule past the
per-file limit is truncated. So the advisories judge the harness's one
listing (every model-invocable skill and command, bound at the root, since
one budget spans both kinds) against that budget; every file the runtime
cuts at the per-file limit, a subdirectory's `CLAUDE.md`, `CLAUDE.local.md`,
their imports and its own `.claude/rules/` among them, against that limit;
and a rule's globs against its skill's own `paths` where both trigger one
area. `explain` states, per member, what loads it, when, and roughly what it
costs. A load moment is a registration fact: a subdirectory's memory
registers on a read beneath its directory, and a skill's own `paths`
register its listing on a read of a match, so `explain` and the advisories
read one fact.

**A string the runtime matches against a target is judged only where a miss
is decidable without the author's intent.** No default clause judges whether
a permission rule reaches the file its author meant, or whether an agent's
name overrides the built-in its author meant; `explain` states how each
permission path pattern resolves and names an agent that shares a built-in
agent's name, so the author sees what the runtime will match. An allow
rule's match against the command a member runs correlates two members, which
is the open question of cross-source correlation.

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

## Tool

The **tool** is an embedded shipped kind: the one home of a command the
harness runs (`model/authoring.md`). It registers nothing. Gate hooks, taps,
the typecheck and script verifiers reach it through edges, and its default
contract is one advisory `degree` clause, incoming at least one, so a tool
nothing runs is reported. Its default is the temper package's bin, rendered
as the anchored `.bin` path; an unrepresented harness keeps `temper` on the
PATH. An unrepresented harness declares no tools, so it keeps each unclaimed
entry's own finding (its remedy may work there), and where settings carry a
hook or status-line command it adds one note that scripts become `{ file }`
tools once represented. That note counts nothing it cannot see.
