//! The adapter byte-fidelity baseline (`specs/architecture/15-kinds.md`, "Decision: the adapter
//! faces are declared — a kind names its projection format").
//!
//! The frozen equivalence the declared-adapter collapse must not move: the
//! per-kind adapter modules (`src/skill.rs`, `src/rule.rs`) retire into one generic
//! frontmatter adapter, and the generic path (DECLARED-FRONTMATTER-ADAPTER) is
//! diffed against exactly these pins. Each property is exercised end-to-end over
//! real-shaped Claude Code fixtures (`.claude/skills/<name>/SKILL.md`,
//! `.claude/rules/*.md`), never a layout invented for the test:
//!
//! - **import→apply is a byte fixpoint** — a source already in the adapter's
//!   canonical projection form survives a full import→apply→import→apply cycle
//!   byte-for-byte;
//! - **projected YAML key ordering is stable** — `apply` re-emits the frontmatter
//!   in a fixed order (typed scalars, then sorted unknown keys), snapshotted;
//! - **unknown keys are preserved verbatim** — a project convention key the harness
//!   ignores (`team:`) survives import and lands back in the projected YAML;
//! - **a fieldless rule projects with no frontmatter** — a rule that declares no
//!   `paths`/unknown keys projects to its body alone, no empty `---` block;
//! - **skill companions are copied byte-for-byte** — a skill's `PLAYBOOK.md` and a
//!   nested script are never re-rendered, on either the import or the apply face.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use temper::check::Workspace;
use temper::drift::{self, EmitOptions};
use temper::import;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "adapter-fidelity-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Snapshot every file under `dir` as a sorted map of relative path -> bytes, so
/// two states of a tree can be compared for an exact byte diff.
fn tree_bytes(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    for entry in walkdir::WalkDir::new(dir).min_depth(1).sort_by_file_name() {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(dir).unwrap().to_path_buf();
            out.insert(rel, fs::read(entry.path()).unwrap());
        }
    }
    out
}

/// A skill source already in the adapter's **canonical projection form** — every
/// scalar double-quoted (JSON flow), the one unknown key (`team`) a flow sequence,
/// keys in the order `apply` re-emits them. Because the source is already what the
/// emit face renders, an import→apply round trip reproduces it byte-for-byte. The
/// body keeps a missing final newline so a byte-faithful copy is observable.
const CANONICAL_SKILL: &str = "---\n\
name: \"coordinate\"\n\
description: \"Use when coordinating agents across axes.\"\n\
license: \"MIT\"\n\
team: [\"platform\",\"infra\"]\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.";

/// The companion files `coordinate` carries beside its member document — a markdown
/// playbook and a nested script, both byte-faithful (odd whitespace, no final
/// newline) so a re-render would be caught.
const PLAYBOOK: &str = "# Playbook\n\nStep one.   \nStep two, no trailing newline.";
const SCRIPT: &str = "#!/bin/sh\necho coordinate\n";

/// A rule with a `paths:` scope and an unknown Cursor key the harness ignores —
/// preserved verbatim through the round trip, not dropped.
const SCOPED_RULE: &str = "---\n\
paths: [\"src/**/*.rs\"]\n\
description: \"A Cursor key Claude Code ignores.\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

/// A rule with no frontmatter at all — the `collaboration.md` shape. It declares no
/// fields, so the adapter projects it to its body alone, no `---` block.
const FIELDLESS_RULE: &str = "# Collaboration\n\nPushback is the point.\n";

/// The on-disk source path of the imported `coordinate` skill in `harness`.
fn skill_source(harness: &Path) -> PathBuf {
    harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md")
}

/// The on-disk source path of the imported `collaboration` rule in `harness`.
fn fieldless_rule_source(harness: &Path) -> PathBuf {
    harness
        .join(".claude")
        .join("rules")
        .join("collaboration.md")
}

/// Build a harness carrying the canonical skill (with two companions), the scoped
/// rule, and the fieldless rule under the real `.claude/` layout.
fn write_harness(root: &Path) {
    let coordinate = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(coordinate.join("scripts")).unwrap();
    fs::write(coordinate.join("SKILL.md"), CANONICAL_SKILL).unwrap();
    fs::write(coordinate.join("PLAYBOOK.md"), PLAYBOOK).unwrap();
    fs::write(coordinate.join("scripts").join("run.sh"), SCRIPT).unwrap();

    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), SCOPED_RULE).unwrap();
    fs::write(rules.join("collaboration.md"), FIELDLESS_RULE).unwrap();
}

/// Import `harness` into a fresh surface and return `(harness, into)`.
fn imported(label: &str) -> (PathBuf, PathBuf) {
    let harness = tmpdir(&format!("{label}-src"));
    write_harness(&harness);
    let into = tmpdir(&format!("{label}-into"));
    import::run(&harness, &into).unwrap();
    (harness, into)
}

/// Emit the surface at `into` back out to its harness sources.
fn emit(into: &Path) {
    let ws = Workspace::load(into).unwrap();
    drift::emit(&ws, into, EmitOptions::default()).unwrap();
}

/// `emit` re-emits each frontmatter in a fixed order — typed scalars in declaration
/// order, then the unknown keys sorted — as compact JSON-flow YAML. Snapshotting the
/// projected skill and rule sources pins that ordering: the generic adapter must
/// reproduce these bytes.
#[test]
fn projected_yaml_key_order_is_stable() {
    let (harness, into) = imported("yaml-order");
    emit(&into);

    let skill = fs::read_to_string(skill_source(&harness)).unwrap();
    insta::assert_snapshot!("projected_skill_source", skill);

    let rule = fs::read_to_string(harness.join(".claude").join("rules").join("rust.md")).unwrap();
    insta::assert_snapshot!("projected_scoped_rule_source", rule);
}

/// A source already in the adapter's canonical projection form is a fixpoint: a full
/// import→apply→import→apply cycle leaves every harness source byte-identical — the
/// two adapter faces compose to the identity on canonical input.
#[test]
fn import_apply_round_trip_is_a_byte_fixpoint() {
    let (harness, into) = imported("fixpoint");

    // The canonical sources are already what the emit face renders, so the first
    // apply changes not a byte.
    let before = tree_bytes(&harness);
    emit(&into);
    assert_eq!(
        before,
        tree_bytes(&harness),
        "a canonical source must survive the first apply byte-for-byte"
    );

    // And a second full round trip through a fresh surface is still the identity.
    let into2 = tmpdir("fixpoint-into2");
    import::run(&harness, &into2).unwrap();
    emit(&into2);
    assert_eq!(
        before,
        tree_bytes(&harness),
        "import→apply→import→apply must reach the same byte fixpoint"
    );
}

/// An unknown frontmatter key the harness ignores (`team:` on a skill, a Cursor
/// `description:` on a rule) survives import and lands back verbatim in the projected
/// YAML — extraction is permissive, the round trip lossless.
#[test]
fn unknown_frontmatter_keys_are_preserved_verbatim() {
    let (harness, into) = imported("unknown-keys");
    emit(&into);

    let skill = fs::read_to_string(skill_source(&harness)).unwrap();
    assert!(
        skill.contains("team: [\"platform\",\"infra\"]\n"),
        "the unknown skill key must round-trip verbatim, got:\n{skill}"
    );

    let rule = fs::read_to_string(harness.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(
        rule.contains("description: \"A Cursor key Claude Code ignores.\"\n"),
        "the unknown rule key must round-trip verbatim, got:\n{rule}"
    );
}

/// A rule that declares no fields projects to its body alone — no empty `---`
/// frontmatter block — and the body is byte-faithful.
#[test]
fn a_fieldless_rule_projects_with_no_frontmatter() {
    let (harness, into) = imported("fieldless");
    emit(&into);

    let projected = fs::read_to_string(fieldless_rule_source(&harness)).unwrap();
    assert!(
        !projected.starts_with("---"),
        "a fieldless rule must project no frontmatter block, got:\n{projected}"
    );
    assert_eq!(
        projected, FIELDLESS_RULE,
        "the body must project byte-faithful"
    );
}

/// A skill's companion files are copied byte-for-byte, never re-rendered — on the
/// import face (into the surface) and left untouched at the source on the apply face.
#[test]
fn skill_companions_are_copied_byte_for_byte() {
    let (harness, into) = imported("companions");

    // Import face: the companions land in the surface skill dir verbatim.
    let surface = into.join("skills").join("coordinate");
    assert_eq!(
        fs::read(surface.join("PLAYBOOK.md")).unwrap(),
        PLAYBOOK.as_bytes()
    );
    assert_eq!(
        fs::read(surface.join("scripts").join("run.sh")).unwrap(),
        SCRIPT.as_bytes()
    );

    // Apply face: re-emitting the member document leaves the source companions
    // untouched — the emit face owns only the frontmatter, never the companions.
    emit(&into);
    let source_dir = harness.join(".claude").join("skills").join("coordinate");
    assert_eq!(
        fs::read(source_dir.join("PLAYBOOK.md")).unwrap(),
        PLAYBOOK.as_bytes()
    );
    assert_eq!(
        fs::read(source_dir.join("scripts").join("run.sh")).unwrap(),
        SCRIPT.as_bytes()
    );
}
