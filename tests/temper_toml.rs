//! End-to-end acceptance over the author-declared `temper.toml` layer
//! (`specs/architecture/40-composition.md`, "Decision: the author-declared contract lives in
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

use temper::builtin_kind;
use temper::compose::{AuthorLayer, ComposeError, DegreeBound, Edge, EdgeBound, Requirement};
use temper::kind::{BUILTIN_KINDS, CustomKind, Governs, KindError, Primitive};

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
    let dir = harness.join(".claude").join("skills").join(name);
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
/// folds over the committed `temper.toml` (`specs/architecture/40-composition.md`).
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

// ---- custom-kind registration + authored KIND.md definition -----------------
//
// A custom kind is *registered* in the assembly (`[kind.<name>]` binds a package by
// name) and *defined* under `.temper/kinds/<name>/KIND.md` (`specs/architecture/40-composition.md`,
// "Decision: a custom kind is an authored `.temper/` artifact, registered in the
// assembly"). The fully-inline `[kind.<name>]` definition is retired: a `governs`/
// `extraction` key under a kind table is now a stray key, rejected at load. The
// registration cases drive the library parser directly; the definition cases drive the
// `KIND.md` loader over an authored fixture.

/// Write a `<root>/.temper/kinds/<name>/KIND.md` definition fixture and return the
/// `.temper/kinds` directory `CustomKind::load` reads from.
fn write_kind_definition(root: &Path, name: &str, kind_md: &str) -> PathBuf {
    let kinds_dir = root.join(".temper").join("kinds");
    let dir = kinds_dir.join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("KIND.md"), kind_md).unwrap();
    kinds_dir
}

#[test]
fn a_custom_kind_registration_binds_a_package_by_name() {
    // The registration is uniform with a built-in binding — `[kind.spec] package =
    // "spec"` wires the require-side, and the definition lives in KIND.md, not here. It
    // parses into the registered-kinds set with its bound package, and is separated from
    // a built-in by matching its name against `BUILTIN_KINDS`.
    let toml = r#"
[kind.skill]
package = "skill.anthropic"

[kind.spec]
package = "spec"
"#;
    let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
    let registered: Vec<&str> = layer.registered_kinds().collect();
    assert_eq!(registered, vec!["skill", "spec"]);
    assert_eq!(layer.kind_package("spec"), Some("spec"));
    // `spec` is a custom kind (not a built-in); `skill` is a built-in layer.
    assert!(!BUILTIN_KINDS.contains(&"spec"));
    assert!(BUILTIN_KINDS.contains(&"skill"));
}

#[test]
fn a_bare_kind_binding_resolves_to_the_unique_qualified_builtin() {
    // The qualified-binding wave: a bare `[kind.skill]` assembly binding (and a
    // requirement's `kind` typing) resolves through provider resolution to the unique
    // provider-qualified built-in (`specs/architecture/15-kinds.md`, "Decision: kind identity carries
    // a provider axis"). temper ships each built-in package bound to that qualified
    // identity — the published-binding form, since a consumer's assembly is unknowable.
    assert_eq!(
        builtin_kind::qualified("skill").unwrap().as_deref(),
        Some("claude-code.skill"),
        "the `skill.anthropic` floor binds the qualified kind `claude-code.skill`"
    );
    assert_eq!(
        builtin_kind::qualified("rule").unwrap().as_deref(),
        Some("claude-code.rule")
    );
    // A `[kind.skill]` binding is a built-in layer, not a custom registration — the split
    // the gate routes through recognizes the bare name as an embedded built-in kind.
    assert!(builtin_kind::definition("skill").unwrap().is_some());
    // A project's own kind mirrors nothing external: it declares no provider, stays bare,
    // and resolves to no built-in — it registers a custom kind instead.
    assert!(builtin_kind::qualified("spec").unwrap().is_none());
    assert!(builtin_kind::definition("spec").unwrap().is_none());
}

#[test]
fn a_bare_reference_with_two_providers_carrying_the_name_is_an_ambiguity_load_error() {
    // Two providers meeting under one bare name is a collision — a load error naming the
    // qualified candidates so the author qualifies the reference (`specs/architecture/15-kinds.md`).
    // The real embedded set carries only `claude-code`, so the collision is exercised over
    // a synthetic two-provider set through the very resolution helper the bindings route
    // through, never a silent wrong-kind pick.
    fn skill_of(provider: &str) -> CustomKind {
        let src = format!(
            "governs = {{ root = \".claude/skills\", glob = \"*/SKILL.md\" }}\nprovider = \"{provider}\"\n"
        );
        let doc = src.parse::<toml_edit::DocumentMut>().unwrap();
        CustomKind::from_header(doc.as_table(), "skill", Path::new("kinds/x/skill/KIND.md"))
            .unwrap()
    }

    // One provider: the bare name resolves to its unique qualified carrier.
    let one = vec![skill_of("claude-code")];
    assert_eq!(
        CustomKind::resolve_bare("skill", &one)
            .unwrap()
            .map(CustomKind::qualified_name)
            .as_deref(),
        Some("claude-code.skill")
    );

    // Two providers: an ambiguity load error, naming the bare name it collides on.
    let two = vec![skill_of("claude-code"), skill_of("agent-skills")];
    let err = CustomKind::resolve_bare("skill", &two).unwrap_err();
    assert!(
        matches!(err, KindError::AmbiguousKind { ref name, .. } if name == "skill"),
        "two providers under one bare name must be an ambiguity load error, got: {err:?}"
    );
}

#[test]
fn an_inline_governs_definition_is_a_retired_stray_key() {
    // The inline custom-kind definition is retired: a `governs` locus under a kind table
    // is no longer a declaration but a stray key, rejected at load. The definition
    // belongs in KIND.md.
    let toml = r#"
[kind.spec]
governs = { root = "specs", glob = "*.md" }
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::KindUnknownKey { ref key, ref kind, .. } if key == "governs" && kind == "spec"),
        "an inline `governs` is a retired stray key, got: {err:?}"
    );
}

#[test]
fn an_inline_extraction_definition_is_a_retired_stray_key() {
    // Likewise the inline `[[kind.spec.extraction]]` array — the composed extractor is
    // authored in KIND.md, never stuffed into the assembly.
    let toml = r#"
[kind.spec]
package = "spec"
[[kind.spec.extraction]]
primitive = "line_count"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::KindUnknownKey { ref key, ref kind, .. } if key == "extraction" && kind == "spec"),
        "an inline `extraction` is a retired stray key, got: {err:?}"
    );
}

#[test]
fn an_authored_kind_md_carries_the_whole_definition() {
    // The authored `KIND.md` header carries the composed definition — the `governs`
    // locus, the composed extraction, and the declared relationships — and the body is
    // the kind's prose (read by no check). `CustomKind::load` reads it back into a typed
    // definition.
    let root = tmpdir("kind-md");
    // Edges range over *declared structured fields*, never body-mined references
    // (law 8; `specs/architecture/15-kinds.md`): the `depends_on` relationship rides a `field`
    // primitive, not the retired `references` extractor.
    let kind_md = "+++\n\
governs = { root = \"specs\", glob = \"*.md\" }\n\
\n\
[[extraction]]\n\
primitive = \"line_count\"\n\
\n\
[[extraction]]\n\
primitive = \"field\"\n\
key = \"depends_on\"\n\
\n\
[[relationships]]\n\
field = \"depends_on\"\n\
to = \"spec\"\n\
+++\n\
\n\
# The spec kind\n\
\n\
A spec is temper's own governing document.\n";
    let kinds_dir = write_kind_definition(&root, "spec", kind_md);

    let spec = CustomKind::load(&kinds_dir, "spec").expect("the authored KIND.md loads");
    assert_eq!(spec.name, "spec");
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
            Primitive::Field {
                key: "depends_on".to_string(),
            },
        ]
    );
    assert_eq!(
        spec.relationships,
        vec![Edge {
            field: "depends_on".to_string(),
            from: "spec".to_string(),
            to: "spec".to_string(),
        }]
    );
}

#[test]
fn a_registered_kind_with_no_kind_md_is_a_load_error() {
    // A registration promises a definition — a missing `KIND.md` is a hard error, never
    // a silent skip (`specs/architecture/40-composition.md`, "Registering a custom kind").
    let root = tmpdir("kind-md-missing");
    let kinds_dir = root.join(".temper").join("kinds");
    let err = CustomKind::load(&kinds_dir, "spec").unwrap_err();
    assert!(matches!(
        err,
        KindError::MissingDefinition { ref kind, .. } if kind == "spec"
    ));
}

#[test]
fn a_kind_md_missing_its_governs_locus_is_a_load_error() {
    // A custom kind that reads no files is meaningless — the `governs` locus is
    // required in the definition.
    let root = tmpdir("kind-md-no-governs");
    let kind_md = "+++\n[[extraction]]\nprimitive = \"line_count\"\n+++\n# spec\n";
    let kinds_dir = write_kind_definition(&root, "spec", kind_md);
    let err = CustomKind::load(&kinds_dir, "spec").unwrap_err();
    assert!(matches!(
        err,
        KindError::MissingGoverns { ref kind, .. } if kind == "spec"
    ));
}

#[test]
fn a_kind_md_with_a_malformed_governs_is_a_load_error() {
    // `governs` must be a table with `root` and `glob` strings; a bare string folds
    // into `BadGoverns`.
    let root = tmpdir("kind-md-bad-governs");
    let kind_md = "+++\ngoverns = \"specs\"\n+++\n# spec\n";
    let kinds_dir = write_kind_definition(&root, "spec", kind_md);
    let err = CustomKind::load(&kinds_dir, "spec").unwrap_err();
    assert!(matches!(
        err,
        KindError::BadGoverns { ref kind, .. } if kind == "spec"
    ));
}

#[test]
fn a_kind_md_with_an_unknown_extraction_primitive_is_a_load_error() {
    // The extraction array goes through the same closed-algebra parser a standalone
    // declaration does — an out-of-vocabulary primitive is rejected at load.
    let root = tmpdir("kind-md-bad-prim");
    let kind_md = "+++\n\
governs = { root = \"specs\", glob = \"*.md\" }\n\
[[extraction]]\n\
primitive = \"paragraph_meaning\"\n\
+++\n# spec\n";
    let kinds_dir = write_kind_definition(&root, "spec", kind_md);
    let err = CustomKind::load(&kinds_dir, "spec").unwrap_err();
    assert!(matches!(
        err,
        KindError::UnknownPrimitive { ref primitive, .. } if primitive == "paragraph_meaning"
    ));
}

#[test]
fn a_kind_md_with_a_stray_header_key_is_a_load_error() {
    // A `KIND.md` header carries only `governs`, `extraction`, and `relationships`. A
    // leftover `clause` (a custom kind carries no clauses — its contract is the bound
    // package) or an `entities` table (nodes derive from `features.id`) must fail
    // loudly, not be silently dropped (`specs/architecture/10-contracts.md`, "Decision: unknown keys
    // are rejected, not ignored").
    let root = tmpdir("kind-md-stray");
    let kind_md = "+++\n\
governs = { root = \"specs\", glob = \"*.md\" }\n\
[entities]\n\
id = \"heading\"\n\
+++\n# spec\n";
    let kinds_dir = write_kind_definition(&root, "spec", kind_md);
    let err = CustomKind::load(&kinds_dir, "spec").unwrap_err();
    assert!(
        matches!(err, KindError::UnknownKey { ref key, ref kind, .. } if key == "entities" && kind == "spec"),
        "a stray KIND.md header key must be a load error, got {err:?}"
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
package = "skill.anthropic"
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
package = "skill.anthropic"
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
package = "skill.anthropic"
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
package = "skill.anthropic"
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
package = "skill.anthropic"
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
package = "skill.anthropic"
degree = { incoming = { min = -1 } }
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(matches!(err, ComposeError::RequirementBadDegree { .. }));
}

#[test]
fn a_requirement_parses_into_a_typed_value_with_means_verbatim() {
    // `[requirement.<name>]` is the harness's named-obligation namespace
    // (`specs/architecture/10-contracts.md`, "Requirements — the harness's named obligations"): an
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
            package: None,
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
    // its name (`specs/architecture/10-contracts.md`, "all facets optional except its name"), so a
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
// `specs/architecture/10-contracts.md`, "Decision: unknown keys are rejected, not ignored": a
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
    // alongside `kind`/`package` is still rejected, not silently dropped.
    let toml = r#"
[requirement.linter]
kind = "skill"
package = "skill.anthropic"
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
    // rather than silently dropped (`specs/architecture/10-contracts.md`, "Decision: unknown keys
    // are rejected, not ignored"; the MATCH-ERADICATE migration).
    let toml = r#"
[requirement.planner]
kind = "skill"
package = "skill.anthropic"
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
fn a_package_binding_parses_onto_a_requirement() {
    // Requirement typing is `package = "<name>"` — a package named *by name*, resolved
    // against the built-in packages ∪ `.temper/packages/` (PACKAGE-BINDING's order). It
    // parses onto the requirement's `package` facet, stored verbatim.
    let toml = r#"
[requirement.linter]
kind = "rule"
package = "lint-standards"
required = true
"#;
    let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
    let requirement = layer.requirements().get("linter").unwrap();
    assert_eq!(requirement.package, Some("lint-standards".to_string()));
}

#[test]
fn the_retired_contract_bundle_key_on_a_requirement_is_an_unknown_key() {
    // `contract = "<path>"` — a requirement adopting a contract bundle by path — retired
    // into the by-name `package` facet (`specs/architecture/10-contracts.md`, the typing facet). A
    // leftover `contract` key is rejected at parse, not silently dropped.
    let toml = r#"
[requirement.linter]
kind = "skill"
contract = "contracts/skill.anthropic.toml"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::RequirementUnknownKey { ref key, ref name, .. } if key == "contract" && name == "linter"),
        "the retired `contract` bundle key is rejected as unknown, got: {err:?}"
    );
}

#[test]
fn inline_clauses_on_a_requirement_are_an_unknown_key() {
    // Inline clauses under a requirement retired — clauses live only in packages
    // (`specs/architecture/10-contracts.md`, the typing facet). A leftover `[[requirement.*.clause]]`
    // array is an unknown `clause` key, rejected at parse.
    let toml = r#"
[requirement.linter]
kind = "skill"
[[requirement.linter.clause]]
severity = "required"
predicate = "required"
field = "name"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::RequirementUnknownKey { ref key, ref name, .. } if key == "clause" && name == "linter"),
        "inline `clause` on a requirement is rejected as unknown, got: {err:?}"
    );
}

#[test]
fn a_stray_key_in_a_kind_layer_is_a_load_error() {
    // A misspelled `pacakge` would silently take the floor instead of the named
    // package — a stray key on a built-in layer, rejected at parse.
    let toml = r#"
[kind.skill]
pacakge = "skill.anthropic"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::KindUnknownKey { ref key, ref kind, .. } if key == "pacakge" && kind == "skill"),
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
package = "skill.anthropic"
[[kind.skill.clause]]
severity = "advisory"
predicate = "max_lines"
max = 300
guidance = "keep skills skimmable"

[requirement.linter]
kind = "skill"
package = "skill.anthropic"
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

// ---- package binding by name ------------------------------------------------
//
// `[kind.<k>] package = "<name>"` binds a kind to a package by *name*, resolved
// against the built-in floor ∪ `.temper/packages/` (`specs/architecture/20-surface.md`,
// "Decision: package binding is by artifact kind"). The retired `adopt = "<path>"`
// key is now a stray key, and an unresolvable name is a precise load error.

/// Write a project package at `<root>/.temper/packages/<name>/PACKAGE.md` — the
/// resolution home a non-built-in bound name loads from (PACKAGE-DOCUMENT's loader).
fn write_package(root: &Path, name: &str, package_md: &str) {
    let dir = root.join(".temper").join("packages").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("PACKAGE.md"), package_md).unwrap();
}

#[test]
fn the_retired_adopt_key_is_now_an_unknown_key() {
    // `adopt = "<path>"` is gone: binding is `package = "<name>"`. A leftover `adopt`
    // is a stray key on a built-in layer, rejected at parse rather than silently
    // taking the floor — the very silent gap temper exists to catch.
    let toml = r#"
[kind.skill]
adopt = "skill.anthropic"
"#;
    let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
    assert!(
        matches!(err, ComposeError::KindUnknownKey { ref key, ref kind, .. } if key == "adopt" && kind == "skill"),
        "the retired `adopt` key is rejected as unknown, got: {err:?}"
    );
}

#[test]
fn binding_a_builtin_package_by_name_is_the_default_made_explicit() {
    // `package = "skill.anthropic"` names the kind's built-in package, resolving to
    // the embedded floor — byte-for-byte the floor-only outcome.
    let root = tmpdir("bind-builtin");
    import_skill(&root, "coordinate", FORBIDDEN_GLOBS_SKILL);

    let absent = check_in(&root);
    assert!(
        !absent.ok,
        "the forbidden `globs` key trips the floor ⇒ non-zero"
    );

    write_temper_toml(&root, "[kind.skill]\npackage = \"skill.anthropic\"\n");
    let bound = check_in(&root);
    assert_eq!(
        absent.output, bound.output,
        "binding the built-in package by name must match the implicit floor exactly"
    );
}

#[test]
fn binding_an_unresolvable_package_name_is_a_precise_load_error() {
    // A name that is neither the built-in nor a `.temper/packages/` project package
    // fails the load precisely, naming the offending package — never a silent
    // fall-through to the floor.
    let root = tmpdir("bind-unknown");
    import_skill(&root, "coordinate", CLEAN_SKILL);

    write_temper_toml(&root, "[kind.skill]\npackage = \"skill.cursor\"\n");
    let run = check_in(&root);
    assert!(
        !run.ok,
        "an unresolvable bound package must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("unknown package") && run.output.contains("skill.cursor"),
        "the load error names the unresolved package precisely, got:\n{}",
        run.output
    );
}

#[test]
fn binding_a_project_package_replaces_the_floor_wholesale() {
    // A non-built-in name resolves from `.temper/packages/<name>/PACKAGE.md` and
    // *replaces* the floor as the kind's package: a skill that trips the floor's
    // `forbidden_keys` passes once bound to a project package that carries no such
    // clause — proof the bound package, not the floor, governs.
    let root = tmpdir("bind-project");
    import_skill(&root, "coordinate", FORBIDDEN_GLOBS_SKILL);

    // Under the floor, the forbidden `globs` key blocks.
    assert!(
        !check_in(&root).ok,
        "the floor's forbidden_keys blocks ⇒ non-zero"
    );

    // A project package with a single benign required clause the skill satisfies and
    // no `forbidden_keys` clause at all.
    write_package(
        &root,
        "lax",
        "+++\n\
[[clause]]\n\
severity = \"required\"\n\
predicate = \"min_len\"\n\
field = \"name\"\n\
min = 1\n\
+++\n\
\n\
# Lax skill package\n\
\n\
The project's own permissive skill package.\n",
    );
    write_temper_toml(&root, "[kind.skill]\npackage = \"lax\"\n");

    let bound = check_in(&root);
    assert!(
        bound.ok,
        "binding the lax project package drops the floor's forbidden_keys ⇒ zero, got:\n{}",
        bound.output
    );
}
