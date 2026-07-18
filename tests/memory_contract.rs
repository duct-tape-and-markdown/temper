//! The memory kind's **File shape** round trip, end to end.
//!
//! The gate already *dispatches* memory members (`tests/memory_gate.rs` drives
//! `check --harness` and proves the `extent` advisory fires/stays-silent). What no
//! test carried was the **adapter read/emit face over a File-shaped unit with no YAML
//! frontmatter** — the retired `adapter_fidelity.rs` exercised the frontmatter faces
//! only over `.claude/` members that all declare `format = "yaml-frontmatter"`, never
//! a repo-root frontmatterless `CLAUDE.md`. This pins that gap:
//!
//! - a `unit_shape = "file"`, `format`-less `memory` member (a repo-root `CLAUDE.md`)
//!   compiles from a hand-built seam payload and
//!   **re-emits idempotently** — the emit face projects it to its harness locus with
//!   no fabricated frontmatter, and a second emit changes not one byte;
//! - the whole frontmatterless file is the byte-faithful body — the read face lifts no
//!   clause module, because the source declares no frontmatter field.
//!
//! `extent` fire/silent is *not* re-tested here — `memory_gate.rs` already owns it
//! across the real `check --harness` process boundary.

use std::fs;

mod common;

use temper::builtin_kind;
use temper::drift::{self, Declarations, EmitOptions, KindFactRow, Payload, PayloadMember};
use temper::frontmatter::Member;

/// A repo-root `CLAUDE.md` in exactly the `memory` kind's shape: **no YAML
/// frontmatter** (plain markdown Claude Code reads at session start), headings and a
/// section, and — deliberately — no final newline, so a byte-faithful body copy is
/// observable and a re-render that "tidied" the trailing whitespace would be caught.
const CLAUDE_MD: &str = "# temper\n\
\n\
Guidance the harness reads into every session.\n\
\n\
## Conventions\n\
\n\
Prefer a clone over a lifetime fight.   \n\
Keep it correct, clear, well-tested.";

/// The `memory` kind's declaration row over its own governs locus (`root = "."`,
/// `glob = "CLAUDE.md"`) — the golden lock fact standing in for a live kind, since
/// `emit` needs no harness scan to compile a hand-built payload.
fn memory_kind_facts() -> KindFactRow {
    KindFactRow {
        unit_shape: Some("file".to_string()),
        ..common::kind_facts("memory", ".", "CLAUDE.md")
    }
}

/// The read/emit face over a File-shaped frontmatterless member: `emit` compiles a
/// repo-root `CLAUDE.md` from a hand-built seam payload and re-emits idempotently — the
/// emit face projects it to its harness locus with no fabricated frontmatter, and a
/// second emit changes not one byte — round-trip discipline (`.claude/rules/rust.md`)
/// over the shape `adapter_fidelity.rs` never covered.
#[test]
fn a_frontmatterless_claude_md_emits_and_re_emits_idempotently() {
    let harness = common::tmpdir("roundtrip");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();

    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![memory_kind_facts()],
            ..Declarations::default()
        },
        members: vec![PayloadMember {
            kind: "memory".to_string(),
            name: "CLAUDE".to_string(),
            host: None,
            fields: Vec::new(),
            body: CLAUDE_MD.to_string(),
            source_path: None,
        }],
    };

    // Emit face: a frontmatterless member (no `fields`) projects as the byte-faithful
    // body alone — no fabricated `---\n---\n` header.
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let claude_md = harness.join("CLAUDE.md");
    assert_eq!(
        fs::read_to_string(&claude_md).unwrap(),
        CLAUDE_MD,
        "a frontmatterless memory member projects as its byte-faithful body alone"
    );

    // Idempotence: a second emit over the unchanged payload changes not one byte — the
    // read→emit faces compose to a fixpoint (`.claude/rules/rust.md`).
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(
        fs::read_to_string(&claude_md).unwrap(),
        CLAUDE_MD,
        "a re-emit of the unchanged payload must change not one byte"
    );

    // Body faithfulness, at the adapter read face directly over the real embedded kind: a
    // frontmatterless file lifts no field, and the whole file is the byte-faithful body
    // (trailing-whitespace line and missing final newline included) — no fabricated
    // frontmatter, no re-render.
    let kind = builtin_kind::definitions()
        .remove("memory")
        .expect("the embedded memory kind is present");
    let member = Member::from_source_rooted(&kind, &claude_md, &harness).unwrap();
    assert!(
        member.fields.is_empty(),
        "a frontmatterless source declares no field, so the read face lifts no clause"
    );
    assert_eq!(
        member.body, CLAUDE_MD,
        "the whole frontmatterless file is the byte-faithful body"
    );
}
