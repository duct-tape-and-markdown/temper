# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox empty; tree clean.
  HEAD 502351e.
- **Last shipped (trunk):** COMMENT-DIET(import,contract,kind) (502351e, build) —
  the three comment sweeps drained; import/contract/kind now carry only sanctioned
  comment classes.
- **This tick:** pure reconcile — no queue change. Verified on disk (two fan-out
  reads): CONTRACTS-RETIRE is valid — `contracts/{rule,skill.anthropic}.toml` are
  orphaned, nothing loads them (build.rs embeds `packages/<name>/PACKAGE.md`; the
  only `contracts/` mentions are spec-pointer comments + one `compose.rs` test
  literal). Every other shippable corpus feature has landed: package rename to
  `skill.anthropic`/`rule.anthropic`, `why`/`requirements` read verbs,
  `section_contains`, `strip_suffix` ref normalization, `lock.toml` rename — all
  SHIPPED (refreshed those breadcrumbs in open-questions). No new gap fileable; no
  TODO/FIXME/unimplemented in `src/`. The one live gap — `edge-representation-unify`
  (authored `[edge.*]` not consumed by the gate's graph, which reads
  `[[relationships]]`+`references`) — stays an OPEN human fork (canonical form
  undecided), already recorded. Inbox empty.
- **Pickable now (1 `open`):** CONTRACTS-RETIRE (delete-only, no shared files).
  Deferred: AGENT-KIND (priority). Parked: PACKAGING-CHANNELS (release creds).

Plan continues: no — queue reconciled, inbox empty, CONTRACTS-RETIRE is pickable;
building drains it.
