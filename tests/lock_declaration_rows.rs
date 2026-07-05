//! The lock's declaration-row family — the composed program's erased declarations
//! (`specs/architecture/20-surface.md`, "The lock and drift — one vocabulary").
//!
//! `emit`/`import` writes a declaration-row family (kind facts, clauses, requirements —
//! including the set-scope `count`/`unique`/`membership`/`degree` facets — assembly
//! facts, and the member→requirement `satisfies` family) beside the existing
//! provenance + emit-fingerprint rows, and the drift/gate side reads it back through
//! [`temper::drift::read_declarations`]. These tests assert the family is present and
//! populated, that a double `import` is byte-stable — the round-trip law 5 pins — and
//! that a bare harness (no `temper.toml`) still round-trips.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use temper::drift;
use temper::import;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "lock-declaration-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

const SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
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

/// A `temper.toml` declaring a surface-authority posture, a `required` requirement, a
/// second requirement exercising every set-scope facet (`count`/`unique`/`membership`/
/// `degree`), and an in-place member that opts into both via `satisfies` — so the
/// requirement and satisfies families carry more than the bare-harness minimum.
const TEMPER_TOML: &str = "authority = \"surface\"\n\
\n\
[requirement.review-coverage]\n\
means = \"Every shipped diff is reviewed before commit.\"\n\
kind = \"skill\"\n\
required = true\n\
\n\
[requirement.roster-coverage]\n\
kind = \"skill\"\n\
count = { min = 1, max = 2 }\n\
unique = [\"name\"]\n\
membership = { field = \"name\", kind = \"skill\", source = \"review-coverage\", feature = \"name\" }\n\
degree = { incoming = { min = 1 }, outgoing = { max = 3 } }\n\
\n\
[[member]]\n\
kind = \"skill\"\n\
name = \"coordinate\"\n\
source = \".claude/skills/coordinate/SKILL.md\"\n\
satisfies = [\"review-coverage\", \"roster-coverage\"]\n";

/// Write a skill + rule harness carrying a `temper.toml`, then import it into a fresh
/// surface, returning `(harness, into)` — the harness kept so a re-import reads the same
/// absolute sources the lock records.
fn imported(label: &str) -> (PathBuf, PathBuf) {
    let harness = tmpdir(&format!("{label}-src"));
    let skill = harness.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), RULE).unwrap();
    fs::write(harness.join("temper.toml"), TEMPER_TOML).unwrap();

    let into = tmpdir(&format!("{label}-into"));
    import::run(&harness, &into).unwrap();
    (harness, into)
}

#[test]
fn lock_carries_all_four_declaration_families() {
    let (_harness, into) = imported("families");
    let declarations = drift::read_declarations(&into).unwrap();

    // Kind facts: one per member-discovering built-in kind, name-sorted, carrying the
    // declared runtime facts.
    let skill = declarations
        .kinds
        .iter()
        .find(|k| k.name == "skill")
        .expect("the skill kind fact is recorded");
    assert_eq!(skill.provider.as_deref(), Some("claude-code"));
    assert_eq!(skill.governs_root, ".claude/skills");
    assert_eq!(skill.governs_glob, "*/SKILL.md");
    assert_eq!(skill.format.as_deref(), Some("yaml-frontmatter"));
    assert_eq!(skill.unit_shape.as_deref(), Some("directory"));
    assert_eq!(
        skill.activation.as_deref(),
        Some("description-trigger(description)")
    );
    assert!(
        declarations.kinds.iter().any(|k| k.name == "rule"),
        "the rule kind fact is recorded"
    );

    // Clauses: the built-in floor contract's clauses, keyed by kind.
    assert!(
        !declarations.clauses.is_empty(),
        "the floor clauses are recorded"
    );
    assert!(
        declarations.clauses.iter().any(|c| c.kind == "skill"),
        "skill floor clauses are keyed by kind"
    );
    for clause in &declarations.clauses {
        assert!(
            matches!(clause.severity.as_str(), "required" | "advisory"),
            "a clause severity is one of the declared vocabulary, got {:?}",
            clause.severity
        );
    }

    // Requirements: the assembly's `[requirement.*]` obligations.
    let requirement = declarations
        .requirements
        .iter()
        .find(|r| r.name == "review-coverage")
        .expect("the declared requirement is recorded");
    assert_eq!(requirement.kind.as_deref(), Some("skill"));
    assert!(requirement.required);

    // The set-scope facets: count/unique/membership/degree all carried on the row.
    let roster = declarations
        .requirements
        .iter()
        .find(|r| r.name == "roster-coverage")
        .expect("the set-scope requirement is recorded");
    let count = roster.count.expect("count bound is recorded");
    assert_eq!((count.min, count.max), (1, 2));
    assert_eq!(roster.unique, vec!["name".to_string()]);
    let membership = roster
        .membership
        .as_ref()
        .expect("membership predicate is recorded");
    assert_eq!(membership.field, "name");
    assert_eq!(membership.source_kind, "skill");
    assert_eq!(membership.source, "review-coverage");
    assert_eq!(membership.source_feature, "name");
    assert_eq!(membership.source_package, None);
    let degree = roster.degree.expect("degree bound is recorded");
    assert_eq!(degree.incoming.expect("incoming bound").min, Some(1));
    assert_eq!(degree.incoming.expect("incoming bound").max, None);
    assert_eq!(degree.outgoing.expect("outgoing bound").max, Some(3));
    assert_eq!(degree.outgoing.expect("outgoing bound").min, None);

    // Satisfies: the in-place member's declared fill keys, one row per key.
    let mut satisfied: Vec<&str> = declarations
        .satisfies
        .iter()
        .filter(|row| row.member == "coordinate")
        .map(|row| row.requirement.as_str())
        .collect();
    satisfied.sort_unstable();
    assert_eq!(satisfied, vec!["review-coverage", "roster-coverage"]);

    // Assembly facts: the surface-authority posture the assembly declared.
    let authority = declarations
        .assembly
        .iter()
        .find(|f| f.fact == "authority")
        .expect("the authority fact is recorded");
    assert_eq!(authority.value.as_deref(), Some("surface"));
}

#[test]
fn a_double_import_is_byte_stable() {
    let (harness, into) = imported("byte-stable");
    let lock = into.join("lock.toml");
    let first = fs::read(&lock).unwrap();

    // The declaration rows are a pure function of the same extraction, so re-importing the
    // same harness reproduces the whole lock byte-for-byte (law 5; import idempotence).
    import::run(&harness, &into).unwrap();
    let second = fs::read(&lock).unwrap();
    assert_eq!(first, second, "a re-import must not churn the lock");

    // The declaration table survived the round-trip: reading it back yields the same
    // populated families.
    let declarations = drift::read_declarations(&into).unwrap();
    assert!(!declarations.kinds.is_empty());
    assert!(!declarations.clauses.is_empty());
    assert!(!declarations.requirements.is_empty());
    assert!(!declarations.assembly.is_empty());
    assert!(!declarations.satisfies.is_empty());
}

/// A harness with no `temper.toml` at all (no requirements, no members) still imports
/// and round-trips: the requirement/satisfies families are simply empty, never an error
/// or a malformed row — the bootstrap's tolerant-read discipline extends to the new
/// facets exactly as it does the existing ones.
#[test]
fn a_bare_harness_lock_still_round_trips() {
    let harness = tmpdir("bare-src");
    let skill = harness.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), RULE).unwrap();

    let into = tmpdir("bare-into");
    import::run(&harness, &into).unwrap();

    let declarations = drift::read_declarations(&into).unwrap();
    assert!(!declarations.kinds.is_empty());
    assert!(!declarations.clauses.is_empty());
    assert!(declarations.requirements.is_empty());
    assert!(declarations.satisfies.is_empty());
}

/// A workspace with no `[declaration]` table (any pre-recut lock) reads back an empty
/// declaration set rather than erroring — absent evidence forges no finding.
#[test]
fn a_lock_without_declarations_reads_empty() {
    let dir = tmpdir("no-declarations");
    fs::write(
        dir.join("lock.toml"),
        "[[skill]]\nname = \"x\"\nsource_path = \"/h/SKILL.md\"\nsource_hash = \"abc\"\nemit_hash = \"abc\"\n",
    )
    .unwrap();

    let declarations = drift::read_declarations(&dir).unwrap();
    assert_eq!(declarations, drift::Declarations::default());
}

/// A missing lock is the pre-import state, not an error.
#[test]
fn a_missing_lock_reads_empty() {
    let dir: &Path = &tmpdir("missing-lock");
    let declarations = drift::read_declarations(dir).unwrap();
    assert_eq!(declarations, drift::Declarations::default());
}
