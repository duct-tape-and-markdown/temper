//! `temper emit` — the seam's compile (`specs/architecture/20-surface.md`,
//! "The seam — one implementation"; "Emit — total, byte-reproducible, refusing").
//!
//! Two tiers:
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

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::drift::{
    self, Declarations, EmitOptions, EmitOutcome, KindFactRow, Payload, PayloadMember,
};

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
        activation: None,
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
        activation: None,
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
/// (`specs/architecture/20-surface.md`): `.temper/` sits beside `.claude/`.
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
        kind: "rule".to_string(),
        predicate: "required".to_string(),
        field: Some("paths".to_string()),
        severity: "required".to_string(),
    });
    payload
        .declarations
        .requirements
        .push(drift::RequirementRow {
            name: "dev-standards".to_string(),
            kind: Some("rule".to_string()),
            package: None,
            required: true,
            count: None,
            unique: Vec::new(),
            membership: None,
            degree: None,
            verified_by: None,
        });
    payload.declarations.assembly.push(drift::AssemblyFactRow {
        fact: "authority".to_string(),
        value: Some("shared".to_string()),
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
// built SDK against a fixture `harness.ts` (`specs/architecture/20-surface.md`,
// "The seam — one implementation": "running the authored program produces plain
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
    ensure_sdk_built();
    let harness = tmpdir(label);
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    fs::write(into.join("harness.ts"), HARNESS_PROGRAM).unwrap();

    let node_modules_scope = into.join("node_modules").join("@dtmd");
    fs::create_dir_all(&node_modules_scope).unwrap();
    std::os::unix::fs::symlink(sdk_root(), node_modules_scope.join("temper")).unwrap();

    (harness, into)
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
