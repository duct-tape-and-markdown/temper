# Plan state

- Spec derived through: edb6ddc4 — contract.md re-cut (7fb49108/c9fbbca8/edb6ddc4, 171→150 lines per 0047's Consequences) routed: pure editorial trim, no ratified-intent change, no entries filed.
- Audited through: 41e8847a — window f60c493e..41e8847a clean (see commit body).
- Residue swept through: 41e8847a — same window, no findings.
- Posture swept through: 173cdf54 mid-rotation — covered src/telemetry.rs+src/tap.rs (filed TELEMETRY-EVENT-LABEL-CONSOLIDATE), sdk/src/declarations.ts+sdk/src/prose.ts (filed TAP-HOOK-KIND-NAME-BY-IMPORT); frontier remaining: sdk/src/dial.ts, src/read.rs, tests/prose_include.rs, tests/read_verbs.rs.
- This tick: posture sweep, declarations.ts+prose.ts neighborhood (declarations.ts imports prose.ts, both frontier — one neighborhood covers both). prose.ts: clean, no lens finding. declarations.ts: kind.ts:261 rules a KindDefinition's identity travels by import, never by string; the file's own idiom bears this out everywhere else it needs a bare kind name (templatesFor's `template.kind.key`, admissionsByHost's `host.key`/`child.key`, compileDeclarations's `binding.kind.key`) — but tapHookRows (L790) hand-spells `kind: "hook"` instead of importing builtins.ts's `hook` (name: "hook", L407-408) and reading `hook.key`. Filed TAP-HOOK-KIND-NAME-BY-IMPORT (open, mechanical). No other lens finding in either file (cohesion, dead plumbing, export-consumer checked — declarations.ts's exports all resolve to real callers in emit.ts/tests).
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 3. Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — posture sweep continues (frontier: sdk/src/dial.ts, src/read.rs, tests/prose_include.rs, tests/read_verbs.rs) once TAP-HOOK-KIND-NAME-BY-IMPORT ships.
