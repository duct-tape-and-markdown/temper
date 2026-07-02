//! End-to-end proofs over the read family — `why` and `requirements`, the two
//! read-only traversals of the requirement↔`satisfies` edge (`specs/20-surface.md`,
//! "Decision: the CLI gains a read family — `why` and `requirements`").
//!
//! Drives the built `temper` binary over a fixture surface so the whole path is
//! pinned: `temper.toml` discovery at the project root, the surface at `./.temper`,
//! and the narration each verb prints to stdout. The cases mirror the entry's
//! acceptance:
//! - `why <member>` narrates a member's filled requirements (with rationale) + the
//!   governing package + its edges out;
//! - `why <member>` over a bare member narrates its incoming edges and that it fills
//!   nothing;
//! - `requirements` lists the roster forward with each requirement's coverage state;
//! - `requirements <name>` walks it in reverse — the satisfier set and the blast
//!   radius a removal would strand;
//! - every invocation, including a name no member/requirement bears, **exits zero**:
//!   the read family is never a gate.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::document::{EdgeClause, Satisfies};
use temper::rule::Rule;
use temper::skill::Skill;

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-read-verbs-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A floor-clean skill source (name matches its dir, a lowercase slug, a present
/// description) — clean against the floor so the fixture is well-formed.
fn clean_skill(name: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

/// Project a skill onto the surface at `<root>/.temper/skills/<name>/SKILL.md` with
/// the given authored `satisfies` (name + optional rationale) and `edge` (target +
/// relation) clauses — exactly as a human editing the member document would, via the
/// same projection the tool uses. `satisfies`/`edges` are surface-authored, never
/// imported, so they are set on the reloaded IR here.
fn write_skill(
    root: &Path,
    name: &str,
    satisfies: &[(&str, Option<&str>)],
    edges: &[(&str, &str)],
) {
    let src = tmpdir("skill-src");
    fs::write(src.join("SKILL.md"), clean_skill(name)).unwrap();
    let mut skill = Skill::from_source_dir(&src).unwrap();
    skill.satisfies = satisfies
        .iter()
        .map(|(req, rationale)| Satisfies {
            requirement: (*req).to_string(),
            rationale: rationale.map(str::to_string),
        })
        .collect();
    skill.edges = edges
        .iter()
        .map(|(target, relation)| EdgeClause {
            target: (*target).to_string(),
            relation: Some((*relation).to_string()),
        })
        .collect();

    let dir = root.join(".temper").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill.to_document().emit()).unwrap();
}

/// Project a rule onto the surface at `<root>/.temper/rules/<name>/RULE.md` with the
/// given authored `satisfies` clauses — the rule-kind mirror of [`write_skill`].
fn write_rule(root: &Path, name: &str, satisfies: &[(&str, Option<&str>)]) {
    let src = tmpdir("rule-src");
    let path = src.join(format!("{name}.md"));
    fs::write(
        &path,
        "---\npaths:\n  - \"src/**/*.rs\"\n---\n# rule\n\nBody.\n",
    )
    .unwrap();
    let mut rule = Rule::from_source_file(&path).unwrap();
    rule.satisfies = satisfies
        .iter()
        .map(|(req, rationale)| Satisfies {
            requirement: (*req).to_string(),
            rationale: rationale.map(str::to_string),
        })
        .collect();

    let dir = root.join(".temper").join("rules").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("RULE.md"), rule.to_document().emit()).unwrap();
}

/// The `temper.toml` roster the fixture surface is read against: one required
/// requirement filled by a single skill (load-bearing), one filled by a rule, and one
/// advisory requirement left unfilled.
const TEMPER_TOML: &str = "\
[requirement.collaboration-discipline]
means = \"pushback is the point\"
required = true

[requirement.engineering-standards]
means = \"the harness maintains development standards\"
required = true

[requirement.nice-to-have]
means = \"an optional convenience\"
";

/// Build the shared fixture surface + assembly under a fresh root and return it. Two
/// skills (`dev-standards` fills `engineering-standards` and points at `lint-runner`;
/// `lint-runner` fills nothing) and a rule (`collaboration` fills
/// `collaboration-discipline`).
fn fixture() -> PathBuf {
    let root = tmpdir("root");
    write_skill(
        &root,
        "dev-standards",
        &[(
            "engineering-standards",
            Some("the home for engineering-standards enforcement"),
        )],
        &[("lint-runner", "depends-on")],
    );
    write_skill(&root, "lint-runner", &[], &[]);
    write_rule(
        &root,
        "collaboration",
        &[("collaboration-discipline", Some("pushback is the point"))],
    );
    fs::write(root.join("temper.toml"), TEMPER_TOML).unwrap();
    root
}

/// The outcome of a read invocation: whether it exited zero and its stdout narration.
struct Run {
    ok: bool,
    stdout: String,
}

/// Run a read verb from `root` (so its `temper.toml` and `./.temper` are discovered),
/// capturing the exit status and stdout narration.
fn read(root: &Path, args: &[&str]) -> Run {
    let out = Command::new(BIN)
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    Run {
        ok: out.status.success(),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
    }
}

#[test]
fn why_narrates_a_members_forward_walk() {
    let root = fixture();
    let run = read(&root, &["why", "dev-standards"]);
    // A read, never a gate — it exits zero.
    assert!(run.ok, "why must exit zero: {}", run.stdout);
    insta::assert_snapshot!("why_dev_standards", run.stdout);
}

#[test]
fn why_narrates_a_bare_member_and_its_incoming_edges() {
    let root = fixture();
    // `lint-runner` fills nothing and declares no edges, but `dev-standards` points at
    // it — so the reverse (incoming) edge is surfaced.
    let run = read(&root, &["why", "lint-runner"]);
    assert!(run.ok, "why must exit zero: {}", run.stdout);
    insta::assert_snapshot!("why_lint_runner", run.stdout);
}

#[test]
fn why_over_an_unknown_member_still_exits_zero() {
    let root = fixture();
    let run = read(&root, &["why", "ghost"]);
    assert!(run.ok, "why over an unknown member must exit zero");
    assert!(run.stdout.contains("No member named `ghost`"));
}

#[test]
fn requirements_lists_the_roster_forward() {
    let root = fixture();
    let run = read(&root, &["requirements"]);
    assert!(run.ok, "requirements must exit zero: {}", run.stdout);
    insta::assert_snapshot!("requirements_roster", run.stdout);
}

#[test]
fn requirements_walks_one_requirement_in_reverse_with_blast_radius() {
    let root = fixture();
    // `engineering-standards` is required and rests on the single satisfier
    // `dev-standards` — the load-bearing case the blast radius surfaces.
    let run = read(&root, &["requirements", "engineering-standards"]);
    assert!(run.ok, "requirements <name> must exit zero: {}", run.stdout);
    insta::assert_snapshot!("requirements_engineering_standards", run.stdout);
}

#[test]
fn requirements_over_an_undeclared_name_still_exits_zero() {
    let root = fixture();
    let run = read(&root, &["requirements", "ghost"]);
    assert!(
        run.ok,
        "requirements over an undeclared name must exit zero"
    );
    assert!(run.stdout.contains("No requirement named `ghost`"));
}
