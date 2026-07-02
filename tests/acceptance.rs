//! Acceptance over the documented round trip (`specs/20-surface.md`, "CLI
//! surface"; `specs/10-contracts.md`, the contract engine).
//!
//! Pins the whole vertical slice — typed IR, sidecar topology, contract engine,
//! diagnostics UX — driving the library `import` plus the generic
//! `engine::validate` directly (logic lives in the lib per `.claude/rules/rust.md`,
//! so no binary harness is needed):
//!
//! - an `insta` snapshot of the **import surface** over a trimmed, real-shaped
//!   copy of the `coordinate` skill, asserted byte-stable across a re-import;
//! - an `insta` snapshot of the **check diagnostics** the built-in skill contract
//!   produces over the deliberately broken `tests/fixtures/rules/*` tree — the
//!   reduced, decidable-only surviving-clause set;
//! - the slice acceptance end to end: `import <fixture>` then validate reproduces
//!   the expected diagnostics, and re-running `import` produces no diff;
//! - the custom-kind acceptance (`specs/15-kinds.md`, "Worked example: `spec`"):
//!   over a corpus whose `temper.toml` declares the `spec` kind, `temper check`
//!   dispatches each spec through its composed extractor and contract — an
//!   over-length spec trips the advisory `max_lines`, exiting zero absent
//!   `--deny-advisories`. That case drives the built binary, since the exit code
//!   (and reading `temper.toml` off the process cwd) is observable only across a
//!   real process boundary.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::check::{self, Diagnostic, Severity, Workspace};
use temper::contract::Contract;
use temper::engine;
use temper::extract::{self, Features};
use temper::import;
use temper::skill::Skill;

/// The built-in Anthropic skill contract, resolved from the embedded `packages/`
/// std-lib exactly as the shipped `check` does — so the acceptance path validates
/// against the same clauses the tool ships (`specs/10-contracts.md`, the
/// `contracts/` retirement: the built-in resolves from the embedded set by name).
fn builtin_skill_contract() -> Contract {
    temper::builtin::contract(temper::builtin::SKILL_PACKAGE)
        .expect("the embedded skill package should parse")
        .expect("the skill package is embedded")
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

/// Path to a directory under `tests/fixtures`, resolved from the manifest so the
/// test is independent of the process working directory.
fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(rel)
}

/// Render an imported surface tree as a single reviewable string: each file as a
/// `--- <relative/path> ---` header (forward slashes) followed by its UTF-8
/// contents, files sorted by path. Two imports rendering identically *is* the
/// byte-stable / no-diff contract.
fn render_surface(dir: &Path) -> String {
    let mut files = BTreeMap::new();
    for entry in walkdir::WalkDir::new(dir).min_depth(1).sort_by_file_name() {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(dir)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        files.insert(rel, fs::read_to_string(entry.path()).unwrap());
    }

    let mut out = String::new();
    for (rel, body) in files {
        out.push_str(&format!("--- {rel} ---\n"));
        out.push_str(&body);
        if !body.ends_with('\n') {
            out.push('\n');
        }
    }
    out
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

/// The `source_path` recorded in `meta.toml` / `lock.toml` is the absolute
/// origin of the fixture, which varies per machine. Redact just that prefix so
/// the surface snapshot pins everything content-derived (hashes, body, header)
/// without pinning an unstable absolute path.
fn surface_filters() -> Vec<(&'static str, &'static str)> {
    vec![(
        r#"source_path = "[^"]*tests/fixtures/coordinate/SKILL\.md""#,
        r#"source_path = "[ROOT]/tests/fixtures/coordinate/SKILL.md""#,
    )]
}

/// The import surface over the trimmed `coordinate` fixture is exactly the golden
/// below, and re-importing into the same workspace changes not one byte.
#[test]
fn import_surface_is_byte_stable() {
    let into = tmpdir("coordinate-into");

    import::run(&fixture("coordinate"), &into).unwrap();
    let first = render_surface(&into);

    insta::with_settings!({filters => surface_filters()}, {
        insta::assert_snapshot!("coordinate_import_surface", first);
    });

    // Re-import into the same workspace: idempotence means an identical tree.
    import::run(&fixture("coordinate"), &into).unwrap();
    let second = render_surface(&into);
    assert_eq!(first, second, "re-import must produce no diff");
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
        let skill = Skill::from_source_dir(dir).expect("fixture skill should parse");
        let features = extract::skill_features(&skill);
        let diagnostics = engine::validate(&contract, std::slice::from_ref(&features));
        report.push_str(&format!("## {name}\n"));
        report.push_str(&render_diagnostics(&diagnostics));
        report.push('\n');
    }

    insta::assert_snapshot!("rules_check_diagnostics", report);
}

/// The slice acceptance, end to end: `import <fixture>` then validate over the
/// written surface reproduces the expected diagnostics (the well-formed
/// `coordinate` skill is clean), and a second `import` produces no diff.
#[test]
fn acceptance_import_check_then_reimport_is_a_no_diff() {
    let into = tmpdir("acceptance-into");

    // import <fixture> --into <tmp>
    import::run(&fixture("coordinate"), &into).unwrap();
    let first = render_surface(&into);

    // check <tmp> — a well-formed skill trips no contract clause, so it is clean.
    let ws = Workspace::load(&into).unwrap();
    let features: Vec<Features> = ws.skills.iter().map(extract::skill_features).collect();
    let diagnostics = engine::validate(&builtin_skill_contract(), &features);
    assert!(
        diagnostics.is_empty(),
        "the trimmed coordinate skill must check clean, got {diagnostics:?}",
    );
    assert!(!check::any_error(&diagnostics));

    // re-import — no diff.
    import::run(&fixture("coordinate"), &into).unwrap();
    assert_eq!(
        first,
        render_surface(&into),
        "re-import must produce no diff"
    );
}

/// A `temper.toml` *registering* the `spec` custom kind exactly as temper's own does
/// (`specs/40-composition.md`, "Decision: a custom kind is an authored `.temper/`
/// artifact, registered in the assembly"): the whole require-side wiring is the package
/// binding. The definition and the package are authored artifacts, below.
const SPEC_TEMPER_TOML: &str = "[kind.spec]\npackage = \"spec\"\n";

/// The authored `spec` KIND.md definition (`specs/20-surface.md`, "Decision: a kind
/// definition is `KIND.md`"): it governs `specs/*.md` and composes the spec extractor
/// (line count, ATX headings, placement) — markdown structure only, no body-mined
/// references (`specs/15-kinds.md`, "Decision: no body-mined references").
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
/// (`specs/40-composition.md`): one advisory `max_lines` clause. The shipped budget is
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

/// Author a registered custom kind's definition + package under `<corpus>/.temper/`,
/// beside the `temper.toml` registration — the authored half of the assembly `import`
/// and `check` read.
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

/// Run the built binary `temper check <workspace> [extra…]` from `cwd` — so the
/// project-root `temper.toml` is discovered at the process working directory —
/// and return whether it exited zero.
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

/// The custom-kind acceptance (`specs/15-kinds.md`, "Worked example: `spec`"):
/// over a corpus whose `temper.toml` declares the `spec` kind, `temper check`
/// dispatches each spec through the composed data extractor and the kind's own
/// contract. The over-length spec trips the advisory `max_lines` (warn), which
/// does not gate — so the run exits zero absent `--deny-advisories` and non-zero
/// under it. That the flag flips the exit is proof the spec contract actually
/// fired over the extracted features, not that the run was silently clean.
#[test]
fn check_dispatches_the_spec_custom_kind_through_its_extractor_and_contract() {
    let corpus = tmpdir("spec-corpus");
    fs::write(corpus.join("temper.toml"), SPEC_TEMPER_TOML).unwrap();
    author_custom_kind(&corpus, "spec", SPEC_KIND_MD, SPEC_PACKAGE_MD);
    let specs = corpus.join("specs");
    fs::create_dir_all(&specs).unwrap();
    // A short spec (clean) beside an over-length one (trips the advisory budget).
    fs::write(specs.join("00-intent.md"), "# Intent\n\nThe north star.\n").unwrap();
    fs::write(specs.join("15-kinds.md"), over_length_spec()).unwrap();

    // import discovers the `spec` kind from the corpus `temper.toml` and writes
    // each spec into the surface — the units the extractor reads at check time.
    let into = corpus.join(".temper");
    import::run(&corpus, &into).unwrap();

    // check from the corpus dir: `temper.toml` at the cwd declares the spec kind,
    // so the run projects each spec through the composed extractor and validates it
    // against the kind's contract. The only violation is the advisory `max_lines`.
    assert!(
        check_from(&corpus, &into, &[]),
        "an advisory-only spec violation must exit zero without --deny-advisories"
    );
    assert!(
        !check_from(&corpus, &into, &["--deny-advisories"]),
        "the over-length spec must exit non-zero under --deny-advisories"
    );
}

/// A custom kind rooted **outside** `specs/` — proof the check path loads units
/// from a *generic* surface loader keyed on each kind's declared `governs.root`,
/// not the retired `root == "specs"` special case that read `Workspace.specs`
/// (`specs/40-composition.md`, "Declaring a custom kind"). The `adr` kind governs
/// `adr/*.md`; `check` reads its units from `<ws>/adr/*` through
/// `Unit::from_surface_dir`, so its contract fires over the extracted features
/// exactly as the `spec` kind's does — a root the built-in `Workspace` never
/// materializes into `ws.specs`, so under the old special case it contributed no
/// units and the advisory could never fire.
#[test]
fn check_reads_a_custom_kind_rooted_outside_specs() {
    let corpus = tmpdir("adr-corpus");
    fs::write(
        corpus.join("temper.toml"),
        "[kind.adr]\npackage = \"adr\"\n",
    )
    .unwrap();
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

    // import discovers the `adr` kind from the corpus `temper.toml` and writes each
    // unit to `<into>/adr/<name>/` — a root the built-in `Workspace` never reads.
    let into = corpus.join(".temper");
    import::run(&corpus, &into).unwrap();

    // check from the corpus dir: the generic loader keys on `governs.root = "adr"`,
    // so the over-length ADR trips the advisory `max_lines`. The flag flipping the
    // exit is proof the contract fired over units read from outside `specs/`.
    assert!(
        check_from(&corpus, &into, &[]),
        "an advisory-only ADR violation must exit zero without --deny-advisories"
    );
    assert!(
        !check_from(&corpus, &into, &["--deny-advisories"]),
        "the over-length ADR must exit non-zero under --deny-advisories"
    );
}
