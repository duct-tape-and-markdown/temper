//! A file locus is a placement template: load refuses a custom kind whose `governs` glob
//! carries any pattern syntax beyond literal segments, one `*`, and a leading `**/`, and
//! every built-in kind's glob loads under the same rule.

use temper::drift::Declarations;
use temper::kind::{CustomKind, PLACEMENT_TEMPLATE_COLUMN};

use crate::common;

/// Run `check` over a lock declaring one custom `domain` kind at `docs` + `glob`, and
/// assert the run fails loud at load naming both the glob and the template rule.
fn assert_refused_at_load(case: &str, glob: &str) {
    let root = common::tmpdir(case);
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![common::kind_facts("domain", "docs", glob)],
            ..Declarations::default()
        },
    );
    let run = common::check_in(&root, &[], Some("github"));
    assert!(
        !run.ok,
        "a non-template locus `{glob}` must refuse at load, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains(glob),
        "the refusal names the offending glob, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains(PLACEMENT_TEMPLATE_COLUMN),
        "the refusal names the placement-template rule, got:\n{}",
        run.output
    );
}

#[test]
fn a_locus_glob_with_a_character_class_refuses_at_load() {
    assert_refused_at_load("locus-template-class", "domains/[a-z0-9-]*.md");
}

#[test]
fn a_locus_glob_with_an_extglob_refuses_at_load() {
    assert_refused_at_load("locus-template-extglob", "!(README).md");
}

#[test]
fn a_locus_glob_with_two_stars_refuses_at_load() {
    assert_refused_at_load("locus-template-two-stars", "*/*.md");
}

#[test]
fn a_locus_glob_with_braces_refuses_at_load() {
    assert_refused_at_load("locus-template-braces", "*.{md,txt}");
}

#[test]
fn every_built_in_kind_glob_is_a_placement_template() {
    let definitions = temper::builtin_kind::definitions();
    let mut seen = 0;
    for (name, kind) in &definitions {
        let Some(governs) = &kind.governs else {
            continue;
        };
        seen += 1;
        let row = common::kind_facts(name, &governs.root, &governs.glob);
        CustomKind::from_kind_fact_row(&row).unwrap_or_else(|err| {
            panic!(
                "built-in `{name}`'s glob `{}` loads as a template: {err}",
                governs.glob
            )
        });
    }
    assert!(
        seen > 0,
        "the built-in population governs at least one locus"
    );
}
