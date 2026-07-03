# Plan state

- **Phase:** reconcile. HEAD 459f140.
- **Last shipped:** CUSTOM-UNIT-REPRESENTATION-CARRY + READ-VERBS-PUBLISHED-DEMANDS
  (build 320444a/74409e0, chore 459f140) — the two dogfood fixes landed:
  `import_custom_unit` now carries the hand-authored `[requirement.*]`/`[satisfies.*]`
  trace forward on re-import, and the read family ranges over the composed
  requirement namespace (not the assembly roster only).
- **This tick:** reconciled the 5 remaining entries clean — every cited line
  verified on disk (kind.rs:30 `["skill","rule"]`, :545 Primitive still
  field/headings/sections/line_count/placement, :588 flat `frontmatter.get`,
  :1171 parse_primitive; builtin.rs:37/44). None shipped: no
  `kinds/claude-code/{memory,agent}`, no `packages/{memory,agent}.anthropic`, no
  CONTRIBUTING.md/SECURITY.md on disk. Inbox empty — nothing to drain. No new gap
  to file; the residuals (join→graph unification; the OPEN strategic forks) are
  human-to-settle, not fileable.
- **Session-start note (accepted, not queued):** the 17 `requirement.dangling`
  findings in the banner are a **stale installed binary** (`~/.cargo/bin/temper`
  predates the member-published-requirements union) — the freshly-built binary's
  `check`/`session-start` are clean. Fix is operational (`cargo install --path .`),
  not spec/build work.
- **In flight / pickable:** none `open`. Parked: MEMORY-KIND, PACKAGING-CHANNELS,
  COMMUNITY-DOCS. Deferred: EXTRACTION-VOCAB-GAPS, AGENT-KIND. All await human
  action (author curated KIND.md/PACKAGE.md; set release creds; widen build's
  root-docs fence; or a consumer kind). The OPEN forks (edge-representation-unify,
  default-assembly-as-data, eval-capability, multi-harness-projection,
  kind-harness-axis directory shape, reachability residuals) stay human-to-settle.

Plan continues: no — queue reconciled clean, inbox empty, no pickable `open`
work remains. The whole queue is blocked on human action; nothing for build to
drain until a human un-parks an entry or settles a fork.
