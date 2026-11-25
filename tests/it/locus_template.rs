//! A file locus is a placement template: load refuses a custom kind whose `governs` glob
//! carries any pattern syntax beyond literal segments, one `*`, and a leading `**/`, and
//! every built-in kind's glob loads under the same rule. A locus less its declared
//! excluded paths — a directory among them excluding what lies beneath it — leaves them
//! ungoverned at discovery, `check` and both guard edges.

use std::collections::BTreeMap;

use temper::coverage_note;
use temper::drift::{self, Declarations, EmitOptions, KindFactRow, Payload};
use temper::import;
use temper::kind::{CustomKind, EXCLUDE_COLUMN, PLACEMENT_TEMPLATE_COLUMN};

use crate::common;

/// Run `check` over a lock declaring one custom `domain` kind at `docs` + `glob`, and
/// assert the run fails loud at load naming both the glob and the template rule.
fn assert_refused_at_load(case: &str, glob: &str) {
    let root = common::tmpdir(case);
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![common::kind_facts("domain", "docs", glob)],
            ..Declarations::default()
        },
    );
    let run = common::check_in(&root, &[], Some("github"));
    assert!(
        !run.ok,
        "a non-template locus `{glob}` must refuse at load, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains(glob),
        "the refusal names the offending glob, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains(PLACEMENT_TEMPLATE_COLUMN),
        "the refusal names the placement-template rule, got:\n{}",
        run.output
    );
}

#[test]
fn a_locus_glob_with_a_character_class_refuses_at_load() {
    assert_refused_at_load("locus-template-class", "domains/[a-z0-9-]*.md");
}

#[test]
fn a_locus_glob_with_an_extglob_refuses_at_load() {
    assert_refused_at_load("locus-template-extglob", "!(README).md");
}

#[test]
fn a_locus_glob_with_two_stars_refuses_at_load() {
    assert_refused_at_load("locus-template-two-stars", "*/*.md");
}

#[test]
fn a_locus_glob_with_braces_refuses_at_load() {
    assert_refused_at_load("locus-template-braces", "*.{md,txt}");
}

#[test]
fn every_built_in_kind_glob_is_a_placement_template() {
    let definitions = temper::builtin_kind::definitions();
    let mut seen = 0;
    for (name, kind) in &definitions {
        let Some(governs) = &kind.governs else {
            continue;
        };
        seen += 1;
        let row = common::kind_facts(name, &governs.root, &governs.glob);
        CustomKind::from_kind_fact_row(&row).unwrap_or_else(|err| {
            panic!(
                "built-in `{name}`'s glob `{}` loads as a template: {err}",
                governs.glob
            )
        });
    }
    assert!(
        seen > 0,
        "the built-in population governs at least one locus"
    );
}

/// A `domain` kind at `domains` + `*.md` excluding `domains/README.md`.
fn domain_excluding_readme() -> KindFactRow {
    KindFactRow {
        unit_shape: Some("file".to_string()),
        exclude: vec!["domains/README.md".to_string()],
        ..common::kind_facts("domain", "domains", "*.md")
    }
}

/// A harness holding the excluded README beside one undeclared domain document.
fn domain_harness(case: &str) -> std::path::PathBuf {
    let root = common::tmpdir(case);
    common::write_sibling(&root, "domains/README.md", "# Domains\n");
    common::write_sibling(&root, "domains/stray.md", "# Stray\n");
    root
}

#[test]
fn an_excluded_path_at_a_kinds_locus_is_not_discovered() {
    let root = domain_harness("locus-exclude-discovery");
    let kind = CustomKind::from_kind_fact_row(&domain_excluding_readme()).unwrap();
    let disc = import::Discovery::new(&root);
    let files = import::discover_kind_files(
        &disc,
        &kind,
        kind.governs.as_ref().unwrap(),
        import::LocalOverride::Honored,
    );
    let names: Vec<_> = files
        .iter()
        .map(|file| file.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        names,
        ["stray.md"],
        "the README is excluded, the stray is not"
    );
}

#[test]
fn an_excluded_path_draws_no_locus_declared_finding() {
    let root = domain_harness("locus-exclude-check");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![domain_excluding_readme()],
            clauses: vec![common::clause("locus-declared", "required")],
            ..Declarations::default()
        },
    );
    let run = common::check_in(&root, &[], Some("github"));
    assert!(
        run.output.contains("domains/stray.md"),
        "the undeclared document at the locus is found, got:\n{}",
        run.output
    );
    assert!(
        !run.output.contains("domains/README.md"),
        "the excluded README draws no finding, got:\n{}",
        run.output
    );
}

#[test]
fn the_guard_binds_nothing_for_a_write_at_an_excluded_path() {
    let root = domain_harness("locus-exclude-guard");
    common::GuardLock::declaring("block")
        .kind_row(domain_excluding_readme())
        .member("rule", "rust", ".claude/rules/rust.md", "abc", "abc")
        .write(&root);

    let (code, output) =
        common::run_guard(&root, &common::guard_write_payload("domains/README.md"));
    assert_eq!(
        code,
        Some(0),
        "a write at the excluded path is allowed, got: {output}"
    );
    let (code, output) = common::run_guard(&root, &common::guard_write_payload("domains/stray.md"));
    assert_eq!(
        code,
        Some(2),
        "a write at the locus is still bound, got: {output}"
    );

    // The shell edge enumerates the locus instead of testing one path: the stray is named,
    // the README is not.
    let post = "{\"session_id\":\"s\",\"hook_event_name\":\"PostToolUse\",\
                \"tool_name\":\"Bash\",\"tool_input\":{\"command\":\"true\"},\
                \"tool_response\":{\"stdout\":\"\",\"stderr\":\"\",\"interrupted\":false}}";
    let (_, output) = common::run_guard(&root, post);
    assert!(
        output.contains("domains/stray.md"),
        "the shell edge names the stray, got: {output}"
    );
    assert!(
        !output.contains("domains/README.md"),
        "and never the excluded README, got: {output}"
    );
}

#[test]
fn an_exclusion_that_is_a_pattern_or_off_the_locus_refuses_at_load() {
    for path in ["domains/*.md", "docs/README.md", "domains/sub/README.md"] {
        let row = KindFactRow {
            exclude: vec![path.to_string()],
            ..domain_excluding_readme()
        };
        let err = CustomKind::from_kind_fact_row(&row)
            .expect_err("a non-path or off-locus exclusion must refuse");
        let message = err.to_string();
        assert!(
            message.contains(path) && message.contains(EXCLUDE_COLUMN),
            "the refusal names `{path}` and the rule, got: {message}"
        );
    }
}

#[test]
fn emit_refuses_to_place_a_member_at_a_path_its_kind_excludes() {
    // Discovery never walks an excluded path, so a member placed there would be written
    // and locked yet never found: emit asks the same membership question and refuses.
    let root = common::tmpdir("locus-exclude-emit");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![domain_excluding_readme()],
            ..Declarations::default()
        },
        members: vec![common::payload_member(
            "domain",
            "README",
            None,
            "# Domains\n",
        )],
    };
    let err = drift::emit(&payload, &root.join(".temper"), EmitOptions::default())
        .expect_err("a placement at an excluded path must refuse");
    let message = format!("{err:?}");
    assert!(
        message.contains("domains/README.md") && message.contains("excludes"),
        "the refusal names the excluded path, got: {message}"
    );
    assert!(
        !root.join("domains/README.md").exists(),
        "nothing is written at the excluded path"
    );

    // A sibling the locus admits still places.
    let payload = Payload {
        members: vec![common::payload_member("domain", "intro", None, "# Intro\n")],
        ..payload
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default())
        .expect("an admitted placement emits");
    assert!(root.join("domains/intro.md").is_file());
}

#[test]
fn the_coverage_advisory_does_not_count_a_path_its_kind_excludes_as_governed() {
    // A kind at `.claude` + `*.md` excluding `.claude/README.md`: the README is no
    // member, so the stray scan names it; its sibling the kind admits stays governed.
    let root = common::tmpdir("locus-exclude-coverage");
    common::write_sibling(&root, ".claude/README.md", "# Readme\n");
    common::write_sibling(&root, ".claude/notes.md", "# Notes\n");
    let row = KindFactRow {
        unit_shape: Some("file".to_string()),
        exclude: vec![".claude/README.md".to_string()],
        ..common::kind_facts("note", ".claude", "*.md")
    };
    let kinds = BTreeMap::from([(
        "note".to_string(),
        CustomKind::from_kind_fact_row(&row).unwrap(),
    )]);
    let diagnostics = coverage_note::check(
        &root,
        &kinds,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
    );
    let unclaimed: Vec<&str> = diagnostics
        .iter()
        .filter(|d| d.rule == "coverage.unclaimed-entry")
        .map(|d| &*d.artifact)
        .collect();
    assert_eq!(
        unclaimed,
        [".claude/README.md"],
        "the excluded README is unclaimed, the admitted sibling is governed"
    );
}

/// A `skill`-shaped kind at `skills` + `*/SKILL.md` excluding the directory `exclude`.
fn skill_excluding(exclude: &str) -> KindFactRow {
    KindFactRow {
        unit_shape: Some("directory".to_string()),
        exclude: vec![exclude.to_string()],
        ..common::kind_facts("tool", "skills", "*/SKILL.md")
    }
}

#[test]
fn an_excluded_directory_leaves_every_path_beneath_it_ungoverned() {
    let root = common::tmpdir("locus-exclude-directory");
    common::write_sibling(&root, "skills/legacy/SKILL.md", "# Legacy\n");
    common::write_sibling(&root, "skills/live/SKILL.md", "# Live\n");
    let kind = CustomKind::from_kind_fact_row(&skill_excluding("skills/legacy")).unwrap();

    // Discovery walks past the excluded directory.
    let disc = import::Discovery::new(&root);
    let files = import::discover_kind_files(
        &disc,
        &kind,
        kind.governs.as_ref().unwrap(),
        import::LocalOverride::Honored,
    );
    let relative: Vec<_> = files
        .iter()
        .map(|file| {
            file.strip_prefix(temper::path::normalize_path(&root))
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect();
    assert_eq!(relative, ["skills/live/SKILL.md"]);

    // Membership — the one home every surface asks — subtracts the whole directory, by
    // whole components only.
    assert!(!kind.locus_admits("skills/legacy/SKILL.md"));
    assert!(kind.locus_admits("skills/live/SKILL.md"));
    assert!(kind.locus_admits("skills/legacy-two/SKILL.md"));

    // Check binds nothing beneath it.
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![skill_excluding("skills/legacy")],
            clauses: vec![common::clause("locus-declared", "required")],
            ..Declarations::default()
        },
    );
    let run = common::check_in(&root, &[], Some("github"));
    assert!(
        run.output.contains("skills/live/SKILL.md"),
        "the undeclared member at the locus is found, got:\n{}",
        run.output
    );
    assert!(
        !run.output.contains("skills/legacy"),
        "nothing beneath the excluded directory draws a finding, got:\n{}",
        run.output
    );

    // The guard binds nothing for a write beneath it, and still binds the live member.
    let root = common::tmpdir("locus-exclude-directory-guard");
    common::GuardLock::declaring("block")
        .kind_row(skill_excluding("skills/legacy"))
        .member("rule", "rust", ".claude/rules/rust.md", "abc", "abc")
        .write(&root);
    let (code, output) = common::run_guard(
        &root,
        &common::guard_write_payload("skills/legacy/SKILL.md"),
    );
    assert_eq!(
        code,
        Some(0),
        "a write beneath the excluded directory is allowed, got: {output}"
    );
    let (code, output) =
        common::run_guard(&root, &common::guard_write_payload("skills/live/SKILL.md"));
    assert_eq!(
        code,
        Some(2),
        "a write at the locus is still bound, got: {output}"
    );
}

#[test]
fn an_exclude_naming_a_directory_the_template_reaches_beneath_loads() {
    for dir in ["skills/legacy", "skills"] {
        CustomKind::from_kind_fact_row(&skill_excluding(dir))
            .unwrap_or_else(|err| panic!("`{dir}` is a directory the template reaches: {err}"));
    }
    // Under a leading `**/`, a directory at any depth is reached.
    let row = KindFactRow {
        exclude: vec!["vendor/deep".to_string()],
        ..common::kind_facts("memo", ".", "**/MEMO.md")
    };
    CustomKind::from_kind_fact_row(&row).expect("an any-depth locus reaches every directory");
}

#[test]
fn an_exclude_naming_a_directory_the_template_never_reaches_refuses_at_load() {
    for dir in [
        "skills/legacy/assets",
        "docs",
        "skills/legacy/",
        "skills//legacy",
    ] {
        let err = CustomKind::from_kind_fact_row(&skill_excluding(dir))
            .expect_err("a directory the template never reaches beneath must refuse");
        let message = err.to_string();
        assert!(
            message.contains(dir) && message.contains(EXCLUDE_COLUMN),
            "the refusal names `{dir}` and the rule, got: {message}"
        );
        assert!(
            message.contains("or a directory it reaches beneath"),
            "the rule names the directory form an author may spell, got: {message}"
        );
    }
}
