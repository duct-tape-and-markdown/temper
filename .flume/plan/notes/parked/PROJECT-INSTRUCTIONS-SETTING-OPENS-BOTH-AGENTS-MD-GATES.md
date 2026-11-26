# Parked: the load-both setting is ignored in the settings temper reads

The entry assumes harness settings (`.claude/settings.json`,
`.claude/settings.local.json`) can declare
`pluginConfigs["cc-plugin-agents-md@builtin"].options.instructionFiles =
"claude-md-and-agents-md"`. The memory docs say otherwise:
"Claude Code reads the entry from `~/.claude/settings.json`, a `--settings`
file, or managed settings, and ignores it in project and local settings
files" (code.claude.com/docs/en/memory, "Choose which instruction files
load", retrieved 2026-11-26). Before v2.1.285 the
plugin ID was `agents-md@builtin`.

So no settings file inside the harness can open the gates. Building the entry
as written would open AGENTS.md gates on a key Claude Code ignores, and report
a shadowed AGENTS.md as reachable when the runtime never loads it. That is
exactly the false "loads" claim the spec's gated-document sentence forbids.

The spec text ("reading the project-instructions setting where settings
declare it", builtins.md "The shipped kinds") needs a human ruling. Options:
(a) the setting is out of the corpus (user/managed scope). Reach stays
default-gated and the coverage line names that the user-scope setting is
unmodeled. (b) A project/local declaration of the key is dead configuration,
so the settings kinds draw an advisory saying Claude Code ignores it there.
(c) Both. I recommend (c): (a) keeps reach honest, and (b) catches the adopter
who writes the key into `.claude/settings.json` expecting it to work.

The other documented values (`claude-md-or-agents-md` default, `claude-md`,
`managed-only`) bear on question
project-instructions-setting-values-that-close-agents-md. They live in the
same out-of-corpus scope.

Tried: an `absence_shutter(harness_root)` in gate.rs reading both settings
files, threaded through absence_gated, committed_member_sites and install.rs's
two absence_gate callers. Built clean; reverted. It is the right
seam if a ruling ever brings an in-corpus source (e.g. a declared dial).
