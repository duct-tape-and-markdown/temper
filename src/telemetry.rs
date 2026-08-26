//! Telemetry field strand — narrate the local telemetry the tap recorded, per-event
//! counts joined to members through the corpus the gate reads (READ-EDGE-UNIFY).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use crate::extract::Features;
use crate::tap::{TapEvent, TapRecord};

/// Event statistics for requirement-grain telemetry: (count, distinct_members, distinct_sessions, satisfiers_with_records)
type EventStats = (usize, BTreeSet<String>, BTreeSet<String>, BTreeSet<String>);

/// Extract a member ID from an InstructionsLoaded record's repo-relative file path,
/// using the lock's path-to-id mapping for placement-folded identities.
/// For nested files (e.g., `.claude/subdir/CLAUDE.md`), looks up the placement-folded id
/// (e.g., `subdir-CLAUDE`); for flat files, returns the bare stem (e.g., `.claude/rules/rust.md` → `rust`).
/// Falls back to the original identity if not found in the map.
fn normalize_instructions_loaded_identity(
    identity: &str,
    path_to_id: &std::collections::BTreeMap<String, String>,
) -> String {
    // First, try to find an exact match in the lock's path-to-id map.
    if let Some(id) = path_to_id.get(identity) {
        return id.clone();
    }
    // Fall back to bare filename stem for members not in the map.
    let path = Path::new(identity);
    if let Some(filename) = path.file_name()
        && let Some(name_str) = filename.to_str()
        && let Some(member_id) = name_str.strip_suffix(".md")
    {
        member_id.to_string()
    } else {
        identity.to_string()
    }
}

/// Normalize a record's identity for member-index lookup: InstructionsLoaded records
/// use path-to-id normalization, all others use the identity as-is.
fn normalized_record_identity(record: &TapRecord, path_to_id: &BTreeMap<String, String>) -> String {
    if record.event == crate::tap::TapEvent::InstructionsLoaded {
        normalize_instructions_loaded_identity(&record.identity, path_to_id)
    } else {
        record.identity.clone()
    }
}

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
    path_to_id: &BTreeMap<String, String>,
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
        // Normalize InstructionsLoaded identities from repo-relative paths to member IDs
        // before comparing against the member_index (which is keyed by member ID).
        let normalized_identity = normalized_record_identity(record, path_to_id);

        if !member_index.contains_key(normalized_identity.as_str()) {
            continue;
        }
        let entry = tallies.entry(event_label(record.event)).or_default();
        entry.1 += 1;
        if normalized_identity == member {
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

/// `explain`'s **field** strand for a requirement — narrate the aggregated telemetry
/// for a requirement's satisfiers: per declared event, counts of records, distinct
/// members among the satisfiers, distinct sessions, and the satisfiers with zero
/// records. Evidence narrated, never judged.
///
/// If the log is empty and no declared events are provided, returns an empty string.
/// If no records match any satisfier, reports zero-hit satisfiers and the absence.
///
/// For a satisfier-less (unfilled) requirement, joins records and reports zero-hit
/// entries over the declared member corpus (via `member_index`) instead of the empty
/// satisfier set, matching the "declared wiring against the empty log" discipline.
#[must_use]
pub fn requirement_field(
    records: &[TapRecord],
    older_version: usize,
    satisfier_ids: &[String],
    declared_events: &[String],
    member_index: &BTreeMap<&str, Vec<(&str, &Features)>>,
    path_to_id: &BTreeMap<String, String>,
) -> String {
    if records.is_empty() && older_version == 0 && declared_events.is_empty() {
        return String::new();
    }

    // For an unfilled requirement, report against declared members; otherwise against satisfiers.
    let use_member_index = satisfier_ids.is_empty();

    // Build a set of satisfier ids (or member ids if unfilled) for fast lookup
    let check_set: BTreeSet<String> = if use_member_index {
        member_index.keys().map(|k| k.to_string()).collect()
    } else {
        satisfier_ids.iter().cloned().collect()
    };

    // Per declared event: counts of records, distinct members, distinct sessions,
    // and (satisfiers/members) with records
    let mut event_stats: BTreeMap<String, EventStats> = BTreeMap::new();

    for declared_event in declared_events {
        event_stats.insert(
            declared_event.clone(),
            (0, BTreeSet::new(), BTreeSet::new(), BTreeSet::new()),
        );
    }

    // Aggregate records: only count those naming a member in the check set
    for record in records {
        // Normalize InstructionsLoaded identities from repo-relative paths to member IDs
        // before comparing against the check set.
        let normalized_identity = normalized_record_identity(record, path_to_id);

        if !check_set.contains(&normalized_identity) {
            continue;
        }
        let event_name = crate::tap::documented_name(record.event).to_string();
        if let Some((count, members, sessions, entities_with_records)) =
            event_stats.get_mut(&event_name)
        {
            *count += 1;
            members.insert(normalized_identity.clone());
            sessions.insert(record.session.clone());
            entities_with_records.insert(normalized_identity.clone());
        }
    }

    let mut out = String::new();
    let _ = writeln!(
        out,
        "Requirement — its local telemetry (evidence narrated, never judged):\n"
    );

    // For unfilled requirements, narrate the absence of satisfiers
    if use_member_index {
        let _ = writeln!(
            out,
            "The requirement has no satisfiers (unfilled). Telemetry counts are reported \
             against the declared member corpus:"
        );
    }

    // Collect which entities (satisfiers or members) have no records
    let entities_with_no_records: Vec<String> = if use_member_index {
        member_index
            .keys()
            .filter(|id| {
                event_stats
                    .values()
                    .all(|(_, _, _, entities_with_records)| {
                        !entities_with_records.contains(&id.to_string())
                    })
            })
            .map(|k| k.to_string())
            .collect()
    } else {
        satisfier_ids
            .iter()
            .filter(|id| {
                event_stats
                    .values()
                    .all(|(_, _, _, entities_with_records)| !entities_with_records.contains(*id))
            })
            .cloned()
            .collect()
    };

    let has_any_records = event_stats.values().any(|(count, _, _, _)| *count > 0);

    if has_any_records {
        let _ = writeln!(
            out,
            "Per declared event, tap counts and the distinct members/sessions among the requirement's {entity_label}:",
            entity_label = if use_member_index {
                "declared members"
            } else {
                "satisfiers"
            }
        );
        for (event_name, (count, members, sessions, _)) in &event_stats {
            if *count > 0 {
                let member_plural = if members.len() == 1 { "" } else { "s" };
                let session_plural = if sessions.len() == 1 { "" } else { "s" };
                let record_plural = if *count == 1 { "" } else { "s" };
                let _ = writeln!(
                    out,
                    "  • `{event_name}` — {count} record{record_plural}, {} distinct member{member_plural}, {} distinct session{session_plural}",
                    members.len(),
                    sessions.len()
                );
            }
        }
    } else if use_member_index {
        let _ = writeln!(
            out,
            "No tap events in the log name any of the declared members."
        );
    } else {
        let _ = writeln!(
            out,
            "No tap events in the log name any of the requirement's satisfiers."
        );
    }

    if !entities_with_no_records.is_empty() {
        if has_any_records {
            let _ = writeln!(out);
        }
        let _ = writeln!(
            out,
            "{label} with zero records: {}",
            entities_with_no_records
                .iter()
                .map(|id| format!("`{}`", id))
                .collect::<Vec<_>>()
                .join(", "),
            label = if use_member_index {
                "Declared members"
            } else {
                "Satisfiers"
            }
        );
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
