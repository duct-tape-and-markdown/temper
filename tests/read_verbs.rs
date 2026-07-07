//! Proofs over the unified read verb, `explain` (`specs/architecture/20-surface.md`, "Decision:
//! one read verb — `explain`"): target-species resolution (member / requirement / leaf
//! address), the member-vs-requirement collision error, the qualified-prefix escape
//! hatch, and coverage disclosure — plus the surviving library-level floor-binding
//! proofs over `why` (READ-FLOOR-BINDING-DEFAULT), one of the four traversals `explain`
//! re-homes.
//!
//! The four read *CLI verbs* (`why`/`requirements`/`impact`/`context`) retired at
//! CLI-COLLAPSE; `explain <target>` is their sole CLI spelling as of EXPLAIN-UNIFY. The
//! traversal *engine* survives untouched ([`temper::read`]) for `explain` to reuse, so
//! this file exercises the read library directly rather than spawning the binary.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::check::Workspace;
use temper::compose::Requirement;
use temper::document::PublishedRequirement;
use temper::extract::{FeatureValue, Features, GenreValue, ValueType};
use temper::read::{self, CustomMember};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-read-verbs-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A member's [`Features`] as the read family reads them: its id, the requirements it
/// opts into, the demands it publishes, and a `description` field (so `impact`'s
/// reachability strand has a non-panicking registration input). Mirrors `read.rs`'s own
/// `impact_tests::feature` helper — duplicated here since this file, being outside the
/// crate, can only build `Features` through its public fields.
fn feature(id: &str, satisfies: &[&str], published: &[&str]) -> Features {
    let mut fields = BTreeMap::new();
    fields.insert(
        "description".to_string(),
        FeatureValue::scalar(ValueType::String, "d"),
    );
    Features {
        id: id.to_string(),
        fields,
        body_lines: 1,
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: Some(id.to_string()),
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        genres: Vec::new(),
        satisfies: satisfies.iter().map(|s| (*s).to_string()).collect(),
        published_requirements: published
            .iter()
            .map(|name| PublishedRequirement {
                name: (*name).to_string(),
                means: None,
                kind: None,
                required: true,
            })
            .collect(),
    }
}

/// A `required`/advisory requirement with everything else defaulted.
fn req(name: &str, required: bool) -> Requirement {
    Requirement {
        name: name.to_string(),
        means: None,
        kind: None,
        required,
        clauses: Vec::new(),
        verified_by: None,
    }
}

/// Call `read::explain` over an empty surface workspace, the given custom members
/// (`why`/`requirements`'s own member listing), and the given by-kind corpus + roster
/// (`impact`/`context`'s corpus, and the species-resolution existence check) — every
/// scenario below only needs those to drive target-species resolution. Kept in sync
/// like `main.rs`'s own `explain` wiring keeps its `Workspace` and by-kind corpus in
/// sync (both read off the same surface directory there); here the caller threads
/// matching ids into both by hand.
fn explain(
    custom: &[CustomMember],
    by_kind: &BTreeMap<&str, &[Features]>,
    roster: &BTreeMap<String, Requirement>,
    target: &str,
) -> String {
    let ws = Workspace::load(&tmpdir("explain")).unwrap();
    let assembly: BTreeMap<String, Requirement> = BTreeMap::new();
    let registrations = BTreeMap::new();
    read::explain(
        &ws,
        custom,
        &assembly,
        roster,
        by_kind,
        &[],
        &registrations,
        &[],
        &[],
        &[],
        target,
    )
}

#[test]
fn a_member_target_walks_why_impact_and_context() {
    // `solo` names no requirement, so it resolves unambiguously as a member and its
    // explanation carries all three member-grain traversals: why's forward walk,
    // impact's blast radius, and context's neighborhood. `why`/`requirements` read a
    // member's existence off `custom` (not `by_kind`), so it is threaded into both.
    let custom = [CustomMember {
        kind: "spec".to_string(),
        id: "solo".to_string(),
        satisfies: Vec::new(),
    }];
    let members = [feature("solo", &[], &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    let out = explain(&custom, &by_kind, &roster, "solo");
    assert!(
        out.contains("everything that holds it in place"),
        "the member species includes why's forward walk: {out}"
    );
    assert!(
        out.contains("the blast radius if it is removed or renamed"),
        "the member species includes impact's blast radius: {out}"
    );
    assert!(
        out.contains("its declared neighborhood"),
        "the member species includes context's neighborhood: {out}"
    );
}

#[test]
fn a_requirement_target_walks_the_reverse_roster() {
    // `only-req` names no member, so it resolves unambiguously as a requirement and
    // narrates through `requirements`'s reverse walk (satisfier set, coverage, blast
    // radius) alone. `filler` satisfies it (`why`/`requirements` read satisfiers off
    // `custom`, with rationale, never off `by_kind`'s decidable `Features::satisfies`).
    let custom = [CustomMember {
        kind: "spec".to_string(),
        id: "filler".to_string(),
        satisfies: vec![temper::document::Satisfies::new("only-req")],
    }];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::new();
    let roster = BTreeMap::from([("only-req".to_string(), req("only-req", true))]);

    let out = explain(&custom, &by_kind, &roster, "only-req");
    assert!(out.contains("Requirement `only-req`:"), "{out}");
    assert!(out.contains("Satisfied by:"), "{out}");
    assert!(out.contains("`filler`"), "{out}");
    // Member-grain traversals never fire for a requirement target.
    assert!(!out.contains("everything that holds it in place"), "{out}");
}

#[test]
fn a_leaf_address_walks_impact_and_context_at_leaf_grain_and_discloses_coverage() {
    let mut leafy = feature("20-surface", &[], &[]);
    leafy.genres = vec![GenreValue {
        genre: "decision".to_string(),
        key: "surface-authority".to_string(),
        leaves: BTreeMap::from([("chosen".to_string(), "the surface is canonical".to_string())]),
        collections: BTreeMap::new(),
    }];
    let members = [leafy];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    // A leaf address never dispatches to `why`/`requirements`, so no custom member is
    // needed to back it.
    let out = explain(
        &[],
        &by_kind,
        &roster,
        "20-surface/decision/surface-authority/chosen",
    );
    assert!(
        out.contains("leaf grain:"),
        "impact's leaf-grain header: {out}"
    );
    assert!(
        out.contains("its declared neighborhood"),
        "context's leaf-grain header: {out}"
    );
    assert!(out.contains("Fallout: none"), "{out}");
    assert!(
        out.contains("Coverage:"),
        "a leaf-grain answer discloses coverage (law 1): {out}"
    );
}

#[test]
fn a_member_vs_requirement_collision_errors_with_both_qualified_spellings() {
    // `shared` is both a member id and a requirement name — `explain` never guesses
    // which the author meant. Species resolution checks `by_kind` alone (never
    // `custom`), so no custom member is needed to trigger the collision.
    let members = [feature("shared", &[], &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &members[..])]);
    let roster = BTreeMap::from([("shared".to_string(), req("shared", false))]);

    let out = explain(&[], &by_kind, &roster, "shared");
    assert!(
        out.contains("names more than one thing"),
        "the collision is reported, never silently resolved: {out}"
    );
    assert!(out.contains("`member:shared`"), "{out}");
    assert!(out.contains("`requirement:shared`"), "{out}");
}

#[test]
fn a_qualified_prefix_resolves_directly_even_when_ambiguous() {
    // The same collision as above, but spelled explicitly — the qualifier is always
    // accepted, bypassing the ambiguity check entirely. `shared` is backed as a custom
    // member too, so the `member:` spelling's `why` walk actually resolves.
    let custom = [CustomMember {
        kind: "spec".to_string(),
        id: "shared".to_string(),
        satisfies: Vec::new(),
    }];
    let members = [feature("shared", &[], &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster = BTreeMap::from([("shared".to_string(), req("shared", false))]);

    let as_member = explain(&custom, &by_kind, &roster, "member:shared");
    assert!(
        as_member.contains("everything that holds it in place"),
        "an explicit `member:` spelling resolves as a member, no collision error: {as_member}"
    );

    let as_requirement = explain(&custom, &by_kind, &roster, "requirement:shared");
    assert!(
        as_requirement.contains("Requirement `shared`:"),
        "an explicit `requirement:` spelling resolves as a requirement: {as_requirement}"
    );
}

#[test]
fn an_unrecognized_target_is_a_clean_read_naming_no_namespace() {
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::new();
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    let out = explain(&[], &by_kind, &roster, "ghost");
    assert!(
        out.contains("No member, requirement, or leaf address named `ghost`"),
        "{out}"
    );
}

/// Floor-binding narration over the read family's public `why` API (READ-FLOOR-BINDING-DEFAULT):
/// a floor is named for its kind — never a `<kind>.<source>` package, and never
/// every non-rule kind defaulting to the skill floor. A memory member is threaded
/// as a custom member the way a built-in reaches the read family. Skills/rules keep
/// their own floors — these exercise the resolution branches directly.
mod floor_binding {
    use std::collections::BTreeMap;

    use temper::check::Workspace;
    use temper::compose::Requirement;
    use temper::extract::Features;
    use temper::read::{self, CustomMember};

    use super::tmpdir;

    /// Narrate one custom member (its `kind` and `id`) through `why` over an otherwise-empty
    /// surface, returning the stdout narration. The workspace loads an empty temp dir (no
    /// skills/rules) and the roster/edge inputs are empty, so the governing-floor line is
    /// all this exercises.
    fn why_kind(kind: &str, id: &str) -> String {
        let ws = Workspace::load(&tmpdir("floor-binding")).unwrap();
        let custom = [CustomMember {
            kind: kind.to_string(),
            id: id.to_string(),
            satisfies: Vec::new(),
        }];
        let roster: BTreeMap<String, Requirement> = BTreeMap::new();
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::new();
        read::why(&ws, &custom, &roster, &by_kind, &[], id)
    }

    #[test]
    fn a_memory_member_names_its_own_floor_never_the_skill_floor() {
        // `memory` binds the `memory` floor — never mis-narrated as the `skill` floor,
        // the default-to-skill bug this entry closes.
        let memory = why_kind("memory", "project-memory");
        assert!(
            memory.contains("binds the `memory` floor"),
            "a memory member is bound to its own floor: {memory}"
        );
        assert!(
            !memory.contains("binds the `skill` floor"),
            "a memory member is never narrated as skill-bound: {memory}"
        );
    }

    #[test]
    fn a_builtin_name_resolves_to_its_bound_floor() {
        // `skill`/`rule` each name their own floor — the kind's bare label.
        let skill = why_kind("skill", "reviewer");
        assert!(skill.contains("binds the `skill` floor"), "{skill}");
        let rule = why_kind("rule", "collaboration");
        assert!(rule.contains("binds the `rule` floor"), "{rule}");
    }

    #[test]
    fn a_floorless_kind_falls_back_to_its_own_name() {
        // A kind with no author binding and no embedded floor is named by its own kind name,
        // not silently mis-bound to the skill floor.
        let out = why_kind("adr", "0001-adopt-temper");
        assert!(out.contains("binds the `adr` floor"), "{out}");
        assert!(!out.contains("binds the `skill` floor"), "{out}");
    }
}
