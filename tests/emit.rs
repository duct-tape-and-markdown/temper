//! `temper emit` — the write direction (`specs/architecture/20-surface.md`,
//! "Content-faithful, deterministically emitted (law 5)").
//!
//! Drives the library `drift::emit` over a real imported surface and proves the
//! properties the entry names:
//!
//! - **deterministic re-emission** — emit regenerates each projection full-file
//!   from the member document; a surface field edit re-renders the whole
//!   frontmatter, and on-disk-only bytes (a hand-added comment) are dropped rather
//!   than preserved — a direct projection edit is drift routed to the source;
//! - **idempotence** — emit twice yields byte-identical output; the re-run changes
//!   nothing;
//! - **double-emit determinism** — a second emit run reproduces the projection *and*
//!   the lock byte-for-byte (nondeterminism would be a loud failure, never a churn);
//! - **dry-run writes nothing** — the outcome is reported but not a byte lands.
//!
//! There is no three-state conflict state: emit re-emits whole, so a hand-edited
//! projection is overwritten (drift routed to the source), never a mergeable
//! conflict. A trailing case pins the lock's two freshness facts and that emit
//! baselines its idempotence on `emit_hash`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use temper::check::Workspace;
use temper::drift::{self, EmitOptions, EmitOutcome};
use temper::frontmatter::Member;
use temper::import;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-emit-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A skill whose frontmatter carries a human comment — an on-disk-only byte
/// deterministic re-emission drops (it is not surface content, so it is drift). The
/// body keeps a missing final newline so the byte-faithful round trip is observable.
const SKILL: &str = "---\n\
name: coordinate\n\
# a human note that must survive a patch\n\
description: Use when coordinating agents across axes.\n\
license: \"MIT\"\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.";

/// A rule with `paths:` frontmatter and an unknown Cursor key preserved verbatim.
const RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

/// The on-disk source path of the imported skill in `harness`.
fn skill_source(harness: &Path) -> PathBuf {
    harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md")
}

/// Build a one-skill + one-rule harness and import it into a fresh surface,
/// returning `(harness, workspace)`.
fn imported(label: &str) -> (PathBuf, PathBuf) {
    let harness = tmpdir(&format!("{label}-src"));
    let skill = harness.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), RULE).unwrap();

    let into = tmpdir(&format!("{label}-into"));
    import::run(&harness, &into).unwrap();
    (harness, into)
}

/// Snapshot every file under `dir` as a sorted map of relative path -> bytes.
fn tree_bytes(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        for entry in fs::read_dir(&current).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path.strip_prefix(dir).unwrap().to_path_buf();
                out.insert(rel, fs::read(&path).unwrap());
            }
        }
    }
    out
}

/// The outcome `emit` reported for `name` in `report`, asserting it is unique.
fn outcome(report: &drift::EmitReport, name: &str) -> EmitOutcome {
    let mut matches = report.entries.iter().filter(|e| e.name == name);
    let found = matches.next().expect("entry should exist");
    assert!(matches.next().is_none(), "entry {name} should be unique");
    found.outcome
}

/// Rewrite the surface skill's `description` to `new`, exactly as a human editing
/// the composition surface would.
fn edit_surface_description(workspace: &Path, new: &str) {
    let dir = workspace.join("skills").join("coordinate");
    let mut member = Member::from_surface(&dir, "SKILL.md").unwrap();
    if let Some(f) = member.fields.iter_mut().find(|(k, _)| k == "description") {
        f.1 = serde_json::Value::String(new.to_string());
    }
    fs::write(dir.join("SKILL.md"), member.to_document().emit()).unwrap();
}

#[test]
fn emit_is_idempotent_over_a_clean_surface() {
    let (harness, into) = imported("idem");

    // The first emit re-emits each projection to its canonical form: the imported
    // bytes are not yet canonical (frontmatter is regenerated deterministically), so
    // emit lands the re-emission and advances the lock.
    let ws = Workspace::load(&into).unwrap();
    drift::emit(&ws, &into, EmitOptions::default()).unwrap();
    let after_first = tree_bytes(&harness);

    // Re-running is the idempotent no-op — every artifact Unchanged, not a byte moves.
    let ws = Workspace::load(&into).unwrap();
    let report = drift::emit(&ws, &into, EmitOptions::default()).unwrap();
    assert!(
        report
            .entries
            .iter()
            .all(|e| e.outcome == EmitOutcome::Unchanged)
    );
    assert_eq!(
        after_first,
        tree_bytes(&harness),
        "emit twice yields byte-identical output"
    );
}

#[test]
fn a_surface_edit_re_emits_the_projection() {
    let (harness, into) = imported("reemit");

    edit_surface_description(&into, "Use when driving a team across many axes.");

    let ws = Workspace::load(&into).unwrap();
    let report = drift::emit(&ws, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "coordinate"), EmitOutcome::Emitted);

    let emitted = fs::read_to_string(skill_source(&harness)).unwrap();
    // The whole projection is regenerated from the member document: the edited field
    // carries its new value and the old value is gone.
    assert!(
        emitted.contains("Use when driving a team across many axes."),
        "the edited description must land on disk, got:\n{emitted}"
    );
    assert!(
        !emitted.contains("Use when coordinating agents across axes."),
        "the old description must be gone, got:\n{emitted}"
    );
    // The on-disk-only comment is NOT preserved — re-emission regenerates the
    // frontmatter from the surface, and a direct projection edit is drift, not
    // something emit merges around.
    assert!(
        !emitted.contains("# a human note"),
        "the on-disk-only comment must be dropped by re-emission, got:\n{emitted}"
    );
    // Every surface field is re-rendered deterministically (JSON-flow YAML), and the
    // body lands byte-faithful (no trailing newline added).
    assert!(
        emitted.contains("name: \"coordinate\"\n"),
        "name re-emitted, got:\n{emitted}"
    );
    assert!(
        emitted.contains("license: \"MIT\"\n"),
        "license re-emitted, got:\n{emitted}"
    );
    assert!(
        emitted.ends_with("Drive the team through the playbook."),
        "the body stays byte-faithful, got:\n{emitted}"
    );

    // The re-emitted source re-parses to the same typed skill the surface holds.
    let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
    let reloaded = Member::from_source(
        &skill_kind,
        &harness
            .join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
    )
    .expect("the re-emitted source must re-parse");
    assert_eq!(
        reloaded
            .field("description")
            .and_then(|v| v.as_str())
            .unwrap(),
        "Use when driving a team across many axes."
    );
    assert_eq!(
        reloaded.field("license").and_then(|v| v.as_str()),
        Some("MIT")
    );
}

#[test]
fn a_hand_edited_projection_is_overwritten_not_conflicted() {
    let (harness, into) = imported("reemit-over-drift");

    // Reach the re-emit fixpoint first: an emit canonicalizes both projections and
    // advances the lock.
    let ws = Workspace::load(&into).unwrap();
    drift::emit(&ws, &into, EmitOptions::default()).unwrap();
    let canonical = fs::read_to_string(skill_source(&harness)).unwrap();

    // A human edits the projection directly, on disk — drift the surface knows
    // nothing about. The surface itself is left as imported.
    let source = skill_source(&harness);
    fs::write(
        &source,
        canonical.clone() + "\nA line added straight to disk.\n",
    )
    .unwrap();

    // Emit re-emits the projection whole: the hand edit is overwritten (it is drift
    // routed to the source, surfaced by config.stale/the guard — not a merge). No
    // three-state conflict state exists to report.
    let ws = Workspace::load(&into).unwrap();
    let report = drift::emit(&ws, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "coordinate"), EmitOutcome::Emitted);
    assert_eq!(
        fs::read_to_string(&source).unwrap(),
        canonical,
        "emit re-emits the canonical projection over a hand edit"
    );
    // The untouched rule is already at its fixpoint — the re-run is a clean no-op.
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Unchanged);
}

#[test]
fn dry_run_reports_the_outcome_but_writes_nothing() {
    let (harness, into) = imported("dry");

    edit_surface_description(&into, "A description the dry run must not write.");
    let before_harness = tree_bytes(&harness);
    let before_lock = fs::read(into.join("lock.toml")).unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::emit(
        &ws,
        &into,
        EmitOptions {
            dry_run: true,
            ..Default::default()
        },
    )
    .unwrap();
    // The report still says what *would* happen...
    assert_eq!(outcome(&report, "coordinate"), EmitOutcome::Emitted);
    // ...but not a byte of the harness or the lock changed.
    assert_eq!(
        before_harness,
        tree_bytes(&harness),
        "--dry-run must not touch the harness sources"
    );
    assert_eq!(
        before_lock,
        fs::read(into.join("lock.toml")).unwrap(),
        "--dry-run must not touch the lock fingerprints"
    );

    // A real emit afterwards does land the edit.
    let ws = Workspace::load(&into).unwrap();
    drift::emit(&ws, &into, EmitOptions::default()).unwrap();
    assert!(
        fs::read_to_string(skill_source(&harness))
            .unwrap()
            .contains("A description the dry run must not write."),
        "the real emit must write what the dry run only reported"
    );
}

#[test]
fn double_emit_reproduces_projection_and_lock_byte_for_byte() {
    // Determinism, verified at the run level (`specs/architecture/20-surface.md`, law 5):
    // a second emit over the same surface reproduces every projected byte *and* the
    // lock exactly. Emit's internal double-emit guard raises on nondeterministic
    // authoring; this pins the observable property.
    let (harness, into) = imported("double");

    // The first emit canonicalizes and advances the lock off the stale import baseline.
    let ws = Workspace::load(&into).unwrap();
    drift::emit(&ws, &into, EmitOptions::default()).unwrap();
    let harness_after_first = tree_bytes(&harness);
    let lock_after_first = fs::read(into.join("lock.toml")).unwrap();

    // The second emit is byte-identical in both the projection tree and the lock.
    let ws = Workspace::load(&into).unwrap();
    let report = drift::emit(&ws, &into, EmitOptions::default()).unwrap();
    assert!(
        report
            .entries
            .iter()
            .all(|e| e.outcome == EmitOutcome::Unchanged),
        "a second emit finds every projection already at its bytes"
    );
    assert_eq!(
        harness_after_first,
        tree_bytes(&harness),
        "double emit reproduces the projection byte-for-byte"
    );
    assert_eq!(
        lock_after_first,
        fs::read(into.join("lock.toml")).unwrap(),
        "double emit leaves the lock byte-for-byte identical"
    );
}

#[test]
fn successive_surface_edits_each_emit_cleanly() {
    // Two successive surface edits both re-emit — emit regenerates whole, so the
    // second is never misread against the first's fingerprint.
    let (harness, into) = imported("successive");

    edit_surface_description(&into, "First edit.");
    let ws = Workspace::load(&into).unwrap();
    assert_eq!(
        outcome(
            &drift::emit(&ws, &into, EmitOptions::default()).unwrap(),
            "coordinate"
        ),
        EmitOutcome::Emitted
    );

    edit_surface_description(&into, "Second edit, atop the first.");
    let ws = Workspace::load(&into).unwrap();
    assert_eq!(
        outcome(
            &drift::emit(&ws, &into, EmitOptions::default()).unwrap(),
            "coordinate"
        ),
        EmitOutcome::Emitted,
        "a second surface edit re-emits, not a no-op"
    );

    assert!(
        fs::read_to_string(skill_source(&harness))
            .unwrap()
            .contains("Second edit, atop the first."),
        "the second edit must be the one on disk"
    );
}

/// One column of the sole `[[skill]]` lock row in `workspace`, or `None` if absent.
fn skill_lock_field(workspace: &Path, column: &str) -> Option<String> {
    let doc = fs::read_to_string(workspace.join("lock.toml"))
        .unwrap()
        .parse::<toml_edit::DocumentMut>()
        .unwrap();
    doc.get("skill")?
        .as_array_of_tables()?
        .iter()
        .next()?
        .get(column)
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

#[test]
fn the_lock_carries_the_two_freshness_facts_and_emit_baselines_on_emit_hash() {
    let (harness, into) = imported("freshness");

    // The lock row carries the two freshness facts under their names — and neither
    // pre-rename column survives (`specs/architecture/20-surface.md`, "two freshness facts").
    let source_at_import = skill_lock_field(&into, "source_hash").expect("source_hash column");
    let emit_at_import = skill_lock_field(&into, "emit_hash").expect("emit_hash column");
    assert_eq!(source_at_import.len(), 64);
    assert!(skill_lock_field(&into, "import_hash").is_none());
    assert!(skill_lock_field(&into, "last_applied").is_none());
    // At import emit provisionally equals source: no `emit` has advanced it yet.
    assert_eq!(emit_at_import, source_at_import);

    // The imported bytes are not yet canonical (the fixture carries a hand comment and
    // loose frontmatter), so the first emit re-emits a deterministic projection over
    // the source and advances `emit_hash` to that projection's fingerprint — while
    // `source_hash`, the authored-bytes fact, is left untouched.
    let ws = Workspace::load(&into).unwrap();
    assert_eq!(
        outcome(
            &drift::emit(&ws, &into, EmitOptions::default()).unwrap(),
            "coordinate"
        ),
        EmitOutcome::Emitted
    );
    let projected = fs::read_to_string(skill_source(&harness)).unwrap();
    let emit_after = skill_lock_field(&into, "emit_hash").expect("emit_hash column");
    assert_eq!(
        skill_lock_field(&into, "source_hash").as_deref(),
        Some(source_at_import.as_str()),
        "source_hash — the authored-bytes fact — is untouched by emit"
    );
    assert_ne!(
        emit_after, source_at_import,
        "emit rewrote emit_hash off the stale import baseline onto the real projection"
    );

    // emit's idempotence baselines on the projection: the bytes now on disk match it,
    // so a re-run is the idempotent no-op rather than a re-emission.
    let ws = Workspace::load(&into).unwrap();
    assert_eq!(
        outcome(
            &drift::emit(&ws, &into, EmitOptions::default()).unwrap(),
            "coordinate"
        ),
        EmitOutcome::Unchanged,
        "with the projection already on disk, emit no-ops"
    );

    // A hand-edit straight to the projection is drift: emit re-emits the canonical
    // projection over it (surfaced elsewhere by config.stale/the guard), never a merge.
    let drifted = projected.clone() + "\nA line added straight to disk.\n";
    fs::write(skill_source(&harness), &drifted).unwrap();
    let ws = Workspace::load(&into).unwrap();
    assert_eq!(
        outcome(
            &drift::emit(&ws, &into, EmitOptions::default()).unwrap(),
            "coordinate"
        ),
        EmitOutcome::Emitted,
        "a projection differing from the surface re-emits whole"
    );
    assert_eq!(
        fs::read_to_string(skill_source(&harness)).unwrap(),
        projected,
        "the canonical projection overwrites the hand edit"
    );
}
