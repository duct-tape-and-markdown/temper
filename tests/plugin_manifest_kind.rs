//! The `plugin-manifest` built-in kind: `.claude-plugin/plugin.json`, a plugin pack's
//! identity (`specs/builtins.md`, "The shipped kinds"; 0031).
//!
//! The first built-in at the `json-document` format — the whole artifact is one JSON
//! object rather than frontmatter over a body — and the first file kind whose identity is
//! read from a document key rather than a path. Driven over fixtures at the real
//! `.claude-plugin/plugin.json` locus (`.claude/rules/rust.md`): the read that turns a
//! manifest into one member named by its `name`, and the default contract's clauses
//! firing through the real gate.
//!
//! The `--strict` bar the kind's contract mirrors is only partly expressible today: the
//! unrecognized-top-level-field check needs an allow-list predicate the algebra does not
//! carry, so the clause proven here is the decidable slice of it — the top-level
//! `themes`/`monitors` spelling the experimental migration retires
//! (`sdk/src/builtins.ts`, `pluginManifestDefaultContract`, names the whole hold).

use std::fs;
use std::path::Path;

mod common;

use common::check_harness;

use temper::builtin_kind;
use temper::json_manifest::DocumentMember;
use temper::kind::{Content, Format, Governs, Registration, UnitShape};

/// A plugin manifest in the real Claude Code shape: `name` alone is required, the rest is
/// optional metadata and component paths (code.claude.com/docs/en/plugins-reference,
/// retrieved 2026-07-16).
const PLUGIN_JSON: &str = r#"{
  "name": "deployment-tools",
  "displayName": "Deployment Tools",
  "version": "2.1.0",
  "description": "Deployment automation tools",
  "keywords": ["deployment", "ci-cd"],
  "agents": ["./custom/agents/reviewer.md"]
}
"#;

/// Write a manifest at the real `.claude-plugin/plugin.json` locus — never a layout
/// invented for the test's convenience (`.claude/rules/rust.md`).
fn write_plugin_json(root: &Path, body: &str) {
    let dir = root.join(".claude-plugin");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("plugin.json"), body).unwrap();
}

fn plugin_manifest_kind() -> temper::kind::CustomKind {
    builtin_kind::definition("plugin-manifest")
        .unwrap()
        .expect("plugin-manifest is embedded")
}

#[test]
fn the_plugin_manifest_kind_owns_its_file_as_a_json_document_named_by_its_name_key() {
    let manifest = plugin_manifest_kind();

    assert_eq!(
        manifest.governs,
        Some(Governs {
            root: ".claude-plugin".to_string(),
            glob: "plugin.json".to_string(),
        })
    );
    // The format label is what routes the artifact to the document reader; a `.json` file
    // read through the frontmatter adapter would carry no fields at all.
    assert_eq!(manifest.format, Some(Format::JsonDocument));
    // Identity from the document's own key: the stem is `plugin` for every manifest ever
    // written, so the named-field mode is the only one that tells two apart.
    assert_eq!(
        manifest.unit_shape,
        Some(UnitShape::NamedField {
            field: "name".to_string(),
        })
    );
    // It *is* the manifest rather than surfacing inside one — so it owns its file, and
    // carries no collection address and no fields-only shape.
    assert_eq!(manifest.collection_address, None);
    assert_eq!(manifest.content, Content::File);
    // Channel-less: distribution metadata reaches the installer, never the model.
    assert_eq!(manifest.registration, Vec::<Registration>::new());
}

#[test]
fn a_plugin_json_surfaces_one_member_whose_identity_is_its_name_field() {
    let harness = common::tmpdir("plugin-manifest-read");
    write_plugin_json(&harness, PLUGIN_JSON);

    let member = DocumentMember::read(
        &plugin_manifest_kind(),
        &harness.join(".claude-plugin/plugin.json"),
    )
    .unwrap();

    // Identity is the `name` value, never the `plugin` stem.
    assert_eq!(member.id, "deployment-tools");
    // The document's top-level keys are the member's fields, so a clause ranges over a
    // manifest exactly as it ranges over a frontmatter member.
    let fields: Vec<&str> = member.fields.keys().map(String::as_str).collect();
    assert_eq!(
        fields,
        vec![
            "agents",
            "description",
            "displayName",
            "keywords",
            "name",
            "version"
        ]
    );
}

#[test]
fn a_well_formed_manifest_passes_the_gate_clean() {
    let harness = common::tmpdir("plugin-manifest-clean");
    write_plugin_json(&harness, PLUGIN_JSON);

    let (findings, ok) = check_harness(&harness);
    assert!(ok, "a documented manifest gates clean: {findings:?}");
    // Every documented optional field above — metadata and a component path alike — draws
    // no error: the contract gates what the format decides, never what it merely allows.
    assert!(
        findings.iter().all(|f| !f.starts_with("::error")),
        "{findings:?}"
    );
    // And the member is really being checked, not silently skipped past.
    assert!(
        findings.iter().any(|f| f.contains("plugin-manifest (1)")),
        "{findings:?}"
    );
}

#[test]
fn a_manifest_with_no_name_refuses_loud_rather_than_degrading_to_a_nameless_member() {
    let harness = common::tmpdir("plugin-manifest-no-name");
    // Omitting the manifest entirely is supported — Claude Code derives the name from the
    // directory — so an absent file is no finding (below). A manifest that exists and
    // declares no `name` is the case the loader rejects outright.
    write_plugin_json(&harness, "{\n  \"description\": \"Nameless\"\n}\n");

    let run = common::check_in(&harness, &["--harness", harness.to_str().unwrap()], None);

    // `name` is this kind's identity, not merely a required field, so its absence is a
    // *read* refusal that never reaches the clause: with no identity there is no member to
    // range over, and naming one after the `plugin` stem — identical for every manifest
    // ever written — is the degradation the format's read refuses. The contract's own
    // `required("name")` clause states the rule portably regardless, exactly as `agent`,
    // the other named-field kind, carries it.
    assert!(!run.ok, "a nameless manifest fails the gate");
    // The diagnostic code, not the prose: miette wraps the message to the terminal width.
    assert!(
        run.output
            .contains("temper::json_manifest::no_identity_value"),
        "{}",
        run.output
    );
}

#[test]
fn a_name_outside_the_kebab_case_charset_is_a_finding() {
    let harness = common::tmpdir("plugin-manifest-bad-name");
    // The documented rule is kebab-case, no spaces — `displayName` is where a
    // human-readable label with spaces and casing belongs.
    write_plugin_json(&harness, "{\n  \"name\": \"Deployment Tools\"\n}\n");

    let (findings, ok) = check_harness(&harness);
    assert!(!ok, "a non-kebab-case name fails the gate");
    assert!(
        !common::findings_for(&findings, "allowed_chars").is_empty(),
        "{findings:?}"
    );
}

#[test]
fn a_top_level_experimental_component_key_is_the_strict_bar_the_algebra_can_decide() {
    let harness = common::tmpdir("plugin-manifest-experimental");
    // The runtime loads this and `claude plugin validate` only warns; `--strict` fails it,
    // and a future release will require `experimental.*`. The clause holds the `--strict`
    // world, and its guidance is where that divergence is stated.
    write_plugin_json(
        &harness,
        "{\n  \"name\": \"deployment-tools\",\n  \"themes\": \"./themes/\"\n}\n",
    );

    let (findings, ok) = check_harness(&harness);
    assert!(!ok, "a top-level experimental key fails the gate");
    let forbidden = common::findings_for(&findings, "forbidden_keys");
    assert!(!forbidden.is_empty(), "{findings:?}");
    assert!(
        forbidden.iter().all(|f| f.starts_with("::error")),
        "the clause holds the --strict world, where the warning is an error: {findings:?}"
    );

    // The guidance rides the finding's help line and is where the divergence is stated:
    // the clause decides the key's presence, never which world the reader is in, so
    // "loads today, `--strict` fails it, a future release requires `experimental.*`" can
    // only be teaching prose carried with the clause.
    let run = common::check_in(&harness, &["--harness", harness.to_str().unwrap()], None);
    assert!(
        run.output.contains("still loads today") && run.output.contains("`--strict` fails it"),
        "{}",
        run.output
    );
}

#[test]
fn a_string_keywords_is_the_wrong_typed_field_the_declared_kind_decides() {
    let harness = common::tmpdir("plugin-manifest-keywords-string");
    // Unlike the experimental-key slice above, this one is not a `--strict` warning at all:
    // a wrong-typed field is a load error on every machine, so the finding is an error in
    // the forgiving runtime's world too.
    write_plugin_json(
        &harness,
        "{\n  \"name\": \"deployment-tools\",\n  \"keywords\": \"deployment\"\n}\n",
    );

    let (findings, ok) = check_harness(&harness);
    assert!(!ok, "a string-valued keywords fails the gate");
    let typed = common::findings_for(&findings, "type");
    assert!(!typed.is_empty(), "{findings:?}");
    assert!(
        typed.iter().all(|f| f.starts_with("::error")),
        "{findings:?}"
    );
}

#[test]
fn an_absent_keywords_is_silent_because_the_field_is_optional() {
    let harness = common::tmpdir("plugin-manifest-keywords-absent");
    // Presence is `required`'s concern and no clause requires this field, so `type` ranges
    // over a value that isn't there and says nothing — the arity that keeps one optional
    // field's omission from drawing a finding the docs never state.
    write_plugin_json(&harness, "{\n  \"name\": \"deployment-tools\"\n}\n");

    let (findings, ok) = check_harness(&harness);
    assert!(ok, "an absent optional field is no finding: {findings:?}");
    assert!(
        common::findings_for(&findings, "type").is_empty(),
        "{findings:?}"
    );
}

#[test]
fn this_repo_authors_no_plugin_manifest_so_the_kind_counts_zero_members() {
    // Honest, not a gap: temper's own harness is not a plugin pack, so the shipped kind
    // has nothing to govern here — the `supporting-doc (0)` precedent.
    let harness = common::tmpdir("plugin-manifest-absent");
    fs::create_dir_all(harness.join(".claude")).unwrap();

    let (findings, ok) = check_harness(&harness);
    assert!(ok, "no manifest is no finding: {findings:?}");
    assert!(
        common::findings_for(&findings, "required").is_empty(),
        "{findings:?}"
    );
}
