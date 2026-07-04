//! End-to-end proofs over the read family — `why` and `requirements`, the two
//! read-only traversals of the requirement↔`satisfies` edge (`specs/architecture/20-surface.md`,
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

#[test]
fn impact_traces_the_blast_radius_of_a_load_bearing_member() {
    let root = fixture();
    // `dev-standards` is the *sole* satisfier of the required `engineering-standards`
    // (removing it fails the gate) and is the target of `collaboration`'s `routes_to`
    // reference — but `routes_to` is a reference edge, not an `@import` directive, so no
    // directive edge points at it and no member's reachability rides on it. So the
    // blast radius names the unfilled requirement and reads "none" on the other three
    // strands — the deterministic tier-1 traversal, narrated like `why`.
    let run = read(&root, &["impact", "dev-standards"]);
    assert!(run.ok, "impact must exit zero: {}", run.stdout);
    assert!(
        run.stdout.contains("fails the gate"),
        "the sole satisfier of a required requirement must surface as unfilled: {}",
        run.stdout
    );
    insta::assert_snapshot!("impact_dev_standards", run.stdout);
}

#[test]
fn impact_stays_quiet_for_a_member_nothing_rests_on() {
    let root = fixture();
    // `lint-runner` fills nothing, publishes nothing, is imported by nobody, and carries
    // no member's reachability — every strand reads "none", the clean floor case.
    let run = read(&root, &["impact", "lint-runner"]);
    assert!(run.ok, "impact must exit zero: {}", run.stdout);
    insta::assert_snapshot!("impact_lint_runner", run.stdout);
}

#[test]
fn impact_over_an_unknown_member_still_exits_zero() {
    let root = fixture();
    let run = read(&root, &["impact", "ghost"]);
    assert!(run.ok, "impact over an unknown member must exit zero");
    assert!(run.stdout.contains("No member named `ghost`"));
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

/// The authored `spec` KIND.md definition (`specs/architecture/20-surface.md`, "Decision: a kind
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
         source_hash = \"deadbeef\"\n\
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

/// The `temper.toml` for the member-published fixture: it declares **no**
/// `[requirement.*]` of its own — the whole roster is published on a surface member
/// (`00-intent` publishes `[requirement.governance]`). It only registers the `spec`
/// kind so its members load. Proof the read family ranges over the composed namespace
/// `check` gates (assembly ∪ member-published, READ-VERBS-PUBLISHED-DEMANDS), not the
/// empty assembly roster: a read blind to member-published demands shows nothing here.
const PUBLISHED_TEMPER_TOML: &str = "\
[kind.spec]
package = \"spec\"
";

/// Author a `spec` member that **publishes** a `[requirement.<name>]` demand — the
/// intent-spec role: an intent document declares the entities an architecture doc must
/// satisfy (`specs/architecture/20-surface.md`, "`requirement` clauses"). The demand lives on the
/// member document, not the assembly, so it reaches the roster only through the
/// composition `check` performs and the read family must mirror
/// (READ-VERBS-PUBLISHED-DEMANDS).
fn write_publishing_spec(root: &Path, name: &str, requirement: &str, means: &str) {
    let dir = root.join(".temper").join("specs").join(name);
    fs::create_dir_all(&dir).unwrap();
    let document = format!(
        "+++\n\
         [requirement.{requirement}]\n\
         means = \"{means}\"\n\
         required = true\n\
         \n\
         [provenance]\n\
         source_path = \"specs/{name}.md\"\n\
         source_hash = \"deadbeef\"\n\
         +++\n\
         # {name}\n\
         \n\
         Body.\n"
    );
    fs::write(dir.join("SPEC.md"), document).unwrap();
}

/// Build a fixture whose requirement roster is published **entirely on a surface
/// member**: `00-intent` publishes `[requirement.governance]` and `45-governance`
/// satisfies it — the intent↔architecture join, on member documents, with no assembly
/// `[requirement.*]`. The join is exactly the shape the entry names: `check` composes
/// assembly ∪ member-published and reports it live, so a read reading the assembly
/// roster alone would misreport `45-governance`'s `satisfies` as dangling.
fn published_fixture() -> PathBuf {
    let root = tmpdir("published-root");
    let kind_dir = root.join(".temper").join("kinds").join("spec");
    fs::create_dir_all(&kind_dir).unwrap();
    fs::write(kind_dir.join("KIND.md"), SPEC_KIND_MD).unwrap();
    write_publishing_spec(
        &root,
        "00-intent",
        "governance",
        "the corpus declares a governance model an architecture doc must satisfy",
    );
    write_spec(
        &root,
        "45-governance",
        "governance",
        "the home for the governance model",
    );
    fs::write(root.join("temper.toml"), PUBLISHED_TEMPER_TOML).unwrap();
    root
}

#[test]
fn requirements_lists_a_member_published_requirement() {
    let root = published_fixture();
    // The roster is published entirely on a member (`00-intent`), not the assembly, so
    // a read blind to member-published demands would show an empty roster. It must list
    // `governance` with its satisfier `45-governance` (READ-VERBS-PUBLISHED-DEMANDS).
    let run = read(&root, &["requirements"]);
    assert!(run.ok, "requirements must exit zero: {}", run.stdout);
    assert!(
        run.stdout.contains("governance"),
        "the member-published requirement must appear in the roster: {}",
        run.stdout
    );
    insta::assert_snapshot!("requirements_member_published", run.stdout);
}

#[test]
fn requirements_detail_walks_a_member_published_requirement() {
    let root = published_fixture();
    // The named reverse walk resolves `governance` (a member-published demand) and lists
    // its satisfier set — not "No requirement named `governance` is published".
    let run = read(&root, &["requirements", "governance"]);
    assert!(run.ok, "requirements <name> must exit zero: {}", run.stdout);
    insta::assert_snapshot!("requirements_member_published_detail", run.stdout);
}

#[test]
fn why_narrates_a_satisfies_to_a_member_published_requirement_as_filled() {
    let root = published_fixture();
    // `45-governance` satisfies `governance`, published by `00-intent`. Reading the
    // assembly roster alone (empty here) narrated this join as "This link dangles" over
    // a green `check` — a falsehood. It must now read FILLED, with the requirement's
    // `means` (READ-VERBS-PUBLISHED-DEMANDS).
    let run = read(&root, &["why", "45-governance"]);
    assert!(run.ok, "why must exit zero: {}", run.stdout);
    assert!(
        !run.stdout.contains("This link dangles"),
        "a live member-published join must not narrate as dangling: {}",
        run.stdout
    );
    insta::assert_snapshot!("why_member_published_satisfier", run.stdout);
}

#[test]
fn impact_dangles_the_satisfiers_of_a_sole_publisher() {
    let root = published_fixture();
    // `00-intent` alone publishes `governance`, which `45-governance` satisfies. Removing
    // `00-intent` drops the demand from the namespace, so `45-governance`'s `satisfies`
    // would dangle — the second blast-radius strand, over a member-published demand the
    // composed roster carries (READ-VERBS-PUBLISHED-DEMANDS).
    let run = read(&root, &["impact", "00-intent"]);
    assert!(run.ok, "impact must exit zero: {}", run.stdout);
    assert!(
        run.stdout.contains(
            "`45-governance` (spec) fills `governance`, which only `00-intent` publishes"
        ),
        "removing the sole publisher must dangle its satisfiers: {}",
        run.stdout
    );
    insta::assert_snapshot!("impact_sole_publisher", run.stdout);
}

/// The `temper.toml` for the genre-fence fixture — registers the `spec` custom kind so its
/// members (and their serialized genre values) load into the read path.
const GENRE_TEMPER_TOML: &str = "\
[kind.spec]
package = \"spec\"
";

/// A `spec` KIND.md that declares a `decision` genre and composes the `fenced` primitive,
/// so a member's genre fence extracts into a serialized [`GenreValue`] the leaf-grain
/// `impact` reads (`specs/architecture/20-surface.md`, "Genre values").
const GENRE_KIND_MD: &str = "+++\n\
governs = { root = \"specs\", glob = \"*.md\" }\n\
\n\
[[extraction]]\n\
primitive = \"line_count\"\n\
\n\
[[extraction]]\n\
primitive = \"fenced\"\n\
\n\
[[genres]]\n\
name = \"decision\"\n\
+++\n\
\n\
# The spec kind\n\
\n\
Governing docs with decision genres.\n";

/// Author a `spec` member carrying a `decision` genre fence — the floor spelling of a
/// genre value (`specs/architecture/20-surface.md`, "the floor spelling is a genre fence"): a fenced
/// block whose info string names the genre and key, whose interior is TOML leaves. The
/// `chosen` leaf addresses as `<name>/decision/surface-authority/chosen`.
fn write_genre_spec(root: &Path, name: &str) {
    let dir = root.join(".temper").join("specs").join(name);
    fs::create_dir_all(&dir).unwrap();
    let document = format!(
        "+++\n\
         [provenance]\n\
         source_path = \"specs/{name}.md\"\n\
         source_hash = \"deadbeef\"\n\
         +++\n\
         # {name}\n\
         \n\
         ```genre.decision surface-authority\n\
         chosen = \"the surface is canonical\"\n\
         ```\n"
    );
    fs::write(dir.join("SPEC.md"), document).unwrap();
}

/// Build a fixture whose `spec` member `20-surface` carries a `decision` genre value with a
/// `chosen` prose leaf — the serialized shape leaf-grain `impact` resolves and reports.
fn genre_fixture() -> PathBuf {
    let root = tmpdir("genre-root");
    let kind_dir = root.join(".temper").join("kinds").join("spec");
    fs::create_dir_all(&kind_dir).unwrap();
    fs::write(kind_dir.join("KIND.md"), GENRE_KIND_MD).unwrap();
    write_genre_spec(&root, "20-surface");
    fs::write(root.join("temper.toml"), GENRE_TEMPER_TOML).unwrap();
    root
}

#[test]
fn impact_over_a_leaf_address_reports_leaf_grain() {
    let root = genre_fixture();
    // A leaf address dispatches to leaf grain: `impact` resolves the leaf against the
    // serialized genre values and reports its citations *separately from* its fallout — a
    // leaf is obligation-free, so deleting or rewording it is never blocked (`45-governance.md`,
    // address grain). No citer is declared (floor leaves carry no mentions), so the
    // citations heading names none.
    let run = read(
        &root,
        &["impact", "20-surface/decision/surface-authority/chosen"],
    );
    assert!(run.ok, "impact on a leaf must exit zero: {}", run.stdout);
    assert!(
        run.stdout
            .contains("Leaf `20-surface/decision/surface-authority/chosen` (spec)"),
        "leaf grain must name the resolved leaf: {}",
        run.stdout
    );
    assert!(
        run.stdout
            .contains("Authored value: \"the surface is canonical\""),
        "the resolved leaf reads its authored value off the manifest: {}",
        run.stdout
    );
    let citations_at = run.stdout.find("Citations (").expect("a citations heading");
    let fallout_at = run.stdout.find("Fallout:").expect("a fallout heading");
    assert!(
        citations_at < fallout_at,
        "citations are reported distinctly from fallout: {}",
        run.stdout
    );
    insta::assert_snapshot!("impact_leaf_grain", run.stdout);
}

#[test]
fn impact_over_a_member_name_is_unchanged_by_leaf_grain() {
    let root = genre_fixture();
    // The bare member name (no slash) still takes the member-grain path — the four blast
    // strands, not leaf grain — proving the leaf dispatch is additive (regression).
    let run = read(&root, &["impact", "20-surface"]);
    assert!(run.ok, "impact on a member must exit zero: {}", run.stdout);
    assert!(
        run.stdout
            .contains("Member `20-surface` (spec) — the blast radius"),
        "a member name reports member grain, not leaf grain: {}",
        run.stdout
    );
    assert!(
        !run.stdout.contains("leaf grain"),
        "a member name never dispatches to leaf grain: {}",
        run.stdout
    );
}

#[test]
fn impact_over_an_unresolved_leaf_address_still_exits_zero() {
    let root = genre_fixture();
    // A leaf address naming no live leaf is a read, not a gate — narrated plainly, exit zero.
    let run = read(
        &root,
        &["impact", "20-surface/decision/surface-authority/rejected"],
    );
    assert!(run.ok, "an unresolved leaf must exit zero: {}", run.stdout);
    assert!(
        run.stdout
            .contains("No leaf `20-surface/decision/surface-authority/rejected`"),
        "an unresolved leaf is named absent: {}",
        run.stdout
    );
}

#[test]
fn impact_leaf_grain_discloses_coverage() {
    let root = genre_fixture();
    // The `impact` leaf-grain answer discloses its mixed-posture coverage WITH the found answer,
    // not only in a not-found error (`specs/architecture/20-surface.md`, "both disclose coverage").
    let run = read(
        &root,
        &["impact", "20-surface/decision/surface-authority/chosen"],
    );
    assert!(run.ok, "impact on a leaf must exit zero: {}", run.stdout);
    assert!(
        run.stdout.contains("not represented at leaf grain"),
        "the found leaf answer must disclose coverage: {}",
        run.stdout
    );
}

/// A genre-fence spec member `20-surface` carrying a `decision` value with *two* leaves —
/// `chosen` and `rejected` — so `context` on `chosen` reports `rejected` as its sibling. The
/// single-leaf `genre_fixture` cannot exercise the sibling strand.
fn write_two_leaf_spec(root: &Path, name: &str) {
    let dir = root.join(".temper").join("specs").join(name);
    fs::create_dir_all(&dir).unwrap();
    let document = format!(
        "+++\n\
         [provenance]\n\
         source_path = \"specs/{name}.md\"\n\
         source_hash = \"deadbeef\"\n\
         +++\n\
         # {name}\n\
         \n\
         ```genre.decision surface-authority\n\
         chosen = \"the surface is canonical\"\n\
         rejected = \"the surface is a read-only lens\"\n\
         ```\n"
    );
    fs::write(dir.join("SPEC.md"), document).unwrap();
}

/// Build a fixture whose `20-surface` spec carries a two-leaf `decision` value, and a second
/// bare spec `00-intent` carrying no genre value — so the coverage disclosure counts one
/// document that carries no genre values (the leafless member).
fn context_fixture() -> PathBuf {
    let root = tmpdir("context-root");
    let kind_dir = root.join(".temper").join("kinds").join("spec");
    fs::create_dir_all(&kind_dir).unwrap();
    fs::write(kind_dir.join("KIND.md"), GENRE_KIND_MD).unwrap();
    write_two_leaf_spec(&root, "20-surface");
    // A second member carrying no serialized genre leaf — the "document that carries no genre
    // values" the coverage disclosure counts.
    write_spec(&root, "00-intent", "intent-encoded", "the north star spec");
    fs::write(root.join("temper.toml"), GENRE_TEMPER_TOML).unwrap();
    root
}

#[test]
fn context_over_a_leaf_emits_its_neighborhood() {
    let root = context_fixture();
    // `context` on a leaf address emits its genre slot, its siblings (the co-resident
    // `rejected` leaf), its citers, the requirements its member satisfies, and the mixed-posture
    // coverage disclosure — the pre-edit context bundle (`specs/architecture/20-surface.md`).
    let run = read(
        &root,
        &["context", "20-surface/decision/surface-authority/chosen"],
    );
    assert!(run.ok, "context on a leaf must exit zero: {}", run.stdout);
    assert!(
        run.stdout
            .contains("Leaf `20-surface/decision/surface-authority/chosen` (spec)"),
        "the neighborhood names the resolved leaf: {}",
        run.stdout
    );
    // The genre slot it lives in.
    assert!(
        run.stdout
            .contains("Genre slot: the `chosen` leaf of the `decision` value `surface-authority`"),
        "the genre slot is named: {}",
        run.stdout
    );
    // Its sibling leaf, co-resident in the same genre value.
    assert!(
        run.stdout.contains("Siblings (the other leaves"),
        "siblings are reported: {}",
        run.stdout
    );
    assert!(
        run.stdout
            .contains("`rejected` — \"the surface is a read-only lens\""),
        "the co-resident sibling leaf is named: {}",
        run.stdout
    );
    // Citers (none on the floor) and satisfied requirements are both reported.
    assert!(
        run.stdout.contains("Citations ("),
        "citers section: {}",
        run.stdout
    );
    assert!(
        run.stdout.contains("Satisfied requirements"),
        "satisfied requirements section: {}",
        run.stdout
    );
    // Coverage disclosure — one document that carries no genre values (the leafless `00-intent`).
    assert!(
        run.stdout.contains("not represented at leaf grain"),
        "the leaf answer discloses coverage: {}",
        run.stdout
    );
    assert!(
        run.stdout.contains("1 document carries no genre values"),
        "the coverage count names the one leafless member: {}",
        run.stdout
    );
    insta::assert_snapshot!("context_leaf_grain", run.stdout);
}

#[test]
fn context_over_a_member_name_emits_its_genre_slots() {
    let root = context_fixture();
    // A bare member name (no slash) reports member grain — its genre slots — not leaf grain,
    // and still discloses coverage.
    let run = read(&root, &["context", "20-surface"]);
    assert!(run.ok, "context on a member must exit zero: {}", run.stdout);
    assert!(
        run.stdout
            .contains("Member `20-surface` (spec) — its declared neighborhood"),
        "a member name reports member grain: {}",
        run.stdout
    );
    assert!(
        run.stdout
            .contains("Genre slots (the genre values it carries)"),
        "the member's genre slots are enumerated: {}",
        run.stdout
    );
    assert!(
        run.stdout.contains("not represented at leaf grain"),
        "a member neighborhood discloses coverage too: {}",
        run.stdout
    );
}

#[test]
fn context_over_an_unresolved_address_still_exits_zero() {
    let root = context_fixture();
    // An unresolved leaf and an unknown member are both reads, not gates — narrated plainly,
    // exit zero.
    let leaf = read(
        &root,
        &["context", "20-surface/decision/surface-authority/ghost"],
    );
    assert!(
        leaf.ok,
        "an unresolved leaf must exit zero: {}",
        leaf.stdout
    );
    assert!(
        leaf.stdout
            .contains("No leaf `20-surface/decision/surface-authority/ghost`"),
        "an unresolved leaf is named absent: {}",
        leaf.stdout
    );

    let member = read(&root, &["context", "ghost"]);
    assert!(
        member.ok,
        "an unknown member must exit zero: {}",
        member.stdout
    );
    assert!(
        member.stdout.contains("No member named `ghost`"),
        "an unknown member is named absent: {}",
        member.stdout
    );
}
