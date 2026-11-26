//! Acceptance for the memory-kind gate: the
//! `check` gate validates every embedded kind's members against
//! its default contract, so a discovered `CLAUDE.md` memory member fires its
//! `memory.anthropic` clauses instead of being silently skipped by a hardcoded skill/rule
//! pair.
//!
//! Driven across the real process boundary through the one-shot `check --harness` verb
//! (the route session-start takes: import into a scratch surface, gate, tear it down),
//! over harness-dir fixtures mirroring the real Claude Code layout — `.claude/skills/*`
//! plus a repo-root `CLAUDE.md`. The GitHub reporter is used for a machine-parseable
//! finding set: each finding is one `::error`/`::warning title=<rule>::<artifact>: …`
//! line, so a `extent` advisory on the `CLAUDE` member is counted exactly.

use std::fs;
use std::path::Path;

use crate::common;

use crate::common::{check_harness, write_sibling};

/// A skill that trips no `error`-severity clause: lowercase `name` matching its
/// directory, a present description, a short body — so the memory finding is not masked
/// by an unrelated skill failure, and the "clean skill still passes" arm is honest.
const CLEAN_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill that violates a `required` clause: the uppercase `name` is outside the
/// `[a-z0-9-]` `allowed_chars` set — the existing skill finding the no-regression arm
/// asserts still fires beside the memory advisory.
const ERROR_SKILL: &str = "---\n\
name: Coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// Write a repo-root `CLAUDE.md` of `lines` total lines — the `memory` member `import`
/// discovers off its `governs` locus (`root = "."`, `glob = "CLAUDE.md"`).
fn write_claude_md(root: &Path, lines: usize) {
    let mut body = String::from("# Memory\n");
    // Already one line; pad to the requested total so `extent` (a body-line budget)
    // sees exactly `lines`.
    for i in 1..lines {
        body.push_str(&format!("Guidance line {i}.\n"));
    }
    fs::write(root.join("CLAUDE.md"), body).unwrap();
}

/// Write a repo-root `CLAUDE.md` whose body carries the given `@`-import directive on
/// its own line — the `memory` member `import` discovers off its `governs` locus, its
/// `at-import` target the directive classing resolves against provenance.
fn write_claude_md_importing(root: &Path, import_line: &str) {
    let body = format!("# Memory\n\nProject guidance.\n\n{import_line}\n");
    fs::write(root.join("CLAUDE.md"), body).unwrap();
}

/// Gate the harness from its own root with cwd = harness root (`check .`, no
/// `--harness`): the path argument resolves as a harness root, so `check` reads
/// built-in kind members live off harness disk and enumerates the whole repo for
/// directive backing — no scratch import needed. Returns the emitted finding lines.
fn check_two_step(harness: &Path) -> Vec<String> {
    common::check_in(harness, &["."], Some("github")).findings()
}

#[test]
fn the_two_step_check_path_backs_a_real_repo_root_import() {
    let harness = common::tmpdir("two-step-backed");
    // The TWO-STEP path (import to `.temper/`, then gate the bare workspace relative
    // to the harness root): a CLAUDE.md `@import`ing a real repo-root sibling. Before a
    // past fix, the harness root resolved to the EMPTY path, so `repo_file_set` walked
    // nothing and EVERY real import read unbacked. The backing set is the whole repo,
    // so the resolving edge must fire no finding.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_sibling(&harness, "docs/ledger.md", "# Ledger\n\nShared state.\n");
    write_claude_md_importing(&harness, "@docs/ledger.md");

    assert!(
        common::findings_for(&check_two_step(&harness), "root.import-backed").is_empty(),
        "the two-step path must enumerate the whole repo and back a real sibling, got: {:#?}",
        check_two_step(&harness)
    );

    // The backing check stays live, not disabled: delete the sibling and the advisory
    // returns (the surface is unchanged — the target is a plain repo file, not a member —
    // so no re-import is needed; the backing set is re-walked from disk each gate).
    fs::remove_file(harness.join("docs/ledger.md")).unwrap();
    let findings = check_two_step(&harness);
    let unbacked = common::findings_for(&findings, "root.import-backed");
    assert_eq!(
        unbacked.len(),
        1,
        "removing the target brings the unbacked advisory back, got: {findings:#?}"
    );
    assert!(
        unbacked[0].contains("docs/ledger.md"),
        "the returned finding names the now-missing target, got: {}",
        unbacked[0]
    );
}

#[test]
fn an_unbacked_at_import_in_a_claude_md_fires_one_unbacked_pointer_finding() {
    let harness = common::tmpdir("unbacked-import");
    // A clean skill so the run is not empty, and a CLAUDE.md importing a path that backs
    // no member and no repo file — an unbacked pointer. Before the collection generalized
    // over every kind (DIRECTIVE-MEMBERS-ALL-KINDS), the hardcoded skill/rule pair never
    // reached the memory member's directives, so this drew no finding (exit 0).
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md_importing(&harness, "@docs/missing.md");

    let findings = check_harness(&harness).0;
    let unbacked = common::findings_for(&findings, "root.import-backed");

    // Exactly one unbacked-pointer finding, on the memory member — the wedge path now
    // collects the CLAUDE.md's `at-import` targets and classes them.
    assert_eq!(
        unbacked.len(),
        1,
        "expected exactly one unbacked-pointer finding on the memory member, got: {findings:#?}"
    );
    let finding = unbacked[0];
    assert!(
        finding.contains("::CLAUDE:"),
        "the finding names the CLAUDE memory member, got: {finding}"
    );
    assert!(
        finding.contains("docs/missing.md"),
        "the finding names the unbacked target, got: {finding}"
    );
}

#[test]
fn a_claude_md_import_resolving_to_a_member_fires_no_unbacked_finding() {
    let harness = common::tmpdir("backed-import");
    // The CLAUDE.md imports the coordinate skill member by its provenance locus — a
    // resolving member→member edge, not an unbacked pointer. The wedge collects the
    // directive and classes it as backed, so nothing fires.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md_importing(&harness, "@.claude/skills/coordinate/SKILL.md");

    let findings = check_harness(&harness).0;

    assert!(
        common::findings_for(&findings, "root.import-backed").is_empty(),
        "a directive resolving to a real member must fire no unbacked-pointer finding, got: {findings:#?}"
    );
}

#[test]
fn an_unbacked_at_import_fires_a_non_gating_advisory_with_zero_config() {
    let harness = common::tmpdir("floor-unbacked-import");
    // The FLOOR-tier wedge (WEDGE-FACT-FLOOR): a discovered CLAUDE.md carrying an unbacked
    // `@import`. Directive classing runs on the floor, so the unbacked pointer surfaces
    // with zero config authored anywhere. It is a non-gating advisory: the pure fact is
    // stated, never escalated.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md_importing(&harness, "@docs/missing.md");

    let findings = check_harness(&harness).0;
    let unbacked = common::findings_for(&findings, "root.import-backed");

    // Exactly one unbacked-pointer finding, on the memory member, drawn with zero config.
    assert_eq!(
        unbacked.len(),
        1,
        "the floor tier surfaces the unbacked `@import` with zero config, got: {findings:#?}"
    );
    let finding = unbacked[0];
    assert!(
        finding.starts_with("::warning "),
        "the unbacked pointer is a non-gating advisory (warn), got: {finding}"
    );
    assert!(
        finding.contains("::CLAUDE:"),
        "the finding names the CLAUDE memory member, got: {finding}"
    );
    assert!(
        finding.contains("docs/missing.md"),
        "the finding names the unbacked target, got: {finding}"
    );
}

#[test]
fn a_backed_at_import_fires_nothing_with_zero_config() {
    let harness = common::tmpdir("floor-backed-import");
    // The floor tier states only the fact: a CLAUDE.md whose `@path` resolves to a real repo
    // file (the coordinate skill's on-disk member) is a backed boundary edge, not an unbacked
    // pointer — so it draws no finding even with zero config. Pairs the fired case above.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md_importing(&harness, "@.claude/skills/coordinate/SKILL.md");

    let findings = check_harness(&harness).0;

    assert!(
        common::findings_for(&findings, "root.import-backed").is_empty(),
        "a backed `@import` fires no unbacked-pointer finding on the floor, got: {findings:#?}"
    );
}

#[test]
fn an_over_length_claude_md_fires_exactly_one_memory_extent_advisory() {
    let harness = common::tmpdir("over-length");
    // A clean skill so the run is not empty, and a 251-line CLAUDE.md over the
    // memory.anthropic 200-line budget.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md(&harness, 251);

    let findings = check_harness(&harness).0;
    let extent = common::findings_for(&findings, "memory.extent");

    // Exactly one `extent` advisory — the memory member dispatched to
    // memory.anthropic, not silently skipped.
    assert_eq!(
        extent.len(),
        1,
        "expected exactly one extent advisory on the memory member, got: {findings:#?}"
    );
    // It is a `warning` (advisory) naming the `CLAUDE` member and the 251/200 budget.
    let finding = extent[0];
    assert!(
        finding.starts_with("::warning "),
        "extent is an advisory (warn), got: {finding}"
    );
    assert!(
        finding.contains("::CLAUDE:"),
        "the advisory names the CLAUDE memory member, got: {finding}"
    );
    assert!(
        finding.contains("251") && finding.contains("200"),
        "the advisory reports the 251-line body over the 200 budget, got: {finding}"
    );

    // No regression: the clean skill still trips no finding under its own kind.
    assert!(
        common::findings_for(&findings, "skill.allowed_chars.name").is_empty(),
        "the clean skill must not trip allowed_chars, got: {findings:#?}"
    );
}

#[test]
fn an_under_length_claude_md_fires_no_memory_advisory() {
    let harness = common::tmpdir("under-length");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    // A short CLAUDE.md, well under the 200-line budget.
    write_claude_md(&harness, 10);

    let findings = check_harness(&harness).0;

    // The memory member is still dispatched to memory.anthropic — it simply conforms, so
    // the body-size budget fires nothing.
    assert!(
        common::findings_for(&findings, "memory.extent").is_empty(),
        "an under-length CLAUDE.md must fire no extent advisory, got: {findings:#?}"
    );
}

#[test]
fn the_memory_dispatch_leaves_skill_findings_unchanged() {
    let harness = common::tmpdir("no-regression");
    // A failing skill (uppercase name) beside an over-length CLAUDE.md: both the skill's
    // existing finding and the new memory advisory must fire, unaffected by each other.
    common::write_skill(&harness, "coordinate", ERROR_SKILL);
    write_claude_md(&harness, 251);

    let findings = check_harness(&harness).0;

    // The skill finding still fires, exactly as before the gate generalized.
    let allowed_chars = common::findings_for(&findings, "skill.allowed_chars.name");
    assert_eq!(
        allowed_chars.len(),
        1,
        "the uppercase-name skill must still trip allowed_chars, got: {findings:#?}"
    );
    assert!(
        allowed_chars[0].contains("::coordinate:"),
        "the skill finding names the coordinate skill, got: {}",
        allowed_chars[0]
    );

    // And the memory advisory fires beside it — the two kinds are judged in one run.
    assert_eq!(
        common::findings_for(&findings, "memory.extent").len(),
        1,
        "the memory advisory fires beside the skill finding, got: {findings:#?}"
    );
}

#[test]
fn a_backslash_escaped_space_continues_a_memory_import_path() {
    let harness = common::tmpdir("escaped-space-import");
    // `@Design\ Docs/api.md` imports `Design Docs/api.md`: the documented escape keeps
    // the space inside the path (code.claude.com/docs/en/memory, "Import additional
    // files", retrieved 2026-11-25), so the real file backs it and nothing fires.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_sibling(&harness, "Design Docs/api.md", "# API\n\nThe surface.\n");
    write_claude_md_importing(&harness, "@Design\\ Docs/api.md");

    let findings = check_two_step(&harness);
    assert!(
        common::findings_for(&findings, "root.import-backed").is_empty(),
        "the escaped space continues the path to the real file, got: {findings:#?}"
    );

    fs::remove_file(harness.join("Design Docs/api.md")).unwrap();
    let findings = check_two_step(&harness);
    let unbacked = common::findings_for(&findings, "root.import-backed");
    assert_eq!(unbacked.len(), 1, "got: {findings:#?}");
    assert!(
        unbacked[0].contains("`@Design Docs/api.md`"),
        "the finding names the unescaped path, got: {}",
        unbacked[0]
    );
}

#[test]
fn a_trailing_period_stays_part_of_a_memory_import_path() {
    let harness = common::tmpdir("trailing-period-import");
    // The docs are silent on trailing punctuation, so the literal bytes bind: `See @a.md.`
    // imports `a.md.`, which the real `a.md` does not back.
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_sibling(&harness, "a.md", "# A\n");
    write_claude_md_importing(&harness, "See @a.md.");

    let findings = check_two_step(&harness);
    let unbacked = common::findings_for(&findings, "root.import-backed");
    assert_eq!(unbacked.len(), 1, "got: {findings:#?}");
    assert!(
        unbacked[0].contains("`@a.md.`"),
        "the period stays on the path, got: {}",
        unbacked[0]
    );
}

#[test]
fn an_unbacked_import_ending_in_a_period_says_the_period_is_part_of_the_path() {
    let harness = common::tmpdir("trailing-period-guidance");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_sibling(&harness, "a.md", "# A\n");
    write_claude_md_importing(&harness, "See @a.md.");

    let findings = check_two_step(&harness);
    let unbacked = common::findings_for(&findings, "root.import-backed");
    assert_eq!(unbacked.len(), 1, "got: {findings:#?}");
    assert!(
        unbacked[0].contains("trailing `.` is part of the import path"),
        "the finding names the trailing period as part of the path, got: {}",
        unbacked[0]
    );

    // A path not ending in a period carries no such note.
    write_claude_md_importing(&harness, "@docs/missing.md");
    let findings = check_two_step(&harness);
    let unbacked = common::findings_for(&findings, "root.import-backed");
    assert_eq!(unbacked.len(), 1, "got: {findings:#?}");
    assert!(
        !unbacked[0].contains("trailing `.`"),
        "no period note without a trailing period, got: {}",
        unbacked[0]
    );
}

/// Write a memory file at `relative` under `root` with a body of `lines` total lines.
fn write_memory_file(root: &Path, relative: &str, lines: usize) {
    let mut body = String::from("# Instructions\n");
    for i in 1..lines {
        body.push_str(&format!("Guidance line {i}.\n"));
    }
    write_sibling(root, relative, &body);
}

#[test]
fn a_harness_with_only_an_agents_md_at_root_has_it_as_a_reachable_memory_member() {
    let harness = common::tmpdir("agents-md-only");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    // Over the memory budget, so the `extent` advisory proves the file is a memory member.
    write_memory_file(&harness, "AGENTS.md", 251);

    let findings = check_harness(&harness).0;
    let extent = common::findings_for(&findings, "memory.extent");
    assert!(
        extent.len() == 1 && extent[0].contains("::AGENTS:"),
        "the root AGENTS.md is a memory member, got: {findings:#?}"
    );
    assert!(
        common::findings_for(&findings, "root.reachable").is_empty(),
        "with no CLAUDE.md at or above, the AGENTS.md is loaded, got: {findings:#?}"
    );
}

#[test]
fn an_agents_md_beside_a_claude_md_with_no_import_draws_a_not_loaded_reachability_finding() {
    let harness = common::tmpdir("agents-md-shadowed");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md(&harness, 3);
    write_memory_file(&harness, "AGENTS.md", 3);

    let findings = check_harness(&harness).0;
    let unreached = common::findings_for(&findings, "root.reachable");
    assert_eq!(
        unreached.len(),
        1,
        "the shadowed AGENTS.md alone is unreachable, got: {findings:#?}"
    );
    assert!(
        unreached[0].contains("`AGENTS`") && unreached[0].contains("`CLAUDE.md`"),
        "the finding names the member and the CLAUDE.md that shadows it, got: {}",
        unreached[0]
    );
}

#[test]
fn a_nested_agents_md_under_only_a_root_claude_md_is_loaded_on_demand() {
    let harness = common::tmpdir("agents-md-nested-open");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md(&harness, 3);
    write_sibling(&harness, "apps/CLAUDE.local.md", "# Local\n");
    write_memory_file(&harness, "apps/web/AGENTS.md", 3);

    let findings = check_harness(&harness).0;
    assert!(
        common::findings_for(&findings, "root.reachable").is_empty(),
        "a subdirectory's AGENTS.md opens on demand past every CLAUDE.md above it, got: {findings:#?}"
    );
}

#[test]
fn a_nested_agents_md_beside_its_own_claude_local_md_is_shadowed() {
    let harness = common::tmpdir("agents-md-nested-shut");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_sibling(&harness, "apps/web/CLAUDE.local.md", "# Local\n");
    write_memory_file(&harness, "apps/web/AGENTS.md", 3);
    write_memory_file(&harness, "tools/AGENTS.md", 3);

    let findings = check_harness(&harness).0;
    let unreached = common::findings_for(&findings, "root.reachable");
    assert_eq!(
        unreached.len(),
        1,
        "only the AGENTS.md beside its own CLAUDE.local.md is shadowed, got: {findings:#?}"
    );
    assert!(
        unreached[0].contains("`apps:web:AGENTS`")
            && unreached[0].contains("on-demand gate is shut")
            && unreached[0].contains("`apps/web/CLAUDE.local.md`"),
        "the finding names the nested member, its on-demand gate and its shadowing file, got: {}",
        unreached[0]
    );
}

#[test]
fn a_dot_claude_agents_md_is_a_memory_member_shut_by_a_root_claude_md() {
    let harness = common::tmpdir("agents-md-dot-claude");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_claude_md(&harness, 3);
    write_memory_file(&harness, ".claude/AGENTS.md", 3);

    let findings = check_harness(&harness).0;
    let unreached = common::findings_for(&findings, "root.reachable");
    assert_eq!(
        unreached.len(),
        1,
        "the shadowed .claude/AGENTS.md alone is unreachable, got: {findings:#?}"
    );
    assert!(
        unreached[0].contains("memory `.claude:AGENTS`")
            && unreached[0].contains("launch gate is shut")
            && unreached[0].contains("`CLAUDE.md`"),
        "the finding names the memory member, its launch gate and the root CLAUDE.md, got: {}",
        unreached[0]
    );
}

#[test]
fn a_claude_md_importing_agents_md_reaches_it_through_the_import_edge() {
    let harness = common::tmpdir("agents-md-imported");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);
    write_memory_file(&harness, "AGENTS.md", 3);
    write_claude_md_importing(&harness, "@AGENTS.md");

    let findings = check_harness(&harness).0;
    assert!(
        common::findings_for(&findings, "root.reachable").is_empty(),
        "the import reaches the shadowed AGENTS.md, got: {findings:#?}"
    );
    assert!(
        common::findings_for(&findings, "root.import-backed").is_empty(),
        "the import resolves, got: {findings:#?}"
    );
}

/// Commit a lock declaring one rule member — a *represented* harness, so the root
/// `locus-declared` clause asks the lock about every memory document on disk.
fn represent_with_one_rule(root: &Path) {
    use temper::drift::{self, Declarations, EmitOptions, Payload};
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::rule_kind_facts(None, &[])],
            ..Declarations::default()
        },
        members: vec![common::rule_member(
            "declared",
            None,
            "# declared\n\nBody.\n",
        )],
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

/// The `locus-declared` finding naming `path`, out of `findings`.
fn locus_declared_for<'a>(findings: &'a [String], path: &str) -> &'a String {
    common::findings_for(findings, "root.locus-declared")
        .into_iter()
        .find(|f| f.contains(&format!("`{path}`")))
        .unwrap_or_else(|| panic!("`{path}` draws a locus-declared finding, got: {findings:#?}"))
}

#[test]
fn an_undeclared_agents_md_under_a_shut_gate_says_its_gate_is_shut_rather_than_that_claude_code_loads_it()
 {
    let harness = common::tmpdir("agents-md-undeclared-shut");
    represent_with_one_rule(&harness);
    write_claude_md(&harness, 3);
    write_memory_file(&harness, "AGENTS.md", 3);

    let findings = check_harness(&harness).0;
    let agents = locus_declared_for(&findings, "AGENTS.md");
    assert!(
        agents.contains("launch gate is shut") && agents.contains("`CLAUDE.md`"),
        "the finding names the shut gate and the CLAUDE.md that shut it, got: {agents}"
    );
    assert!(
        !agents.contains("Claude Code loads it"),
        "a document behind a shut gate is never said to load, got: {agents}"
    );
}

#[test]
fn an_undeclared_claude_md_says_claude_code_loads_it() {
    let harness = common::tmpdir("claude-md-undeclared");
    represent_with_one_rule(&harness);
    write_claude_md(&harness, 3);

    let findings = check_harness(&harness).0;
    let claude = locus_declared_for(&findings, "CLAUDE.md");
    assert!(
        claude.contains("yet Claude Code loads it") && !claude.contains("gate"),
        "an ungated memory document is said to load, got: {claude}"
    );
}

#[test]
fn an_undeclared_agents_md_under_an_open_gate_says_its_gate_is_open_and_claude_code_loads_it() {
    let harness = common::tmpdir("agents-md-undeclared-open");
    represent_with_one_rule(&harness);
    write_claude_md(&harness, 3);
    write_memory_file(&harness, "apps/web/AGENTS.md", 3);

    let findings = check_harness(&harness).0;
    let agents = locus_declared_for(&findings, "apps/web/AGENTS.md");
    assert!(
        agents.contains("its on-demand gate is open, so Claude Code loads it"),
        "a document behind an open gate says the gate is open and that it loads, got: {agents}"
    );
}
