//! The slice-1 skill lint rules.
//!
//! Implements the rule table in `spec/RELEASE-v0.1.md` ("Check behavior (the
//! lint engine)"): ten mechanical string/structure checks over the typed
//! [`Skill`] IR, each a [`Rule`] that emits zero or more [`Diagnostic`]s. The
//! checks encode the documented Claude Code skill best practices (Anthropic's
//! hard mechanics + Pocock's precision heuristics — see the spec's Sources).
//!
//! Every check is plain `std` string/char/path work over the byte-faithful body
//! and the parsed frontmatter — no regex, no markdown parser (both outside the
//! sanctioned crate set, SPEC §7). The deferred heuristic rules (gerund naming,
//! no-op detection, Leitwort reuse, reference table-of-contents) are *not* here:
//! the spec parks them for a later release as judgment calls, not slice-1
//! mechanics.
//!
//! ## One root cause, one diagnostic
//!
//! A rule that presupposes a field is well-formed defers to the rule that owns
//! that field: the `name`/`description` content rules stay silent when their
//! field is empty, because [`FrontmatterValid`] already reports the emptiness.
//! Without this, one broken field would cascade into a pile of derived findings
//! and bury the real cause.

use std::collections::BTreeSet;
use std::path::Path;

use crate::check::{Diagnostic, Rule, Workspace};
use crate::skill::Skill;

/// Frontmatter `name` cap (Anthropic spec).
const MAX_NAME_LEN: usize = 64;
/// Frontmatter `description` cap (Anthropic spec).
const MAX_DESCRIPTION_LEN: usize = 1024;
/// Body line budget; at or above this, split via progressive disclosure.
const MAX_BODY_LINES: usize = 500;
/// Names that collide with the platform's own identifiers.
const RESERVED_NAMES: [&str; 2] = ["anthropic", "claude"];
/// Extensions that mark a body token as a file reference (not prose).
const REFERENCE_EXTENSIONS: [&str; 17] = [
    ".md",
    ".markdown",
    ".sh",
    ".bash",
    ".py",
    ".js",
    ".ts",
    ".rs",
    ".toml",
    ".json",
    ".yaml",
    ".yml",
    ".txt",
    ".csv",
    ".sql",
    ".rb",
    ".go",
];

/// The full slice-1 rule set, in the order of the spec table (the four hard
/// errors, the description/body advisories, then the reference checks).
///
/// Registration lives here, not in the engine: [`crate::check::run`] takes the
/// rules as a slice, so the CLI composes this set and the engine stays disjoint
/// from it.
pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(FrontmatterValid),
        Box::new(NameFormat),
        Box::new(NameMatchesDir),
        Box::new(DescriptionLength),
        Box::new(DescriptionThirdPerson),
        Box::new(DescriptionHasTrigger),
        Box::new(DescriptionHasAntiTrigger),
        Box::new(BodyLength),
        Box::new(CompanionRefsResolve),
        Box::new(RefsOneLevelDeep),
    ]
}

/// `skill.frontmatter-valid` (error): the required `name` and `description`
/// fields are present and non-empty. (Their *presence* is enforced when the IR
/// loads; this rule catches the present-but-blank case the loader admits.)
pub struct FrontmatterValid;

impl Rule for FrontmatterValid {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            if skill.name.trim().is_empty() {
                out.push(Diagnostic::error(
                    "skill.frontmatter-valid",
                    &skill.name,
                    "frontmatter field `name` is empty",
                ));
            }
            if skill.description.trim().is_empty() {
                out.push(Diagnostic::error(
                    "skill.frontmatter-valid",
                    &skill.name,
                    "frontmatter field `description` is empty",
                ));
            }
        }
        out
    }
}

/// `skill.name-format` (error): `name` is ≤ 64 chars, `[a-z0-9-]` only, and not
/// a reserved word. Silent on an empty name — [`FrontmatterValid`] owns that.
pub struct NameFormat;

impl Rule for NameFormat {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            let name = &skill.name;
            if name.trim().is_empty() {
                continue;
            }

            if name.chars().count() > MAX_NAME_LEN {
                out.push(Diagnostic::error(
                    "skill.name-format",
                    name,
                    format!("name is longer than {MAX_NAME_LEN} characters"),
                ));
            }

            let invalid: BTreeSet<char> = name
                .chars()
                .filter(|&c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'))
                .collect();
            if !invalid.is_empty() {
                let rendered: String = invalid.into_iter().collect();
                out.push(Diagnostic::error(
                    "skill.name-format",
                    name,
                    format!("name has characters outside [a-z0-9-]: {rendered}"),
                ));
            }

            if RESERVED_NAMES.contains(&name.to_ascii_lowercase().as_str()) {
                out.push(Diagnostic::error(
                    "skill.name-format",
                    name,
                    format!("name `{name}` is a reserved word"),
                ));
            }
        }
        out
    }
}

/// `skill.name-matches-dir` (error): `name` equals the skill's containing
/// directory (the folder Claude Code discovers it under). The directory is read
/// from `provenance.source_path` — the original on-disk location, preserved
/// across import. Silent on an empty name ([`FrontmatterValid`] owns that).
pub struct NameMatchesDir;

impl Rule for NameMatchesDir {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            if skill.name.trim().is_empty() {
                continue;
            }
            if let Some(dir) = source_dir_name(skill)
                && dir != skill.name
            {
                out.push(Diagnostic::error(
                    "skill.name-matches-dir",
                    &skill.name,
                    format!("name `{}` does not match its directory `{dir}`", skill.name),
                ));
            }
        }
        out
    }
}

/// `skill.description-length` (error): `description` is ≤ 1024 chars. Silent on
/// an empty description ([`FrontmatterValid`] owns that).
pub struct DescriptionLength;

impl Rule for DescriptionLength {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            if skill.description.trim().is_empty() {
                continue;
            }
            let len = skill.description.chars().count();
            if len > MAX_DESCRIPTION_LEN {
                out.push(Diagnostic::error(
                    "skill.description-length",
                    &skill.name,
                    format!("description is {len} characters (max {MAX_DESCRIPTION_LEN})"),
                ));
            }
        }
        out
    }
}

/// `skill.description-third-person` (warn): the description reads in the third
/// person — no first/second-person pronouns. It is injected into the system
/// prompt, so `I`/`you` there is addressed at the wrong subject.
pub struct DescriptionThirdPerson;

impl Rule for DescriptionThirdPerson {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            if skill.description.trim().is_empty() {
                continue;
            }
            if word_tokens(&skill.description).any(is_person_token) {
                out.push(Diagnostic::warn(
                    "skill.description-third-person",
                    &skill.name,
                    "description uses a first/second-person pronoun; write it in the third person",
                ));
            }
        }
        out
    }
}

/// `skill.description-has-trigger` (warn): the description says *when* to reach
/// for the skill (a trigger/context), not only what it does.
pub struct DescriptionHasTrigger;

impl Rule for DescriptionHasTrigger {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            if skill.description.trim().is_empty() {
                continue;
            }
            if !has_trigger(&skill.description) {
                out.push(Diagnostic::warn(
                    "skill.description-has-trigger",
                    &skill.name,
                    "description states what the skill does but not when to use it",
                ));
            }
        }
        out
    }
}

/// `skill.description-has-anti-trigger` (warn): the description says when **not**
/// to use the skill — the branch precision that keeps it from over-firing.
pub struct DescriptionHasAntiTrigger;

impl Rule for DescriptionHasAntiTrigger {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            if skill.description.trim().is_empty() {
                continue;
            }
            if !has_anti_trigger(&skill.description) {
                out.push(Diagnostic::warn(
                    "skill.description-has-anti-trigger",
                    &skill.name,
                    "description does not state when NOT to use the skill",
                ));
            }
        }
        out
    }
}

/// `skill.body-length` (warn): the SKILL.md body is under 500 lines; beyond that,
/// move detail into companions via progressive disclosure.
pub struct BodyLength;

impl Rule for BodyLength {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            let lines = skill.body.lines().count();
            if lines >= MAX_BODY_LINES {
                out.push(Diagnostic::warn(
                    "skill.body-length",
                    &skill.name,
                    format!("body is {lines} lines (keep it under {MAX_BODY_LINES})"),
                ));
            }
        }
        out
    }
}

/// `skill.companion-refs-resolve` (error): every file path the body references
/// exists among the skill's companions.
pub struct CompanionRefsResolve;

impl Rule for CompanionRefsResolve {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            let companions = companion_set(skill);
            for reference in referenced_paths(&skill.body) {
                if !companions.contains(&reference) {
                    out.push(Diagnostic::error(
                        "skill.companion-refs-resolve",
                        &skill.name,
                        format!("body references `{reference}`, which is not on disk"),
                    ));
                }
            }
        }
        out
    }
}

/// `skill.refs-one-level-deep` (warn): referenced files sit at most one directory
/// below SKILL.md; deeper paths risk partial `head` reads.
pub struct RefsOneLevelDeep;

impl Rule for RefsOneLevelDeep {
    fn check(&self, ws: &Workspace) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        for skill in &ws.skills {
            for reference in referenced_paths(&skill.body) {
                if reference.matches('/').count() > 1 {
                    out.push(Diagnostic::warn(
                        "skill.refs-one-level-deep",
                        &skill.name,
                        format!("body references `{reference}`, more than one level deep"),
                    ));
                }
            }
        }
        out
    }
}

/// The name of the directory a skill lives in, taken from its source path.
fn source_dir_name(skill: &Skill) -> Option<&str> {
    skill
        .provenance
        .source_path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
}

/// The skill's companions as normalized (forward-slash) strings, for comparison
/// against the references extracted from the body.
fn companion_set(skill: &Skill) -> BTreeSet<String> {
    skill
        .companions
        .iter()
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect()
}

/// Whether `description` states a usage trigger. Mechanical: the canonical
/// "Use when …" phrasing and its close variants. A coarse heuristic on purpose —
/// the spec scopes this to a slice-1 string check, not a judgment call.
fn has_trigger(description: &str) -> bool {
    let lower = description.to_ascii_lowercase();
    const MARKERS: [&str; 5] = ["when", "use this", "use when", "use for", "useful for"];
    MARKERS.iter().any(|marker| lower.contains(marker))
}

/// Whether `description` states an anti-trigger — a negation or exception that
/// scopes the skill away from cases it should not fire on.
fn has_anti_trigger(description: &str) -> bool {
    word_tokens(description).any(is_anti_token)
}

/// A first- or second-person pronoun token (the third-person check's quarry).
/// Bare `I` is matched case-sensitively; the rest case-insensitively, so a
/// lowercase `i` (rarely a pronoun) does not raise a false positive.
fn is_person_token(token: &str) -> bool {
    if token == "I" {
        return true;
    }
    matches!(
        token.to_ascii_lowercase().as_str(),
        "you"
            | "your"
            | "yours"
            | "you're"
            | "you'll"
            | "you've"
            | "you'd"
            | "i'm"
            | "i'll"
            | "i've"
            | "i'd"
            | "we"
            | "we're"
            | "we'll"
            | "we've"
            | "my"
            | "mine"
            | "me"
            | "our"
            | "ours"
            | "us"
    )
}

/// A negation/exception token that signals an anti-trigger.
fn is_anti_token(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    if lower.ends_with("n't") {
        return true;
    }
    matches!(
        lower.as_str(),
        "not"
            | "never"
            | "avoid"
            | "except"
            | "unless"
            | "without"
            | "rather"
            | "instead"
            | "none"
            | "neither"
            | "nor"
    )
}

/// Split text into word tokens — runs of alphanumerics and apostrophes — so
/// contractions (`don't`, `you're`) survive intact for the pronoun/negation
/// checks while surrounding punctuation is dropped.
fn word_tokens(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '\''))
        .filter(|token| !token.is_empty())
}

/// Extract the file paths a body references — markdown link/image targets and
/// inline-code spans that look like relative paths — normalized to a comparable
/// form (first whitespace-delimited part, no `#anchor`, no leading `./`),
/// deduped in first-seen order.
fn referenced_paths(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    collect_markdown_targets(body, &mut out);
    collect_inline_code(body, &mut out);
    out
}

/// Push every markdown link/image target (`](target)`) onto `out`.
fn collect_markdown_targets(body: &str, out: &mut Vec<String>) {
    let mut rest = body;
    while let Some(pos) = rest.find("](") {
        let after = &rest[pos + 2..];
        match after.find(')') {
            Some(end) => {
                push_candidate(&after[..end], out);
                rest = &after[end + 1..];
            }
            None => break,
        }
    }
}

/// Push the contents of every single-backtick inline-code span onto `out`.
fn collect_inline_code(body: &str, out: &mut Vec<String>) {
    let mut in_code = false;
    for segment in body.split('`') {
        if in_code {
            push_candidate(segment, out);
        }
        in_code = !in_code;
    }
}

/// Normalize a raw candidate and, if it looks like a relative file path, append
/// it to `out` (skipping URLs, anchors, and duplicates).
fn push_candidate(raw: &str, out: &mut Vec<String>) {
    let first = raw.split_whitespace().next().unwrap_or("");
    let no_anchor = first.split('#').next().unwrap_or(first);
    let normalized = no_anchor.strip_prefix("./").unwrap_or(no_anchor);

    if normalized.is_empty() || normalized.contains("://") {
        return;
    }
    if normalized.starts_with("http") || normalized.starts_with("mailto:") {
        return;
    }

    let looks_like_path = normalized.contains('/') || has_reference_extension(normalized);
    let owned = normalized.to_string();
    if looks_like_path && !out.contains(&owned) {
        out.push(owned);
    }
}

/// Whether a token ends in one of the recognized file-reference extensions.
fn has_reference_extension(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    REFERENCE_EXTENSIONS.iter().any(|ext| lower.ends_with(ext))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_detects_use_when_phrasing() {
        assert!(has_trigger("Use when importing a harness."));
        assert!(!has_trigger("Imports a harness into the typed surface."));
    }

    #[test]
    fn anti_trigger_detects_negations_and_contractions() {
        assert!(has_anti_trigger("Use this; not for single files."));
        assert!(has_anti_trigger("Use this, but don't run it on bundles."));
        assert!(!has_anti_trigger("Use when importing a harness."));
    }

    #[test]
    fn person_token_flags_i_and_you_but_not_lowercase_i() {
        assert!(is_person_token("I"));
        assert!(is_person_token("You"));
        assert!(is_person_token("you're"));
        // A lowercase bare `i` is almost never the pronoun — no false positive.
        assert!(!is_person_token("i"));
        assert!(!is_person_token("imports"));
    }

    #[test]
    fn referenced_paths_picks_links_and_code_paths_not_prose_or_urls() {
        let body = "See [the playbook](PLAYBOOK.md) and run `scripts/deep/run.sh`.\n\
            A [link](https://example.com) and a plain word are ignored.";
        let refs = referenced_paths(body);
        assert_eq!(refs, vec!["PLAYBOOK.md", "scripts/deep/run.sh"]);
    }

    #[test]
    fn referenced_paths_strips_anchors_and_dedupes() {
        let body = "[a](PLAYBOOK.md#top) then `PLAYBOOK.md` again and `./PLAYBOOK.md`.";
        let refs = referenced_paths(body);
        assert_eq!(refs, vec!["PLAYBOOK.md"]);
    }
}
