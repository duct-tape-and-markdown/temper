//! The memory kind's **File shape** round trip, end to end (`specs/architecture/15-kinds.md`,
//! "A built-in kind is an adapter — two faces").
//!
//! The gate already *dispatches* memory members (`tests/memory_gate.rs` drives
//! `check --harness` and proves the `max_lines` advisory fires/stays-silent,
//! un-double-reported across the two providers). What no test carried was the
//! **adapter read/emit face over a File-shaped unit with no YAML frontmatter**:
//! `adapter_fidelity.rs` exercises the frontmatter faces over `.claude/` members that
//! all declare `format = "yaml-frontmatter"`, never a repo-root frontmatterless
//! `CLAUDE.md`. This pins that gap:
//!
//! - a `unit_shape = "file"`, `format`-less `claude-code.memory` member (a repo-root
//!   `CLAUDE.md`) imports and **re-emits idempotently** — the import face projects it
//!   to its surface member document, a re-import changes not one byte (`insta`
//!   snapshot), and the emit→reload cycle is a byte fixpoint;
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

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use temper::builtin_kind;
use temper::frontmatter::Member;
use temper::import;
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

/// The `source_path` recorded in `lock.toml` / the member document's `[provenance]` is
/// the absolute origin of the scratch harness, which varies per run and per machine.
/// Redact just that path so the snapshot pins everything content-derived (the hash, the
/// header shape, the body) without pinning an unstable absolute path.
fn memory_filters() -> Vec<(&'static str, &'static str)> {
    vec![(
        r#"source_path = "[^"]*CLAUDE\.md""#,
        r#"source_path = "[HARNESS]/CLAUDE.md""#,
    )]
}

/// Render an imported surface tree as a single reviewable string: each file as a
/// `--- <relative/path> ---` header (forward slashes) followed by its contents, sorted
/// by path. Two imports rendering identically *is* the byte-stable / no-diff contract.
fn render_surface(dir: &Path) -> String {
    let mut files = BTreeMap::new();
    for entry in walkdir::WalkDir::new(dir).min_depth(1).sort_by_file_name() {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(dir)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        files.insert(rel, fs::read_to_string(entry.path()).unwrap());
    }

    let mut out = String::new();
    for (rel, body) in files {
        out.push_str(&format!("--- {rel} ---\n"));
        out.push_str(&body);
        if !body.ends_with('\n') {
            out.push('\n');
        }
    }
    out
}

/// The read/emit face over a File-shaped frontmatterless member: `temper import`
/// projects a repo-root `CLAUDE.md` to its surface member document, a re-import is
/// byte-identical, and the emit→reload cycle is a byte fixpoint — round-trip discipline
/// (`.claude/rules/rust.md`) over the shape `adapter_fidelity.rs` never covered.
#[test]
fn a_frontmatterless_claude_md_imports_and_re_emits_idempotently() {
    let harness = tmpdir("roundtrip-src");
    fs::write(harness.join("CLAUDE.md"), CLAUDE_MD).unwrap();

    // Read face: `import` discovers the repo-root `CLAUDE.md` off the `claude-code.memory`
    // `governs` locus (`root = "."`, `glob = "CLAUDE.md"`) and projects it to the surface.
    let into = tmpdir("roundtrip-into");
    import::run(&harness, &into).unwrap();

    // Emit face: the projected surface — a `[[memory]]` lock row plus the member document
    // `CLAUDE/MEMORY.md`, whose `+++` header carries only `[provenance]` (a frontmatterless
    // source lifts no clause module) over the byte-faithful body. Snapshot it whole.
    let surface = render_surface(&into);
    insta::with_settings!({filters => memory_filters()}, {
        insta::assert_snapshot!("frontmatterless_memory_surface", surface);
    });

    // Idempotence: re-importing the unchanged harness into a fresh surface reproduces
    // every byte — the read→emit faces compose to a fixpoint (`.claude/rules/rust.md`).
    let into2 = tmpdir("roundtrip-into2");
    import::run(&harness, &into2).unwrap();
    assert_eq!(
        surface,
        render_surface(&into2),
        "a re-import of the unchanged CLAUDE.md must change not one byte"
    );

    // Reload fixpoint: the written member document read back through the surface face and
    // re-emitted is byte-identical — `Member::from_surface` and `to_document` are inverses
    // over the frontmatterless member, no drift on the emit side.
    let member_dir = into.join("CLAUDE");
    let reloaded = Member::from_surface(&member_dir, "MEMORY.md").unwrap();
    assert_eq!(
        reloaded.to_document().emit(),
        fs::read_to_string(member_dir.join("MEMORY.md")).unwrap(),
        "reloading and re-emitting the member document must be a byte fixpoint"
    );

    // Body faithfulness, at the adapter read face directly over the real embedded kind: a
    // frontmatterless file lifts no field, and the whole file is the byte-faithful body
    // (trailing-whitespace line and missing final newline included) — no fabricated
    // frontmatter, no re-render.
    let kind = builtin_kind::definitions()
        .unwrap()
        .remove("claude-code.memory")
        .expect("the embedded claude-code.memory kind resolves by qualified identity");
    let member = Member::from_source_rooted(&kind, &harness.join("CLAUDE.md"), &harness).unwrap();
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
