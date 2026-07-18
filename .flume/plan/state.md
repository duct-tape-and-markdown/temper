# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: d40a9f8 — unchanged; `git log d40a9f8..HEAD -- src/
  tests/ sdk/` is empty (all four commits since — 5af93d9, 3871eba,
  9e197d6, 7a5f86c — are plan-only).
- Residue swept through: d40a9f8 — unchanged, same empty window.
- Posture swept through: 5af93d9 — unchanged, mid-rotation. This tick
  was the inbox job, not a sweep tick; foundation/model/formats stay
  ticked, next: pipeline.
- This tick: INBOX — `<inbox>` was empty (nothing to route) but
  `.flume/refactor/` held 2 live captures filed last tick
  (formats sweep, 9e197d6). Both re-verified live at HEAD (`git log
  9e197d6..HEAD -- src/ tests/ sdk/` empty — every cited line unmoved)
  and drained into pending entries, both `per`-cited and each
  serialized behind an existing chain rather than left `open`, since
  each shares a file with an in-flight entry:
  - `plan-json-manifest-import-layering.md` → **JSON-MANIFEST-
    DISCOVERY-BOUNDARY-RESTORE** (per architecture.md's Invariants
    section — the fifth instance of the ruled upward-edge disease).
    Serialized behind COVERAGE-NOTE-LOCK-PARSE-HOIST, the last entry
    in the existing chain touching main.rs; also shares json_manifest.rs
    with the open JSON-MANIFEST-READ-DECODE-CONSOLIDATE, which is
    queue-front and ships first regardless.
  - `plan-format-read-decode-duplication.md` → **FORMAT-READ-UTF8-
    DECODE-CONSOLIDATE** (per engineering.md, "One job, one home" —
    the two copies outside json_manifest.rs). Serialized behind
    JSON-MANIFEST-READ-DECODE-CONSOLIDATE per the capture's own
    sequencing ask; also shares toml_document.rs with the open
    TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE, queue-front, ships first
    regardless.
  Both capture files deleted; `.flume/refactor/` holds only README.md.
- Queue: 29 pending (27 + 2 filed this tick). 6 pickable OPEN
  (unchanged — both new entries are blockedBy), 21 chained blockedBy,
  2 parked on human action. Open forks: (multi-harness-projection),
  (lazy-grounds) unchanged. Refactor captures: 0 live. Inbox empty.

Plan continues: yes — posture sweep resumes at `pipeline` (drift/
import/read/builtin_lock/placement), the rotation's next subsystem.
