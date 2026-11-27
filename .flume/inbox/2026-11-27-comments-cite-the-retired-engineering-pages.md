# Comments cite the retired engineering pages

`specs/process/engineering.md` and `architecture.md` are gone (0122): the posture is the engineering playbook in `.temper/`. About 45 comments still point at them, in `tests/it/check_cost.rs` (most of them), `src/compose.rs`, `src/gate.rs`, `src/drift.rs`, `src/graph.rs`, `src/member_address.rs`, `sdk/src/declarations.ts`, `Cargo.toml`, and the tests `main.rs`, `common/mod.rs`, `read_verbs.rs`, `session_start.rs`, `version_parity.rs`, `lock_declaration_rows.rs`. Find them with `rg -n 'engineering\.md|architecture\.md' src tests sdk/src Cargo.toml`.

The narration topic already settles the fix: spec citations are retired from comments, so the pointer is cut and the constraint it carried stays, in the comment's own words. A repoint to the playbook would put back the unchecked pointer the topic cuts.

Acceptance:
- **new:** that `rg` returns nothing under `src`, `tests`, `sdk/src` and `Cargo.toml`;
- **kept:** no constraint a cut comment stated is lost. The cut removes the parenthetical and keeps the sentence.
