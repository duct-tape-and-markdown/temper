# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 1b0ea01 — unchanged, no src/tests/sdk commits since.
- Residue swept through: 1b0ea01 — unchanged, no src/tests/sdk commits since.
- Posture swept through: src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs covered, mid-rotation — advanced from src/builtin_lock.rs. src/check.rs is the tree-order candidate next.
- This tick: POSTURE SWEEP, src/bundle.rs neighborhood (frontier module + its immediate imports: builtin_kind::definition, drift::project_bytes, json_manifest::write_manifest, install::session_start_group/SESSION_START_COMMAND, display::plural, frontmatter::Member). Found: the plugin's skill (`skills/temper/SKILL.md`) and hooks (`hooks/hooks.json`) placements are Claude Code plugin-layout facts asserted as bare literals with no citation, the skill path also silently diverging from the `skill` kind's own governs locus (builtin_kind.rs 179-182) — filed BUNDLE-PLUGIN-LAYOUT-CITE (per architecture.md, "The provider face is data"). Checked clean: plugin.json/marketplace.json already route through `write_member`'s kind-derived governs (no duplicate-locus residue there); `BundleError` variants are a real safety net over the embedded roster, not dead plumbing; the byte-faithful skill test drives the real writer through the real reader (no self-agreement gate); `bundle::run`/`render` are consumed by main.rs (export earns its consumer); no cohesion or shared-enumeration violation found.
- Queue: 3 pending, 1 open (BUNDLE-PLUGIN-LAYOUT-CITE), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unchanged, not this tick's job). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — a pickable entry now exists (BUNDLE-PLUGIN-LAYOUT-CITE); the posture sweep's frontier is still open past src/bundle.rs (src/check.rs next), so it resumes once the wave hands back.
