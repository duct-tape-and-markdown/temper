# Plan state

- **Phase:** reconcile. HEAD c4e2155.
- **Last shipped (trunk):** IMPORT-DISCOVERY-GENERIC (cafad7c) — the
  extraction/read-path **unification wave has fully drained**: built-in kinds
  discover off their embedded `kinds/<name>/KIND.md` `governs`, every surface reads
  through the one generic `Unit` loader, and import discovery is generic. Verified
  on disk.
- **This tick:** audited `src/` against the whole corpus — the engine **conforms to
  current decidable intent**. Present and generic: custom kinds as data
  (`.temper/kinds/spec/KIND.md`, no `src/spec.rs`), composed extraction over the
  closed `Primitive` algebra, project-authored packages + admissibility
  (`.temper/packages/`), the `temper.toml` assembly, coverage/read-verbs/graph
  (READ-EDGE-UNIFY shipped), drift/apply/re-add, bundle/install/schema/reporters,
  and the full predicate + governance vocabulary (range/count/unique/membership/
  degree/acyclic/section_contains). Filed **EXTRACTION-VOCAB-GAPS** (deferred) — the
  two cited-but-consumer-less primitives (`fenced` block, key-path `field`). Drained
  inbox: KIND-PROSE-RECONCILE done by hand (kinds/ is curated territory outside
  build's fence — the revert was correct) → accepted, not re-filed.
- **Pickable now:** none. AGENT-KIND + EXTRACTION-VOCAB-GAPS deferred;
  PACKAGING-CHANNELS + COMMUNITY-DOCS parked. All remaining work is **human-gated**:
  the `(edge-representation-unify)` fork (`[edge.*]` is now dead surface syntax the
  gate ignores — human to settle the canonical edge form); the spec class-split
  corpus migration (`.temper/`/`packages/`/`specs/` — outside build's fence); release
  creds; a chain.ts fence-widen for the root community docs.

Plan continues: no — queue reconciled, inbox drained, no in-fence decidable open
gap. Remaining work is human-gated (an open fork, dogfood-corpus territory, release
creds), so there is no pickable build entry; hand off.
