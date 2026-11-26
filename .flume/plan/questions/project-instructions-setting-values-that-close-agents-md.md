# Does a project-instructions value that loads no AGENTS.md close its channels?

`specs/builtins.md`, "The shipped kinds" (memory, 0113) states one value of the project-instructions setting: where it loads both files, neither absence gate applies. The memory docs (code.claude.com/docs/en/memory, "Choose which instruction files load", retrieved 2026-11-26) name four values of `pluginConfigs["cc-plugin-agents-md@builtin"].options.instructionFiles`: `claude-md-or-agents-md` (default), `claude-md-and-agents-md`, `claude-md`, and `managed-only`. Under `claude-md` or `managed-only`, the runtime would load no project `AGENTS.md`.

0101 (a built-in reads as the runtime reads) and 0103 (a finding is true for its reader) cover the class. The section does not state these values, though, so reach would call such an `AGENTS.md` loaded when its gate is open. Per 0105, a ruled behavior reaches code only through the spec.

Needed: a `builtins.md` ruling on whether those values shut every `AGENTS.md` gate, and whether `managed-only` also shuts the project `CLAUDE.md` files. PROJECT-INSTRUCTIONS-SETTING-OPENS-BOTH-AGENTS-MD-GATES models only the load-both value, and nothing rests on this question.
