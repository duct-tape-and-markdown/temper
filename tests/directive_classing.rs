//! Directive-target classing — the three verdicts.
//!
//! A directive occurrence yields an edge as a fact, resolved at check time against
//! provenance (`source_path` is the join key). Three classes, three verdicts:
//! - a target resolving to another **member** is a member→member observed edge (it
//!   enters the resolved-edge set, no finding);
//! - a target resolving to an ungoverned **repo file** is a backed boundary edge (no
//!   finding, no member edge);
//! - a target resolving to **nothing** is an **unbacked pointer** — the importing
//!   member's finding, the silent-context-loss failure class made author-time.
//!
//! Relative targets resolve against the importing file's directory; absolute targets
//! resolve as authored. This drives the library classer over constructed members, the
//! way `tests/graph.rs`'s `reachability` module drives `graph::reachable` — the check
//! wiring (`src/main.rs`) reuses this exact function over the imported corpus.
//!
//! The classing's edges are no longer dropped at the gate: they are the **import
//! relation** `graph::acyclic` is scoped to (`specs/model/contract.md`,
//! "well-formedness"), so the verdict split is asserted across the whole process
//! boundary at the bottom of this file — an unbacked pointer still warns without
//! failing the run, a member↔member ring reaches `graph.acyclic` and does.

use std::path::PathBuf;

use temper::check::Severity;
use temper::graph::{DirectiveMember, classify_directives};

mod common;

/// A member carrying a kind, an id, the provenance `source_path` the classing joins on,
/// and its `at-import` target occurrences in document order.
fn member(kind: &str, id: &str, source_path: &str, directives: &[&str]) -> DirectiveMember {
    DirectiveMember {
        kind: kind.to_string(),
        id: id.to_string(),
        source_path: PathBuf::from(source_path),
        directives: directives.iter().map(|s| (*s).to_string()).collect(),
    }
}

/// The repo file-set the backing class joins against — relative slash paths, as
/// `repo_file_set` collects them.
fn repo(files: &[&str]) -> Vec<String> {
    files.iter().map(|s| (*s).to_string()).collect()
}

#[test]
fn a_target_resolving_to_a_member_is_a_member_edge_and_no_finding() {
    // `docs/CLAUDE.md` imports `./shared.md`, which resolves (relative to the importing
    // file's directory `docs/`) to `docs/shared.md` — another member's `source_path`. A
    // member→member observed edge enters the resolved-edge set; nothing fires.
    let members = [
        member("memory", "root", "docs/CLAUDE.md", &["./shared.md"]),
        member("memory", "shared", "docs/shared.md", &[]),
    ];
    let classing = classify_directives(&members, &repo(&["docs/CLAUDE.md", "docs/shared.md"]));

    assert!(
        classing.findings.is_empty(),
        "a resolving member import is no finding, got: {:?}",
        classing.findings
    );
    assert_eq!(classing.edges.len(), 1);
    assert_eq!(
        classing.edges[0].from,
        ("memory".to_string(), "root".to_string())
    );
    assert_eq!(
        classing.edges[0].to,
        ("memory".to_string(), "shared".to_string())
    );
}

#[test]
fn a_target_backed_by_a_repo_file_is_no_finding_and_no_member_edge() {
    // `docs/CLAUDE.md` imports `./styleguide.md` → `docs/styleguide.md`, which is not a
    // member but *is* present in the repo file-set — a backed boundary edge. No finding,
    // and no member edge (the boundary is one-way, toward the world).
    let members = [member(
        "memory",
        "root",
        "docs/CLAUDE.md",
        &["./styleguide.md"],
    )];
    let classing = classify_directives(&members, &repo(&["docs/CLAUDE.md", "docs/styleguide.md"]));

    assert!(
        classing.findings.is_empty(),
        "a backed repo-file import is no finding, got: {:?}",
        classing.findings
    );
    assert!(
        classing.edges.is_empty(),
        "a backed repo file is not a member edge, got: {:?}",
        classing
            .edges
            .iter()
            .map(|e| (&e.from, &e.to))
            .collect::<Vec<_>>()
    );
}

#[test]
fn a_target_resolving_to_nothing_is_one_unbacked_pointer_finding() {
    // `docs/CLAUDE.md` imports `./ghost.md` → `docs/ghost.md`, which names no member and
    // no repo file — an unbacked pointer that loads nothing. Exactly one error finding,
    // keyed to the importing member, naming the target.
    let members = [member("memory", "root", "docs/CLAUDE.md", &["./ghost.md"])];
    let classing = classify_directives(&members, &repo(&["docs/CLAUDE.md"]));

    assert!(classing.edges.is_empty());
    assert_eq!(classing.findings.len(), 1);
    let finding = &classing.findings[0];
    assert_eq!(finding.severity, Severity::Error);
    assert_eq!(finding.rule, "graph.directive-unbacked");
    assert_eq!(finding.artifact, "root");
    assert!(
        finding.message.contains("ghost.md"),
        "the finding names the dead target, got: {}",
        finding.message
    );
}

#[test]
fn relative_targets_resolve_against_the_importing_files_directory() {
    // The importing file lives at `docs/team/CLAUDE.md`; `../shared.md` climbs out of
    // `docs/team/` to `docs/shared.md` (a member), while `./local.md` stays in
    // `docs/team/` and resolves to a repo file. The parent-directory resolution is what
    // distinguishes the two — a naive root-relative resolve would misclass both.
    let members = [
        member(
            "memory",
            "team",
            "docs/team/CLAUDE.md",
            &["../shared.md", "./local.md"],
        ),
        member("memory", "shared", "docs/shared.md", &[]),
    ];
    let classing = classify_directives(
        &members,
        &repo(&[
            "docs/team/CLAUDE.md",
            "docs/shared.md",
            "docs/team/local.md",
        ]),
    );

    assert!(
        classing.findings.is_empty(),
        "both relative imports resolve, got: {:?}",
        classing.findings
    );
    // `../shared.md` → the member `shared`; `./local.md` → a backed repo file (no edge).
    assert_eq!(classing.edges.len(), 1);
    assert_eq!(
        classing.edges[0].to,
        ("memory".to_string(), "shared".to_string())
    );
}

#[test]
fn an_absolute_target_resolves_as_authored_not_against_the_importing_dir() {
    // An absolute `@/root/base.md` ignores the importing file's directory and resolves to
    // itself — joining the member at that exact `source_path` (absolute allowed;
    // code.claude.com/docs/en/memory).
    let members = [
        member("memory", "root", "docs/CLAUDE.md", &["/root/base.md"]),
        member("memory", "base", "/root/base.md", &[]),
    ];
    let classing = classify_directives(&members, &repo(&["docs/CLAUDE.md"]));

    assert!(classing.findings.is_empty());
    assert_eq!(classing.edges.len(), 1);
    assert_eq!(
        classing.edges[0].to,
        ("memory".to_string(), "base".to_string())
    );
}

#[test]
fn the_three_verdicts_partition_one_members_occurrences() {
    // One importing member with three occurrences, one of each class: a member import, a
    // backed repo-file import, and an unbacked pointer. The classer partitions them — one
    // edge, one finding — proving the three verdicts are decided per occurrence.
    let members = [
        member(
            "memory",
            "root",
            "docs/CLAUDE.md",
            &["./shared.md", "./styleguide.md", "./ghost.md"],
        ),
        member("memory", "shared", "docs/shared.md", &[]),
    ];
    let classing = classify_directives(
        &members,
        &repo(&["docs/CLAUDE.md", "docs/shared.md", "docs/styleguide.md"]),
    );

    assert_eq!(
        classing.edges.len(),
        1,
        "the member import is the sole edge"
    );
    assert_eq!(
        classing.edges[0].to,
        ("memory".to_string(), "shared".to_string())
    );
    assert_eq!(classing.findings.len(), 1, "the ghost is the sole finding");
    assert_eq!(classing.findings[0].artifact, "root");
    assert!(classing.findings[0].message.contains("ghost.md"));
}

#[test]
fn a_backing_repo_file_is_found_with_absolute_harness_root() {
    use std::fs;
    use std::path::Path;
    use temper::compose;
    use temper::path;

    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    let subdir = root.join("src");
    fs::create_dir_all(&subdir).expect("create subdir");
    fs::write(root.join("CLAUDE.md"), "root").expect("write root file");
    fs::write(subdir.join("helper.md"), "helper").expect("write helper file");

    // Member imports a repo file using a relative path.
    let member_path = subdir
        .join("CLAUDE.md")
        .to_string_lossy()
        .replace('\\', "/");
    let members = [member("memory", "root", &member_path, &["./helper.md"])];

    // repo_file_set with absolute root produces absolute paths.
    let repo_files = compose::repo_file_set(root);

    // Normalize the repo files in the same way classify_directives does.
    let repo_files_normalized: Vec<String> = repo_files
        .iter()
        .map(|f| {
            path::normalize_path(Path::new(f))
                .to_string_lossy()
                .replace('\\', "/")
                .to_string()
        })
        .collect();

    let classing = classify_directives(&members, &repo_files_normalized);

    assert!(
        classing.findings.is_empty(),
        "an absolute-root backed repo-file import is no finding, got: {:?}",
        classing.findings
    );
}

/// Write a repo-root `CLAUDE.md` whose body carries `import_line` on its own line — a
/// `memory` member, the one built-in kind composing the `at-import` directive
/// primitive.
fn write_memory(root: &std::path::Path, rel: &str, import_line: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create memory dir");
    }
    std::fs::write(&path, format!("# Memory\n\nGuidance.\n\n{import_line}\n"))
        .expect("write memory member");
}

#[test]
fn an_unbacked_pointer_warns_without_failing_the_run() {
    // The FLOOR-tier verdict is unchanged by the gate now reading the classing's edges:
    // an unbacked `@import` is a pure fact about the importing member, extended as a
    // non-gating advisory (WEDGE ruling 2026-07-03). It warns; the run still exits zero.
    let root = common::tmpdir("directive-unbacked-warns");
    common::write_skill(&root, "standards", &common::clean_skill("standards"));
    write_memory(&root, "CLAUDE.md", "@./ghost.md");

    let run = common::check_in(&root, &["."], Some("github"));
    let findings = run.findings();
    assert!(
        run.ok,
        "an unbacked pointer is advisory ⇒ zero, got:\n{}",
        run.output
    );
    let unbacked = common::findings_for(&findings, "graph.directive-unbacked");
    assert_eq!(
        unbacked.len(),
        1,
        "exactly one unbacked-pointer warning, got: {findings:#?}"
    );
    assert!(
        unbacked[0].starts_with("::warning"),
        "the unbacked pointer is a warning, not an error, got: {}",
        unbacked[0]
    );
}

#[test]
fn a_member_to_member_ring_reaches_the_acyclicity_gate() {
    // The other half of the split: every occurrence in the ring resolves to a member, so
    // no unbacked-pointer finding fires at all — the edges the classing yields carry the
    // verdict instead, as the import relation `graph::acyclic` is founded on.
    let root = common::tmpdir("directive-ring-gates");
    common::write_skill(&root, "standards", &common::clean_skill("standards"));
    write_memory(&root, "CLAUDE.md", "@docs/CLAUDE.md");
    write_memory(&root, "docs/CLAUDE.md", "@../CLAUDE.md");

    let run = common::check_in(&root, &["."], Some("github"));
    let findings = run.findings();
    assert!(
        common::findings_for(&findings, "graph.directive-unbacked").is_empty(),
        "both imports resolve to members, so nothing is unbacked, got: {findings:#?}"
    );
    let acyclic = common::findings_for(&findings, "graph.acyclic");
    assert_eq!(
        acyclic.len(),
        1,
        "the ring fires exactly one acyclicity finding, got: {findings:#?}"
    );
    assert!(
        acyclic[0].starts_with("::error"),
        "acyclicity is well-formedness — an error, never a dialable advisory, got: {}",
        acyclic[0]
    );
    assert!(
        !run.ok,
        "the ring fails the run ⇒ non-zero, got:\n{}",
        run.output
    );
}
