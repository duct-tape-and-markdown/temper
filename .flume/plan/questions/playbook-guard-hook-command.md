# What command does a pattern-emitted hook with path-conditional behavior run?

`specs/distribution.md`, "What ships — three channels": a playbook attachment may be "a guard hook that points an edit on the area's paths at its rule, or refuses it naming the rule". A `PreToolUse` matcher selects by tool name only, so the path test has to live in the command, which reads the tool input on stdin. The section does not say what that command is:

1. a temper verb (`guard` grown a point-at/refuse mode, fed the area's paths and rule from the lock), or
2. an emitted script, a `tool` member the playbook composes.

Both are consistent with the section and with the gate-hook precedent (`adoption.md`: a gate hook states the command it runs, a temper bin unless declared). Option 1 keeps behavior in the engine and the area in the lock; option 2 needs no new verb surface but ships logic outside `check`. Recommendation: 1, since the guard already reads the edit's path against the lock. A ruling is a `distribution.md` edit. Blocks only the guard-hook half of PLAYBOOK-ATTACHMENTS.
