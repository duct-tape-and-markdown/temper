+++
provider = "agents-md"
governs = { root = ".", glob = "**/AGENTS.md" }
unit_shape = "file"
activation = { via = "always" }

[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "sections"

[[extraction]]
primitive = "placement"
+++

# The memory kind — built-in

The declared definition of the AGENTS.md memory kind: an `AGENTS.md` at the
repository root — "just standard Markdown ... the agent simply parses the text
you provide," with no required fields, sections, or frontmatter
(https://agents.md/, retrieved 2026-07-02). Identity is the file stem. Because
the format carries no frontmatter, this kind declares no `format`: the sole
vocabulary value `yaml-frontmatter` would assert a schema the standard
explicitly disclaims. Omitting it is accurate and precedented — the project's
spec kinds carry no `format` either.

Kind identity is qualified `agents-md.memory`, and here the `provider` is a
**standard, not a tool**: `agents-md` names the open format stewarded by the
Agentic AI Foundation under the Linux Foundation
(https://www.linuxfoundation.org/press/linux-foundation-announces-the-formation-of-the-agentic-ai-foundation,
retrieved 2026-07-02), the authority that defines the file every consuming tool
reads. The provider axis carries exactly this case (`specs/architecture/15-kinds.md`,
"a provider is a tool ... or a standard (`agents-md`, `agent-skills`)"):
`agents-md.memory` and `claude-code.memory` are two kinds because two authorities
define two files, not one file wearing two names.

This kind **deliberately shares the bare name `memory`** with `claude-code.memory`.
A bare `memory` reference therefore no longer resolves uniquely: an assembly
binding it collides with a load error naming both qualified candidates
(`KindError::AmbiguousKind`) — bind `agents-md.memory` explicitly. `unit_shape =
"file"` and `activation = { via = "always" }` are honest, typed metadata (a lone
root file, read into every session) but inert on the custom-import path today,
carried for documentation value like the built-in kinds' activation (`src/kind.rs`,
"Inert until").

The extraction is markdown structure only — line budget, headings, sections,
placement. `governs` captures **every `AGENTS.md` in the repository**
(`root = "."`, `glob = "**/AGENTS.md"`) — the standard's defining behavior is
nested files with nearest-wins precedence ("agents automatically read the
nearest file in the directory tree," https://agents.md/, retrieved 2026-07-02
— the main OpenAI repo ships 88 of them). Each nested file is its own member
with a placement-folded id, and discovery honors the repository's ignore
rules — a dependency tree's `AGENTS.md` files are not this project's members
(`specs/architecture/20-surface.md`, "discovery respects ignore rules"). Provider-specific *dialect* is deliberately
excluded from this base-standard kind: Codex's `AGENTS.override.md` precedence
file and 32 KiB `project_doc_max_bytes` budget
(https://developers.openai.com/codex/guides/agents-md, retrieved 2026-07-02),
and Gemini CLI's `@path` imports and opt-in `AGENTS.md` aliasing via
`context.fileName` (https://geminicli.com/docs/cli/gemini-md/, retrieved
2026-07-02), are one tool's *reading* of the file, not the format's contract — a
`codex.*` or `gemini.*` kind's concern, not this one's.

Built-in means **temper-sourced, not privileged** (`specs/architecture/15-kinds.md`):
embedded exactly as a custom kind, differing only in source. `build.rs` embeds it
and the engine parses and composes its extraction through the generic built-in
path (`src/builtin_kind.rs`); what is not yet wired is discovery — `import`'s
built-in scan still names only `skill` and `rule` (`src/import.rs`), so a real
harness's `AGENTS.md` is not imported off this locus until that scan generalizes.
Embedded beside `claude-code.memory`, it also trips the eager bare-name
resolution in `builtin_kind::definitions()`: the collision this wave exercises is
real engine behavior, and scoping it to assemblies that actually reference bare
`memory` is the open work.
