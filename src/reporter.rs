//! The reporter family — the gate's machine-format placements.
//!
//! Implements the reporters:
//! **one reporter family, every placement**. Each member serializes
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
//! hostile gate gets disabled, so the reporter
//! routes a failing contract through the human rather than denying the session:
//!
//! - a **failing contract** (any `error`-severity diagnostic — a `required`
//!   conformance violation or an inadmissible contract) emits the verdict as
//!   `additionalContext`, prefixed with an instruction to **notify the user and
//!   get approval before continuing**;
//! - a **clean harness** emits a *quiet* payload — the hook envelope with no
//!   `additionalContext`, so nothing is injected into the session — unless the
//!   run has something to announce, which is worth a session's attention whatever
//!   the verdict.
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
/// inject. The rendered verdict is truncated to fit.
pub const ADDITIONAL_CONTEXT_CAP: usize = 10_000;

/// The `hookEventName` Claude Code expects in a `SessionStart` hook's
/// `hookSpecificOutput` envelope.
const HOOK_EVENT_NAME: &str = "SessionStart";

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
/// exits zero regardless.
#[must_use]
pub fn session_start(diagnostics: &[Diagnostic], announcement: &Announcement) -> String {
    let payload = match context(diagnostics, announcement) {
        Some(context) => json!({
            "hookSpecificOutput": {
                "hookEventName": HOOK_EVENT_NAME,
                "additionalContext": cap(&context),
        }
        }),
        None => json!({
            "hookSpecificOutput": {
                "hookEventName": HOOK_EVENT_NAME,
        }
        }),
    };
    payload.to_string()
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
    if blocking.is_empty() && announcement.is_empty() {
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
        for diagnostic in blocking {
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
/// GitHub code-scanning and the wider ecosystem ingest.
const SARIF_VERSION: &str = "2.1.0";

/// The `title=` an announcement's `::notice` line carries — the one title for all
/// three families, so a workflow filtering the announcement out of its log spells
/// one name.
const ANNOUNCE_TITLE: &str = "temper.announce";

/// Render the diagnostic set as GitHub Actions workflow-command lines — one
/// `::error` / `::warning::` annotation per finding, so findings surface inline on
/// the PR, led by one `::notice` per announced input.
///
/// Each line carries the rule as the annotation `title=` and the finding message
/// as the command body; the [`Severity`] picks the command (`error` / `warning`).
/// An announced input has no severity — it is not a finding — so it rides
/// `::notice`, the command for a message that is not a problem. Data and property
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

/// Render the diagnostic set as a SARIF 2.1.0 log for code-scanning ingestion:
/// one run, driver
/// `temper`, one `results` entry per diagnostic.
///
/// Each result maps the rule to `ruleId`, the message to `message.text`, the
/// [`Severity`] to `level` (`error` / `warning`), and the artifact to a
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

        // Advisory-only is still a clean *contract* (no error severity) — quiet.
        let advisory = vec![Diagnostic::warn(
            "extent",
            "coordinate",
            "rendered extent is over budget",
        )];
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
