# What leniency does the frontmatter reader declare for YAML Claude Code accepts?

Driver: a real-harness probe (0.0.25). It produced `member.load-fault` ×55 (48 commands, 7 skills), each with an unquoted `description:` containing `": "`. The fault cascades: 131 of 168 `supporting-doc.degree` findings belong to those skills' docs.

The sources disagree on what the runtime does:
- Lab (Claude Code 2.1.x, `claude -p --setting-sources project`, 2026-11-25): a skill of that shape is listed with its full description, verbatim.
- Docs (code.claude.com/docs/en/skills, retrieved 2026-11-25, secondhand summary): with malformed YAML, a skill "still loads with no fields set".
- Docs (code.claude.com/docs/en/sub-agents): an agent whose YAML "doesn't parse" is skipped.
- Docs (output-styles): a style "still loads under its file name with no fields set".

`specs/model/representation.md`, "kind", says the lenient reader declares the leniency its reads forgive. `builtins.md` says format facts are documented truth. Calling a skill or command that loads a load fault is false either way. But the declared leniency is underdetermined, and so is the acceptance ("description reads as the member"). It could be (a) the lab's rest-of-line scalar, (b) the docs' load-with-no-fields, or (c) per kind, with agents still faulted. Also open: whether a non-portable shape is an advisory. Decide which source binds before an entry is filed.
