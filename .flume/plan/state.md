# Plan state

- **Phase:** reconcile. HEAD 7e7e5ee.
- **Last shipped (trunk):** SURFACE-READING-GENERIC — `check` reads every kind's
  surface member through the one generic `Unit` loader (verified on disk: the
  IR→Unit adapter is off the check path).
- **This tick:** unblocked IMPORT-DISCOVERY-GENERIC — its blocker
  SURFACE-READING-GENERIC shipped, so gate → open. Verified on disk it is NOT yet
  done: `discover_skill_dirs`/`discover_rule_files` still exist and are called from
  import/drift/install; the embedded skill/rule KIND.md carry `governs` and the
  `*/SKILL.md` subdir-glob wrinkle in `discover_kind_units` is real. Inbox empty;
  no new decidable gap with a clean cite. Deferred/parked trio unchanged.
- **Pickable now:** IMPORT-DISCOVERY-GENERIC (open, sole). AGENT-KIND deferred;
  PACKAGING-CHANNELS / COMMUNITY-DOCS parked. Sole live OPEN fork:
  `(edge-representation-unify)` — human to settle the canonical edge form.

Plan continues: no — queue reconciled, one open entry pickable, inbox drained; hand to build.
