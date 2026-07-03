# Plan state

- **Phase:** reconcile. HEAD c65c2ed.
- **Last shipped:** EMBED-NESTED-WALK (build 1f3d761, chore c65c2ed) — build.rs's
  kind embed walk tolerates the nested `kinds/<provider>/<name>/` layout
  (`collect_qualified`, dormant over today's flat tree) beside the flat one, and
  `builtin_kind` lookups (`source`/`definition`/`definitions`) route bare → unique
  via `CustomKind::resolve_bare`, a two-provider collision an `AmbiguousKind` load
  error. Verified on disk.
- **This tick:** drained shipped EMBED-NESTED-WALK (build already removed it from
  pending). Refreshed BINDING-QUALIFY's gate — dropped the now-shipped
  EMBED-NESTED-WALK blocker; its **only** remaining blocker is the human file-move
  (kinds/skill|rule → kinds/claude-code/* with `provider` lines, outside build's
  fence — disk today: flat kinds/skill, kinds/rule, no provider lines). Other five
  entries reconciled unchanged. Inbox empty.
- **In flight / pickable:** none — the queue is **fully human-gated**. Parked:
  BINDING-QUALIFY (human file-move), MEMORY-KIND, PACKAGING-CHANNELS, COMMUNITY-DOCS.
  Deferred: EXTRACTION-VOCAB-GAPS, AGENT-KIND (both no-consumer).
- **Next:** a human moves the curated skill/rule KIND.md into kinds/claude-code/*
  and adds `provider = "claude-code"` lines; that un-parks BINDING-QUALIFY.

Plan continues: no — queue reconciled, inbox empty, no buildable open entry to file;
the axis waits on the human file-move (kinds/** outside build's fence).
