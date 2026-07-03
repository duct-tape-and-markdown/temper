# Plan state

- **Phase:** reconcile. HEAD b82be78.
- **Last shipped:** REACHABILITY-WIRE (build 4a4bce2, chore 5eaf2ff) — the queue's
  prior sole pickable; then the human resolved `(kind-harness-axis)` (b82be78,
  cited market sweep 1bda46c).
- **This tick:** drained the inbox's `(kind-harness-axis)` resolution into two
  entries — filed **PROVIDER-KEY-PARSE** (open; kind.rs adds the inert `provider`
  key + qualified-identity + bare→unique-or-collision resolution, red-interim shape
  like the shipped FORMAT/ACTIVATION key-parses) and **BINDING-QUALIFY** (parked;
  build.rs nested walk + qualified bindings, needs the human file-move (2), outside
  build's fence). Re-reconciled the other five against disk — all stay accurately
  gated (kinds/ = skill+rule only, `Primitive` still field/headings/sections/
  line_count/placement + flat `Field`, no release.yml, private flume package.json,
  no CONTRIBUTING/SECURITY). Inbox drained.
- **In flight / pickable:** PROVIDER-KEY-PARSE (the sole open entry). Everything else
  is parked (BINDING-QUALIFY, MEMORY-KIND, PACKAGING-CHANNELS, COMMUNITY-DOCS) or
  deferred (EXTRACTION-VOCAB-GAPS, AGENT-KIND).
- **Next:** build picks PROVIDER-KEY-PARSE. Its consequence (3) BINDING-QUALIFY then
  waits on the human follow-up (2) — move curated skill/rule KIND.md to
  kinds/claude-code/* with `provider = "claude-code"` lines.

Plan continues: no — queue reconciled, inbox drained, one pickable entry filed; hand
to build.
