//! The `memory-local` built-in kind: a `CLAUDE.local.md` at any depth, personal memory
//! loaded beside the `CLAUDE.md` of its directory, at the **local** commitment class —
//! read in place and gated at check, never emitted or locked. The generic local-locus
//! machinery is `local_locus.rs`'s; these cases own the shipped kind's face.

use std::fs;

use temper::drift::{self, EmitOptions, Payload};
use temper::kind::{Commitment, Registration};

use crate::common::{self, check_harness_in, write_sibling};

const LOCAL_MEMORY: &str = "# Mine\n\nUse my sandbox URL.\n";

#[test]
fn a_root_claude_local_md_is_a_memory_local_member_read_in_place_at_check() {
    let harness = common::tmpdir("memory-local-root");
    // Claude Code gitignores the file, so only the local class's discovery override
    // reaches it.
    write_sibling(&harness, ".gitignore", "CLAUDE.local.md\n");
    write_sibling(&harness, "CLAUDE.md", "# Team\n");
    write_sibling(&harness, "CLAUDE.local.md", LOCAL_MEMORY);

    let run = check_harness_in(&harness, Some("github"));
    assert!(run.ok, "the local memory gates clean: {}", run.output);
    assert!(
        run.announcements()
            .iter()
            .any(|line| line.contains("local member: memory-local:CLAUDE.local")),
        "the active local memory is announced: {:?}",
        run.announcements()
    );
    assert!(
        run.output.contains("memory-local (1)"),
        "the local memory is checked, not skipped: {}",
        run.output
    );

    let kind = temper::builtin_kind::definition("memory-local").expect("memory-local ships");
    assert_eq!(kind.commitment, Some(Commitment::Local));
    assert_eq!(kind.registration, vec![Registration::OnReadBeneath]);
}

#[test]
fn a_subdirectory_claude_local_md_is_a_memory_local_member() {
    let harness = common::tmpdir("memory-local-nested");
    write_sibling(&harness, ".gitignore", "CLAUDE.local.md\n");
    write_sibling(&harness, "apps/web/CLAUDE.md", "# Web\n");
    write_sibling(&harness, "apps/web/CLAUDE.local.md", LOCAL_MEMORY);

    let run = check_harness_in(&harness, Some("github"));
    assert!(run.ok, "{}", run.output);
    // Named by the nested grammar memory's own subdirectory members take.
    assert!(
        run.announcements()
            .iter()
            .any(|line| line.contains("local member: memory-local:apps:web:CLAUDE.local")),
        "the nested local memory is announced: {:?}",
        run.announcements()
    );
}

#[test]
fn a_local_memory_over_the_size_target_draws_the_advisory() {
    let harness = common::tmpdir("memory-local-long");
    write_sibling(&harness, "CLAUDE.local.md", &"- a line\n".repeat(201));

    let run = check_harness_in(&harness, Some("github"));
    let findings = run.findings();
    let extent = common::findings_for(&findings, "memory-local.extent");
    assert_eq!(extent.len(), 1, "{findings:#?}");
    assert!(extent[0].starts_with("::warning"), "{findings:#?}");
}

#[test]
fn emit_writes_no_lock_row_for_a_claude_local_md() {
    let harness = common::tmpdir("memory-local-emit");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    write_sibling(&harness, "CLAUDE.local.md", LOCAL_MEMORY);

    let row = temper::builtin_lock::declarations()
        .kinds
        .iter()
        .find(|row| row.name == "memory-local")
        .cloned()
        .expect("the embedded lock carries memory-local");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: drift::Declarations {
            kinds: vec![row],
            ..Default::default()
        },
        members: Vec::new(),
    };
    let report = drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    assert_eq!(
        fs::read_to_string(harness.join("CLAUDE.local.md")).unwrap(),
        LOCAL_MEMORY,
        "the local memory survives emit byte-identical: {:?}",
        report.entries
    );
    assert!(
        report
            .entries
            .iter()
            .all(|entry| entry.name != "CLAUDE.local"),
        "a local memory is neither projected nor reaped: {:?}",
        report.entries
    );
    let declarations = drift::read_declarations(&into).unwrap();
    assert_eq!(declarations.kinds[0].commitment.as_deref(), Some("local"));
    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        !lock.contains("source_path = \"CLAUDE.local.md\""),
        "no provenance row of the local memory lands in the lock:\n{lock}"
    );
}

#[test]
fn a_claude_md_is_a_committed_memory_member() {
    let memory = temper::builtin_kind::definition("memory").expect("memory ships");
    assert_eq!(memory.commitment, None);

    let harness = common::tmpdir("memory-committed");
    write_sibling(&harness, "CLAUDE.md", "# Team\n");
    let run = check_harness_in(&harness, Some("github"));
    assert!(run.ok, "{}", run.output);
    assert!(
        !run.announcements()
            .iter()
            .any(|line| line.contains("local member: memory")),
        "a committed CLAUDE.md is never announced as local: {:?}",
        run.announcements()
    );
}
