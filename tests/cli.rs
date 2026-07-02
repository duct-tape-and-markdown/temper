//! End-to-end CLI acceptance over the documented surface (`specs/20-surface.md`,
//! "CLI surface"; `specs/10-contracts.md`, the contract engine `check` runs).
//!
//! Spawns the built `temper` binary via `CARGO_BIN_EXE_temper` and drives the
//! documented round trip — `temper import <harness> --into <tmp>` then
//! `temper check <tmp>` — asserting the exit semantics: zero on a clean skill,
//! non-zero once a `required`-severity contract clause is violated. A
//! `--deny-advisories` case pins the strict policy: an advisory-only run exits
//! zero by default but non-zero under the flag. A final case pins the default
//! workspace: with `--into` / the `check` argument omitted, both resolve to
//! `./.temper` under the process's working directory.
//!
//! These checks live here (not in a `src` unit test) precisely because the exit
//! code is observable only across a real process boundary — `process::ExitCode`
//! is surfaced by `main`, not returned by the library.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-cli-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A skill that trips no `error`-severity rule: the `name` is valid and matches
/// its directory, the description is present and short, and the body references
/// no files. (`when` / `not` keep even the description advisories quiet.)
const CLEAN_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill that violates `required` clauses: the uppercase `name` is outside
/// `[a-z0-9-]` (the `allowed_chars` clause) and no longer equals its directory
/// (the `name-matches-dir` clause). Both are required ⇒ a non-zero exit.
const ERROR_SKILL: &str = "---\n\
name: Coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill clean but for its over-budget body: every `required` clause holds
/// (lowercase `name` matching its directory, a present short description, no
/// forbidden keys), and the only violation is the advisory `max_lines` budget
/// (warn). That isolates the `--deny-advisories` promotion.
fn advisory_only_skill() -> String {
    let mut body = String::from("# Coordinate\n");
    for line in 1..=600 {
        body.push_str(&format!("Line {line} of an over-budget body.\n"));
    }
    format!(
        "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
{body}"
    )
}

/// Write a one-skill harness at `<root>/skills/<name>/SKILL.md`.
fn write_harness(root: &Path, name: &str, skill_md: &str) {
    let dir = root.join(".claude").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();
}

/// A rule that trips no `required` clause: `paths:`-only frontmatter (Claude
/// Code's real scoping key) and a short body — the clean shape of `rust.md`.
const CLEAN_RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

/// A rule that violates the `forbidden_keys` clause: a Cursor `.mdc` key
/// (`globs`) Claude Code silently ignores — the exact mistake the rule contract
/// exists to catch. That clause is `required` ⇒ a non-zero exit.
const FORBIDDEN_KEY_RULE: &str = "---\n\
globs: \"**/*.rs\"\n\
alwaysApply: true\n\
---\n\
# Rust conventions\n\
\n\
This frontmatter loads nothing in Claude Code.\n";

/// Write a one-rule harness at `<root>/.claude/rules/<name>.md` — the location
/// `import` scans for the rule kind (`specs/20-surface.md`).
fn write_rule_harness(root: &Path, name: &str, rule_md: &str) {
    let dir = root.join(".claude").join("rules");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(format!("{name}.md")), rule_md).unwrap();
}

/// Run `temper import <harness> --into <into>` and assert it succeeded.
fn import(harness: &Path, into: &Path) {
    let status = Command::new(BIN)
        .arg("import")
        .arg(harness)
        .arg("--into")
        .arg(into)
        .status()
        .unwrap();
    assert!(status.success(), "import should succeed: {status}");
}

/// Run `temper check <workspace> [extra…]` and return whether it exited zero.
///
/// Runs with the CWD set to the workspace itself, which carries no `temper.toml`,
/// so the run exercises the pure by-kind floor — the same CWD-isolation the
/// `schema` tests use for the same reason. Without it, an ambient project
/// `temper.toml` at the process CWD (e.g. temper's own, which registers the `spec`
/// custom kind) would leak into a foreign workspace that lacks that kind's
/// definition and abort the load — an artifact of the test harness, not the floor
/// behaviour these cases pin.
fn run_check(workspace: &Path, extra: &[&str]) -> bool {
    Command::new(BIN)
        .current_dir(workspace)
        .arg("check")
        .arg(workspace)
        .args(extra)
        .status()
        .unwrap()
        .success()
}

/// Run `temper check <workspace>` and return whether it exited zero.
fn check_succeeds(workspace: &Path) -> bool {
    run_check(workspace, &[])
}

#[test]
fn import_then_check_is_clean_for_a_well_formed_skill() {
    let harness = tmpdir("clean-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);
    let into = tmpdir("clean-into");

    import(&harness, &into);
    assert!(
        check_succeeds(&into),
        "a clean skill must exit zero (no error-severity diagnostics)"
    );
}

#[test]
fn check_exits_non_zero_when_an_error_rule_fires() {
    let harness = tmpdir("error-src");
    // Directory `coordinate` but `name: Coordinate` — trips name-format and
    // name-matches-dir, both error severity.
    write_harness(&harness, "coordinate", ERROR_SKILL);
    let into = tmpdir("error-into");

    import(&harness, &into);
    assert!(
        !check_succeeds(&into),
        "an error-severity diagnostic must make check exit non-zero"
    );
}

#[test]
fn deny_advisories_promotes_a_warn_only_run_to_a_failure() {
    let harness = tmpdir("advisory-src");
    // The only clause this skill violates is the advisory `max_lines` budget.
    write_harness(&harness, "coordinate", &advisory_only_skill());
    let into = tmpdir("advisory-into");

    import(&harness, &into);
    // Default policy: an advisory-only run is clean — warn does not gate.
    assert!(
        check_succeeds(&into),
        "an advisory-only violation must exit zero without --deny-advisories"
    );
    // Strict policy: --deny-advisories promotes the warn to a blocking failure.
    assert!(
        !run_check(&into, &["--deny-advisories"]),
        "an advisory-only violation must exit non-zero under --deny-advisories"
    );
}

#[test]
fn import_then_check_dispatches_the_rule_kind_to_the_rule_contract() {
    // A clean rule (`paths:`-only) trips no `required` clause ⇒ check is zero.
    let clean_src = tmpdir("rule-clean-src");
    write_rule_harness(&clean_src, "rust", CLEAN_RULE);
    let clean_into = tmpdir("rule-clean-into");
    import(&clean_src, &clean_into);
    assert!(
        check_succeeds(&clean_into),
        "a clean rule must exit zero — the rule contract has no `required` violation"
    );

    // A forbidden Cursor key (`globs`/`alwaysApply`) trips the `forbidden_keys`
    // clause, which is `required` ⇒ check is non-zero. This proves `check`
    // dispatches the rule kind to the rule contract, not the skill one.
    let forbidden_src = tmpdir("rule-forbidden-src");
    write_rule_harness(&forbidden_src, "rust", FORBIDDEN_KEY_RULE);
    let forbidden_into = tmpdir("rule-forbidden-into");
    import(&forbidden_src, &forbidden_into);
    assert!(
        !check_succeeds(&forbidden_into),
        "a forbidden-key rule must exit non-zero (the rule contract's required clause)"
    );
}

/// Run `temper check --harness <harness>` (the one-shot wedge) and return
/// `(exit-zero, stdout)`.
fn run_check_harness(harness: &Path) -> (bool, String) {
    let output = Command::new(BIN)
        .arg("check")
        .arg("--harness")
        .arg(harness)
        .output()
        .unwrap();
    (
        output.status.success(),
        String::from_utf8(output.stdout).unwrap(),
    )
}

#[test]
fn check_harness_one_shot_lints_a_raw_harness_without_a_workspace() {
    // The zero-config wedge (`specs/20-surface.md`, "CLI surface" — `check --harness`
    // is the one-shot mode): a raw harness is linted directly, no `import` step, and
    // no surface workspace is written. A forbidden Cursor key trips a `required`
    // clause ⇒ non-zero, and the finding is on stdout.
    let harness = tmpdir("one-shot-src");
    write_rule_harness(&harness, "rust", FORBIDDEN_KEY_RULE);

    let (ok, stdout) = run_check_harness(&harness);
    assert!(
        !ok,
        "check --harness must exit non-zero on a required-clause violation"
    );
    assert!(
        stdout.contains("forbidden_keys"),
        "the finding must reach stdout, got:\n{stdout}"
    );
    // One-shot means no workspace ceremony: the harness is imported internally into a
    // scratch dir, so no `.temper` surface is left beside it.
    assert!(
        !harness.join(".temper").exists(),
        "check --harness must not write a surface workspace beside the harness"
    );

    // A clean harness over the same one-shot path exits zero.
    let clean = tmpdir("one-shot-clean");
    write_rule_harness(&clean, "rust", CLEAN_RULE);
    let (ok, _) = run_check_harness(&clean);
    assert!(ok, "check --harness over a clean harness must exit zero");
}

#[test]
fn check_rejects_a_harness_and_workspace_together() {
    // `--harness` and the positional workspace are the two mutually-exclusive routes
    // into the gate; supplying both is a usage error, not a silent precedence pick.
    let ws = tmpdir("conflict-ws");
    let harness = tmpdir("conflict-harness");
    let status = Command::new(BIN)
        .arg("check")
        .arg(&ws)
        .arg("--harness")
        .arg(&harness)
        .status()
        .unwrap();
    assert!(
        !status.success(),
        "check <workspace> --harness <path> must be a usage error"
    );
}

#[test]
fn self_host_check_is_clean_over_tempers_own_rules() {
    // The bootstrap proof (`specs/00-intent.md`): import `temper`'s OWN repo —
    // whose `.claude/rules/` carries `rust.md` (`paths:`) and `collaboration.md`
    // (no frontmatter) — and `check` its own house clean. `CARGO_MANIFEST_DIR` is
    // the crate root, the harness root `import` scans for `.claude/rules/`.
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let into = tmpdir("self-host-into");
    import(repo_root, &into);
    assert!(
        check_succeeds(&into),
        "temper must lint its own .claude/rules/ clean — the self-hosting finish line"
    );
}

#[test]
fn schema_kind_skill_emits_the_skill_floor_decidable_clauses() {
    // Run in a fresh CWD with no `temper.toml`, so the emitted schema is the pure
    // skill floor (no author layer) and the assertions are deterministic.
    let cwd = tmpdir("schema-skill");
    let output = Command::new(BIN)
        .current_dir(&cwd)
        .arg("schema")
        .arg("--kind")
        .arg("skill")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "temper schema --kind skill must exit zero"
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    // A JSON object over frontmatter.
    assert!(
        stdout.contains("\"type\": \"object\""),
        "schema must be an object schema, got:\n{stdout}"
    );
    // The skill floor requires `name` (required → required[]).
    assert!(
        stdout.contains("\"required\""),
        "the skill floor's `required` clause must project a required[] array"
    );
    // `allowed_chars` on `name` → a generated `pattern` charclass.
    assert!(
        stdout.contains("\"pattern\""),
        "the skill floor's allowed_chars clause must project a `pattern`"
    );
    // `forbidden_keys` (globs / alwaysApply) → a `not`/`required` combinator per key.
    assert!(
        stdout.contains("globs") && stdout.contains("alwaysApply"),
        "the skill floor's forbidden_keys must appear as forbidden-key combinators"
    );

    // It parses back as JSON — a well-formed schema, not just a string that looks
    // like one.
    serde_json::from_str::<serde_json::Value>(&stdout).expect("emitted schema must be valid JSON");
}

#[test]
fn schema_kind_skill_emits_guidance_as_the_docs_channel_description() {
    // The docs (hover) channel of the emitted schema (`specs/50-distribution.md`,
    // "The gate at keystroke"): a field clause's `guidance` prose rides its JSON
    // Schema property's `description`, strictly alongside the validation keywords.
    // The embedded `skill.anthropic` built-in now carries guidance on its clauses
    // (`specs/10-contracts.md`, the `contracts/` retirement into product source), so
    // the pure floor — no `temper.toml` layer — already exercises both channels.
    let cwd = tmpdir("schema-guidance");
    let output = Command::new(BIN)
        .current_dir(&cwd)
        .arg("schema")
        .arg("--kind")
        .arg("skill")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "temper schema --kind skill must exit zero"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Docs channel: the floor's `name` guidance rides its `description`, beside its
    // validation keywords (`maxLength`/`pattern` et al.), never mixed into them.
    assert!(
        json["properties"]["name"]["description"].is_string(),
        "the floor's name guidance must ride the property description, got:\n{stdout}"
    );
    assert_eq!(json["properties"]["name"]["maxLength"], 64);
    assert!(json["properties"]["name"]["pattern"].is_string());

    // The prose stays in the docs channel — it never became a validation keyword.
    assert!(json["properties"]["name"].get("enum").is_none());
    assert!(json["properties"]["name"].get("const").is_none());
    // Guidance never leaks to the schema root, only onto property `description`s.
    assert!(json.get("description").is_none());
}

#[test]
fn schema_without_kind_maps_every_modeled_kind() {
    let cwd = tmpdir("schema-all");
    let output = Command::new(BIN)
        .current_dir(&cwd)
        .arg("schema")
        .output()
        .unwrap();
    assert!(output.status.success(), "temper schema must exit zero");

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    // A by-kind map: each modeled kind resolves to its own object schema.
    assert_eq!(json["skill"]["type"], "object");
    assert_eq!(json["rule"]["type"], "object");
}

#[test]
fn schema_rejects_an_unknown_kind() {
    let cwd = tmpdir("schema-unknown");
    let status = Command::new(BIN)
        .current_dir(&cwd)
        .arg("schema")
        .arg("--kind")
        .arg("nonesuch")
        .status()
        .unwrap();
    assert!(
        !status.success(),
        "an unknown kind must be a hard error, not a silent empty schema"
    );
}

/// Run `temper diff <harness> --into <ws>` and return `(exit-zero, stdout)`.
fn run_diff(harness: &Path, into: &Path) -> (bool, String) {
    let output = Command::new(BIN)
        .arg("diff")
        .arg(harness)
        .arg("--into")
        .arg(into)
        .output()
        .unwrap();
    (
        output.status.success(),
        String::from_utf8(output.stdout).unwrap(),
    )
}

/// A recursive snapshot of every file under `dir` as relative-path -> bytes, so a
/// read-only command can be proven to leave the tree untouched.
fn snapshot(dir: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    let mut out = std::collections::BTreeMap::new();
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

#[test]
fn diff_reports_the_four_states_and_writes_nothing() {
    // A harness with two skills and a rule, freshly imported into the surface.
    // Each skill's frontmatter `name` matches its directory (and the two differ),
    // so they project to distinct surface artifacts.
    let review_skill = CLEAN_SKILL.replacen("coordinate", "review", 1);
    let harness = tmpdir("diff-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);
    write_harness(&harness, "review", &review_skill);
    write_rule_harness(&harness, "rust", CLEAN_RULE);
    let into = tmpdir("diff-into");
    import(&harness, &into);

    // Unchanged, freshly-imported harness: every artifact is in-sync.
    let (ok, stdout) = run_diff(&harness, &into);
    assert!(ok, "diff over a clean harness must exit zero");
    assert!(
        stdout.lines().all(|line| line.contains("in-sync")),
        "every line should report in-sync, got:\n{stdout}"
    );
    assert_eq!(stdout.lines().count(), 3, "one line per imported artifact");

    // Mutate the harness three ways: edit one source, add a new one, delete one.
    let coordinate_md = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let edited = fs::read_to_string(&coordinate_md).unwrap() + "\nAn extra line.\n";
    fs::write(&coordinate_md, edited).unwrap();
    write_rule_harness(&harness, "extra", CLEAN_RULE);
    fs::remove_dir_all(harness.join(".claude").join("skills").join("review")).unwrap();

    // The surface is unchanged, so the report reflects each on-disk mutation.
    let before = snapshot(&into);
    let (ok, stdout) = run_diff(&harness, &into);
    assert!(ok, "diff is read-only — it always exits zero");

    let line_for = |name: &str| -> String {
        stdout
            .lines()
            .find(|line| line.split_whitespace().nth(2) == Some(name))
            .unwrap_or_else(|| panic!("no line for {name} in:\n{stdout}"))
            .to_string()
    };
    assert!(
        line_for("coordinate").contains("drifted"),
        "edited source drifts"
    );
    assert!(
        line_for("rust").contains("in-sync"),
        "untouched source stays in-sync"
    );
    assert!(
        line_for("extra").contains("added"),
        "a new on-disk source is added"
    );
    assert!(
        line_for("review").contains("removed"),
        "a deleted source is removed"
    );

    // Read-only: not a byte of the surface workspace changed.
    assert_eq!(before, snapshot(&into), "diff must write nothing");
}

/// Run `temper apply --into <ws> [extra…]` and return `(exit-zero, stdout)`.
fn run_apply(into: &Path, extra: &[&str]) -> (bool, String) {
    let output = Command::new(BIN)
        .arg("apply")
        .arg("--into")
        .arg(into)
        .args(extra)
        .output()
        .unwrap();
    (
        output.status.success(),
        String::from_utf8(output.stdout).unwrap(),
    )
}

#[test]
fn apply_projects_a_surface_edit_and_dry_run_writes_nothing() {
    // Import a clean skill, then edit its description in the surface `SKILL.md`
    // document — exactly the field a human recomposes.
    let harness = tmpdir("apply-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);
    let into = tmpdir("apply-into");
    import(&harness, &into);

    let document = into.join("skills").join("coordinate").join("SKILL.md");
    let edited = fs::read_to_string(&document)
        .unwrap()
        .replace("Use when coordinating", "Use when orchestrating");
    fs::write(&document, edited).unwrap();

    let source = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");

    // A dry run reports the pending write but touches nothing on disk.
    let before = snapshot(&harness);
    let (ok, stdout) = run_apply(&into, &["--dry-run"]);
    assert!(ok, "apply --dry-run must exit zero");
    assert!(
        stdout.contains("dry run") && stdout.contains("applied"),
        "the dry run must report the pending apply, got:\n{stdout}"
    );
    assert_eq!(before, snapshot(&harness), "a dry run writes nothing");

    // The real apply lands the edited field on the harness source.
    let (ok, stdout) = run_apply(&into, &[]);
    assert!(ok, "apply must exit zero");
    assert!(
        stdout.contains("applied"),
        "the report names the applied skill"
    );
    assert!(
        fs::read_to_string(&source)
            .unwrap()
            .contains("Use when orchestrating"),
        "the edited description must be projected onto the source"
    );

    // Re-running is idempotent — the second apply finds nothing to do.
    let (ok, stdout) = run_apply(&into, &[]);
    assert!(ok, "the re-run must exit zero");
    assert!(
        stdout.contains("unchanged"),
        "a re-applied surface reports unchanged, got:\n{stdout}"
    );
}

#[test]
fn into_and_workspace_default_to_dot_author() {
    // With `--into` omitted, import writes to `./.temper` relative to the
    // process CWD; with the `check` argument omitted, check reads the same path.
    let cwd = tmpdir("default-cwd");
    let harness = tmpdir("default-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);

    let import_status = Command::new(BIN)
        .current_dir(&cwd)
        .arg("import")
        .arg(&harness)
        .status()
        .unwrap();
    assert!(
        import_status.success(),
        "default-into import should succeed"
    );

    // The default surface landed under `<cwd>/.temper`.
    let default_ws = cwd.join(".temper");
    assert!(
        default_ws.join("lock.toml").is_file(),
        "import without --into must resolve to ./.temper"
    );
    assert!(
        default_ws
            .join("skills")
            .join("coordinate")
            .join("SKILL.md")
            .is_file()
    );

    // `check` with no argument reads that same `./.temper` and finds it clean.
    let check_status = Command::new(BIN)
        .current_dir(&cwd)
        .arg("check")
        .status()
        .unwrap();
    assert!(
        check_status.success(),
        "check without an argument must lint ./.temper and exit zero"
    );
}

/// Write a custom-kind member surface at `<root>/.temper/specs/<id>/SPEC.md` — a
/// provenance-only `+++` header over the byte-faithful body, the shape `import`
/// projects a custom unit into (`src/import.rs`). The member id is the surface
/// directory name, which the backtick-filename reference syntax names exactly.
fn write_spec_member(root: &Path, id: &str, body: &str) {
    let dir = root.join(".temper").join("specs").join(id);
    fs::create_dir_all(&dir).unwrap();
    let document = format!(
        "+++\n[provenance]\nsource_path = \"specs/{id}.md\"\nimport_hash = \"deadbeef\"\n+++\n{body}"
    );
    fs::write(dir.join("SPEC.md"), document).unwrap();
}

/// Author a fixture workspace registering a custom `spec` kind whose members
/// reference one another, then run `temper check` over it from `root` (so the
/// sibling `temper.toml` is discovered). `reference` is the backtick token
/// `intro.spec`'s body cites — a real member id resolves, a bogus one dangles.
/// Returns whether the run exited zero and its combined stdout+stderr.
fn check_custom_spec_graph(label: &str, reference: &str) -> (bool, String) {
    let root = tmpdir(label);

    // The assembly registers the `spec` custom kind and binds its package by name.
    fs::write(
        root.join("temper.toml"),
        "[kind.spec]\npackage = \"spec\"\n",
    )
    .unwrap();

    // The authored kind definition: a `references` extractor feeding a `ref` feature,
    // and a `[[relationships]]` edge declaring that `ref` resolves to another `spec`.
    let kind_dir = root.join(".temper").join("kinds").join("spec");
    fs::create_dir_all(&kind_dir).unwrap();
    fs::write(
        kind_dir.join("KIND.md"),
        "+++\n\
         governs = { root = \"specs\", glob = \"*.md\" }\n\
         \n\
         [[extraction]]\n\
         primitive = \"references\"\n\
         feature = \"ref\"\n\
         \n\
         [[relationships]]\n\
         field = \"ref\"\n\
         to = \"spec\"\n\
         +++\n\
         # The spec kind\n\
         \n\
         Specs reference one another by backtick filename.\n",
    )
    .unwrap();

    // A trivial bound package: the kind's require-side must resolve for the run to be
    // green, but this fixture exercises the graph, not the clause engine — so it
    // carries no clauses.
    let pkg_dir = root.join(".temper").join("packages").join("spec");
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(
        pkg_dir.join("PACKAGE.md"),
        "+++\n+++\n# The spec package\n\nNo clauses — the graph tier is what this pins.\n",
    )
    .unwrap();

    // Two members on the surface: a leaf `core.spec` and an `intro.spec` that cites
    // `reference`. The declared `ref` edge resolves when the citation names a real id.
    write_spec_member(&root, "core.spec", "# Core\n\nA leaf spec.\n");
    write_spec_member(
        &root,
        "intro.spec",
        &format!("# Intro\n\nSee `{reference}` for details.\n"),
    );

    let out = Command::new(BIN)
        .current_dir(&root)
        .arg("check")
        .output()
        .unwrap();
    let mut output = String::from_utf8_lossy(&out.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&out.stderr));
    (out.status.success(), output)
}

#[test]
fn a_custom_kind_member_participates_in_the_reference_graph() {
    // A declared edge from a custom-kind member to a real target resolves through the
    // same generic graph the built-in kinds use ⇒ green; a dangling one is a finding
    // ⇒ non-zero. This proves custom-kind members join the `by_kind` corpus and their
    // `KIND.md` relationships reach the reference graph (`specs/15-kinds.md`, "The
    // entity graph is a kind capability"). Reference-value normalization (stripping
    // `.md`) is a separate downstream concern; here a citation names a member id
    // exactly, so the graph's exact-match resolution applies uniformly to both kinds.
    let (resolving_ok, resolving_out) = check_custom_spec_graph("graph-resolves", "core.spec");
    assert!(
        resolving_ok,
        "a custom-kind member's edge to a real target must keep check green, got:\n{resolving_out}"
    );

    let (dangling_ok, dangling_out) = check_custom_spec_graph("graph-dangles", "missing.spec");
    assert!(
        !dangling_ok,
        "a custom-kind member's dangling edge must fail the run ⇒ non-zero, got:\n{dangling_out}"
    );
    assert!(
        dangling_out.contains("missing.spec") && dangling_out.contains("spec"),
        "the finding names the dangling target and its target kind, got:\n{dangling_out}"
    );
}
