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
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};
use temper::drift::{
    self, CollectionAddressRow, Declarations, EmitOptions, EmitOutcome, KindFactRow,
    NestedMemberRow, Payload, PayloadMember, RegistrationRow, SettingsRow,
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
        host: None,
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

/// A flat `file` kind fact over `root`/`glob` named `name` — unit shape `file`, the
/// shape whose projection path splices the member name into the glob's lone `*`.
fn flat_file_kind_facts(name: &str, root: &str, glob: &str) -> KindFactRow {
    KindFactRow {
        unit_shape: Some("file".to_string()),
        ..common::kind_facts(name, root, glob)
    }
}

/// A bare `PayloadMember` of `kind` named `name` — no fields, a one-line body.
fn plain_member(kind: &str, name: &str) -> PayloadMember {
    PayloadMember {
        kind: kind.to_string(),
        name: name.to_string(),
        host: None,
        fields: Vec::new(),
        body: "# Body\n".to_string(),
        source_path: None,
    }
}

#[test]
fn a_flat_file_kind_with_a_multi_segment_glob_refuses_naming_the_depth_shapes() {
    let (_harness, into) = workspace("flat-glob-multi-segment");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![flat_file_kind_facts("spec", ".claude", "docs/*.md")],
            ..Default::default()
        },
        members: vec![plain_member("spec", "intent")],
    };

    // A multi-segment glob would splice the name into the `*` and leave a literal
    // `docs/` segment: no one path to project onto, so emit refuses before writing.
    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("spec"), "names the offending kind: {msg}");
    assert!(msg.contains("skill"), "names the skill depth shape: {msg}");
    assert!(
        msg.contains("nesting kind"),
        "names the nesting-kind shape: {msg}"
    );
}

#[test]
fn a_flat_file_kind_with_a_multi_star_glob_refuses() {
    let (_harness, into) = workspace("flat-glob-multi-star");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![flat_file_kind_facts("spec", ".claude", "*-*.md")],
            ..Default::default()
        },
        members: vec![plain_member("spec", "intent")],
    };

    // Two `*`s: splicing the first leaves a literal `*` in the path.
    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    assert!(format!("{err}").contains("spec"), "{err}");
}

#[test]
fn a_single_star_and_any_depth_glob_project_to_the_expected_paths() {
    let (harness, into) = workspace("flat-glob-project-unchanged");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                flat_file_kind_facts("spec", "specs", "*.md"),
                flat_file_kind_facts("memory", ".", "**/CLAUDE.md"),
            ],
            ..Default::default()
        },
        members: vec![
            plain_member("spec", "intent"),
            plain_member("memory", "root"),
        ],
    };

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "intent"), EmitOutcome::Emitted);
    assert_eq!(outcome(&report, "root"), EmitOutcome::Emitted);

    // The single-`*` glob splices the name into its one segment; the any-depth `**`
    // glob lands the root `<name>.md` — both unchanged by the depth refusal.
    assert!(harness.join("specs").join("intent.md").is_file());
    assert!(harness.join("root.md").is_file());
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
            host: None,
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

#[test]
fn a_lock_carried_to_a_second_root_owns_that_root_and_strands_nothing() {
    // Every row is harness-root-relative, so a lock is a statement about its harness
    // and not about the tree the emit that wrote it happened to run over. Carried to a
    // second root, its rows name that root's own files: the second harness emits its
    // whole tree, the first is not the second's business, and neither is stranded.
    let (harness, into) = workspace("carried-lock");
    let payload = with_both_members();
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let rust = harness.join(".claude/rules/rust.md");
    let skill = harness.join(".claude/skills/coordinate/SKILL.md");
    assert!(rust.is_file() && skill.is_file());

    let nested = harness.join("inner").join(".temper");
    fs::create_dir_all(&nested).unwrap();
    fs::copy(into.join("lock.toml"), nested.join("lock.toml")).unwrap();

    let report = drift::emit(&payload, &nested, EmitOptions::default()).unwrap();

    assert!(
        !report
            .entries
            .iter()
            .any(|e| e.outcome == EmitOutcome::Reaped),
        "the carried rows resolve under the root this emit targets, so nothing reads \
         ownerless and nothing is reaped: {:?}",
        report.entries
    );
    assert!(
        harness.join("inner/.claude/rules/rust.md").is_file(),
        "the second root's tree lands under it"
    );
    assert!(
        rust.is_file() && skill.is_file(),
        "the first root's projections are untouched — a second harness never reaches \
         into one it does not own"
    );
}

#[test]
fn a_payload_owning_nothing_over_a_live_tree_refuses_the_total_reap_wave() {
    // Every member gone from the program at once, while the lock's projections all sit
    // byte-faithful on disk: the whole prior tree reads ownerless and would be deleted
    // with nothing emitted in its place. That teardown refuses at the cliff (decision
    // 0024) and deletes nothing, unless the author spells it.
    let (harness, into) = workspace("total-wave");
    drift::emit(&with_both_members(), &into, EmitOptions::default()).unwrap();

    let rust = harness.join(".claude/rules/rust.md");
    let skill = harness.join(".claude/skills/coordinate/SKILL.md");
    assert!(rust.is_file() && skill.is_file());

    let memberless = basic_payload(Vec::new());
    let err = drift::emit(&memberless, &into, EmitOptions::default()).unwrap_err();
    assert!(
        err.to_string().contains("refusing to reap"),
        "the wave refuses with the finding stated: {err}"
    );
    assert!(
        rust.is_file() && skill.is_file(),
        "the refused wave deletes nothing — every live projection stays intact"
    );

    // Spelled teardown: the author names the wave on purpose, and it proceeds.
    let report = drift::emit(
        &memberless,
        &into,
        EmitOptions {
            teardown: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        report
            .entries
            .iter()
            .filter(|e| e.outcome == EmitOutcome::Reaped)
            .count(),
        2,
        "the spelled teardown reaps both projections: {:?}",
        report.entries
    );
    assert!(
        !rust.exists() && !skill.exists(),
        "the spelled teardown reaps the whole tree"
    );
}

/// A payload of one rule host plus the embedded-member declaration rows the lock
/// carries under it — the layer a re-read mining nothing would drop.
fn host_with_layer(rows: Vec<NestedMemberRow>) -> Payload {
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::rule_kind_facts(None, &[])],
            nested_members: rows,
            ..Default::default()
        },
        members: vec![common::rule_member(
            "rust",
            Some(&["src/**/*.rs"]),
            RUST_BODY,
        )],
    }
}

/// One embedded member declared under `host`, keyed `key` — a bare `NestedMemberRow`
/// with no leaves or sibling collections, enough to populate the layer.
fn embedded(host: &str, key: &str) -> NestedMemberRow {
    NestedMemberRow {
        host: host.to_string(),
        kind: "decision".to_string(),
        key: key.to_string(),
        leaves: std::collections::BTreeMap::new(),
        collections: Vec::new(),
        placed_edges: None,
    }
}

#[test]
fn a_re_read_dropping_a_whole_declared_layer_refuses_unless_teardown_is_spelled() {
    // A harness whose committed lock declares an embedded-member layer under a host,
    // re-emitted from a source that now mines zero members for it: the whole layer
    // would vanish silently (the pre-0018 harness's ~57 embedded members gone on
    // re-emit). That drop refuses at the cliff (decision 0024), the finding stated,
    // unless the author spells the teardown.
    let (_harness, into) = workspace("layer-drop");

    // First emit: the lock carries two embedded members under `rule:rust`.
    let full = host_with_layer(vec![
        embedded("rule:rust", "surface-authority"),
        embedded("rule:rust", "read-lens"),
    ]);
    drift::emit(&full, &into, EmitOptions::default()).unwrap();

    // A re-read mining nothing for the layer the lock still declares: refused before
    // a byte is written, the host and the standing member count in the finding.
    let empty = host_with_layer(Vec::new());
    let err = drift::emit(&empty, &into, EmitOptions::default()).unwrap_err();
    let rendered = err.to_string();
    assert!(
        rendered.contains("rule:rust")
            && rendered.contains("embedded-member layer")
            && rendered.contains('2'),
        "the whole-layer drop refuses with the finding stated: {err}"
    );

    // Spelled teardown: the author names the layer removal on purpose, and it proceeds.
    let report = drift::emit(
        &empty,
        &into,
        EmitOptions {
            teardown: true,
            ..Default::default()
        },
    )
    .expect("the spelled teardown lets the whole-layer removal through");
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Unchanged);
}

#[test]
fn a_layer_that_loses_one_of_many_members_still_emits() {
    // A host that keeps at least one derived member is not a dropped layer: a partial
    // loss is ordinary drift, never the cliff refusal.
    let (_harness, into) = workspace("layer-partial");
    let full = host_with_layer(vec![
        embedded("rule:rust", "surface-authority"),
        embedded("rule:rust", "read-lens"),
    ]);
    drift::emit(&full, &into, EmitOptions::default()).unwrap();

    let partial = host_with_layer(vec![embedded("rule:rust", "surface-authority")]);
    drift::emit(&partial, &into, EmitOptions::default())
        .expect("a partial member loss is not a dropped layer — it emits");
}

/// Serializes any chdir'ing test — cwd is process-global, so a relative-path
/// emit run must not overlap another test that reads or writes cwd.
static CWD_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn emitting_into_a_dot_slash_workspace_never_reaps_a_projection_a_bare_lock_owns() {
    // `.temper` and `./.temper` name one surface, but their raw `parent()`s differ —
    // left to fork, the two emits would spell one owned path two ways, strand every
    // live projection as an ownerless orphan, and reap the freshly written bytes.
    // Nothing is reaped and every member stays Unchanged.
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

#[test]
fn a_lock_whose_rows_are_dot_slash_spelled_matches_its_live_projections_and_reaps_none() {
    // A pre-normalization engine wrote `./`-prefixed source paths into the lock; the
    // current pass keys the bare spelling. Reading the older lock robustly (decision
    // 0024) must normalize both sides of the reap join, or every live projection reads
    // ownerless and the sweep mass-reaps the very bytes it just wrote.
    let harness = common::tmpdir("reap-dot-slash-lock");
    fs::create_dir_all(harness.join(".temper")).unwrap();
    let payload = with_both_members();

    let guard = CWD_MUTEX.lock().unwrap();
    let original_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&harness).unwrap();

    let into = PathBuf::from(".temper");
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // Rewrite the freshly written lock as a pre-normalization engine would have spelled
    // it: every relative `source_path` carries a redundant `./` the current pass drops.
    let lock_path = into.join("lock.toml");
    let older_lock = fs::read_to_string(&lock_path)
        .unwrap()
        .replace("source_path = \"", "source_path = \"./");
    assert!(
        older_lock.contains("source_path = \"./.claude/rules/rust.md\""),
        "the simulated older lock spells owned paths with a `./` prefix: {older_lock}"
    );
    fs::write(&lock_path, &older_lock).unwrap();

    let rule_path = PathBuf::from(".claude/rules/rust.md");
    let skill_path = PathBuf::from(".claude/skills/coordinate/SKILL.md");
    assert!(rule_path.is_file() && skill_path.is_file());

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let rewritten = fs::read_to_string(&lock_path).unwrap();

    std::env::set_current_dir(&original_cwd).unwrap();
    drop(guard);

    let reaped: Vec<_> = report
        .entries
        .iter()
        .filter(|e| e.outcome == EmitOutcome::Reaped)
        .collect();
    assert!(
        reaped.is_empty(),
        "a `./`-spelled row joins its byte-identical projection — nothing is reaped: {reaped:?}"
    );
    assert!(
        harness.join(".claude/rules/rust.md").is_file()
            && harness.join(".claude/skills/coordinate/SKILL.md").is_file(),
        "every live projection the older lock owned survives the re-emit"
    );
    assert!(
        !rewritten.contains("source_path = \"./"),
        "the next emit rewrites the lock whole in canonical `./`-free form: {rewritten}"
    );
}

#[test]
fn a_lock_emitted_from_a_foreign_cwd_carries_the_rows_of_one_emitted_at_the_harness_root() {
    // `--into`'s only non-default use is naming a workspace off the cwd, so the cwd an
    // emit happens to run under must reach no row: the lock is committed, and a row
    // carrying the prefix `examples/base-harness/` resolves from the repo root and
    // nowhere else. Same payload, two cwds, one set of bytes.
    let at_root = common::tmpdir("cwd-rows-at-root");
    let from_parent = common::tmpdir("cwd-rows-from-parent");
    fs::create_dir_all(at_root.join(".temper")).unwrap();
    fs::create_dir_all(from_parent.join(".temper")).unwrap();
    let payload = with_both_members();

    let guard = CWD_MUTEX.lock().unwrap();
    let original_cwd = std::env::current_dir().unwrap();

    // Lane A: cwd = the harness root — the one spelling already committed on disk.
    std::env::set_current_dir(&at_root).unwrap();
    let at_root_report = drift::emit(&payload, Path::new(".temper"), EmitOptions::default());

    // Lane B: cwd = the harness's parent, the workspace named off it — the shape that
    // re-based every row before, silently and at exit 0.
    std::env::set_current_dir(from_parent.parent().unwrap()).unwrap();
    let named_off_cwd = PathBuf::from(from_parent.file_name().unwrap()).join(".temper");
    let foreign_report = drift::emit(&payload, &named_off_cwd, EmitOptions::default());

    std::env::set_current_dir(&original_cwd).unwrap();
    drop(guard);

    let at_root_report = at_root_report.unwrap();
    let foreign_report = foreign_report.unwrap();

    let at_root_lock = fs::read_to_string(at_root.join(".temper/lock.toml")).unwrap();
    let foreign_lock = fs::read_to_string(from_parent.join(".temper/lock.toml")).unwrap();
    assert_eq!(
        at_root_lock, foreign_lock,
        "the same payload emits the same lock bytes from any cwd"
    );
    assert!(
        at_root_lock.contains("source_path = \".claude/rules/rust.md\""),
        "every row is spelled against the harness root, never the cwd: {at_root_lock}"
    );

    for report in [&at_root_report, &foreign_report] {
        assert!(
            !report
                .entries
                .iter()
                .any(|e| e.outcome == EmitOutcome::Reaped),
            "a first emit over a fresh harness reaps nothing: {:?}",
            report.entries
        );
    }
    assert!(
        from_parent.join(".claude/rules/rust.md").is_file(),
        "the projection lands under the targeted harness, not under the cwd"
    );
}

#[test]
fn a_re_emit_from_a_foreign_cwd_joins_the_prior_rows_under_the_targeted_root() {
    // The reap join is the row-reading side of the same contract: a row names a path
    // under the harness the verb was aimed at. Emit once with the workspace named off a
    // parent cwd, then re-emit the same harness from a cwd it does not sit under — the
    // prior rows must still find their files, or a live projection reads ownerless
    // while a dropped member's stays on disk forever.
    let harness = common::tmpdir("foreign-cwd-reemit");
    fs::create_dir_all(harness.join(".temper")).unwrap();

    let guard = CWD_MUTEX.lock().unwrap();
    let original_cwd = std::env::current_dir().unwrap();

    std::env::set_current_dir(harness.parent().unwrap()).unwrap();
    let named_off_cwd = PathBuf::from(harness.file_name().unwrap()).join(".temper");
    let first = drift::emit(&with_both_members(), &named_off_cwd, EmitOptions::default());

    // Back to a cwd the harness sits nowhere under, and aim the re-emit at it by path.
    std::env::set_current_dir(&original_cwd).unwrap();
    let report = drift::emit(
        &coordinate_only(),
        &harness.join(".temper"),
        EmitOptions::default(),
    );
    drop(guard);

    first.unwrap();
    let report = report.unwrap();

    assert_eq!(
        outcome(&report, "coordinate"),
        EmitOutcome::Unchanged,
        "the live projection the prior lock owns joins its row and survives: {:?}",
        report.entries
    );
    assert!(
        harness.join(".claude/skills/coordinate/SKILL.md").is_file(),
        "the surviving projection is never reaped"
    );

    // The other side of the same join: the dropped member's row resolves under the
    // targeted root too, so its stranded projection is found and reaped.
    assert_eq!(
        outcome(&report, "rust"),
        EmitOutcome::Reaped,
        "the dropped member's row resolves under the targeted root: {:?}",
        report.entries
    );
    assert!(
        !harness.join(".claude/rules/rust.md").exists(),
        "the reap deletes the file the row names under the harness, not under the cwd"
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

/// The common fixture-harness wiring bound to this suite's [`HARNESS_PROGRAM`].
fn wire_sdk_harness(label: &str) -> (PathBuf, PathBuf) {
    common::wire_sdk_harness(label, HARNESS_PROGRAM)
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
        common::wire_sdk_harness("requirement-clauses", REQUIREMENT_CLAUSES_PROGRAM);

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
    let (_harness, into) = common::wire_sdk_harness("argv-probe", ARGV_PROBE_PROGRAM);

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
    let (harness, _into) = common::wire_sdk_harness("broken-program", BROKEN_HARNESS_PROGRAM);

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

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let harness_run = common::check_in(
        manifest_dir,
        &["--harness", harness.to_str().unwrap()],
        None,
    );
    assert!(
        harness_run.ok,
        "a clean harness must gate green over --harness: {}",
        harness_run.output
    );

    let session_start_run = common::check_in(
        manifest_dir,
        &[harness.to_str().unwrap()],
        Some("session-start"),
    );
    assert!(
        session_start_run.ok,
        "session-start is always advisory: {}",
        session_start_run.output
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
