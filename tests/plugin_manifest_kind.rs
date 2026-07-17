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
//! The `--strict` bar the kind's contract mirrors lands whole here, in its two halves: an
//! *undocumented* top-level key, which `closed-keys` decides against the key set the
//! contract's own rows declare, and a *documented* key at the wrong level — the top-level
//! `themes`/`monitors` spelling the experimental migration retires, which stays the
//! `forbidden_keys` clause's, since the key is recognized and only its placement is not.

use std::fs;

mod common;

use common::{check_harness, write_plugin_json};

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

    let run = common::check_harness_in(&harness, None);

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
        !common::findings_for(&findings, "plugin-manifest.allowed_chars.name").is_empty(),
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
    let forbidden = common::findings_for(&findings, "plugin-manifest.forbidden_keys");
    assert!(!forbidden.is_empty(), "{findings:?}");
    assert!(
        forbidden.iter().all(|f| f.starts_with("::error")),
        "the clause holds the --strict world, where the warning is an error: {findings:?}"
    );

    // The guidance rides the finding's help line and is where the divergence is stated:
    // the clause decides the key's presence, never which world the reader is in, so
    // "loads today, `--strict` fails it, a future release requires `experimental.*`" can
    // only be teaching prose carried with the clause.
    let run = common::check_harness_in(&harness, None);
    assert!(
        run.output.contains("still loads today") && run.output.contains("`--strict` fails it"),
        "{}",
        run.output
    );

    // The key is documented — the migration is about *where* it is declared — so the
    // closed key set holds over it. The two clauses split the `--strict` bar rather than
    // both indicting one mistake.
    assert!(
        common::findings_for(&findings, "plugin-manifest.closed-keys").is_empty(),
        "a recognized key is not an unrecognized one: {findings:?}"
    );
}

#[test]
fn an_undocumented_top_level_key_fails_the_gate_as_the_strict_bar() {
    let harness = common::tmpdir("plugin-manifest-unrecognized");
    // The substance of `--strict`. Claude Code ignores this key and the plugin loads —
    // deliberately, so one `plugin.json` can double as another ecosystem's manifest — and
    // `claude plugin validate` calls it a warning. `--strict` is the CI bar that fails it,
    // and the strictest documented profile is the one this kind's contract mirrors.
    write_plugin_json(
        &harness,
        "{\n  \"name\": \"deployment-tools\",\n  \"contributes\": {}\n}\n",
    );

    let (findings, ok) = check_harness(&harness);
    assert!(
        !ok,
        "an undocumented top-level key fails the gate: {findings:?}"
    );
    let undeclared = common::findings_for(&findings, "plugin-manifest.closed-keys");
    assert_eq!(undeclared.len(), 1, "{findings:?}");
    assert!(
        undeclared[0].starts_with("::error") && undeclared[0].contains("contributes"),
        "the finding names the offending key, at the --strict severity: {findings:?}"
    );

    // The guidance is where the forgiving runtime is stated — the clause decides the key,
    // never which world the reader is validating in.
    let run = common::check_harness_in(&harness, None);
    assert!(
        run.output
            .contains("ignores an unrecognized top-level field"),
        "{}",
        run.output
    );
}

#[test]
fn every_documented_key_of_a_full_manifest_passes_the_closed_key_set() {
    let harness = common::tmpdir("plugin-manifest-full");
    // The failure this widening can produce is a documented key left out of the contract's
    // own rows, which would indict every manifest carrying it. So the fixture is the
    // reference's complete-schema example — every documented key at once — and it gates
    // clean (code.claude.com/docs/en/plugins-reference, "Complete schema", retrieved
    // 2026-07-17).
    write_plugin_json(
        &harness,
        r#"{
  "$schema": "https://json.schemastore.org/claude-code-plugin-manifest.json",
  "name": "plugin-name",
  "displayName": "Plugin Name",
  "version": "1.2.0",
  "description": "Brief plugin description",
  "author": { "name": "Author Name", "email": "author@example.com" },
  "homepage": "https://docs.example.com/plugin",
  "repository": "https://github.com/author/plugin",
  "license": "MIT",
  "keywords": ["keyword1", "keyword2"],
  "defaultEnabled": false,
  "skills": "./custom/skills/",
  "commands": ["./custom/commands/special.md"],
  "agents": ["./custom/agents/reviewer.md"],
  "hooks": "./config/hooks.json",
  "mcpServers": "./mcp-config.json",
  "outputStyles": "./styles/",
  "lspServers": "./.lsp.json",
  "settings": {},
  "userConfig": {},
  "channels": [],
  "experimental": { "themes": "./themes/", "monitors": "./monitors.json" },
  "dependencies": ["helper-lib"]
}
"#,
    );

    let (findings, ok) = check_harness(&harness);
    assert!(ok, "every documented key is a declared key: {findings:?}");
}

#[test]
fn both_documented_forms_of_a_union_typed_component_path_gate_clean() {
    // `skills` is documented `string|array`, and the clause declares the set — so each
    // documented form passes through the real gate. A single-kind clause could only have
    // admitted one of the two, which is why the field was held rather than gated.
    for (label, value) in [
        ("skills-string", "\"./custom/skills/\""),
        ("skills-array", "[\"./custom/skills/\", \"./more/skills/\"]"),
    ] {
        let harness = common::tmpdir(label);
        write_plugin_json(
            &harness,
            &format!("{{\n  \"name\": \"deployment-tools\",\n  \"skills\": {value}\n}}\n"),
        );

        let (findings, ok) = check_harness(&harness);
        assert!(ok, "a documented `skills` form gates clean: {findings:?}");
        assert!(
            findings.iter().all(|f| !f.starts_with("::error")),
            "{findings:?}"
        );
    }
}

#[test]
fn a_component_path_outside_its_documented_union_fails_the_gate() {
    let harness = common::tmpdir("plugin-manifest-skills-number");
    write_plugin_json(
        &harness,
        "{\n  \"name\": \"deployment-tools\",\n  \"skills\": 7\n}\n",
    );

    let (findings, ok) = check_harness(&harness);
    assert!(!ok, "a number-valued skills fails the gate");
    let typed = common::findings_for(&findings, "plugin-manifest.type.skills");
    assert!(!typed.is_empty(), "{findings:?}");
    assert!(
        typed.iter().all(|f| f.starts_with("::error")),
        "{findings:?}"
    );
    // The finding names the whole declared set, not one arbitrary member of it: an
    // author told only "not a string" cannot see the list form is open to them.
    assert!(
        typed.iter().any(|f| f.contains("string|list")),
        "{findings:?}"
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
    let typed = common::findings_for(&findings, "plugin-manifest.type.keywords");
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
        common::findings_for(&findings, "plugin-manifest.type.keywords").is_empty(),
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
        common::findings_for(&findings, "plugin-manifest.required.name").is_empty(),
        "{findings:?}"
    );
}
