//! The `settings-local` built-in kind: `.claude/settings.local.json`, the machine's own
//! per-project settings overlay (`specs/builtins.md`, "The shipped kinds"; decisions
//! 0032/0034/0036).
//!
//! A `json-document` at the **local** commitment class — the plugin-manifest read (a whole
//! JSON object whose top-level keys are the member's fields) crossed with the dial's locus
//! (per-machine, uncommitted, read in place and gated, never emitted). Its identity is the
//! fixed singleton stem `settings.local` (the `file` unit shape — every machine's overlay is
//! the one file at this path, so no declared key names it). The generic local-locus
//! machinery is `tests/local_locus.rs`'s; these cases own the shipped kind's own face:
//! discovery under the ignore override, the documented-key clauses firing while the
//! unschematized residue stays opaque, and the active member announced.
//!
//! Every format fact here is the live settings docs' (code.claude.com/docs/en/settings,
//! retrieved 2026-07-16).

use std::fs;

mod common;

use common::{check_harness, check_harness_in, write_sibling};

use temper::drift::{self, Declarations, EmitOptions, KindFactRow, Payload};
use temper::json_manifest::DocumentMember;
use temper::kind::{Commitment, Content, Format, Governs, Registration, UnitShape};

/// A well-formed local overlay in the real Claude Code shape: the structural container keys
/// the default contract gates (`permissions`/`env`/`hooks`, each an object) beside opaque
/// residue no clause ranges over — a `model` scalar, and an `enabledPlugins` enablement the
/// kind never models as a member.
const SETTINGS_LOCAL_JSON: &str = r#"{
  "permissions": { "allow": ["Bash(cargo build:*)"], "deny": [] },
  "env": { "CLAUDE_CODE_ENABLE_TELEMETRY": "1" },
  "hooks": { "PreToolUse": [] },
  "model": "claude-opus-4-1",
  "enabledPlugins": { "formatter@my-marketplace": true }
}
"#;

fn settings_local_kind() -> temper::kind::CustomKind {
    temper::builtin_kind::definition("settings-local")
        .unwrap()
        .expect("settings-local is embedded")
}

/// The settings-local kind's lock-shaped fact row — a local-class json-document `file` kind
/// over its real locus, the shape emit rows for it.
fn settings_local_kind_facts() -> KindFactRow {
    KindFactRow {
        commitment: Some("local".to_string()),
        format: Some("json-document".to_string()),
        unit_shape: Some("file".to_string()),
        ..common::kind_facts("settings-local", ".claude", "settings.local.json")
    }
}

#[test]
fn the_settings_local_kind_owns_its_file_as_a_local_json_document_named_by_its_stem() {
    let kind = settings_local_kind();

    assert_eq!(
        kind.governs,
        Some(Governs {
            root: ".claude".to_string(),
            glob: "settings.local.json".to_string(),
        })
    );
    // The format routes the artifact to the document reader, exactly as plugin-manifest's does.
    assert_eq!(kind.format, Some(Format::JsonDocument));
    // A singleton at a fixed path: identity is the file stem, so no declared key names it —
    // distinct from plugin-manifest's `named-field(name)`.
    assert_eq!(kind.unit_shape, Some(UnitShape::File));
    // The local commitment class: read in place at check, never an emit input or target.
    assert_eq!(kind.commitment, Some(Commitment::Local));
    // It owns its file rather than surfacing inside a manifest, and reaches the model on no
    // channel of its own — machine configuration read by the harness.
    assert_eq!(kind.collection_address, None);
    assert_eq!(kind.content, Content::File);
    assert_eq!(kind.registration, Vec::<Registration>::new());
}

#[test]
fn a_settings_local_document_surfaces_one_member_named_by_its_fixed_stem() {
    let harness = common::tmpdir("settings-local-read");
    write_sibling(&harness, ".claude/settings.local.json", SETTINGS_LOCAL_JSON);

    let member = DocumentMember::read(
        &settings_local_kind(),
        &harness.join(".claude/settings.local.json"),
    )
    .unwrap();

    // Identity is the fixed stem, never a declared key — every machine's overlay is the one
    // file at this path.
    assert_eq!(member.id, "settings.local");
    // The document's top-level keys are the member's fields, so a clause ranges over the
    // overlay exactly as it ranges over a plugin manifest. The `enabledPlugins` enablement
    // rides here as one opaque field, never exploded into modeled members.
    let fields: Vec<&str> = member.fields.keys().map(String::as_str).collect();
    assert_eq!(
        fields,
        vec!["enabledPlugins", "env", "hooks", "model", "permissions"]
    );
}

#[test]
fn a_well_formed_overlay_gates_clean_and_its_residue_passes_opaque() {
    let harness = common::tmpdir("settings-local-clean");
    write_sibling(&harness, ".claude/settings.local.json", SETTINGS_LOCAL_JSON);

    let (findings, ok) = check_harness(&harness);
    assert!(ok, "a documented overlay gates clean: {findings:?}");
    // The `model` scalar and the `enabledPlugins` enablement are unschematized residue: no
    // clause ranges over them, so they draw no finding — the partial-governance posture.
    assert!(
        findings.iter().all(|f| !f.starts_with("::error")),
        "{findings:?}"
    );
    // And the member is really being checked, not silently skipped past.
    assert!(
        findings.iter().any(|f| f.contains("settings-local (1)")),
        "{findings:?}"
    );
}

#[test]
fn a_locally_registered_hook_stays_opaque_residue_never_a_modeled_member() {
    let harness = common::tmpdir("settings-local-opaque-hook");
    // A hook registered here under an event no `hook` kind would accept: the `hooks` map
    // passes the `type` clause, and — crucially — no `hook` member is derived off this
    // uncommitted overlay (that kind reads the committed `settings.json`), so the bad event
    // draws no `hook.enum.event` finding. Fields, not members.
    write_sibling(
        &harness,
        ".claude/settings.local.json",
        "{\n  \"hooks\": { \"NotARealEvent\": [] }\n}\n",
    );

    let (findings, ok) = check_harness(&harness);
    assert!(ok, "the overlay's hooks object gates clean: {findings:?}");
    assert!(
        common::findings_for(&findings, "hook.enum.event").is_empty(),
        "a hook inside the local overlay is opaque residue, never a modeled member: {findings:?}"
    );
}

#[test]
fn a_wrong_typed_structural_key_fires_its_documented_clause() {
    let harness = common::tmpdir("settings-local-bad-permissions");
    // `permissions` is documented as an object; a string carries no rules Claude Code can
    // read, so the documented-key clause fires.
    write_sibling(
        &harness,
        ".claude/settings.local.json",
        "{\n  \"permissions\": \"everything\"\n}\n",
    );

    let (findings, ok) = check_harness(&harness);
    assert!(!ok, "a non-object permissions fails the gate: {findings:?}");
    let typed = common::findings_for(&findings, "settings-local.type.permissions");
    assert_eq!(typed.len(), 1, "{findings:?}");
    assert!(typed[0].starts_with("::error"), "{findings:?}");
}

#[test]
fn the_gitignored_overlay_is_discovered_read_and_announced() {
    let harness = common::tmpdir("settings-local-gitignored");
    // A real local overlay is always an ignored one — Claude Code gitignores the file it
    // creates — so a walk pruning on the repo's ignore rules would never find it. The local
    // class's discovery override is what reaches it.
    write_sibling(&harness, ".gitignore", ".claude/settings.local.json\n");
    write_sibling(&harness, ".claude/settings.local.json", SETTINGS_LOCAL_JSON);

    let run = check_harness_in(&harness, Some("github"));
    assert!(
        run.ok,
        "the ignored overlay is discovered, read, and gated clean: {}",
        run.output
    );
    // The active local member is announced by its `<kind>:<id>` address — what its findings
    // would name it by, and what tells a reader the uncommitted overlay judged this run.
    assert!(
        run.announcements()
            .iter()
            .any(|line| line.contains("local member: settings-local:settings.local")),
        "the active overlay is announced: {:?}",
        run.announcements()
    );
}

#[test]
fn emit_writes_nothing_at_the_overlays_path_and_rows_none_of_it() {
    let harness = common::tmpdir("settings-local-emit");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    write_sibling(&harness, ".claude/settings.local.json", SETTINGS_LOCAL_JSON);
    let doc_path = harness.join(".claude/settings.local.json");

    // A memberless payload declaring the kind: emit owns the committed tree, and a local
    // kind's overlay is never in it — the document is the author's, byte-untouched, and no
    // entry projects or reaps it.
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![settings_local_kind_facts()],
            ..Default::default()
        },
        members: Vec::new(),
    };
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    assert_eq!(
        fs::read_to_string(&doc_path).unwrap(),
        SETTINGS_LOCAL_JSON,
        "the local overlay survives emit byte-identical: {:?}",
        report.entries
    );
    assert!(
        report
            .entries
            .iter()
            .all(|entry| entry.name != "settings.local"),
        "a local overlay is neither projected nor reaped: {:?}",
        report.entries
    );

    // The kind's own row is committed; no row of its member ever enters the lock.
    let declarations = drift::read_declarations(&into).unwrap();
    assert_eq!(
        declarations.kinds[0].commitment.as_deref(),
        Some("local"),
        "the kind's row carries the declared commitment class"
    );
    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        !lock.contains("settings.local.json") || !lock.contains("provenance"),
        "no provenance/emit-hash row of the local overlay lands in the lock:\n{lock}"
    );
}
