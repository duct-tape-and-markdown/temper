# Plan state

- **Phase:** reconcile + inbox drain. HEAD c00a7a2.
- **Last shipped (trunk):** READ-CUSTOM-SATISFIERS (4e3b323 build, c00a7a2 flume)
  — `why`/`requirements` now range over custom-kind satisfiers, carrying each
  member's rationale, so a custom member filling a requirement is no longer absent
  from either read verb. Verified on disk (src/read.rs).
- **This tick:** audited engine vs corpus (Explore sweep, disk not log). Every
  engine surface is shipped and wired — predicate algebra; governance set/graph
  predicates (count/membership/unique/degree/acyclic/range); apply/re-add/schema/
  bundle/install/reporters; verified_by resolution; member-published requirements;
  the read family (custom satisfiers included); the surface-language migration
  (document.rs/builtin.rs/compose.rs); custom kinds as authored KIND.md data; the
  References primitive retired; temper-local layering. No TODOs/stubs. Dogfood
  `.temper/` + root `temper.toml` + embedded `packages/*/PACKAGE.md` all present.
  One live decidable gap remains — the `[edge.<target>]` member clause is authored
  and round-tripped but never lifted into the graph's edge features (silent no-op)
  — but its fix shape is undecided ((edge-representation-unify), OPEN, 3 candidate
  representations); left as the open question, no build entry (inventing the
  canonical form would be papering over the fork). Filed COMMUNITY-DOCS (parked):
  CONTRIBUTING.md + SECURITY.md are the two missing launch docs but fall outside
  build's root-docs fence. AGENT-KIND (deferred) and PACKAGING-CHANNELS (parked)
  re-verified accurate. Inbox empty; open-questions unchanged (edge-representation-
  unify the sole live OPEN fork). Accepted debt: a stale EMBED-BUILTIN-PACKAGES note
  in root temper.toml (dogfood/human territory, not build-writable).
- **Pickable now:** none — 0 `open` entries. Deferred: AGENT-KIND. Parked:
  PACKAGING-CHANNELS, COMMUNITY-DOCS. The remaining moves are all human: settle the
  (edge-representation-unify) canonical edge form, widen the doc fence, or provide
  release creds.

Plan continues: no — queue reconciled, inbox empty, no `open` entry pickable (all
remaining work is deferred/parked or gated on an OPEN fork). Re-emitting an
unchanged queue would be the failure mode; the next wave unblocks on a human
decision, not another plan tick.
