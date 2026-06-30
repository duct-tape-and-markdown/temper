//! Slice-1 acceptance (`spec/RELEASE-v0.1.md`, "Tests / acceptance").
//!
//! Pins the whole vertical slice — typed IR, sidecar topology, lint engine,
//! diagnostics UX — against the documented round trip, driving the library
//! `import`/`check` functions directly (logic lives in the lib per
//! `.claude/rules/rust.md`, so no binary harness is needed):
//!
//! - an `insta` snapshot of the **import surface** over a trimmed, real-shaped
//!   copy of the `coordinate` skill, asserted byte-stable across a re-import;
//! - an `insta` snapshot of the **check diagnostics** over the deliberately
//!   broken `tests/fixtures/rules/*` tree — the expected diagnostic set;
//! - the slice acceptance end to end: `import <fixture>` then `check` reproduces
//!   the expected diagnostics, and re-running `import` produces no diff.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use author::check::{self, Diagnostic, Severity, Workspace};
use author::import;
use author::rules::all_rules;
use author::skill::Skill;

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

/// The `source_path` recorded in `meta.toml` / `author.toml` is the absolute
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

/// Loading the deliberately-broken `tests/fixtures/rules/*` tree and running the
/// full rule set reproduces the expected diagnostic set — every rule firing on
/// its own fixture, the `clean` control silent.
#[test]
fn check_reproduces_the_expected_diagnostic_set() {
    let rules_root = fixture("rules");
    let mut fixtures: Vec<PathBuf> = fs::read_dir(&rules_root)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect();
    fixtures.sort();

    let mut report = String::new();
    let rules = all_rules();
    for dir in &fixtures {
        let name = dir.file_name().unwrap().to_string_lossy();
        let skill = Skill::from_source_dir(dir).expect("fixture skill should parse");
        let ws = Workspace {
            skills: vec![skill],
        };
        let diagnostics = check::run(&ws, &rules);
        report.push_str(&format!("## {name}\n"));
        report.push_str(&render_diagnostics(&diagnostics));
        report.push('\n');
    }

    insta::assert_snapshot!("rules_check_diagnostics", report);
}

/// The slice acceptance, end to end: `import <fixture>` then `check` over the
/// written surface reproduces the expected diagnostics (the well-formed
/// `coordinate` skill is clean), and a second `import` produces no diff.
#[test]
fn acceptance_import_check_then_reimport_is_a_no_diff() {
    let into = tmpdir("acceptance-into");

    // import <fixture> --into <tmp>
    import::run(&fixture("coordinate"), &into).unwrap();
    let first = render_surface(&into);

    // check <tmp> — a well-formed skill trips no rule, so the run is clean.
    let ws = Workspace::load(&into).unwrap();
    let diagnostics = check::run(&ws, &all_rules());
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
