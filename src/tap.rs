//! `temper tap` — the advisory telemetry recorder.
//!
//! The tap reads a Claude Code hook payload and appends one machine-written
//! record to the per-machine, uncommitted event log. A record is an event's
//! identity and its minimal discriminant — the member or path it names, the
//! load reason, the session id — and never captured prose. The record is the
//! engine's own, not a member: bespoke-parsed and versioned in lockstep with
//! the one binary that both writes and reads it. One home for the record's IO:
//! the append writer, the version-tolerant reader, and the log-path locator
//! ride together.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// The tap record's on-disk version, bumped in lockstep with the binary that
/// both writes and reads it. A reader meeting a record an older tap wrote
/// tolerates it and counts it against this.
pub const TAP_RECORD_VERSION: u32 = 1;

/// Filename of the per-machine event log under the workspace — uncommitted,
/// machine-written, never an emit input or target.
pub const LOG_FILENAME: &str = "tap.jsonl";

/// The lifecycle event a record names — its minimal discriminant, one of the
/// hook events the tap recognizes. The payload's prose (file content, tool
/// output, prompt text) never reaches this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TapEvent {
    /// A `CLAUDE.md` / `.claude/rules/*.md` file loaded into context — identity
    /// is its path.
    InstructionsLoaded,
    /// A skill invoked through the `Skill` tool, surfaced under `PostToolUse` —
    /// identity is the skill name.
    SkillInvoked,
    /// A user-typed command expanded into a prompt — identity is the command
    /// name.
    UserPromptExpansion,
    /// A tool call completed — identity is the tool name.
    ToolUse,
}

/// One tap event, the whole record: the version it was written at, the session
/// it fired in, the lifecycle event, the member or path it names, and the load
/// reason an `InstructionsLoaded` event carries. Serialized one-per-line as
/// JSONL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TapRecord {
    /// The on-disk version this record was written at.
    pub version: u32,
    /// The session the event fired in (`session_id`).
    pub session: String,
    /// Which lifecycle event fired.
    pub event: TapEvent,
    /// The member or path the event names — a file path, a skill name, a command
    /// name, or a tool name, per the event.
    pub identity: String,
    /// The load reason an `InstructionsLoaded` event carries (`session_start`,
    /// `nested_traversal`, …); absent for every other event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Errors raised writing or reading the tap log.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TapError {
    /// The log could not be appended to (its parent unwritable, or the write
    /// failed).
    #[error("failed to append to the tap log at `{path}`")]
    LogAppend {
        /// The log path the append targeted.
        path: PathBuf,
        /// The underlying filesystem error.
        #[source]
        source: std::io::Error,
    },
    /// A record could not be encoded — a build invariant, since the record is
    /// plain data.
    #[error("failed to encode a tap record")]
    Encode {
        /// The underlying serialization error.
        #[source]
        source: serde_json::Error,
    },
    /// The log exists but could not be read.
    #[error("failed to read the tap log at `{path}`")]
    LogRead {
        /// The log path the read targeted.
        path: PathBuf,
        /// The underlying filesystem error.
        #[source]
        source: std::io::Error,
    },
    /// The log carries a line at the current version that the current schema
    /// cannot parse — genuine corruption, distinct from a tolerated older-version
    /// record.
    #[error("the tap log at `{path}` carries an unparseable current-version line")]
    LogParse {
        /// The log path the read targeted.
        path: PathBuf,
        /// The underlying deserialization error.
        #[source]
        source: serde_json::Error,
    },
}

/// The per-machine event log's path under a workspace directory — the one home
/// for locating the log, shared by the writer and the reader.
pub(crate) fn log_path(workspace_dir: &Path) -> PathBuf {
    workspace_dir.join(LOG_FILENAME)
}

/// Build a record from a raw Claude Code hook payload, when the payload names a
/// recognized lifecycle event. Extracts identity + minimal discriminant only —
/// the payload's prose fields (`content`, `tool_response`, `expanded_prompt`)
/// are never read into the record.
///
/// Returns [`None`] for a payload that does not parse, names no recognized
/// event, or lacks the identity field its event needs: a tap is advisory, so an
/// unrecognized payload records nothing rather than failing.
#[must_use]
pub fn record_from_payload(payload: &str) -> Option<TapRecord> {
    let value: JsonValue = serde_json::from_str(payload).ok()?;
    let string = |key: &str| value.get(key).and_then(JsonValue::as_str);
    let session = string("session_id").unwrap_or_default().to_string();

    // The payload shapes are Claude Code's hook contract, an external fact:
    // code.claude.com/docs/en/hooks (retrieved 2026-07-17). InstructionsLoaded
    // carries {file_path, load_reason, content}; UserPromptExpansion
    // {command_name, expanded_prompt}; PostToolUse {tool_name, tool_input,
    // tool_response}; a skill invocation rides PostToolUse with tool_name="Skill"
    // and the skill name under tool_input.skill. The prose fields — content,
    // expanded_prompt, tool_response — are never read into the record.
    let (event, identity, reason) = match string("hook_event_name")? {
        "InstructionsLoaded" => (
            TapEvent::InstructionsLoaded,
            string("file_path")?.to_string(),
            string("load_reason").map(str::to_string),
        ),
        "UserPromptExpansion" => (
            TapEvent::UserPromptExpansion,
            string("command_name")?.to_string(),
            None,
        ),
        "PostToolUse" => {
            let tool = string("tool_name")?;
            if tool == "Skill" {
                let skill = value
                    .get("tool_input")
                    .and_then(|input| input.get("skill"))
                    .and_then(JsonValue::as_str)?;
                (TapEvent::SkillInvoked, skill.to_string(), None)
            } else {
                (TapEvent::ToolUse, tool.to_string(), None)
            }
        }
        _ => return None,
    };

    Some(TapRecord {
        version: TAP_RECORD_VERSION,
        session,
        event,
        identity,
        reason,
    })
}

/// Append one record as a single JSONL line to the per-machine log under
/// `workspace_dir`, creating the log (and its parent) if absent. An append never
/// rewrites the file — it opens in append mode and writes one line — so parallel
/// sessions interleave lines safely rather than clobbering each other.
///
/// # Errors
///
/// Returns a [`TapError`] if the record cannot be encoded or the log's parent is
/// unwritable or the append fails.
pub fn append(workspace_dir: &Path, record: &TapRecord) -> Result<(), TapError> {
    let path = log_path(workspace_dir);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| TapError::LogAppend {
            path: path.clone(),
            source,
        })?;
    }
    let mut line = serde_json::to_string(record).map_err(|source| TapError::Encode { source })?;
    line.push('\n');
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|source| TapError::LogAppend {
            path: path.clone(),
            source,
        })?;
    file.write_all(line.as_bytes())
        .map_err(|source| TapError::LogAppend { path, source })?;
    Ok(())
}

/// The result of reading the whole log: every record the current and tolerated
/// older versions yielded, plus a count of records an older `TAP_RECORD_VERSION`
/// wrote — surfaced, never silently skipped.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LogReadout {
    /// Every record read, in file (append) order.
    pub records: Vec<TapRecord>,
    /// How many log lines an older `TAP_RECORD_VERSION` wrote — the count a
    /// reader narrates out loud rather than dropping in silence.
    pub older_version: usize,
}

/// Read the whole per-machine log under `workspace_dir` into a [`LogReadout`]:
/// every record, plus a count of the older-version records tolerated. A missing
/// log yields an empty readout — absent evidence is no error. A line an older
/// tap wrote is tolerated: read into a record when the current schema still
/// materializes it, and counted; one it cannot materialize is still counted
/// (never a silent skip) rather than aborting the read. Only an unparseable line
/// self-identifying as the *current* version is genuine corruption and errors.
///
/// # Errors
///
/// Returns a [`TapError`] if the log exists but cannot be read, or carries a
/// current-version line the schema cannot parse.
pub fn read_log(workspace_dir: &Path) -> Result<LogReadout, TapError> {
    let path = log_path(workspace_dir);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok(LogReadout::default());
        }
        Err(source) => return Err(TapError::LogRead { path, source }),
    };

    let mut readout = LogReadout::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<TapRecord>(line) {
            Ok(record) => {
                if record.version < TAP_RECORD_VERSION {
                    readout.older_version += 1;
                }
                readout.records.push(record);
            }
            // A line the current schema cannot materialize is tolerated only when
            // it self-identifies as an older version — counted out loud, never
            // dropped and never aborting the read. Anything else is corruption.
            Err(source) => match probe_version(line) {
                Some(version) if version < TAP_RECORD_VERSION => readout.older_version += 1,
                _ => {
                    return Err(TapError::LogParse {
                        path: path.clone(),
                        source,
                    });
                }
            },
        }
    }
    Ok(readout)
}

/// The `version` field of a log line read on its own — the probe that decides
/// whether a line the full schema rejected is a tolerated older record or genuine
/// corruption.
fn probe_version(line: &str) -> Option<u32> {
    serde_json::from_str::<JsonValue>(line)
        .ok()?
        .get("version")?
        .as_u64()
        .and_then(|version| u32::try_from(version).ok())
}
