//! The memory kind's **File shape** round trip, end to end (`specs/architecture/15-kinds.md`,
//! "A built-in kind is an adapter — two faces").
//!
//! The gate already *dispatches* memory members (`tests/memory_gate.rs` drives
//! `check --harness` and proves the `max_lines` advisory fires/stays-silent,
//! un-double-reported across the two providers). What no test carried was the
//! **adapter read/emit face over a File-shaped unit with no YAML frontmatter** — the
//! retired `adapter_fidelity.rs` exercised the frontmatter faces only over `.claude/`
//! members that all declare `format = "yaml-frontmatter"`, never a repo-root
//! frontmatterless `CLAUDE.md`. This pins that gap:
//!
//! - a `unit_shape = "file"`, `format`-less `claude-code.memory` member (a repo-root
//!   `CLAUDE.md`) compiles from a hand-built seam payload (`tests/emit.rs`'s pattern —
//!   `emit` is the sole producer, `specs/architecture/20-surface.md`, "The lock and
//!   drift") and **re-emits idempotently** — the emit face projects it to its harness
//!   locus with no fabricated frontmatter, and a second emit changes not one byte;
//! - the whole frontmatterless file is the byte-faithful body — the read face lifts no
//!   clause module, because the source declares no frontmatter field;
//! - the two bare-`memory` providers (`claude-code.memory` + `agents-md.memory`)
//!   resolve **by qualified identity** — a non-colliding bare lookup resolves cleanly,
//!   and a bare `memory` reference is the `AmbiguousKind` load error naming both
//!   qualified candidates (`specs/architecture/15-kinds.md`, "Decision: kind identity
//!   carries a provider axis").
//!
//! `max_lines` fire/silent is *not* re-tested here — `memory_gate.rs` already owns it
//! across the real `check --harness` process boundary.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::builtin_kind;
use temper::drift::{self, Declarations, EmitOptions, KindFactRow, Payload, PayloadMember};
use temper::frontmatter::Member;
use temper::kind::KindError;

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "memory-contract-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A repo-root `CLAUDE.md` in exactly the `claude-code.memory` shape: **no YAML
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

/// The `memory` kind's declaration row over the `claude-code` provider's own governs
/// locus (`root = "."`, `glob = "CLAUDE.md"`) — the golden lock fact standing in for a
/// live kind, since `emit` needs no harness scan to compile a hand-built payload.
fn memory_kind_facts() -> KindFactRow {
    KindFactRow {
        name: "memory".to_string(),
        provider: Some("claude-code".to_string()),
        governs_root: ".".to_string(),
        governs_glob: "CLAUDE.md".to_string(),
        format: None,
        unit_shape: Some("file".to_string()),
        activation: None,
    }
}

/// The read/emit face over a File-shaped frontmatterless member: `emit` compiles a
/// repo-root `CLAUDE.md` from a hand-built seam payload and re-emits idempotently — the
/// emit face projects it to its harness locus with no fabricated frontmatter, and a
/// second emit changes not one byte — round-trip discipline (`.claude/rules/rust.md`)
/// over the shape `adapter_fidelity.rs` never covered.
#[test]
fn a_frontmatterless_claude_md_emits_and_re_emits_idempotently() {
    let harness = tmpdir("roundtrip");
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
            fields: Vec::new(),
            body: CLAUDE_MD.to_string(),
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
        .unwrap()
        .remove("claude-code.memory")
        .expect("the embedded claude-code.memory kind resolves by qualified identity");
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

/// The two `memory` providers co-embed the bare name by design, so they resolve by
/// **qualified identity**: a non-colliding bare lookup resolves cleanly with no spurious
/// collision, and a bare `memory` reference is the `AmbiguousKind` load error naming both
/// qualified candidates (`specs/architecture/15-kinds.md`, "Decision: kind identity
/// carries a provider axis").
#[test]
fn the_two_memory_providers_resolve_by_qualified_identity() {
    let defs = builtin_kind::definitions().unwrap();

    // Both providers are distinct entries under distinct qualified keys — the same bare
    // `name`, two qualified identities, neither overwriting the other.
    let claude_code = defs
        .get("claude-code.memory")
        .expect("claude-code.memory is an embedded entry");
    let agents_md = defs
        .get("agents-md.memory")
        .expect("agents-md.memory is an embedded entry");
    assert_eq!(claude_code.name, "memory");
    assert_eq!(agents_md.name, "memory");
    assert_eq!(claude_code.qualified_name(), "claude-code.memory");
    assert_eq!(agents_md.qualified_name(), "agents-md.memory");

    // A non-colliding bare lookup resolves cleanly to its unique carrier — the two
    // `memory` providers meeting costs no qualification tax on an unrelated name.
    assert_eq!(
        builtin_kind::qualified("skill").unwrap().as_deref(),
        Some("claude-code.skill"),
        "a bare `skill` resolves to its unique carrier, uncollided by the two memory kinds"
    );
    assert_eq!(
        builtin_kind::qualified("rule").unwrap().as_deref(),
        Some("claude-code.rule"),
        "a bare `rule` resolves to its unique carrier, uncollided by the two memory kinds"
    );

    // A bare `memory` reference is the load error — naming both qualified candidates so an
    // author binding it knows what to disambiguate against.
    match builtin_kind::qualified("memory") {
        Err(KindError::AmbiguousKind { name, candidates }) => {
            assert_eq!(name, "memory");
            assert!(
                candidates.contains("claude-code.memory"),
                "candidates name claude-code.memory, got: {candidates}"
            );
            assert!(
                candidates.contains("agents-md.memory"),
                "candidates name agents-md.memory, got: {candidates}"
            );
        }
        other => panic!("expected AmbiguousKind for bare `memory`, got {other:?}"),
    }
}
