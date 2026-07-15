//! `temper emit` — the seam's compile.
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
//!   extractor, straight off harness disk.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use sha2::{Digest, Sha256};
use temper::drift::{
    self, CollectionAddressRow, Declarations, EmitOptions, EmitOutcome, KindFactRow, Payload,
    PayloadMember, RegistrationRow, SettingsRow,
};
use temper::json_manifest;

mod common;

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

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

/// A rule + skill payload, declarations carrying just their kind facts.
fn basic_payload(members: Vec<PayloadMember>) -> Payload {
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                common::rule_kind_facts(None, &[]),
                common::skill_kind_facts(None, &[]),
            ],
            ..Default::default()
        },
        members,
    }
}

/// A fresh `<harness>/.temper` pair — `drift::emit` derives the projection root
/// from the workspace dir's parent, matching the seam's own topology:
/// `.temper/` sits beside `.claude/`.
fn workspace(label: &str) -> (PathBuf, PathBuf) {
    let harness = common::tmpdir(label);
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
        common::rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY),
        common::skill_member(
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
    let mut payload = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
    payload.declarations.clauses.push(drift::ClauseRow {
        kind: Some("rule".to_string()),
        field: Some("paths".to_string()),
        ..common::clause("required", "required")
    });
    payload.declarations.requirements.push(common::requirement(
        "dev-standards",
        true,
        Some("rule"),
    ));
    payload.declarations.assembly.push(drift::AssemblyFactRow {
        fact: "authority".to_string(),
        value: Some("warn".to_string()),
        from: None,
        field: None,
        to: None,
    });
    payload.declarations.satisfies.push(drift::SatisfiesRow {
        member: "rule:rust".to_string(),
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
    let payload = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let after_first = common::tree_bytes(&harness);
    let lock_after_first = fs::read(into.join("lock.toml")).unwrap();

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Unchanged);
    assert_eq!(
        after_first,
        common::tree_bytes(&harness),
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
    let first = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
    drift::emit(&first, &into, EmitOptions::default()).unwrap();

    let second = basic_payload(vec![common::rule_member(
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
        common::rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY),
        common::skill_member(
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
    let first = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
    drift::emit(&first, &into, EmitOptions::default()).unwrap();

    let before_harness = common::tree_bytes(&harness);
    let before_lock = fs::read(into.join("lock.toml")).unwrap();

    let second = basic_payload(vec![common::rule_member(
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
        common::tree_bytes(&harness),
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
    let payload = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
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
fn a_crlf_or_lone_cr_body_emits_an_lf_only_projection() {
    let (harness, into) = workspace("lf-normalize");
    let crlf_body = "# Windows-authored\r\n\r\nCarries CRLF line endings.\r\n";
    let lone_cr_body = "# Old-Mac-authored\rCarries lone CR line endings.\r";
    let payload = basic_payload(vec![
        common::rule_member("crlf", Some(&["src/**/*.rs"]), crlf_body),
        common::rule_member("lonecr", Some(&["src/**/*.rs"]), lone_cr_body),
    ]);

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let crlf_path = harness.join(".claude").join("rules").join("crlf.md");
    let lonecr_path = harness.join(".claude").join("rules").join("lonecr.md");
    let crlf_bytes = fs::read(&crlf_path).unwrap();
    let lonecr_bytes = fs::read(&lonecr_path).unwrap();
    assert!(
        !crlf_bytes.contains(&b'\r'),
        "a CRLF source must emit LF-only bytes"
    );
    assert!(
        !lonecr_bytes.contains(&b'\r'),
        "a lone-CR source must emit LF-only bytes"
    );
    assert_eq!(
        String::from_utf8(crlf_bytes.clone()).unwrap(),
        "---\npaths: [\"src/**/*.rs\"]\n---\n# Windows-authored\n\nCarries CRLF line endings.\n"
    );
    assert_eq!(
        String::from_utf8(lonecr_bytes.clone()).unwrap(),
        "---\npaths: [\"src/**/*.rs\"]\n---\n# Old-Mac-authored\nCarries lone CR line endings.\n"
    );

    // The lock's emit_hash is computed over the same normalized bytes written to disk.
    let doc = fs::read_to_string(into.join("lock.toml"))
        .unwrap()
        .parse::<toml_edit::DocumentMut>()
        .unwrap();
    let rows = doc["rule"].as_array_of_tables().unwrap();
    for (name, bytes) in [("crlf", &crlf_bytes), ("lonecr", &lonecr_bytes)] {
        let row = rows
            .iter()
            .find(|row| row.get("name").and_then(|v| v.as_str()) == Some(name))
            .unwrap();
        let emit_hash = row.get("emit_hash").and_then(|v| v.as_str()).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        assert_eq!(emit_hash, format!("{:x}", hasher.finalize()));
    }

    // Idempotent double-emit: re-emitting the same (CRLF-carrying) payload
    // byte-reproduces and reports Unchanged, never a nondeterminism refusal.
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "crlf"), EmitOutcome::Unchanged);
    assert_eq!(outcome(&report, "lonecr"), EmitOutcome::Unchanged);
    assert_eq!(fs::read(&crlf_path).unwrap(), crlf_bytes);
    assert_eq!(fs::read(&lonecr_path).unwrap(), lonecr_bytes);
}

#[test]
fn a_member_naming_an_undeclared_kind_is_a_clear_refusal() {
    let (_harness, into) = workspace("unknown-kind");
    let mut payload = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
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
// Represented manifests — a container member's residue plus its registration
// members are regenerated whole through the canonical write face, never
// json_splice's in-place edit.
// ---------------------------------------------------------------------------

/// A `hook` registration kind fact: fields-only, keyed at `settings.json`'s `hooks.<Event>`.
fn hook_kind_facts() -> KindFactRow {
    KindFactRow {
        shape: Some("fields".to_string()),
        collection_address: Some(CollectionAddressRow {
            manifest: "settings.json".to_string(),
            key_path: "hooks.<Event>".to_string(),
        }),
        ..common::kind_facts("hook", ".claude", "settings.json")
    }
}

#[test]
fn a_represented_manifest_emits_whole_through_the_write_face_not_json_splice() {
    let (harness, into) = workspace("manifest-write-face");

    // A `settings.json` container member carrying the opaque residue (permissions), plus a
    // hook registration member keyed at `hooks.SessionStart` — the represented manifest
    // emit must regenerate the whole file.
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                common::rule_kind_facts(None, &[]),
                hook_kind_facts(),
                // The container kind owns the same `.claude/settings.json` file locus, so
                // its member projects to the manifest path the hook surfaces inside.
                common::kind_facts("settings", ".claude", "settings.json"),
            ],
            registrations: vec![RegistrationRow {
                kind: "hook".to_string(),
                key: "SessionStart".to_string(),
                manifest: "settings.json".to_string(),
                key_path: "hooks.<Event>".to_string(),
                fields: vec![
                    ("type".to_string(), serde_json::json!("command")),
                    ("command".to_string(), serde_json::json!("temper reporter")),
                ],
            }],
            ..Default::default()
        },
        members: vec![PayloadMember {
            kind: "settings".to_string(),
            name: "settings".to_string(),
            fields: vec![(
                "permissions".to_string(),
                serde_json::json!({ "allow": ["Bash(cargo build:*)"] }),
            )],
            body: String::new(),
            source_path: None,
        }],
    };

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "settings"), EmitOutcome::Emitted);

    // The emitted bytes are exactly what the canonical write face produces — the `hooks`
    // segment then the opaque residue — never json_splice's in-place edit (which preserves
    // a prior document's own formatting rather than regenerating canonically). A hook nests
    // into Claude Code's array-of-matcher-groups shape (`hooks.<Event> = [{hooks:[{…}]}]`);
    // `SessionStart` carries no `matcher`, so its one group is the lone handler list.
    let mut entries = std::collections::BTreeMap::new();
    entries.insert(
        "SessionStart".to_string(),
        serde_json::json!([ { "hooks": [ { "type": "command", "command": "temper reporter" } ] } ]),
    );
    let segment = json_manifest::CollectionSegment {
        collection_key: "hooks".to_string(),
        entries,
    };
    let mut residue = std::collections::BTreeMap::new();
    residue.insert(
        "permissions".to_string(),
        serde_json::json!({ "allow": ["Bash(cargo build:*)"] }),
    );
    let expected = json_manifest::write_manifest(&[segment], &residue);

    let manifest_path = harness.join(".claude").join("settings.json");
    assert_eq!(fs::read_to_string(&manifest_path).unwrap(), expected);

    // Double emit reproduces the manifest byte-for-byte and reports the idempotent no-op.
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "settings"), EmitOutcome::Unchanged);
    assert_eq!(fs::read_to_string(&manifest_path).unwrap(), expected);

    // The registration member crosses the seam into the lock's declaration rows.
    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        lock.contains("[[declaration.registration]]"),
        "the registration member is recorded as a declaration row: {lock}"
    );
}

#[test]
fn harness_settings_residue_folds_into_settings_json_beside_the_hooks_segment() {
    let (harness, into) = workspace("settings-residue");

    // No member projects to `settings.json` — the residue rides the seam's own `settings`
    // family (`harness({ settings: { autoMemoryEnabled, worktree } })`), and a hook
    // registration builds the `hooks` segment of the same manifest.
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![hook_kind_facts()],
            registrations: vec![RegistrationRow {
                kind: "hook".to_string(),
                key: "SessionStart".to_string(),
                manifest: "settings.json".to_string(),
                key_path: "hooks.<Event>".to_string(),
                fields: vec![
                    ("type".to_string(), serde_json::json!("command")),
                    ("command".to_string(), serde_json::json!("temper reporter")),
                ],
            }],
            settings: vec![
                SettingsRow {
                    manifest: "settings.json".to_string(),
                    key: "worktree".to_string(),
                    value: serde_json::json!(true),
                },
                SettingsRow {
                    manifest: "settings.json".to_string(),
                    key: "autoMemoryEnabled".to_string(),
                    value: serde_json::json!(false),
                },
            ],
            ..Default::default()
        },
        members: Vec::new(),
    };

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    // With no container member, the manifest is labelled by its filename under a `manifest`
    // kind — it is emitted, not shed.
    assert_eq!(outcome(&report, "settings.json"), EmitOutcome::Emitted);

    // The bytes are the canonical write face's: the `hooks` collection segment, then every
    // authored residue key in sorted order — nothing shed.
    let mut entries = std::collections::BTreeMap::new();
    entries.insert(
        "SessionStart".to_string(),
        serde_json::json!([ { "hooks": [ { "type": "command", "command": "temper reporter" } ] } ]),
    );
    let segment = json_manifest::CollectionSegment {
        collection_key: "hooks".to_string(),
        entries,
    };
    let mut residue = std::collections::BTreeMap::new();
    residue.insert("autoMemoryEnabled".to_string(), serde_json::json!(false));
    residue.insert("worktree".to_string(), serde_json::json!(true));
    let expected = json_manifest::write_manifest(&[segment], &residue);

    let manifest_path = harness.join(".claude").join("settings.json");
    assert_eq!(fs::read_to_string(&manifest_path).unwrap(), expected);
    // The residue keys survive verbatim beside the hooks segment.
    let written = fs::read_to_string(&manifest_path).unwrap();
    assert!(written.contains("\"autoMemoryEnabled\": false"));
    assert!(written.contains("\"worktree\": true"));
    assert!(written.contains("\"hooks\""));

    // Double emit reproduces the manifest byte-for-byte — the residue fold is deterministic.
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "settings.json"), EmitOutcome::Unchanged);
    assert_eq!(fs::read_to_string(&manifest_path).unwrap(), expected);
}

#[test]
fn a_settings_residue_key_with_no_manifest_to_land_in_refuses_loud() {
    let (_harness, into) = workspace("settings-residue-unplaceable");

    // A settings row naming `settings.json`, but no in-play kind declares that manifest —
    // the residue has nowhere to land, so emit refuses rather than shedding the key.
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::rule_kind_facts(None, &[])],
            settings: vec![SettingsRow {
                manifest: "settings.json".to_string(),
                key: "autoMemoryEnabled".to_string(),
                value: serde_json::json!(false),
            }],
            ..Default::default()
        },
        members: Vec::new(),
    };

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let rendered = format!("{err:?}");
    assert!(
        rendered.contains("autoMemoryEnabled") && rendered.contains("nowhere to land"),
        "an unplaceable settings key refuses loud: {rendered}"
    );
}

#[test]
fn a_hook_round_trips_settings_json_read_to_members_to_write_byte_identical() {
    use std::collections::BTreeMap;
    use temper::kind::{CollectionAddress, CollectionKeyPath};

    let (harness, into) = workspace("hook-round-trip");

    // A canonical `settings.json` in Claude Code's documented shape — the array of matcher
    // groups this repo's own live file carries (code.claude.com/docs/en/hooks): one
    // tool-scoped event (a `matcher`) and one without. Built through the write face itself
    // so the fixture is exactly the bytes emit produces, then proven to survive a read.
    let mut entries = BTreeMap::new();
    entries.insert(
        "PostToolUse".to_string(),
        serde_json::json!([ { "hooks": [ { "command": "cargo fmt", "type": "command" } ], "matcher": "Edit|Write" } ]),
    );
    entries.insert(
        "SessionStart".to_string(),
        serde_json::json!([ { "hooks": [ { "command": "temper reporter", "type": "command" } ] } ]),
    );
    let source = json_manifest::write_manifest(
        &[json_manifest::CollectionSegment {
            collection_key: "hooks".to_string(),
            entries,
        }],
        &BTreeMap::new(),
    );
    // The fixture reproduces the live array-of-matcher-groups shape, never the flat object.
    assert!(source.contains("\"PostToolUse\": [\n"));
    assert!(source.contains("\"matcher\": \"Edit|Write\""));

    let settings_path = harness.join(".claude").join("settings.json");
    fs::create_dir_all(settings_path.parent().unwrap()).unwrap();
    fs::write(&settings_path, &source).unwrap();

    // Read each hook off the source through the adapter's read face, decomposing the matcher
    // groups back into flat {matcher?, type, command} fields.
    let address = CollectionAddress {
        manifest: "settings.json".to_string(),
        key_path: CollectionKeyPath::HooksEvent,
    };
    let manifest = json_manifest::Manifest::read(&settings_path, &[&address]).unwrap();
    assert_eq!(manifest.members.len(), 2);

    // Feed the read members back through emit's write face — the nesting the read inverts.
    let registrations = manifest
        .members
        .iter()
        .map(|member| RegistrationRow {
            kind: "hook".to_string(),
            key: member.key.clone(),
            manifest: "settings.json".to_string(),
            key_path: "hooks.<Event>".to_string(),
            fields: member.fields.clone().into_iter().collect(),
        })
        .collect();
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![hook_kind_facts()],
            registrations,
            ..Default::default()
        },
        members: Vec::new(),
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // read -> members -> write reproduces the source byte-for-byte: the idempotence keystone.
    assert_eq!(fs::read_to_string(&settings_path).unwrap(), source);
}

// ---------------------------------------------------------------------------
// Reap — a member dropped from the program leaves its projection stranded in
// the prior lock; emit reaps it iff untouched, else reports the drift and
// leaves it in place.
// ---------------------------------------------------------------------------

fn with_both_members() -> Payload {
    basic_payload(vec![
        common::rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY),
        common::skill_member(
            "coordinate",
            "Use when coordinating agents across axes.",
            COORDINATE_BODY,
        ),
    ])
}

fn coordinate_only() -> Payload {
    basic_payload(vec![common::skill_member(
        "coordinate",
        "Use when coordinating agents across axes.",
        COORDINATE_BODY,
    )])
}

#[test]
fn re_emitting_after_a_member_is_removed_reaps_an_untouched_projection() {
    let (harness, into) = workspace("reap-clean");
    drift::emit(&with_both_members(), &into, EmitOptions::default()).unwrap();
    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    assert!(rule_path.is_file());

    let report = drift::emit(&coordinate_only(), &into, EmitOptions::default()).unwrap();

    assert_eq!(outcome(&report, "rust"), EmitOutcome::Reaped);
    assert!(
        !rule_path.exists(),
        "a byte-identical orphan is reaped — temper wrote every one of its bytes"
    );
    assert_eq!(outcome(&report, "coordinate"), EmitOutcome::Unchanged);

    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        !lock.contains("[[rule]]"),
        "the reaped member's kind carries no current owner and no rollup row: {lock}"
    );
}

#[test]
fn re_emitting_after_a_member_is_removed_leaves_a_hand_edited_projection_and_reports_drift() {
    let (harness, into) = workspace("reap-drift");
    drift::emit(&with_both_members(), &into, EmitOptions::default()).unwrap();

    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    let hand_edited = fs::read_to_string(&rule_path).unwrap() + "\nHand-authored addendum.\n";
    fs::write(&rule_path, &hand_edited).unwrap();

    let report = drift::emit(&coordinate_only(), &into, EmitOptions::default()).unwrap();

    assert_eq!(outcome(&report, "rust"), EmitOutcome::OrphanDrift);
    assert_eq!(
        fs::read_to_string(&rule_path).unwrap(),
        hand_edited,
        "a drifted orphan is left on disk, never silently deleted"
    );
}

#[test]
fn dry_run_reports_a_reap_without_deleting_the_orphan() {
    let (harness, into) = workspace("reap-dry");
    drift::emit(&with_both_members(), &into, EmitOptions::default()).unwrap();
    let rule_path = harness.join(".claude").join("rules").join("rust.md");

    let report = drift::emit(
        &coordinate_only(),
        &into,
        EmitOptions {
            dry_run: true,
            ..Default::default()
        },
    )
    .unwrap();

    assert_eq!(outcome(&report, "rust"), EmitOutcome::Reaped);
    assert!(rule_path.is_file(), "--dry-run must not delete the orphan");
}

#[test]
fn an_orphan_already_removed_by_hand_is_neither_reaped_nor_reported() {
    let (harness, into) = workspace("reap-gone");
    drift::emit(&with_both_members(), &into, EmitOptions::default()).unwrap();
    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    fs::remove_file(&rule_path).unwrap();

    let report = drift::emit(&coordinate_only(), &into, EmitOptions::default()).unwrap();

    assert!(
        report.entries.iter().all(|e| e.name != "rust"),
        "nothing is left to reap or report once the file is already gone: {:?}",
        report.entries
    );
}

#[test]
fn the_emit_report_distinguishes_reaped_from_drifted_orphan() {
    let (harness, into) = workspace("reap-both");
    let payload = basic_payload(vec![
        common::rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY),
        common::rule_member("go", Some(&["**/*.go"]), "# Go conventions\n"),
        common::skill_member(
            "coordinate",
            "Use when coordinating agents across axes.",
            COORDINATE_BODY,
        ),
    ]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let go_path = harness.join(".claude").join("rules").join("go.md");
    let hand_edited = fs::read_to_string(&go_path).unwrap() + "\nHand-authored.\n";
    fs::write(&go_path, &hand_edited).unwrap();

    let report = drift::emit(&coordinate_only(), &into, EmitOptions::default()).unwrap();

    assert_eq!(outcome(&report, "rust"), EmitOutcome::Reaped);
    assert_eq!(outcome(&report, "go"), EmitOutcome::OrphanDrift);

    let rendered = drift::render_emit(&report);
    assert!(rendered.contains("reaped"), "{rendered}");
    assert!(rendered.contains("orphan-drift"), "{rendered}");
    assert!(
        rendered.contains("1 reaped, 1 orphan-drift"),
        "the tally names both outcomes: {rendered}"
    );
}

/// Serializes any chdir'ing test — cwd is process-global, so a relative-path
/// emit run must not overlap another test that reads or writes cwd.
static CWD_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn emitting_into_a_dot_slash_workspace_never_reaps_a_projection_a_bare_lock_owns() {
    // The prior lock spells owned paths off `.temper` (harness_root `""`); a re-emit
    // into `./.temper` (harness_root `"."` before normalization) spells them
    // `./.claude/…` — a mismatch that would strand every live projection as an
    // ownerless orphan and reap the freshly written bytes. Both spellings name one
    // surface, so nothing is reaped and every member stays Unchanged.
    let harness = common::tmpdir("dot-slash-workspace");
    fs::create_dir_all(harness.join(".temper")).unwrap();

    let guard = CWD_MUTEX.lock().unwrap();
    let original_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&harness).unwrap();

    let payload = with_both_members();
    let bare = PathBuf::from(".temper");
    drift::emit(&payload, &bare, EmitOptions::default()).unwrap();

    let rule_path = PathBuf::from(".claude/rules/rust.md");
    assert!(rule_path.is_file(), "the first emit writes the projection");

    let dot_slash = PathBuf::from("./.temper");
    let report = drift::emit(&payload, &dot_slash, EmitOptions::default()).unwrap();

    std::env::set_current_dir(&original_cwd).unwrap();
    drop(guard);

    assert_eq!(
        outcome(&report, "rust"),
        EmitOutcome::Unchanged,
        "a live projection a bare-spelled lock owns is reported unchanged, never reaped"
    );
    assert!(
        !report
            .entries
            .iter()
            .any(|e| e.name == "rust" && e.outcome == EmitOutcome::Reaped),
        "the live projection is never both reaped and unchanged: {:?}",
        report.entries
    );
    assert!(
        harness.join(".claude/rules/rust.md").is_file(),
        "the byte-faithful projection survives the differently-spelled re-emit"
    );
}

// ---------------------------------------------------------------------------
// The seam — `drift::emit_program` over a real `node` subprocess running the
// built SDK against a fixture `harness.ts`.
// ---------------------------------------------------------------------------

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
    let harness = common::tmpdir(label);
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    fs::write(into.join("harness.ts"), program).unwrap();

    let node_modules_scope = into.join("node_modules").join("@dtmd");
    common::vendor_sdk(&node_modules_scope);

    (harness, into)
}

/// A fixture SDK program declaring a `require`d requirement carrying a `count`
/// set-scope clause — proving the real SDK emits a requirement's demand as
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
      prose: "the harness fields a bounded agent roster",
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

    // The satisfies row addresses its filler by the `kind:name` label the real SDK emits,
    // never the bare member name — the wire the read side joins on.
    assert_eq!(
        declarations.satisfies,
        vec![drift::SatisfiesRow {
            member: "skill:coordinate".to_string(),
            requirement: "agents".to_string(),
        }],
    );
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
    let harness_after_first = common::tree_bytes(&harness);
    let lock_after_first = fs::read(into.join("lock.toml")).unwrap();

    let second = drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&second, "rust"), EmitOutcome::Unchanged);
    assert_eq!(outcome(&second, "coordinate"), EmitOutcome::Unchanged);
    assert_eq!(
        harness_after_first,
        common::tree_bytes(&harness),
        "a second, independent node run reproduces the projection byte-for-byte"
    );
    assert_eq!(
        lock_after_first,
        fs::read(into.join("lock.toml")).unwrap(),
        "a second, independent node run reproduces the lock byte-for-byte"
    );
}

/// A fixture SDK program that records the entry path Node actually received
/// (`process.argv[1]`) to a sibling file before emitting — the probe for the
/// `\\?\`-verbatim cascade (field report 1): `run_sdk_program` canonicalizes
/// the harness entry, which on Windows yields the verbatim form Node's
/// `resolveMainPath` rejects, so the argv it received must never carry it.
const ARGV_PROBE_PROGRAM: &str = r#"
import { emit, harness, text } from "@dtmd/temper";
import { skill } from "@dtmd/temper/claude-code";
import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

writeFileSync(
  join(dirname(fileURLToPath(import.meta.url)), "argv-received.txt"),
  process.argv[1],
);

const program = harness({
  members: [
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

#[test]
fn emit_program_hands_node_an_entry_path_with_no_verbatim_prefix() {
    let (_harness, into) = wire_sdk_harness_program("argv-probe", ARGV_PROBE_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let argv = fs::read_to_string(into.join("argv-received.txt")).unwrap();
    assert!(
        !argv.starts_with(r"\\?\"),
        "the entry path handed to the SDK program must carry no Windows verbatim prefix: {argv}"
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
    let harness = common::tmpdir("no-scratch");
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
