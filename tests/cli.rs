//! End-to-end CLI acceptance over the documented surface (`specs/architecture/20-surface.md`,
//! "CLI surface"; `specs/architecture/10-contracts.md`, the contract engine `check` runs).
//!
//! Spawns the built `temper` binary via `CARGO_BIN_EXE_temper` and drives the
//! documented on-ramp — `temper init <harness>` then `temper check` from the
//! harness root — asserting the exit semantics: zero on a clean skill, non-zero
//! once a `required`-severity contract clause is violated. `init` scans the harness
//! into a manifest over its members **in place** (no `.temper/` copy tree,
//! byte-identical members), and `check` live-extracts those members from their
//! landscape files. A `--deny-advisories` case pins the strict policy; a final case
//! pins the in-place default (`init` with no path scans the current directory).
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

/// Write a one-skill harness at `<root>/.claude/skills/<name>/SKILL.md`.
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
/// `init` scans for the rule kind (`specs/architecture/20-surface.md`).
fn write_rule_harness(root: &Path, name: &str, rule_md: &str) {
    let dir = root.join(".claude").join("rules");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(format!("{name}.md")), rule_md).unwrap();
}

/// Run `temper init <harness>` and assert it succeeded — the on-ramp writes the
/// manifest over the harness's members in place (no `.temper/` copy tree).
fn init(harness: &Path) {
    let status = Command::new(BIN).arg("init").arg(harness).status().unwrap();
    assert!(status.success(), "init should succeed: {status}");
}

/// Run `temper check [extra…]` from the harness `root` and return whether it
/// exited zero.
///
/// The CWD is the harness root itself — the manifest `init` wrote lives there, and
/// its in-place `[[member]]` tables name their landscape files *relative to the
/// harness*, so the gate resolves them from the CWD. The harness carries no ambient
/// `temper.toml` beyond the one `init` wrote, so the run exercises the pure by-kind
/// floor over the live-extracted in-place members.
fn check_at(root: &Path, extra: &[&str]) -> bool {
    Command::new(BIN)
        .current_dir(root)
        .arg("check")
        .args(extra)
        .status()
        .unwrap()
        .success()
}

/// Run `temper check` from the harness `root` and return whether it exited zero.
fn check_succeeds(root: &Path) -> bool {
    check_at(root, &[])
}

#[test]
fn init_then_check_is_clean_for_a_well_formed_skill() {
    let harness = tmpdir("clean-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);

    init(&harness);
    assert!(
        check_succeeds(&harness),
        "a clean skill must exit zero (no error-severity diagnostics)"
    );
}

#[test]
fn check_exits_non_zero_when_an_error_rule_fires() {
    let harness = tmpdir("error-src");
    // Directory `coordinate` but `name: Coordinate` — trips name-format and
    // name-matches-dir, both error severity.
    write_harness(&harness, "coordinate", ERROR_SKILL);

    init(&harness);
    assert!(
        !check_succeeds(&harness),
        "an error-severity diagnostic must make check exit non-zero"
    );
}

#[test]
fn deny_advisories_promotes_a_warn_only_run_to_a_failure() {
    let harness = tmpdir("advisory-src");
    // The only clause this skill violates is the advisory `max_lines` budget.
    write_harness(&harness, "coordinate", &advisory_only_skill());

    init(&harness);
    // Default policy: an advisory-only run is clean — warn does not gate.
    assert!(
        check_succeeds(&harness),
        "an advisory-only violation must exit zero without --deny-advisories"
    );
    // Strict policy: --deny-advisories promotes the warn to a blocking failure.
    assert!(
        !check_at(&harness, &["--deny-advisories"]),
        "an advisory-only violation must exit non-zero under --deny-advisories"
    );
}

#[test]
fn init_then_check_dispatches_the_rule_kind_to_the_rule_contract() {
    // A clean rule (`paths:`-only) trips no `required` clause ⇒ check is zero.
    let clean = tmpdir("rule-clean-src");
    write_rule_harness(&clean, "rust", CLEAN_RULE);
    init(&clean);
    assert!(
        check_succeeds(&clean),
        "a clean rule must exit zero — the rule contract has no `required` violation"
    );

    // A forbidden Cursor key (`globs`/`alwaysApply`) trips the `forbidden_keys`
    // clause, which is `required` ⇒ check is non-zero. This proves `check`
    // dispatches the rule kind to the rule contract, not the skill one.
    let forbidden = tmpdir("rule-forbidden-src");
    write_rule_harness(&forbidden, "rust", FORBIDDEN_KEY_RULE);
    init(&forbidden);
    assert!(
        !check_succeeds(&forbidden),
        "a forbidden-key rule must exit non-zero (the rule contract's required clause)"
    );
}

#[test]
fn init_leaves_members_byte_identical_in_place_and_check_reads_the_manifest() {
    // The on-ramp's core invariant (`specs/architecture/20-surface.md`, "Decision: `init` is
    // the on-ramp"): scan into a manifest over members IN PLACE — zero file moves,
    // byte-identical members, no `.temper/` copy tree — and `check` reads that
    // manifest green by live-extracting each in-place member.
    let harness = tmpdir("inplace-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);
    let skill_md = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let before = fs::read(&skill_md).unwrap();

    init(&harness);

    // The member is untouched in place — not a byte moved or reformatted.
    assert_eq!(
        fs::read(&skill_md).unwrap(),
        before,
        "init must leave the member byte-identical in place"
    );
    // No copy tree: the manifest lands beside the harness, the member stays put.
    assert!(
        !harness.join(".temper").exists(),
        "init must write no `.temper/` copy tree"
    );
    let manifest = fs::read_to_string(harness.join("temper.toml")).unwrap();
    assert!(
        manifest.contains("source = \".claude/skills/coordinate/SKILL.md\""),
        "the in-place member records its landscape source, got:\n{manifest}"
    );

    // The gate reads the manifest and live-extracts the in-place member clean.
    assert!(
        check_succeeds(&harness),
        "check must read the in-place manifest green"
    );
}

#[test]
fn an_in_place_member_cannot_drift() {
    // In-place members live-extract from their landscape file, so an edit to that
    // file is picked up on the next check — never a `config.stale` finding (there is
    // no projection to diverge from). "In-place members cannot drift"
    // (`specs/architecture/20-surface.md`).
    let root = tmpdir("no-drift");
    write_harness(&root, "coordinate", CLEAN_SKILL);
    init(&root);

    let clean = Command::new(BIN)
        .current_dir(&root)
        .arg("check")
        .output()
        .unwrap();
    assert!(
        clean.status.success(),
        "a fresh in-place manifest checks green"
    );
    assert!(
        !String::from_utf8_lossy(&clean.stdout).contains("config.stale"),
        "an in-place member carries no stale-projection fact"
    );

    // Edit the landscape file (still a clean skill); the next check re-extracts it
    // live and stays green, never stale.
    let skill_md = root
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let edited = fs::read_to_string(&skill_md)
        .unwrap()
        .replace("Drive the team", "Drive the crew");
    fs::write(&skill_md, edited).unwrap();

    let after = Command::new(BIN)
        .current_dir(&root)
        .arg("check")
        .output()
        .unwrap();
    let out = String::from_utf8_lossy(&after.stdout);
    assert!(
        after.status.success(),
        "an in-place edit stays green (live re-extraction), got:\n{out}"
    );
    assert!(
        !out.contains("config.stale"),
        "an in-place member cannot drift, got:\n{out}"
    );
}

#[test]
fn init_lift_migrates_one_member_into_a_richer_carriage() {
    // `init --lift <member>` migrates one in-place member into a richer carriage — into
    // document carriage (`specs/architecture/20-surface.md`, "adoption is a gradient"): the
    // body rides byte-identical, the framing normalizes, and the manifest entry
    // flips from a `source`-bearing in-place table to the pre-extracted document form.
    let harness = tmpdir("lift-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);
    init(&harness);

    // Before the lift: the member is in-place (a `source` path, no baked features).
    let before = fs::read_to_string(harness.join("temper.toml")).unwrap();
    assert!(before.contains("source = \".claude/skills/coordinate/SKILL.md\""));

    let status = Command::new(BIN)
        .arg("init")
        .arg(&harness)
        .arg("--lift")
        .arg("coordinate")
        .status()
        .unwrap();
    assert!(status.success(), "init --lift should succeed: {status}");

    // After: the member is document-carried — the manifest bakes its features and no
    // longer names a `source`, and the projected document exists under `.temper/`.
    let after = fs::read_to_string(harness.join("temper.toml")).unwrap();
    assert!(
        !after.contains("source ="),
        "the lifted member no longer carries a `source`, got:\n{after}"
    );
    assert!(
        after.contains("[member.field]"),
        "the lifted member is pre-extracted (a `[member.field]` table), got:\n{after}"
    );
    assert!(
        harness
            .join(".temper")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md")
            .is_file(),
        "the lift projects the member into document carriage under `.temper/`"
    );

    // The migrated member still checks green.
    assert!(
        check_succeeds(&harness),
        "check must read the lifted member green"
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
    // The zero-config wedge (`specs/architecture/20-surface.md`, "CLI surface" — `check --harness`
    // is the one-shot mode): a raw harness is linted directly, no `init` step, and
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
fn self_host_check_is_clean_over_tempers_own_surface() {
    // The bootstrap proof (`specs/intent/00-intent.md`): `temper check` over temper's
    // OWN committed surface — its `.temper/` document-carried rules plus the `temper.toml`
    // assembly (spec kinds, requirements) — lints clean. `CARGO_MANIFEST_DIR` is the
    // crate root; a bare `check` reads the committed surface there, read-only, so the
    // repo is never mutated (the flume `temper check (self)` gate's exact invocation).
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let status = Command::new(BIN)
        .current_dir(repo_root)
        .arg("check")
        .status()
        .unwrap();
    assert!(
        status.success(),
        "temper must lint its own committed surface clean — the self-hosting finish line"
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
    // The docs (hover) channel of the emitted schema (`specs/architecture/50-distribution.md`,
    // "The gate at keystroke"): a field clause's `guidance` prose rides its JSON
    // Schema property's `description`, strictly alongside the validation keywords.
    // The embedded `skill.anthropic` built-in now carries guidance on its clauses
    // (`specs/architecture/10-contracts.md`, the `contracts/` retirement into product source), so
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
    // A by-kind map keyed by each kind's qualified identity (`specs/architecture/15-kinds.md`,
    // "a published package binds a qualified kind name"): each resolves to its own schema.
    assert_eq!(json["claude-code.skill"]["type"], "object");
    assert_eq!(json["claude-code.rule"]["type"], "object");
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

#[test]
fn init_defaults_to_the_current_directory_and_writes_no_copy_tree() {
    // With the harness path omitted, `init` scans the current directory in place
    // (`specs/architecture/20-surface.md`, `init [<harness-path>]`): the manifest lands at
    // `<cwd>/temper.toml`, no `./.temper` copy tree, and `check` reads it green.
    let cwd = tmpdir("default-cwd");
    write_harness(&cwd, "coordinate", CLEAN_SKILL);

    let init_status = Command::new(BIN)
        .current_dir(&cwd)
        .arg("init")
        .status()
        .unwrap();
    assert!(init_status.success(), "default-path init should succeed");

    // The manifest landed in place; no copy tree was written.
    assert!(
        cwd.join("temper.toml").is_file(),
        "init without a path must write ./temper.toml"
    );
    assert!(
        !cwd.join(".temper").exists(),
        "in-place init writes no copy tree"
    );

    // `check` with no argument reads that same manifest and finds it clean.
    let check_status = Command::new(BIN)
        .current_dir(&cwd)
        .arg("check")
        .status()
        .unwrap();
    assert!(
        check_status.success(),
        "check without an argument must lint the in-place manifest and exit zero"
    );
}
