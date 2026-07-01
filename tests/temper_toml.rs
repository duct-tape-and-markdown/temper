//! End-to-end acceptance over the author-declared `temper.toml` layer
//! (`specs/40-composition.md`, "Decision: the author-declared contract lives in
//! `temper.toml`, layered").
//!
//! Drives the built `temper` binary so the *whole* path is pinned — `temper.toml`
//! discovery at the project root (the invocation dir), the layering of its
//! per-kind overrides over the embedded floor, both greens (admissibility +
//! conformance) on the *effective* contract, and the exit code. Each case sets the
//! process working directory to a project root that may or may not carry a
//! `temper.toml`, exactly as a real invocation would.
//!
//! The cases mirror the entry's acceptance:
//! - a severity flip (`required`→`advisory`) turns a violating skill from blocking
//!   to non-blocking;
//! - an override that *adds* a clause makes a previously-clean skill fire;
//! - a layered clause naming an unknown predicate is a load error;
//! - an inadmissible override (an empty `enum`) fails admissibility on the
//!   effective contract;
//! - an absent `temper.toml` leaves the floor outcome byte-for-byte unchanged
//!   (here: identical to a present-but-empty one, toggled over one workspace).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::compose::{
    AuthorLayer, ComposeError, DegreeBound, Edge, EdgeBound, Governs, Requirement,
};
use temper::contract::{Clause, Predicate, Severity};
use temper::kind::{KindError, Primitive};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-temper-toml-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A skill that trips no `required` clause: lowercase `name` matching its
/// directory, a present short description, no forbidden keys. Clean against the
/// floor.
const CLEAN_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill clean but for a Cursor `globs` key Claude Code ignores — it trips the
/// floor's `required` `forbidden_keys` clause and nothing else, so it is the
/// isolated subject for a severity flip.
const FORBIDDEN_GLOBS_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
globs: \"**/*.rs\"\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill that violates two `required` floor clauses (uppercase `name` is outside
/// `[a-z0-9-]` and no longer equals its directory) — a non-trivial diagnostic set
/// for the byte-stability case.
const ERROR_SKILL: &str = "---\n\
name: Coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// Project a one-skill harness into `<root>/.temper` via the real `import` verb, so
/// the workspace `check` reads is built exactly as a user's would be.
fn import_skill(root: &Path, name: &str, skill_md: &str) {
    let harness = tmpdir("harness");
    let dir = harness.join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();

    let status = Command::new(BIN)
        .arg("import")
        .arg(&harness)
        .arg("--into")
        .arg(root.join(".temper"))
        .status()
        .unwrap();
    assert!(status.success(), "import should succeed: {status}");
}

/// The outcome of a `check` run: whether it exited zero and its combined
/// stdout+stderr (diagnostics render to stdout, a load error to stderr).
struct CheckRun {
    ok: bool,
    output: String,
}

/// Run `temper check` from `root` (so a `temper.toml` there is discovered) against
/// the default `./.temper` workspace, capturing the result.
fn check_in(root: &Path) -> CheckRun {
    let out = Command::new(BIN)
        .current_dir(root)
        .arg("check")
        .output()
        .unwrap();
    let mut output = String::from_utf8_lossy(&out.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&out.stderr));
    CheckRun {
        ok: out.status.success(),
        output,
    }
}

/// Write `<root>/temper.toml`.
fn write_temper_toml(root: &Path, contents: &str) {
    fs::write(root.join("temper.toml"), contents).unwrap();
}

/// Write `<root>/temper-local.toml` — the gitignored personal override layer that
/// folds over the committed `temper.toml` (`specs/40-composition.md`).
fn write_temper_local(root: &Path, contents: &str) {
    fs::write(root.join("temper-local.toml"), contents).unwrap();
}

#[test]
fn a_severity_flip_turns_a_violating_skill_from_blocking_to_non_blocking() {
    let root = tmpdir("flip");
    import_skill(&root, "coordinate", FORBIDDEN_GLOBS_SKILL);

    // No `temper.toml`: the floor's `required` `forbidden_keys` blocks.
    assert!(
        !check_in(&root).ok,
        "the forbidden `globs` key trips the floor's required clause ⇒ non-zero"
    );

    // Flip that clause to `advisory` (same identity ⇒ override in place): the same
    // violation now only warns, so the run is non-blocking.
    write_temper_toml(
        &root,
        "[kind.skill]\n\
[[kind.skill.clause]]\n\
severity = \"advisory\"\n\
predicate = \"forbidden_keys\"\n\
keys = [\"globs\", \"alwaysApply\"]\n",
    );
    assert!(
        check_in(&root).ok,
        "flipping the clause to advisory must make the run non-blocking ⇒ zero"
    );
}

#[test]
fn an_override_that_adds_a_clause_makes_a_previously_clean_skill_fire() {
    let root = tmpdir("extend");
    import_skill(&root, "coordinate", CLEAN_SKILL);

    // No `temper.toml`: a clean skill passes the floor.
    assert!(
        check_in(&root).ok,
        "the clean skill passes the floor ⇒ zero"
    );

    // Extend the floor with a `required` section the skill's body lacks (it has a
    // `Coordinate` heading, no `Usage`): a new identity appends, and now it fires.
    write_temper_toml(
        &root,
        "[kind.skill]\n\
[[kind.skill.clause]]\n\
severity = \"required\"\n\
predicate = \"require_sections\"\n\
sections = [\"Usage\"]\n",
    );
    let run = check_in(&root);
    assert!(
        !run.ok,
        "the added required clause must make the previously-clean skill fire ⇒ non-zero"
    );
    assert!(
        run.output.contains("require_sections"),
        "the finding names the added clause, got:\n{}",
        run.output
    );
}

#[test]
fn a_layered_clause_naming_an_unknown_predicate_is_a_load_error() {
    let root = tmpdir("unknown-predicate");
    import_skill(&root, "coordinate", CLEAN_SKILL);

    write_temper_toml(
        &root,
        "[kind.skill]\n\
[[kind.skill.clause]]\n\
severity = \"required\"\n\
predicate = \"word_count\"\n\
field = \"description\"\n",
    );
    let run = check_in(&root);
    assert!(
        !run.ok,
        "a layered clause outside the closed vocabulary must fail the load ⇒ non-zero"
    );
    assert!(
        run.output.contains("unknown predicate"),
        "the load error names the unknown predicate, got:\n{}",
        run.output
    );
}

#[test]
fn an_inadmissible_override_fails_admissibility_on_the_effective_contract() {
    let root = tmpdir("inadmissible");
    import_skill(&root, "coordinate", CLEAN_SKILL);

    // An `enum` with no values is vacuous — it parses, but admissibility on the
    // *effective* contract (floor ⊕ layer) rejects it, even though the floor alone
    // is clean.
    write_temper_toml(
        &root,
        "[kind.skill]\n\
[[kind.skill.clause]]\n\
severity = \"required\"\n\
predicate = \"enum\"\n\
field = \"status\"\n\
values = []\n",
    );
    let run = check_in(&root);
    assert!(
        !run.ok,
        "an inadmissible layered clause must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("lists no values"),
        "admissibility names the vacuous clause, got:\n{}",
        run.output
    );
}

#[test]
fn an_absent_temper_toml_leaves_the_floor_outcome_byte_for_byte_unchanged() {
    let root = tmpdir("absent");
    import_skill(&root, "coordinate", ERROR_SKILL);

    // Same workspace, toggling only `temper.toml`. Absent ⇒ the floor runs.
    let absent = check_in(&root);
    assert!(
        !absent.ok,
        "the floor blocks the violating skill ⇒ non-zero"
    );

    // A present-but-empty `temper.toml` declares no kind, so every kind still falls
    // through to the floor — the result must be byte-for-byte identical.
    write_temper_toml(&root, "# this temper.toml declares nothing\n");
    let empty = check_in(&root);

    assert!(
        !empty.ok,
        "an empty temper.toml changes nothing ⇒ still non-zero"
    );
    assert_eq!(
        absent.output, empty.output,
        "an absent and a declares-nothing temper.toml must produce identical output"
    );
}

#[test]
fn a_temper_local_toml_overrides_and_extends_the_committed_layer() {
    let root = tmpdir("local-override");
    import_skill(&root, "coordinate", FORBIDDEN_GLOBS_SKILL);

    // The committed `temper.toml` relaxes the floor's `required` `forbidden_keys`
    // clause to `advisory`, so on its own the forbidden `globs` key only warns and
    // the run is non-blocking.
    write_temper_toml(
        &root,
        "[kind.skill]\n\
[[kind.skill.clause]]\n\
severity = \"advisory\"\n\
predicate = \"forbidden_keys\"\n\
keys = [\"globs\", \"alwaysApply\"]\n",
    );
    assert!(
        check_in(&root).ok,
        "the committed layer alone relaxes the clause to advisory ⇒ zero"
    );

    // A personal `temper-local.toml` folds over that committed layer: it flips the
    // *committed* `forbidden_keys` clause back to `required` (same identity ⇒
    // override) and extends the contract with a new `require_sections` clause the
    // skill's body lacks. Both take effect in the effective contract.
    write_temper_local(
        &root,
        "[kind.skill]\n\
[[kind.skill.clause]]\n\
severity = \"required\"\n\
predicate = \"forbidden_keys\"\n\
keys = [\"globs\", \"alwaysApply\"]\n\
[[kind.skill.clause]]\n\
severity = \"required\"\n\
predicate = \"require_sections\"\n\
sections = [\"Usage\"]\n",
    );
    let run = check_in(&root);
    assert!(
        !run.ok,
        "the local override flips the committed clause back to required ⇒ non-zero"
    );
    assert!(
        run.output.contains("require_sections"),
        "the local-added clause fires in the effective contract, got:\n{}",
        run.output
    );
}

#[test]
fn an_absent_temper_local_is_a_verbatim_pass_through_of_the_committed_layer() {
    let root = tmpdir("local-absent");
    import_skill(&root, "coordinate", CLEAN_SKILL);

    // A committed `temper.toml` that extends the floor with a `required` section the
    // clean skill's body lacks — a non-trivial effective contract that fires.
    write_temper_toml(
        &root,
        "[kind.skill]\n\
[[kind.skill.clause]]\n\
severity = \"required\"\n\
predicate = \"require_sections\"\n\
sections = [\"Usage\"]\n",
    );

    // Absent `temper-local.toml`: the committed layer runs unchanged.
    let absent = check_in(&root);
    assert!(
        !absent.ok,
        "the committed layer's added clause fires ⇒ non-zero"
    );

    // A present-but-empty `temper-local.toml` folds nothing over the committed
    // layer, so the effective contract — and thus the diagnostic output — must be
    // byte-for-byte identical to the absent-local run.
    write_temper_local(&root, "# this temper-local.toml declares nothing\n");
    let empty = check_in(&root);
    assert_eq!(
        absent.output, empty.output,
        "an absent and a declares-nothing temper-local.toml must produce identical output"
    );
}

// ---- custom-kind declaration (parse-only) -----------------------------------
//
// The `check` engine does not yet discover units at a custom kind's `governs`
// locus (a follow-on entry), so these cases drive the library parser directly —
// the seam that lands a `[kind.<name>]` declaration in `AuthorLayer`.

#[test]
fn a_custom_kind_declaration_parses_distinct_from_a_built_in_layer() {
    // `[kind.spec]` carries a `governs` locus, an `[[kind.spec.extraction]]` array,
    // and a `[[kind.spec.clause]]` contract — a *full* custom-kind declaration. It
    // parses into a typed `CustomKind` in the custom-kind map, while `[kind.skill]`
    // (adopt/clause-only, no `governs`/`extraction`) stays a built-in layer.
    let toml = r#"
[kind.skill]
adopt = "skill.anthropic"
[[kind.skill.clause]]
severity = "advisory"
predicate = "max_lines"
max = 300

[kind.spec]
governs = { root = "specs", glob = "*.md" }

[[kind.spec.extraction]]
primitive = "line_count"

[[kind.spec.extraction]]
primitive = "references"
feature = "references"

[[kind.spec.clause]]
severity = "advisory"
predicate = "max_lines"
max = 400
"#;
    let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();

    // The built-in layer is *not* a custom kind; the full declaration is.
    assert!(!layer.custom_kinds().contains_key("skill"));
    let spec = layer
        .custom_kinds()
        .get("spec")
        .expect("the custom kind parses into the roster");

    // The locus, the composed extractor, and the contract all parse.
    assert_eq!(
        spec.governs,
        Governs {
            root: "specs".to_string(),
            glob: "*.md".to_string(),
        }
    );
    assert_eq!(
        spec.extraction.primitives(),
        &[
            Primitive::LineCount,
            Primitive::References {
                feature: "references".to_string(),
            },
        ]
    );
    assert_eq!(
        spec.clauses,
        vec![Clause {
            severity: Severity::Advisory,
            guidance: None,
            predicate: Predicate::MaxLines { max: 400 },
        }]
    );
}

#[test]
fn relationships_parse_under_the_owning_kind_as_a_kind_capability() {
    // A reference is a kind capability, not a standalone construct: it is declared
    // under its owning kind's `[[kind.<name>.relationships]]` array (`specs/15-kinds.md`,
    // "The entity graph is a kind capability"). The `[kind.spec]` custom kind declares
    // one relationship, and `[kind.rule]` — a built-in layer, orthogonal to the split —
    // declares another; both parse into edges whose `from` is the owning kind.
    let toml = r#"
[kind.rule]
adopt = "rule"
[[kind.rule.relationships]]
field = "routes_to"
to = "skill"

[kind.spec]
governs = { root = "specs", glob = "*.md" }
[[kind.spec.extraction]]
primitive = "references"
feature = "references"
[[kind.spec.relationships]]
field = "references"
to = "spec"
"#;
    let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();

    // The custom kind still classifies as custom; the built-in layer does not —
    // relationships change neither.
    assert!(layer.custom_kinds().contains_key("spec"));
    assert!(!layer.custom_kinds().contains_key("rule"));

    // Both relationships are gathered as edges, each `from` its owning kind.
    let edges: Vec<&Edge> = layer.edges().iter().collect();
    assert!(edges.contains(&&Edge {
        field: "routes_to".to_string(),
        from: "rule".to_string(),
        to: "skill".to_string(),
    }));
    assert!(edges.contains(&&Edge {
        field: "references".to_string(),
        from: "spec".to_string(),
        to: "spec".to_string(),
    }));
}

#[test]
fn a_custom_kind_missing_its_governs_locus_is_a_load_error() {
    // An `[[kind.spec.extraction]]` array marks this custom, but it names no
    // `governs` locus — a custom kind that reads no files is malformed.
    let toml = r#"
[kind.spec]
[[kind.spec.extraction]]
primitive = "line_count"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(matches!(
        err,
        ComposeError::CustomKindMissingGoverns { ref kind, .. } if kind == "spec"
    ));
}

#[test]
fn a_custom_kind_with_a_malformed_governs_is_a_load_error() {
    // `governs` must be a table with `root` and `glob` strings; a bare string is
    // neither, so it folds into `BadGoverns`.
    let toml = r#"
[kind.spec]
governs = "specs"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(matches!(
        err,
        ComposeError::BadGoverns { ref kind, .. } if kind == "spec"
    ));
}

#[test]
fn an_unknown_extraction_primitive_in_a_custom_kind_is_a_load_error() {
    // The extraction array goes through the same closed-algebra parser a standalone
    // declaration does — an out-of-vocabulary primitive is rejected at load.
    let toml = r#"
[kind.spec]
governs = { root = "specs", glob = "*.md" }
[[kind.spec.extraction]]
primitive = "paragraph_meaning"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(matches!(
        err,
        ComposeError::Extraction(KindError::UnknownPrimitive { ref primitive, .. })
            if primitive == "paragraph_meaning"
    ));
}

#[test]
fn a_custom_kind_with_a_stray_key_is_a_load_error() {
    // A custom-kind declaration carries only `governs`, `extraction`, `clause`, and
    // `relationships`. A leftover `[kind.spec.entities]` subtable — there is no
    // separate entities table, a kind's nodes derive from its `features.id` — must
    // fail loudly, not be silently dropped, exactly as the built-in-layer path already
    // rejects a stray key (`specs/10-contracts.md`, "Decision: unknown keys are
    // rejected, not ignored").
    let toml = r#"
[kind.spec]
governs = { root = "specs", glob = "*.md" }
[kind.spec.entities]
id = "heading"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(
            err,
            ComposeError::CustomKindUnknownKey { ref key, ref kind, .. }
                if key == "entities" && kind == "spec"
        ),
        "a stray custom-kind key must be a load error, got {err:?}"
    );
}

#[test]
fn a_degree_bound_parses_into_a_typed_requirement() {
    // The graph-scope `degree` predicate: an inline `{ incoming, outgoing }` table
    // with per-direction `{ min?, max? }` bounds parses onto the requirement. The two
    // worked cases — "self-registering" `incoming = { max = 0 }` and a bounded outgoing
    // — land as `EdgeBound`s with their open endpoints left `None`.
    let toml = r#"
[requirement.self-registering]
kind = "skill"
contract = "contracts/skill.toml"
degree = { incoming = { max = 0 }, outgoing = { min = 1, max = 3 } }
"#;
    let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
    let requirement = layer
        .requirements()
        .get("self-registering")
        .expect("the requirement parses");
    assert_eq!(
        requirement.degree,
        Some(DegreeBound {
            incoming: Some(EdgeBound {
                min: None,
                max: Some(0),
            }),
            outgoing: Some(EdgeBound {
                min: Some(1),
                max: Some(3),
            }),
        })
    );
}

#[test]
fn a_routed_degree_bound_leaves_the_upper_endpoint_open() {
    // "Routed: at least one incoming" is `incoming = { min = 1 }` — an open-above
    // bound, its `max` left `None` so any positive in-degree satisfies it.
    let toml = r#"
[requirement.routed]
kind = "skill"
contract = "contracts/skill.toml"
degree = { incoming = { min = 1 } }
"#;
    let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
    let requirement = layer
        .requirements()
        .get("routed")
        .expect("the requirement parses");
    assert_eq!(
        requirement.degree,
        Some(DegreeBound {
            incoming: Some(EdgeBound {
                min: Some(1),
                max: None,
            }),
            outgoing: None,
        })
    );
}

#[test]
fn a_degree_naming_no_direction_is_a_load_error() {
    // A `degree` that names neither `incoming` nor `outgoing` constrains nothing —
    // a vacuous clause the author cannot have meant, rejected at load.
    let toml = r#"
[requirement.gate]
kind = "skill"
contract = "contracts/skill.toml"
degree = { }
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(matches!(
        err,
        ComposeError::RequirementBadDegree { ref name, .. } if name == "gate"
    ));
}

#[test]
fn an_endpoint_less_degree_direction_is_a_load_error() {
    // A direction bound with neither `min` nor `max` admits every degree — malformed,
    // the way a `degree` naming no direction is.
    let toml = r#"
[requirement.gate]
kind = "skill"
contract = "contracts/skill.toml"
degree = { incoming = { } }
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(matches!(err, ComposeError::RequirementBadDegree { .. }));
}

#[test]
fn an_inverted_degree_bound_is_a_load_error() {
    // `min > max` admits no degree at all — a vacuous bound, rejected at load the way
    // an inverted `count` bound is rejected as inadmissible.
    let toml = r#"
[requirement.gate]
kind = "skill"
contract = "contracts/skill.toml"
degree = { outgoing = { min = 3, max = 1 } }
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(matches!(err, ComposeError::RequirementBadDegree { .. }));
}

#[test]
fn a_negative_degree_endpoint_is_a_load_error() {
    // A negative endpoint cannot be a `usize` edge count — rejected, not floored.
    let toml = r#"
[requirement.gate]
kind = "skill"
contract = "contracts/skill.toml"
degree = { incoming = { min = -1 } }
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(matches!(err, ComposeError::RequirementBadDegree { .. }));
}

#[test]
fn a_requirement_parses_into_a_typed_value_with_means_verbatim() {
    // `[requirement.<name>]` is the harness's named-obligation namespace
    // (`specs/10-contracts.md`, "Requirements — the harness's named obligations"): an
    // optional `means` string stated in meaning, and an optional `required` coverage
    // flag. `means` is carried *verbatim* and never interpreted (law 3 — no proxy);
    // `required = true` parses through as declared.
    let means = "the harness has a skill that maintains development standards";
    let toml = format!(
        r#"
[requirement.dev-standards]
means = "{means}"
required = true
"#
    );
    let layer = AuthorLayer::parse(&toml, Path::new("temper.toml")).unwrap();

    let requirement = layer
        .requirements()
        .get("dev-standards")
        .expect("the requirement parses into the namespace");
    assert_eq!(
        requirement,
        &Requirement {
            name: "dev-standards".to_string(),
            means: Some(means.to_string()),
            kind: None,
            contract: None,
            required: true,
            count: None,
            unique: Vec::new(),
            membership: None,
            degree: None,
            verified_by: None,
        }
    );
    // The meaning is stored byte-for-byte — temper organizes it, never judges it.
    assert_eq!(requirement.means.as_deref(), Some(means));
}

#[test]
fn an_absent_required_defaults_to_false() {
    // `temper` never fabricates a gate the author did not declare (`00-intent.md` law
    // 4): an omitted `required` is `false`, not a coverage gate.
    let toml = r#"
[requirement.dev-standards]
means = "the harness maintains dev standards"
"#;
    let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
    let requirement = layer.requirements().get("dev-standards").unwrap();
    assert!(!requirement.required);
}

#[test]
fn a_requirement_with_no_means_parses() {
    // `means` is optional too — the unified requirement's *only* mandatory element is
    // its name (`specs/10-contracts.md`, "all facets optional except its name"), so a
    // requirement carrying only structural facets parses, `means` left `None`.
    let toml = r#"
[requirement.linter]
kind = "rule"
required = true
"#;
    let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
    let requirement = layer.requirements().get("linter").unwrap();
    assert_eq!(requirement.means, None);
    assert!(requirement.required);
}

#[test]
fn a_non_table_requirement_root_is_a_load_error() {
    // `requirement` is its own namespace — a table of named requirements. A scalar in
    // its place is malformed, rejected as a non-table root.
    let toml = r#"
requirement = "dev-standards"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(matches!(err, ComposeError::RequirementRootNotTable { .. }));
}

// ---- unknown keys are rejected, not ignored ---------------------------------
//
// `specs/10-contracts.md`, "Decision: unknown keys are rejected, not ignored": a
// misspelled key in any parsed contract-surface table must fail admissibility, not
// silently degrade the gate it was meant to arm. One case per parsed table, plus a
// clean-table control that must still parse untouched.

#[test]
fn a_stray_key_in_a_requirement_is_a_load_error() {
    // A misspelled `requird` on a requirement would quietly drop its coverage gate.
    let toml = r#"
[requirement.dev-standards]
means = "the harness maintains dev standards"
requird = true
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::RequirementUnknownKey { ref key, ref name, .. } if key == "requird" && name == "dev-standards"),
        "a stray requirement key names itself precisely, got: {err:?}"
    );
}

#[test]
fn a_stray_key_in_a_requirement_with_facets_is_a_load_error() {
    // The unified requirement's allowlist spans the folded facet set — a stray key
    // alongside `kind`/`contract` is still rejected, not silently dropped.
    let toml = r#"
[requirement.linter]
kind = "skill"
contract = "contracts/skill.anthropic.toml"
requird = true
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::RequirementUnknownKey { ref key, ref name, .. } if key == "requird" && name == "linter"),
        "a stray facet-set key names itself precisely, got: {err:?}"
    );
}

#[test]
fn a_match_key_in_a_requirement_is_rejected_as_an_unknown_key() {
    // The name-`match` selector is eradicated — fill is opt-in `satisfies` alone. A
    // leftover `match = {…}` is no longer a facet but an unknown key, rejected at parse
    // rather than silently dropped (`specs/10-contracts.md`, "Decision: unknown keys
    // are rejected, not ignored"; the MATCH-ERADICATE migration).
    let toml = r#"
[requirement.planner]
kind = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "plan*" }
required = true
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::RequirementUnknownKey { ref key, ref name, .. } if key == "match" && name == "planner"),
        "a `match` key is now an unknown-key reject, got: {err:?}"
    );
}

#[test]
fn a_stray_key_in_a_kind_layer_is_a_load_error() {
    // A misspelled `adpot` would silently take the floor instead of the named
    // template — a stray key on a built-in layer, rejected at parse.
    let toml = r#"
[kind.skill]
adpot = "skill.anthropic"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::KindUnknownKey { ref key, ref kind, .. } if key == "adpot" && kind == "skill"),
        "a stray kind-layer key names itself precisely, got: {err:?}"
    );
}

#[test]
fn a_stray_key_in_a_clause_fails_admissibility_end_to_end() {
    // A misspelled clause key must fail the whole `check` run, not degrade the
    // clause. Drive the binary so the parse → admissibility → gate path is pinned,
    // exactly as the unknown-predicate case is.
    let root = tmpdir("clause-stray-key");
    import_skill(&root, "coordinate", CLEAN_SKILL);

    write_temper_toml(
        &root,
        "[kind.skill]\n\
[[kind.skill.clause]]\n\
severity = \"required\"\n\
predicate = \"max_len\"\n\
field = \"name\"\n\
max = 64\n\
feild = \"nmae\"\n",
    );
    let run = check_in(&root);
    assert!(
        !run.ok,
        "a clause carrying a stray key must fail the load ⇒ non-zero"
    );
    assert!(
        run.output.contains("unknown key"),
        "the load error names the stray clause key, got:\n{}",
        run.output
    );
}

#[test]
fn clean_contract_surface_tables_still_parse_unchanged() {
    // The control: a requirement, a built-in kind layer, and a clause each carrying
    // only their admissible keys parse clean — the reject fires on strays only, never
    // on the closed vocabulary itself.
    let toml = r#"
[kind.skill]
adopt = "skill.anthropic"
[[kind.skill.clause]]
severity = "advisory"
predicate = "max_lines"
max = 300
guidance = "keep skills skimmable"

[requirement.linter]
kind = "skill"
contract = "contracts/skill.anthropic.toml"
required = true
verified_by = "tests/lint.rs"

[requirement.dev-standards]
means = "the harness maintains dev standards"
required = true
"#;
    let layer = AuthorLayer::parse(toml, Path::new("temper.toml"))
        .expect("clean contract-surface tables parse without a stray-key error");
    assert!(layer.requirements().contains_key("linter"));
    assert!(layer.requirements().contains_key("dev-standards"));
}
