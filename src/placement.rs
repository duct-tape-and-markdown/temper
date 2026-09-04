//! Managed-metadata line vocabulary — the marker constants and recognizers install
//! places and emit preserves.

use crate::frontmatter;

/// The managed-by note's stable marker — the comment prefix that *locates* an already
/// placed note (so a second `install` never duplicates it); whether that note is then
/// left verbatim or re-placed keys on the line's bytes vs `NOTE_COMMENT`, not this
/// prefix (`project_note`, content-drift-aware).
pub const NOTE_MARKER: &str = "# temper: managed projection";

/// The banner form's stable marker — the block-level HTML comment prefix that *locates*
/// an already placed banner on a frontmatterless projection, the [`NOTE_MARKER`]
/// counterpart for a body that carries no frontmatter to hold the `#` note
/// (`project_banner`, content-drift-aware).
pub const BANNER_MARKER: &str = "<!-- temper: managed projection";

/// The managed-by note's block-level HTML-comment form, for a frontmatterless
/// markdown projection (a memory `CLAUDE.md`, any frontmatterless kind) with no
/// frontmatter to carry the `#` note. Claude Code strips a block-level HTML comment
/// before injection, so the banner is human-visible and model-invisible — a courtesy
/// marker, the drift hash still catching a hand-edit either way.
pub const BANNER: &str = "<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->";

/// The schema modeline's stable marker — the frontmatter comment prefix `install` keys
/// its idempotence on and `emit` keys its preservation on, so both projectors agree on
/// which line is the modeline. The prefix encodes the yaml-language-server modeline syntax
/// `# yaml-language-server: $schema=<schema-url-or-path>` per github.com/redhat-developer/yaml-language-server
/// README (retrieved 2026-07-20).
pub const MODELINE_MARKER: &str = "# yaml-language-server:";

pub(crate) fn placement_lines(source: &str) -> Vec<String> {
    if let Some((_, matter)) = frontmatter::frontmatter_matter(source) {
        return matter
            .lines()
            .filter(|line| is_placement_comment(line))
            .map(str::to_string)
            .collect();
    }
    // Frontmatterless: the banner rides the head of the body, not a frontmatter
    // block. Return it so emit re-places it exactly as it re-places the `#` note.
    source
        .lines()
        .next()
        .filter(|line| line.trim_start().starts_with(BANNER_MARKER))
        .map(|line| vec![line.to_string()])
        .unwrap_or_default()
}

/// Whether `path` names a markdown file — the one body shape the block-level
/// HTML-comment banner is safe in (an HTML comment is inert markdown, malformed inside
/// a JSON manifest). Emit uses this to decide whether to place the banner.
pub fn is_markdown_path(path: &std::path::Path) -> bool {
    path.extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

/// Strip a leading block-level HTML-comment banner from a body string, dropping the
/// banner line matching [`BANNER_MARKER`] plus the following blank line. Returns the
/// body unchanged if no leading banner is present or if the banner is not followed
/// by a blank line.
pub fn strip_leading_banner(body: &str) -> &str {
    if !body.trim_start().starts_with(BANNER_MARKER) {
        return body;
    }

    let first_line_end = match body.find('\n') {
        Some(pos) => pos,
        None => return body,
    };

    if first_line_end + 1 >= body.len() {
        return "";
    }

    let after_banner = &body[first_line_end + 1..];
    let next_line_end = after_banner.find('\n').unwrap_or(after_banner.len());
    let next_line = &after_banner[..next_line_end];

    if next_line.trim().is_empty() {
        if next_line_end + 1 >= after_banner.len() {
            return "";
        }
        return &after_banner[next_line_end + 1..];
    }

    body
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

    #[test]
    fn placement_lines_extracts_managed_comments_from_crlf_frontmatter() {
        let modeline = "# yaml-language-server: $schema=.temper/harness.json";
        let note = "# temper: managed projection";
        let source = format!("---\r\n{note}\r\n{modeline}\r\nauthor: test\r\n---\r\n# Body\r\n");
        let lines = placement_lines(&source);
        assert_eq!(lines.len(), 2);
        assert!(lines.iter().any(|l| l.contains(NOTE_MARKER)));
        assert!(lines.iter().any(|l| l.contains("yaml-language-server")));
    }
}
