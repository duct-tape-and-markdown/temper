//! Acceptance over the documented round trip.
//!
//! Pins the whole vertical slice — typed IR, sidecar topology, contract engine,
//! diagnostics UX — driving the generic `engine::validate` and `drift::emit` directly
//! (logic lives in the lib per `.claude/rules/rust.md`, so no binary harness is needed):
//!
//! - an `insta` snapshot of the **check diagnostics** the built-in skill contract
//!   produces over the deliberately broken `tests/fixtures/rules/*` tree — the
//!   reduced, decidable-only surviving-clause set;
//! - the slice acceptance end to end: a well-formed `coordinate` skill checks clean,
//!   and compiling its seam payload twice reproduces the
//!   projection with no diff;
//! - the custom-kind acceptance:
//!   over a corpus whose lock declares a custom kind and an advisory `extent`
//!   clause naming it, `temper check` names that clause and the offending member
//!   in its diagnostic output (and flips the exit code under `--deny-advisories`)
//!   — driving the built binary, since both the rendered diagnostics and the exit
//!   code are observable only across a real process boundary.

use std::fs;
use std::path::{Path, PathBuf};

mod common;

use temper::builtin_kind;
use temper::check::{self, Diagnostic, Severity};
use temper::contract::Contract;
use temper::drift::{self, Declarations, EmitOptions, Payload, PayloadMember};
use temper::engine;
use temper::frontmatter::Member;

/// The built-in Anthropic skill contract, resolved from the embedded built-in lock
/// exactly as the shipped `check` does — so the acceptance path validates against
/// the same clauses the tool ships.
fn builtin_skill_contract() -> Contract {
    temper::builtin::contract("skill").expect("the skill floor is embedded")
}

/// Render a diagnostic set as one stable line per finding (`<severity> <rule>:
/// <message>`), in the order the engine collects them.
fn render_diagnostics(diagnostics: &[Diagnostic]) -> String {
    if diagnostics.is_empty() {
        return "(no diagnostics)\n".to_string();
    }
    let mut out = String::new();
    for diagnostic in diagnostics {
        let severity = match diagnostic.severity {
            Severity::Error => "error",
            Severity::Warn => "warn",
            Severity::Note => "note",
        };
        out.push_str(&format!(
            "{severity} {}: {}\n",
            diagnostic.rule, diagnostic.message
        ));
    }
    out
}

/// Loading the deliberately-broken `tests/fixtures/rules/*` tree and validating
/// each fixture against the built-in skill contract reproduces the expected
/// diagnostic set — the reduced, decidable-only surviving-clause findings, the
/// `clean` control silent.
#[test]
fn check_reproduces_the_expected_diagnostic_set() {
    let rules_root = common::fixture("rules");
    let mut fixtures: Vec<PathBuf> = fs::read_dir(&rules_root)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect();
    fixtures.sort();

    let contract = builtin_skill_contract();
    let mut report = String::new();
    for dir in &fixtures {
        let name = dir.file_name().unwrap().to_string_lossy();
        let skill_kind = temper::builtin_kind::definition("skill").unwrap();
        let skill = Member::from_source(&skill_kind, &dir.join("SKILL.md"))
            .expect("fixture skill should parse");
        // Read features off the projected surface member document through the generic
        // `Unit` loader `check` uses — no IR→Unit adapter. `placement` still reads the
        // imported source directory off provenance, so `name-matches-dir` is unchanged.
        let unit = common::skill_surface_unit(&skill);
        let features = builtin_kind::skill_features(&unit);
        let diagnostics = engine::validate(&contract, std::slice::from_ref(&features));
        report.push_str(&format!("## {name}\n"));
        report.push_str(&render_diagnostics(&diagnostics));
        report.push('\n');
    }

    insta::assert_snapshot!("rules_check_diagnostics", report);
}

/// The slice acceptance, end to end: the well-formed `coordinate` fixture skill checks
/// clean over its projected surface member document, and compiling its seam payload
/// twice
/// reproduces the projection with no diff.
#[test]
fn acceptance_check_then_reemit_is_a_no_diff() {
    let skill_kind = temper::builtin_kind::definition("skill").unwrap();
    let skill =
        Member::from_source(&skill_kind, &common::fixture("coordinate").join("SKILL.md")).unwrap();

    // check — a well-formed skill trips no contract clause, so it is clean. The gate
    // reads each skill's surface member document through the one generic `Unit` loader.
    let unit = common::skill_surface_unit(&skill);
    let features = [builtin_kind::skill_features(&unit)];
    let diagnostics = engine::validate(&builtin_skill_contract(), &features);
    assert!(
        diagnostics.is_empty(),
        "the coordinate skill must check clean, got {diagnostics:?}",
    );
    assert!(!check::any_error(&diagnostics));

    // emit — a hand-built seam payload over the same fixture skill, compiled twice: the
    // second compile reproduces the harness projection byte-for-byte.
    let harness = common::tmpdir("acceptance-harness");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::skill_kind_facts(
                Some("claude-code"),
                &["description-trigger(description)"],
            )],
            ..Declarations::default()
        },
        members: vec![PayloadMember {
            kind: "skill".to_string(),
            name: skill.id.clone(),
            host: None,
            fields: skill.fields.clone(),
            body: skill.body.clone(),
            source_path: None,
        }],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let projected = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let first = fs::read_to_string(&projected).unwrap();

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(
        first,
        fs::read_to_string(&projected).unwrap(),
        "a re-emit of the unchanged payload must produce no diff"
    );
}

/// Write `<corpus>/.temper/lock.toml` verbatim — the SDK-emitted lock a converted
/// harness carries, stood in for directly so a test declares the exact clause rows a
/// built-in kind's `expect` binding erases to (`sdk/src/declarations.ts`).
fn write_lock(corpus: &Path, contents: &str) {
    let temper = corpus.join(".temper");
    fs::create_dir_all(&temper).unwrap();
    fs::write(temper.join("lock.toml"), contents).unwrap();
}

/// A skill named for its directory `name`, carrying a `tier` frontmatter field and the
/// Cursor `globs` key — a body clean against every embedded default clause, so the only
/// findings a case sees are the ones its declared contract produces over `tier`/`globs`
/// (or the embedded default's `deny(name)` when `name` is a reserved word).
fn skill_with_tier(name: &str, tier: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         tier: {tier}\n\
         globs: \"**/*.rs\"\n\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

/// A built-in kind the lock declares no clause row for gates on its embedded default:
/// a skill named `anthropic` trips the shipped `deny(name)` clause with no lock rows to
/// compose from — the rowless fallback (`builtin_default_contract`) holds exactly as
/// before the flip layer retired.
#[test]
fn builtin_skill_rowless_gates_on_the_embedded_default() {
    let corpus = common::tmpdir("skill-rowless");
    common::write_skill(&corpus, "anthropic", &common::clean_skill("anthropic"));

    let run = common::check_in(&corpus, &[], None);
    assert!(
        run.output.contains("deny") && run.output.contains("anthropic"),
        "a rowless built-in kind must gate on the embedded default — `deny(name)` \
         fires over the reserved `anthropic`, got:\n{}",
        run.output
    );
}

/// The lock's declared skill clause rows ARE the kind's whole contract — a spread that
/// keeps `required(description)` and appends an `enum(tier)` (the range/enumOf tier),
/// omitting `deny(name)` and `forbidden_keys`. The appended clause fires on the member;
/// the omitted defaults no longer gate, even over inputs (`name = anthropic`, a `globs`
/// key) they would have caught — array surgery removes, no severity-flip layer survives.
#[test]
fn builtin_skill_declared_rows_are_the_whole_contract() {
    let corpus = common::tmpdir("skill-rows-are-contract");
    common::write_skill(
        &corpus,
        "anthropic",
        &skill_with_tier("anthropic", "experimental"),
    );
    write_lock(
        &corpus,
        "[[declaration.clause]]\n\
         label = \"skill.required.description\"\n\
         kind = \"skill\"\n\
         predicate = \"required\"\n\
         field = \"description\"\n\
         severity = \"required\"\n\
         \n\
         [[declaration.clause]]\n\
         label = \"skill.enum.tier\"\n\
         kind = \"skill\"\n\
         predicate = \"enum\"\n\
         field = \"tier\"\n\
         severity = \"required\"\n\
         values = [\"core\", \"extra\"]\n",
    );

    let run = common::check_in(&corpus, &[], None);
    assert!(
        run.output.contains("enum") && run.output.contains("anthropic"),
        "the appended enum clause must fire on the member's `tier`, got:\n{}",
        run.output
    );
    assert!(
        !run.output.contains("deny"),
        "the omitted `deny(name)` default must not gate — `anthropic` passes, got:\n{}",
        run.output
    );
    assert!(
        !run.output.contains("forbidden_keys"),
        "the omitted `forbidden_keys` default must not gate — the `globs` key passes, got:\n{}",
        run.output
    );
}

/// An out-of-vocabulary row naming a built-in kind rejects loud, never sits inert: a
/// skill clause row whose predicate names nothing in the closed vocabulary fails the
/// load — the same reject-loud lift a custom kind's rows take, now that a built-in
/// kind's rows lift the identical way.
#[test]
fn builtin_skill_out_of_vocabulary_row_is_a_load_error() {
    let corpus = common::tmpdir("skill-bad-row");
    common::write_skill(&corpus, "widget", &common::clean_skill("widget"));
    write_lock(
        &corpus,
        "[[declaration.clause]]\n\
         label = \"skill.not_a_predicate\"\n\
         kind = \"skill\"\n\
         predicate = \"not_a_predicate\"\n\
         severity = \"required\"\n",
    );

    let run = common::check_in(&corpus, &[], None);
    assert!(
        !run.ok,
        "an out-of-vocabulary skill clause row must fail the load, got exit-zero:\n{}",
        run.output
    );
    assert!(
        run.output.contains("not_a_predicate"),
        "the load error must name the offending predicate, got:\n{}",
        run.output
    );
}

/// Author a custom kind's `lock.toml` declaration row pair — one
/// `[[declaration.kind]]` naming its `governs` root/glob, one
/// `[[declaration.clause]]` binding an advisory `extent` budget to it — the
/// live authoring surface (`tests/session_start.rs`'s
/// `a_custom_kind_synthesized_from_the_lock_resolves_its_requirement_with_no_false_admissibility_finding`
/// uses the identical shape). `extent` is the fixture's small 10-line budget so a
/// short over-length body trips it without a real spec-sized corpus.
fn author_custom_kind_lock(corpus: &Path, name: &str, governs_root: &str) {
    let temper = corpus.join(".temper");
    fs::create_dir_all(&temper).unwrap();
    fs::write(
        temper.join("lock.toml"),
        format!(
            "[[declaration.kind]]\n\
             name = \"{name}\"\n\
             governs_root = \"{governs_root}\"\n\
             governs_glob = \"*.md\"\n\
             \n\
             [[declaration.clause]]\n\
             label = \"{name}.extent\"\n\
             kind = \"{name}\"\n\
             predicate = \"extent\"\n\
             severity = \"advisory\"\n\
             bound = {{ max = 10 }}\n\
             unit = \"lines\"\n"
        ),
    )
    .unwrap();
}

/// A body over the fixture's 10-line `extent` budget — used to prove the
/// advisory fires (and, under `--deny-advisories`, blocks), naming itself in the
/// diagnostic.
fn over_length_body() -> String {
    let mut body = String::from("# Kinds\n");
    for line in 1..=40 {
        body.push_str(&format!("Line {line} of an over-budget body.\n"));
    }
    body
}

/// Run the built binary `temper check <root> [extra…]` from `cwd` and return
/// whether it exited zero, plus the rendered diagnostic set on stdout. `root` is a
/// harness root: `check` resolves `<root>/.temper`'s committed lock and walks its
/// members off `<root>`.
fn check_from(cwd: &Path, root: &Path, extra: &[&str]) -> (bool, String) {
    let mut args = vec![root.to_str().unwrap()];
    args.extend_from_slice(extra);
    let run = common::check_in(cwd, &args, None);
    (run.ok, run.stdout)
}

/// The same run under the GitHub reporter — one line per finding, so a claim about one
/// rule is pinned to that rule's own lines. A whole-stdout `contains` cannot make such a
/// claim: a second rule naming the same artifact moves it.
fn check_findings(cwd: &Path, root: &Path, extra: &[&str]) -> (bool, Vec<String>) {
    let mut args = vec![root.to_str().unwrap()];
    args.extend_from_slice(extra);
    let run = common::check_in(cwd, &args, Some("github"));
    (run.ok, run.findings())
}

/// The custom-kind acceptance:
/// over a corpus whose lock declares a `spec` kind + an advisory `extent`
/// clause naming it, `check` names that clause and the offending member in its
/// diagnostic output for the over-length spec, and stays silent about `extent`
/// for the clean one — proof the custom kind's own clause fires, not just the
/// always-on coverage/gate-installed notes every corpus carries.
#[test]
fn check_dispatches_the_spec_custom_kind_through_its_extractor_and_contract() {
    let corpus = common::tmpdir("spec-corpus");
    author_custom_kind_lock(&corpus, "spec", "specs");
    let specs = corpus.join("specs");
    fs::create_dir_all(&specs).unwrap();
    // A short spec (clean) beside an over-length one (trips the advisory budget).
    fs::write(specs.join("00-intent.md"), "# Intent\n\nThe north star.\n").unwrap();
    fs::write(specs.join("15-kinds.md"), over_length_body()).unwrap();

    let (ok, findings) = check_findings(&corpus, &corpus, &[]);
    assert!(
        ok,
        "an advisory-only spec violation must exit zero without --deny-advisories"
    );
    let extent = common::findings_for(&findings, "spec.extent");
    assert_eq!(
        extent.len(),
        1,
        "the over-length spec's own extent clause must fire exactly once, got: \
         {findings:#?}"
    );
    assert!(
        extent[0].contains("15-kinds"),
        "and name the offending member, got: {}",
        extent[0]
    );
    // Pinned to the `extent` rule, never to whole stdout: the lock declares this kind
    // and no member of it, so `locus.undeclared-member` names the clean spec too — a
    // second rule over the same artifact, and not this claim's business.
    assert!(
        !extent[0].contains("00-intent"),
        "the clean spec must trip no extent finding, got: {}",
        extent[0]
    );

    let (ok, findings) = check_findings(&corpus, &corpus, &["--deny-advisories"]);
    assert!(
        !ok,
        "the over-length spec must exit non-zero under --deny-advisories"
    );
    // The exit code alone no longer proves the escalation — a second advisory over the
    // same corpus would carry it — so the arm pins the promoted finding itself.
    let extent = common::findings_for(&findings, "spec.extent");
    assert_eq!(extent.len(), 1, "got: {findings:#?}");
    assert!(
        extent[0].starts_with("::warning ") && extent[0].contains("15-kinds"),
        "the promoted finding is the over-length spec's own advisory extent \
         violation, got: {}",
        extent[0]
    );
}

/// A custom kind authored **outside** `specs/` (`adr/*.md`), the same shape as the
/// `spec` case above and driven for the same reason: its own lock-declared
/// `extent` clause names itself and the offending ADR in the diagnostic output,
/// independent of the always-on coverage/gate-installed notes.
#[test]
fn check_reads_a_custom_kind_rooted_outside_specs() {
    let corpus = common::tmpdir("adr-corpus");
    author_custom_kind_lock(&corpus, "adr", "adr");
    let adrs = corpus.join("adr");
    fs::create_dir_all(&adrs).unwrap();
    // A short ADR (clean) beside an over-length one (trips the advisory budget).
    fs::write(adrs.join("0001-short.md"), "# ADR 1\n\nDecided.\n").unwrap();
    fs::write(adrs.join("0002-long.md"), over_length_body()).unwrap();

    let (ok, findings) = check_findings(&corpus, &corpus, &[]);
    assert!(
        ok,
        "an advisory-only ADR violation must exit zero without --deny-advisories"
    );
    let extent = common::findings_for(&findings, "adr.extent");
    assert_eq!(
        extent.len(),
        1,
        "the over-length ADR's own extent clause must fire exactly once, got: \
         {findings:#?}"
    );
    assert!(
        extent[0].contains("0002-long"),
        "and name the offending member, got: {}",
        extent[0]
    );
    assert!(
        !extent[0].contains("0001-short"),
        "the clean ADR must trip no extent finding, got: {}",
        extent[0]
    );

    let (ok, findings) = check_findings(&corpus, &corpus, &["--deny-advisories"]);
    assert!(
        !ok,
        "the over-length ADR must exit non-zero under --deny-advisories"
    );
    let extent = common::findings_for(&findings, "adr.extent");
    assert_eq!(extent.len(), 1, "got: {findings:#?}");
    assert!(
        extent[0].starts_with("::warning ") && extent[0].contains("0002-long"),
        "the promoted finding is the over-length ADR's own advisory extent \
         violation, got: {}",
        extent[0]
    );
}

/// A represented harness that violates nothing exits **zero** under
/// `--deny-advisories`, even though the always-on `coverage.checked` summary rides in
/// the output. The flag escalates advisory *violations* — clauses a corpus declared and
/// could dial — and a disclosure note is neither, so nothing about it is promotable:
/// without this the strict CI policy could never pass on any harness.
#[test]
fn deny_advisories_never_blocks_on_the_coverage_disclosure_note() {
    let corpus = common::tmpdir("clean-under-deny-advisories");
    common::write_skill(&corpus, "coordinate", &common::clean_skill("coordinate"));

    let (ok, output) = check_from(&corpus, &corpus, &["--deny-advisories"]);
    assert!(
        ok,
        "a clean harness must exit zero under --deny-advisories, got:\n{output}"
    );
    assert!(
        output.contains("coverage.checked"),
        "and the disclosure note is still printed, never suppressed to earn that \
         zero, got:\n{output}"
    );
}

/// A hash no file on disk can match, so a lock row carrying it is drifted the moment it
/// is written — the corpus *is* the drifted state, with nothing to move afterwards.
const UNMATCHABLE_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// The lock's kind-less `[[declaration.clause]]` row binding the root's `fresh` clause at
/// `severity` — `compose::root_contract` reads the kind-less rows as the root's whole
/// contract, so a case says exactly which root predicates bind by what it writes here.
fn root_fresh_row(severity: &str) -> String {
    format!(
        "[[declaration.clause]]\n\
         label = \"root.fresh\"\n\
         predicate = \"fresh\"\n\
         severity = \"{severity}\"\n"
    )
}

/// A corpus stale in all three ways the root `fresh` clause judges — one committed
/// projection whose bytes no longer match its `emit_hash`, one layout-import target and
/// one composed-prose include target whose bytes no longer match their `import_hash` —
/// with `root_rows` as the root member's whole contract.
///
/// Every hash is [`UNMATCHABLE_HASH`], so the three facts are the lock's own claim
/// against real files rather than a re-emit the test would have to drive.
fn drifted_corpus(label: &str, root_rows: &str) -> PathBuf {
    let corpus = common::tmpdir(label);
    common::write_skill(&corpus, "coordinate", &common::clean_skill("coordinate"));
    fs::write(corpus.join("imported.md"), "imported prose.\n").unwrap();
    fs::write(corpus.join("included.md"), "included prose.\n").unwrap();
    // `[[skill]]` leads: a top-level provenance row written after `[declaration]` would
    // nest under it.
    write_lock(
        &corpus,
        &format!(
            "[[skill]]\n\
             name = \"coordinate\"\n\
             source_path = \".claude/skills/coordinate/SKILL.md\"\n\
             source_hash = \"{UNMATCHABLE_HASH}\"\n\
             emit_hash = \"{UNMATCHABLE_HASH}\"\n\
             \n\
             [[declaration.layout_import]]\n\
             member = \"skill:coordinate\"\n\
             source_path = \"imported.md\"\n\
             import_hash = \"{UNMATCHABLE_HASH}\"\n\
             \n\
             [[declaration.include]]\n\
             member = \"skill:coordinate\"\n\
             source_path = \"included.md\"\n\
             import_hash = \"{UNMATCHABLE_HASH}\"\n\
             \n\
             {root_rows}"
        ),
    );
    corpus
}

/// The freshness findings are the root `fresh` clause's to weigh, and the dial reaches
/// the clause by its label: the same drifted corpus is advisory undialed and **blocking**
/// once a `.temper/dial.toml` reads the label at `required`.
///
/// This is the 0.0.18 field report's point. The three drift pushes were fixed `warn`s:
/// the dial reaches clause labels only and enforcement mode acts at `guard`, so no
/// placement could make a drifted content pin fail CI. All three now report under one
/// clause's label, so one dial entry — or one composed clause — decides all three.
#[test]
fn the_fresh_clause_weighs_all_three_staleness_facts_and_the_dial_reaches_it() {
    let corpus = drifted_corpus("fresh-advisory", &root_fresh_row("advisory"));

    let (ok, findings) = check_findings(&corpus, &corpus, &[]);
    let fresh = common::findings_for(&findings, "root.fresh");
    assert_eq!(
        fresh.len(),
        3,
        "one finding per drifted lock row — the projection, the layout import, the \
         include — all under the one clause's label, got: {findings:#?}"
    );
    assert!(
        fresh.iter().all(|line| line.starts_with("::warning")),
        "the clause declared `advisory`, so none of them blocks, got: {fresh:#?}"
    );
    assert!(ok, "and the run exits zero, got:\n{findings:#?}");
    // Each drifted row is named, so the count above cannot be three findings over one row.
    for artifact in [
        ".claude/skills/coordinate/SKILL.md",
        "imported.md",
        "included.md",
    ] {
        assert!(
            fresh.iter().any(|line| line.contains(artifact)),
            "`{artifact}` is one of the drifted rows the clause indicts, got: {fresh:#?}"
        );
    }

    // The advisory posture is promotable, which is what the strict CI policy buys today.
    // The exit code alone does not prove the escalation — the always-on
    // `install.gate-installed` advisory would carry it — so the arm pins the promoted
    // findings themselves beside it.
    let (denied_ok, findings) = check_findings(&corpus, &corpus, &["--deny-advisories"]);
    assert!(
        !denied_ok,
        "an advisory freshness violation exits non-zero under --deny-advisories, got:\n\
         {findings:#?}"
    );
    assert_eq!(
        common::findings_for(&findings, "root.fresh").len(),
        3,
        "and the three promoted findings are the clause's own, got: {findings:#?}"
    );

    // And dialing the label at `required` blocks the *unflagged* run — the placement the
    // field report found unreachable.
    common::write_sibling(
        &corpus,
        ".temper/dial.toml",
        "name = \"workstation\"\n\n[[clause]]\nlabel = \"root.fresh\"\nseverity = \"required\"\n",
    );
    let (dialed_ok, findings) = check_findings(&corpus, &corpus, &[]);
    let fresh = common::findings_for(&findings, "root.fresh");
    assert_eq!(
        fresh.len(),
        3,
        "the dial softens nothing away: {findings:#?}"
    );
    assert!(
        fresh.iter().all(|line| line.starts_with("::error")),
        "the dialed clause reads `required`, so every drifted row blocks, got: {fresh:#?}"
    );
    assert!(
        !dialed_ok,
        "so a drifted content pin fails CI with no flag at all, got:\n{findings:#?}"
    );
}

/// A root contract that binds no `fresh` clause reports no staleness finding at all —
/// the observable form of the judges' opt-in: the gate locates the clause on the root
/// selection and walks nothing where none binds.
///
/// Pinned here rather than in the cost suite, which counts lock reads and parses: the
/// lock document is already read and parsed before the `fresh` lookup, so a skipped walk
/// contributes zero to either counter and those pins cannot see it.
#[test]
fn a_root_contract_binding_no_fresh_clause_reports_no_staleness_at_all() {
    // `reachable` alone: real kind-less rows, so the fallback to the shipped default —
    // which binds both — never fires, and the corpus is drifted exactly as the case above.
    let corpus = drifted_corpus(
        "fresh-unbound",
        "[[declaration.clause]]\n\
         label = \"root.reachable\"\n\
         predicate = \"reachable\"\n\
         severity = \"advisory\"\n",
    );

    let (ok, findings) = check_findings(&corpus, &corpus, &[]);
    assert!(
        common::findings_for(&findings, "root.fresh").is_empty(),
        "no clause, no finding — freshness is not a fact the tool pushes unasked, got: \
         {findings:#?}"
    );
    assert!(
        !findings.iter().any(
            |line| line.contains("does not match the lock's emit fingerprint")
                || line.contains("no longer matches the lock's fingerprint")
        ),
        "and no staleness message rides any other rule id either, got: {findings:#?}"
    );
    assert!(ok, "the run exits zero, got:\n{findings:#?}");

    // Nor does the strict CI policy resurrect an unbound clause: `--deny-advisories`
    // promotes advisory *violations*, and a clause nobody declared produced none.
    let (_, findings) = check_findings(&corpus, &corpus, &["--deny-advisories"]);
    assert!(
        common::findings_for(&findings, "root.fresh").is_empty(),
        "opting out is a real opt-out, not a failure deferred to the strict policy, got: \
         {findings:#?}"
    );
}
