# Is a `CLAUDE.md` inside a nested skill directory memory or a supporting-doc?

Driver: `specs/builtins.md`, intro (where two loaders' reach meets, the documented owner keeps the path and the other kind excludes it). Memory (`**/CLAUDE.md`) excludes the root `.claude/skills`, so a root skill directory's `CLAUDE.md` is a supporting-doc. A nested skill's directory (`apps/web/.claude/skills/x/CLAUDE.md`) is not excluded, so memory claims it. The same file is typed differently depending on depth. `specs/model/representation.md`, "locus", makes an exclusion a path, so no exclusion can reach every nested `.claude/skills`.

The answers: let the exclusion grammar take a `**/`-led path; carve host templates' claims out of memory by rule; or accept that a nested skill's `CLAUDE.md` is memory (does Claude Code load it as memory there? UNVERIFIED). Seen by `SKILL-CLAIMS-SUBDIRECTORY-SKILLS`.
