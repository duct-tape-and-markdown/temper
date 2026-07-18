//! Measure-first cost diagnosis for `check` at consumer scale
//! (`specs/process/engineering.md`, "Cost scale is hoisted, and pinned by count").
//!
//! A synthetic harness the size of a real consumer's tree is generated in a tempdir at
//! test time and never committed. Discovery — the phase `check` opens with, walking the
//! consumer's whole tree per kind — is timed over it so the numbers, not a guess, name
//! where the residual concentrates; the timings print (a manual signal a human reads) and
//! the test asserts the work-count pin the cut earns: glob compilation is hoisted per
//! distinct glob, never recomputed per candidate file, decided by a count and independent
//! of tree size.

mod common;

use std::fs;
use std::path::Path;
use std::time::Instant;

use common::tmpdir;
use temper::builtin_kind;
use temper::frontmatter::Member;
use temper::import::{self, Discovery, LocalOverride};
use temper::kind;

/// Generate a Claude Code harness at consumer scale under `root`, mirroring the real
/// layout (`.claude/skills/<name>/SKILL.md` + companions, `.claude/rules/*.md`,
/// `.claude/commands/*.md`, `.claude/agents/**/*.md`, nested `**/CLAUDE.md` memory), and
/// return the file count. Never committed — the tree is disposable synthetic input, built
/// only to name a cost.
fn generate_harness(root: &Path, scale: usize) -> usize {
    let mut files = 0usize;

    // Skills: directory-unit members, each SKILL.md with two companions — the `*/SKILL.md`
    // subdir glob's whole-input shape.
    for i in 0..scale {
        let dir = root
            .join(".claude")
            .join("skills")
            .join(format!("skill-{i}"));
        fs::create_dir_all(dir.join("scripts")).unwrap();
        fs::write(
            dir.join("SKILL.md"),
            format!("---\nname: skill-{i}\ndescription: Synthetic skill {i} for the cost fixture.\n---\n# Skill {i}\n"),
        )
        .unwrap();
        fs::write(dir.join("REFERENCE.md"), format!("# Reference {i}\n")).unwrap();
        fs::write(dir.join("scripts").join("run.sh"), "#!/bin/sh\necho hi\n").unwrap();
        files += 3;
    }

    // Rules and commands: flat `*.md` loci.
    let rules = root.join(".claude").join("rules");
    let commands = root.join(".claude").join("commands");
    fs::create_dir_all(&rules).unwrap();
    fs::create_dir_all(&commands).unwrap();
    for i in 0..scale * 2 {
        fs::write(rules.join(format!("rule-{i}.md")), format!("# Rule {i}\n")).unwrap();
        fs::write(
            commands.join(format!("cmd-{i}.md")),
            format!("---\ndescription: Synthetic command {i}.\n---\n# Command {i}\n"),
        )
        .unwrap();
        files += 2;
    }

    // Agents: an any-depth `**/*.md` locus — its walk descends every level of the agents
    // subtree, so members nest one directory down.
    for i in 0..scale {
        let dir = root
            .join(".claude")
            .join("agents")
            .join(format!("team-{i}"));
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join(format!("agent-{i}.md")),
            format!("---\nname: agent-{i}\ndescription: Synthetic agent {i}.\n---\n# Agent {i}\n"),
        )
        .unwrap();
        files += 1;
    }

    // Memory: the `**/CLAUDE.md` root-`.` locus walks the *entire* tree, so scattered
    // nested memory files exercise the whole-input any-depth traversal.
    fs::write(root.join("CLAUDE.md"), "# Root memory\n").unwrap();
    files += 1;
    for i in 0..scale * 2 {
        let dir = root.join("packages").join(format!("pkg-{i}")).join("src");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("CLAUDE.md"), format!("# Package {i} memory\n")).unwrap();
        files += 1;
    }

    files
}

/// Discover every built-in kind's members exactly as `gate` does — one shared
/// [`Discovery`] threaded through a `governs` scan per kind and a per-host scan for the
/// nested-file kinds — returning the discovered files paired with the base each folds
/// against, for the read phase to consume.
fn discover_all(
    disc: &Discovery,
    harness: &Path,
) -> Vec<(kind::CustomKind, std::path::PathBuf, std::path::PathBuf)> {
    let kinds = builtin_kind::definitions().unwrap();
    let mut out = Vec::new();
    for kind in kinds.values() {
        match &kind.governs {
            Some(governs) => {
                let base = harness.join(&governs.root);
                for file in import::discover_kind_files(disc, kind, governs, LocalOverride::Honored)
                    .unwrap()
                {
                    out.push((kind.clone(), file, base.clone()));
                }
            }
            None => {
                for unit in import::discover_nested_file(disc, kind, &kinds, LocalOverride::Honored)
                    .unwrap()
                {
                    out.push((kind.clone(), unit.file, unit.host_unit));
                }
            }
        }
    }
    out
}

#[test]
fn check_cost_is_diagnosed_and_glob_compilation_is_pinned_per_distinct_glob() {
    // `scale` sets the member count per flat locus; the generated tree lands well past
    // ten thousand files — the consumer scale the field measured the residual at.
    let scale: usize = std::env::var("TEMPER_COST_SCALE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_700);
    let harness = tmpdir("check-cost");
    let build_start = Instant::now();
    let file_count = generate_harness(&harness, scale);
    let build_ms = build_start.elapsed().as_millis();

    assert!(
        file_count > 10_000,
        "the cost fixture must reach consumer scale; got {file_count} files",
    );

    // Phase 1 — the shared ignore-honoring tree walk, once per flavor (`import::Discovery`).
    let disc = Discovery::new(&harness);
    let walks_before = import::walk_count();
    let compiles_before = kind::glob_compile_count();
    let walk_start = Instant::now();
    let discovered = discover_all(&disc, &harness);
    let discover_ms = walk_start.elapsed().as_millis();
    let walks = import::walk_count() - walks_before;
    let compiles = kind::glob_compile_count() - compiles_before;

    // Phase 2 — read + hash every discovered member (the read-side phase).
    let read_files = discovered.len();
    let read_start = Instant::now();
    for (kind, file, base) in &discovered {
        // A member may fail to parse (a companion, a malformed body); the cost of the read
        // attempt is what we measure, so a failure is counted, not unwrapped.
        let _ = Member::from_source_rooted(kind, file, base);
    }
    let read_ms = read_start.elapsed().as_millis();

    // Coarse per-phase timing — a manual signal a human reads (the numbers land in the
    // commit body), never an asserted wall-clock bar.
    eprintln!("check-cost diagnosis over {file_count} files (build {build_ms} ms):");
    eprintln!(
        "  phase 1  discovery walk + per-kind scan : {discover_ms:>6} ms  ({walks} flavor walks, {compiles} glob compiles)"
    );
    eprintln!("  phase 2  read + hash {read_files:>6} members    : {read_ms:>6} ms");

    // The count-pin (`engineering.md`): whole-input glob compilation is hoisted per
    // distinct glob, never recomputed per candidate file. The discovery above tests one
    // leaf glob against every candidate name at every level of a >10k-file tree — without
    // the memo the compile count scales with the file count (tens of thousands); with it,
    // the count is the number of distinct loci globs the built-in kinds declare, a small
    // constant independent of tree size. A generous ceiling well below the file count
    // states the invariant decidably and machine-independently.
    assert!(
        compiles <= 32,
        "glob compilation must hoist per distinct glob, not per candidate file: \
         {compiles} compiles over {file_count} files (expected a small constant)",
    );

    // The shared walk is pinned at run granularity elsewhere; assert it here too so the
    // discovery phase's whole-tree walk is one-per-flavor at consumer scale.
    assert!(
        walks <= 2,
        "discovery must walk each flavor at most once, not per kind: {walks} walks",
    );
}
