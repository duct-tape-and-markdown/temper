//! Measure-first cost diagnosis for `check` at consumer scale
//! (`specs/process/engineering.md`, "Cost scale is hoisted, and pinned by count").
//!
//! A synthetic harness the size of a real consumer's tree is generated in a tempdir at
//! test time and never committed. Discovery — the phase `check` opens with, walking the
//! consumer's whole tree per kind — is timed over it so the numbers, not a guess, name
//! where the residual concentrates; the timings print (a manual signal a human reads) and
//! the test asserts the work-count pins the cuts earn — decided by counts, independent of
//! tree size: the shared walk runs once per flavor, glob compilation is hoisted per
//! distinct glob rather than per candidate file, and the per-kind glob scan reads its
//! members from that one walk's index, opening no directory of its own.

mod common;

use std::fs;
use std::path::Path;
use std::time::Instant;

use common::tmpdir;
use temper::builtin_kind;
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
fn gate_lock_parse_is_hoisted_with_source_dependencies() {
    let workspace = tmpdir("gate-lock-parse-cost");

    // Create a lock.toml with layout_import and include source-dependency rows,
    // which exercises the hoisted parse path: read_lock_document() once, then
    // layout_imports_from_doc/includes_from_doc/layout_import_stale_from_doc/
    // include_stale_from_doc all reuse the pre-parsed document.
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
"#,
    )
    .unwrap();

    // Count lock reads/parses before hoisting test.
    let reads_before = temper::drift::lock_read_count();
    let parses_before = temper::drift::lock_parse_count();

    // Simulate what gate() does: read the lock once and pass it through to all the
    // source-dependency call sites. This verifies the hoisting: one read, one parse,
    // even though four call sites access source dependencies.
    let lock_doc = temper::drift::read_lock_document(&workspace).expect("lock should parse");

    // The four call sites that would each re-read the lock (pre-hoisting):
    // 1. layout_imports (called by import_edges_from_lock)
    let _ = temper::drift::layout_imports_from_doc(&lock_doc);
    // 2. includes (called by import_edges_from_lock)
    let _ = temper::drift::includes_from_doc(&lock_doc);
    // 3. layout_import_stale
    let harness_root = workspace
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let _ = temper::drift::layout_import_stale_from_doc(&lock_doc, harness_root);
    // 4. include_stale
    let _ = temper::drift::include_stale_from_doc(&lock_doc, harness_root);

    // Count lock reads/parses after hoisting test.
    let reads_after = temper::drift::lock_read_count();
    let parses_after = temper::drift::lock_parse_count();

    let reads = reads_after - reads_before;
    let parses = parses_after - parses_before;

    // The cost doctrine: lock parsing is hoisted — one read per run, one parse per run,
    // even though four call sites access source dependencies. Each _from_doc variant
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

#[test]
fn coverage_note_accepts_pre_parsed_locked_kinds() {
    use std::collections::BTreeMap;
    use temper::coverage_note;
    use temper::drift;

    let harness = tmpdir("coverage-note-lock-parse-hoist");

    // Create a harness with a `.claude/settings.json` and a lock that declares a custom
    // `widget` kind governing that file.
    common::write_skill(&harness, "test-skill", "# Test\n\nBody.");
    std::fs::create_dir_all(harness.join(".claude")).unwrap();
    // One residue key no built-in segment kind governs, so the finding has something to
    // fire on: an empty manifest names no ungoverned segment and the suppression below
    // would read as vacuous.
    std::fs::write(
        harness.join(".claude/settings.json"),
        r#"{ "permissions": { "allow": [] } }"#,
    )
    .unwrap();

    // Create a lock.toml with a widget kind row
    let lock_dir = harness.join(".temper");
    std::fs::create_dir_all(&lock_dir).unwrap();
    std::fs::write(
        lock_dir.join("lock.toml"),
        r#"[declaration]

[[declaration.kind]]
name = "widget"
governs_root = ".claude"
governs_glob = "settings.json"
unit_shape = "file"
"#,
    )
    .unwrap();

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

    // Call coverage_note::check with the pre-parsed kind rows (the new API). The in-scope
    // set is the built-ins with the `settings` container withheld: it governs the file
    // whole, and under it a locked kind's own contribution would be unobservable.
    let member_counts = BTreeMap::from([("skill".to_string(), 1usize)]);
    let mut in_scope = temper::builtin_kind::definitions();
    in_scope.remove("settings");
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
    let names_settings = |diagnostics: &[temper::check::Diagnostic]| {
        diagnostics.iter().any(|d| {
            d.rule == "coverage.unmodeled-surface" && d.artifact == ".claude/settings.json"
        })
    };

    // Non-vacuity: with no locked rows handed in, the residue key is flagged.
    assert!(
        names_settings(&check(&[])),
        "without the locked rows the ungoverned residue is flagged"
    );

    // Verify: the custom widget kind was used to suppress the settings.json finding
    // because it governs the file. This proves the hoisted kind rows are being used
    // correctly to suppress the finding, just as if they had been read internally.
    let diagnostics = check(&committed.kinds);
    assert!(
        !names_settings(&diagnostics),
        "the locked widget kind should suppress the settings.json finding, got: {diagnostics:#?}"
    );
}

#[test]
fn gate_resolved_edge_walk_is_hoisted_per_gate_invocation() {
    // Verify that the edge-resolution walk is computed exactly once per gate() invocation
    // and shared across its consumers. The cost doctrine (engineering.md, "Cost scale is
    // hoisted, and pinned by count") requires whole-input work computes once per run and
    // is shared, never recomputed per call site.
    //
    // `acyclic` is off that consumer list: `contract.md` ("well-formedness") scopes it to
    // the import relation, which is not this walk. `check` is the walk's own thin
    // wrapper (it reads the dangling half of the same computation), so the consumers of
    // the *pre-computed* slice are `degree`, `reached_from`, `mention_reachable`, and
    // `reachable`.
    use std::collections::BTreeMap;
    use temper::compose;
    use temper::extract::Features;
    use temper::graph;

    // A simple edge set: skill:s → rule:r.
    let edges = [compose::Edge {
        field: "routes_to".to_string(),
        from: "skill".to_string(),
        to: vec!["rule".to_string()],
    }];

    // A minimal by_kind corpus: one skill and one rule.
    let mut skill_fields = BTreeMap::new();
    skill_fields.insert("routes_to".to_string(), serde_json::json!(["r"]));
    let skill = Features {
        fields: skill_fields,
        body_lines: 1,
        ..common::features("s")
    };

    let rule = Features {
        body_lines: 1,
        ..common::features("r")
    };

    let skills = [skill];
    let rules = [rule];
    let by_kind: BTreeMap<&str, &[Features]> =
        BTreeMap::from([("skill", &skills[..]), ("rule", &rules[..])]);

    let count_before = graph::resolved_edges_count();

    // Call resolved_edges once.
    let resolved_result = graph::resolved_edges(&edges, &by_kind);
    let resolved_edges = &resolved_result.resolved;

    // Use the pre-computed resolved edges in each consumer: neither re-walks.
    let selections: [temper::engine::Selection; 0] = [];
    // `degree` also takes the composed corpus, for the containment family it derives
    // *after* its opt-in early return — no always-on walk, so the pin below does not move.
    let _ = graph::degree(&selections, resolved_edges, &[], &by_kind);
    // `reached_from` walks the closure `degree` cannot see, over the same slice and
    // under the same opt-in early return — so it re-walks nothing either.
    let _ = graph::reached_from(&selections, resolved_edges, &[], &by_kind);
    let _ = graph::mention_reachable(
        &selections,
        resolved_edges,
        &[],
        &by_kind,
        &graph::embedded_hosts_by_key(&by_kind),
    );
    // The third consumer: reachability closes over the same resolved slice (plus the
    // directive edges, empty here), so it re-walks nothing either.
    let _ = graph::reachable(
        &BTreeMap::new(),
        &by_kind,
        &[],
        resolved_edges,
        temper::check::Severity::Warn,
    );

    // The narrowing itself, pinned at the cost seam: `acyclic` no longer takes this
    // slice, so its input is the import relation — empty here, since no member imports.
    assert!(graph::acyclic(&[]).is_empty());

    let count_after = graph::resolved_edges_count();
    let resolves_calls = count_after - count_before;

    // The cost doctrine: the walk is computed exactly once per gate invocation.
    assert_eq!(
        resolves_calls, 1,
        "gate() must compute resolved_edges exactly once, shared across degree, reached_from and mention_reachable: {resolves_calls} calls (before {count_before}, after {count_after})",
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
    use temper::contract::{Clause, Predicate, Severity};
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
    let quiet = [Selection {
        selector: Selector::Kind("skill".to_string()),
        clauses: vec![Clause {
            label: "skill.count".to_string(),
            severity: Severity::Advisory,
            predicate: Predicate::Count {
                min: 0,
                max: usize::MAX,
            },
            guidance: None,
            source: None,
        }],
        members: members.clone(),
    }];
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
        Selection {
            selector: Selector::Kind("skill".to_string()),
            clauses: vec![Clause {
                label: "skill.reached-from".to_string(),
                severity: Severity::Required,
                predicate: Predicate::ReachedFrom {
                    roots: "entrypoint".to_string(),
                    via: Some(vec!["routes_to".to_string()]),
                },
                guidance: None,
                source: None,
            }],
            members,
        },
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
    let _ = temper::drift::config_stale_from_doc(&lock_doc, &workspace);

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

/// The per-kind resolution count-pin: every kind's members are read from disk and
/// parsed exactly once per gate/explain invocation. The test verifies that no kind is
/// resolved more than once — before the fix, kinds were resolved twice (once through
/// kind_features for validation and again through collect_directive_members).
#[test]
fn resolve_kind_units_runs_once_per_kind_not_twice() {
    use temper::compose;
    use temper::gate;

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

    // Create a custom kind with one member to verify custom kinds are also resolved
    // exactly once.
    let custom_kinds = harness.join(".temper");
    std::fs::create_dir_all(&custom_kinds).unwrap();
    std::fs::write(
        custom_kinds.join("lock.toml"),
        r#"[[kind]]
name = "custom-kind"
content = "file"
format = "toml-document"
governs = ".custom"
unit_shape = "file"
"#,
    )
    .unwrap();
    let custom_dir = harness.join(".custom");
    std::fs::create_dir_all(&custom_dir).unwrap();
    std::fs::write(
        custom_dir.join("member.toml"),
        "[custom]\ndata = \"test\"\n",
    )
    .unwrap();

    let before = compose::resolve_kind_units_count();
    gate::gate(&harness, &harness, &[]).unwrap();
    let resolves = compose::resolve_kind_units_count() - before;

    // The count-pin verifies resolve_kind_units is called exactly once per kind.
    // With 15+ built-in kinds plus custom kinds, a vacuous test would pass with zero;
    // a real run must demonstrate non-zero delta.
    assert!(
        resolves > 0,
        "resolve_kind_units not called at all (delta was 0); the real counter may not be wired",
    );

    // Before the fix, resolve_kind_units was called twice per kind: once through
    // kind_features and again through collect_directive_members. After the fix, it's
    // called exactly once per kind. The exact count depends on how many built-in kinds
    // exist (14) plus custom kinds (at least 1), but the key invariant is that with
    // 2+ kinds, we should see fewer than `2 * kind_count` resolves. For 15+ kinds,
    // a pre-fix run would make 30+ calls; post-fix should be ~15-20.
    let kind_count_estimate = 15;
    let max_expected_if_doubled = kind_count_estimate * 2;
    assert!(
        resolves < max_expected_if_doubled,
        "resolve_kind_units called {resolves} times; if it ran twice per kind \
         (pre-fix), would expect {max_expected_if_doubled}+ — the threading fix may not be working",
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
