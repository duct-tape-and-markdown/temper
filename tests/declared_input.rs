//! A declared input — the file a member's claims rest on: resolved and fingerprinted at
//! emit, with nothing copied anywhere.
//!
//! A member declares an input and the lock carries its hash under the `input` family, so a
//! moved input is drift. Unlike an include, no byte reaches the projection: the host's
//! artifact is what it would have been without the input, the row names no member edge,
//! and the target is never decoded — a binary input is legal. A dangling input refuses
//! before a byte is written. The finding's remedy is its own: re-verify the member's
//! claims against the input, then re-emit.

use std::fs;

use temper::drift::{self, Declarations, EmitOptions, InputRow, KindFactRow, Payload};

mod common;

/// The shipped root default's own `fresh` clause — the value `gate` threads into
/// `drift::input_stale`, taken from the embedded lock so a drift assertion here reads the
/// same label and severity a real `check` reports under.
fn fresh_clause() -> temper::contract::Clause {
    temper::builtin::root_contract()
        .clauses
        .into_iter()
        .find(|clause| clause.predicate == temper::contract::Predicate::Fresh)
        .expect("the shipped root default binds `fresh`")
}

/// A `rule` kind governing `.claude/rules/*.md` — a plain markdown, field-less projection,
/// so the emitted artifact is the authored body verbatim and any extra byte would show.
fn rule_kind() -> KindFactRow {
    common::kind_facts("rule", ".claude/rules", "*.md")
}

/// The body every case's host rule authors — the whole of what its projection may carry.
const HOST_BODY: &str = "The legacy handler still validates on write.\n";

/// A payload whose one `rule:host` member declares `input` as its input.
fn payload_declaring(input: &std::path::Path) -> Payload {
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![rule_kind()],
            inputs: vec![InputRow {
                member: "rule:host".to_string(),
                source_path: input.to_string_lossy().into_owned(),
            }],
            ..Default::default()
        },
        members: vec![common::rule_member("host", None, HOST_BODY)],
    }
}

/// The bytes emit writes for a frontmatterless markdown projection of [`HOST_BODY`].
fn projected_host() -> String {
    format!("{}\n\n{HOST_BODY}", temper::placement::BANNER)
}

#[test]
fn an_input_is_fingerprinted_without_moving_a_byte_into_the_projection() {
    let harness = common::scaffold("declared-input-fingerprint");
    let into = harness.join(".temper");
    // The input — a committed code file the member's claim rests on, governed by no kind.
    fs::create_dir_all(harness.join("snapshot")).unwrap();
    fs::write(
        harness.join("snapshot/handler.py"),
        "def handle(request):\n    validate(request)\n",
    )
    .unwrap();

    drift::emit(
        &payload_declaring(&harness.join("snapshot/handler.py")),
        &into,
        EmitOptions::default(),
    )
    .unwrap();

    // The whole difference from an include: the projection is the authored body alone.
    assert_eq!(
        fs::read_to_string(harness.join(".claude/rules/host.md")).unwrap(),
        projected_host(),
        "a declared input copies nothing into the member it hangs off"
    );

    // The dependency reached the lock under its own family: the declaring member, the
    // resolved path, a hash — and no edge, since an input's target need not be a member.
    let inputs = drift::inputs(&into).unwrap();
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].member, "rule:host");
    assert_eq!(inputs[0].source_path, "snapshot/handler.py");
    assert!(inputs[0].target.is_empty());
    assert!(!inputs[0].import_hash.is_empty());

    // The `include` family is the input's sibling, not its home — nothing crossed over.
    assert!(drift::includes(&into).unwrap().is_empty());
}

#[test]
fn a_moved_input_fires_one_finding_naming_the_member_and_its_own_remedy() {
    let harness = common::scaffold("declared-input-drift");
    let into = harness.join(".temper");
    fs::create_dir_all(harness.join("snapshot")).unwrap();
    fs::write(
        harness.join("snapshot/handler.py"),
        "def handle(request):\n    validate(request)\n",
    )
    .unwrap();

    drift::emit(
        &payload_declaring(&harness.join("snapshot/handler.py")),
        &into,
        EmitOptions::default(),
    )
    .unwrap();

    let clause = fresh_clause();
    assert!(drift::input_stale(&into, &clause).unwrap().is_empty());

    // The ground the claim rests on moves.
    fs::write(
        harness.join("snapshot/handler.py"),
        "def handle(request):\n    pass\n",
    )
    .unwrap();

    let stale = drift::input_stale(&into, &clause).unwrap();
    assert_eq!(stale.len(), 1, "a moved input is drift: {stale:?}");
    assert_eq!(
        stale[0].rule, clause.label,
        "it reports under the `fresh` clause's own label, never a baked rule id"
    );
    assert!(
        stale[0].message.contains("rule:host") && stale[0].message.contains("snapshot/handler.py"),
        "the finding names the dependent member and the input: {}",
        stale[0].message
    );
    assert!(
        stale[0]
            .message
            .contains("re-verify the member's claims against the input, then re-emit"),
        "and routes the author to the claim before the re-emit that would bless the new bytes: {}",
        stale[0].message
    );
}

#[test]
fn a_dangling_input_refuses_before_any_byte_is_written() {
    let harness = common::scaffold("declared-input-dangling");
    let into = harness.join(".temper");

    // The host declares an input that is not there, beside a sibling rule whose projection
    // would land were emit to reach its write pass.
    let mut payload = payload_declaring(&harness.join("snapshot/missing.py"));
    payload
        .members
        .push(common::rule_member("sibling", None, "# Sibling\n"));

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let rendered = format!("{err:?}");
    assert!(rendered.contains("dangling"), "names the fault: {rendered}");
    assert!(
        rendered.contains("missing.py"),
        "names the input: {rendered}"
    );

    assert!(!harness.join(".claude/rules/host.md").exists());
    assert!(
        !harness.join(".claude/rules/sibling.md").exists(),
        "no projection is written when a sibling member's input dangles"
    );
    assert!(drift::inputs(&into).unwrap().is_empty());
}

#[test]
fn a_non_utf8_input_is_legal() {
    let harness = common::scaffold("declared-input-binary");
    let into = harness.join(".temper");
    // No byte of an input moves into a projection, so nothing decodes it — a compiled
    // artifact is as fingerprintable as a source file.
    fs::create_dir_all(harness.join("snapshot")).unwrap();
    fs::write(harness.join("snapshot/model.bin"), [0xff, 0xfe, 0x00, 0x01]).unwrap();

    drift::emit(
        &payload_declaring(&harness.join("snapshot/model.bin")),
        &into,
        EmitOptions::default(),
    )
    .unwrap();

    let inputs = drift::inputs(&into).unwrap();
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].source_path, "snapshot/model.bin");
    assert!(
        drift::input_stale(&into, &fresh_clause())
            .unwrap()
            .is_empty(),
        "and it reads fresh against the baseline emit just wrote"
    );
}

#[test]
fn an_input_above_the_harness_root_records_a_relative_path_that_resolves_in_any_checkout() {
    // An input's path arrives SDK-resolved and absolute, and a target the harness root is
    // not a prefix of used to ride the lock in that form — so the committed row named the
    // machine that emitted it and resolved nowhere else. The row is `../`-segmented from
    // the root instead, the spelling a layout region's import above the root already
    // records: two checkouts of one repo, one lock, and the read side joins it back.
    let mut locks = Vec::new();
    let mut workspaces = Vec::new();

    for label in ["input-above-root-a", "input-above-root-b"] {
        let checkout = common::tmpdir(label);
        // The harness two directories down, the input at the checkout root — the source
        // file a rule's claim rests on, outside the harness it governs.
        let into = checkout.join("packages/agent/.temper");
        fs::create_dir_all(&into).unwrap();
        fs::create_dir_all(checkout.join("src")).unwrap();
        let input = checkout.join("src/x.txt");
        fs::write(&input, "def handle(request):\n    validate(request)\n").unwrap();

        drift::emit(&payload_declaring(&input), &into, EmitOptions::default()).unwrap();

        let inputs = drift::inputs(&into).unwrap();
        assert_eq!(
            inputs[0].source_path, "../../src/x.txt",
            "an input above the root is spelled with `..` from the root, never absolutely"
        );
        locks.push(fs::read_to_string(into.join("lock.toml")).unwrap());
        workspaces.push(into);
    }

    assert_eq!(
        locks[0], locks[1],
        "the same program emitted from two checkout paths writes one lock"
    );

    // The read side of the same row: the second checkout's `check` joins `../../src/x.txt`
    // onto its own root, reads the file there, and finds it fresh.
    let clause = fresh_clause();
    assert!(
        drift::input_stale(&workspaces[1], &clause)
            .unwrap()
            .is_empty(),
        "a `../`-segmented row resolves under whatever root the reader holds"
    );
}
