//! The `claude-session-start` reporter — the gate's session-start placement.
//!
//! Implements the `claude-session-start` reporter of `specs/50-distribution.md`
//! ("Decision: the session-start gate is advisory, not blocking"; "Outward seams
//! — Reporters"): one member of the reporter family that serializes the same
//! merged diagnostic set every placement carries, here into the JSON payload a
//! Claude Code `SessionStart` hook writes to stdout.
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
