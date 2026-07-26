//! Telemetry field strand — narrate the local telemetry the tap recorded, per-event
//! counts joined to members through the corpus the gate reads (READ-EDGE-UNIFY).

use std::collections::BTreeMap;
use std::fmt::Write;

use crate::extract::Features;
use crate::tap::{TapEvent, TapRecord};

/// `explain`'s **field** strand — narrate the local telemetry the tap recorded for
/// `member`: its per-event counts and the denominators they range against, both joined
/// to members through the same member index the gate reads (READ-EDGE-UNIFY), so the
/// strand cannot disagree with a green `check`. Evidence narrated, never judged: it
/// reports what fired and scores nothing, and no verdict enters an exit code.
///
/// An absent or empty log narrates no field strand at all — an empty string, so the
/// caller joins nothing: absent evidence is silence, not a section reading "zero". A
/// present log naming no member still narrates the strand, stating plainly that nothing
/// named it. A record an older tap wrote surfaces as a counted line, never a silent skip.
///
/// When the log is absent/empty and the lock declares tap registrations (via `has_declared_telemetry`),
/// the absence is narrated against the declared wiring, evidence of what was declared but not recorded.
#[must_use]
pub fn field(
    records: &[TapRecord],
    older_version: usize,
    member_index: &BTreeMap<&str, Vec<(&str, &Features)>>,
    member: &str,
    has_declared_telemetry: bool,
) -> String {
    // Absent/empty log with no declared telemetry: no evidence to narrate.
    if records.is_empty() && older_version == 0 {
        // If the lock declares tap registrations but the log is empty, narrate the absence
        // against the declared wiring.
        if has_declared_telemetry {
            let mut out = String::new();
            let _ = writeln!(
                out,
                "Member `{member}` — its local telemetry (evidence narrated, never judged):\n"
            );
            let _ = writeln!(
                out,
                "The lock declares tap registrations, but the tap log carries no records."
            );
            return out;
        }
        return String::new();
    }

    // Per event: this member's count (numerator) against every lock-joined record's count
    // (denominator). A `BTreeMap` keyed by the record's event label for stable output.
    // A record enters a denominator only when its identity names a member the lock declares
    // (through the member_index), so an event naming a tool or path no kind declares never
    // counts against the members: the join is through the lock, never a raw string tally.
    let mut tallies: BTreeMap<&'static str, (usize, usize)> = BTreeMap::new();
    for record in records {
        if !member_index.contains_key(record.identity.as_str()) {
            continue;
        }
        let entry = tallies.entry(event_label(record.event)).or_default();
        entry.1 += 1;
        if record.identity == member {
            entry.0 += 1;
        }
    }

    let mut out = String::new();
    let _ = writeln!(
        out,
        "Member `{member}` — its local telemetry (evidence narrated, never judged):\n"
    );

    let named: Vec<(&str, usize, usize)> = tallies
        .iter()
        .filter(|(_, (numerator, _))| *numerator > 0)
        .map(|(label, (numerator, denominator))| (*label, *numerator, *denominator))
        .collect();
    if named.is_empty() {
        let _ = writeln!(out, "No tap event in the log names it.");
    } else {
        let _ = writeln!(
            out,
            "Tap events naming it, each counted against every event of that kind the log \
             joins to a declared member:"
        );
        for (label, numerator, denominator) in named {
            let _ = writeln!(out, "  • `{label}` — {numerator} of {denominator}");
        }
    }

    if older_version > 0 {
        let _ = writeln!(
            out,
            "Older records: {older_version} line{} an older tap wrote — counted, never \
             silently skipped.",
            crate::display::plural(older_version)
        );
    }

    out
}

/// The record vocabulary's label for a [`TapEvent`] — the snake_case name the tap log
/// carries, so the field strand narrates each event under the same name the record was
/// written with.
fn event_label(event: TapEvent) -> &'static str {
    match event {
        TapEvent::InstructionsLoaded => "instructions_loaded",
        TapEvent::SkillInvoked => "skill_invoked",
        TapEvent::UserPromptExpansion => "user_prompt_expansion",
        TapEvent::ToolUse => "tool_use",
    }
}
