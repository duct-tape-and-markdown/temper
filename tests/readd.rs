//! `temper re-add` — the on-disk → surface direction, the third drift direction
//! (`specs/20-surface.md`, "Drift / apply — three states, never two"; "the surface
//! is the source of truth", where `re-add` keeps direct on-disk editing
//! first-class).
//!
//! Drives the library `drift::re_add` over a real imported surface and proves the
//! three properties the entry names, across both built-in kinds:
//!
//! - **drifted → reconciled** — a source edited straight on disk is pulled back into
//!   the surface (its `meta.toml` header and body rewritten) and its lock row's
//!   fingerprints are refreshed to the current source bytes;
//! - **added → new artifact** — an on-disk source the surface never imported gains a
//!   surface directory and a lock row;
//! - **in-sync → no-op** — an unchanged harness leaves every surface byte identical.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use temper::check::Workspace;
use temper::drift::{self, ReAddOutcome};
use temper::import;
use temper::rule::Rule;
use temper::skill::Skill;
use toml_edit::DocumentMut;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-readd-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// The one-skill + one-rule harness the tests import as their baseline.
const SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes.\n\
version: \"0.3.0\"\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

const RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

/// The on-disk source paths of the imported artifacts in `harness`.
fn skill_source(harness: &Path) -> PathBuf {
    harness.join("skills").join("coordinate").join("SKILL.md")
}

fn skill_dir(harness: &Path) -> PathBuf {
    harness.join("skills").join("coordinate")
}

fn rule_source(harness: &Path) -> PathBuf {
    harness.join(".claude").join("rules").join("rust.md")
}

/// Build a one-skill + one-rule harness and import it into a fresh surface,
/// returning `(harness, workspace)`.
fn imported(label: &str) -> (PathBuf, PathBuf) {
    let harness = tmpdir(&format!("{label}-src"));
    let skill = harness.join("skills").join("coordinate");
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

/// The outcome `re_add` reported for `name`, asserting it is unique.
fn outcome(report: &drift::ReAddReport, name: &str) -> ReAddOutcome {
    let mut matches = report.entries.iter().filter(|e| e.name == name);
    let found = matches.next().expect("entry should exist");
    assert!(matches.next().is_none(), "entry {name} should be unique");
    found.outcome
}

/// Read one string column from the `[[<kind>]]` lock row named `name`.
fn lock_field(into: &Path, kind: &str, name: &str, field: &str) -> String {
    let doc = fs::read_to_string(into.join("lock.toml"))
        .unwrap()
        .parse::<DocumentMut>()
        .unwrap();
    doc[kind]
        .as_array_of_tables()
        .unwrap()
        .iter()
        .find(|row| row["name"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("lock is missing a [[{kind}]] row named {name}"))[field]
        .as_str()
        .unwrap()
        .to_string()
}

/// Whether the `[[<kind>]]` lock array carries a row named `name`.
fn lock_has_row(into: &Path, kind: &str, name: &str) -> bool {
    let doc = fs::read_to_string(into.join("lock.toml"))
        .unwrap()
        .parse::<DocumentMut>()
        .unwrap();
    doc.get(kind)
        .and_then(|item| item.as_array_of_tables())
        .is_some_and(|rows| rows.iter().any(|row| row["name"].as_str() == Some(name)))
}

#[test]
fn an_unchanged_harness_is_a_noop() {
    let (harness, into) = imported("clean");
    let before = tree_bytes(&into);

    let ws = Workspace::load(&into).unwrap();
    let report = drift::re_add(&ws, &into, &harness).unwrap();

    // Every artifact still hashes to the import baseline — nothing to pull in.
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Unchanged);
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Unchanged);
    assert_eq!(
        before,
        tree_bytes(&into),
        "an in-sync re-add must leave every surface byte identical"
    );
}

#[test]
fn a_drifted_skill_is_reconciled_into_the_surface() {
    let (harness, into) = imported("skill-drift");
    let before_hash = lock_field(&into, "skill", "coordinate", "import_hash");

    // The human edits the skill straight on disk — a frontmatter field *and* the
    // body change, so a genuine re-projection (not just a body copy) is required.
    let drifted = "---\n\
name: coordinate\n\
description: Edited straight on disk, outside the surface.\n\
version: \"0.4.0\"\n\
---\n\
# Coordinate\n\
\n\
An edited body, straight on disk.\n";
    fs::write(skill_source(&harness), drifted).unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::re_add(&ws, &into, &harness).unwrap();
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Reconciled);
    // The untouched rule stays in sync.
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Unchanged);

    // The surface header was rewritten: the reloaded skill carries the edited field.
    let surface = into.join("skills").join("coordinate");
    let reloaded = Skill::from_surface_dir(&surface).unwrap();
    assert_eq!(
        reloaded.description,
        "Edited straight on disk, outside the surface."
    );
    assert_eq!(reloaded.version.as_deref(), Some("0.4.0"));
    // ...and the body was pulled in byte-faithfully (frontmatter stripped).
    assert_eq!(
        fs::read_to_string(surface.join("SKILL.md")).unwrap(),
        "# Coordinate\n\nAn edited body, straight on disk.\n"
    );

    // The lock fingerprints were refreshed to the current source bytes: the drift
    // anchor now hashes the edited file, and `last_applied` tracks it — both
    // different from the pre-drift baseline. The fresh hash matches what a source
    // re-parse computes, so the lock truly reflects on-disk reality.
    let fresh = Skill::from_source_dir(&skill_dir(&harness))
        .unwrap()
        .provenance
        .import_hash;
    let after_hash = lock_field(&into, "skill", "coordinate", "import_hash");
    assert_ne!(after_hash, before_hash, "the import_hash must be bumped");
    assert_eq!(
        after_hash, fresh,
        "the lock anchors the current source bytes"
    );
    assert_eq!(
        lock_field(&into, "skill", "coordinate", "last_applied"),
        fresh,
        "last_applied is reconciled to the current source"
    );
    // The surface provenance and the lock agree on the refreshed anchor.
    assert_eq!(reloaded.provenance.import_hash, fresh);
}

#[test]
fn a_drifted_rule_is_reconciled_into_the_surface() {
    let (harness, into) = imported("rule-drift");
    let before_hash = lock_field(&into, "rule", "rust", "import_hash");

    // Edit the rule on disk: broaden `paths` and rewrite the body.
    let drifted = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
  - \"tests/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
An edited rule body.\n";
    fs::write(rule_source(&harness), drifted).unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::re_add(&ws, &into, &harness).unwrap();
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Reconciled);
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Unchanged);

    // The surface header carries the broadened scope, and the body is byte-faithful.
    let surface = into.join("rules").join("rust");
    let reloaded = Rule::from_surface_dir(&surface).unwrap();
    assert_eq!(
        reloaded.paths.as_deref(),
        Some(&["src/**/*.rs".to_string(), "tests/**/*.rs".to_string()][..])
    );
    assert_eq!(
        fs::read_to_string(surface.join("RULE.md")).unwrap(),
        "# Rust conventions\n\nAn edited rule body.\n"
    );

    // The lock anchor is refreshed to the edited source bytes.
    let fresh = Rule::from_source_file(&rule_source(&harness))
        .unwrap()
        .provenance
        .import_hash;
    assert_ne!(
        lock_field(&into, "rule", "rust", "import_hash"),
        before_hash
    );
    assert_eq!(lock_field(&into, "rule", "rust", "import_hash"), fresh);
    assert_eq!(lock_field(&into, "rule", "rust", "last_applied"), fresh);
}

#[test]
fn a_drifted_body_readd_preserves_authored_representation() {
    let (harness, into) = imported("rep-preserve");

    // Author the surface-only representation layer on both kinds — `satisfies`
    // and `rationale` the source files never carry.
    let skill_surface = into.join("skills").join("coordinate");
    let mut skill = Skill::from_surface_dir(&skill_surface).unwrap();
    skill.satisfies = vec!["req.coordinate".to_string()];
    skill.rationale = Some("Fills the coordination requirement.".to_string());
    fs::write(
        skill_surface.join("meta.toml"),
        skill.to_meta_document().to_string(),
    )
    .unwrap();

    let rule_surface = into.join("rules").join("rust");
    let mut rule = Rule::from_surface_dir(&rule_surface).unwrap();
    rule.satisfies = vec!["req.rust-style".to_string()];
    rule.rationale = Some("Encodes the Rust conventions the gate enforces.".to_string());
    fs::write(
        rule_surface.join("meta.toml"),
        rule.to_meta_document().to_string(),
    )
    .unwrap();

    // Drift only the bodies on disk, so `re-add` genuinely rebuilds `meta.toml`
    // from source — the path that clobbers representation today.
    fs::write(
        skill_source(&harness),
        "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes.\n\
version: \"0.3.0\"\n\
---\n\
# Coordinate\n\
\n\
An edited body, straight on disk.\n",
    )
    .unwrap();
    fs::write(
        rule_source(&harness),
        "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
An edited rule body.\n",
    )
    .unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::re_add(&ws, &into, &harness).unwrap();
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Reconciled);
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Reconciled);

    // The authored representation survived the body-drift re-add — the data-loss
    // this entry fixes (both fields are wiped without the carry).
    let skill = Skill::from_surface_dir(&skill_surface).unwrap();
    assert_eq!(skill.satisfies, vec!["req.coordinate"]);
    assert_eq!(
        skill.rationale.as_deref(),
        Some("Fills the coordination requirement.")
    );
    // ...and the drifted body was still pulled in byte-faithfully.
    assert_eq!(
        fs::read_to_string(skill_surface.join("SKILL.md")).unwrap(),
        "# Coordinate\n\nAn edited body, straight on disk.\n"
    );

    let rule = Rule::from_surface_dir(&rule_surface).unwrap();
    assert_eq!(rule.satisfies, vec!["req.rust-style"]);
    assert_eq!(
        rule.rationale.as_deref(),
        Some("Encodes the Rust conventions the gate enforces.")
    );
    assert_eq!(
        fs::read_to_string(rule_surface.join("RULE.md")).unwrap(),
        "# Rust conventions\n\nAn edited rule body.\n"
    );
}

#[test]
fn a_reimport_of_an_authored_surface_preserves_representation_and_is_idempotent() {
    let (harness, into) = imported("rep-reimport");

    // Author representation on the surface, then re-import the *unchanged* harness.
    let skill_surface = into.join("skills").join("coordinate");
    let mut skill = Skill::from_surface_dir(&skill_surface).unwrap();
    skill.satisfies = vec!["req.coordinate".to_string()];
    skill.rationale = Some("Fills the coordination requirement.".to_string());
    fs::write(
        skill_surface.join("meta.toml"),
        skill.to_meta_document().to_string(),
    )
    .unwrap();

    let before = tree_bytes(&into);
    // A re-import rebuilds every `meta.toml` from source; carrying the surface's
    // authored representation forward keeps it — and the workspace byte-identical.
    import::run(&harness, &into).unwrap();

    assert_eq!(
        before,
        tree_bytes(&into),
        "re-importing an authored, unchanged surface must not change a byte"
    );
    let skill = Skill::from_surface_dir(&skill_surface).unwrap();
    assert_eq!(skill.satisfies, vec!["req.coordinate"]);
    assert_eq!(
        skill.rationale.as_deref(),
        Some("Fills the coordination requirement.")
    );
}

#[test]
fn an_added_source_becomes_a_new_surface_artifact_and_lock_row() {
    let (harness, into) = imported("added");

    // A skill and a rule that live on disk but the surface never imported.
    let helper = harness.join("skills").join("helper");
    fs::create_dir_all(&helper).unwrap();
    fs::write(
        helper.join("SKILL.md"),
        "---\n\
name: helper\n\
description: A skill added straight to the harness, after import.\n\
---\n\
# Helper\n\
\n\
A helping hand.\n",
    )
    .unwrap();
    fs::write(
        harness.join(".claude").join("rules").join("extra.md"),
        "# Extra\n\nA rule added straight to the harness, after import.\n",
    )
    .unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::re_add(&ws, &into, &harness).unwrap();

    // The new sources are added; the pre-existing artifacts stay in sync.
    assert_eq!(outcome(&report, "helper"), ReAddOutcome::Added);
    assert_eq!(outcome(&report, "extra"), ReAddOutcome::Added);
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Unchanged);
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Unchanged);

    // Each added source gained a surface directory that reloads through its kind's
    // loader — a first-class surface artifact, not a partial write.
    let skill_surface = into.join("skills").join("helper");
    assert!(skill_surface.join("meta.toml").is_file());
    let reloaded = Skill::from_surface_dir(&skill_surface).unwrap();
    assert_eq!(reloaded.name, "helper");
    assert_eq!(
        fs::read_to_string(skill_surface.join("SKILL.md")).unwrap(),
        "# Helper\n\nA helping hand.\n"
    );

    let rule_surface = into.join("rules").join("extra");
    assert!(rule_surface.join("meta.toml").is_file());
    assert_eq!(
        fs::read_to_string(rule_surface.join("RULE.md")).unwrap(),
        "# Extra\n\nA rule added straight to the harness, after import.\n"
    );

    // Each added source gained a lock row anchored to its source bytes.
    assert!(lock_has_row(&into, "skill", "helper"));
    assert!(lock_has_row(&into, "rule", "extra"));
    let fresh = Skill::from_source_dir(&helper)
        .unwrap()
        .provenance
        .import_hash;
    assert_eq!(lock_field(&into, "skill", "helper", "import_hash"), fresh);
    assert_eq!(lock_field(&into, "skill", "helper", "last_applied"), fresh);
    // The original rows survive alongside the new ones.
    assert!(lock_has_row(&into, "skill", "coordinate"));
    assert!(lock_has_row(&into, "rule", "rust"));
}
