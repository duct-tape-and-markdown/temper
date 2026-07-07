//! `temper emit` — the seam's compile (`specs/model/pipeline.md`, "Emit" —
//! total, byte-reproducible, refusing).
//!
//! Three tiers:
//!
//! - **the compiler**, `drift::emit`, driven directly over hand-built [`drift::Payload`]
//!   values — no `node` involved — proving the properties the entry names: every
//!   projection and the whole five-family lock compile from the payload alone;
//!   double-emit (a second compile of the same payload) reproduces every byte;
//!   a hand-edited projection is overwritten, never merged (drift routed to the
//!   source); `--dry-run` reports outcomes but writes nothing; an unknown kind or
//!   an unsupported seam version is a clear refusal.
//! - **the seam**, `drift::emit_program`, driven once end-to-end over a real `node`
//!   subprocess running the built SDK against a fixture `harness.ts` — proving
//!   `emit` actually executes the SDK program and that a second, independent
//!   process run reproduces the same projections and lock byte-for-byte.
//! - **the one-shot gate**, `check --harness` / session-start, driven across the real
//!   process boundary over a raw harness with no lock and no `.temper/` — proving
//!   the copy-tree scratch import is gone: the discovery walk is the only member
//!   extractor, straight off harness disk (`specs/model/pipeline.md`, "Decision:
//!   one authored surface, one implementation").

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::drift::{
    self, Declarations, EmitOptions, EmitOutcome, KindFactRow, Payload, PayloadMember,
};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-emit-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Snapshot every file under `dir` as a sorted map of relative path -> bytes.
fn tree_bytes(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
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

/// The outcome `emit` reported for `name` in `report`, asserting it is unique.
fn outcome(report: &drift::EmitReport, name: &str) -> EmitOutcome {
    let mut matches = report.entries.iter().filter(|e| e.name == name);
    let found = matches.next().expect("entry should exist");
    assert!(matches.next().is_none(), "entry {name} should be unique");
    found.outcome
}

// ---------------------------------------------------------------------------
// The compiler — `drift::emit` over hand-built payloads, no `node` involved.
// ---------------------------------------------------------------------------

fn rule_kind_facts() -> KindFactRow {
    KindFactRow {
        name: "rule".to_string(),
        provider: None,
        governs_root: ".claude/rules".to_string(),
        governs_glob: "*.md".to_string(),
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("file".to_string()),
        registration: None,
        templates: Vec::new(),
    }
}

fn skill_kind_facts() -> KindFactRow {
    KindFactRow {
        name: "skill".to_string(),
        provider: None,
        governs_root: ".claude/skills".to_string(),
        governs_glob: "*/SKILL.md".to_string(),
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("directory".to_string()),
        registration: None,
        templates: Vec::new(),
    }
}

fn rule_member(name: &str, paths: Option<&[&str]>, body: &str) -> PayloadMember {
    let mut fields = Vec::new();
    if let Some(paths) = paths {
        fields.push(("paths".to_string(), serde_json::json!(paths)));
    }
    PayloadMember {
        kind: "rule".to_string(),
        name: name.to_string(),
        fields,
        body: body.to_string(),
        source_path: None,
    }
}

fn skill_member(name: &str, description: &str, body: &str) -> PayloadMember {
    PayloadMember {
        kind: "skill".to_string(),
        name: name.to_string(),
        fields: vec![
            ("name".to_string(), serde_json::json!(name)),
            ("description".to_string(), serde_json::json!(description)),
        ],
        body: body.to_string(),
        source_path: None,
    }
}

/// A rule + skill payload, declarations carrying just their kind facts.
fn basic_payload(members: Vec<PayloadMember>) -> Payload {
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![rule_kind_facts(), skill_kind_facts()],
            ..Default::default()
        },
        members,
    }
}

/// A fresh `<harness>/.temper` pair — `drift::emit` derives the projection root
/// from the workspace dir's parent, matching the seam's own topology
/// (`specs/model/pipeline.md`): `.temper/` sits beside `.claude/`.
fn workspace(label: &str) -> (PathBuf, PathBuf) {
    let harness = tmpdir(label);
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    (harness, into)
}

const RUST_BODY: &str =
    "# Rust conventions\n\nErrors via miette/thiserror; clippy clean under -D warnings.\n";
const COORDINATE_BODY: &str = "# Coordinate\n\nDrive the team.\n";

#[test]
fn emit_compiles_every_projection_and_the_whole_lock_from_the_payload() {
    let (harness, into) = workspace("compile");
    let payload = basic_payload(vec![
        rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY),
        skill_member(
            "coordinate",
            "Use when coordinating agents across axes.",
            COORDINATE_BODY,
        ),
    ]);

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Emitted);
    assert_eq!(outcome(&report, "coordinate"), EmitOutcome::Emitted);

    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    assert_eq!(
        fs::read_to_string(&rule_path).unwrap(),
        format!("---\npaths: [\"src/**/*.rs\"]\n---\n{RUST_BODY}")
    );

    let skill_path = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    assert_eq!(
        fs::read_to_string(&skill_path).unwrap(),
        format!(
            "---\nname: \"coordinate\"\ndescription: \"Use when coordinating agents across axes.\"\n---\n{COORDINATE_BODY}"
        )
    );

    // The lock carries a rollup row per member, kind-then-name ordered, plus the
    // declaration-kind family the payload's own `kinds` carried.
    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(lock.contains("[[rule]]\n"), "rollup: {lock}");
    assert!(lock.contains("[[skill]]\n"), "rollup: {lock}");
    assert!(
        lock.contains("[[declaration.kind]]\n"),
        "declarations: {lock}"
    );
}

#[test]
fn emit_writes_all_five_declaration_families_the_payload_carries() {
    let (_harness, into) = workspace("five-families");
    let mut payload = basic_payload(vec![rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY)]);
    payload.declarations.clauses.push(drift::ClauseRow {
        kind: Some("rule".to_string()),
        predicate: "required".to_string(),
        field: Some("paths".to_string()),
        severity: "required".to_string(),
        guidance: None,
        cite: None,
        count: None,
        target: None,
        degree: None,
        bound: None,
        charset: None,
        keys: None,
        values: None,
    });
    payload
        .declarations
        .requirements
        .push(drift::RequirementRow {
            name: "dev-standards".to_string(),
            kind: Some("rule".to_string()),
            required: true,
            clauses: Vec::new(),
            verified_by: None,
        });
    payload.declarations.assembly.push(drift::AssemblyFactRow {
        fact: "authority".to_string(),
        value: Some("warn".to_string()),
        from: None,
        field: None,
        to: None,
    });
    payload.declarations.satisfies.push(drift::SatisfiesRow {
        member: "rust".to_string(),
        requirement: "dev-standards".to_string(),
    });

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    for header in [
        "[[declaration.kind]]",
        "[[declaration.clause]]",
        "[[declaration.requirement]]",
        "[[declaration.assembly]]",
        "[[declaration.satisfies]]",
    ] {
        assert!(lock.contains(header), "missing {header} in:\n{lock}");
    }
}

#[test]
fn emit_is_idempotent_over_an_unchanged_payload() {
    let (harness, into) = workspace("idem");
    let payload = basic_payload(vec![rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY)]);

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let after_first = tree_bytes(&harness);
    let lock_after_first = fs::read(into.join("lock.toml")).unwrap();

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Unchanged);
    assert_eq!(
        after_first,
        tree_bytes(&harness),
        "a second emit over the same payload changes not a byte"
    );
    assert_eq!(
        lock_after_first,
        fs::read(into.join("lock.toml")).unwrap(),
        "double emit reproduces the lock byte-for-byte"
    );
}

#[test]
fn a_changed_payload_field_re_emits_the_projection() {
    let (harness, into) = workspace("reemit");
    let first = basic_payload(vec![rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY)]);
    drift::emit(&first, &into, EmitOptions::default()).unwrap();

    let second = basic_payload(vec![rule_member(
        "rust",
        Some(&["src/**/*.rs", "tests/**/*.rs"]),
        RUST_BODY,
    )]);
    let report = drift::emit(&second, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Emitted);

    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    let emitted = fs::read_to_string(&rule_path).unwrap();
    assert!(emitted.contains("\"tests/**/*.rs\""), "got:\n{emitted}");
}

#[test]
fn a_hand_edited_projection_is_overwritten_not_conflicted() {
    let (harness, into) = workspace("hand-edit");
    let payload = basic_payload(vec![
        rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY),
        skill_member(
            "coordinate",
            "Use when coordinating agents across axes.",
            COORDINATE_BODY,
        ),
    ]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    let canonical = fs::read_to_string(&rule_path).unwrap();
    fs::write(
        &rule_path,
        canonical.clone() + "\nA line added straight to disk.\n",
    )
    .unwrap();

    // Emit re-emits the projection whole: the hand edit is overwritten (drift routed
    // to the source), never merged — there is no three-state conflict here.
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Emitted);
    assert_eq!(fs::read_to_string(&rule_path).unwrap(), canonical);
    // The untouched skill is already at its fixpoint.
    assert_eq!(outcome(&report, "coordinate"), EmitOutcome::Unchanged);
}

#[test]
fn dry_run_reports_the_outcome_but_writes_nothing() {
    let (harness, into) = workspace("dry");
    let first = basic_payload(vec![rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY)]);
    drift::emit(&first, &into, EmitOptions::default()).unwrap();

    let before_harness = tree_bytes(&harness);
    let before_lock = fs::read(into.join("lock.toml")).unwrap();

    let second = basic_payload(vec![rule_member(
        "rust",
        Some(&["src/**/*.rs", "tests/**/*.rs"]),
        RUST_BODY,
    )]);
    let report = drift::emit(
        &second,
        &into,
        EmitOptions {
            dry_run: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Emitted);
    assert_eq!(
        before_harness,
        tree_bytes(&harness),
        "--dry-run must not touch the harness sources"
    );
    assert_eq!(
        before_lock,
        fs::read(into.join("lock.toml")).unwrap(),
        "--dry-run must not touch the lock"
    );

    // A real emit afterwards does land the edit.
    drift::emit(&second, &into, EmitOptions::default()).unwrap();
    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    assert!(
        fs::read_to_string(&rule_path)
            .unwrap()
            .contains("tests/**/*.rs"),
        "the real emit must write what the dry run only reported"
    );
}

#[test]
fn the_lock_baselines_source_hash_and_emit_hash_equal_for_a_payload_compiled_member() {
    let (_harness, into) = workspace("hash-baseline");
    let payload = basic_payload(vec![rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY)]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let doc = fs::read_to_string(into.join("lock.toml"))
        .unwrap()
        .parse::<toml_edit::DocumentMut>()
        .unwrap();
    let row = doc["rule"]
        .as_array_of_tables()
        .unwrap()
        .iter()
        .next()
        .unwrap();
    let source_hash = row.get("source_hash").and_then(|v| v.as_str()).unwrap();
    let emit_hash = row.get("emit_hash").and_then(|v| v.as_str()).unwrap();
    assert_eq!(source_hash.len(), 64);
    assert_eq!(source_hash, emit_hash);
}

#[test]
fn a_member_naming_an_undeclared_kind_is_a_clear_refusal() {
    let (_harness, into) = workspace("unknown-kind");
    let mut payload = basic_payload(vec![rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY)]);
    payload.members.push(PayloadMember {
        kind: "ghost".to_string(),
        name: "phantom".to_string(),
        fields: Vec::new(),
        body: "boo".to_string(),
        source_path: None,
    });

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    assert!(format!("{err}").contains("ghost"), "{err}");
}

#[test]
fn an_unsupported_seam_version_is_a_clear_refusal() {
    let (_harness, into) = workspace("bad-version");
    let mut payload = basic_payload(vec![]);
    payload.version = 999;

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    assert!(format!("{err}").contains("999"), "{err}");
}

// ---------------------------------------------------------------------------
// The seam — `drift::emit_program` over a real `node` subprocess running the
// built SDK against a fixture `harness.ts` (`specs/model/pipeline.md`,
// "The SDK": "running the authored program produces plain
// data").
// ---------------------------------------------------------------------------

/// The repo's `sdk/` directory — the SDK package this crate's worktree carries
/// beside `Cargo.toml`.
fn sdk_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sdk")
}

/// Build the SDK's `dist/` once per test binary run — the compiled package a
/// fixture harness program's bare `@dtmd/temper` import resolves to, exactly as
/// an installed npm dependency would.
fn ensure_sdk_built() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let status = std::process::Command::new("npm")
            .args(["run", "build"])
            .current_dir(sdk_root())
            .status()
            .expect("failed to run `npm run build` in sdk/ — is npm on PATH?");
        assert!(status.success(), "sdk build failed");
    });
}

/// A fixture SDK program: a single file with no relative imports (so it runs
/// directly under Node's native TypeScript support with no build step of its
/// own), importing only the bare `@dtmd/temper`/`@dtmd/temper/claude-code`
/// specifiers a real consumer's `node_modules` would resolve.
const HARNESS_PROGRAM: &str = r#"
import { emit, harness, text } from "@dtmd/temper";
import { rule, skill } from "@dtmd/temper/claude-code";

const program = harness({
  members: [
    rule({
      name: "rust",
      paths: ["src/**/*.rs"],
      prose: text`
        # Rust conventions

        Errors via miette/thiserror; clippy clean under -D warnings.
      `,
    }),
    skill({
      name: "coordinate",
      description: "Use when coordinating agents across axes.",
      prose: text`
        # Coordinate

        Drive the team.
      `,
    }),
  ],
});

process.stdout.write(emit(program).seam);
"#;

/// Wire a fixture harness under `<harness>/.temper/harness.ts`, with a
/// `node_modules/@dtmd/temper` resolving to the repo's own built SDK — the
/// stand-in for a real consumer's installed dependency.
fn wire_sdk_harness(label: &str) -> (PathBuf, PathBuf) {
    wire_sdk_harness_program(label, HARNESS_PROGRAM)
}

/// [`wire_sdk_harness`], parameterized over the fixture program text — the seam
/// each real-SDK test drives is the same; only the authored harness differs.
fn wire_sdk_harness_program(label: &str, program: &str) -> (PathBuf, PathBuf) {
    ensure_sdk_built();
    let harness = tmpdir(label);
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    fs::write(into.join("harness.ts"), program).unwrap();

    let node_modules_scope = into.join("node_modules").join("@dtmd");
    fs::create_dir_all(&node_modules_scope).unwrap();
    std::os::unix::fs::symlink(sdk_root(), node_modules_scope.join("temper")).unwrap();

    (harness, into)
}

/// A fixture SDK program declaring a `require`d requirement carrying a `count`
/// set-scope clause (`specs/model/contract.md`, "Decision: set-scope
/// demands are clauses") — proving the real SDK emits a requirement's demand as
/// a nested clause row, not a facet field, end to end across the seam.
const REQUIREMENT_CLAUSES_PROGRAM: &str = r#"
import { clause, count, emit, harness, requirement, text } from "@dtmd/temper";
import { skill } from "@dtmd/temper/claude-code";

const program = harness({
  members: [
    skill({
      name: "coordinate",
      description: "Use when coordinating agents across axes.",
      satisfies: ["agents"],
      prose: text`
        # Coordinate

        Drive the team.
      `,
    }),
  ],
  require: {
    agents: requirement({
      means: "the harness fields a bounded agent roster",
      kind: skill,
      clauses: [clause(count({ min: 1, max: 2 }), { severity: "required" })],
    }),
  },
});

process.stdout.write(emit(program).seam);
"#;

#[test]
fn emit_program_emits_a_requirements_clauses_end_to_end() {
    let (_harness, into) =
        wire_sdk_harness_program("requirement-clauses", REQUIREMENT_CLAUSES_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let declarations = drift::read_declarations(&into).unwrap();
    let agents = declarations
        .requirements
        .iter()
        .find(|r| r.name == "agents")
        .expect("the `agents` requirement is recorded");
    assert_eq!(agents.kind.as_deref(), Some("skill"));

    let count_clause = agents
        .clauses
        .iter()
        .find(|c| c.predicate == "count")
        .expect("the requirement's `count` clause round-trips as a clause row, not a facet field");
    assert_eq!(count_clause.severity, "required");
    let bound = count_clause.count.expect("the count bound is recorded");
    assert_eq!((bound.min, bound.max), (1, 2));

    // The lock's requirement row carries no top-level facet columns for the
    // demand — only the nested clause.
    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        lock.contains("[[declaration.requirement.clauses]]"),
        "the demand rides a nested clause row: {lock}"
    );
}

#[test]
fn emit_program_executes_the_sdk_program_and_byte_reproduces_across_a_second_run() {
    let (harness, into) = wire_sdk_harness("seam");

    let first = drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&first, "rust"), EmitOutcome::Emitted);
    assert_eq!(outcome(&first, "coordinate"), EmitOutcome::Emitted);

    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    let rule_projected = fs::read_to_string(&rule_path).unwrap();
    assert!(
        rule_projected.contains("paths: [\"src/**/*.rs\"]"),
        "{rule_projected}"
    );
    assert!(
        rule_projected.contains("Errors via miette/thiserror"),
        "{rule_projected}"
    );

    let skill_path = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let skill_projected = fs::read_to_string(&skill_path).unwrap();
    assert!(
        skill_projected.contains("name: \"coordinate\""),
        "{skill_projected}"
    );
    assert!(
        skill_projected.contains("Drive the team."),
        "{skill_projected}"
    );

    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(lock.contains("[[declaration.kind]]"), "{lock}");

    // A second, independent `node` run over the identical program reproduces every
    // projection and the lock byte-for-byte — double-emit verified across real
    // process boundaries, not just within one SDK invocation.
    let harness_after_first = tree_bytes(&harness);
    let lock_after_first = fs::read(into.join("lock.toml")).unwrap();

    let second = drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&second, "rust"), EmitOutcome::Unchanged);
    assert_eq!(outcome(&second, "coordinate"), EmitOutcome::Unchanged);
    assert_eq!(
        harness_after_first,
        tree_bytes(&harness),
        "a second, independent node run reproduces the projection byte-for-byte"
    );
    assert_eq!(
        lock_after_first,
        fs::read(into.join("lock.toml")).unwrap(),
        "a second, independent node run reproduces the lock byte-for-byte"
    );
}

#[test]
fn emit_program_refuses_when_no_sdk_program_exists() {
    let (_harness, into) = workspace("no-program");
    let err = drift::emit_program(&into, EmitOptions::default()).unwrap_err();
    assert!(format!("{err}").contains("harness.ts"), "{err}");
}

/// A fixture SDK program that throws before it ever prints the JSON pipe — a
/// broken program, standing in for the cascade's exit-0 concern (`entry.notes`):
/// the seam must fail loud, never let a broken program read as a silent pass.
const BROKEN_HARNESS_PROGRAM: &str = r#"
throw new Error("the SDK program is broken");
"#;

#[test]
fn emit_cli_resolves_the_default_relative_into_without_doubling_the_path() {
    // `temper emit` with no `--into` uses the CLI's own relative default
    // (`./.temper`, `DEFAULT_WORKSPACE`) — the exact shape the cascade field
    // report hit: `current_dir` moves to the entry's parent, so a still-relative
    // `node` arg re-resolves against the new cwd and doubles the path
    // (`./.temper/.temper/harness.ts`, MODULE_NOT_FOUND).
    let (harness, _into) = wire_sdk_harness("relative-into");

    let output = Command::new(BIN)
        .arg("emit")
        .current_dir(&harness)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "emit over the default relative --into must resolve <into>/harness.ts without \
         doubling the path: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !harness.join(".temper").join(".temper").exists(),
        "the relative --into must never double onto itself"
    );

    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    assert!(
        rule_path.is_file(),
        "emit should have run the program and projected the rule at {rule_path:?}"
    );
}

#[test]
fn emit_cli_fails_loud_when_the_sdk_program_is_broken() {
    let (harness, _into) = wire_sdk_harness_program("broken-program", BROKEN_HARNESS_PROGRAM);

    let output = Command::new(BIN)
        .arg("emit")
        .current_dir(&harness)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "a broken SDK program must fail loud with a non-zero exit, never a silent pass: \
         stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

// ---------------------------------------------------------------------------
// The one-shot gate — `check --harness` / session-start over a raw harness with no
// lock and no `.temper/`: no copy-tree scratch import, the discovery walk
// (`discover_kind_units`/`discover_builtin`) is the only member extractor.
// ---------------------------------------------------------------------------

#[test]
fn check_harness_and_session_start_gate_the_raw_harness_with_no_scratch_import() {
    let harness = tmpdir("no-scratch");
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(
        rules.join("rust.md"),
        "# Rust conventions\n\nPrefer a clone over a lifetime fight.\n",
    )
    .unwrap();

    let harness_output = Command::new(BIN)
        .arg("check")
        .arg("--harness")
        .arg(&harness)
        .output()
        .unwrap();
    assert!(
        harness_output.status.success(),
        "a clean harness must gate green over --harness: {}",
        String::from_utf8_lossy(&harness_output.stdout)
    );

    let session_start_output = Command::new(BIN)
        .arg("check")
        .arg(&harness)
        .arg("--reporter")
        .arg("session-start")
        .output()
        .unwrap();
    assert!(
        session_start_output.status.success(),
        "session-start is always advisory: {}",
        String::from_utf8_lossy(&session_start_output.stdout)
    );

    // Neither gate ever imports: no surface workspace or lock lands beside the harness,
    // because both read the harness's `skill`/`rule` members straight off disk through
    // the discovery walk, never a throwaway copy tree.
    assert!(
        !harness.join(".temper").exists(),
        "the one-shot gate must never write a surface workspace beside the harness"
    );
    assert!(
        !harness.join("lock.toml").exists(),
        "the one-shot gate must never write a lock beside the harness"
    );
}
