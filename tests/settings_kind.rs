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
//! What 0050 buys is one fact stated four ways below: the file is a whole projection,
//! so it carries a rollup row, so a hand edit to *any* part of it — segment or residue
//! — is drift under the root `fresh` clause, and the unmodeled-surface advisory retires
//! because a kind now governs every key. The contrast case holds the same bytes under
//! the pre-0050 posture (the residue riding the harness-level `settings` option, no
//! container kind) and shows both halves absent: no rollup row, and the file left
//! undeclared at the
//! kind's own governed locus — the built-in ships for every harness now, so an ownerless
//! settings.json is an undeclared member rather than an unmodeled surface.
//!
//! Every format fact here is the live settings docs' (code.claude.com/docs/en/settings
//! and code.claude.com/docs/en/settings-reference, retrieved 2026-09-22).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use sha2::{Digest, Sha256};
use temper::drift::{self, EmitOptions, EmitOutcome};
use temper::json_manifest;

mod common;

/// The 0050 posture: a `settings` container member composing the committed file's
/// documented keys, beside a `hook` registering at that same file's `hooks` collection
/// address. Every key of `.claude/settings.json` is authored in the program.
const SETTINGS_MEMBER_PROGRAM: &str = r#"
import { emit, harness } from "@dtmd/temper";
import { hook, settings } from "@dtmd/temper/claude-code";

const sessionStart = hook({
  name: "SessionStart",
  type: "command",
  command: "temper reporter",
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
  type: "command",
  command: "temper reporter",
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
    assert_eq!(facts.governs_glob.as_deref(), Some("settings.json"));
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
fn the_unmodeled_surface_advisory_retires_once_the_settings_kind_governs_the_file_whole() {
    let (harness, into) = common::wire_sdk_harness("settings-governed", SETTINGS_MEMBER_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let (findings, ok) = common::check_harness(&harness);
    assert!(ok, "a governed settings file gates clean: {findings:?}");
    assert!(
        common::findings_for(&findings, "coverage.unmodeled-surface").is_empty(),
        "every key of the file is governed — the segment by its own kind, the residue by \
         the container — so there is no gap left to name: {findings:?}"
    );
    // Silence here is truthful only because the container really is checked: the note
    // says so by name.
    assert!(
        findings.iter().any(|f| f.contains("settings (1)")),
        "the container member is announced as checked, never silently skipped: {findings:?}"
    );
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

    // The built-in kind ships for every harness, so the file is no longer an unmodeled
    // surface — it is a document at a governed locus the program declares no member for,
    // which is the finding an ownerless settings.json draws now.
    let (findings, _ok) = common::check_harness(&harness);
    assert!(
        common::findings_for(&findings, "coverage.unmodeled-surface").is_empty(),
        "a governing kind retires the unmodeled-surface advisory: {findings:?}"
    );
    let undeclared = common::findings_for(&findings, "locus.undeclared-member");
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
