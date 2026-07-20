//! Managed-metadata line vocabulary — the marker constants and recognizers install
//! places and emit preserves.

use crate::frontmatter;

/// The managed-by note's stable marker — the comment prefix that *locates* an already
/// placed note (so a second `install` never duplicates it); whether that note is then
/// left verbatim or re-placed keys on the line's bytes vs [`NOTE_COMMENT`], not this
/// prefix (`project_note`, content-drift-aware).
pub const NOTE_MARKER: &str = "# temper: managed projection";

/// The banner form's stable marker — the block-level HTML comment prefix that *locates*
/// an already placed banner on a frontmatterless projection, the [`NOTE_MARKER`]
/// counterpart for a body that carries no frontmatter to hold the `#` note
/// (`project_banner`, content-drift-aware).
pub const BANNER_MARKER: &str = "<!-- temper: managed projection";

/// The schema modeline's stable marker — the frontmatter comment prefix `install` keys
/// its idempotence on and `emit` keys its preservation on, so both projectors agree on
/// which line is the modeline.
pub const MODELINE_MARKER: &str = "# yaml-language-server:";

pub(crate) fn placement_lines(source: &str) -> Vec<String> {
    if let Some((_, matter)) = frontmatter::frontmatter_matter(source) {
        return matter
            .lines()
            .filter(|line| is_placement_comment(line))
            .map(str::to_string)
            .collect();
    }
    // Frontmatterless: install's banner rides the head of the body, not a frontmatter
    // block. Return it so emit re-places it exactly as it re-places the `#` note.
    source
        .lines()
        .next()
        .filter(|line| line.trim_start().starts_with(BANNER_MARKER))
        .map(|line| vec![line.to_string()])
        .unwrap_or_default()
}

/// Whether `line` is one of install's managed metadata comments — the schema modeline
/// or the managed-by note. The single predicate install's idempotence and emit's
/// preservation share, so the two projectors never disagree on which lines are install's.
fn is_placement_comment(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with(MODELINE_MARKER) || trimmed.starts_with(NOTE_MARKER)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTE_BANNER: &str = "<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->";

    #[test]
    fn placement_lines_round_trips_the_body_banner_of_a_frontmatterless_source() {
        let source = format!("{NOTE_BANNER}\n\n# Project\n\nMemory body.\n");
        assert_eq!(placement_lines(&source), vec![NOTE_BANNER.to_string()]);
        // A bare frontmatterless body carries no placement.
        assert!(placement_lines("# Project\n\nMemory body.\n").is_empty());
    }
}
