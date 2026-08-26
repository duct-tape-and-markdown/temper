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
    // The record the writer serializes reads back with its fields populated by
    // `append` (ts is set by append, identity may be relativized).
    let root = common::tmpdir("tap-roundtrip");
    let workspace = root.join(".temper");
    std::fs::create_dir_all(&workspace).unwrap();

    let record = TapRecord {
        version: TAP_RECORD_VERSION,
        session: "sess".to_string(),
        event: TapEvent::SkillInvoked,
        identity: "verify".to_string(),
        ts: String::new(),
        reason: None,
        raw_path: None,
    };
    tap::append(&workspace, &record).unwrap();

    let readout = tap::read_log(&workspace).unwrap();
    assert_eq!(readout.records.len(), 1, "one append produces one record");
    assert_eq!(readout.older_version, 0, "the current tap wrote it");

    let read_record = &readout.records[0];
    assert_eq!(read_record.version, TAP_RECORD_VERSION);
    assert_eq!(read_record.session, "sess");
    assert_eq!(read_record.event, TapEvent::SkillInvoked);
    assert_eq!(read_record.identity, "verify");
    assert!(!read_record.ts.is_empty(), "ts is populated by append");
    assert_eq!(read_record.reason, None);
    assert_eq!(read_record.raw_path, None);
}

#[test]
fn an_instructions_loaded_identity_relativizes_against_the_checkout_root() {
    // The v2 contract: `append` rewrites an absolute InstructionsLoaded path
    // under the checkout root to the repo-relative id, keeping the absolute
    // original in `raw_path` — the worktree-collapse half of the schema bump.
    let root = common::tmpdir("tap-relativize");
    let workspace = root.join(".temper");
    std::fs::create_dir_all(&workspace).unwrap();

    let absolute = root.join(".claude").join("rules").join("rust.md");
    let record = TapRecord {
        version: TAP_RECORD_VERSION,
        session: "sess".to_string(),
        event: TapEvent::InstructionsLoaded,
        identity: absolute.to_string_lossy().into_owned(),
        ts: String::new(),
        reason: Some("session_start".to_string()),
        raw_path: Some(absolute.to_string_lossy().into_owned()),
    };
    tap::append(&workspace, &record).unwrap();

    let readout = tap::read_log(&workspace).unwrap();
    let read_record = &readout.records[0];
    assert_eq!(
        std::path::Path::new(&read_record.identity),
        std::path::Path::new(".claude/rules/rust.md"),
        "identity is the repo-relative id, not the absolute path"
    );
    assert_eq!(
        read_record.raw_path.as_deref(),
        Some(absolute.to_string_lossy().as_ref()),
        "the raw absolute path survives in raw_path"
    );
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
        ts: String::new(),
        reason: None,
        raw_path: None,
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
    assert_eq!(readout.records.len(), 2);
    assert_eq!(readout.records[0].session, "a");
    assert_eq!(readout.records[0].identity, "Read");
    assert!(!readout.records[0].ts.is_empty());
    assert_eq!(readout.records[1].session, "b");
    assert_eq!(readout.records[1].identity, "Write");
    assert!(!readout.records[1].ts.is_empty());
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
        ts: String::new(),
        reason: None,
        raw_path: None,
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

#[test]
fn log_path_resolves_linked_worktree_to_primary_checkout() {
    // A workspace inside a linked worktree (where .git is a file pointing to the
    // admin directory with relative paths) resolves to the primary checkout's
    // .temper/tap.jsonl instead of the worktree's. This ensures telemetry persists
    // across worktree deletion. Uses a real `git worktree add` to ensure the fixture
    // matches git's actual layout with relative commondir and gitdir paths.
    let tmpdir = common::tmpdir("tap-worktree");
    let primary = tmpdir.join("primary");
    std::fs::create_dir_all(&primary).unwrap();

    // Initialize primary as a git repo
    Command::new("git")
        .arg("init")
        .arg("--initial-branch=main")
        .current_dir(&primary)
        .output()
        .expect("git init failed");

    let worktree = tmpdir.join("worktree");

    // Create a linked worktree using git worktree add, which will write relative
    // paths in the .git file and commondir file (the real git layout).
    Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg(&worktree)
        .current_dir(&primary)
        .output()
        .expect("git worktree add failed");

    // Now append a record using the worktree's workspace and verify it lands in the
    // primary checkout's .temper/tap.jsonl.
    let worktree_workspace = worktree.join(".temper");
    let record = TapRecord {
        version: TAP_RECORD_VERSION,
        session: "test-session".to_string(),
        event: TapEvent::ToolUse,
        identity: "TestTool".to_string(),
        ts: String::new(),
        reason: None,
        raw_path: None,
    };
    tap::append(&worktree_workspace, &record).unwrap();

    // The record should be in the primary checkout's log, not the worktree's.
    let primary_log = primary.join(".temper").join("tap.jsonl");
    assert!(
        primary_log.exists(),
        "log should be in primary checkout at {:?}",
        primary_log
    );

    let worktree_log = worktree.join(".temper").join("tap.jsonl");
    assert!(
        !worktree_log.exists(),
        "log should not be in worktree at {:?}",
        worktree_log
    );

    // Reading the log from the worktree workspace should read from the primary.
    let readout = tap::read_log(&worktree_workspace).unwrap();
    assert_eq!(
        readout.records.len(),
        1,
        "the record was appended to the primary checkout's log"
    );
    assert_eq!(readout.records[0].identity, "TestTool");
}

#[test]
fn log_path_unchanged_for_primary_checkout_with_git_directory() {
    // A primary checkout (where .git is a directory, not a file) resolves to its
    // own workspace unchanged — no linked-worktree chain to follow.
    let primary = common::tmpdir("tap-primary-git-dir");
    let primary_git = primary.join(".git");
    std::fs::create_dir_all(&primary_git).unwrap();

    let workspace = primary.join(".temper");
    let record = TapRecord {
        version: TAP_RECORD_VERSION,
        session: "test-session".to_string(),
        event: TapEvent::ToolUse,
        identity: "Tool".to_string(),
        ts: String::new(),
        reason: None,
        raw_path: None,
    };
    tap::append(&workspace, &record).unwrap();

    let log = workspace.join("tap.jsonl");
    assert!(log.exists(), "log should be at the workspace path");
    let readout = tap::read_log(&workspace).unwrap();
    assert_eq!(readout.records.len(), 1);
    assert_eq!(readout.records[0].identity, "Tool");
}

#[test]
fn log_path_unchanged_for_non_git_directory() {
    // A non-git workspace (no .git file or directory) resolves to its own
    // workspace unchanged — no resolution needed.
    let root = common::tmpdir("tap-non-git");
    let workspace = root.join(".temper");

    let record = TapRecord {
        version: TAP_RECORD_VERSION,
        session: "test-session".to_string(),
        event: TapEvent::ToolUse,
        identity: "Tool".to_string(),
        ts: String::new(),
        reason: None,
        raw_path: None,
    };
    tap::append(&workspace, &record).unwrap();

    let log = workspace.join("tap.jsonl");
    assert!(log.exists(), "log should be at the workspace path");
    let readout = tap::read_log(&workspace).unwrap();
    assert_eq!(readout.records.len(), 1);
    assert_eq!(readout.records[0].identity, "Tool");
}
