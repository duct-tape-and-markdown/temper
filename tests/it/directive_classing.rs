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

use std::path::{Path, PathBuf};

use temper::check::Severity;
use temper::graph::{DirectiveMember, backing_in_set, classify_directives};

use crate::common;

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

/// The fixed-set backing flavor the classing asks about each cited target — relative
/// slash paths, as `repo_file_set` spells them. The stat flavor
/// (`compose::backed_on_disk`) is what a run resolves through; the cases at the bottom
/// of this file pin that the two agree verdict for verdict.
fn repo(files: &[&str]) -> impl Fn(&Path) -> bool + use<> {
    let files: Vec<String> = files.iter().map(|s| (*s).to_string()).collect();
    backing_in_set(&files)
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

/// A `memory` member at `dir/CLAUDE.md` importing `target` — the shape every stat-flavor
/// case below drives, spelling the importing file's absolute path so the resolution joins
/// the absolute harness root the resolver is built over.
fn importer_in(dir: &Path, target: &str) -> [DirectiveMember; 1] {
    let source = dir.join("CLAUDE.md").to_string_lossy().replace('\\', "/");
    [member("memory", "root", &source, &[target])]
}

#[test]
fn a_backing_repo_file_is_found_by_stat_with_an_absolute_harness_root() {
    use std::fs;
    use temper::compose;

    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    let subdir = root.join("src");
    fs::create_dir_all(&subdir).expect("create subdir");
    fs::write(root.join("CLAUDE.md"), "root").expect("write root file");
    fs::write(subdir.join("helper.md"), "helper").expect("write helper file");

    // The resolved target is absolute (the importing file's is), so the resolver's own
    // root must be — the path-domain join the walk's normalization used to make.
    let classing = classify_directives(
        &importer_in(&subdir, "./helper.md"),
        &compose::backed_on_disk(root),
    );

    assert!(
        classing.findings.is_empty(),
        "an absolute-root backed repo-file import is no finding, got: {:?}",
        classing.findings
    );
}

#[test]
fn a_gitignored_sibling_is_backed_by_stat() {
    use std::fs;
    use temper::compose;

    // The backing set is **raw disk**, never the discovery view: a path-resolved edge
    // reads what is there (`specs/model/contract.md`, "edge"), so an ignored file backs
    // its import exactly as a tracked one does. Resolution by stat keeps that ruling —
    // `fs::symlink_metadata` knows nothing of `.gitignore`.
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    fs::write(root.join(".gitignore"), "notes.md\n").expect("write gitignore");
    fs::write(root.join("notes.md"), "private").expect("write ignored sibling");

    let classing = classify_directives(
        &importer_in(root, "./notes.md"),
        &compose::backed_on_disk(root),
    );

    assert!(
        classing.findings.is_empty(),
        "a gitignored sibling backs its import, got: {:?}",
        classing.findings
    );
}

#[test]
fn a_directory_target_is_unbacked_by_stat() {
    use std::fs;
    use temper::compose;

    // The walk collects `file_type().is_file()` entries only, so a directory was never in
    // the set — an `@docs` import loads nothing and says so.
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    fs::create_dir_all(root.join("docs")).expect("create dir target");
    fs::write(root.join("docs").join("page.md"), "page").expect("write file under it");

    let classing =
        classify_directives(&importer_in(root, "./docs"), &compose::backed_on_disk(root));

    assert_eq!(
        classing.findings.len(),
        1,
        "a directory target is unbacked, got: {:?}",
        classing.findings
    );
    assert_eq!(classing.findings[0].rule, "graph.directive-unbacked");
}

/// Unix only: the verdict under test is what a symlink's *own* file type decides, and
/// Windows needs `SeCreateSymbolicLinkPrivilege` to create one at all (`common::vendor_sdk`
/// carries the same split).
#[cfg(unix)]
#[test]
fn a_symlink_target_is_unbacked_by_stat() {
    use std::fs;
    use temper::compose;

    // The walk runs with `follow_links` off, so it sees a symlink's own file type and
    // collects neither the link nor anything under a linked directory. The stat flavor
    // lstats every segment for exactly that reason.
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    fs::write(root.join("real.md"), "real").expect("write link target");
    std::os::unix::fs::symlink(root.join("real.md"), root.join("link.md")).expect("link file");
    fs::create_dir_all(root.join("real-dir")).expect("create linked dir");
    fs::write(root.join("real-dir").join("page.md"), "page").expect("write under linked dir");
    std::os::unix::fs::symlink(root.join("real-dir"), root.join("link-dir")).expect("link dir");

    for target in ["./link.md", "./link-dir/page.md"] {
        let classing =
            classify_directives(&importer_in(root, target), &compose::backed_on_disk(root));
        assert_eq!(
            classing.findings.len(),
            1,
            "`{target}` resolves through a symlink the walk never entered, got: {:?}",
            classing.findings
        );
        assert_eq!(classing.findings[0].rule, "graph.directive-unbacked");
    }

    // Non-vacuity: the same corpus over the real file the links point at is backed, so
    // the two findings above are the symlink's verdict and not a broken fixture.
    let real = classify_directives(
        &importer_in(root, "./real.md"),
        &compose::backed_on_disk(root),
    );
    assert!(
        real.findings.is_empty(),
        "the link's own target is backed, got: {:?}",
        real.findings
    );
}

#[test]
fn a_target_outside_the_harness_root_is_unbacked_by_stat() {
    use std::fs;
    use temper::compose;

    // The walk starts at the harness root, so a file above it was never in the set
    // whatever stands at that path — a `../` import out of the harness loads nothing the
    // harness can answer for.
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let outside = temp_dir.path();
    let root = outside.join("harness");
    fs::create_dir_all(&root).expect("create harness root");
    fs::write(outside.join("elsewhere.md"), "elsewhere").expect("write file above the root");
    fs::write(root.join("inside.md"), "inside").expect("write file under the root");

    let classing = classify_directives(
        &importer_in(&root, "../elsewhere.md"),
        &compose::backed_on_disk(&root),
    );

    assert_eq!(
        classing.findings.len(),
        1,
        "a target above the harness root is unbacked, got: {:?}",
        classing.findings
    );
    assert_eq!(classing.findings[0].rule, "graph.directive-unbacked");

    // Non-vacuity: the sibling *under* the root, written by the same fixture, is backed.
    let inside = classify_directives(
        &importer_in(&root, "./inside.md"),
        &compose::backed_on_disk(&root),
    );
    assert!(
        inside.findings.is_empty(),
        "a target under the root is backed, got: {:?}",
        inside.findings
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
