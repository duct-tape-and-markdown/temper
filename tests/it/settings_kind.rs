//! The `settings` built-in kind: the committed `.claude/settings.json` as a whole
//! (`specs/builtins.md`, "The shipped kinds"; decision 0050).
//!
//! A `json-document` at the committed commitment class, singleton identity from its
//! documented path — and, unlike every other file kind, the **container** of three
//! registration collection addresses. The engine's container machinery is already
//! generic (`src/drift.rs`'s manifest build, `src/coverage_note.rs`'s whole-file
//! governance); these cases pin it to the shipped kind, driven over the real SDK the
//! way `tests/emit.rs` drives the seam — `node` running the built
//! `@dtmd/temper/claude-code` against a fixture program, since the kind ships there.
//!
//! What 0050 buys is one fact stated several ways below: the file is a whole projection,
//! so it carries a rollup row, so a hand edit to *any* part of it — segment or residue
//! — is drift under the root `fresh` clause. The contrast case holds the same bytes under
//! the pre-0050 posture (the residue riding the harness-level `settings` option, no
//! container kind) and shows both halves absent: no rollup row, and the file left
//! undeclared at the kind's own governed locus.
//!
//! Every format fact here is the live settings docs' (code.claude.com/docs/en/settings
//! and code.claude.com/docs/en/settings-reference, retrieved 2026-09-22).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use sha2::{Digest, Sha256};
use temper::drift::{self, EmitOptions, EmitOutcome};
use temper::json_manifest;

use crate::common;

/// The 0050 posture: a `settings` container member composing the committed file's
/// documented keys, beside a `hook` registering at that same file's `hooks` collection
/// address. Every key of `.claude/settings.json` is authored in the program.
const SETTINGS_MEMBER_PROGRAM: &str = r#"
import { emit, harness } from "@dtmd/temper";
import { hook, settings } from "@dtmd/temper/claude-code";

const sessionStart = hook({
  name: "SessionStart",
  hooks: [{ type: "command", command: "temper reporter" }],
});

const projectSettings = settings({
  name: "settings",
  permissions: { allow: ["Bash(cargo build:*)"] },
  env: { CLAUDE_CODE_ENABLE_TELEMETRY: "1" },
  autoMemoryEnabled: false,
});

process.stdout.write(emit(harness({ members: [sessionStart, projectSettings] })).seam);
"#;

/// The pre-0050 posture, for contrast: the same hook and the same residue keys, but the
/// residue rides the harness-level `settings` option and no kind governs the file whole.
const SETTINGS_RESIDUE_PROGRAM: &str = r#"
import { emit, harness } from "@dtmd/temper";
import { hook } from "@dtmd/temper/claude-code";

const sessionStart = hook({
  name: "SessionStart",
  hooks: [{ type: "command", command: "temper reporter" }],
});

process.stdout.write(
  emit(
    harness({
      members: [sessionStart],
      settings: {
        permissions: { allow: ["Bash(cargo build:*)"] },
        env: { CLAUDE_CODE_ENABLE_TELEMETRY: "1" },
        autoMemoryEnabled: false,
      },
    }),
  ).seam,
);
"#;

/// The manifest the two programs above each describe: the `hooks` collection segment
/// carrying the one registration, then the container's three opaque residue keys —
/// rendered through the canonical write face, so this is the byte oracle rather than a
/// hand-spelled JSON string.
fn expected_manifest() -> String {
    let mut entries = BTreeMap::new();
    entries.insert(
        "SessionStart".to_string(),
        serde_json::json!([ { "hooks": [ { "type": "command", "command": "temper reporter" } ] } ]),
    );
    let segment = json_manifest::CollectionSegment {
        collection_key: "hooks".to_string(),
        entries,
    };
    let residue = BTreeMap::from([
        (
            "permissions".to_string(),
            serde_json::json!({ "allow": ["Bash(cargo build:*)"] }),
        ),
        (
            "env".to_string(),
            serde_json::json!({ "CLAUDE_CODE_ENABLE_TELEMETRY": "1" }),
        ),
        ("autoMemoryEnabled".to_string(), serde_json::json!(false)),
    ]);
    json_manifest::write_manifest(&[segment], &residue)
}

/// The committed settings file under `harness`.
fn settings_path(harness: &Path) -> std::path::PathBuf {
    harness.join(".claude").join("settings.json")
}

/// The lock's rollup rows for `kind` — the `[[<kind>]]` array of tables, as
/// `(name, source_path, emit_hash)` triples. Empty when the family is absent, which is
/// the contrast case's whole point.
fn rollup_rows(into: &Path, kind: &str) -> Vec<(String, String, String)> {
    let doc = fs::read_to_string(into.join("lock.toml"))
        .unwrap()
        .parse::<toml_edit::DocumentMut>()
        .unwrap();
    let Some(rows) = doc.get(kind).and_then(|item| item.as_array_of_tables()) else {
        return Vec::new();
    };
    rows.iter()
        .map(|row| {
            let column = |key: &str| {
                row.get(key)
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string()
            };
            (column("name"), column("source_path"), column("emit_hash"))
        })
        .collect()
}

#[test]
fn the_settings_kind_crosses_the_seam_as_a_committed_whole_file_json_document() {
    let (_harness, into) = common::wire_sdk_harness("settings-facts", SETTINGS_MEMBER_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let declarations = drift::read_declarations(&into).unwrap();
    let facts = declarations
        .kinds
        .iter()
        .find(|row| row.name == "settings")
        .expect("the `settings` kind the program imported crosses the seam as a kind-fact row");

    // The documented singleton locus: a literal glob, so the member's name never splices
    // into the path — every project's committed settings are the one file here.
    assert_eq!(facts.governs_root.as_deref(), Some(".claude"));
    assert_eq!(facts.sole_glob(), Some("settings.json"));
    // Committed, not local: absent is the class, and it is what makes the file an emit
    // target where `settings-local`'s overlay is read in place and never written.
    assert_eq!(facts.commitment, None);
    // The whole-file JSON format routes it to the document reader, like settings-local.
    assert_eq!(facts.format.as_deref(), Some("json-document"));
    assert_eq!(facts.unit_shape.as_deref(), Some("file"));
    // It is the container of the three collection addresses, never a member at one — and
    // it reaches the model on no channel of its own.
    assert_eq!(facts.collection_address, None);
    assert_eq!(facts.shape, None);
    assert!(facts.registration.is_empty());
}

#[test]
fn emit_renders_the_settings_file_whole_and_rolls_up_its_byte_fingerprint() {
    let (harness, into) = common::wire_sdk_harness("settings-emit", SETTINGS_MEMBER_PROGRAM);

    let report = drift::emit_program(&into, EmitOptions::default()).unwrap();
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.kind == "settings")
        .expect("the container member is one emit entry, labelled by its own kind");
    assert_eq!(entry.outcome, EmitOutcome::Emitted);

    // The bytes are the canonical write face's: the declared collection segment first,
    // then the container's fields as opaque residue — one file, two authorship routes.
    let path = settings_path(&harness);
    let expected = expected_manifest();
    assert_eq!(fs::read_to_string(&path).unwrap(), expected);

    // The container's rollup row carries the file's own byte fingerprint. This is the row
    // 0050 buys: without it nothing in the lock describes these bytes.
    let rows = rollup_rows(&into, "settings");
    assert_eq!(
        rows.len(),
        1,
        "one container member, one rollup row: {rows:?}"
    );
    let (name, source_path, emit_hash) = &rows[0];
    assert_eq!(name, "settings");
    assert_eq!(source_path, ".claude/settings.json");
    let mut hasher = Sha256::new();
    hasher.update(expected.as_bytes());
    assert_eq!(
        emit_hash,
        &format!("{:x}", hasher.finalize()),
        "the rollup fingerprint is the SHA-256 of the whole file, segments and residue alike"
    );

    // A second, independent compile reproduces every byte and reports the idempotent no-op.
    let second = drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert_eq!(
        second
            .entries
            .iter()
            .find(|entry| entry.kind == "settings")
            .map(|entry| entry.outcome),
        Some(EmitOutcome::Unchanged)
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), expected);
}

#[test]
fn a_hand_edit_to_either_the_segment_or_the_residue_reports_config_stale() {
    // Two edits, one per authorship route, each against a freshly compiled harness: the
    // fingerprint is over the whole file, so neither half can be touched invisibly.
    for (label, edit) in [
        (
            "residue",
            (
                "\"autoMemoryEnabled\": false",
                "\"autoMemoryEnabled\": true",
            ),
        ),
        ("segment", ("temper reporter", "temper reporter --quiet")),
    ] {
        let (harness, into) =
            common::wire_sdk_harness(&format!("settings-stale-{label}"), SETTINGS_MEMBER_PROGRAM);
        drift::emit_program(&into, EmitOptions::default()).unwrap();

        let path = settings_path(&harness);
        let before = fs::read_to_string(&path).unwrap();
        let (needle, replacement) = edit;
        let after = before.replace(needle, replacement);
        assert_ne!(
            before, after,
            "the {label} edit must actually change the file"
        );
        fs::write(&path, &after).unwrap();

        let (findings, _ok) = common::check_harness(&harness);
        let stale = common::findings_for(&findings, "root.fresh");
        assert_eq!(
            stale.len(),
            1,
            "a hand edit to the {label} half reports exactly one stale projection: {findings:?}"
        );
        assert!(
            stale[0].contains(".claude/settings.json"),
            "the finding names the file that moved: {}",
            stale[0]
        );
    }
}

#[test]
fn the_same_bytes_under_the_pre_0050_posture_carry_no_rollup_row_and_go_undeclared() {
    let (harness, into) = common::wire_sdk_harness("settings-ungoverned", SETTINGS_RESIDUE_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    // Identical bytes on disk — the container is a governance fact, not a format change.
    assert_eq!(
        fs::read_to_string(settings_path(&harness)).unwrap(),
        expected_manifest()
    );

    // But nothing fingerprints them: the residue rode a harness-level option, so no member
    // owns the file and the lock carries no row to compare a hand edit against.
    assert!(
        rollup_rows(&into, "settings").is_empty(),
        "an ownerless manifest gets no container rollup row"
    );

    // The built-in kind ships for every harness, so the file is a document at a governed
    // locus the program declares no member for — the finding an ownerless settings.json
    // draws now.
    let (findings, _ok) = common::check_harness(&harness);
    let undeclared = common::findings_for(&findings, "root.locus-declared");
    assert_eq!(undeclared.len(), 1, "{findings:?}");
    assert!(
        undeclared[0].contains(".claude/settings.json"),
        "the finding names the undeclared document at the settings locus: {}",
        undeclared[0]
    );
    // And the edit the missing row cannot catch really is invisible.
    let path = settings_path(&harness);
    let edited = fs::read_to_string(&path).unwrap().replace(
        "\"autoMemoryEnabled\": false",
        "\"autoMemoryEnabled\": true",
    );
    fs::write(&path, edited).unwrap();
    let (findings, _ok) = common::check_harness(&harness);
    assert!(
        common::findings_for(&findings, "root.fresh").is_empty(),
        "with no rollup row the hand edit draws no freshness finding — the gap 0050 closes: \
         {findings:?}"
    );
}

/// A hook-only represented harness — temper's own install over a lone rule, whose program
/// composes the gate hooks and no `settings` member — with a `permissions` key then
/// hand-added to the settings file it rendered. Returns the root and its `.temper/`.
fn hook_only_harness_with_a_hand_added_key(
    label: &str,
) -> (std::path::PathBuf, std::path::PathBuf) {
    use temper::install::{self, Represent};

    let root = common::tmpdir(label);
    common::write_sibling(&root, ".claude/rules/rust.md", "Run `cargo fmt`.\n");
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert!(
        !temper_dir.join("settings").exists(),
        "the program composes no settings member"
    );

    let path = settings_path(&root);
    let mut document: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    document["permissions"] = serde_json::json!({ "allow": ["Bash(cargo build:*)"] });
    fs::write(
        &path,
        serde_json::to_string_pretty(&document).unwrap() + "\n",
    )
    .unwrap();
    (root, temper_dir)
}

#[test]
fn emit_refuses_a_hand_added_key_in_a_settings_file_no_settings_member_declares() {
    let (root, temper_dir) = hook_only_harness_with_a_hand_added_key("settings-ownerless-key");
    let before = fs::read_to_string(settings_path(&root)).unwrap();

    let err = drift::emit_program(&temper_dir, EmitOptions::default()).unwrap_err();
    let message = format!("{err}");
    assert!(message.contains("`permissions`"), "{message}");
    assert!(message.contains(".claude/settings.json"), "{message}");
    assert_eq!(
        fs::read_to_string(settings_path(&root)).unwrap(),
        before,
        "the refusal lands before a byte is written — the hand-added key survives"
    );

    // `--teardown` spells a collection drop, never the deletion of keys no emit wrote.
    let err = drift::emit_program(
        &temper_dir,
        EmitOptions {
            teardown: true,
            ..EmitOptions::default()
        },
    )
    .unwrap_err();
    assert!(format!("{err}").contains("`permissions`"), "{err}");
    assert_eq!(fs::read_to_string(settings_path(&root)).unwrap(), before);
}

#[test]
fn install_hoists_a_hand_added_settings_key_into_a_settings_member_on_a_represented_harness() {
    use temper::install::{self, Represent};

    let (root, temper_dir) = hook_only_harness_with_a_hand_added_key("settings-hoist-key");
    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(outcome.scaffolded, 1, "the one hoisted settings member");

    // The key now lives in the program: a `settings` member module composed into the entry.
    let module = fs::read_to_string(temper_dir.join("settings").join("settings.ts")).unwrap();
    assert!(
        module.contains("permissions: {\"allow\":[\"Bash(cargo build:*)\"]},"),
        "got:\n{module}"
    );
    assert!(
        !module.contains("hooks"),
        "a claimed collection stays its registrations': got:\n{module}"
    );
    let entry = fs::read_to_string(temper_dir.join("harness.ts")).unwrap();
    assert!(
        entry.contains("import { settings_settings } from \"./settings/settings.ts\";"),
        "got:\n{entry}"
    );
    assert!(
        entry.contains("members: [settings_settings, "),
        "got:\n{entry}"
    );

    // The file survived the re-render with the key, and is now a fingerprinted projection.
    let projected: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(settings_path(&root)).unwrap()).unwrap();
    assert_eq!(projected["permissions"]["allow"][0], "Bash(cargo build:*)");
    assert!(projected["hooks"]["PreToolUse"].is_array(), "{projected}");
    assert_eq!(rollup_rows(&temper_dir, "settings").len(), 1);

    // Converged: the next emit owns the key and moves no byte.
    let before = common::tree_bytes(&root);
    drift::emit_program(&temper_dir, EmitOptions::default()).unwrap();
    assert_eq!(before, common::tree_bytes(&root));
}
