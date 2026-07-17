//! `temper tap` — the advisory telemetry recorder and its versioned JSONL
//! event record. The tap reads a hook payload from stdin and appends one
//! machine-written record (identity + minimal discriminant, never prose) to the
//! per-machine, uncommitted log; the record round-trips, appends interleave
//! without rewriting the file, and an older-version record reads tolerated.

use std::io::Write;
use std::process::Command;

use temper::tap::{self, LogReadout, TAP_RECORD_VERSION, TapEvent, TapRecord};

mod common;

const BIN: &str = env!("CARGO_BIN_EXE_temper");

/// Drive `temper tap <root>` with `payload` on stdin, returning the process exit
/// success and the log's readout under `<root>/.temper`.
fn tap(root: &std::path::Path, payload: &str) -> (bool, LogReadout) {
    let mut child = Command::new(BIN)
        .arg("tap")
        .arg(root)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();
    let ok = child.wait_with_output().unwrap().status.success();
    let readout = tap::read_log(&root.join(".temper")).unwrap();
    (ok, readout)
}

#[test]
fn tap_appends_one_record_carrying_identity_and_discriminant_and_no_prose() {
    // A sample `InstructionsLoaded` payload carries the loaded file's path, the
    // load reason, the session id — and the file's whole `content`, prose the
    // record must never capture. The tap appends exactly one record naming the
    // identity + discriminant, exits zero, and the log carries none of the prose.
    let root = common::tmpdir("tap-record");
    let content = "SECRET PROSE THAT MUST NEVER BE RECORDED";
    let payload = format!(
        "{{\"session_id\":\"sess-1\",\"hook_event_name\":\"InstructionsLoaded\",\
         \"file_path\":\".claude/rules/rust.md\",\"load_reason\":\"session_start\",\
         \"content\":\"{content}\"}}"
    );

    let (ok, readout) = tap(&root, &payload);
    assert!(ok, "a tap is advisory and always exits zero");
    assert_eq!(readout.records.len(), 1, "one payload appends one record");
    assert_eq!(readout.older_version, 0, "the current tap wrote it");

    let record = &readout.records[0];
    assert_eq!(record.version, TAP_RECORD_VERSION);
    assert_eq!(record.session, "sess-1");
    assert_eq!(record.event, TapEvent::InstructionsLoaded);
    assert_eq!(record.identity, ".claude/rules/rust.md");
    assert_eq!(record.reason.as_deref(), Some("session_start"));

    // No prose reaches the log — the raw bytes carry neither the file content nor
    // any prose field name.
    let raw = std::fs::read_to_string(root.join(".temper").join("tap.jsonl")).unwrap();
    assert!(!raw.contains(content), "the file's prose is never recorded");
    assert!(!raw.contains("content"), "no prose field is recorded");
}

#[test]
fn tap_extracts_each_lifecycle_event() {
    // Each recognized lifecycle event names its own identity: a skill invocation
    // rides `PostToolUse` (tool_name="Skill", skill under tool_input), a plain
    // tool use names the tool, a command expansion names the command — none
    // capturing the payload's prose (tool_response, expanded_prompt).
    let root = common::tmpdir("tap-events");

    let skill = "{\"session_id\":\"s\",\"hook_event_name\":\"PostToolUse\",\
         \"tool_name\":\"Skill\",\"tool_input\":{\"skill\":\"capture-friction\"},\
         \"tool_response\":\"OUTPUT PROSE\"}";
    let (_, readout) = tap(&root, skill);
    assert_eq!(readout.records[0].event, TapEvent::SkillInvoked);
    assert_eq!(readout.records[0].identity, "capture-friction");

    let tool = "{\"session_id\":\"s\",\"hook_event_name\":\"PostToolUse\",\
         \"tool_name\":\"Bash\",\"tool_input\":{\"command\":\"ls\"},\
         \"tool_response\":\"OUTPUT PROSE\"}";
    let (_, readout) = tap(&root, tool);
    assert_eq!(readout.records[1].event, TapEvent::ToolUse);
    assert_eq!(readout.records[1].identity, "Bash");

    let expand = "{\"session_id\":\"s\",\"hook_event_name\":\"UserPromptExpansion\",\
         \"command_name\":\"grilling\",\"expanded_prompt\":\"PROMPT PROSE\"}";
    let (_, readout) = tap(&root, expand);
    assert_eq!(readout.records[2].event, TapEvent::UserPromptExpansion);
    assert_eq!(readout.records[2].identity, "grilling");

    let raw = std::fs::read_to_string(root.join(".temper").join("tap.jsonl")).unwrap();
    assert!(
        !raw.contains("PROSE"),
        "no prose from any event reaches the log"
    );
}

#[test]
fn an_unrecognized_payload_records_nothing_and_exits_zero() {
    // A payload naming no recognized event, and one that is not even JSON, both
    // record nothing — advisory, so an unrecognized input is inert, never a fail.
    let root = common::tmpdir("tap-noop");

    let (ok, readout) = tap(
        &root,
        "{\"session_id\":\"s\",\"hook_event_name\":\"Stop\",\"last_assistant_message\":\"hi\"}",
    );
    assert!(ok);
    assert_eq!(
        readout.records.len(),
        0,
        "an unrecognized event records nothing"
    );

    let (ok, readout) = tap(&root, "not json at all");
    assert!(ok, "a non-JSON payload never gates");
    assert_eq!(readout.records.len(), 0);
}

#[test]
fn a_record_round_trips_through_append_and_read() {
    // The record the writer serializes reads back byte-for-byte identical at the
    // current version — the writer and the version-tolerant reader agree.
    let root = common::tmpdir("tap-roundtrip");
    let workspace = root.join(".temper");
    std::fs::create_dir_all(&workspace).unwrap();

    let record = TapRecord {
        version: TAP_RECORD_VERSION,
        session: "sess".to_string(),
        event: TapEvent::SkillInvoked,
        identity: "verify".to_string(),
        reason: None,
    };
    tap::append(&workspace, &record).unwrap();

    let readout = tap::read_log(&workspace).unwrap();
    assert_eq!(readout.records, vec![record]);
    assert_eq!(readout.older_version, 0);
}

#[test]
fn two_appends_interleave_as_two_lines_without_rewriting() {
    // An append is a single record: the second append never rewrites the file, so
    // both records survive as two lines — the parallel-safe interleave.
    let root = common::tmpdir("tap-append");
    let workspace = root.join(".temper");
    std::fs::create_dir_all(&workspace).unwrap();

    let first = TapRecord {
        version: TAP_RECORD_VERSION,
        session: "a".to_string(),
        event: TapEvent::ToolUse,
        identity: "Read".to_string(),
        reason: None,
    };
    let second = TapRecord {
        session: "b".to_string(),
        identity: "Write".to_string(),
        ..first.clone()
    };
    tap::append(&workspace, &first).unwrap();
    tap::append(&workspace, &second).unwrap();

    let raw = std::fs::read_to_string(workspace.join("tap.jsonl")).unwrap();
    assert_eq!(raw.lines().count(), 2, "two appends produce two lines");

    let readout = tap::read_log(&workspace).unwrap();
    assert_eq!(readout.records, vec![first, second]);
}

#[test]
fn an_older_version_record_reads_tolerated_and_counted() {
    // A record an older `TAP_RECORD_VERSION` wrote deserializes tolerated: it is
    // read into the readout and counted, never silently skipped and never a hard
    // error. Written by hand to stand for a prior tap's output.
    let root = common::tmpdir("tap-skew");
    let workspace = root.join(".temper");
    std::fs::create_dir_all(&workspace).unwrap();

    let current = TapRecord {
        version: TAP_RECORD_VERSION,
        session: "now".to_string(),
        event: TapEvent::ToolUse,
        identity: "Read".to_string(),
        reason: None,
    };
    // A line an older tap wrote: version 0, otherwise the current schema.
    let older = "{\"version\":0,\"session\":\"then\",\"event\":\"tool_use\",\"identity\":\"Grep\"}";
    let log = workspace.join("tap.jsonl");
    std::fs::write(
        &log,
        format!("{older}\n{}\n", serde_json::to_string(&current).unwrap()),
    )
    .unwrap();

    let readout = tap::read_log(&workspace).unwrap();
    assert_eq!(
        readout.records.len(),
        2,
        "the older record is read, not skipped"
    );
    assert_eq!(
        readout.older_version, 1,
        "the older record is counted out loud"
    );
    assert_eq!(readout.records[0].version, 0);
    assert_eq!(readout.records[0].identity, "Grep");
    assert_eq!(readout.records[1], current);
}
