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
    self, Declarations, EmitOptions, EmitOutcome, InputRow, KindFactRow, NestedMemberRow, Payload,
    PayloadMember, RegistrationRow, SettingsRow,
};
use temper::json_manifest;
use temper::placement;

use crate::common;
use crate::common::{fresh_clause, hook_kind_facts, workspace};
use crate::module_closure::{is_vendored_sdk, without_module_rows};

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
        format!(
            "---\n{}\npaths: [\"src/**/*.rs\"]\n---\n{RUST_BODY}",
            placement::NOTE_COMMENT
        )
    );

    let skill_path = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    assert_eq!(
        fs::read_to_string(&skill_path).unwrap(),
        format!(
            "---\n{}\nname: \"coordinate\"\ndescription: \"Use when coordinating agents across axes.\"\n---\n{COORDINATE_BODY}",
            placement::NOTE_COMMENT
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
fn a_freshly_emitted_markdown_projection_carries_the_managed_projection_banner() {
    let (harness, into) = workspace("emit-banner");
    const MEMORY_BODY: &str = "# Project Memory\n\nShared context for agents.\n";

    // Emit a frontmatterless memory (CLAUDE.md) — it should carry the banner.
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                // Memory: markdown at root with CLAUDE.md name
                common::kind_facts("memory", ".", "CLAUDE.md"),
            ],
            ..Default::default()
        },
        members: vec![common::payload_member("memory", "root", None, MEMORY_BODY)],
    };

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "root"), EmitOutcome::Emitted);

    let memory_path = harness.join("CLAUDE.md");
    let content = fs::read_to_string(&memory_path).unwrap();

    // The banner should be prepended to the body with one blank line separating them.
    assert!(
        content.starts_with(&format!("{}\n\n", placement::BANNER)),
        "freshly emitted markdown should carry the banner, got: {content}"
    );
    assert!(content.contains(MEMORY_BODY), "body should be intact");

    // A re-emit should be unchanged (idempotent).
    let report2 = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report2, "root"), EmitOutcome::Unchanged);
}

#[test]
fn a_freshly_emitted_frontmatter_projection_carries_its_note_with_no_install_run() {
    // Emit owns a represented projection's bytes in full — no file is part-emitted and
    // part-installed — so the `#` managed-by note lands in the same pass that writes the
    // frontmatter block, and `check`'s self-verify has nothing left to nudge for.
    let (harness, into) = workspace("emit-note");
    let payload = basic_payload(vec![
        common::rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY),
        common::skill_member(
            "coordinate",
            "Use when coordinating agents across axes.",
            COORDINATE_BODY,
        ),
    ]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // Delete a projection and re-emit it from nothing: the note is emit's to write, not
    // carried over from bytes a prior `install` left on disk.
    let skill_path = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    fs::remove_file(&skill_path).unwrap();
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "coordinate"), EmitOutcome::Emitted);

    let content = fs::read_to_string(&skill_path).unwrap();
    assert!(
        content.starts_with(&format!("---\n{}\n", placement::NOTE_COMMENT)),
        "a re-emitted projection carries its note as the leading frontmatter line, got: {content}"
    );

    // Idempotent: the marker locates the note already there, so a re-emit neither
    // doubles it nor churns the file.
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "coordinate"), EmitOutcome::Unchanged);
    assert_eq!(
        fs::read_to_string(&skill_path)
            .unwrap()
            .matches(placement::NOTE_MARKER)
            .count(),
        1
    );

    // The gate agrees: with the harness represented and no `install` run behind it,
    // `check`'s self-verify names no missing managed-by note.
    fs::write(into.join("harness.ts"), "export default {};\n").unwrap();
    // The program alone is what marks this harness represented, and both install's faces
    // read it there: the lock only records an emit, and this one has never been installed.
    assert!(
        temper::install::represented_by(&harness).is_some(),
        "an authored program is the represent answer, whatever the lock says"
    );
    let lock = temper::drift::read_lock_document(&into).unwrap();
    let findings = temper::install::gate_installed(&temper::bytes::Bytes::new(), &harness, &lock);
    assert!(
        !findings
            .iter()
            .any(|f| f.message.contains("managed-by note")),
        "a freshly emitted projection needs no install run to converge its note, got: {findings:?}"
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
        unit: None,
        id: None,
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
        provided: None,
        member: None,
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
fn the_lock_names_the_engine_that_wrote_it_and_an_unstamped_lock_reads_unknown() {
    let (_harness, into) = workspace("engine-stamp");
    let mut payload = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
    payload.declarations.requirements.push(common::requirement(
        "dev-standards",
        true,
        Some("rule"),
    ));

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    assert_eq!(
        drift::read_engine(&into).unwrap().as_deref(),
        Some(temper::ENGINE_STAMP),
        "an emitted lock names the engine version that wrote it"
    );
    let stamped = drift::read_declarations(&into).unwrap();

    // The robust read an upgraded engine owes a committed lock. A lock written before
    // the stamp existed simply has no key, and that absence is *unknown*: never a
    // malformed-lock refusal, never a synthesized value, and never a row the
    // declaration families lose — the whole set still lifts off the older text.
    let lock = into.join("lock.toml");
    let text = fs::read_to_string(&lock).unwrap();
    let unstamped: String = text
        .split_inclusive('\n')
        .filter(|line| !line.starts_with("engine = "))
        .collect();
    assert!(
        !unstamped.contains("engine = "),
        "the older spelling under test carries no stamp at all:\n{unstamped}"
    );
    fs::write(&lock, &unstamped).unwrap();

    assert_eq!(drift::read_engine(&into).unwrap(), None);
    assert_eq!(
        drift::read_declarations(&into).unwrap(),
        stamped,
        "an unstamped lock parses to the same declarations the stamped one did"
    );

    // A *present* key of the wrong TOML type reads unknown the same way: the stamp is
    // one opaque string the engine only ever compares, so a malformed one carries no
    // less than none, and refusing the lock over it would turn a corrupt spelling into
    // a dead gate.
    fs::write(&lock, format!("engine = 20\n{unstamped}")).unwrap();
    assert_eq!(drift::read_engine(&into).unwrap(), None);
    assert_eq!(drift::read_declarations(&into).unwrap(), stamped);
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
    // to the source), never merged — there is no three-state conflict here. The report
    // names the replacement rather than riding the line a source-side re-emit takes, so
    // bytes temper never wrote are never silently destroyed.
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Overwritten);
    assert_eq!(fs::read_to_string(&rule_path).unwrap(), canonical);
    // The untouched skill is already at its fixpoint.
    assert_eq!(outcome(&report, "coordinate"), EmitOutcome::Unchanged);

    let rendered = drift::render_emit(&report);
    assert!(
        rendered.contains("overwritten    rule   rust"),
        "the report line names the member whose bytes were replaced: {rendered}"
    );
    assert!(
        rendered.contains("0 emitted, 1 overwritten, 1 unchanged"),
        "the tally counts the overwrite apart from a plain re-emit: {rendered}"
    );
}

#[test]
fn an_emit_over_a_projection_whose_kind_dropped_its_frontmatter_carries_no_stale_managed_projection_note()
 {
    // The prior file carries the `#` note in its frontmatter; the new render has no
    // frontmatter to hold it, so the overwrite must match an emit onto an absent file.
    let (harness, into) = workspace("frontmatter-dropped");
    let with_paths = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
    drift::emit(&with_paths, &into, EmitOptions::default()).unwrap();
    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    assert!(
        fs::read_to_string(&rule_path)
            .unwrap()
            .contains(placement::NOTE_MARKER)
    );

    let frontmatterless = basic_payload(vec![common::rule_member("rust", None, RUST_BODY)]);
    let report = drift::emit(&frontmatterless, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Emitted);
    let overwritten = fs::read_to_string(&rule_path).unwrap();

    let (fresh_harness, fresh_into) = workspace("frontmatter-dropped-fresh");
    drift::emit(&frontmatterless, &fresh_into, EmitOptions::default()).unwrap();
    let fresh =
        fs::read_to_string(fresh_harness.join(".claude").join("rules").join("rust.md")).unwrap();
    assert_eq!(
        overwritten, fresh,
        "the overwrite carries no placement the new shape cannot hold"
    );
    assert!(
        !overwritten.contains(placement::NOTE_MARKER),
        "got: {overwritten}"
    );

    let report = drift::emit(&frontmatterless, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Unchanged);
}

#[test]
fn an_emit_over_bytes_no_lock_fingerprinted_is_a_plain_emit() {
    // A first emit — adoption's included: the one reviewable adoption diff overwrites a
    // hand-authored file on purpose, and with no prior row there is no fingerprint to
    // have drifted from, so the outcome stays `Emitted`.
    let (harness, into) = workspace("no-prior-row");
    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    fs::create_dir_all(rule_path.parent().unwrap()).unwrap();
    fs::write(
        &rule_path,
        "# Hand-authored

Nothing temper wrote.
",
    )
    .unwrap();

    let payload = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Emitted);
    assert!(
        fs::read_to_string(&rule_path).unwrap().contains(RUST_BODY),
        "the first emit still regenerates the file whole"
    );
}

#[test]
fn a_crlf_filtered_checkout_re_emits_as_a_plain_emit() {
    // Line endings are layout, never content, so the overwrite classification hashes
    // EOL-canonicalized bytes: a checkout git filtered to CRLF is rewritten back to LF
    // and reported `Emitted`, never as a hand edit it replaced.
    let (harness, into) = workspace("crlf-rewrite");
    let payload = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    let canonical = fs::read_to_string(&rule_path).unwrap();
    fs::write(&rule_path, canonical.replace('\n', "\r\n")).unwrap();

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Emitted);
    assert_eq!(
        fs::read_to_string(&rule_path).unwrap(),
        canonical,
        "the re-emit writes LF back"
    );
}

#[test]
fn dry_run_reports_the_overwrite_and_writes_nothing() {
    let (harness, into) = workspace("dry-overwrite");
    let payload = basic_payload(vec![common::rule_member(
        "rust",
        Some(&["src/**/*.rs"]),
        RUST_BODY,
    )]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    let hand_edited =
        fs::read_to_string(&rule_path).unwrap() + "\nA line added straight to disk.\n";
    fs::write(&rule_path, &hand_edited).unwrap();

    let before_harness = common::tree_bytes(&harness);
    let before_lock = fs::read(into.join("lock.toml")).unwrap();

    let report = drift::emit(
        &payload,
        &into,
        EmitOptions {
            dry_run: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Overwritten);
    assert_eq!(
        before_harness,
        common::tree_bytes(&harness),
        "--dry-run must leave the hand edit on disk"
    );
    assert_eq!(
        before_lock,
        fs::read(into.join("lock.toml")).unwrap(),
        "--dry-run must not touch the lock"
    );
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
        format!(
            "---\n{}\npaths: [\"src/**/*.rs\"]\n---\n# Windows-authored\n\nCarries CRLF line endings.\n",
            placement::NOTE_COMMENT
        )
    );
    assert_eq!(
        String::from_utf8(lonecr_bytes.clone()).unwrap(),
        format!(
            "---\n{}\npaths: [\"src/**/*.rs\"]\n---\n# Old-Mac-authored\nCarries lone CR line endings.\n",
            placement::NOTE_COMMENT
        )
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
    payload
        .members
        .push(common::payload_member("ghost", "phantom", None, "boo"));

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    assert!(format!("{err}").contains("ghost"), "{err}");
}

#[test]
fn emit_refuses_two_members_that_derive_one_projection_path() {
    let (harness, into) = workspace("projection-path-collision");
    // A literal glob places every member at the one path, whatever its name.
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                common::rule_kind_facts(None, &[]),
                common::kind_facts("memory", ".", "CLAUDE.md"),
            ],
            ..Default::default()
        },
        members: vec![
            common::rule_member("rust", Some(&["src/**/*.rs"]), RUST_BODY),
            common::payload_member("memory", "root", None, "# Root\n"),
            common::payload_member("memory", "nested", None, "# Nested\n"),
        ],
    };

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let message = format!("{err}");
    assert!(message.contains("memory:root"), "{message}");
    assert!(message.contains("memory:nested"), "{message}");
    assert!(message.contains("`CLAUDE.md`"), "{message}");
    // Refused before a byte is written: neither the colliding path, the unrelated
    // member's projection, nor the lock lands.
    assert!(!harness.join("CLAUDE.md").exists());
    assert!(!harness.join(".claude").exists());
    assert_eq!(
        fs::read_dir(&into).unwrap().count(),
        0,
        "no lock is written"
    );
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
    common::payload_member(kind, name, None, "# Body\n")
}

/// A [`plain_member`] declaring its location relative to its kind's locus root.
fn located_member(kind: &str, name: &str, location: &str) -> PayloadMember {
    PayloadMember {
        location: Some(location.to_string()),
        ..plain_member(kind, name)
    }
}

#[test]
fn a_flat_glob_with_a_literal_leading_segment_places_under_it() {
    let (harness, into) = workspace("flat-glob-literal-segment");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![flat_file_kind_facts("spec", ".claude", "docs/*.md")],
            ..Default::default()
        },
        members: vec![plain_member("spec", "intent")],
    };

    // A literal leading segment is fixed placement, not depth the splice cannot carry:
    // the name takes the final segment's lone `*` and `docs/` is spliced verbatim.
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "intent"), EmitOutcome::Emitted);
    assert!(
        harness
            .join(".claude")
            .join("docs")
            .join("intent.md")
            .is_file()
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
fn a_flat_file_kind_whose_star_sits_above_the_leaf_refuses() {
    let (_harness, into) = workspace("flat-glob-star-above-leaf");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![flat_file_kind_facts("spec", ".claude", "*/NOTES.md")],
            ..Default::default()
        },
        members: vec![plain_member("spec", "intent")],
    };

    // The one `*` stars a directory the member name does not identify — that reading is
    // the starred-segment unit shape's, which this `file`-shaped kind does not declare.
    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("spec"), "names the offending kind: {msg}");
    assert!(
        msg.contains("above the leaf"),
        "states the narrowed rule: {msg}"
    );
}

#[test]
fn a_member_name_carrying_the_address_separator_refuses_before_placing_outside_its_glob() {
    let (harness, into) = workspace("member-name-separator");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![flat_file_kind_facts("spec", "docs", "*.json")],
            ..Default::default()
        },
        members: vec![plain_member("spec", "a/b/sub/x")],
    };

    // A flat `*.json` glob reaches exactly one segment under its root; splicing a slashed
    // name through it would land `docs/a/b/sub/x.json`, three levels past anything the
    // glob can find — so the name is refused, not the path repaired.
    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("a/b/sub/x"), "names the offending name: {msg}");
    assert!(
        msg.contains("separator"),
        "names the separator fault: {msg}"
    );
    assert!(
        !harness.join("docs").exists(),
        "the refusal lands before a byte is written",
    );
}

#[test]
fn a_derived_path_its_own_kinds_glob_cannot_find_refuses_rather_than_emitting() {
    let (harness, into) = workspace("ungoverned-projection");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![KindFactRow {
                unit_shape: Some("directory".to_string()),
                ..common::kind_facts("guide", ".claude/guides", "GUIDE.md")
            }],
            ..Default::default()
        },
        members: vec![plain_member("guide", "operate-the-gate")],
    };

    // A directory unit lands its entry file one segment deep, so this kind projects
    // `.claude/guides/operate-the-gate/GUIDE.md` — a path its own bare `GUIDE.md` glob
    // never reaches, since discovery matches a glob segment by segment from the locus
    // root. Emitted, that projection is written and locked yet ungoverned; the declared
    // shape is `*/GUIDE.md`.
    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("operate-the-gate/GUIDE.md"),
        "names the derived path: {msg}",
    );
    assert!(msg.contains("`GUIDE.md`"), "names the glob: {msg}");
    assert!(
        !harness.join(".claude").join("guides").exists(),
        "the refusal lands before a byte is written",
    );
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
            plain_member("memory", "CLAUDE"),
        ],
    };

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "intent"), EmitOutcome::Emitted);
    assert_eq!(outcome(&report, "CLAUDE"), EmitOutcome::Emitted);

    // The single-`*` glob splices the name into its one segment. The `**/CLAUDE.md` glob
    // collapses its any-depth prefix to nothing and carries no `*` at all: it is the
    // fixed-path class, landing the root `CLAUDE.md` rather than a `<name>.md` the glob
    // never spelled.
    assert!(harness.join("specs").join("intent.md").is_file());
    assert!(harness.join("CLAUDE.md").is_file());
}

#[test]
fn an_any_depth_glob_places_its_own_formats_leaf_never_a_hardcoded_md() {
    let (harness, into) = workspace("any-depth-own-format");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![KindFactRow {
                format: Some("json-document".to_string()),
                ..flat_file_kind_facts("catalog", ".claude", "**/sub/*.json")
            }],
            ..Default::default()
        },
        members: vec![PayloadMember {
            kind: "catalog".to_string(),
            name: "tools".to_string(),
            host: None,
            location: None,
            fields: vec![("name".to_string(), serde_json::json!("tools"))],
            body: String::new(),
            source_path: None,
        }],
    };

    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "tools"), EmitOutcome::Emitted);

    // The any-depth prefix names where the glob *matches*, never where a projection
    // *lands*: what remains places the member under its literal `sub/` segment in its
    // own format's extension. The retired arm hardcoded `<name>.md`, which handed a
    // JSON kind a markdown path no format of its could read back.
    assert!(
        harness
            .join(".claude")
            .join("sub")
            .join("tools.json")
            .is_file(),
        "the `**/sub/*.json` kind lands `sub/tools.json`"
    );
    assert!(!harness.join(".claude").join("tools.md").exists());
}

// ---------------------------------------------------------------------------
// Represented manifests — a container member's residue plus its registration
// members are regenerated whole through the canonical write face, never
// json_splice's in-place edit.
// ---------------------------------------------------------------------------

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
                emit_hash: None,
            }],
            ..Default::default()
        },
        members: vec![PayloadMember {
            kind: "settings".to_string(),
            name: "settings".to_string(),
            host: None,
            location: None,
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

/// The project-root-relative program the default tool renders to — the temper package's
/// bin at the `.bin` path the workspace's package manager lays out.
const DEFAULT_PROGRAM: &str = "./.temper/node_modules/.bin/temper";

/// A [`workspace`] whose harness carries the committed script the `script` tool of
/// [`runner_tools`] states, so the tool's file input resolves.
fn tool_workspace(label: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let (harness, into) = workspace(label);
    fs::create_dir_all(harness.join("bin")).unwrap();
    fs::write(harness.join("bin").join("temper"), "#!/bin/sh\n").unwrap();
    (harness, into)
}

/// The root-hosted tool members a runner payload composes: a workspace package bin, a
/// committed script, and a system tool.
fn runner_tools() -> Vec<temper::drift::NestedMemberRow> {
    let tool = |key: &str, leaves: &[(&str, &str)]| temper::drift::NestedMemberRow {
        host: "root".to_string(),
        kind: "tool".to_string(),
        key: key.to_string(),
        leaves: leaves
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect(),
        collections: Vec::new(),
        placed_edges: None,
        rendered_lines: None,
        rendered_chars: None,
    };
    vec![
        tool(
            "workspace-temper",
            &[("package", "@dtmd/temper"), ("bin", "temper")],
        ),
        tool("script", &[("file", "bin/temper")]),
        tool("system", &[("system", "temper")]),
    ]
}

/// A payload carrying one synthesized tap-hook row — the shape the SDK derives from a
/// telemetry verifier: `kind` hook, the group's own `matcher` field, and a single
/// `command` handler carrying the declaration's tool address under the seam's reserved
/// `temper:tap-tool` key (`null` declaring none). A `Skill` invocation surfaces under
/// `PostToolUse` with the tool-name matcher.
fn tap_hook_payload(tool: serde_json::Value) -> Payload {
    tap_hook_payload_with(serde_json::json!([{ "type": "command", "temper:tap-tool": tool }]))
}

/// [`tap_hook_payload`] with the `PostToolUse` tap's whole handler array given — the shape
/// the SDK sends when declarations sharing the tap reach different tools: one handler per
/// tool, each naming its declaration under `temper:tap-declaration`.
fn tap_hook_payload_with(handlers: serde_json::Value) -> Payload {
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![hook_kind_facts()],
            registrations: vec![RegistrationRow {
                kind: "hook".to_string(),
                key: "PostToolUse".to_string(),
                manifest: "settings.json".to_string(),
                key_path: "hooks.<Event>".to_string(),
                fields: vec![
                    ("matcher".to_string(), serde_json::json!("Skill")),
                    ("hooks".to_string(), handlers),
                ],
                emit_hash: None,
            }],
            nested_members: runner_tools(),
            ..Default::default()
        },
        members: Vec::new(),
    }
}

/// The `PostToolUse` tap handler's `command` in the settings manifest `emit` projected.
fn projected_tap_command(harness: &Path) -> String {
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap(),
    )
    .unwrap();
    let handler = &manifest["hooks"]["PostToolUse"][0]["hooks"][0];
    assert!(
        handler.get("temper:tap-tool").is_none(),
        "the seam's tool key never reaches the manifest: {handler}"
    );
    handler["command"].as_str().unwrap().to_owned()
}

#[test]
fn a_synthesized_tap_hook_projects_its_matcher_group_through_the_registration_loop() {
    let (harness, into) = tool_workspace("tap-hook-matcher-group");
    let payload = tap_hook_payload(serde_json::Value::Null);

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // The `matcher` field lifts to the group level and the handler array rides through with
    // its command rendered by the engine — `hooks.PostToolUse` is the array of matcher
    // groups Claude Code loads, each `{matcher, hooks:[{…handler}]}`.
    let mut entries = std::collections::BTreeMap::new();
    entries.insert(
        "PostToolUse".to_string(),
        serde_json::json!([
            { "matcher": "Skill", "hooks": [ { "type": "command", "command": temper::install::tap_hook_command(DEFAULT_PROGRAM) } ] }
        ]),
    );
    let segment = json_manifest::CollectionSegment {
        collection_key: "hooks".to_string(),
        entries,
    };
    let expected = json_manifest::write_manifest(&[segment], &std::collections::BTreeMap::new());

    let manifest_path = harness.join(".claude").join("settings.json");
    assert_eq!(fs::read_to_string(&manifest_path).unwrap(), expected);

    // Double emit reproduces the manifest byte-for-byte: the rendered command is the
    // entry's hash, so a re-render is no drift.
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(fs::read_to_string(&manifest_path).unwrap(), expected);
}

#[test]
fn a_tap_hook_command_is_anchored_at_the_project() {
    // A declaration naming no tool runs the temper package's bin — entered at the project
    // root Claude Code names, so the bin's path and the tap's `.` target resolve there
    // whatever directory the session has moved to, as a gate command's do.
    let (harness, into) = tool_workspace("tap-hook-anchored");
    drift::emit(
        &tap_hook_payload(serde_json::Value::Null),
        &into,
        EmitOptions::default(),
    )
    .unwrap();

    assert_eq!(
        projected_tap_command(&harness),
        "cd \"${CLAUDE_PROJECT_DIR:-.}\" || exit 1; command -v ./.temper/node_modules/.bin/temper >/dev/null 2>&1 || { echo \"./.temper/node_modules/.bin/temper: command not found; install temper with npm install -g @dtmd/temper, or npm -C .temper install for a represented harness\" >&2; exit 2; } && ./.temper/node_modules/.bin/temper tap ."
    );
}

#[test]
fn a_tap_runs_the_command_of_its_declarations_tool() {
    // A declaration reaching a committed-script tool runs that script, resolved at the
    // project root the anchor entered; a system tool runs its name on the PATH.
    let (harness, into) = tool_workspace("tap-hook-declared-tool");
    drift::emit(
        &tap_hook_payload(serde_json::json!("root/tool/script")),
        &into,
        EmitOptions::default(),
    )
    .unwrap();
    assert_eq!(
        projected_tap_command(&harness),
        "cd \"${CLAUDE_PROJECT_DIR:-.}\" || exit 1; command -v ./bin/temper >/dev/null 2>&1 || { echo \"./bin/temper: command not found; install temper with npm install -g @dtmd/temper, or npm -C .temper install for a represented harness\" >&2; exit 2; } && ./bin/temper tap ."
    );

    let (harness, into) = tool_workspace("tap-hook-system-tool");
    drift::emit(
        &tap_hook_payload(serde_json::json!("root/tool/system")),
        &into,
        EmitOptions::default(),
    )
    .unwrap();
    assert_eq!(
        projected_tap_command(&harness),
        temper::install::tap_hook_command("temper")
    );
}

#[test]
fn tap_rows_sharing_a_tap_emit_one_tap_when_one_declares_no_tool_and_the_other_the_temper_package_bin()
 {
    // An undeclared tool is the engine's default, the temper package's bin, so the two
    // resolve alike and are one tap — one handler, its command rendered once.
    let (harness, into) = tool_workspace("tap-hook-resolved-agree");
    drift::emit(
        &tap_hook_payload_with(serde_json::json!([
            { "type": "command", "temper:tap-tool": "root/tool/workspace-temper", "temper:tap-declaration": "a" },
            { "type": "command", "temper:tap-tool": null, "temper:tap-declaration": "b" },
        ])),
        &into,
        EmitOptions::default(),
    )
    .unwrap();

    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        manifest["hooks"]["PostToolUse"],
        serde_json::json!([
            { "matcher": "Skill", "hooks": [ { "type": "command", "command": temper::install::tap_hook_command(DEFAULT_PROGRAM) } ] }
        ])
    );
}

#[test]
fn tap_rows_sharing_a_tap_refuse_at_emit_when_their_tools_render_differently_naming_both_declarations()
 {
    let (harness, into) = tool_workspace("tap-hook-resolved-disagree");
    let err = drift::emit(
        &tap_hook_payload_with(serde_json::json!([
            { "type": "command", "temper:tap-tool": "root/tool/script", "temper:tap-declaration": "a" },
            { "type": "command", "temper:tap-tool": null, "temper:tap-declaration": "b" },
        ])),
        &into,
        EmitOptions::default(),
    )
    .unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("`a` and `b`")
            && message.contains("`PostToolUse`")
            && message.contains(&format!("`./bin/temper` and `{DEFAULT_PROGRAM}`")),
        "{message}"
    );
    assert!(
        !harness.join(".claude").join("settings.json").exists(),
        "a refused emit writes no manifest"
    );
}

#[test]
fn a_tap_hook_reaching_a_tool_the_program_does_not_compose_is_refused() {
    let (_harness, into) = tool_workspace("tap-hook-stray-tool");
    let err = drift::emit(
        &tap_hook_payload(serde_json::json!("root/tool/absent")),
        &into,
        EmitOptions::default(),
    )
    .unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("tap hook on `PostToolUse`") && message.contains("`root/tool/absent`"),
        "{message}"
    );
}

/// A payload carrying one gate hook per event temper's gate rides, as the SDK's
/// `gateHook(...)` erases them: one `command` handler spelling no command, and the tool's
/// address under the seam's reserved `temper:gate-tool` key (`null` declaring none).
fn gate_hook_payload(tool: serde_json::Value) -> Payload {
    let row = |key: &str, matcher: Option<&str>| {
        let mut fields = Vec::new();
        if let Some(matcher) = matcher {
            fields.push(("matcher".to_string(), serde_json::json!(matcher)));
        }
        fields.push((
            "hooks".to_string(),
            serde_json::json!([{ "type": "command" }]),
        ));
        fields.push(("temper:gate-tool".to_string(), tool.clone()));
        RegistrationRow {
            kind: "hook".to_string(),
            key: key.to_string(),
            manifest: "settings.json".to_string(),
            key_path: "hooks.<Event>".to_string(),
            fields,
            emit_hash: None,
        }
    };
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![hook_kind_facts()],
            registrations: vec![
                row("PostToolUse", Some("Bash")),
                row("PreToolUse", Some("Write|Edit|MultiEdit")),
                row("SessionStart", None),
            ],
            nested_members: runner_tools(),
            ..Default::default()
        },
        members: Vec::new(),
    }
}

/// The projected `settings.json` a gate hook payload emits, parsed — with the one command
/// each event's group runs.
fn projected_gate_commands(harness: &Path) -> (serde_json::Value, [String; 3]) {
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap(),
    )
    .unwrap();
    let command = |event: &str| {
        manifest["hooks"][event][0]["hooks"][0]["command"]
            .as_str()
            .unwrap_or_else(|| panic!("`{event}`'s gate handler carries a command: {manifest}"))
            .to_string()
    };
    let commands = [
        command("SessionStart"),
        command("PreToolUse"),
        command("PostToolUse"),
    ];
    (manifest, commands)
}

#[test]
fn a_gate_hook_runs_the_command_of_the_tool_it_reaches() {
    let (harness, into) = tool_workspace("gate-hook-declared-tool");
    let payload = gate_hook_payload(serde_json::json!("root/tool/script"));

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // Every gate hook probes, names and runs the script its tool states — resolved at the
    // project root as itself rather than looked up on the PATH.
    let (manifest, [session_start, guard, post_guard]) = projected_gate_commands(&harness);
    let probe = "cd \"${CLAUDE_PROJECT_DIR:-.}\" || exit 1; command -v ./bin/temper >/dev/null 2>&1 || { echo \"./bin/temper: command not found; install temper with npm install -g @dtmd/temper, or npm -C .temper install for a represented harness\" >&2; exit 2; } && ./bin/temper";
    assert_eq!(
        session_start,
        format!("{probe} check . --reporter session-start")
    );
    assert_eq!(guard, format!("{probe} guard ."));
    assert_eq!(post_guard, guard, "both guard edges run the one command");

    // The seam's reserved key is consumed, never projected.
    assert!(
        !manifest.to_string().contains("temper:gate-tool"),
        "the tool key never reaches the manifest: {manifest}"
    );

    // Re-emit is the idempotent no-op.
    let before = fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap();
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(
        fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap(),
        before
    );

    // A system tool runs its name on the PATH: today's commands, byte for byte.
    let (harness, into) = tool_workspace("gate-hook-system-tool");
    drift::emit(
        &gate_hook_payload(serde_json::json!("root/tool/system")),
        &into,
        EmitOptions::default(),
    )
    .unwrap();
    let (_, [session_start, guard, _]) = projected_gate_commands(&harness);
    assert_eq!(session_start, temper::install::SESSION_START_COMMAND);
    assert_eq!(guard, temper::install::GUARD_COMMAND);
}

#[test]
fn a_gate_hook_declaring_no_tool_runs_the_temper_package_bins_anchored_path() {
    let (harness, into) = tool_workspace("gate-hook-default-tool");

    drift::emit(
        &gate_hook_payload(serde_json::Value::Null),
        &into,
        EmitOptions::default(),
    )
    .unwrap();

    // No tool declared is the temper package's bin, at the `.bin` path the workspace's
    // package manager lays out, resolved at the project root the anchor entered.
    let (manifest, [session_start, guard, post_guard]) = projected_gate_commands(&harness);
    let expected_session_start =
        temper::install::gate_hook_command("SessionStart", DEFAULT_PROGRAM).unwrap();
    assert_eq!(session_start, expected_session_start);
    assert!(
        session_start.contains("&& ./.temper/node_modules/.bin/temper check ."),
        "{session_start}"
    );
    let expected_guard = temper::install::gate_hook_command("PreToolUse", DEFAULT_PROGRAM).unwrap();
    assert_eq!(guard, expected_guard);
    assert_eq!(post_guard, expected_guard);
    // The handler is exactly the command handler — the reserved key rides nowhere in it.
    assert_eq!(
        manifest["hooks"]["SessionStart"],
        serde_json::json!([{ "hooks": [{ "type": "command", "command": expected_session_start }] }])
    );
}

#[test]
fn a_gate_hook_reaching_a_tool_the_program_does_not_compose_refuses_at_emit() {
    let (_harness, into) = tool_workspace("gate-hook-stray-tool");
    let err = drift::emit(
        &gate_hook_payload(serde_json::json!("root/tool/absent")),
        &into,
        EmitOptions::default(),
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("`root/tool/absent`"),
        "the refusal names the tool: {err}"
    );
}

#[test]
fn a_gate_hook_on_an_event_temper_does_not_ride_refuses_at_emit() {
    let (_harness, into) = tool_workspace("gate-hook-foreign-event");
    let mut payload = gate_hook_payload(serde_json::Value::Null);
    payload.declarations.registrations[0].key = "Stop".to_string();

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    assert!(
        err.to_string()
            .contains("`Stop` is not an event temper's gate rides"),
        "the refusal names the event: {err}"
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
                emit_hash: None,
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

/// A represented `settings.json` payload: one hook registration per event in `events`
/// (keyed `hooks.<event>`), plus a `settings` container member whose fields are the opaque
/// `residue`. The container kind and the hook kind both govern `.claude/settings.json`, so
/// the container's member projects to the manifest the hooks surface inside.
fn settings_payload(events: &[&str], residue: &[(&str, serde_json::Value)]) -> Payload {
    let registrations = events
        .iter()
        .map(|event| RegistrationRow {
            kind: "hook".to_string(),
            key: (*event).to_string(),
            manifest: "settings.json".to_string(),
            key_path: "hooks.<Event>".to_string(),
            fields: vec![
                ("type".to_string(), serde_json::json!("command")),
                ("command".to_string(), serde_json::json!("temper reporter")),
            ],
            emit_hash: None,
        })
        .collect();
    let fields = residue
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect();
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                hook_kind_facts(),
                common::kind_facts("settings", ".claude", "settings.json"),
            ],
            registrations,
            ..Default::default()
        },
        members: vec![PayloadMember {
            kind: "settings".to_string(),
            name: "settings".to_string(),
            host: None,
            location: None,
            fields,
            body: String::new(),
            source_path: None,
        }],
    }
}

#[test]
fn a_partial_manifest_rewrite_names_every_dropped_member_in_the_reap_ledger() {
    let (harness, into) = workspace("manifest-segment-reap");

    // The live settings.json: two hooks (distinct events) and two opaque residue keys.
    let full = settings_payload(
        &["PreToolUse", "SessionStart"],
        &[
            (
                "permissions",
                serde_json::json!({ "allow": ["Bash(cargo build:*)"] }),
            ),
            (
                "extraKnownMarketplaces",
                serde_json::json!(["acme/plugins"]),
            ),
        ],
    );
    drift::emit(&full, &into, EmitOptions::default()).unwrap();

    // The partial rewrite: the SessionStart hook and `permissions` survive; the PreToolUse
    // hook and the `extraKnownMarketplaces` residue key are dropped. A whole-file rewrite
    // would strand both silently — the segment reap names each in the ledger instead.
    let partial = settings_payload(
        &["SessionStart"],
        &[(
            "permissions",
            serde_json::json!({ "allow": ["Bash(cargo build:*)"] }),
        )],
    );
    let report = drift::emit(&partial, &into, EmitOptions::default()).unwrap();

    // One reap-ledger entry per dropped discovered member — the hook keyed by its event,
    // the residue key by the manifest filename.
    assert_eq!(outcome(&report, "PreToolUse"), EmitOutcome::MemberReaped);
    assert_eq!(
        outcome(&report, "extraKnownMarketplaces"),
        EmitOutcome::MemberReaped
    );

    // The tally counts both, and the surviving hook is never reaped.
    let rendered = drift::render_emit(&report);
    assert!(rendered.contains("2 member-reaped"), "{rendered}");
    assert!(
        !report
            .entries
            .iter()
            .any(|e| e.name == "SessionStart" && e.outcome == EmitOutcome::MemberReaped),
        "the surviving hook is never reaped: {:?}",
        report.entries
    );

    // The rewritten manifest keeps the survivors and sheds the dropped members.
    let written = fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap();
    assert!(written.contains("SessionStart"), "{written}");
    assert!(written.contains("permissions"), "{written}");
    assert!(!written.contains("PreToolUse"), "{written}");
    assert!(!written.contains("extraKnownMarketplaces"), "{written}");
}

/// `settings_payload` with no `settings` container member: the hooks render into a
/// `settings.json` no member of the program owns, so nothing fingerprints the file.
fn ownerless_settings_payload(events: &[&str]) -> Payload {
    let mut payload = settings_payload(events, &[]);
    payload.members.clear();
    payload
}

/// Append a hand-written matcher group under `hooks.<event>` of the on-disk settings file.
fn hand_add_hook_group(settings: &Path, event: &str) {
    let mut document: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(settings).unwrap()).unwrap();
    let group =
        serde_json::json!({ "hooks": [ { "type": "command", "command": "./my-hook.sh" } ] });
    match document["hooks"][event].as_array_mut() {
        Some(groups) => groups.push(group),
        None => document["hooks"][event] = serde_json::json!([group]),
    }
    fs::write(
        settings,
        serde_json::to_string_pretty(&document).unwrap() + "\n",
    )
    .unwrap();
}

#[test]
fn emit_refuses_a_hand_added_hook_group_in_a_settings_manifest_no_member_owns() {
    for (label, event) in [
        ("ownerless-hand-group-new-event", "Stop"),
        ("ownerless-hand-group-shared-event", "PreToolUse"),
    ] {
        let (harness, into) = workspace(label);
        let payload = ownerless_settings_payload(&["PreToolUse"]);
        drift::emit(&payload, &into, EmitOptions::default()).unwrap();

        let settings = harness.join(".claude").join("settings.json");
        hand_add_hook_group(&settings, event);
        let before = fs::read_to_string(&settings).unwrap();

        for teardown in [false, true] {
            let err = drift::emit(
                &payload,
                &into,
                EmitOptions {
                    teardown,
                    ..EmitOptions::default()
                },
            )
            .unwrap_err();
            let message = format!("{err}");
            assert!(message.contains(&format!("`hooks.{event}`")), "{message}");
            assert!(message.contains(".claude/settings.json"), "{message}");
            assert_eq!(
                fs::read_to_string(&settings).unwrap(),
                before,
                "the refusal lands before a byte is written: the hand group survives"
            );
        }
    }
}

/// Rewrite the command of the first matcher group under `hooks.<event>` on disk.
fn hand_edit_hook_command(settings: &Path, event: &str) {
    let mut document: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(settings).unwrap()).unwrap();
    document["hooks"][event][0]["hooks"][0]["command"] = serde_json::json!("./hand-edited.sh");
    fs::write(
        settings,
        serde_json::to_string_pretty(&document).unwrap() + "\n",
    )
    .unwrap();
}

#[test]
fn emit_refuses_a_hand_edited_registration_entry_in_a_manifest_no_member_owns() {
    let (harness, into) = workspace("ownerless-hand-edited-group");
    let payload = ownerless_settings_payload(&["PreToolUse", "SessionStart"]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let settings = harness.join(".claude").join("settings.json");
    hand_edit_hook_command(&settings, "PreToolUse");
    let before = fs::read_to_string(&settings).unwrap();

    for teardown in [false, true] {
        let err = drift::emit(
            &payload,
            &into,
            EmitOptions {
                teardown,
                ..EmitOptions::default()
            },
        )
        .unwrap_err();
        let message = format!("{err}");
        assert!(message.contains("`hook:PreToolUse`"), "{message}");
        assert!(!message.contains("SessionStart"), "{message}");
        assert!(message.contains(".claude/settings.json"), "{message}");
        assert_eq!(
            fs::read_to_string(&settings).unwrap(),
            before,
            "the refusal lands before a byte is written: the hand edit survives"
        );
    }

    // An unedited re-emit stays clean: the fingerprints match what emit rendered.
    fs::remove_file(&settings).unwrap();
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
}

#[test]
fn a_lock_without_a_registration_entry_hash_reads_it_as_unknown_on_first_emit() {
    let (harness, into) = workspace("ownerless-hashless-lock");
    let payload = ownerless_settings_payload(&["PreToolUse"]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // An older lock: its registration rows carry no entry hash.
    let lock_path = into.join("lock.toml");
    let lock = fs::read_to_string(&lock_path).unwrap();
    let mut doc: toml_edit::DocumentMut = lock.parse().unwrap();
    let rows = doc["declaration"]["registration"]
        .as_array_of_tables_mut()
        .unwrap();
    for row in rows.iter_mut() {
        assert!(row.remove("emit_hash").is_some(), "{lock}");
    }
    let hashless = doc.to_string();
    fs::write(&lock_path, hashless).unwrap();

    let settings = harness.join(".claude").join("settings.json");
    hand_edit_hook_command(&settings, "PreToolUse");

    // Unknown is never drift: the re-emit regenerates the entry and writes its hash.
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let written = fs::read_to_string(&settings).unwrap();
    assert!(!written.contains("hand-edited"), "{written}");
    let relocked = fs::read_to_string(&lock_path).unwrap();
    let doc: toml_edit::DocumentMut = relocked.parse().unwrap();
    let rows = doc["declaration"]["registration"]
        .as_array_of_tables()
        .unwrap();
    assert!(
        rows.iter().all(|row| row.contains_key("emit_hash")),
        "{relocked}"
    );
}

#[test]
fn check_reports_a_hand_edited_ownerless_hook_group_as_drift_on_its_hook_member() {
    let (harness, into) = workspace("ownerless-hand-edited-group-check");
    let payload = ownerless_settings_payload(&["PreToolUse", "SessionStart"]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert!(drift::config_stale(&into, &fresh_clause()).is_empty());

    let settings = harness.join(".claude").join("settings.json");
    hand_edit_hook_command(&settings, "PreToolUse");

    let findings = drift::config_stale(&into, &fresh_clause());
    assert_eq!(findings.len(), 1, "{findings:?}");
    let finding = format!("{:?}", findings[0]);
    assert!(finding.contains("`hook:PreToolUse`"), "{finding}");
    assert!(!finding.contains("SessionStart"), "{finding}");
    assert!(finding.contains(".claude/settings.json"), "{finding}");
    assert!(finding.contains("re-emit"), "{finding}");
}

#[test]
fn check_reads_an_ownerless_registration_row_with_no_hash_as_unknown() {
    let (harness, into) = workspace("ownerless-hashless-lock-check");
    let payload = ownerless_settings_payload(&["PreToolUse"]);
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let lock_path = into.join("lock.toml");
    let mut doc: toml_edit::DocumentMut = fs::read_to_string(&lock_path).unwrap().parse().unwrap();
    for row in doc["declaration"]["registration"]
        .as_array_of_tables_mut()
        .unwrap()
        .iter_mut()
    {
        assert!(row.remove("emit_hash").is_some());
    }
    fs::write(&lock_path, doc.to_string()).unwrap();

    hand_edit_hook_command(&harness.join(".claude").join("settings.json"), "PreToolUse");
    let findings = drift::config_stale(&into, &fresh_clause());
    assert!(findings.is_empty(), "{findings:?}");
}

#[test]
fn emit_still_reaps_the_registration_of_a_removed_hook_member() {
    let (harness, into) = workspace("ownerless-removed-hook-reaps");
    drift::emit(
        &ownerless_settings_payload(&["PreToolUse", "SessionStart"]),
        &into,
        EmitOptions::default(),
    )
    .unwrap();

    // The PreToolUse hook leaves the program; the prior lock's registration row records the
    // group emit wrote, so the group is temper's to reap even with no member owning the file.
    let report = drift::emit(
        &ownerless_settings_payload(&["SessionStart"]),
        &into,
        EmitOptions::default(),
    )
    .unwrap();
    assert_eq!(outcome(&report, "PreToolUse"), EmitOutcome::MemberReaped);
    let written = fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap();
    assert!(!written.contains("PreToolUse"), "{written}");
    assert!(written.contains("SessionStart"), "{written}");
}

#[test]
fn a_total_collection_drop_refuses_at_the_segment_cliff_unless_teardown() {
    let (harness, into) = workspace("manifest-segment-cliff");

    // The live settings.json carries one hook and a residue key.
    let full = settings_payload(
        &["PreToolUse"],
        &[(
            "permissions",
            serde_json::json!({ "allow": ["Bash(cargo build:*)"] }),
        )],
    );
    drift::emit(&full, &into, EmitOptions::default()).unwrap();

    // A payload declaring the hook kind but no hook registration — the whole `hooks`
    // collection would be emptied. A settings residue keeps the manifest represented so the
    // diff still runs.
    let emptied = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                hook_kind_facts(),
                common::kind_facts("settings", ".claude", "settings.json"),
            ],
            settings: vec![SettingsRow {
                manifest: "settings.json".to_string(),
                key: "permissions".to_string(),
                value: serde_json::json!({ "allow": ["Bash(cargo build:*)"] }),
            }],
            ..Default::default()
        },
        members: Vec::new(),
    };

    // The total drop refuses at the segment cliff, naming the collection and the flag.
    let err = drift::emit(&emptied, &into, EmitOptions::default()).unwrap_err();
    let message = format!("{err}");
    assert!(message.contains("hooks"), "{message}");
    assert!(message.contains("teardown"), "{message}");

    // The refusal fires before any byte is written — the live hook is untouched.
    let written = fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap();
    assert!(written.contains("PreToolUse"), "{written}");

    // Spelled `--teardown`: the same emit proceeds and names the cleared hook in the ledger.
    let report = drift::emit(
        &emptied,
        &into,
        EmitOptions {
            teardown: true,
            ..EmitOptions::default()
        },
    )
    .unwrap();
    assert_eq!(outcome(&report, "PreToolUse"), EmitOutcome::MemberReaped);
    assert!(
        !fs::read_to_string(harness.join(".claude").join("settings.json"))
            .unwrap()
            .contains("PreToolUse"),
        "the spelled teardown clears the hook from the manifest"
    );
}

#[test]
fn a_declared_input_rides_the_seam_into_the_lock_and_double_emit_reproduces_it() {
    let (harness, into) = workspace("declared-input-seam");
    // The input is a committed file the member's claim rests on, governed by no kind — it
    // is fingerprinted, never projected and never reaped.
    fs::create_dir_all(harness.join("snapshot")).unwrap();
    fs::write(
        harness.join("snapshot/handler.py"),
        "def handle():\n    pass\n",
    )
    .unwrap();

    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::rule_kind_facts(None, &[])],
            inputs: vec![InputRow {
                member: "rule:rust".to_string(),
                source_path: harness
                    .join("snapshot/handler.py")
                    .to_string_lossy()
                    .into_owned(),
            }],
            ..Default::default()
        },
        members: vec![common::rule_member("rust", None, RUST_BODY)],
    };

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let after_first = common::tree_bytes(&harness);
    let lock_after_first = fs::read(into.join("lock.toml")).unwrap();

    // One `[[declaration.input]]` row, and the seam's own `inputs` family stays inbound —
    // it is lowered to that source dependency, never written back as a declaration row.
    assert_eq!(drift::inputs(&into).unwrap().len(), 1);
    assert!(drift::read_declarations(&into).unwrap().inputs.is_empty());

    // The declaring member's projection carries no byte of the input, so a second emit
    // over the same payload reports Unchanged and reproduces both sides exactly.
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "rust"), EmitOutcome::Unchanged);
    assert_eq!(
        after_first,
        common::tree_bytes(&harness),
        "a declared input moves no byte on either emit"
    );
    assert_eq!(
        lock_after_first,
        fs::read(into.join("lock.toml")).unwrap(),
        "double emit reproduces the input row byte-for-byte"
    );
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
        key_path: CollectionKeyPath::HooksEvent,
        entry_shape: temper::kind::EntryShape::GroupArray {
            member_key: "hooks".to_string(),
            lifted_fields: vec!["matcher".to_string()],
        },
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
            emit_hash: None,
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

// A payload with a skill member that includes another file via composed-prose include.
// The member body carries one include slot (U+0001) that will splice the target's contents.
fn with_composed_prose_include() -> (Payload, String) {
    let include_target_content = "# Included Content\n\nThis is included prose.";
    let member_body = format!(
        "# Coordinate\n\n{}\n\nMore prose after include.",
        "\u{0001}"
    );
    let mut payload = basic_payload(vec![common::skill_member(
        "coordinate",
        "Use when coordinating agents across axes.",
        &member_body,
    )]);
    payload.declarations.includes.push(drift::IncludeRow {
        member: "skill:coordinate".to_string(),
        source_path: "/absolute/path/to/included.md".to_string(),
    });
    (payload, include_target_content.to_string())
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
        rendered_lines: None,
        rendered_chars: None,
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
fn a_dot_slash_spelled_lock_row_names_its_stale_projection_by_the_bare_spelling() {
    // The reap sweep's `./`-row sibling, over the judge that never re-spelled by hand:
    // `config_stale` keys its finding straight off the row's `source_path`, so before the
    // spelling moved to `HarnessRelativePath`'s door this named a pre-0024 row's finding
    // `./CLAUDE.md` where every other judge over the same lock spells it bare.
    let (harness, into) = workspace("config-stale-dot-slash-row");
    const MEMORY_BODY: &str = "# Memory\n\nProject memory.\n";
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::kind_facts("memory", ".", "CLAUDE.md")],
            ..Default::default()
        },
        members: vec![common::payload_member("memory", "root", None, MEMORY_BODY)],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // The older spelling, as a pre-normalization engine wrote it.
    let lock_path = into.join("lock.toml");
    let older_lock = fs::read_to_string(&lock_path).unwrap().replace(
        "source_path = \"CLAUDE.md\"",
        "source_path = \"./CLAUDE.md\"",
    );
    assert!(
        older_lock.contains("source_path = \"./CLAUDE.md\""),
        "the simulated older lock spells the root-`.` projection with a `./` prefix: {older_lock}"
    );
    fs::write(&lock_path, &older_lock).unwrap();

    // A hand edit is all it takes to make the fingerprint mismatch and the finding fire.
    let projection = harness.join("CLAUDE.md");
    let edited = format!(
        "{}\n\nhand-edited.\n",
        fs::read_to_string(&projection).unwrap()
    );
    fs::write(&projection, edited).unwrap();

    let findings = drift::config_stale_from_doc(
        &drift::read_lock_document(&into).unwrap(),
        &into,
        &fresh_clause(),
    );
    assert_eq!(
        findings.len(),
        1,
        "the `./`-spelled row resolves its live projection and reports its one hand edit: {findings:?}"
    );
    assert_eq!(
        findings[0].artifact, "CLAUDE.md",
        "the finding is addressed by the bare spelling, never the row's `./` prefix"
    );
    assert!(
        findings[0].message.contains("`CLAUDE.md`") && !findings[0].message.contains("./CLAUDE.md"),
        "and reads it back the same way: {}",
        findings[0].message
    );
}

#[test]
fn a_lock_emitted_from_a_foreign_cwd_carries_the_rows_of_one_emitted_at_the_harness_root() {
    // The workspace argument's only non-default use is naming a workspace off the cwd, so the cwd an
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
fn a_lock_with_composed_prose_includes_emitted_from_a_foreign_cwd_is_harness_relative() {
    // Extends the foreign-cwd test to cover the one row family the prior test proves nothing
    // about: composed-prose includes. A member's include row carries an absolute source_path
    // resolved by the SDK; emit must spell it harness-relative in the lock, regardless of
    // the cwd. Before the fix, a foreign-cwd invocation would silently bake the raw absolute
    // path instead, degrading downstream to an unowned-file finding.
    let at_root = common::tmpdir("include-cwd-rows-at-root");
    let from_parent = common::tmpdir("include-cwd-rows-from-parent");
    fs::create_dir_all(at_root.join(".temper")).unwrap();
    fs::create_dir_all(from_parent.join(".temper")).unwrap();

    let (payload, include_target) = with_composed_prose_include();

    // Create the include target file under each harness so the same payload can be emitted
    // from either location.
    let included_md_at_root = at_root.join("included.md");
    let included_md_from_parent = from_parent.join("included.md");
    fs::write(&included_md_at_root, &include_target).unwrap();
    fs::write(&included_md_from_parent, &include_target).unwrap();

    let guard = CWD_MUTEX.lock().unwrap();
    let original_cwd = std::env::current_dir().unwrap();

    // Lane A: cwd = the harness root
    std::env::set_current_dir(&at_root).unwrap();
    // Update the payload's include source_path to point at the file we just created
    let mut payload_at_root = payload.clone();
    payload_at_root.declarations.includes[0].source_path =
        included_md_at_root.to_string_lossy().to_string();
    let at_root_report = drift::emit(
        &payload_at_root,
        Path::new(".temper"),
        EmitOptions::default(),
    );

    // Lane B: cwd = the harness's parent
    std::env::set_current_dir(from_parent.parent().unwrap()).unwrap();
    let named_off_cwd = PathBuf::from(from_parent.file_name().unwrap()).join(".temper");
    let mut payload_from_parent = payload;
    payload_from_parent.declarations.includes[0].source_path =
        included_md_from_parent.to_string_lossy().to_string();
    let foreign_report = drift::emit(&payload_from_parent, &named_off_cwd, EmitOptions::default());

    std::env::set_current_dir(&original_cwd).unwrap();
    drop(guard);

    let at_root_report = at_root_report.unwrap();
    let foreign_report = foreign_report.unwrap();

    let at_root_lock = fs::read_to_string(at_root.join(".temper/lock.toml")).unwrap();
    let foreign_lock = fs::read_to_string(from_parent.join(".temper/lock.toml")).unwrap();
    assert_eq!(
        at_root_lock, foreign_lock,
        "composed-prose includes emit byte-identical locks from any cwd"
    );

    assert!(
        at_root_lock.contains("[[declaration.include]]"),
        "the lock carries an include row: {at_root_lock}"
    );

    // The critical assertion: the include row's source_path must be harness-relative, not
    // a raw absolute path baked by a foreign-cwd invocation. Read the row through the
    // engine's own include reader — the same reader the drift comparison and the
    // include-edge lift run — and pin the exact spelling both lanes must carry.
    for (label, workspace) in [
        ("at-root", at_root.join(".temper")),
        ("foreign-cwd", from_parent.join(".temper")),
    ] {
        let includes = drift::includes(&workspace).unwrap();
        assert_eq!(
            includes.len(),
            1,
            "the {label} lane's lock carries exactly the one include row emitted"
        );
        assert_eq!(
            &*includes[0].source_path, "included.md",
            "the {label} lane spells the include target against the harness root, \
             never as the absolute path the SDK resolved"
        );
    }

    // Both emissions should succeed with no reaped entries
    for report in [&at_root_report, &foreign_report] {
        assert!(
            !report
                .entries
                .iter()
                .any(|e| e.outcome == EmitOutcome::Reaped),
            "includes don't cause spurious reaps: {:?}",
            report.entries
        );
    }
}

#[test]
fn an_include_target_above_the_harness_root_is_spelled_the_same_in_two_checkouts() {
    // The sibling case to the foreign-cwd test above: the include's path arrives
    // SDK-resolved and absolute, and a target the harness root is not a prefix of used to
    // ride the lock in that absolute form — so the committed row spelled the one checkout
    // that emitted it (and, on Windows, carried canonicalize's verbatim prefix with it).
    // A `../`-segmented row joins back under whatever root a reader holds: one program,
    // two checkout paths, one set of lock bytes.
    let (payload, include_target) = with_composed_prose_include();
    let mut locks = Vec::new();

    for label in ["above-root-checkout-a", "above-root-checkout-b"] {
        let checkout = common::tmpdir(label);
        // The harness sits two directories down from the checkout root, and the include
        // target beside it at the top — a repo-wide doc a member composes in.
        let into = checkout.join("packages/agent/.temper");
        fs::create_dir_all(&into).unwrap();
        fs::create_dir_all(checkout.join("docs")).unwrap();
        let target = checkout.join("docs/included.md");
        fs::write(&target, &include_target).unwrap();

        let mut payload = payload.clone();
        payload.declarations.includes[0].source_path = target.to_string_lossy().into_owned();
        drift::emit(&payload, &into, EmitOptions::default()).unwrap();

        let includes = drift::includes(&into).unwrap();
        assert_eq!(
            &*includes[0].source_path, "../../docs/included.md",
            "a target above the root is spelled with `..` from the root, never absolutely"
        );
        locks.push(fs::read_to_string(into.join("lock.toml")).unwrap());
    }

    assert_eq!(
        locks[0], locks[1],
        "the same program emitted from two checkout paths writes one lock"
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
import { harness, text } from "@dtmd/temper";
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

export default program;
"#;

/// The common fixture-harness wiring bound to this suite's [`HARNESS_PROGRAM`].
fn wire_sdk_harness(label: &str) -> (PathBuf, PathBuf) {
    common::wire_sdk_harness(label, HARNESS_PROGRAM)
}

/// A fixture SDK program whose one member is a gate hook filling a `require`d
/// requirement — a gate hook is an ordinary hook member, so it speaks every member's
/// `satisfies` edge.
const GATE_HOOK_SATISFIES_PROGRAM: &str = r#"
import { harness, requirement } from "@dtmd/temper";
import { gateHook, hook } from "@dtmd/temper/claude-code";

const program = harness({
  members: [gateHook({ event: "SessionStart", satisfies: ["session-gate"] })],
  require: {
    "session-gate": requirement({ prose: "the gate reports at session open", kind: hook }),
  },
});

export default program;
"#;

#[test]
fn a_gate_hook_declaring_satisfies_fills_its_requirement() {
    let (harness, into) =
        common::wire_sdk_harness("gate-hook-satisfies", GATE_HOOK_SATISFIES_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).unwrap();

    // The gate hook's fill claim reaches the lock as its satisfies row.
    let declarations = drift::read_declarations(&into).unwrap();
    assert_eq!(
        declarations.satisfies,
        vec![drift::SatisfiesRow {
            member: "hook:SessionStart".to_string(),
            requirement: "session-gate".to_string(),
        }],
    );

    // And the gate counts it: the requirement is filled at check.
    let run = common::check_in(&harness, &[], None);
    assert!(
        !run.output.contains("requirement.count"),
        "the gate hook's satisfies fills `session-gate`, got:\n{}",
        run.output
    );
}

/// A fixture SDK program declaring a `require`d requirement carrying a `count`
/// set-scope clause — proving the real SDK emits a requirement's demand as
/// a nested clause row, not a facet field, end to end across the seam.
const REQUIREMENT_CLAUSES_PROGRAM: &str = r#"
import { clause, count, harness, requirement, text } from "@dtmd/temper";
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

export default program;
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

/// A fixture SDK program whose skill description interpolates the current time —
/// authoring code that is a different program on every run, which no render-side
/// double-emit can see: one payload renders the same bytes twice.
const CLOCK_PROGRAM: &str = r#"
import { harness, text } from "@dtmd/temper";
import { skill } from "@dtmd/temper/claude-code";

const program = harness({
  members: [
    skill({
      name: "stamped",
      description: `Use when stamping; generated at ${Date.now()}.`,
      prose: text`# Stamped`,
    }),
  ],
});

export default program;
"#;

#[test]
fn a_program_interpolating_the_current_time_refuses_at_emit_as_nondeterministic() {
    let (harness, into) = common::wire_sdk_harness("clock-program", CLOCK_PROGRAM);
    let before = common::tree_bytes(&harness);

    let err = drift::emit_program(&into, EmitOptions::default())
        .expect_err("a program reading the clock must refuse at emit");

    let drift_err = err
        .downcast_ref::<drift::DriftError>()
        .unwrap_or_else(|| panic!("expected a DriftError, got {err:?}"));
    assert!(
        matches!(drift_err, drift::DriftError::Nondeterministic { path } if path.ends_with("harness.ts")),
        "the refusal is nondeterminism naming the program's entry: {drift_err:?}"
    );
    assert_eq!(
        before,
        common::tree_bytes(&harness),
        "the refusal lands before a byte is written"
    );
}

/// A deterministic program that appends one line to a counter file per run — the
/// count pin on the program-side double-emit: exactly two `node` runs per emit.
const COUNTED_PROGRAM: &str = r#"
import { appendFileSync } from "node:fs";
import { harness, text } from "@dtmd/temper";
import { rule } from "@dtmd/temper/claude-code";

appendFileSync(new URL("./runs.log", import.meta.url), "run\n");

const program = harness({
  members: [rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust` })],
});

export default program;
"#;

#[test]
fn a_deterministic_program_emits_as_today_across_exactly_two_node_runs() {
    let (harness, into) = common::wire_sdk_harness("counted-program", COUNTED_PROGRAM);

    let report =
        drift::emit_program(&into, EmitOptions::default()).expect("a deterministic program emits");

    assert_eq!(outcome(&report, "rust"), EmitOutcome::Emitted);
    assert!(harness.join(".claude/rules/rust.md").is_file());
    assert_eq!(
        fs::read_to_string(into.join("runs.log")).unwrap(),
        "run\nrun\n",
        "emit runs the program exactly twice — once to compile, once to verify"
    );
}

/// A fixture SDK program declaring a custom kind with `content: Layout` (three
/// regions: prose, field, collection) and a second custom kind with
/// `shape: "fields"` + `collectionAddress` — proving the real SDK emits a
/// custom kind's layout content and fields-only shape+collectionAddress
/// facts end to end, reaching `CustomKind::from_kind_fact_row`'s typed
/// fields byte-stably.
const CUSTOM_KIND_CONTENT_PROGRAM: &str = r#"
import { kind, harness, text } from "@dtmd/temper";

// A custom kind with layout content: prose (import), field (slot), and collection (member_kind + key).
const layoutKind = kind<{ intent?: string }>({
  name: "spec",
  locus: { kind: "at", root: "specs", glob: "*.md" },
  unitShape: "file",
  format: "yaml-frontmatter",
  registration: [],
  content: {
    regions: [
      { region: "prose", import: "specs/intent.md" },
      { region: "field", slot: "intent" },
      { region: "collection", memberKind: "invariant", key: "core" },
    ],
  },
});

// A custom kind with fields-only shape and collection address (mimics hook).
const registrationKind = kind({
  name: "custom-hook",
  locus: { kind: "at", root: ".claude", glob: "settings.json" },
  unitShape: "file",
  registration: [{ via: "event", field: "event" }],
  shape: "fields",
  collectionAddress: {
    keyPath: "hooks.<Event>",
    entryShape: "group-array(hooks;matcher)",
  },
});

const program = harness({
  members: [],
  expect: [
    { kind: layoutKind, clauses: [] },
    { kind: registrationKind, clauses: [] },
  ],
});

export default program;
"#;

#[test]
fn emit_program_emits_a_custom_kinds_layout_content_and_fields_shape_end_to_end() {
    let (_harness, into) =
        common::wire_sdk_harness("custom-kind-content", CUSTOM_KIND_CONTENT_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let declarations = drift::read_declarations(&into).unwrap();

    // The layout-content kind's three regions survive the SDK emit → lock round-trip.
    let spec_row = declarations
        .kinds
        .iter()
        .find(|k| k.name == "spec")
        .expect("the layout-content kind row is recorded");
    let spec = temper::kind::CustomKind::from_kind_fact_row(spec_row)
        .expect("the layout-content kind fact decodes through the engine reader");

    assert_eq!(
        spec.content,
        temper::kind::Content::Layout(temper::layout::Layout {
            regions: vec![
                temper::layout::LayoutRegion::Prose {
                    import: Some("specs/intent.md".to_string()),
                },
                temper::layout::LayoutRegion::Field {
                    slot: "intent".to_string(),
                },
                temper::layout::LayoutRegion::Collection {
                    member_kind: "invariant".to_string(),
                    key: Some("core".to_string()),
                },
            ],
        }),
        "the layout kind's declared content reaches the engine CustomKind"
    );

    // The fields-only registration kind's shape and collection address survive the round-trip.
    let hook_row = declarations
        .kinds
        .iter()
        .find(|k| k.name == "custom-hook")
        .expect("the fields-only registration kind row is recorded");
    assert_eq!(
        hook_row.shape.as_deref(),
        Some("fields"),
        "the registration kind carries the fields-only shape column"
    );
    let hook = temper::kind::CustomKind::from_kind_fact_row(hook_row)
        .expect("the fields-only registration kind fact decodes through the engine reader");

    assert_eq!(
        hook.content,
        temper::kind::Content::Fields,
        "the fields-only shape lifts to Content::Fields"
    );
    assert_eq!(
        hook.collection_address,
        Some(temper::kind::CollectionAddress {
            key_path: temper::kind::CollectionKeyPath::HooksEvent,
            entry_shape: temper::kind::EntryShape::GroupArray {
                member_key: "hooks".to_string(),
                lifted_fields: vec!["matcher".to_string()],
            },
        }),
        "the fields-only registration kind's collection address reaches the engine CustomKind"
    );

    // The lock carries both kinds' declaration rows, decorated with their respective facts.
    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        lock.contains(r#"name = "spec""#),
        "the layout-content kind fact is recorded in the lock: {lock}"
    );
    assert!(
        lock.contains(r#"name = "custom-hook""#),
        "the fields-only registration kind fact is recorded in the lock: {lock}"
    );
    assert!(
        lock.contains("region = \"prose\""),
        "the layout regions are recorded in the lock: {lock}"
    );
    assert!(
        lock.contains("shape = \"fields\""),
        "the fields-only shape is recorded in the lock: {lock}"
    );
    assert!(
        lock.contains("collection_address = { key_path = \"hooks.<Event>\""),
        "the collection address is recorded in the lock: {lock}"
    );
}

/// A fixture SDK program that records the entry path Node actually received
/// (`process.argv[1]`) to a sibling file before emitting — the probe for the
/// `\\?\`-verbatim cascade (field report 1): `run_sdk_program` canonicalizes
/// the harness entry, which on Windows yields the verbatim form Node's
/// `resolveMainPath` rejects, so the argv it received must never carry it.
const ARGV_PROBE_PROGRAM: &str = r#"
import { harness, text } from "@dtmd/temper";
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

export default program;
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

/// A program that logs to stdout as its module runs — the stray line lands ahead of the
/// pipe's seam, so the JSON no longer parses: a broken output, never SDK/engine version
/// skew.
const NOISY_STDOUT_PROGRAM: &str = r#"
import { harness } from "@dtmd/temper";
console.log("debug: building the harness");
export default harness({ members: [] });
"#;

/// A program whose module does nothing but default-export its harness — no stdout, no
/// emit call: the engine reads the export through the SDK's pipe.
const DEFAULT_EXPORT_PROGRAM: &str = r#"
import { harness, text } from "@dtmd/temper";
import { rule } from "@dtmd/temper/claude-code";

export default harness({
  members: [rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust conventions` })],
});
"#;

#[test]
fn a_program_that_default_exports_its_harness_emits() {
    let (harness, into) = common::wire_sdk_harness("default-export", DEFAULT_EXPORT_PROGRAM);

    let report = drift::emit_program(&into, EmitOptions::default()).unwrap();

    assert_eq!(outcome(&report, "rust"), EmitOutcome::Emitted);
    let rule = fs::read_to_string(harness.join(".claude/rules/rust.md")).unwrap();
    assert!(rule.contains("# Rust conventions"), "{rule}");
    let (findings, ok) = common::check_harness(&harness);
    assert!(ok, "the emitted program checks clean: {findings:?}");
}

/// A program that composes its harness but exports none — the pipe has nothing to read
/// and refuses, naming the default export the module owes.
const NO_DEFAULT_EXPORT_PROGRAM: &str = r#"
import { harness } from "@dtmd/temper";
export const program = harness({ members: [] });
"#;

#[test]
fn a_program_that_exports_no_default_harness_refuses_naming_the_export() {
    let (_harness, into) = common::wire_sdk_harness("no-default", NO_DEFAULT_EXPORT_PROGRAM);

    let err = drift::emit_program(&into, EmitOptions::default()).unwrap_err();

    let drift_err = err
        .downcast_ref::<drift::DriftError>()
        .unwrap_or_else(|| panic!("expected a DriftError, got {err:?}"));
    assert!(
        matches!(drift_err, drift::DriftError::SdkProgramFailed { stderr, .. }
            if stderr.contains("exports no harness as its default")),
        "{drift_err:?}"
    );
}

#[test]
fn a_payload_that_is_not_json_refuses_as_a_payload_parse_error_not_a_seam_version_mismatch() {
    let (_harness, into) = common::wire_sdk_harness("non-json-payload", NOISY_STDOUT_PROGRAM);

    let err = drift::emit_program(&into, EmitOptions::default()).unwrap_err();

    let drift_err = err
        .downcast_ref::<drift::DriftError>()
        .unwrap_or_else(|| panic!("expected a DriftError, got {err:?}"));
    assert!(
        matches!(drift_err, drift::DriftError::PayloadParse { path, .. } if path.ends_with("harness.ts")),
        "a non-JSON payload must refuse as a payload parse error naming the entry: {drift_err:?}"
    );
    assert!(!format!("{err}").contains("seam version"), "{err}");
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
fn emit_cli_resolves_the_default_relative_workspace_without_doubling_the_path() {
    // `temper emit` with no workspace argument uses the CLI's own relative default
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
        "emit over the default relative workspace must resolve <workspace>/harness.ts without \
         doubling the path: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !harness.join(".temper").join(".temper").exists(),
        "the relative workspace must never double onto itself"
    );

    let rule_path = harness.join(".claude").join("rules").join("rust.md");
    assert!(
        rule_path.is_file(),
        "emit should have run the program and projected the rule at {rule_path:?}"
    );
}

/// `emit` takes its workspace as a positional argument, the way `check` and `bundle` take
/// theirs: one spelling for one concept across the verbs (decision 0114). Run from a cwd
/// that holds no workspace, so only the positional can be what names it.
#[test]
fn emit_takes_its_workspace_positionally() {
    let (harness, workspace) = wire_sdk_harness("positional-workspace");
    let elsewhere = common::tmpdir("positional-workspace-cwd");

    let output = Command::new(BIN)
        .arg("emit")
        .arg(&workspace)
        .current_dir(&elsewhere)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "`temper emit <workspace>` must project the named workspace: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        harness
            .join(".claude")
            .join("rules")
            .join("rust.md")
            .is_file(),
        "emit should have projected the named workspace's rule under its harness root"
    );
    assert!(
        workspace.join("lock.toml").is_file(),
        "emit should have written the named workspace's lock"
    );
    assert!(
        !elsewhere.join(".claude").exists() && !elsewhere.join(".temper").exists(),
        "the cwd names no workspace, so emit writes nothing under it"
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

/// A fixture SDK program declaring two `degree` bounds on one kind, each filtered to
/// its own by-incidence field set — the two-clause corpus the filter made meaningful
/// (`specs/model/contract.md`, "selection"). Both bounds hold of the members below,
/// so the only thing that can refuse this program is its own addressing.
const FILTERED_DEGREE_PROGRAM: &str = r#"
import { clause, degree, harness, kind, text, type KindDefinition } from "@dtmd/temper";

interface Service {
  readonly "depends-on"?: readonly string[];
  readonly "reads-from"?: readonly string[];
}

// A self-edge names its own kind through a getter: `to` is read after `service` exists.
const service: KindDefinition<Service> = kind<Service>({
  name: "service",
  locus: { kind: "at", root: "docs/services", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [{ via: "always" }],
  edgeFields: [
    { field: "depends-on", get to() { return [service] as const; } },
    { field: "reads-from", get to() { return [service] as const; } },
  ],
});

const ledger = service({ name: "ledger", prose: text`# Ledger` });
const billing = service({
  name: "billing",
  "depends-on": ["ledger"],
  "reads-from": ["ledger"],
  prose: text`# Billing`,
});

const program = harness({
  members: [ledger, billing],
  expect: [
    {
      kind: service,
      clauses: [
        clause(degree({ outgoing: { max: 1 }, fields: ["depends-on"] }), { severity: "required" }),
        clause(degree({ outgoing: { max: 2 }, fields: ["reads-from"] }), { severity: "required" }),
      ],
    },
  ],
});

export default program;
"#;

#[test]
fn two_filtered_degree_bounds_on_one_kind_emit_and_check_under_their_own_addresses() {
    let (harness, into) = common::wire_sdk_harness("filtered-degree", FILTERED_DEGREE_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).unwrap();

    // Each bound's filter is what distinguishes it from its sibling, so each label
    // carries it. Pre-fix both rows compiled the bare `service.degree`.
    let declarations = drift::read_declarations(&into).unwrap();
    let labels: Vec<&str> = declarations
        .clauses
        .iter()
        .filter(|row| row.predicate == "degree")
        .filter_map(|row| row.id.as_deref())
        .collect();
    assert_eq!(
        labels,
        vec!["service.degree.depends-on", "service.degree.reads-from"],
        "each filtered bound compiles its own address off the field set it ranges over"
    );

    // And the corpus is admissible: two rows under one label are a malformed lock
    // (`clause.label-collision`), which is what this program used to emit.
    let run = common::check_in(&harness, &["."], Some("github"));
    assert!(
        common::findings_for(&run.findings(), "clause.label-collision").is_empty(),
        "two filtered `degree` bounds are two addressable clauses, not a collision: {}",
        run.output
    );
    assert!(
        run.ok,
        "a corpus whose two filtered `degree` bounds both hold must gate green: {}",
        run.output
    );
}

/// The shipped example harness is a real consumer's program, and nothing ran it:
/// its modules imported two symbols the SDK had since dropped (`maxLines`, retired
/// into `extent(unit, bound)`; `span`, moved off the example's own `kinds.ts` onto
/// the shipped surface), so `node` refused the module graph at link — a break no
/// gate saw, because every other seam case here drives a fixture program this suite
/// authors. This one drives the committed example instead — from a temp copy of the
/// whole `examples/base-harness/` tree, never the checkout: the SDK is vendored at
/// the copy's `node_modules` path a real `npm install` would fill, and the emit is a
/// real one — every byte it writes, projections and lock alike, lands in the copy and
/// none in the repo tree.
///
/// The copy is what keeps the vendoring honest. `vendor_sdk` skips *linking* when a
/// link already exists, and only this case's link used to persist — in the checkout,
/// gitignored and therefore invisible, so every later run reused it and drove `node`
/// over whatever `sdk/dist` happened to hold. A fresh scope per run has no link to
/// skip. The example harness root is self-contained — every lock row's
/// `source_path`/`locus_root` is root-relative (`docs/`, `src`, `.claude/`) — so
/// the copy proves exactly the three claims below that the checkout would.
///
/// `Unchanged` across every projection is three claims at once: the program links,
/// it composes to a payload the engine accepts, and the artifacts committed beside
/// it are byte-current with their authored sources. The lock is the fourth claim, and
/// it is why the emit is not a dry run: `emit_program` writes `lock.toml` only when
/// `dry_run` is off, so a dry run reports projections alone and reproduces not one of
/// the lock's rows — labels, `extent`/`unit`, layout source rows and rendered-line
/// columns all rode uncompared. The copy's lock byte-compares against the committed
/// one instead: `tests/builtin_lock_frozen.rs`'s currency shape, applied to the one
/// other lock this repo commits.
#[test]
fn emit_program_runs_the_shipped_example_harness() {
    let committed = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("base-harness");
    // A link an *earlier* run (or a hand `npm install`) left in the checkout is not
    // this run's doing, and the checkout pin below reads the delta, not the state.
    let residue_before = committed.join(".temper").join("node_modules").exists();

    let harness = shipped_example_copy(&committed);
    let into = harness.join(".temper");

    // `copy_tree` carried the committed lock in, so a compare against the copy would
    // pass on a file this run never touched — vacuous exactly where the bug was. A
    // trailing comment makes the write observable: it is inert to every reader (the
    // prior lock still parses, so the orphan classification and the layer-dropped
    // guard still run against the real committed rows), and `write_rollup` writes the
    // file whole, so the comment survives if and only if emit never wrote.
    let lock = into.join("lock.toml");
    let carried = fs::read_to_string(&lock).expect("the copy carries the committed lock");
    fs::write(
        &lock,
        format!("{carried}# emit must rewrite this file whole\n"),
    )
    .unwrap();

    let report = drift::emit_program(&into, EmitOptions::default())
        .expect("the shipped example's program must link, run, and compile");

    assert!(
        !report.entries.is_empty(),
        "the example projects artifacts; an empty report means the walk found none"
    );
    for entry in &report.entries {
        assert_eq!(
            entry.outcome,
            EmitOutcome::Unchanged,
            "the example's committed artifacts are byte-current: {} {} reported {:?}. \
             That emit needs the SDK resolvable from the example's own root first — \
             vendor it the way `vendor_sdk` does, `ln -s $PWD/sdk \
             examples/base-harness/.temper/node_modules/@dtmd/temper`, and link its \
             compiler the way `link_typescript_bin` does (both gitignored, never \
             committed) — then re-run `cargo run -- emit examples/base-harness/.temper` \
             and commit what moves.",
            entry.kind,
            entry.name,
            entry.outcome
        );
        assert!(
            entry.source_path.starts_with(&harness),
            "the emit runs from the copy: {} {} targeted {}, outside {}",
            entry.kind,
            entry.name,
            entry.source_path.display(),
            harness.display()
        );
    }

    // The lock no projection row speaks for. Every path it records is root-relative
    // (`source_path`/`locus_root` hold `.claude/rules/conduct.md`, `docs`, `src`, `.`),
    // so the copy's bytes and the checkout's are the same bytes — or the committed file
    // is stale. The engine stamp is compared like any other byte, deliberately: an
    // adopter's committed lock takes this same one-line diff on every upgrade, and a
    // normalization here would hide exactly the churn the release bump owes them.
    // The vendored SDK's module rows name this checkout, so they are held out of both
    // sides; the example's own modules compare like any other row.
    let derived = without_module_rows(
        &fs::read_to_string(&lock).expect("emit writes the example's lock into the copy"),
        is_vendored_sdk,
    );
    assert_eq!(
        derived, carried,
        "examples/base-harness/.temper/lock.toml has drifted from what the example's own \
         program emits — a version bump alone moves its `engine` stamp, and regenerating \
         is the whole remedy. Vendor the SDK into the example's root the way `vendor_sdk` does \
         (`ln -s $PWD/sdk examples/base-harness/.temper/node_modules/@dtmd/temper` — \
         gitignored, never committed; without it the program cannot resolve \
         `@dtmd/temper` and the emit fails to link), then re-run `cargo run -- emit \
         examples/base-harness/.temper` and commit what moves — never hand-edit \
         a row."
    );

    // The regression, pinned from both ends. The vendoring landed in the copy —
    // pre-fix it landed in the checkout, so this link is absent there — and the run
    // created nothing under the committed tree.
    assert!(
        into.join("node_modules")
            .join("@dtmd")
            .join("temper")
            .exists(),
        "the SDK is vendored into the copy's node_modules, never the checkout's"
    );
    assert_eq!(
        committed.join(".temper").join("node_modules").exists(),
        residue_before,
        "the example emit runs from a copy: this run may not create \
         examples/base-harness/.temper/node_modules in the checkout"
    );
}

/// A temp copy of the committed example harness with the SDK vendored where a real
/// `npm install` would put it — the scope every run over the example drives, so no
/// run writes into the checkout.
fn shipped_example_copy(committed: &Path) -> PathBuf {
    let harness = common::tmpdir("shipped-example");
    common::copy_tree(committed, &harness);
    common::vendor_sdk(&harness.join(".temper").join("node_modules").join("@dtmd"));
    // The example declares its typecheck over its own `typescript` devDependency, which
    // a real `npm install` lays out beside the SDK.
    #[cfg(unix)]
    common::link_typescript_bin(&harness.join(".temper"));
    harness
}

/// The example is what an adopter copies, so it holds the bar it teaches: its own
/// gate, advisories denied, from its own root — the gate hooks installed as gate
/// members and every document at a governed locus one its program declares.
#[test]
fn the_shipped_example_harness_passes_its_own_check_with_advisories_denied() {
    let committed = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("base-harness");
    let harness = shipped_example_copy(&committed);

    let run = common::check_in(&harness, &[".", "--deny-advisories"], None);
    assert!(
        run.ok,
        "examples/base-harness must pass `temper check . --deny-advisories` from its own root:\n{}",
        run.output
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

#[test]
fn crlf_checkout_reads_clean_for_config_stale() {
    let (harness, into) = workspace("crlf-config-stale");
    const BODY: &str = "# Skill\n\nUse this skill.\n";

    // Emit with LF content, creating a lock with LF-based hashes.
    let payload = basic_payload(vec![common::skill_member("demo", "demo skill", BODY)]);
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "demo"), EmitOutcome::Emitted);

    let skill_path = harness
        .join(".claude")
        .join("skills")
        .join("demo")
        .join("SKILL.md");

    // Read the emitted LF content.
    let lf_content = fs::read_to_string(&skill_path).unwrap();

    // CRLF-only change: rewrite the skill with CRLF but same content.
    let crlf_content = lf_content.replace('\n', "\r\n");
    fs::write(&skill_path, &crlf_content).unwrap();

    // The projection judge must find no staleness (CRLF is canonicalized away). It reads
    // its rows under the workspace dir — the harness dir would resolve them off-tree and
    // read clean whatever the bytes.
    let lock = drift::read_lock_document(&into).unwrap();
    let stale_findings = drift::config_stale_from_doc(&lock, &into, &fresh_clause());
    assert!(
        stale_findings.is_empty(),
        "CRLF-only diff should read clean: {:?}",
        stale_findings
    );

    // Contrast: a real content edit under the same CRLF endings does fire, so the clean
    // verdict above was judged over the emitted row, not over nothing.
    fs::write(
        &skill_path,
        crlf_content.replace("Use this skill.", "Edited."),
    )
    .unwrap();
    let edited = drift::config_stale_from_doc(&lock, &into, &fresh_clause());
    assert_eq!(
        edited.len(),
        1,
        "a content edit must read stale: {edited:?}"
    );
}

#[test]
fn crlf_orphan_classifies_as_reaped_not_drift() {
    use temper::drift::EmitOutcome;

    let (harness, into) = workspace("crlf-orphan");
    const BODY: &str = "# Rule\n\nRule body.\n";

    // Emit a rule, creating a lock.
    let payload = basic_payload(vec![common::rule_member("demo", None, BODY)]);
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(outcome(&report, "demo"), EmitOutcome::Emitted);

    let rule_path = harness.join(".claude").join("rules").join("demo.md");

    // Rewrite the rule to CRLF (same content, including the banner emit placed).
    let banner_and_body = format!("{}\n\n{}", placement::BANNER, BODY);
    fs::write(&rule_path, banner_and_body.replace('\n', "\r\n")).unwrap();

    // Re-emit with the member removed (orphan scenario).
    let orphan_payload = basic_payload(vec![]);
    let report = drift::emit(&orphan_payload, &into, EmitOptions::default()).unwrap();

    // The orphan should classify as Reaped (not OrphanDrift) because the CRLF-rewritten
    // file canonicalizes to the lock's LF-based hash.
    assert_eq!(outcome(&report, "demo"), EmitOutcome::Reaped);

    // Verify the file is actually gone (reaped).
    assert!(!rule_path.exists(), "reaped file should be deleted");

    // Now test that a genuine edit causes OrphanDrift classification.
    let (harness2, into2) = workspace("crlf-orphan-drift");
    let payload2 = basic_payload(vec![common::rule_member("test", None, BODY)]);
    drift::emit(&payload2, &into2, EmitOptions::default()).unwrap();

    let rule_path2 = harness2.join(".claude").join("rules").join("test.md");

    // Edit the file (non-EOL change).
    fs::write(&rule_path2, "# Rule\n\nEdited content.\n").unwrap();

    // Re-emit with the member removed.
    let orphan_payload2 = basic_payload(vec![]);
    let report2 = drift::emit(&orphan_payload2, &into2, EmitOptions::default()).unwrap();

    // Now it should be OrphanDrift because of the genuine edit.
    assert_eq!(outcome(&report2, "test"), EmitOutcome::OrphanDrift);

    // Verify the file still exists (not reaped).
    assert!(
        rule_path2.exists(),
        "orphan-drift file should not be reaped"
    );
}

/// A fixture SDK program whose `CLAUDE` memory carries one `citation` value per
/// spelling of the same dangling `rule` target — bare and kind-qualified — through a
/// one-element `to` set; `rust` composes so the `rule` kind is in play.
const DANGLING_EDGE_SPELLINGS_PROGRAM: &str = r#"
import { blocks, embeddedMemberValue, harness, kind, text } from "@dtmd/temper";
import { memory, rule } from "@dtmd/temper/claude-code";

const citation = kind({
  name: "citation",
  locus: { kind: "embedded" },
  unitShape: "file",
  edgeFields: [{ field: "source", to: [rule] }],
});

const program = harness({
  members: [
    rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust` }),
    memory({
      name: "CLAUDE",
      prose: blocks(
        embeddedMemberValue({ kind: citation, key: "bare", leaves: { source: "ghost" } }),
        embeddedMemberValue({ kind: citation, key: "qualified", leaves: { source: "rule:phantom" } }),
      ),
    }),
  ],
  admit: [{ host: memory, admits: [citation] }],
});

export default program;
"#;

#[test]
fn a_bare_edge_target_in_a_one_element_set_defers_to_check_like_its_qualified_spelling() {
    let (harness, into) =
        common::wire_sdk_harness("bare-edge-defers", DANGLING_EDGE_SPELLINGS_PROGRAM);

    drift::emit_program(&into, EmitOptions::default())
        .expect("both spellings of a dangling one-element target defer to check");

    let run = common::check_in(&harness, &[], None);
    assert!(run.output.contains("root.resolves"), "got:\n{}", run.output);
    for (value, target) in [("bare", "ghost"), ("qualified", "rule:phantom")] {
        let finding = format!(
            "`memory:CLAUDE/citation/{value}` `source` routes to `{target}`, which resolves to no `rule` artifact"
        );
        assert!(
            run.output.contains(&finding),
            "check owns the route verdict for `{target}`, got:\n{}",
            run.output
        );
    }
}

#[test]
fn a_bare_edge_target_in_a_multi_element_set_still_refuses_at_emit() {
    // A multi-element set requires the kind-qualified address: a bare name names no
    // kind, so nothing on disk could resolve it and emit refuses rather than defer.
    let program = DANGLING_EDGE_SPELLINGS_PROGRAM
        .replace(r#"to: [rule]"#, r#"to: [rule, skill]"#)
        .replace(
            r#"import { memory, rule } from"#,
            r#"import { memory, rule, skill } from"#,
        )
        .replace(
            r#"embeddedMemberValue({ kind: citation, key: "qualified", leaves: { source: "rule:phantom" } }),"#,
            "",
        );
    let (_harness, into) = common::wire_sdk_harness("bare-edge-multi-refuses", &program);

    let err = drift::emit_program(&into, EmitOptions::default())
        .expect_err("a bare name in a multi-element set refuses at emit");
    assert!(
        format!("{err:?}").contains("names `ghost`, which resolves to no composed member"),
        "the refusal names the bare target: {err:?}"
    );
}

/// A fixture program declaring its typecheck: the TypeScript compiler as a composed
/// `{ package, bin }` tool, over the workspace's own `tsconfig.json`. `BODY` is the slot a
/// case writes a statement into — a type error, or nothing.
const TYPECHECKED_PROGRAM: &str = r#"
import { harness, input, text, tool } from "@dtmd/temper";
import { rule } from "@dtmd/temper/claude-code";


BODY

const tsc = tool({ package: "typescript", bin: "tsc" });

const program = harness({
  members: [
    rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`
      # Rust conventions
    ` }),
    tsc,
  ],
  typecheck: { tool: tsc, project: input(import.meta.url, "./tsconfig.json") },
});

export default program;
"#;

#[test]
fn emit_refuses_a_program_that_declares_no_typecheck() {
    // Written past `wire_sdk_harness`'s splice: this program is the one that declares none.
    let harness = common::tmpdir("typecheck-undeclared");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let program = TYPECHECKED_PROGRAM
        .replace("BODY", "")
        .replace("    tsc,\n  ],", "  ],")
        .replace(
            "  typecheck: { tool: tsc, project: input(import.meta.url, \"./tsconfig.json\") },\n",
            "",
        );
    assert!(!program.contains("typecheck:"), "the fixture declares none");
    fs::write(into.join("harness.ts"), program).unwrap();
    common::vendor_sdk(&into.join("node_modules").join("@dtmd"));

    let err = drift::emit_program(&into, EmitOptions::default())
        .expect_err("a program declaring no typecheck refuses");
    assert!(
        matches!(
            err.downcast_ref::<drift::DriftError>(),
            Some(drift::DriftError::TypecheckUndeclared { .. })
        ),
        "the refusal is the undeclared typecheck: {err:?}"
    );
    let rendered = format!("{err:?}");
    assert!(
        rendered.contains("declares no typecheck") && rendered.contains("typecheck: {"),
        "the refusal names the declaration it wants: {rendered}"
    );
    assert!(!into.join("lock.toml").exists(), "no lock is written");
    assert!(
        !harness.join(".claude/rules/rust.md").exists(),
        "no projection is written"
    );
}

#[cfg(unix)]
#[test]
fn emit_refuses_a_program_whose_typecheck_reports_a_type_error() {
    let program = TYPECHECKED_PROGRAM.replace("BODY", "const width: string = 80;");
    let (harness, into) = common::wire_sdk_harness("typecheck-type-error", &program);
    common::wire_typescript_compiler(&into);

    let err = drift::emit_program(&into, EmitOptions::default())
        .expect_err("a type error refuses the emit");
    let drift_err = err
        .downcast_ref::<drift::DriftError>()
        .expect("a typed drift error");
    let drift::DriftError::TypecheckFailed {
        tool,
        project,
        diagnostics,
    } = drift_err
    else {
        panic!("expected TypecheckFailed, got {drift_err:?}");
    };
    assert_eq!(tool, "root/tool/tsc");
    assert_eq!(project, ".temper/tsconfig.json");
    // The compiler's own diagnostic, verbatim: TS2322 at the offending line.
    assert!(
        diagnostics.contains("harness.ts") && diagnostics.contains("TS2322"),
        "the refusal carries tsc's diagnostics: {diagnostics}"
    );
    assert!(
        format!("{err}").contains("TS2322"),
        "the rendered refusal names the diagnostics: {err}"
    );
    // Refused before a byte is written: no lock, no projection.
    assert!(!into.join("lock.toml").exists(), "no lock is written");
    assert!(
        !harness.join(".claude/rules/rust.md").exists(),
        "no projection is written"
    );
}

#[cfg(unix)]
#[test]
fn a_relative_workspace_path_runs_the_typecheck_from_the_workspace_bin() {
    // Under this process's working directory, so the workspace spells downward from it,
    // as `temper emit examples/base-harness/.temper` spells it from the checkout.
    // Held for the whole run: a chdir'ing case elsewhere would move the cwd under it.
    let _cwd = CWD_MUTEX.lock().unwrap();
    let cwd = std::env::current_dir().unwrap();
    let harness = tempfile::Builder::new()
        .prefix("typecheck-relative-into")
        .tempdir_in(env!("CARGO_TARGET_TMPDIR"))
        .unwrap();
    let into = harness.path().join(".temper");
    fs::create_dir_all(&into).unwrap();
    let program = TYPECHECKED_PROGRAM.replace("BODY", "const width: string = 80;");
    fs::write(into.join("harness.ts"), program).unwrap();
    common::vendor_sdk(&into.join("node_modules").join("@dtmd"));
    common::wire_typescript_compiler(&into);
    let relative = into
        .strip_prefix(&cwd)
        .expect("the target tmpdir sits under the crate root");

    let err = drift::emit_program(relative, EmitOptions::default())
        .expect_err("the compiler runs and reports the type error");
    let drift_err = err
        .downcast_ref::<drift::DriftError>()
        .expect("a typed drift error");
    assert!(
        matches!(drift_err, drift::DriftError::TypecheckFailed { diagnostics, .. } if diagnostics.contains("TS2322")),
        "the anchored `.bin/tsc` is found from a relative workspace path: {drift_err:?}"
    );
}

#[cfg(unix)]
#[test]
fn the_typecheck_project_file_is_a_declared_input() {
    let program = TYPECHECKED_PROGRAM.replace("BODY", "");
    let (_harness, into) = common::wire_sdk_harness("typecheck-project-input", &program);
    common::wire_typescript_compiler(&into);

    drift::emit_program(&into, EmitOptions::default()).expect("a clean program emits");

    let inputs = drift::inputs(&into).unwrap();
    let project = inputs
        .iter()
        .find(|row| row.source_path.as_ref() as &str == ".temper/tsconfig.json")
        .unwrap_or_else(|| panic!("the project rides the lock as an input: {inputs:?}"));
    assert_eq!(
        project.member, "root",
        "the root member owns the project input"
    );
    assert_eq!(
        project.import_hash,
        format!("{:x}", Sha256::digest(common::TYPECHECK_PROJECT.as_bytes())),
        "the project's bytes are fingerprinted"
    );
}

#[test]
fn a_typecheck_naming_a_tool_the_harness_does_not_compose_refuses() {
    // The tool is constructed but left out of `members`: the reference would point at
    // nothing the lock carries, so the SDK refuses before the engine runs anything.
    let program = TYPECHECKED_PROGRAM
        .replace("BODY", "")
        .replace("    tsc,\n  ],", "  ],");
    let (_harness, into) = common::wire_sdk_harness("typecheck-uncomposed-tool", &program);

    let err = drift::emit_program(&into, EmitOptions::default())
        .expect_err("an uncomposed typecheck tool refuses");
    assert!(
        format!("{err}")
            .contains("`typecheck`: `tool` names tool `tsc`, which the harness does not compose"),
        "the refusal names the uncomposed tool: {err}"
    );
}

/// A program whose run fails outright: `node` throws before a payload is printed.
const UNRUNNABLE_PROGRAM: &str = r#"
import { harness } from "@dtmd/temper";
throw new Error("the program cannot run");
export default harness({ members: [] });
"#;

/// A plain fixture program — one rule — whose emit the [`common::typechecked`] splice
/// types through the [`common::FIXTURE_TSC`] stub.
const PLAIN_PROGRAM: &str = r#"
import { harness, text } from "@dtmd/temper";
import { rule } from "@dtmd/temper/claude-code";
const program = harness({
  members: [
    rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`
      # Rust conventions
    ` }),
  ],
});
export default program;
"#;

/// Rewrite the fixture typecheck's stub as a compiler reporting `diagnostic` and failing.
fn fail_fixture_compiler(harness: &Path, diagnostic: &str) {
    let path = harness.join(common::FIXTURE_TSC);
    if cfg!(windows) {
        fs::write(&path, format!("@echo {diagnostic}\r\n@exit /b 1\r\n")).unwrap();
    } else {
        fs::write(&path, format!("#!/bin/sh\necho '{diagnostic}'\nexit 1\n")).unwrap();
    }
}

#[test]
fn emit_runs_the_typecheck_the_last_lock_records_before_running_the_program() {
    let (harness, into) = common::wire_sdk_harness("typecheck-locked-first", PLAIN_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).expect("a clean program emits");
    let lock_before = fs::read(into.join("lock.toml")).unwrap();

    // The lock now records the fixture typecheck. Break the program so it cannot run at
    // all, and the compiler so it reports a type error: the locked typecheck runs first,
    // so the author receives the compiler's diagnostics, not the program's crash.
    fs::write(into.join("harness.ts"), UNRUNNABLE_PROGRAM).unwrap();
    fail_fixture_compiler(&harness, "harness.ts(4,1): error TS9999: locked typecheck");

    let err = drift::emit_program(&into, EmitOptions::default())
        .expect_err("the locked typecheck refuses");
    let Some(drift::DriftError::TypecheckFailed {
        tool,
        project,
        diagnostics,
    }) = err.downcast_ref::<drift::DriftError>()
    else {
        panic!("expected the locked typecheck's refusal, got {err:?}");
    };
    assert_eq!(tool, common::FIXTURE_TSC_ADDRESS);
    assert_eq!(project, ".temper/tsconfig.json");
    assert!(
        diagnostics.contains("TS9999: locked typecheck"),
        "the refusal carries the compiler's diagnostics: {diagnostics}"
    );
    assert_eq!(
        fs::read(into.join("lock.toml")).unwrap(),
        lock_before,
        "the refused emit writes no byte of the lock"
    );
}

#[test]
fn a_first_emit_with_no_lock_runs_the_declared_typecheck_after_the_program() {
    // No lock: the program runs first, so an unrunnable program refuses as itself even
    // with a failing compiler declared beside it.
    let (harness, into) = common::wire_sdk_harness("typecheck-first-emit-order", PLAIN_PROGRAM);
    fail_fixture_compiler(&harness, "harness.ts(1,1): error TS9999: first emit");
    fs::write(
        into.join("harness.ts"),
        common::typechecked(UNRUNNABLE_PROGRAM),
    )
    .unwrap();
    assert!(!into.join("lock.toml").exists(), "a first emit has no lock");
    let err = drift::emit_program(&into, EmitOptions::default())
        .expect_err("the unrunnable program refuses");
    assert!(
        matches!(
            err.downcast_ref::<drift::DriftError>(),
            Some(drift::DriftError::SdkProgramFailed { .. })
        ),
        "with no lock the program runs before the typecheck: {err:?}"
    );

    // A runnable program then reaches the payload's own declared typecheck, run right
    // after the program.
    fs::write(into.join("harness.ts"), common::typechecked(PLAIN_PROGRAM)).unwrap();
    let err = drift::emit_program(&into, EmitOptions::default())
        .expect_err("the payload's typecheck refuses");
    assert!(
        matches!(
            err.downcast_ref::<drift::DriftError>(),
            Some(drift::DriftError::TypecheckFailed { diagnostics, .. })
                if diagnostics.contains("TS9999: first emit")
        ),
        "the payload's declared typecheck runs after the program: {err:?}"
    );
    assert!(!into.join("lock.toml").exists(), "no lock is written");
}

/// A custom directory-unit host at `domains/*/DOMAIN.md` beside the built-in `memory` row.
fn domain_host_and_memory_kinds() -> Vec<KindFactRow> {
    let memory = temper::builtin_lock::declarations()
        .kinds
        .iter()
        .find(|row| row.name == "memory")
        .expect("the built-in lock declares memory")
        .clone();
    vec![
        KindFactRow {
            unit_shape: Some("directory".to_string()),
            templates: vec![drift::TemplateRow {
                kind: "supporting-doc".to_string(),
                path: Some("*.md".to_string()),
            }],
            ..common::kind_facts("domain", "domains", "*/DOMAIN.md")
        },
        KindFactRow {
            ceded: Vec::new(),
            ..memory
        },
    ]
}

#[test]
fn emit_refuses_a_memory_placement_inside_a_custom_host_unit() {
    let (harness, into) = workspace("memory-inside-custom-host-unit");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: domain_host_and_memory_kinds(),
            ..Default::default()
        },
        members: vec![located_member(
            "memory",
            "domains:billing:CLAUDE",
            "domains/billing/CLAUDE.md",
        )],
    };

    // `check` reads `domains/billing/CLAUDE.md` as the `domain` host's file, so a memory
    // member written there would never read back as memory.
    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let message = format!("{err}");
    assert!(
        message.contains("`domains/billing/CLAUDE.md`") && message.contains("`domain`"),
        "{message}"
    );
    assert!(!harness.join("domains/billing/CLAUDE.md").exists());

    // Outside every host's unit, the same kind places as it always has.
    let payload = Payload {
        members: vec![located_member(
            "memory",
            "apps:web:CLAUDE",
            "apps/web/CLAUDE.md",
        )],
        ..payload
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert!(harness.join("apps/web/CLAUDE.md").is_file());
}

#[test]
fn emit_refuses_a_placement_inside_a_subkind_locus() {
    let (harness, into) = workspace("placement-inside-subkind-locus");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                flat_file_kind_facts("note", "notes", "*.md"),
                KindFactRow {
                    base: Some("note".to_string()),
                    ..common::kind_facts("draft", "notes", "draft-*.md")
                },
            ],
            ..Default::default()
        },
        members: vec![plain_member("note", "draft-plan")],
    };

    // A document's kind is its position: `notes/draft-plan.md` is the `draft` subkind's.
    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let message = format!("{err}");
    assert!(
        message.contains("subkind `draft`") && message.contains("`notes/draft-plan.md`"),
        "{message}"
    );
    assert!(!harness.join("notes/draft-plan.md").exists());
}

/// The built-in lock's own row for `kind`.
fn builtin_row(kind: &str) -> KindFactRow {
    temper::builtin_lock::declarations()
        .kinds
        .iter()
        .find(|row| row.name == kind)
        .unwrap_or_else(|| panic!("the built-in lock declares `{kind}`"))
        .clone()
}

#[test]
fn emit_places_a_member_at_its_declared_location_rather_than_a_path_parsed_from_its_name() {
    let (harness, into) = workspace("placement-from-location");
    // A field-derived leaf is opaque: `Teaching Mode` names the style, `teach.md` holds it,
    // and only the declared location says so.
    let member = PayloadMember {
        location: Some("apps/.claude/output-styles/teach.md".to_string()),
        fields: vec![("name".to_string(), "Teaching Mode".into())],
        ..plain_member("output-style", "apps:Teaching Mode")
    };
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![builtin_row("output-style")],
            ..Default::default()
        },
        members: vec![member],
    };

    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert!(
        harness
            .join("apps/.claude/output-styles/teach.md")
            .is_file()
    );
    assert!(
        !harness
            .join("apps/.claude/output-styles/Teaching Mode.md")
            .exists()
    );
    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        lock.contains("location = \"apps/.claude/output-styles/teach.md\""),
        "the lock row carries the placement: {lock}"
    );
}

#[test]
fn a_member_no_template_can_place_whole_without_a_declared_location_is_refused() {
    let (harness, into) = workspace("placement-undeclared");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![builtin_row("memory")],
            ..Default::default()
        },
        members: vec![plain_member("memory", "apps:web:CLAUDE")],
    };

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    assert!(
        matches!(
            err.downcast_ref::<drift::DriftError>(),
            Some(drift::DriftError::UnplacedMember { member, .. }) if member == "memory:apps:web:CLAUDE"
        ),
        "{err:?}"
    );
    assert!(!harness.join("apps/web/CLAUDE.md").exists());
}

#[test]
fn emit_refuses_a_location_that_disagrees_with_the_names_prefix() {
    let (_harness, into) = workspace("placement-disagrees");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![builtin_row("memory")],
            ..Default::default()
        },
        members: vec![located_member(
            "memory",
            "apps:web:CLAUDE",
            "apps/api/CLAUDE.md",
        )],
    };

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    assert!(
        matches!(
            err.downcast_ref::<drift::DriftError>(),
            Some(drift::DriftError::MemberLocation { detail, .. }) if detail.contains("apps:api:CLAUDE")
        ),
        "{err:?}"
    );
}
