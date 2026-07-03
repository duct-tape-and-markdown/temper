//! `temper re-add` — the on-disk → surface direction, the third drift direction
//! (`specs/architecture/20-surface.md`, "Drift / apply — three states, never two"; "the surface
//! is the source of truth", where `re-add` keeps direct on-disk editing
//! first-class).
//!
//! Drives the library `drift::re_add` over a real imported surface and proves the
//! three properties the entry names, across both built-in kinds:
//!
//! - **drifted → reconciled** — a source edited straight on disk is pulled back into
//!   the surface (its member document rewritten) and its lock row's
//!   fingerprints are refreshed to the current source bytes;
//! - **added → new artifact** — an on-disk source the surface never imported gains a
//!   surface directory and a lock row;
//! - **in-sync → no-op** — an unchanged harness leaves every surface byte identical.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use temper::check::Workspace;
use temper::document::{self, Document, PublishedRequirement, Satisfies};
use temper::drift::{self, ReAddOutcome};
use temper::frontmatter::Member;
use temper::import;
use temper::kind::{CustomKind, Unit};
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
license: \"MIT\"\n\
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
    harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md")
}

fn skill_dir(harness: &Path) -> PathBuf {
    harness.join(".claude").join("skills").join("coordinate")
}

fn rule_source(harness: &Path) -> PathBuf {
    harness.join(".claude").join("rules").join("rust.md")
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
    let report = drift::re_add(&ws, &into, &harness, &[]).unwrap();

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
license: \"Apache-2.0\"\n\
---\n\
# Coordinate\n\
\n\
An edited body, straight on disk.\n";
    fs::write(skill_source(&harness), drifted).unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::re_add(&ws, &into, &harness, &[]).unwrap();
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Reconciled);
    // The untouched rule stays in sync.
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Unchanged);

    // The surface header was rewritten: the reloaded skill carries the edited field.
    let surface = into.join("skills").join("coordinate");
    let reloaded = Member::from_surface(&surface, "SKILL.md").unwrap();
    assert_eq!(
        reloaded
            .field("description")
            .and_then(|v| v.as_str())
            .unwrap(),
        "Edited straight on disk, outside the surface."
    );
    assert_eq!(
        reloaded.field("license").and_then(|v| v.as_str()),
        Some("Apache-2.0")
    );
    // ...and the body was pulled in byte-faithfully (frontmatter stripped), below
    // the member document's header.
    assert_eq!(
        reloaded.body,
        "# Coordinate\n\nAn edited body, straight on disk.\n"
    );

    // The lock fingerprints were refreshed to the current source bytes: the drift
    // anchor now hashes the edited file, and `last_applied` tracks it — both
    // different from the pre-drift baseline. The fresh hash matches what a source
    // re-parse computes, so the lock truly reflects on-disk reality.
    let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
    let fresh = Member::from_source(&skill_kind, &skill_dir(&harness).join("SKILL.md"))
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
    let report = drift::re_add(&ws, &into, &harness, &[]).unwrap();
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Reconciled);
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Unchanged);

    // The surface header carries the broadened scope, and the body is byte-faithful.
    let surface = into.join("rules").join("rust");
    let reloaded = Member::from_surface(&surface, "RULE.md").unwrap();
    assert_eq!(
        reloaded.field("paths"),
        Some(&serde_json::json!(["src/**/*.rs", "tests/**/*.rs"]))
    );
    assert_eq!(
        reloaded.body,
        "# Rust conventions\n\nAn edited rule body.\n"
    );

    // The lock anchor is refreshed to the edited source bytes.
    let rule_kind = temper::builtin_kind::definition("rule").unwrap().unwrap();
    let fresh = Member::from_source(&rule_kind, &rule_source(&harness))
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

    // Author the surface-only layer on both kinds — `satisfies` and its `rationale`
    // the source files never carry.
    let skill_surface = into.join("skills").join("coordinate");
    let mut skill = Member::from_surface(&skill_surface, "SKILL.md").unwrap();
    skill.satisfies = vec![Satisfies {
        requirement: "req.coordinate".to_string(),
        rationale: Some("Fills the coordination requirement.".to_string()),
    }];
    fs::write(skill_surface.join("SKILL.md"), skill.to_document().emit()).unwrap();

    let rule_surface = into.join("rules").join("rust");
    let mut rule = Member::from_surface(&rule_surface, "RULE.md").unwrap();
    rule.satisfies = vec![Satisfies {
        requirement: "req.rust-style".to_string(),
        rationale: Some("Encodes the Rust conventions the gate enforces.".to_string()),
    }];
    fs::write(rule_surface.join("RULE.md"), rule.to_document().emit()).unwrap();

    // Drift only the bodies on disk, so `re-add` genuinely rebuilds the member
    // document from source — the path that clobbers the authored layer today.
    fs::write(
        skill_source(&harness),
        "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes.\n\
license: \"MIT\"\n\
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
    let report = drift::re_add(&ws, &into, &harness, &[]).unwrap();
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Reconciled);
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Reconciled);

    // The authored layer survived the body-drift re-add — the data-loss the carry
    // prevents (satisfies + rationale are wiped without it).
    let skill = Member::from_surface(&skill_surface, "SKILL.md").unwrap();
    assert_eq!(skill.satisfies[0].requirement, "req.coordinate");
    assert_eq!(
        skill.satisfies[0].rationale.as_deref(),
        Some("Fills the coordination requirement.")
    );
    // ...and the drifted body was still pulled in byte-faithfully.
    assert_eq!(
        skill.body,
        "# Coordinate\n\nAn edited body, straight on disk.\n"
    );

    let rule = Member::from_surface(&rule_surface, "RULE.md").unwrap();
    assert_eq!(rule.satisfies[0].requirement, "req.rust-style");
    assert_eq!(
        rule.satisfies[0].rationale.as_deref(),
        Some("Encodes the Rust conventions the gate enforces.")
    );
    assert_eq!(rule.body, "# Rust conventions\n\nAn edited rule body.\n");
}

#[test]
fn a_reimport_of_an_authored_surface_preserves_representation_and_is_idempotent() {
    let (harness, into) = imported("rep-reimport");

    // Author the layer on the surface, then re-import the *unchanged* harness.
    let skill_surface = into.join("skills").join("coordinate");
    let mut skill = Member::from_surface(&skill_surface, "SKILL.md").unwrap();
    skill.satisfies = vec![Satisfies {
        requirement: "req.coordinate".to_string(),
        rationale: Some("Fills the coordination requirement.".to_string()),
    }];
    fs::write(skill_surface.join("SKILL.md"), skill.to_document().emit()).unwrap();

    let before = tree_bytes(&into);
    // A re-import rebuilds every member document from source; carrying the surface's
    // authored layer forward keeps it — and the workspace byte-identical.
    import::run(&harness, &into).unwrap();

    assert_eq!(
        before,
        tree_bytes(&into),
        "re-importing an authored, unchanged surface must not change a byte"
    );
    let skill = Member::from_surface(&skill_surface, "SKILL.md").unwrap();
    assert_eq!(skill.satisfies[0].requirement, "req.coordinate");
    assert_eq!(
        skill.satisfies[0].rationale.as_deref(),
        Some("Fills the coordination requirement.")
    );
}

#[test]
fn an_added_source_becomes_a_new_surface_artifact_and_lock_row() {
    let (harness, into) = imported("added");

    // A skill and a rule that live on disk but the surface never imported.
    let helper = harness.join(".claude").join("skills").join("helper");
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
    let report = drift::re_add(&ws, &into, &harness, &[]).unwrap();

    // The new sources are added; the pre-existing artifacts stay in sync.
    assert_eq!(outcome(&report, "helper"), ReAddOutcome::Added);
    assert_eq!(outcome(&report, "extra"), ReAddOutcome::Added);
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Unchanged);
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Unchanged);

    // Each added source gained a surface directory that reloads through its kind's
    // loader — a first-class surface artifact, not a partial write.
    let skill_surface = into.join("skills").join("helper");
    assert!(skill_surface.join("SKILL.md").is_file());
    let reloaded = Member::from_surface(&skill_surface, "SKILL.md").unwrap();
    assert_eq!(reloaded.id, "helper");
    assert_eq!(reloaded.body, "# Helper\n\nA helping hand.\n");

    let rule_surface = into.join("rules").join("extra");
    assert!(rule_surface.join("RULE.md").is_file());
    assert_eq!(
        Member::from_surface(&rule_surface, "RULE.md").unwrap().body,
        "# Extra\n\nA rule added straight to the harness, after import.\n"
    );

    // Each added source gained a lock row anchored to its source bytes.
    assert!(lock_has_row(&into, "skill", "helper"));
    assert!(lock_has_row(&into, "rule", "extra"));
    let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
    let fresh = Member::from_source(&skill_kind, &helper.join("SKILL.md"))
        .unwrap()
        .provenance
        .import_hash;
    assert_eq!(lock_field(&into, "skill", "helper", "import_hash"), fresh);
    assert_eq!(lock_field(&into, "skill", "helper", "last_applied"), fresh);
    // The original rows survive alongside the new ones.
    assert!(lock_has_row(&into, "skill", "coordinate"));
    assert!(lock_has_row(&into, "rule", "rust"));
}

/// A `temper.toml` registering `spec` as a custom kind over a `governs` locus, plus
/// the authored `KIND.md` definition discovery keys on (`specs/architecture/40-composition.md`).
const SPEC_TEMPER_TOML: &str = "[kind.spec]\npackage = \"spec\"\n";
const SPEC_KIND_MD: &str = "+++\n\
governs = { root = \"specs\", glob = \"*.md\" }\n\
\n\
[[extraction]]\n\
primitive = \"headings\"\n\
+++\n\
\n\
# The spec kind\n\
\n\
temper's own governing documents.\n";

/// The two spec bodies the harness imports as its custom-kind baseline — a spec has
/// no frontmatter, so the whole file is the byte-faithful body.
const INTENT_SPEC: &str = "# Intent\n\nThe north star.\n";
const SURFACE_SPEC: &str = "# The config surface\n\nThe composition write surface.\n";

/// Build a harness carrying the built-in skill + rule *and* a registered `spec`
/// custom kind over `specs/`, import it, and return `(harness, into, custom_kinds)`
/// — the custom-kind definitions threaded into `diff`/`re_add`.
fn imported_with_spec(label: &str) -> (PathBuf, PathBuf, Vec<CustomKind>) {
    let harness = tmpdir(&format!("{label}-src"));
    let skill = harness.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), RULE).unwrap();

    // Register the `spec` kind and author its definition beside the harness.
    fs::write(harness.join("temper.toml"), SPEC_TEMPER_TOML).unwrap();
    let kind_dir = harness.join(".temper").join("kinds").join("spec");
    fs::create_dir_all(&kind_dir).unwrap();
    fs::write(kind_dir.join("KIND.md"), SPEC_KIND_MD).unwrap();

    let specs = harness.join("specs");
    fs::create_dir_all(&specs).unwrap();
    fs::write(specs.join("00-intent.md"), INTENT_SPEC).unwrap();
    fs::write(specs.join("20-surface.md"), SURFACE_SPEC).unwrap();

    let into = tmpdir(&format!("{label}-into"));
    import::run(&harness, &into).unwrap();

    let kinds_dir = harness.join(".temper").join("kinds");
    let custom = vec![CustomKind::load(&kinds_dir, "spec").unwrap()];
    (harness, into, custom)
}

#[test]
fn a_drifted_custom_kind_unit_is_reconciled_into_the_surface() {
    let (harness, into, custom) = imported_with_spec("spec-drift");
    let before_hash = lock_field(&into, "spec", "20-surface", "import_hash");

    // A hand edit to a spec straight on disk — the law-8 dogfood case the gate
    // otherwise reads stale (`specs/architecture/20-surface.md`, the hard core).
    let edited = "# The config surface\n\nAn edit made straight in the spec file.\n";
    fs::write(harness.join("specs").join("20-surface.md"), edited).unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::re_add(&ws, &into, &harness, &custom).unwrap();

    // The edited spec reconciles; the untouched spec and the built-ins stay in sync.
    assert_eq!(outcome(&report, "20-surface"), ReAddOutcome::Reconciled);
    assert_eq!(outcome(&report, "00-intent"), ReAddOutcome::Unchanged);
    assert_eq!(outcome(&report, "coordinate"), ReAddOutcome::Unchanged);
    assert_eq!(outcome(&report, "rust"), ReAddOutcome::Unchanged);

    // The surface member document was rewritten: the whole file reloads as the unit
    // body byte-faithfully, no longer the stale pre-edit text.
    let surface = into.join("specs").join("20-surface");
    let unit = Unit::from_surface_dir(&surface).unwrap();
    assert_eq!(unit.id, "20-surface");
    assert_eq!(unit.body, edited);

    // The lock fingerprints track the current source bytes — the drift anchor and
    // `last_applied` both bumped off the pre-drift baseline to the fresh hash.
    let after_hash = lock_field(&into, "spec", "20-surface", "import_hash");
    assert_ne!(after_hash, before_hash, "the import_hash must be bumped");
    assert_eq!(
        after_hash,
        lock_field(&into, "spec", "20-surface", "last_applied"),
        "last_applied is reconciled to the current source"
    );
}

/// The `SPEC.md` surface document of the `20-surface` spec unit.
fn spec_surface_doc(into: &Path) -> PathBuf {
    into.join("specs").join("20-surface").join("SPEC.md")
}

/// Author the surface-only representation layer onto a custom-unit surface document
/// — a published `[requirement.*]` and a `[satisfies.*]` the spec *source* never
/// carries — in the canonical tables-first, provenance-last layout the tool emits,
/// so a re-import of the unchanged surface stays byte-stable. This is the 17-join
/// intent↔architecture trace's shape in miniature: hand-authored tables the
/// custom-unit re-import must carry forward, never clobber.
fn author_spec_layer(surface_doc: &Path) {
    let doc = Document::parse(&fs::read_to_string(surface_doc).unwrap()).unwrap();
    let (source_path, import_hash) = document::provenance(doc.header()).unwrap();
    let body = doc.body().to_string();

    let mut header = DocumentMut::new();
    document::add_requirement(
        &mut header,
        &PublishedRequirement {
            name: "member".to_string(),
            means: Some("the surface projects one document per member".to_string()),
            kind: Some("spec".to_string()),
            package: None,
            required: true,
        },
    );
    document::add_satisfies(
        &mut header,
        &Satisfies {
            requirement: "projection".to_string(),
            rationale: Some("20-surface owns the projection contract.".to_string()),
        },
    );
    document::add_provenance(&mut header, &source_path, &import_hash);
    fs::write(surface_doc, Document::new(header, body).emit()).unwrap();
}

/// The parsed header of a surface document at `path`.
fn header_of(path: &Path) -> DocumentMut {
    Document::parse(&fs::read_to_string(path).unwrap())
        .unwrap()
        .header()
        .clone()
}

/// Assert the hand-authored `member` requirement and `projection` satisfies survived
/// on `surface_doc` — the tables `author_spec_layer` wrote, intact after a re-import
/// or re-add rebuilt the member document from source.
fn assert_authored_layer_intact(surface_doc: &Path) {
    let header = header_of(surface_doc);
    let reqs = document::requirements(&header).unwrap();
    assert_eq!(
        reqs.len(),
        1,
        "the published requirement is carried forward"
    );
    assert_eq!(reqs[0].name, "member");
    assert_eq!(reqs[0].kind.as_deref(), Some("spec"));
    assert!(reqs[0].required);
    let sats = document::satisfies(&header);
    assert_eq!(sats.len(), 1, "the satisfies clause is carried forward");
    assert_eq!(sats[0].requirement, "projection");
    assert_eq!(
        sats[0].rationale.as_deref(),
        Some("20-surface owns the projection contract.")
    );
}

#[test]
fn a_drifted_custom_unit_readd_preserves_authored_representation() {
    let (harness, into, custom) = imported_with_spec("spec-rep-preserve");
    let surface_doc = spec_surface_doc(&into);
    author_spec_layer(&surface_doc);
    let before_hash = lock_field(&into, "spec", "20-surface", "import_hash");

    // Drift only the spec body on disk, so `re-add` genuinely rebuilds the member
    // document from source — the path that clobbers the authored tables today.
    let edited = "# The config surface\n\nAn edit made straight in the spec file.\n";
    fs::write(harness.join("specs").join("20-surface.md"), edited).unwrap();

    let ws = Workspace::load(&into).unwrap();
    let report = drift::re_add(&ws, &into, &harness, &custom).unwrap();
    assert_eq!(outcome(&report, "20-surface"), ReAddOutcome::Reconciled);

    // The drifted body was pulled in byte-faithfully...
    let unit = Unit::from_surface_dir(&into.join("specs").join("20-surface")).unwrap();
    assert_eq!(unit.body, edited);

    // ...while the hand-authored `[requirement.*]`/`[satisfies.*]` tables survived —
    // carried forward, not clobbered by a bare provenance header (the data loss the
    // carry prevents)...
    assert_authored_layer_intact(&surface_doc);

    // ...and only `[provenance]` was re-stamped to the drifted source's fresh hash,
    // which the lock row also tracks.
    let (_, new_hash) = document::provenance(&header_of(&surface_doc)).unwrap();
    assert_ne!(
        new_hash, before_hash,
        "provenance is re-stamped to the drifted body's hash"
    );
    assert_eq!(
        lock_field(&into, "spec", "20-surface", "import_hash"),
        new_hash,
        "the lock anchor agrees with the re-stamped surface provenance"
    );
}

#[test]
fn a_reimport_of_an_authored_custom_unit_preserves_representation_and_is_idempotent() {
    let (harness, into, _custom) = imported_with_spec("spec-rep-reimport");
    let surface_doc = spec_surface_doc(&into);
    author_spec_layer(&surface_doc);

    let before = tree_bytes(&into);
    // A re-import rebuilds every member document from source; carrying the custom
    // unit's authored tables forward keeps them — and the workspace byte-identical.
    import::run(&harness, &into).unwrap();

    assert_eq!(
        before,
        tree_bytes(&into),
        "re-importing an authored, unchanged custom-unit surface must not change a byte"
    );
    assert_authored_layer_intact(&surface_doc);
}

#[test]
fn an_unchanged_custom_kind_unit_is_a_noop() {
    let (harness, into, custom) = imported_with_spec("spec-clean");
    let before = tree_bytes(&into);

    let ws = Workspace::load(&into).unwrap();
    let report = drift::re_add(&ws, &into, &harness, &custom).unwrap();

    // No spec drifted, so every custom unit — and every built-in — is a no-op.
    assert_eq!(outcome(&report, "20-surface"), ReAddOutcome::Unchanged);
    assert_eq!(outcome(&report, "00-intent"), ReAddOutcome::Unchanged);
    assert_eq!(
        before,
        tree_bytes(&into),
        "an in-sync custom-kind re-add must leave every surface byte identical"
    );
}
