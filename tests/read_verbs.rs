//! End-to-end proofs over the read family — `why` and `requirements`, the two
//! read-only traversals of the requirement↔`satisfies` edge (`specs/20-surface.md`,
//! "Decision: the CLI gains a read family — `why` and `requirements`").
//!
//! Drives the built `temper` binary over a fixture surface so the whole path is
//! pinned: `temper.toml` discovery at the project root, the surface at `./.temper`,
//! and the narration each verb prints to stdout. The cases mirror the entry's
//! acceptance (READ-EDGE-UNIFY — the edge walk ranges over the gate's resolved edge
//! set, a `[[kind.rule.relationships]]` `routes_to` edge over an extracted field, never
//! an `[edge.*]` document clause):
//! - `why <rule>` narrates a member's filled requirements (with rationale) + the
//!   governing package + its **outgoing** resolved edge to the skill;
//! - `why <skill>` narrates the **incoming** resolved edge from the rule (the same edge
//!   `graph::check` resolves);
//! - `why <member>` over a member touching no edge stays silent on both directions;
//! - `requirements` lists the roster forward with each requirement's coverage state;
//! - `requirements <name>` walks it in reverse — the satisfier set and the blast
//!   radius a removal would strand;
//! - every invocation, including a name no member/requirement bears, **exits zero**:
//!   the read family is never a gate.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::document::Satisfies;
use temper::frontmatter::Member;

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
/// the given authored `satisfies` (name + optional rationale) clauses — exactly as a
/// human editing the member document would, via the same projection the tool uses.
/// `satisfies` is surface-authored, never imported, so it is set on the reloaded IR
/// here. Edges are **not** authored on the skill: in the resolved-edge model the read
/// family narrates (READ-EDGE-UNIFY), an edge is a `[[kind.<name>.relationships]]`
/// declaration over an extracted field, not a per-member `[edge.*]` clause.
fn write_skill(root: &Path, name: &str, satisfies: &[(&str, Option<&str>)]) {
    let src = tmpdir("skill-src");
    fs::write(src.join("SKILL.md"), clean_skill(name)).unwrap();
    let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
    let mut skill = Member::from_source(&skill_kind, &src.join("SKILL.md")).unwrap();
    skill.satisfies = satisfies
        .iter()
        .map(|(req, rationale)| Satisfies {
            requirement: (*req).to_string(),
            rationale: rationale.map(str::to_string),
        })
        .collect();

    let dir = root.join(".temper").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill.to_document().emit()).unwrap();
}

/// Project a rule onto the surface at `<root>/.temper/rules/<name>/RULE.md` with the
/// given authored `satisfies` clauses and an optional `routes_to` reference field — the
/// rule-kind mirror of [`write_skill`]. `routes_to` is authored as an **extracted
/// frontmatter field** on the source (it round-trips into the surface as a
/// `[clause.routes_to]` module and back out into `Features`), the source side of the
/// `[[kind.rule.relationships]]` edge the gate — and now the read family — resolve
/// over (READ-EDGE-UNIFY). It is *not* an `[edge.*]` document clause.
fn write_rule(
    root: &Path,
    name: &str,
    satisfies: &[(&str, Option<&str>)],
    routes_to: Option<&str>,
) {
    let src = tmpdir("rule-src");
    let path = src.join(format!("{name}.md"));
    let routes = routes_to.map_or_else(String::new, |target| format!("routes_to: {target}\n"));
    fs::write(
        &path,
        format!("---\npaths:\n  - \"src/**/*.rs\"\n{routes}---\n# rule\n\nBody.\n"),
    )
    .unwrap();
    let rule_kind = temper::builtin_kind::definition("rule").unwrap().unwrap();
    let mut rule = Member::from_source(&rule_kind, &path).unwrap();
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

/// The `temper.toml` the fixture surface is read against: the requirement roster — one
/// required requirement filled by a single skill (load-bearing), one filled by a rule,
/// and one advisory requirement left unfilled — plus a `[[kind.rule.relationships]]`
/// declaration (`routes_to`, a rule → skill edge). The relationship + the source's
/// extracted `routes_to` field are the edge set `graph::check` resolves, and the read
/// family narrates that same set (READ-EDGE-UNIFY) — never an `[edge.*]` document clause.
const TEMPER_TOML: &str = "\
[requirement.collaboration-discipline]
means = \"pushback is the point\"
required = true

[requirement.engineering-standards]
means = \"the harness maintains development standards\"
required = true

[requirement.nice-to-have]
means = \"an optional convenience\"

[[kind.rule.relationships]]
field = \"routes_to\"
to = \"skill\"
";

/// Build the shared fixture surface + assembly under a fresh root and return it. Two
/// skills (`dev-standards` fills `engineering-standards` and is *pointed at* by the
/// rule; `lint-runner` fills nothing and touches no edge) and a rule (`collaboration`
/// fills `collaboration-discipline` and `routes_to` the skill `dev-standards`). The
/// edge is a `[[kind.rule.relationships]]` declaration over `collaboration`'s extracted
/// `routes_to` field — the same edge `graph::check` resolves — so `why` narrates the
/// outgoing edge on the rule and the incoming one on the skill from that one set.
fn fixture() -> PathBuf {
    let root = tmpdir("root");
    write_skill(
        &root,
        "dev-standards",
        &[(
            "engineering-standards",
            Some("the home for engineering-standards enforcement"),
        )],
    );
    write_skill(&root, "lint-runner", &[]);
    write_rule(
        &root,
        "collaboration",
        &[("collaboration-discipline", Some("pushback is the point"))],
        Some("dev-standards"),
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
fn why_narrates_a_skill_and_its_incoming_resolved_edge() {
    let root = fixture();
    // `dev-standards` fills `engineering-standards` and is *pointed at* by the rule
    // `collaboration`'s `routes_to` edge — so `why` narrates the filled requirement,
    // the governing package, and the **incoming** edge from the exact set
    // `graph::check` resolves.
    let run = read(&root, &["why", "dev-standards"]);
    // A read, never a gate — it exits zero.
    assert!(run.ok, "why must exit zero: {}", run.stdout);
    insta::assert_snapshot!("why_dev_standards", run.stdout);
}

#[test]
fn why_narrates_a_rule_and_its_outgoing_resolved_edge() {
    let root = fixture();
    // `collaboration` fills `collaboration-discipline` and `routes_to` the skill
    // `dev-standards` — so `why` narrates the **outgoing** edge, the same
    // `[[kind.rule.relationships]]` edge `graph::check` resolves over the extracted
    // `routes_to` field (never an `[edge.*]` clause).
    let run = read(&root, &["why", "collaboration"]);
    assert!(run.ok, "why must exit zero: {}", run.stdout);
    insta::assert_snapshot!("why_collaboration", run.stdout);
}

#[test]
fn why_stays_silent_when_a_member_touches_no_edge() {
    let root = fixture();
    // `lint-runner` fills nothing and neither declares nor receives a resolved edge, so
    // both edge directions read silent — the no-declared-edge case.
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

/// The `temper.toml` for the custom-kind fixture: a `required` requirement plus a
/// `[kind.spec]` registration binding the `spec` package. A custom-kind member fills
/// the requirement, so the read family must range over it (READ-CUSTOM-SATISFIERS) —
/// the silent under-report the verbs exist to prevent.
const CUSTOM_TEMPER_TOML: &str = "\
[requirement.intent-encoded]
means = \"the corpus carries a north-star intent spec\"
required = true

[kind.spec]
package = \"spec\"
";

/// The authored `spec` KIND.md definition (`specs/20-surface.md`, "Decision: a kind
/// definition is `KIND.md`"): it governs `specs/*.md` and composes the markdown-only
/// spec extractor. The read family loads its members off this locus.
const SPEC_KIND_MD: &str = "+++\n\
governs = { root = \"specs\", glob = \"*.md\" }\n\
\n\
[[extraction]]\n\
primitive = \"line_count\"\n\
\n\
[[extraction]]\n\
primitive = \"headings\"\n\
\n\
[[extraction]]\n\
primitive = \"placement\"\n\
+++\n\
\n\
# The spec kind\n\
\n\
temper's own governing documents.\n";

/// Author a `spec` custom-kind member surface at `<root>/.temper/specs/<name>/SPEC.md`
/// carrying a `[satisfies.<requirement>]` opt-in (with its authored rationale) over the
/// provenance-only header `import` writes. Written directly as the member document a
/// human authors — the surface language over which the read family walks the forward
/// `satisfies` edge.
fn write_spec(root: &Path, name: &str, satisfies: &str, rationale: &str) {
    let dir = root.join(".temper").join("specs").join(name);
    fs::create_dir_all(&dir).unwrap();
    let document = format!(
        "+++\n\
         [satisfies.{satisfies}]\n\
         rationale = \"{rationale}\"\n\
         \n\
         [provenance]\n\
         source_path = \"specs/{name}.md\"\n\
         import_hash = \"deadbeef\"\n\
         +++\n\
         # {name}\n\
         \n\
         Body.\n"
    );
    fs::write(dir.join("SPEC.md"), document).unwrap();
}

/// Build a fixture whose custom-kind (`spec`) member fills an assembly-published
/// requirement: the `spec` kind registered + defined, and one member `00-intent`
/// declaring `[satisfies.intent-encoded]` with a rationale. No skill or rule fills the
/// requirement, so the custom member is the *only* satisfier — proof the read family
/// counts it rather than under-reporting.
fn custom_fixture() -> PathBuf {
    let root = tmpdir("custom-root");
    let kind_dir = root.join(".temper").join("kinds").join("spec");
    fs::create_dir_all(&kind_dir).unwrap();
    fs::write(kind_dir.join("KIND.md"), SPEC_KIND_MD).unwrap();
    write_spec(
        &root,
        "00-intent",
        "intent-encoded",
        "the north star spec that encodes the requirement",
    );
    fs::write(root.join("temper.toml"), CUSTOM_TEMPER_TOML).unwrap();
    root
}

#[test]
fn why_narrates_a_custom_kind_member_and_its_satisfies() {
    let root = custom_fixture();
    // `00-intent` is a `spec` (a custom kind), and it fills `intent-encoded` — so
    // `why` narrates the satisfied requirement with its authored rationale and the
    // `spec` package its kind binds, exactly as it does for a skill/rule
    // (READ-CUSTOM-SATISFIERS). A custom member is no longer silently absent.
    let run = read(&root, &["why", "00-intent"]);
    assert!(run.ok, "why must exit zero: {}", run.stdout);
    insta::assert_snapshot!("why_custom_spec_member", run.stdout);
}

#[test]
fn requirements_counts_a_custom_kind_satisfier() {
    let root = custom_fixture();
    // The reverse walk over `intent-encoded` must list the `spec` member in its
    // satisfier set and count it in coverage (required, filled by one) — the custom
    // satisfier is in the set, not under-reported (READ-CUSTOM-SATISFIERS).
    let run = read(&root, &["requirements", "intent-encoded"]);
    assert!(run.ok, "requirements <name> must exit zero: {}", run.stdout);
    insta::assert_snapshot!("requirements_custom_satisfier", run.stdout);
}
