---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
paths: [".claude/settings.json"]
---
## Requirements

- `governance`: the harness maintains itself: the gate placements are wired, and growing the harness is itself a stated procedure
  Satisfied by `hook:PostToolUse:Bash`, `hook:PreToolUse:Write|Edit|MultiEdit`, `hook:SessionStart`, `skill:grow-harness`.