//! The non-file locus's `registration` field: like any locus with no file of its own,
//! an embedded (or nested-file) member loads through its host, never on its own, so
//! it registers nothing — its registration is empty, and a non-empty registration is a
//! well-formedness fault.
//!
//! The fixture kind is a lock-declared kind of the suite's own, never a shipped kind,
//! so these cases falsify the *locus* rather than one kind's use of it. It declares
//! a non-file locus (nested-file, `governs: None`) to test the fault — the same
//! shape that supporting-doc (the sole shipped `governs: None` kind) uses under its
//! own empty registration.

use temper::drift::{Declarations, KindFactRow};

mod common;

/// A `posture` kind's fact row: a **nested-file**-locus (governs: None) prose-only kind.
fn posture_kind_facts() -> KindFactRow {
    KindFactRow {
        name: "posture".to_string(),
        governs_root: None,
        governs_glob: None,
        ..common::kind_facts("posture", "", "")
    }
}

#[test]
fn a_governs_none_kind_declaring_a_non_empty_registration_is_a_fault() {
    // A nested-file kind's members load through their host, so a declared registration
    // channel is meaningless. The gate must trip a well-formedness fault.
    let harness = common::tmpdir("registration-locus-fault");
    std::fs::create_dir_all(harness.join(".temper")).unwrap();

    let mut kind = posture_kind_facts();
    kind.registration = vec!["always".to_string()];

    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![kind],
            ..Default::default()
        },
    );

    let (findings, ok) = common::check_harness(&harness);

    assert!(
        !ok,
        "a governs-None kind declaring a non-empty registration fails: {findings:?}"
    );
    assert_eq!(
        common::findings_for(&findings, "kind.registration-locus").len(),
        1,
        "one finding names the fault: {findings:?}"
    );
    assert!(
        findings[0].contains("posture"),
        "the finding names the kind: {findings:?}"
    );
}

#[test]
fn a_governs_none_kind_with_empty_registration_is_clean() {
    // A nested-file kind with an empty registration is clean — that is its correct form.
    let harness = common::tmpdir("registration-locus-clean");
    std::fs::create_dir_all(harness.join(".temper")).unwrap();

    let kind = posture_kind_facts();
    // registration defaults to Vec::new()

    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![kind],
            ..Default::default()
        },
    );

    let (findings, ok) = common::check_harness(&harness);

    assert!(
        ok,
        "a governs-None kind with empty registration is clean: {findings:?}"
    );
    assert!(
        common::findings_for(&findings, "kind.registration-locus").is_empty(),
        "no registration-locus finding fires: {findings:?}"
    );
}

#[test]
fn every_shipped_builtin_stays_clean_under_the_new_gate() {
    // The sole shipped governs-None built-in, supporting-doc, has an empty registration
    // by design. The gate must not break it.
    let harness = common::tmpdir("registration-locus-builtins");
    std::fs::create_dir_all(harness.join(".temper")).unwrap();

    // Write a minimal lock with no custom kinds — the gate checks both builtins and
    // custom kinds. Builtins are loaded from their definition.
    common::write_lock(
        &harness,
        Declarations {
            ..Default::default()
        },
    );

    let (findings, ok) = common::check_harness(&harness);

    assert!(ok, "shipped builtins pass the new gate: {findings:?}");
    assert!(
        common::findings_for(&findings, "kind.registration-locus").is_empty(),
        "no registration-locus finding fires on builtins: {findings:?}"
    );
}
