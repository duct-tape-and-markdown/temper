# An unrepresented harness keeps its unclaimed findings and adds one script note when settings run commands (0103, 0104)

This answers the `unrepresented-script-coverage` question under class 0103 (a finding is true for its reader), using option (b), the plan's own recommendation. `specs/builtins.md`, "Tool", now states the rule. Unclaimed-entry findings stay per entry, so the worktrees gitignore remedy keeps working. When settings carry any hook or `statusLine` command, coverage adds one note: "scripts these commands run become `{ file }` tools once the harness is represented". It never counts or names scripts, since parsing commands is mining. Acceptance:
- **new:** an unrepresented harness whose settings hold a hook command draws the note;
- **new:** one with no hook or status-line command draws none;
- **kept:** `.claude/worktrees/` still draws its own finding with the gitignore advice.
