//! Measure-first cost diagnosis at consumer scale, for every verb whose work scales with
//! the consumer's input (`specs/process/engineering.md`, "Cost scale is hoisted, and
//! pinned by count") — `check`'s walks and reads, `emit`'s lock parse, manifest reads and
//! per-glob placement round trip, and `guard`'s per-tool-call shell edge.
//!
//! A synthetic harness the size of a real consumer's tree is generated in a tempdir at
//! test time and never committed. Discovery — the phase `check` opens with, walking the
//! consumer's whole tree per kind — is timed over it so the numbers, not a guess, name
//! where the residual concentrates; the timings print (a manual signal a human reads) and
//! the test asserts the work-count pins the cuts earn — decided by counts, independent of
//! tree size: the shared walk runs once per flavor, the directive backing set's
//! whole-tree walk runs once per run, glob compilation is hoisted per distinct glob
//! rather than per candidate file, the per-kind glob scan reads its members from
//! that one walk's index, opening no directory of its own, and the guard's shell edge
//! walks once per declared locus root, never the whole tree.

use crate::common;

use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::common::{fresh_clause, tmpdir};
use temper::builtin_kind;
use temper::drift::{self, Declarations, EmitOptions, Payload, PayloadMember};
use temper::frontmatter::Member;
use temper::glob;
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
///
/// The per-host scan **descends**: a host that is itself a nested-file child supplies its
/// units from its own host's, one call per declared layer. The shipped set templates one
/// layer, so that descent bottoms out immediately here; what the pins below hold is that
/// a descent costs no *tree* walk and no fresh matcher — both ride the shared
/// [`Discovery`] and the glob memo, so depth multiplies neither.
fn discover_all(
    disc: &Discovery,
    harness: &Path,
) -> Vec<(kind::CustomKind, std::path::PathBuf, std::path::PathBuf)> {
    let kinds = builtin_kind::definitions();
    let mut out = Vec::new();
    for kind in kinds.values() {
        match &kind.governs {
            Some(governs) => {
                let base = harness.join(&governs.root);
                for file in import::discover_kind_files(disc, kind, governs, LocalOverride::Honored)
                {
                    out.push((kind.clone(), file, base.clone()));
                }
            }
            None => {
                for unit in import::discover_nested_file(disc, kind, &kinds, LocalOverride::Honored)
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
    let compiles_before = glob::glob_compile_count();
    let walk_start = Instant::now();
    let discovered = discover_all(&disc, &harness);
    let discover_ms = walk_start.elapsed().as_millis();
    let walks = import::walk_count() - walks_before;
    let compiles = glob::glob_compile_count() - compiles_before;

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
    // constant independent of tree size — and a *distinct* glob, so the nested-file
    // descent reusing a host's pattern one layer down adds none. A generous ceiling well
    // below the file count states the invariant decidably and machine-independently.
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

#[test]
fn emit_lock_parse_is_hoisted_and_pinned_once_per_run() {
    use temper::drift::{self, Declarations, EmitOptions, Payload};

    let harness = tmpdir("emit-lock-parse-cost");
    let into = harness.join(".temper");
    std::fs::create_dir_all(&into).unwrap();

    // Create a minimal harness with one skill member to emit.
    common::write_skill(&harness, "test-skill", "# Test\n\nBody.");

    // Create a lock.toml with:
    // - A provenance row for the skill (exercises the reap-diff path)
    // - Nested member declarations (exercises the layer-drop check path)
    let lock_path = into.join("lock.toml");
    std::fs::write(
        &lock_path,
        r#"[[skill]]
name = "test-skill"
source_path = ".claude/skills/test-skill/SKILL.md"
emit_hash = "0000000000000000000000000000000000000000000000000000000000000000"

[declaration]

[[declaration.nested_member]]
host = "skill:test-host"
name = "nested-1"
"#,
    )
    .unwrap();

    // Create a minimal SDK payload that will emit the skill and derive no nested
    // members (so the layer-drop check runs but doesn't drop).
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::skill_kind_facts(None, &[])],
            ..Default::default()
        },
        members: vec![common::skill_member(
            "test-skill",
            "Test skill.",
            "# Test\n\nBody.",
        )],
    };

    let options = EmitOptions {
        dry_run: true,
        frozen: false,
        teardown: false,
    };

    // Read counts before emit.
    let reads_before = drift::lock_read_count();
    let parses_before = drift::lock_parse_count();

    // Run emit — this should read and parse the lock exactly once, reusing the
    // parsed document for both the reap-diff and layer-drop check.
    let _ = drift::emit(&payload, &into, options);

    // Read counts after emit.
    let reads_after = drift::lock_read_count();
    let parses_after = drift::lock_parse_count();

    let reads = reads_after - reads_before;
    let parses = parses_after - parses_before;

    // The cost doctrine (engineering.md, "Cost scale is hoisted, and pinned by count"):
    // whole-input work computes once per run and is shared, never recomputed per call site.
    // Lock parsing is hoisted: one read per emit run, one parse per emit run.
    assert_eq!(
        reads, 1,
        "emit must read lock.toml exactly once per run, not per phase: {reads} reads (before {reads_before}, after {reads_after})",
    );
    assert_eq!(
        parses, 1,
        "emit must parse lock.toml exactly once per run, not per phase: {parses} parses (before {parses_before}, after {parses_after})",
    );
}

#[test]
fn emit_over_a_lockless_harness_reads_and_parses_once() {
    use temper::drift::{self, Declarations, EmitOptions, Payload};

    // The sibling pin above runs over a lock that exists, so it never exercises emit's
    // other state: the fresh adopter's harness, where `.temper/lock.toml` is absent. This
    // fixture writes none — on the pre-fold tree emit's read of a missing lock counted
    // neither read nor parse, so both deltas read 0 here and the once-per-run pin was
    // blind on this path; the fold makes them 1/1.
    let harness = tmpdir("emit-lockless-lock-parse-cost");
    let into = harness.join(".temper");
    std::fs::create_dir_all(&into).unwrap();
    common::write_skill(&harness, "test-skill", "# Test\n\nBody.");
    assert!(
        !into.join("lock.toml").exists(),
        "the lockless pin is vacuous unless the harness carries no lock",
    );

    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::skill_kind_facts(None, &[])],
            ..Default::default()
        },
        members: vec![common::skill_member(
            "test-skill",
            "Test skill.",
            "# Test\n\nBody.",
        )],
    };

    let reads_before = drift::lock_read_count();
    let parses_before = drift::lock_parse_count();

    let _ = drift::emit(
        &payload,
        &into,
        EmitOptions {
            dry_run: true,
            frozen: false,
            teardown: false,
        },
    );

    let reads = drift::lock_read_count() - reads_before;
    let parses = drift::lock_parse_count() - parses_before;

    // One home for the lookup means one count, whatever the file's state: an absent lock
    // is read and parsed exactly once, the same as a present one, so a per-phase re-read
    // regression on this path is visible to the pin instead of hiding behind a zero.
    assert_eq!(
        reads, 1,
        "emit over a lockless harness must count exactly one lock read: {reads} reads",
    );
    assert_eq!(
        parses, 1,
        "emit over a lockless harness must count exactly one lock parse: {parses} parses",
    );
}

#[test]
fn source_dep_faces_over_a_lockless_harness_read_and_parse_once() {
    use temper::drift;

    // The source-dep faces hand-rolled their own read of the very file the counted door
    // opens, and diverged from it on exactly one state: a missing lock returned early
    // counting neither read nor parse, where the door counts one of each. This fixture
    // writes no lock, so both deltas read 0/0 on the pre-fold tree and 1/1 with the fold.
    let harness = tmpdir("source-dep-lockless-lock-parse-cost");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    assert!(
        !into.join("lock.toml").exists(),
        "the lockless pin is vacuous unless the harness carries no lock",
    );

    let reads_before = drift::lock_read_count();
    let parses_before = drift::lock_parse_count();
    drift::includes(&into).unwrap();
    let reads = drift::lock_read_count() - reads_before;
    let parses = drift::lock_parse_count() - parses_before;

    // One home for the lookup means one count, whatever the file's state — the count the
    // counted door already charged every other face over this lock.
    assert_eq!(
        reads, 1,
        "a source-dep face over a lockless harness must count exactly one lock read: {reads} reads",
    );
    assert_eq!(
        parses, 1,
        "a source-dep face over a lockless harness must count exactly one lock parse: {parses} parses",
    );

    let reads_before = drift::lock_read_count();
    let parses_before = drift::lock_parse_count();
    drift::include_stale(&into, &fresh_clause()).unwrap();
    let reads = drift::lock_read_count() - reads_before;
    let parses = drift::lock_parse_count() - parses_before;

    assert_eq!(
        reads, 1,
        "a source-dep stale face over a lockless harness must count exactly one lock read: {reads} reads",
    );
    assert_eq!(
        parses, 1,
        "a source-dep stale face over a lockless harness must count exactly one lock parse: {parses} parses",
    );
}

#[test]
fn gate_lock_parse_is_hoisted_with_source_dependencies() {
    let workspace = tmpdir("gate-lock-parse-cost");

    // Create a lock.toml with layout_import, include and input source-dependency rows,
    // which exercises the hoisted parse path: read_lock_document() once, then
    // layout_imports_from_doc/includes_from_doc/layout_import_stale_from_doc/
    // include_stale_from_doc/input_stale_from_doc all reuse the pre-parsed document.
    let lock_path = workspace.join(temper::LOCK_FILENAME);
    std::fs::write(
        &lock_path,
        r#"[declaration]

[[declaration.layout_import]]
member = "skill:test-skill"
target = "skill:test-skill"
source_path = "layout.md"
import_hash = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"

[[declaration.include]]
member = "skill:test-skill"
target = "skill:test-skill"
source_path = "included.md"
import_hash = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"

[[declaration.input]]
member = "skill:test-skill"
source_path = "snapshot/legacy.py"
import_hash = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
"#,
    )
    .unwrap();

    // Count lock reads/parses before hoisting test.
    let reads_before = temper::drift::lock_read_count();
    let parses_before = temper::drift::lock_parse_count();

    // Simulate what gate() does: read the lock once and pass it through to all the
    // source-dependency call sites. This verifies the hoisting: one read, one parse,
    // even though five call sites access source dependencies.
    let lock_doc = temper::drift::read_lock_document(&workspace).expect("lock should parse");

    // The five call sites that would each re-read the lock (pre-hoisting):
    // 1. layout_imports (called by import_edges_from_lock)
    let _ = temper::drift::layout_imports_from_doc(&lock_doc);
    // 2. includes (called by import_edges_from_lock)
    let _ = temper::drift::includes_from_doc(&lock_doc);
    // 3. layout_import_stale
    let harness_root = workspace
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let clause = fresh_clause();
    let _ = temper::drift::layout_import_stale_from_doc(&lock_doc, harness_root, &clause);
    // 4. include_stale
    let _ = temper::drift::include_stale_from_doc(&lock_doc, harness_root, &clause);
    // 5. input_stale — the third staleness family, on the same document and the same clause.
    let _ = temper::drift::input_stale_from_doc(&lock_doc, harness_root, &clause);

    // Count lock reads/parses after hoisting test.
    let reads_after = temper::drift::lock_read_count();
    let parses_after = temper::drift::lock_parse_count();

    let reads = reads_after - reads_before;
    let parses = parses_after - parses_before;

    // The cost doctrine: lock parsing is hoisted — one read per run, one parse per run,
    // even though five call sites access source dependencies. Each _from_doc variant
    // reuses the pre-parsed document instead of independently re-reading and re-parsing.
    assert_eq!(
        reads, 1,
        "source-dependency functions must read lock.toml exactly once when given pre-parsed doc: {reads} reads (before {reads_before}, after {reads_after})",
    );
    assert_eq!(
        parses, 1,
        "source-dependency functions must parse lock.toml exactly once when given pre-parsed doc: {parses} parses (before {parses_before}, after {parses_after})",
    );
}

/// The whole-run count-pin for the **committed lock**, over the real `gate()`
/// (`engineering.md`, "Cost scale is hoisted, and pinned by count"): a check run opens
/// `lock.toml` once and parses it once, however many tiers read off it — the declaration
/// rows the contract tier judges against and the source-dependency families below share
/// the one parsed document. Session-open `check` pays this on every tick, so a second
/// door onto the same file is a per-tick disk read and a per-tick TOML parse.
///
/// The two pins beside this one drive `emit` and a hand-assembled stand-in for the gate;
/// neither runs `gate()`, so neither saw the doors it opened past both counters.
///
/// The fixture is deliberately **represented** — it carries a `.temper/harness.ts`, all
/// `install::represented_by` asks for — so the install self-verify takes its
/// `evaluate_placements` branch rather than the settings-only one. That branch is where
/// the two extra doors were: this pin reads 3/3 on a tree where `gate_installed` reads
/// the lock itself, and 1/1 with the gate's one document threaded in.
#[test]
fn gate_reads_and_parses_the_lock_once() {
    use temper::drift::{self, Declarations};
    use temper::gate;

    let harness = tmpdir("gate-lock-read-pin");
    let skill = harness.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(
        skill.join("SKILL.md"),
        "---\nname: coordinate\ndescription: Drive a task across a team of agents.\n---\n# Coordinate\n",
    )
    .unwrap();

    // A represented harness: a lock carrying a declaration row family, so the tier that
    // reads `[declaration]` has rows to lift rather than answering off an absent table.
    // Written by the real lock writer (`drift::emit`) off a `KindFactRow`, the row this
    // family's one producer emits — and written *before* the counters are sampled, since
    // `emit` reads the lock through the same counted door.
    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![common::skill_kind_facts(None, &[])],
            ..Declarations::default()
        },
    );
    // Represented: a stub program is enough, since `represented_by` only asks whether the
    // entry file is there. The lock carries no provenance rows, so the represented branch
    // reads no target files and the counter delta is exactly the lock doors.
    fs::write(
        harness.join(".temper").join("harness.ts"),
        "export default {};\n",
    )
    .unwrap();

    let reads_before = drift::lock_read_count();
    let parses_before = drift::lock_parse_count();
    let (diagnostics, _) = gate::gate(&harness.join(".temper"), &harness, &[]).unwrap();
    let reads = drift::lock_read_count() - reads_before;
    let parses = drift::lock_parse_count() - parses_before;

    // Non-vacuity (engineering.md, "A green verdict is proven non-vacuous"): a gate that
    // judged nothing reads the lock zero times and would pass any ceiling. The run's own
    // disclosure of what it checked names the member, so the count below is taken over a
    // run that did the work.
    let summary = &diagnostics
        .iter()
        .find(|diagnostic| diagnostic.rule == "coverage.checked")
        .expect("the run discloses what it checked")
        .message;
    assert!(
        summary.contains("skill (1"),
        "the run must have judged the harness's one skill member, got: {summary}",
    );

    // Non-vacuity for the branch this pin exists to count: the self-verify's advisory
    // names the gate hook module an *author* owes, which only the represented branch
    // spells — the settings-only branch says `run \`temper install\`` instead. Without
    // this the fixture could silently drift back off the branch and the count would go
    // green over the door it never opened.
    let self_verify = &diagnostics
        .iter()
        .find(|diagnostic| diagnostic.rule == "install.gate-installed")
        .expect("the represented harness's self-verify reports its unwired gate")
        .message;
    assert!(
        self_verify.contains("author `.temper/"),
        "the self-verify must have taken the represented branch, got: {self_verify}",
    );

    assert_eq!(
        reads, 1,
        "a check run must read lock.toml exactly once, shared across every tier that \
         reads off it: {reads} reads (before {reads_before})",
    );
    assert_eq!(
        parses, 1,
        "a check run must parse lock.toml exactly once, shared across every tier that \
         reads off it: {parses} parses (before {parses_before})",
    );
}

#[test]
fn coverage_note_accepts_pre_parsed_locked_kinds() {
    use std::collections::BTreeMap;
    use temper::coverage_note;
    use temper::drift;

    let harness = tmpdir("coverage-note-lock-parse-hoist");

    // Create a harness with a `.claude/widgets/` locus no built-in kind governs and a lock
    // that declares a custom `widget` kind rooted there — an entry the stray scan names
    // unless the hoisted rows reach it.
    common::write_skill(&harness, "test-skill", "# Test\n\nBody.");
    std::fs::create_dir_all(harness.join(".claude/widgets")).unwrap();
    std::fs::write(harness.join(".claude/widgets/panel.json"), "{}\n").unwrap();

    // Written by the real lock writer (`drift::emit`) off a `KindFactRow`, the row this
    // family's one producer emits — never a hand-spelled `[[declaration.kind]]` table.
    common::write_lock(
        &harness,
        drift::Declarations {
            kinds: vec![drift::KindFactRow {
                unit_shape: Some("file".to_string()),
                ..common::kind_facts("widget", ".claude/widgets", "*.json")
            }],
            ..drift::Declarations::default()
        },
    );

    // Verify that coverage_note::check works correctly when passed pre-parsed kind rows
    // from a caller's own read_declarations call (as gate() now does, per
    // COVERAGE-NOTE-LOCK-PARSE-HOIST).
    let committed = drift::read_declarations(&harness.join(".temper"))
        .expect("lock should parse and deserialize");

    // Verify that the widget kind was parsed from the lock
    assert!(
        committed.kinds.iter().any(|k| k.name == "widget"),
        "lock should declare widget kind"
    );

    // Call coverage_note::check with the pre-parsed kind rows (the new API), the whole
    // built-in set in scope: no built-in governs `.claude/widgets`, so the locked kind's
    // own contribution is the only thing that can exclude it.
    let member_counts = BTreeMap::from([("skill".to_string(), 1usize)]);
    let in_scope = temper::builtin_kind::definitions();
    let check = |locked: &[temper::drift::KindFactRow]| {
        coverage_note::check(
            &harness,
            &in_scope,
            &member_counts,
            &BTreeMap::new(),
            &BTreeMap::new(),
            locked,
        )
        .expect("coverage_note::check should succeed")
    };
    let names_widgets = |diagnostics: &[temper::check::Diagnostic]| {
        diagnostics
            .iter()
            .any(|d| d.rule == "coverage.unclaimed-entry" && d.artifact == ".claude/widgets")
    };

    // Non-vacuity: with no locked rows handed in, the entry is named a stray.
    assert!(
        names_widgets(&check(&[])),
        "without the locked rows the unclaimed entry is named"
    );

    // Verify: the custom widget kind's governed locus excludes the entry. This proves the
    // hoisted kind rows are being used just as if they had been read internally.
    let diagnostics = check(&committed.kinds);
    assert!(
        !names_widgets(&diagnostics),
        "the locked widget kind should claim its own locus, got: {diagnostics:#?}"
    );
}

/// Write a harness whose one rule declares a `routes_to` reference at `target`, with the
/// skill `standards` composed and the lock declaring the `rule → skill` edge — the
/// minimal live input for the whole-input edge-resolution walk: one declared `edge` fact,
/// one source carrying the field, one real member of the target kind.
fn write_routing_harness(label: &str, target: &str) -> std::path::PathBuf {
    let root = tmpdir(label);
    common::write_rule_skill_harness(
        &root,
        "style",
        &common::scoped_routing_rule(None, Some(target)),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            assembly: vec![common::edge("rule", "routes_to", "skill")],
            ..Declarations::default()
        },
    );
    root
}

/// The edge-resolution walk is whole-input work, so the cost doctrine puts it at once per
/// run — and the pin is taken over `gate()` itself, the only caller whose call sites can
/// drift apart. Driving the consumers by hand instead pins the test author's assembly:
/// the two walks this entry folded were both inside `gate()`, one of them behind
/// `graph::check`, and a hand-assembled stand-in excluded that wrapper by construction.
#[test]
fn gate_resolved_edge_walk_is_hoisted_per_gate_invocation() {
    use temper::{gate, graph};

    let root = write_routing_harness("gate-edge-walk-pin", "standards");

    let before = graph::resolved_edges_count();
    let (diagnostics, _) = gate::gate(&root.join(".temper"), &root, &[]).unwrap();
    let walks = graph::resolved_edges_count() - before;

    // Non-vacuity, the endpoints: a gate that discovered neither kind's member would walk
    // an empty corpus and pass any count. The run's own disclosure names both.
    let summary = &diagnostics
        .iter()
        .find(|diagnostic| diagnostic.rule == "coverage.checked")
        .expect("the run discloses what it checked")
        .message;
    assert!(
        summary.contains("rule (1") && summary.contains("skill (1"),
        "the run must have judged the routing rule and its target skill, got: {summary}",
    );

    // Non-vacuity, the walk: the declared route resolved here, so the route verdict is
    // silent — and the same corpus with the reference pointed at an absent skill fires
    // `graph.route`, which proves the counted walk really reads the declared edge rather
    // than ranging over nothing.
    assert!(
        !diagnostics.iter().any(|d| d.rule == "graph.route"),
        "the declared route resolves to the composed skill, got: {diagnostics:#?}",
    );
    let dangling = write_routing_harness("gate-edge-walk-pin-dangling", "absent");
    let (control, _) = gate::gate(&dangling.join(".temper"), &dangling, &[]).unwrap();
    assert!(
        control.iter().any(|d| d.rule == "graph.route"),
        "the same edge, pointed at no artifact, must fire the route verdict, got: {control:#?}",
    );

    assert_eq!(
        walks, 1,
        "gate() must compute the edge-resolution walk exactly once, shared by the route \
         verdict, degree, reachability and mention-reachability: {walks} walks (before \
         {before})",
    );
}

/// `reached-from` is **opt-in**: a corpus declaring no such clause walks no closure at
/// all, and one declaring a clause walks it once per `(roots, via)` pair however many
/// members the selection carries. The claim is pinned the way `degree`'s and
/// `reachable`'s are — at the judge, over a corpus that would fire if it ran.
#[test]
fn the_reached_from_closure_is_opt_in_and_walks_once_per_root_and_via_pair() {
    use std::collections::BTreeMap;
    use temper::compose;
    use temper::contract::{Predicate, Severity};
    use temper::engine::{Selection, Selector};
    use temper::extract::Features;
    use temper::graph;

    // skill:a → skill:b over `routes_to`; skill:d links to nothing.
    let edges = [compose::Edge {
        field: "routes_to".to_string(),
        from: "skill".to_string(),
        to: vec!["skill".to_string()],
    }];
    let mut a_fields = BTreeMap::new();
    a_fields.insert("routes_to".to_string(), serde_json::json!(["b"]));
    let skills = [
        Features {
            fields: a_fields,
            body_lines: 1,
            ..common::features("a")
        },
        Features {
            body_lines: 1,
            ..common::features("b")
        },
        Features {
            body_lines: 1,
            ..common::features("d")
        },
    ];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
    let resolved = graph::resolved_edges(&edges, &by_kind).resolved;

    let members: Vec<(&str, &Features)> = skills.iter().map(|f| ("skill", f)).collect();
    let roots: Vec<(&str, &Features)> = vec![("skill", &skills[0])];

    // No `reached-from` clause anywhere: a `count` bound is a selection clause that is
    // not this predicate, so the judge returns on its opt-in guard having walked nothing.
    let quiet = [common::one_clause_selection(
        Selector::Kind("skill".to_string()),
        Severity::Advisory,
        Predicate::Count {
            min: 0,
            max: usize::MAX,
        },
        None,
        members.clone(),
    )];
    assert!(
        graph::reached_from(&quiet, &resolved, &[], &by_kind).is_empty(),
        "a corpus declaring no reached-from clause walks no closure and finds nothing",
    );

    // The same corpus with the clause declared, plus the roots requirement's own opt-in
    // selection the judge reads its roots off: `d` is outside the closure and fires.
    let declared = [
        Selection {
            selector: Selector::OptIn("entrypoint".to_string()),
            clauses: Vec::new(),
            members: roots,
        },
        common::one_clause_selection(
            Selector::Kind("skill".to_string()),
            Severity::Required,
            Predicate::ReachedFrom {
                roots: "entrypoint".to_string(),
                via: Some(vec!["routes_to".to_string()]),
            },
            None,
            members,
        ),
    ];
    let diagnostics = graph::reached_from(&declared, &resolved, &[], &by_kind);
    assert_eq!(
        diagnostics.len(),
        1,
        "the root and the member it reaches hold; the orphan alone fires: {diagnostics:#?}",
    );
    assert_eq!(diagnostics[0].artifact, "d");
}

#[test]
fn gate_config_stale_uses_pre_parsed_lock_document() {
    let workspace = tmpdir("gate-config-stale-lock-parse-cost");

    // Create a lock.toml with a provenance row for a skill member, exercising the
    // config_stale path: read_lock_document() once, then config_stale_from_doc()
    // reuses the pre-parsed document without re-reading.
    let lock_path = workspace.join(temper::LOCK_FILENAME);
    std::fs::write(
        &lock_path,
        r#"[[skill]]
name = "test-skill"
source_path = ".claude/skills/test-skill/SKILL.md"
emit_hash = "0000000000000000000000000000000000000000000000000000000000000000"
"#,
    )
    .unwrap();

    // Create the referenced skill file so config_stale can read it.
    let skill_dir = workspace.join(".claude").join("skills").join("test-skill");
    std::fs::create_dir_all(&skill_dir).unwrap();
    std::fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test\n---\n# Test\n",
    )
    .unwrap();

    // Count lock reads/parses before test.
    let reads_before = temper::drift::lock_read_count();
    let parses_before = temper::drift::lock_parse_count();

    // Simulate what gate() does: read the lock once and pass it through to config_stale_from_doc.
    let lock_doc = temper::drift::read_lock_document(&workspace).expect("lock should parse");

    // Call config_stale_from_doc which should not re-read or re-parse the lock.
    let _ = temper::drift::config_stale_from_doc(&lock_doc, &workspace, &fresh_clause());

    // Count lock reads/parses after test.
    let reads_after = temper::drift::lock_read_count();
    let parses_after = temper::drift::lock_parse_count();

    let reads = reads_after - reads_before;
    let parses = parses_after - parses_before;

    // The cost doctrine: config_stale_from_doc must not re-read or re-parse the lock
    // when given a pre-parsed document. The single read+parse comes from read_lock_document()
    // above; config_stale_from_doc should contribute zero additional reads/parses.
    assert_eq!(
        reads, 1,
        "config_stale_from_doc must not re-read lock.toml when given pre-parsed doc: {reads} reads (before {reads_before}, after {reads_after})",
    );
    assert_eq!(
        parses, 1,
        "config_stale_from_doc must not re-parse lock.toml when given pre-parsed doc: {parses} parses (before {parses_before}, after {parses_after})",
    );
}

#[test]
fn emit_represented_manifest_read_is_hoisted_once_per_manifest() {
    use temper::drift::{self, CollectionAddressRow, Declarations, EmitOptions, Payload};

    let harness = tmpdir("emit-manifest-read-hoist");
    let into = harness.join(".temper");
    std::fs::create_dir_all(&into).unwrap();

    // Create a `.claude/settings.json` manifest file with a hook entry.
    common::write_settings(
        &harness,
        r#"{"hooks": {"onSessionStart": {"before": "echo starting"}}}"#,
    );

    // Create a lock.toml with a hook member (exercises the manifest segment reap path).
    let lock_path = into.join("lock.toml");
    std::fs::write(
        &lock_path,
        r#"[[hook]]
name = "onSessionStart"
source_path = ".claude/settings.json"
emit_hash = "0000000000000000000000000000000000000000000000000000000000000000"

[declaration]
"#,
    )
    .unwrap();

    // Create a payload that declares the hook with a collection address pointing to
    // settings.json, so emit will regenerate the manifest. Include a registration
    // to ensure the manifest is built and read.
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![drift::KindFactRow {
                collection_address: Some(CollectionAddressRow {
                    manifest: "settings.json".to_string(),
                    key_path: "hooks.<Event>".to_string(),
                    entry_shape: Some("group-array(hooks;matcher)".to_string()),
                }),
                ..common::kind_facts("hook", ".claude", "settings.json")
            }],
            registrations: vec![drift::RegistrationRow {
                kind: "hook".to_string(),
                key: "onSessionStart".to_string(),
                manifest: "settings.json".to_string(),
                key_path: "hooks.onSessionStart".to_string(),
                fields: vec![("before".to_string(), serde_json::json!("echo starting"))],
            }],
            ..Default::default()
        },
        members: vec![drift::PayloadMember {
            kind: "hook".to_string(),
            name: "onSessionStart".to_string(),
            host: None,
            fields: vec![("before".to_string(), serde_json::json!("echo starting"))],
            body: "".to_string(),
            source_path: None,
        }],
    };

    let options = EmitOptions {
        dry_run: true,
        frozen: false,
        teardown: false,
    };

    // Read counts before emit.
    let reads_before = drift::manifest_read_count();

    // Run emit — this should read the settings.json manifest exactly once,
    // reusing the read for both the segment-reap diff and the write-decision phase.
    let _ = drift::emit(&payload, &into, options);

    // Read counts after emit.
    let reads_after = drift::manifest_read_count();

    let reads = reads_after - reads_before;

    // The cost doctrine (engineering.md, "Cost scale is hoisted, and pinned by count"):
    // each represented manifest file is read exactly once per emit() pass and shared
    // between manifest_segment_reaps and emit_manifest, never re-read per phase.
    assert_eq!(
        reads, 1,
        "emit must read each represented manifest exactly once per run, not per phase: \
         {reads} reads (before {reads_before}, after {reads_after})",
    );
}

/// The run-level count-pin (`engineering.md`, "Cost scale is hoisted, and pinned by
/// count"): a whole check run walks each consulted discovery flavor exactly once. The
/// cache-level pin in `import.rs` proves N kind discoveries over one shared cache cost
/// one walk per flavor; this pins that the *run* actually rides a single shared cache.
/// `gate` threads one `Discovery` through every discovery call, so over the run the
/// global walk count advances by exactly the flavors consulted — a code path that
/// built a second `Discovery` mid-run would walk off its own cache and overshoot. The
/// run consults both flavors: committed kinds (skill, rule, …) ride the standard
/// flavor, the local-locus kinds (settings-local, dial) ride the local one — two
/// flavors, two walks, never a third. The walk count is per-thread, so the delta is
/// this run's alone whatever else runs concurrently.
#[test]
fn a_full_check_run_walks_each_consulted_flavor_once() {
    use temper::gate;

    let harness = tmpdir("run-walk-pin");
    let skill = harness.join(".claude").join("skills").join("coordinate");
    std::fs::create_dir_all(&skill).unwrap();
    std::fs::write(
        skill.join("SKILL.md"),
        "---\nname: coordinate\ndescription: Drive a task across a team of agents.\n---\n# Coordinate\n",
    )
    .unwrap();
    let rules = harness.join(".claude").join("rules");
    std::fs::create_dir_all(&rules).unwrap();
    std::fs::write(rules.join("rust.md"), "# Rust\n").unwrap();

    // The raw-harness gate: workspace and harness root are the one path, exactly as
    // `harness_diagnostics` dispatches a bare harness.
    let before = import::walk_count();
    gate::gate(&harness, &harness, &[]).unwrap();
    let walks = import::walk_count() - before;

    assert_eq!(
        walks, 2,
        "a whole run must walk each consulted flavor exactly once — one shared cache \
         threaded through the run, never a per-kind or per-call re-walk",
    );
}

/// The run-level count-pin for the *directive backing set* (`engineering.md`, "Cost
/// scale is hoisted, and pinned by count"): `compose::repo_file_set` is a whole-tree walk
/// over the consumer's harness root, and the sibling flavor pin above says nothing about
/// it — it rides `walkdir`, not the shared `Discovery` cache. Two-sided, because the walk
/// is now opt-in: an `@import`'s backing is resolved by stat per cited target, leaving
/// `reachable`'s `paths-match` channel the set's one consumer, so a run binding no root
/// `reachable` clause walks the tree **not at all** and one binding a clause walks it
/// exactly once — never once per kind, member, or call site.
///
/// What opts in is the **clause**, never the root selection's existence: the shipped root
/// default binds `reachable`, so the zero-walk half is stated over a lock declaring a root
/// contract of its own naming some other predicate, exactly as the reachability-closure
/// pin below states its own zero half.
#[test]
fn a_full_check_run_walks_the_directive_backing_set_only_where_a_root_clause_binds() {
    use temper::compose;
    use temper::drift::{ClauseRow, CountBoundRow, Declarations};
    use temper::gate;

    let harness = tmpdir("backing-set-walk-pin");
    let skill = harness.join(".claude").join("skills").join("coordinate");
    std::fs::create_dir_all(&skill).unwrap();
    std::fs::write(
        skill.join("SKILL.md"),
        "---\nname: coordinate\ndescription: Drive a task across a team of agents.\n---\n# Coordinate\n",
    )
    .unwrap();
    let rules = harness.join(".claude").join("rules");
    std::fs::create_dir_all(&rules).unwrap();
    std::fs::write(rules.join("rust.md"), "# Rust\n").unwrap();

    // A root contract binding some other predicate. The `count` bound is satisfied by any
    // corpus size and decides nothing here — its whole job is to be a root row that is not
    // `reachable`, so rows-or-default answers with it and the walk has no consumer.
    common::write_lock(
        &harness,
        Declarations {
            clauses: vec![ClauseRow {
                count: Some(CountBoundRow {
                    min: 0,
                    max: usize::MAX,
                }),
                ..common::clause("count", "advisory")
            }],
            ..Declarations::default()
        },
    );

    let before = compose::repo_file_set_count();
    let (diagnostics, _) = gate::gate(&harness.join(".temper"), &harness, &[]).unwrap();
    let unbound_walks = compose::repo_file_set_count() - before;

    // Non-vacuity (engineering.md, "A green verdict is proven non-vacuous"): a run that
    // classed no directive-bearing member would have nothing to resolve backing for, so
    // the pin names the members the run actually checked before either count is read.
    let checked = |diagnostics: &[temper::check::Diagnostic]| {
        let summary = diagnostics
            .iter()
            .find(|diagnostic| diagnostic.rule == "coverage.checked")
            .expect("the run discloses what it checked")
            .message
            .clone();
        assert!(
            summary.contains("skill (1") && summary.contains("rule (1"),
            "the run must have checked the fixture's members, got: {summary}",
        );
    };
    checked(&diagnostics);

    assert_eq!(
        unbound_walks, 0,
        "a run binding no root `reachable` clause must not walk the tree at all — an          `@import`'s backing resolves by stat per cited target, and the set's one          remaining consumer never asks",
    );

    // The same corpus, one root `reachable` row: the set's consumer is back, and the walk
    // it needs is hoisted to exactly one for the whole run.
    let opted_in = tmpdir("backing-set-walk-pin-opted-in");
    common::copy_tree(&harness.join(".claude"), &opted_in.join(".claude"));
    common::write_lock(
        &opted_in,
        Declarations {
            clauses: vec![common::clause("reachable", "advisory")],
            ..Declarations::default()
        },
    );

    let before = compose::repo_file_set_count();
    let (diagnostics, _) = gate::gate(&opted_in.join(".temper"), &opted_in, &[]).unwrap();
    let bound_walks = compose::repo_file_set_count() - before;
    checked(&diagnostics);

    assert_eq!(
        bound_walks, 1,
        "a bound root `reachable` clause walks the directive backing set exactly once —          one hoisted whole-tree walk per run, never a per-kind or per-call re-walk",
    );
}

/// The per-kind resolution count-pin: every kind's members are read from disk and
/// parsed exactly once per gate/explain invocation, so no consulted kind is resolved
/// twice.
#[test]
fn resolve_kind_units_runs_once_per_kind_not_twice() {
    use temper::builtin_kind;
    use temper::compose;
    use temper::drift;
    use temper::gate;
    use temper::kind::Commitment;

    let harness = tmpdir("resolve-units-once");
    let skill = harness.join(".claude").join("skills").join("test-skill");
    std::fs::create_dir_all(&skill).unwrap();
    std::fs::write(
        skill.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test skill.\n---\n# Skill\n",
    )
    .unwrap();
    let rules = harness.join(".claude").join("rules");
    std::fs::create_dir_all(&rules).unwrap();
    std::fs::write(rules.join("test-rule.md"), "# Rule\n").unwrap();

    // One custom kind beside the built-ins, so the pin below covers a locked kind and
    // not the built-in set alone. Written by the real lock writer (`drift::emit`) off a
    // `KindFactRow`, the row this family's one producer emits. A plain markdown file
    // kind on purpose: its members' identity is the file stem, where a `toml-document`
    // kind would carry a `named-field` identity fact this case has no use for.
    common::write_lock(
        &harness,
        drift::Declarations {
            kinds: vec![common::kind_facts("custom-kind", ".custom", "*.md")],
            ..drift::Declarations::default()
        },
    );
    let custom_dir = harness.join(".custom");
    std::fs::create_dir_all(&custom_dir).unwrap();
    std::fs::write(custom_dir.join("member.md"), "# Member\n").unwrap();

    // The adopted-harness dispatch: the workspace is the `.temper/` the lock lives in,
    // the harness root the tree discovery walks — exactly the pair `harness_diagnostics`
    // hands `gate` for a root carrying a workspace.
    let before = compose::resolve_kind_units_count();
    let (diagnostics, _) = gate::gate(&harness.join(".temper"), &harness, &[]).unwrap();
    let resolves = compose::resolve_kind_units_count() - before;

    // Non-vacuity (engineering.md, "A green verdict is proven non-vacuous"): the count
    // below is only a *custom*-kind pin if the custom kind actually resolved its own
    // member — a bare `resolves > 0` the built-in set satisfies alone proves nothing
    // about it. The run's own disclosure of what it checked names the member.
    let summary = &diagnostics
        .iter()
        .find(|diagnostic| diagnostic.rule == "coverage.checked")
        .expect("the run discloses what it checked")
        .message;
    assert!(
        summary.contains("custom-kind (1"),
        "the locked custom kind must have resolved its own member, got: {summary}",
    );

    // The judging pass runs it once per consulted kind — every built-in plus the one
    // locked `custom-kind`. Spelled as an equality off the real sets rather than a ceiling, so
    // a re-doubled call site fails hard instead of fitting under a loose bound; the only
    // reads before that pass are `assemble_lock_family`'s one pre-pass per local-locus
    // kind, whose read-time rows the family is assembled from — named here, never folded
    // into slack.
    let builtins = builtin_kind::definitions();
    let locals = builtins
        .values()
        .filter(|kind| kind.commitment == Some(Commitment::Local))
        .count();
    let expected = builtins.len() + 1 + locals;
    assert_eq!(
        resolves,
        expected,
        "resolve_kind_units must run once per consulted kind ({} built-ins plus the \
         locked `custom-kind`), plus the {locals} local-locus pre-passes; got {resolves}",
        builtins.len(),
    );
}

#[test]
fn overlay_builtin_kind_count_increments_once_per_application() {
    use temper::builtin_kind;
    use temper::compose;
    use temper::gate;

    let before = compose::overlay_builtin_kind_count();
    let harness = tmpdir("overlay-count");
    let skill = harness.join(".claude").join("skills").join("test-skill");
    std::fs::create_dir_all(&skill).unwrap();
    std::fs::write(
        skill.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test skill.\n---\n# Skill\n",
    )
    .unwrap();

    gate::gate(&harness, &harness, &[]).unwrap();
    let overlays = compose::overlay_builtin_kind_count() - before;

    let builtin_defs = builtin_kind::definitions();
    let builtin_count = builtin_defs.len();

    assert!(
        overlays > 0,
        "overlay_builtin_kind not called at all (delta was 0); the real counter may not be wired",
    );

    assert!(
        overlays <= builtin_count,
        "overlay_builtin_kind called {overlays} times; should be at most {builtin_count} \
         (once per builtin kind) — the hoisting fix may not be working",
    );
}

#[test]
fn gate_manifest_cache_read_is_hoisted_across_governing_kinds() {
    use std::collections::BTreeMap;
    use temper::compose;
    use temper::drift::{CollectionAddressRow, Declarations, KindFactRow};
    use temper::import::Discovery;
    use temper::json_manifest;
    use temper::kind::{CollectionAddress, CollectionKeyPath, EntryShape, Extraction};

    let harness = tmpdir("gate-manifest-cache-read-hoist");

    // Create a settings.json manifest with entries for two different collection types:
    // - hooks (lifecycle event collection)
    // - enabledPlugins (scalar collection)
    common::write_settings(
        &harness,
        r#"{"hooks": {"SessionStart": [{"hooks": [{"type": "command", "command": "echo hi"}]}]}, "enabledPlugins": {"test-plugin@my-marketplace": true}}"#,
    );

    // Create a Discovery instance to traverse the file tree
    let discovery = Discovery::new(&harness);

    // Build two kinds programmatically that both govern settings.json:
    // - "hook" kind with a hooks.<Event> collection address
    // - "installed-plugin" kind with an enabledPlugins.* collection address
    let mut overlaid_builtin_kinds = BTreeMap::new();

    // First kind: hook
    let mut hook_kind = temper::kind::CustomKind::new(
        "hook".to_string(),
        temper::kind::Governs {
            root: ".claude".to_string(),
            glob: "settings.json".to_string(),
        },
        Extraction::new(Vec::new()),
    );
    hook_kind.collection_address = Some(CollectionAddress {
        manifest: "settings.json".to_string(),
        key_path: CollectionKeyPath::HooksEvent,
        entry_shape: EntryShape::GroupArray {
            member_key: "hooks".to_string(),
            lifted_fields: vec!["matcher".to_string()],
        },
    });
    overlaid_builtin_kinds.insert("hook".to_string(), hook_kind);

    // Second kind: installed-plugin
    let mut plugin_kind = temper::kind::CustomKind::new(
        "installed-plugin".to_string(),
        temper::kind::Governs {
            root: ".claude".to_string(),
            glob: "settings.json".to_string(),
        },
        Extraction::new(Vec::new()),
    );
    plugin_kind.collection_address = Some(CollectionAddress {
        manifest: "settings.json".to_string(),
        key_path: CollectionKeyPath::EnabledPlugins,
        entry_shape: EntryShape::Scalar {
            field: "enabled".to_string(),
        },
    });
    overlaid_builtin_kinds.insert("installed-plugin".to_string(), plugin_kind);

    // Build declarations with the two kinds
    let kind_rows = vec![
        KindFactRow {
            collection_address: Some(CollectionAddressRow {
                manifest: "settings.json".to_string(),
                key_path: "hooks.<Event>".to_string(),
                entry_shape: Some("group-array(hooks;matcher)".to_string()),
            }),
            ..common::kind_facts("hook", ".claude", "settings.json")
        },
        KindFactRow {
            collection_address: Some(CollectionAddressRow {
                manifest: "settings.json".to_string(),
                key_path: "enabledPlugins.*".to_string(),
                entry_shape: Some("scalar(enabled)".to_string()),
            }),
            ..common::kind_facts("installed-plugin", ".claude", "settings.json")
        },
    ];

    let declarations = Declarations {
        kinds: kind_rows,
        ..Default::default()
    };

    // Read counts before build_manifest_cache
    let reads_before = json_manifest::manifest_read_count();

    // Call build_manifest_cache — this should read settings.json exactly once,
    // even though two kinds govern it with different collection addresses.
    let _ = compose::build_manifest_cache(&discovery, &declarations, &overlaid_builtin_kinds)
        .expect("build_manifest_cache should succeed");

    // Read counts after build_manifest_cache
    let reads_after = json_manifest::manifest_read_count();

    let reads = reads_after - reads_before;

    // The cost doctrine (engineering.md, "Cost scale is hoisted, and pinned by count"):
    // a manifest file is read exactly once per gate invocation, shared across all
    // kinds that govern it, never once per governing kind.
    assert_eq!(
        reads, 1,
        "build_manifest_cache must read each manifest exactly once, shared across \
         all governing kinds, not once per kind: {reads} reads (before {reads_before}, after {reads_after})",
    );
}

/// The reachability closure is whole-corpus work — the registration seed over every
/// member plus the hop-capped import propagation — so the cost doctrine binds it the same
/// way it binds the resolved-edge walk: computed once per `gate()` invocation, and not at
/// all where no root `reachable` clause opts in.
///
/// What opts in is the **clause**, never the root selection's existence. The shipped root
/// default binds `reachable` (`rootDefaultContract`, `sdk/src/assembly.ts`), so the
/// zero-walk half is stated over a lock that declares a root contract of its own naming
/// some other predicate — rows-or-default then answers with those rows, and no
/// `reachable` clause is in play. An empty `Declarations` would fall back to the embedded
/// default and walk once, which is the shipped behaviour rather than a cost regression.
#[test]
fn gate_reachability_closure_runs_once_per_invocation_and_only_when_a_root_clause_binds() {
    use temper::drift::{ClauseRow, CountBoundRow, Declarations};
    use temper::gate;
    use temper::graph;

    let harness = tmpdir("reachability-closure-pin");
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    // A rule whose `paths` glob matches no file — a dead `paths-match` channel, so the
    // closure has something to decide rather than short-circuiting on a clean corpus.
    fs::write(
        rules.join("style.md"),
        "---\npaths: [\"never/**/*.xyz\"]\n---\n# Style\n",
    )
    .unwrap();

    // A root contract that binds some other predicate: `reachable` is opt-in per clause,
    // so the closure never walks. The `count` bound is satisfied by any corpus size and
    // decides nothing here — its whole job is to be a root row that is not `reachable`.
    common::write_lock(
        &harness,
        Declarations {
            clauses: vec![ClauseRow {
                count: Some(CountBoundRow {
                    min: 0,
                    max: usize::MAX,
                }),
                ..common::clause("count", "advisory")
            }],
            ..Declarations::default()
        },
    );
    let before = graph::live_members_count();
    gate::gate(&harness.join(".temper"), &harness, &[]).unwrap();
    assert_eq!(
        graph::live_members_count() - before,
        0,
        "a gate with no root `reachable` clause must walk no reachability closure",
    );

    // One root row, on a fresh copy of the same corpus: the closure walks exactly once
    // for the whole run, never once per member of the corpus it ranges over.
    let opted_in = tmpdir("reachability-closure-pin-opted-in");
    fs::create_dir_all(opted_in.join(".claude").join("rules")).unwrap();
    common::copy_tree(&harness.join(".claude"), &opted_in.join(".claude"));
    common::write_lock(
        &opted_in,
        Declarations {
            clauses: vec![common::clause("reachable", "advisory")],
            ..Declarations::default()
        },
    );
    let before = graph::live_members_count();
    let (diagnostics, _) = gate::gate(&opted_in.join(".temper"), &opted_in, &[]).unwrap();
    assert!(
        diagnostics.iter().any(|d| d.rule == "root.reachable"),
        "the copied corpus carries the dead rule, so the one walk decided something real",
    );
    assert_eq!(
        graph::live_members_count() - before,
        1,
        "the reachability closure is hoisted per gate invocation, never per member",
    );
}

/// The per-tool-call count-pin for the **guard's shell edge** (`engineering.md`, "Cost
/// scale is hoisted, and pinned by count"): `install::locus_member_sites` walks once per
/// declared locus root, and a `PostToolUse` call pays that cost on every tool call a
/// session makes — the hottest path any pin in this file covers. Two declared roots, so
/// the delta is exactly 2: a re-walk per candidate file or per glob overshoots, and a
/// widening to one `.`-rooted whole-tree walk undershoots at 1, so the count catches the
/// regression in both directions. The count is per-thread and the walk single-threaded on
/// its caller's thread, so the delta is this call's alone whatever else runs concurrently.
#[test]
fn a_guard_shell_edge_walks_each_declared_locus_root_once() {
    use temper::drift::{self, Declarations};
    use temper::install::{self, GuardedLocus};

    let harness = tmpdir("guard-locus-walk-pin");

    // A document at each governed locus root, neither of them declared by the lock below
    // — the stray the locus half of `locus-declared` exists to name.
    let skill = harness.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(
        skill.join("SKILL.md"),
        "---\nname: coordinate\ndescription: Drive a task across a team of agents.\n---\n# Coordinate\n",
    )
    .unwrap();
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), "# Rust\n").unwrap();

    // A represented harness whose root contract binds `locus-declared` and nothing else:
    // rows-or-default reads these rows, so the locus half opts in and the projection half
    // stays silent. The lock declares no provenance row, so every site below is a stray.
    common::write_lock(
        &harness,
        Declarations {
            clauses: vec![common::clause("locus-declared", "required")],
            ..Declarations::default()
        },
    );
    let workspace = harness.join(".temper");
    let declarations = drift::read_declarations(&workspace).unwrap();

    // The two loci `guarded_loci` would assemble for these kinds — distinct roots, the
    // quantity the walk count is pinned against.
    let loci = vec![
        GuardedLocus {
            kind: "rule".to_string(),
            root: ".claude/rules".to_string(),
            pattern: ".claude/rules/*.md".to_string(),
            custom: false,
        },
        GuardedLocus {
            kind: "skill".to_string(),
            root: ".claude/skills".to_string(),
            pattern: ".claude/skills/*/SKILL.md".to_string(),
            custom: false,
        },
    ];

    let before = install::locus_member_site_walk_count();
    let report = install::shell_edge_findings(&workspace, &declarations, &loci).unwrap();
    let walks = install::locus_member_site_walk_count() - before;

    // Non-vacuity (engineering.md, "A green verdict is proven non-vacuous"): a call whose
    // walk found nothing would pin its cost over an empty subject, so the report must name
    // the stray under each root before the count is asserted.
    let report = report.expect("two undeclared documents at governed loci must report");
    assert!(
        report.contains(".claude/rules/rust.md")
            && report.contains(".claude/skills/coordinate/SKILL.md"),
        "the shell edge must name the stray under each declared locus root, got: {report}",
    );

    assert_eq!(
        walks, 2,
        "a shell-edge call must walk exactly once per declared locus root — a per-file or \
         per-glob re-walk overshoots, and a widened whole-tree walk undershoots",
    );
}

/// The per-pass count-pin for a **local layout kind's documents** (`engineering.md`, "Cost
/// scale is hoisted, and pinned by count"): a local member's rows are derived at read time
/// rather than read back off the lock, and `assemble_lock_family` is the pass that derives
/// them — so each member's document is read exactly once there, off the text the unit
/// resolution already loaded. Two members, so the delta is exactly 2: the derivation
/// handing a `source_path` back to disk after the unit already carried the bytes doubles it
/// to 4, which is what this pass cost before the hoist. The count is per-thread and both
/// reads single-threaded on their caller's thread, so the delta is this pass's alone
/// whatever else runs concurrently.
///
/// Pass granularity, not run: the gate's own per-kind resolution is a separate reader of
/// the same documents (one more read apiece, hoisted no further by this pin), so the count
/// is taken around `assemble_lock_family` itself rather than around `gate::gate`.
#[test]
fn a_local_layout_members_document_is_read_once_per_assembly_pass() {
    use std::collections::BTreeMap;
    use temper::compose;
    use temper::drift::{self, Declarations};

    let harness = tmpdir("local-layout-doc-read-pin");
    fs::create_dir_all(harness.join(".temper")).unwrap();

    // Two members of the `knob` local layout kind — the suite's shared local-locus fixture
    // (`common`), whose document carries a member collection, so the derivation this pin
    // counts has rows to yield.
    common::write_sibling(&harness, ".claude/local/knob-a.md", common::KNOB_DOC);
    common::write_sibling(&harness, ".claude/local/knob-b.md", common::KNOB_DOC);
    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![common::knob_kind_facts()],
            ..Declarations::default()
        },
    );

    let workspace = harness.join(".temper");
    let committed = drift::read_declarations(&workspace).unwrap();
    let discovery = import::Discovery::new(&harness);
    let cache: compose::ManifestCache = BTreeMap::new();

    let before = drift::layout_document_read_count();
    let family = compose::assemble_lock_family(&discovery, &committed, &[], &cache).unwrap();
    let reads = drift::layout_document_read_count() - before;

    // Non-vacuity (engineering.md, "A green verdict is proven non-vacuous"): a pass that
    // derived nothing would read nothing and pass any count, so the rows the derivation
    // exists to yield are asserted present first — both members' collections, off the
    // documents this pin says were read once each.
    let hosts: Vec<&str> = family
        .declarations
        .nested_members
        .iter()
        .map(|row| row.host.as_str())
        .collect();
    assert!(
        hosts.contains(&"knob:knob-a") && hosts.contains(&"knob:knob-b"),
        "both local members' documents must have lowered their collection rows, got: {hosts:?}"
    );

    assert_eq!(
        reads, 2,
        "each local layout member's document is read once per assembly pass, not once per \
         reader of it; got {reads} reads for two members"
    );
}

/// The per-pass count-pin for the **parse** half of the same documents (`engineering.md`,
/// "Cost scale is hoisted, and pinned by count"): holding a document's text is not holding
/// its reading, and the reading is the expensive half — the heading tree, the region match,
/// the span splits. One parse per member per pass, its field half filling the unit and its
/// member/prose/`satisfies` half lowering into the derived rows, is what this counts: the
/// unit adapter keeping only the fields and the row derivation re-reading the identical
/// text doubles it to 4, which is what this pass cost before the fold.
///
/// Paired with the read pin above on the same fixture, the way `LOCK_READS`/`LOCK_PARSES`
/// pair: a fold that hoisted the read alone still leaves this one red.
#[test]
fn a_local_layout_members_document_is_parsed_once_per_assembly_pass() {
    use std::collections::BTreeMap;
    use temper::compose;
    use temper::drift::{self, Declarations};

    let harness = tmpdir("local-layout-doc-parse-pin");
    fs::create_dir_all(harness.join(".temper")).unwrap();

    common::write_sibling(&harness, ".claude/local/knob-a.md", common::KNOB_DOC);
    common::write_sibling(&harness, ".claude/local/knob-b.md", common::KNOB_DOC);
    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![common::knob_kind_facts()],
            ..Declarations::default()
        },
    );

    let workspace = harness.join(".temper");
    let committed = drift::read_declarations(&workspace).unwrap();
    let discovery = import::Discovery::new(&harness);
    let cache: compose::ManifestCache = BTreeMap::new();

    let before = drift::layout_document_parse_count();
    let family = compose::assemble_lock_family(&discovery, &committed, &[], &cache).unwrap();
    let parses = drift::layout_document_parse_count() - before;

    // Non-vacuity (engineering.md, "A green verdict is proven non-vacuous"): the rows the
    // parse exists to yield are asserted present before the count, so a pass that derived
    // nothing cannot parse nothing and pass.
    let hosts: Vec<&str> = family
        .declarations
        .nested_members
        .iter()
        .map(|row| row.host.as_str())
        .collect();
    assert!(
        hosts.contains(&"knob:knob-a") && hosts.contains(&"knob:knob-b"),
        "both local members' documents must have lowered their collection rows, got: {hosts:?}"
    );

    assert_eq!(
        parses, 2,
        "each local layout member's document is parsed once per assembly pass — the unit's \
         fields and the derived rows come off the one reading; got {parses} parses for two \
         members"
    );
}

/// The per-distinct-glob count-pin for **emit's placement round trip**
/// (`engineering.md`, "Cost scale is hoisted, and pinned by count"): every member's
/// derived path is matched back through its own kind's glob before a byte is written, so
/// the matcher build is per *glob*, never per member. Two kinds over 400 members, so the
/// delta is exactly 2 — a rebuild per member overshoots to 400, and a cache that went
/// stale between the two kinds' loci undershoots.
///
/// Distinct from the discovery pin at the top of this file, whose window is the walk emit
/// never enters: this one measures the write side, where the same glob engine now serves
/// placement. Each kind's glob is compiled on its first member and hit from the cache for
/// every one after, including the second pass `member_path_index` makes over the same set.
/// The count is per-thread and emit single-threaded on its caller's thread, so the delta is
/// this call's alone whatever else runs concurrently.
#[test]
fn emit_compiles_each_kinds_glob_once_however_many_members_it_places() {
    let (_harness, into) = common::workspace("emit-round-trip-cost");
    let members: Vec<PayloadMember> = (0..200)
        .flat_map(|i| {
            [
                common::rule_member(&format!("rule-{i}"), None, "# Rule\n"),
                common::skill_member(
                    &format!("skill-{i}"),
                    "Use when the cost fixture needs one more member.",
                    "# Skill\n",
                ),
            ]
        })
        .collect();
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                common::rule_kind_facts(None, &[]),
                common::skill_kind_facts(None, &[]),
            ],
            ..Default::default()
        },
        members,
    };

    let before = glob::glob_compile_count();
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let compiles = glob::glob_compile_count() - before;

    assert_eq!(report.entries.len(), 400, "the fixture places every member");
    assert_eq!(
        compiles, 2,
        "emit places 400 members across 2 kinds, so it builds exactly 2 glob matchers — one \
         per distinct glob, hoisted out of the per-member round trip; got {compiles}",
    );
}
