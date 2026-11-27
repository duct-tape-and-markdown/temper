# PLAYBOOK-ATTACHMENTS: three attachments shipped, guard hook left out

Shipped `PlaybookInit.attachments` with `{ clauses, fence, tasks }` (`sdk/src/playbook.ts`).

- Clause: the model has one way to judge clauses over named members: a requirement's
  attached clauses over its opt-in selection. So the playbook publishes
  `playbook:<id>` on its rule's `requires`, and the rule, the skill and every topic
  satisfy it. The requirement exists only when clauses are attached. This does not rest
  on the open `playbook-satisfies-requirement-arcs` question: these edges are the
  shape's own wiring, not a consumer `satisfies` option. I did not run the engine round
  trip with `check` (an SDK-only tick, no binary built), so a `:` in a requirement name
  inside a dial label (`requirement.playbook:x.<pred>`) is untested on the Rust side.
- Fence: each harness-root-relative glob becomes the ask rule `Edit(/<glob>)` on the
  rule's `needs`.
- Task: each command as invoked becomes the exact rule `Bash(<command>)` on the skill's
  `needs`. A `*` or a shell separator is refused, because it would widen the rule or
  match nothing.

Break: `Capability` gains an optional `list`, `capability(p, { list })` takes it,
and `permissionUnion` plus `EmitResult.permissions` now return `{ allow, ask }`
instead of a flat allow array. A release migration note is owed.

Still open: the guard-hook attachment is not built. The entry says it waits on
question `playbook-guard-hook-command`, but no such file is in
`.flume/plan/questions/`. Plan should check whether that question was answered or
never filed.

Also: emit derives the permission lists off the seam, and no projection writes them
into `settings.json`. So "settings permissions.ask" exists today only as the
`EmitResult` value, the same as allow always has.
