//! Acceptance over the documented round trip (`specs/architecture/20-surface.md`, "CLI
//! surface"; `specs/architecture/10-contracts.md`, the contract engine).
//!
//! Pins the whole vertical slice — typed IR, sidecar topology, contract engine,
//! diagnostics UX — driving the generic `engine::validate` and `drift::emit` directly
//! (logic lives in the lib per `.claude/rules/rust.md`, so no binary harness is needed):
//!
//! - an `insta` snapshot of the **check diagnostics** the built-in skill contract
//!   produces over the deliberately broken `tests/fixtures/rules/*` tree — the
//!   reduced, decidable-only surviving-clause set;
//! - the slice acceptance end to end: a well-formed `coordinate` skill checks clean,
//!   and compiling its seam payload twice (`emit` is the sole producer,
//!   `specs/architecture/20-surface.md`, "The lock and drift") reproduces the
//!   projection with no diff;
//! - the custom-kind acceptance (`specs/architecture/15-kinds.md`, "Worked example: `spec`"):
//!   over a corpus carrying an authored `spec` kind + package, `temper check`'s
//!   exit code flips under `--deny-advisories` (the empty-corpus coverage note is
//!   itself warn-severity) — driving the built binary, since the exit code is
//!   observable only across a real process boundary.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::builtin_kind;
use temper::check::{self, Diagnostic, Severity};
use temper::contract::Contract;
use temper::drift::{self, Declarations, EmitOptions, KindFactRow, Payload, PayloadMember};
use temper::engine;
use temper::frontmatter::Member;
use temper::kind::Unit;

/// The built-in Anthropic skill contract, resolved from the embedded built-in lock
/// exactly as the shipped `check` does — so the acceptance path validates against
/// the same clauses the tool ships (`specs/architecture/50-distribution.md`,
/// "Decision: the built-in lock is derived from the SDK module, never transcribed").
fn builtin_skill_contract() -> Contract {
    temper::builtin::contract("skill").expect("the skill floor is embedded")
}

/// The built `temper` binary, located by Cargo at compile time — the custom-kind
/// acceptance drives it to observe the process exit code.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-acceptance-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Project an imported skill to its authored surface member document
/// `<skill.id>/SKILL.md` (`Member::to_document`) and reload it through the generic
/// `Unit` loader `check` reads. The surface directory is named for the skill so the
/// generic id matches the imported member; `placement` reads the imported source
/// directory off the preserved provenance, not this scratch directory.
fn skill_surface_unit(skill: &Member) -> Unit {
    let dir = tmpdir(&format!("surface-{}", skill.id)).join(&skill.id);
    fs::create_dir_all(&dir).unwrap();
    let doc_path = dir.join("SKILL.md");
    fs::write(&doc_path, skill.to_document().emit()).unwrap();
    Unit::from_member_document(&dir, &doc_path).unwrap()
}

/// Path to a directory under `tests/fixtures`, resolved from the manifest so the
/// test is independent of the process working directory.
fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(rel)
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
    let rules_root = fixture("rules");
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
        let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
        let skill = Member::from_source(&skill_kind, &dir.join("SKILL.md"))
            .expect("fixture skill should parse");
        // Read features off the projected surface member document through the generic
        // `Unit` loader `check` uses — no IR→Unit adapter. `placement` still reads the
        // imported source directory off provenance, so `name-matches-dir` is unchanged.
        let unit = skill_surface_unit(&skill);
        let features = builtin_kind::skill_features(&unit);
        let diagnostics = engine::validate(&contract, std::slice::from_ref(&features));
        report.push_str(&format!("## {name}\n"));
        report.push_str(&render_diagnostics(&diagnostics));
        report.push('\n');
    }

    insta::assert_snapshot!("rules_check_diagnostics", report);
}

/// The `skill` built-in kind's declaration row this fixture's emit payload carries.
fn skill_kind_facts() -> KindFactRow {
    KindFactRow {
        name: "skill".to_string(),
        provider: Some("claude-code".to_string()),
        governs_root: ".claude/skills".to_string(),
        governs_glob: "*/SKILL.md".to_string(),
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("directory".to_string()),
        registration: Some("description-trigger(description)".to_string()),
        templates: Vec::new(),
    }
}

/// The slice acceptance, end to end: the well-formed `coordinate` fixture skill checks
/// clean over its projected surface member document, and compiling its seam payload
/// twice (`emit` is the sole producer, `specs/architecture/20-surface.md`, "The lock and
/// drift") reproduces the projection with no diff.
#[test]
fn acceptance_check_then_reemit_is_a_no_diff() {
    let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
    let skill = Member::from_source(&skill_kind, &fixture("coordinate").join("SKILL.md")).unwrap();

    // check — a well-formed skill trips no contract clause, so it is clean. The gate
    // reads each skill's surface member document through the one generic `Unit` loader
    // (`specs/architecture/15-kinds.md`, "A built-in kind is an adapter").
    let unit = skill_surface_unit(&skill);
    let features = [builtin_kind::skill_features(&unit)];
    let diagnostics = engine::validate(&builtin_skill_contract(), &features);
    assert!(
        diagnostics.is_empty(),
        "the coordinate skill must check clean, got {diagnostics:?}",
    );
    assert!(!check::any_error(&diagnostics));

    // emit — a hand-built seam payload over the same fixture skill, compiled twice: the
    // second compile reproduces the harness projection byte-for-byte.
    let harness = tmpdir("acceptance-harness");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![skill_kind_facts()],
            ..Declarations::default()
        },
        members: vec![PayloadMember {
            kind: "skill".to_string(),
            name: skill.id.clone(),
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

/// The authored `spec` KIND.md definition (`specs/architecture/20-surface.md`, "Decision: a kind
/// definition is `KIND.md`"): it governs `specs/*.md` and composes the spec extractor
/// (line count, ATX headings, placement) — markdown structure only, no body-mined
/// references (`specs/architecture/15-kinds.md`, "Decision: no body-mined references").
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

/// The authored `spec` **package** — the require-side the kind binds
/// (`specs/architecture/40-composition.md`): one advisory `max_lines` clause. The shipped budget is
/// ~150 (`90-spec-system.md`); this fixture uses a small one so a short over-length spec
/// trips it without a 150-line corpus.
const SPEC_PACKAGE_MD: &str = "+++\n\
[[clause]]\n\
severity = \"advisory\"\n\
predicate = \"max_lines\"\n\
max = 10\n\
+++\n\
\n\
# The spec package\n\
\n\
The require-side of the spec kind.\n";

/// Author a custom kind's definition + package under `<corpus>/.temper/` — the
/// authored half of the assembly `import` and `check` read.
fn author_custom_kind(corpus: &Path, name: &str, kind_md: &str, package_md: &str) {
    let temper = corpus.join(".temper");
    let kind_dir = temper.join("kinds").join(name);
    fs::create_dir_all(&kind_dir).unwrap();
    fs::write(kind_dir.join("KIND.md"), kind_md).unwrap();
    let pkg_dir = temper.join("packages").join(name);
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(pkg_dir.join("PACKAGE.md"), package_md).unwrap();
}

/// A spec body over the fixture's 10-line `max_lines` budget — used to prove the
/// advisory fires (and, under `--deny-advisories`, blocks).
fn over_length_spec() -> String {
    let mut body = String::from("# Kinds\n");
    for line in 1..=40 {
        body.push_str(&format!("Line {line} of an over-budget spec body.\n"));
    }
    body
}

/// Run the built binary `temper check <workspace> [extra…]` from `cwd` and return
/// whether it exited zero.
fn check_from(cwd: &Path, workspace: &Path, extra: &[&str]) -> bool {
    Command::new(BIN)
        .current_dir(cwd)
        .arg("check")
        .arg(workspace)
        .args(extra)
        .status()
        .unwrap()
        .success()
}

/// The custom-kind acceptance (`specs/architecture/15-kinds.md`, "Worked example: `spec`"):
/// over a corpus carrying an authored `spec` kind + package (no built-in kind's members
/// resolve here), `--deny-advisories` flips the exit code, since the empty-corpus
/// coverage note is itself warn-severity — the flag-flip proof pattern this suite's
/// other custom-kind case also drives.
#[test]
fn check_dispatches_the_spec_custom_kind_through_its_extractor_and_contract() {
    let corpus = tmpdir("spec-corpus");
    author_custom_kind(&corpus, "spec", SPEC_KIND_MD, SPEC_PACKAGE_MD);
    let specs = corpus.join("specs");
    fs::create_dir_all(&specs).unwrap();
    // A short spec (clean) beside an over-length one (trips the advisory budget).
    fs::write(specs.join("00-intent.md"), "# Intent\n\nThe north star.\n").unwrap();
    fs::write(specs.join("15-kinds.md"), over_length_spec()).unwrap();

    let into = corpus.join(".temper");

    assert!(
        check_from(&corpus, &into, &[]),
        "an advisory-only spec violation must exit zero without --deny-advisories"
    );
    assert!(
        !check_from(&corpus, &into, &["--deny-advisories"]),
        "the over-length spec must exit non-zero under --deny-advisories"
    );
}

/// A custom kind authored **outside** `specs/` (`adr/*.md`), the same shape as the
/// `spec` case above and driven for the same reason: `--deny-advisories` flips the
/// exit code over the empty-corpus coverage note's warn severity.
#[test]
fn check_reads_a_custom_kind_rooted_outside_specs() {
    let corpus = tmpdir("adr-corpus");
    let adr_kind_md = "+++\n\
governs = { root = \"adr\", glob = \"*.md\" }\n\
\n\
[[extraction]]\n\
primitive = \"line_count\"\n\
+++\n\
\n\
# The adr kind\n\
\n\
Architecture decision records.\n";
    let adr_package_md = "+++\n\
[[clause]]\n\
severity = \"advisory\"\n\
predicate = \"max_lines\"\n\
max = 10\n\
+++\n\
\n\
# The adr package\n";
    author_custom_kind(&corpus, "adr", adr_kind_md, adr_package_md);
    let adrs = corpus.join("adr");
    fs::create_dir_all(&adrs).unwrap();
    // A short ADR (clean) beside an over-length one (trips the advisory budget).
    fs::write(adrs.join("0001-short.md"), "# ADR 1\n\nDecided.\n").unwrap();
    fs::write(adrs.join("0002-long.md"), over_length_spec()).unwrap();

    let into = corpus.join(".temper");

    assert!(
        check_from(&corpus, &into, &[]),
        "an advisory-only ADR violation must exit zero without --deny-advisories"
    );
    assert!(
        !check_from(&corpus, &into, &["--deny-advisories"]),
        "the over-length ADR must exit non-zero under --deny-advisories"
    );
}
