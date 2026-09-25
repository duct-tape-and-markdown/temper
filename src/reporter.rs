//! The reporter family — the gate's machine-format placements.
//!
//! Implements the reporters:
//! **one reporter family, every placement**. Each member serializes
//! the same merged `check::Diagnostic` set into a different machine format — it
//! never re-judges the harness, so the gate's verdict is identical whichever
//! reporter renders it:
//!
//! - [`github`] — GitHub Actions `::error`/`::warning`/`::notice::` workflow-command
//!   lines, one per finding, so findings land as annotations inline on the PR;
//! - [`sarif`] — a SARIF 2.1.0 log for code-scanning, so findings land in the
//!   team's security review surface;
//! - [`session_start`] — the `claude-session-start` reporter, the JSON payload a
//!   Claude Code `SessionStart` hook writes to stdout (the gate above);
//! - [`tool_use`] — the guard's in-band reporter, the JSON payload a tool-call hook
//!   writes to stdout when the declared enforcement mode is `warn`, at either edge of
//!   the call;
//! - [`post_tool_use_block`] — the same guard's `block` at the **post** edge, where the
//!   write has already landed and only the call's *result* is left to refuse.
//!
//! [`session_start`] and [`tool_use`] share one envelope ([`envelope`]) under a
//! [`HookEvent`] parameter, because Claude Code accepts only the firing event's own name
//! in it. [`post_tool_use_block`] speaks the top-level `decision`/`reason` pair its event
//! decides through instead, so it names no event at all.
//!
//! Every member is built through `serde_json` (SARIF and the hook payload) or
//! precise workflow-command escaping (`github`), so the output is well-formed by
//! construction — the binary owns the output contract, no hand-escaping.
//!
//! The gate's advisory, quiet-or-notify behavior is documented at [`session_start`] and [`context`].
//!
//! Each member also carries the run's [`Announcement`] — the inputs beyond the
//! committed harness that judged it — in its own format's shape: a `::notice`
//! line, a SARIF run property bag, the hook's `additionalContext`. It is
//! independent of the verdict, so a clean run judged by a dial or a joined lock
//! is no longer silent.
//!
//! The `additionalContext` string is capped to Claude Code's 10k limit
//! ([`ADDITIONAL_CONTEXT_CAP`]); the notify-and-approve instruction leads the
//! verdict so it always survives truncation of a long finding list. The payload
//! is built through `serde_json`, so every message is escaped correctly and the
//! output is valid JSON by construction — the binary owns the output contract,
//! no shell wrapper and no hand-escaping.

use serde_json::json;

use crate::check::{Announcement, Diagnostic, Severity};

/// Claude Code's cap on the `additionalContext` string a `SessionStart` hook may
/// inject. The rendered verdict is truncated to fit
/// (code.claude.com/docs/en/hooks, retrieved 2026-07-20).
pub const ADDITIONAL_CONTEXT_CAP: usize = 10_000;

/// A hook event, as Claude Code names it in the `hookSpecificOutput` envelope's
/// `hookEventName` field.
///
/// The name is a parameter of [`envelope`] rather than a constant because Claude
/// Code rejects an output stamping any name but the firing event's — so one
/// envelope shape cannot be shared across placements by hard-coding a name; each
/// firing site names its own event (code.claude.com/docs/en/hooks, retrieved
/// 2026-09-24).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HookEvent {
    /// The session-start gate ([`session_start`]).
    SessionStart,
    /// The guard at the edge before a file-writing tool call ([`tool_use`]).
    PreToolUse,
    /// The guard at the edge after a shell tool call, judging the tree the call left
    /// ([`tool_use`], and [`post_tool_use_block`] for the refusal this edge can still make).
    PostToolUse,
}

impl HookEvent {
    /// The event's `hookEventName` spelling.
    const fn name(self) -> &'static str {
        match self {
            Self::SessionStart => "SessionStart",
            Self::PreToolUse => "PreToolUse",
            Self::PostToolUse => "PostToolUse",
        }
    }
}

/// Shape the `hookSpecificOutput` envelope for `event`, carrying `context` as
/// `additionalContext` capped to [`ADDITIONAL_CONTEXT_CAP`], or the quiet envelope
/// when there is nothing to inject.
///
/// This is the one home for the envelope, shared by every hook placement that
/// injects into the live context: `additionalContext` on a hook exiting zero is the
/// **only** channel that reaches the model — a hook's stderr on exit 0 reaches the
/// debug log alone, never the model's context
/// (code.claude.com/docs/en/hooks, "Exit code 0", retrieved 2026-09-24). Only exit
/// code 2 delivers stderr to the model, which is why a non-blocking finding has to
/// ride this envelope to be seen at all.
fn envelope(event: HookEvent, context: Option<&str>) -> String {
    let payload = match context {
        Some(context) => json!({
            "hookSpecificOutput": {
                "hookEventName": event.name(),
                "additionalContext": cap(context),
        }
        }),
        None => json!({
            "hookSpecificOutput": {
                "hookEventName": event.name(),
        }
        }),
    };
    payload.to_string()
}

/// Render a guard finding into the in-band envelope for `event` — the payload the
/// guard writes to stdout under the `warn` enforcement mode, where the write is
/// allowed (exit 0) but the finding must still reach the live context.
///
/// The event name is the caller's, not this function's: the guard fires at both
/// edges of a tool call and Claude Code accepts only the firing event's name
/// ([`HookEvent`]). `block` does not come through here — it exits 2, the one exit
/// code Claude Code delivers a hook's stderr from — and `note` emits nothing at all.
#[must_use]
pub fn tool_use(event: HookEvent, finding: &str) -> String {
    envelope(event, Some(finding))
}

/// Render a guard finding into the `PostToolUse` refusal — the payload the guard writes
/// to stdout under the `block` enforcement mode at the **post** edge of a tool call,
/// where the write the finding indicts has already landed. A write already made cannot
/// be denied, so `block` here refuses the *call's result* and the `reason` names the
/// restore.
///
/// Two external facts shape it (code.claude.com/docs/en/hooks, "JSON output" and
/// "PostToolUse decision control", retrieved 2026-09-25):
///
/// - `PostToolUse` takes its decision from the **top-level** `decision`/`reason` pair,
///   not the `hookSpecificOutput` envelope [`tool_use`] rides: `"block"` adds `reason`
///   next to the tool result, which is the strongest in-band refusal this event offers.
/// - Exit 0 plus this object is the intended shape — the docs' own instruction is to
///   pick one signal per hook, exit codes or JSON, so the guard exits zero here and lets
///   the object refuse. Exit 2 would block whether or not the JSON parsed, and its
///   message would be stderr's rather than this reason's.
///
/// It carries no `hookSpecificOutput`, which is why it takes no [`HookEvent`]: the event
/// name is required only *inside* that envelope, and a name that is not the firing
/// event's is rejected whole ([`HookEvent`]).
#[must_use]
pub fn post_tool_use_block(reason: &str) -> String {
    json!({
        "decision": "block",
        "reason": reason,
    })
    .to_string()
}

/// The instruction that leads a failing verdict: the gate is advisory, so it asks
/// the agent to route the findings through the human rather than block.
const NOTIFY_INSTRUCTION: &str =
    "Notify the user of these findings and get their approval before continuing.";

/// Render the merged diagnostic set and the run's [`Announcement`] into the
/// `SessionStart` hook JSON payload.
///
/// Returns the serialized payload for the hook to write to stdout. A run with
/// something to say — a failing contract, an announced input, or both — yields an
/// envelope carrying it as `additionalContext`, capped to
/// [`ADDITIONAL_CONTEXT_CAP`]; a clean harness judged by its committed lock alone
/// yields the quiet envelope — no `additionalContext`, nothing injected. The gate
/// never blocks, so this reporter carries no failure signal of its own; the caller
/// exits zero regardless. The payload envelope shape — `hookSpecificOutput` with
/// `hookEventName` and `additionalContext` fields — is defined by Claude Code's
/// `SessionStart` hook (code.claude.com/docs/en/hooks, retrieved 2026-07-20).
#[must_use]
pub fn session_start(diagnostics: &[Diagnostic], announcement: &Announcement) -> String {
    envelope(
        HookEvent::SessionStart,
        context(diagnostics, announcement).as_deref(),
    )
}

/// The plain-text context the hook injects, or `None` when the run has nothing to
/// say: a clean contract judged by the committed harness alone.
///
/// Ordered by what must survive [`cap`]'s truncation. The notify-and-approve
/// instruction leads, because a verdict nobody routes to the human is no verdict;
/// the announcement follows, because which inputs judged the run is what the
/// finding list below it cannot be read without; the findings themselves are last
/// and are the length here, so they are what a cut eats.
///
/// Plain text, not the terminal's graphical render ([`crate::check::render`]): this
/// string is injected into an agent's context, where ANSI framing would be noise.
fn context(diagnostics: &[Diagnostic], announcement: &Announcement) -> Option<String> {
    let blocking: Vec<&Diagnostic> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == Severity::Error)
        .collect();
    let advisory: Vec<&Diagnostic> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == Severity::Warn)
        .collect();
    let disclosure: Vec<&Diagnostic> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == Severity::Note)
        .collect();
    if blocking.is_empty()
        && advisory.is_empty()
        && disclosure.is_empty()
        && announcement.is_empty()
    {
        return None;
    }

    let mut out = String::new();
    if !blocking.is_empty() {
        out.push_str(&format!(
            "temper session-start gate — the harness contract is failing ({} blocking finding{}).\n\n{NOTIFY_INSTRUCTION}\n\n",
            blocking.len(),
            crate::display::plural(blocking.len()),
        ));
    }
    if !announcement.is_empty() {
        // "temper judged by inputs the committed harness does not carry:" — the
        // heading is written to take a subject, so the block reads as a sentence to
        // the agent this lands in front of.
        out.push_str("temper ");
        out.push_str(announcement.render().trim_end());
        out.push_str("\n\n");
    }
    if !blocking.is_empty() {
        out.push_str("Blocking findings:\n");
        for diagnostic in &blocking {
            out.push_str(&format!(
                "  - [{}] {}: {}\n",
                diagnostic.rule, diagnostic.artifact, diagnostic.message
            ));
        }
    }
    if !advisory.is_empty() {
        if !blocking.is_empty() {
            out.push('\n');
        }
        out.push_str("Advisory findings:\n");
        for diagnostic in &advisory {
            out.push_str(&format!(
                "  - [{}] {}: {}\n",
                diagnostic.rule, diagnostic.artifact, diagnostic.message
            ));
        }
    }
    if !disclosure.is_empty() {
        if !blocking.is_empty() || !advisory.is_empty() {
            out.push('\n');
        }
        // Last, under its own heading: a disclosure is not a finding to act on — it is
        // what the gate checked, said out loud so silence never reads as "checked".
        out.push_str("Checked:\n");
        for diagnostic in &disclosure {
            out.push_str(&format!(
                "  - [{}] {}: {}\n",
                diagnostic.rule, diagnostic.artifact, diagnostic.message
            ));
        }
    }
    Some(out)
}

/// Truncate `text` to [`ADDITIONAL_CONTEXT_CAP`] characters, marking a cut with a
/// trailing ellipsis so the payload never exceeds Claude Code's limit. Counts and
/// cuts on `char` boundaries so a multi-byte character is never split.
fn cap(text: &str) -> String {
    if text.chars().count() <= ADDITIONAL_CONTEXT_CAP {
        return text.to_string();
    }
    // Reserve one character for the ellipsis so the result is exactly the cap.
    let head: String = text.chars().take(ADDITIONAL_CONTEXT_CAP - 1).collect();
    format!("{head}…")
}

/// The SARIF version this reporter emits. 2.1.0 is the OASIS standard
/// GitHub code-scanning and the wider ecosystem ingest
/// (docs.oasis-open.org/sarif/sarif/v2.1.0/os/sarif-v2.1.0-os.html, retrieved 2026-07-20).
const SARIF_VERSION: &str = "2.1.0";

/// The `title=` an announcement's `::notice` line carries — the one title for all
/// three families, so a workflow filtering the announcement out of its log spells
/// one name.
const ANNOUNCE_TITLE: &str = "temper.announce";

/// Render the diagnostic set as GitHub Actions workflow-command lines — one
/// `::error` / `::warning` / `::notice::` annotation per finding, so findings surface
/// inline on the PR, led by one `::notice` per announced input.
///
/// Each line carries the rule as the annotation `title=` and the finding message
/// as the command body; the [`Severity`] picks the command
/// ([`workflow_command`]: `error` / `warning` / `notice`). An announced input has no
/// severity — it is not a finding — so it rides `::notice` too, the command for a
/// message that is not a problem; the `title=` tells the two apart, an announcement
/// always spelling [`ANNOUNCE_TITLE`] and a disclosure note its own rule id. Data and property
/// values are escaped per GitHub's workflow-command rules ([`escape_data`] /
/// [`escape_property`]) so a message containing a newline, `%`, `:`, or `,` can
/// never break out of its line. Purely a presentation of the shared diagnostic set
/// — it re-judges nothing, so the gate's verdict is untouched.
#[must_use]
pub fn github(diagnostics: &[Diagnostic], announcement: &Announcement) -> String {
    let mut out = String::new();
    for (family, name) in announcement.entries() {
        out.push_str(&format!(
            "::notice title={}::{}: {}\n",
            escape_property(ANNOUNCE_TITLE),
            escape_data(family),
            escape_data(name),
        ));
    }
    for diagnostic in diagnostics {
        let command = workflow_command(diagnostic.severity);
        // `title=` carries the rule (escaped as a property value); the artifact
        // rides the body so the annotation names what it is about, then the
        // message (both escaped as command data).
        out.push_str(&format!(
            "::{command} title={}::{}: {}\n",
            escape_property(&diagnostic.rule),
            escape_data(&diagnostic.artifact),
            escape_data(&diagnostic.message),
        ));
    }
    out
}

/// Render the diagnostic set as a SARIF 2.1.0 log for code-scanning ingestion:
/// one run, driver
/// `temper`, one `results` entry per diagnostic.
///
/// Each result maps the rule to `ruleId`, the message to `message.text`, the
/// [`Severity`] to `level` (`error` / `warning` / `note`), and the artifact to a
/// `locations` `artifactLocation.uri`. The [`Announcement`] rides the run's
/// `properties` bag — SARIF's own home for a tool-specific fact about the run,
/// which is what an announced input is: it names what judged these results rather
/// than being one. The bag is absent entirely when there is nothing to announce.
/// Built through `serde_json`, so every field is escaped correctly and the log is
/// valid JSON by construction. Purely a presentation of the shared diagnostic set
/// — it re-judges nothing, so the gate's verdict is untouched.
#[must_use]
pub fn sarif(diagnostics: &[Diagnostic], announcement: &Announcement) -> String {
    let results: Vec<serde_json::Value> = diagnostics
        .iter()
        .map(|diagnostic| {
            let level = sarif_level(diagnostic.severity);
            json!({
                "ruleId": diagnostic.rule,
                "level": level,
                "message": { "text": diagnostic.message },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": { "uri": diagnostic.artifact }
            }
            }]
            })
        })
        .collect();

    let mut run = json!({
        "tool": {
            "driver": {
                "name": "temper",
                "informationUri": "https://github.com/temper",
                "version": crate::VERSION,
    }
        },
        "results": results,
    });
    if !announcement.is_empty() {
        run["properties"] = json!({
            "localMembers": announcement.local_members,
            "dialedClauses": announcement.dialed_clauses,
            "joinedLocks": announcement.joined_locks,
        });
    }

    let log = json!({
        "version": SARIF_VERSION,
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "runs": [run],
    });
    log.to_string()
}

/// Map a [`Severity`] to its SARIF `level` word. SARIF 2.1.0 defines exactly
/// `error` / `warning` / `note` for a result level, so a disclosure note lands as
/// `note` — present in the log, never a violation.
fn sarif_level(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warn => "warning",
        Severity::Note => "note",
    }
}

/// Map a [`Severity`] to the GitHub Actions workflow command that carries it. GitHub
/// spells the annotation levels `error` / `warning` / `notice` — its own vocabulary, not
/// SARIF's ([`sarif_level`]), so the two are named apart rather than one string
/// re-mapped into the other (code.claude.com is not the source here:
/// docs.github.com/actions/reference/workflow-commands-for-github-actions, retrieved
/// 2026-09-06). A disclosure note rides `::notice` — the command for a message that is
/// not a problem — never `::debug`, which is hidden unless debug logging is on and would
/// suppress the disclosure.
fn workflow_command(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warn => "warning",
        Severity::Note => "notice",
    }
}

/// Escape a string for use as GitHub workflow-command **data** (the message body
/// after `::`). Per GitHub's rules, `%`, carriage return, and newline must be
/// percent-encoded so multi-line data cannot spill past its command line.
fn escape_data(text: &str) -> String {
    text.replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

/// Escape a string for use as a GitHub workflow-command **property** value (e.g.
/// `title=`). A property additionally escapes `:` and `,` — the command's
/// property delimiters — on top of the data escapes.
fn escape_property(text: &str) -> String {
    escape_data(text).replace(':', "%3A").replace(',', "%2C")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse the emitted payload as JSON so a test reads through the same
    /// structure Claude Code would. Announcement-free: the cases below that are
    /// about the announcement reach [`announced`].
    fn payload(diagnostics: &[Diagnostic]) -> serde_json::Value {
        announced(diagnostics, &Announcement::default())
    }

    /// [`payload`] over a run with something to announce.
    fn announced(diagnostics: &[Diagnostic], announcement: &Announcement) -> serde_json::Value {
        serde_json::from_str(&session_start(diagnostics, announcement))
            .expect("payload must be valid JSON")
    }

    #[test]
    fn a_failing_contract_carries_the_verdict_and_notify_instruction() {
        let diagnostics = vec![
            Diagnostic::error(
                "allowed_chars",
                "Coordinate",
                "name has characters outside [a-z0-9-]",
            ),
            Diagnostic::warn("extent", "coordinate", "rendered extent is over budget"),
        ];
        let json = payload(&diagnostics);

        assert_eq!(json["hookSpecificOutput"]["hookEventName"], "SessionStart");
        let context = json["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .expect("a failing contract carries additionalContext");
        // The notify-and-approve instruction and the blocking finding both ride
        // in the verdict; the advisory warn is not counted as blocking.
        assert!(context.contains("approval before continuing"));
        assert!(context.contains("1 blocking finding"));
        assert!(context.contains("allowed_chars"));
        assert!(context.contains("Coordinate"));
    }

    #[test]
    fn a_clean_harness_is_quiet() {
        // No diagnostics at all: the quiet envelope carries no additionalContext.
        let json = payload(&[]);
        assert_eq!(json["hookSpecificOutput"]["hookEventName"], "SessionStart");
        assert!(json["hookSpecificOutput"]["additionalContext"].is_null());

        // Advisory-only surfaces in additionalContext, distinguishable from a clean run.
        let advisory = vec![Diagnostic::warn(
            "extent",
            "coordinate",
            "rendered extent is over budget",
        )];
        let json = payload(&advisory);
        let context = json["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .expect("advisory-only diagnostics must carry additionalContext");
        assert!(context.contains("Advisory findings:"));
        assert!(context.contains("extent"));
        assert!(context.contains("coordinate"));
        // Advisory-only should not carry the notify-and-approve instruction
        // (that only leads a failing contract).
        assert!(!context.contains("approval before continuing"));
    }

    #[test]
    fn additional_context_is_capped_and_valid_json() {
        // Far more findings than fit under the cap — each with a long message.
        let long = "x".repeat(200);
        let diagnostics: Vec<Diagnostic> = (0..500)
            .map(|i| Diagnostic::error("required", format!("artifact-{i}"), &long))
            .collect();

        let announcement = Announcement {
            joined_locks: vec!["/org/lock.toml".to_string()],
            ..Default::default()
        };
        let rendered = session_start(&diagnostics, &announcement);
        let json: serde_json::Value =
            serde_json::from_str(&rendered).expect("even a capped payload is valid JSON");
        let context = json["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .unwrap();
        assert_eq!(context.chars().count(), ADDITIONAL_CONTEXT_CAP);
        // Both lead the finding list, so a cut eats findings and never the
        // instruction or what judged the run.
        assert!(context.contains("approval before continuing"));
        assert!(context.contains("joined lock: /org/lock.toml"));
    }

    #[test]
    fn the_guard_envelope_stamps_its_own_event_and_shares_the_cap() {
        let json: serde_json::Value =
            serde_json::from_str(&tool_use(HookEvent::PreToolUse, "a finding"))
                .expect("the guard envelope is valid JSON");
        assert_eq!(json["hookSpecificOutput"]["hookEventName"], "PreToolUse");
        assert_eq!(
            json["hookSpecificOutput"]["additionalContext"], "a finding",
            "the finding rides additionalContext — the only channel a zero-exit hook \
             reaches the model through"
        );

        // The one envelope, so the one cap: a finding longer than the limit is cut the
        // same way a session-start verdict is.
        let long = "x".repeat(ADDITIONAL_CONTEXT_CAP + 1);
        let json: serde_json::Value =
            serde_json::from_str(&tool_use(HookEvent::PreToolUse, &long)).unwrap();
        let context = json["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .unwrap();
        assert_eq!(context.chars().count(), ADDITIONAL_CONTEXT_CAP);
        assert!(context.ends_with('…'));
    }

    #[test]
    fn the_post_edge_refusal_speaks_the_top_level_decision_pair() {
        let json: serde_json::Value =
            serde_json::from_str(&post_tool_use_block("re-run `temper emit`"))
                .expect("the refusal is valid JSON");
        assert_eq!(json["decision"], "block");
        assert_eq!(json["reason"], "re-run `temper emit`");
        // No envelope, so no event name to get wrong: this event decides through the
        // top-level pair, and the envelope's `hookEventName` requirement never applies.
        assert!(
            json.get("hookSpecificOutput").is_none(),
            "got: {json}, which would have to stamp an event name to be accepted"
        );
    }

    #[test]
    fn a_clean_run_still_announces_what_judged_it() {
        let announcement = Announcement {
            local_members: vec!["dial:workstation".to_string()],
            dialed_clauses: vec!["skill.extent".to_string()],
            joined_locks: vec!["/org/lock.toml".to_string()],
        };
        let json = announced(&[], &announcement);
        let context = json["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .expect("an announced run carries additionalContext whatever its verdict");

        assert!(context.contains("local member: dial:workstation"));
        assert!(context.contains("dialed clause: skill.extent"));
        assert!(context.contains("joined lock: /org/lock.toml"));
        // Nothing is failing, so there is nothing to route through the human.
        assert!(!context.contains("approval before continuing"));
    }
}
