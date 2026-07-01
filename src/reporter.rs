//! The reporter family — the gate's machine-format placements.
//!
//! Implements the reporters of `specs/50-distribution.md` ("Outward seams —
//! Reporters"): **one reporter family, every placement**. Each member serializes
//! the same merged `check::Diagnostic` set into a different machine format — it
//! never re-judges the harness, so the gate's verdict is identical whichever
//! reporter renders it:
//!
//! - [`github`] — GitHub Actions `::error`/`::warning::` workflow-command lines,
//!   one per finding, so findings land as annotations inline on the PR;
//! - [`sarif`] — a SARIF 2.1.0 log for code-scanning, so findings land in the
//!   team's security review surface;
//! - [`session_start`] — the `claude-session-start` reporter, the JSON payload a
//!   Claude Code `SessionStart` hook writes to stdout (the gate above).
//!
//! Every member is built through `serde_json` (SARIF and the hook payload) or
//! precise workflow-command escaping (`github`), so the output is well-formed by
//! construction — the binary owns the output contract, no hand-escaping.
//!
//! The gate is **advisory, never blocking**. `SessionStart` cannot block, and a
//! hostile gate gets disabled (law 3's UX-door failure mode), so the reporter
//! routes a failing contract through the human rather than denying the session:
//!
//! - a **failing contract** (any `error`-severity diagnostic — a `required`
//!   conformance violation or an inadmissible contract) emits the verdict as
//!   `additionalContext`, prefixed with an instruction to **notify the user and
//!   get approval before continuing**;
//! - a **clean harness** emits a *quiet* payload — the hook envelope with no
//!   `additionalContext`, so nothing is injected into the session.
//!
//! The `additionalContext` string is capped to Claude Code's 10k limit
//! ([`ADDITIONAL_CONTEXT_CAP`]); the notify-and-approve instruction leads the
//! verdict so it always survives truncation of a long finding list. The payload
//! is built through `serde_json`, so every message is escaped correctly and the
//! output is valid JSON by construction — the binary owns the output contract,
//! no shell wrapper and no hand-escaping (`specs/50-distribution.md`).

use serde_json::json;

use crate::check::{self, Diagnostic, Severity};

/// Claude Code's cap on the `additionalContext` string a `SessionStart` hook may
/// inject (`specs/50-distribution.md`). The rendered verdict is truncated to fit.
pub const ADDITIONAL_CONTEXT_CAP: usize = 10_000;

/// The `hookEventName` Claude Code expects in a `SessionStart` hook's
/// `hookSpecificOutput` envelope.
const HOOK_EVENT_NAME: &str = "SessionStart";

/// The instruction that leads a failing verdict: the gate is advisory, so it asks
/// the agent to route the findings through the human rather than block.
const NOTIFY_INSTRUCTION: &str =
    "Notify the user of these findings and get their approval before continuing.";

/// Render the merged diagnostic set into the `SessionStart` hook JSON payload.
///
/// Returns the serialized payload for the hook to write to stdout. A failing
/// contract yields an envelope carrying the verdict (the notify-and-approve
/// instruction plus one line per blocking finding, capped to
/// [`ADDITIONAL_CONTEXT_CAP`]) as `additionalContext`; a clean harness yields the
/// quiet envelope — no `additionalContext`, nothing injected. The gate never
/// blocks, so this reporter carries no failure signal of its own; the caller
/// exits zero regardless.
#[must_use]
pub fn session_start(diagnostics: &[Diagnostic]) -> String {
    let payload = if check::any_error(diagnostics) {
        json!({
            "hookSpecificOutput": {
                "hookEventName": HOOK_EVENT_NAME,
                "additionalContext": cap(&verdict(diagnostics)),
            }
        })
    } else {
        // Quiet: the hook envelope with no `additionalContext`, so a clean
        // harness injects nothing into the session.
        json!({
            "hookSpecificOutput": {
                "hookEventName": HOOK_EVENT_NAME,
            }
        })
    };
    payload.to_string()
}

/// The plain-text verdict for a failing contract: the notify-and-approve
/// instruction first (so it survives truncation), then one line per blocking
/// (`error`-severity) finding naming its clause, artifact, and message.
///
/// Plain text, not the terminal's graphical render ([`check::render`]): this
/// string is injected into an agent's context, where ANSI framing would be noise.
fn verdict(diagnostics: &[Diagnostic]) -> String {
    let blocking: Vec<&Diagnostic> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == Severity::Error)
        .collect();

    let mut out = format!(
        "temper session-start gate — the harness contract is failing ({} blocking finding{}).\n\n",
        blocking.len(),
        if blocking.len() == 1 { "" } else { "s" },
    );
    out.push_str(NOTIFY_INSTRUCTION);
    out.push_str("\n\nBlocking findings:\n");
    for diagnostic in blocking {
        out.push_str(&format!(
            "  - [{}] {}: {}\n",
            diagnostic.rule, diagnostic.artifact, diagnostic.message
        ));
    }
    out
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

/// The SARIF version this reporter emits (`specs/50-distribution.md`, "Outward
/// seams — Reporters": SARIF for code-scanning). 2.1.0 is the OASIS standard
/// GitHub code-scanning and the wider ecosystem ingest.
const SARIF_VERSION: &str = "2.1.0";

/// Render the diagnostic set as GitHub Actions workflow-command lines — one
/// `::error` / `::warning::` annotation per finding, so findings surface inline on
/// the PR (`specs/50-distribution.md`, "Outward seams — Reporters").
///
/// Each line carries the rule as the annotation `title=` and the finding message
/// as the command body; the [`Severity`] picks the command (`error` / `warning`).
/// Data and property values are escaped per GitHub's workflow-command rules
/// ([`escape_data`] / [`escape_property`]) so a message containing a newline,
/// `%`, `:`, or `,` can never break out of its line. Purely a presentation of the
/// shared diagnostic set — it re-judges nothing, so the gate's verdict is
/// untouched.
#[must_use]
pub fn github(diagnostics: &[Diagnostic]) -> String {
    let mut out = String::new();
    for diagnostic in diagnostics {
        let command = match diagnostic.severity {
            Severity::Error => "error",
            Severity::Warn => "warning",
        };
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

/// Render the diagnostic set as a SARIF 2.1.0 log for code-scanning ingestion
/// (`specs/50-distribution.md`, "Outward seams — Reporters"): one run, driver
/// `temper`, one `results` entry per diagnostic.
///
/// Each result maps the rule to `ruleId`, the message to `message.text`, the
/// [`Severity`] to `level` (`error` / `warning`), and the artifact to a
/// `locations` `artifactLocation.uri`. Built through `serde_json`, so every field
/// is escaped correctly and the log is valid JSON by construction. Purely a
/// presentation of the shared diagnostic set — it re-judges nothing, so the gate's
/// verdict is untouched.
#[must_use]
pub fn sarif(diagnostics: &[Diagnostic]) -> String {
    let results: Vec<serde_json::Value> = diagnostics
        .iter()
        .map(|diagnostic| {
            let level = match diagnostic.severity {
                Severity::Error => "error",
                Severity::Warn => "warning",
            };
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

    let log = json!({
        "version": SARIF_VERSION,
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "temper",
                    "informationUri": "https://github.com/temper",
                    "version": crate::VERSION,
                }
            },
            "results": results,
        }]
    });
    log.to_string()
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
    /// structure Claude Code would.
    fn payload(diagnostics: &[Diagnostic]) -> serde_json::Value {
        serde_json::from_str(&session_start(diagnostics)).expect("payload must be valid JSON")
    }

    #[test]
    fn a_failing_contract_carries_the_verdict_and_notify_instruction() {
        let diagnostics = vec![
            Diagnostic::error(
                "allowed_chars",
                "Coordinate",
                "name has characters outside [a-z0-9-]",
            ),
            Diagnostic::warn("max_lines", "coordinate", "body is long"),
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

        // Advisory-only is still a clean *contract* (no error severity) — quiet.
        let advisory = vec![Diagnostic::warn("max_lines", "coordinate", "body is long")];
        let json = payload(&advisory);
        assert!(json["hookSpecificOutput"]["additionalContext"].is_null());
    }

    #[test]
    fn additional_context_is_capped_and_valid_json() {
        // Far more findings than fit under the cap — each with a long message.
        let long = "x".repeat(200);
        let diagnostics: Vec<Diagnostic> = (0..500)
            .map(|i| Diagnostic::error("required", format!("artifact-{i}"), &long))
            .collect();

        let rendered = session_start(&diagnostics);
        let json: serde_json::Value =
            serde_json::from_str(&rendered).expect("even a capped payload is valid JSON");
        let context = json["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .unwrap();
        assert_eq!(context.chars().count(), ADDITIONAL_CONTEXT_CAP);
        // The instruction leads the verdict, so truncation never drops it.
        assert!(context.contains("approval before continuing"));
    }
}
