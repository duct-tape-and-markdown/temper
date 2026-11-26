//! The `command` built-in kind: the skill surface's legacy file placement
//! (`specs/builtins.md`, "The shipped kinds").
//!
//! Discovery folds a lone `.claude/commands/**/*.md` file into a `command` member named
//! by its path under `commands/`, each `/` a `:` (no `name` field required for identity),
//! and no
//! required frontmatter fields — all command frontmatter is optional: both
//! invocation channels, and any fields the frontmatter carries are extracted as
//! context. Driven at the crate-public API a real `import`/`check` read takes —
//! `import::discover_kind_files`, `Member::from_source`, and the generic surface
//! loader `builtin_kind::features` — over a fixture mirroring the real Claude Code
//! layout (`.claude/rules/rust.md`, "Harness-input fixtures mirror the real Claude
//! Code layout").

use std::fs;
use std::path::PathBuf;

use crate::common;

use temper::builtin_kind;
use temper::extract::{FeatureValue, ValueType};
use temper::frontmatter::Member;
use temper::import;
use temper::kind::Registration;

/// A command file in the real Claude Code shape: YAML frontmatter over a markdown
/// body, the same schema a skill's `SKILL.md` carries.
const DEPLOY_COMMAND: &str = "---\n\
description: Deploy the application to production.\n\
---\n\
# Deploy\n\
\n\
Run the release pipeline.\n";

/// Write a command member at `<root>/.claude/commands/<relative>.md` — the real Claude
/// Code locus (`.claude/commands/**/*.md`), never a layout invented for the test.
/// `relative` may carry subdirectories (`frontend/component`).
fn write_command(root: &std::path::Path, relative: &str, body: &str) -> PathBuf {
    let path = root
        .join(".claude")
        .join("commands")
        .join(format!("{relative}.md"));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, body).unwrap();
    path
}

/// Discover the command files under `harness` and name each one as `check` reads it:
/// rooted at the locus the command glob folds against.
fn command_members(harness: &std::path::Path) -> Vec<(PathBuf, String)> {
    let command_kind = builtin_kind::definition("command").expect("command is embedded");
    let governs = command_kind.governs.clone().unwrap();
    let disc = import::Discovery::new(harness);
    let base = temper::drift::fold_base(&harness.join(&governs.root), &governs.glob);
    import::discover_kind_files(
        &disc,
        &command_kind,
        &governs,
        import::LocalOverride::Honored,
    )
    .into_iter()
    .map(|file| {
        let id = Member::from_source_rooted(&command_kind, &file, &base)
            .unwrap()
            .id;
        (file, id)
    })
    .collect()
}

#[test]
fn a_command_in_a_subdirectory_of_the_commands_folder_is_a_member() {
    let harness = common::tmpdir("nested-member");
    write_command(&harness, "deploy", DEPLOY_COMMAND);
    write_command(&harness, "frontend/component", DEPLOY_COMMAND);

    let files: Vec<PathBuf> = command_members(&harness)
        .into_iter()
        .map(|(file, _)| file)
        .collect();
    assert_eq!(
        files,
        vec![
            harness.join(".claude/commands/deploy.md"),
            harness.join(".claude/commands/frontend/component.md"),
        ]
    );
}

#[test]
fn a_nested_commands_identity_is_its_path_under_commands_joined_with_colons() {
    let harness = common::tmpdir("nested-identity");
    write_command(&harness, "frontend/component", DEPLOY_COMMAND);
    write_command(&harness, "a/b/c", DEPLOY_COMMAND);

    let ids: Vec<String> = command_members(&harness)
        .into_iter()
        .map(|(_, id)| id)
        .collect();
    assert_eq!(
        ids,
        vec!["a:b:c".to_string(), "frontend:component".to_string()]
    );
}

#[test]
fn two_same_named_commands_in_different_subdirectories_are_two_members() {
    let harness = common::tmpdir("nested-same-name");
    write_command(&harness, "frontend/x", DEPLOY_COMMAND);
    write_command(&harness, "backend/x", DEPLOY_COMMAND);

    let ids: Vec<String> = command_members(&harness)
        .into_iter()
        .map(|(_, id)| id)
        .collect();
    assert_eq!(ids, vec!["backend:x".to_string(), "frontend:x".to_string()]);
}

#[test]
fn discovery_over_the_embedded_governs_finds_the_command_file() {
    let harness = common::tmpdir("discover");
    write_command(&harness, "coordinate", DEPLOY_COMMAND);
    write_command(&harness, "deploy", DEPLOY_COMMAND);

    let command_kind = builtin_kind::definition("command").expect("command is embedded");
    let found = import::discover_kind_files(
        &import::Discovery::new(&harness),
        &command_kind,
        command_kind.governs.as_ref().unwrap(),
        import::LocalOverride::Honored,
    );

    assert_eq!(
        found,
        vec![
            harness.join(".claude/commands/coordinate.md"),
            harness.join(".claude/commands/deploy.md"),
        ]
    );
}

#[test]
fn a_command_file_folds_into_a_member_with_file_stem_identity() {
    let harness = common::tmpdir("deploy");
    let source = write_command(&harness, "deploy", DEPLOY_COMMAND);

    let command_kind = builtin_kind::definition("command").expect("command is embedded");
    let member = Member::from_source(&command_kind, &source).unwrap();

    // File-stem identity — like `rule`, not the `name`-field identity a directory-
    // shaped `skill` carries.
    assert_eq!(member.id, "deploy");
}

#[test]
fn a_command_member_registers_on_both_documented_invocation_channels() {
    let command_kind = builtin_kind::definition("command").expect("command is embedded");

    assert_eq!(
        command_kind.registration,
        vec![
            Registration::UserInvoked,
            Registration::DescriptionTrigger {
                field: "description".to_string()
            },
        ]
    );
}

#[test]
fn a_command_member_with_no_frontmatter_fields_is_valid() {
    let harness = common::tmpdir("no-fields");
    let body = "# Deploy\n\nRun the release pipeline.\n";
    let source = write_command(&harness, "deploy", body);

    let command_kind = builtin_kind::definition("command").expect("command is embedded");
    let member = Member::from_source(&command_kind, &source).unwrap();

    // A command file with no frontmatter at all is valid — the contract is empty.
    assert_eq!(member.id, "deploy");
}

#[test]
fn a_command_member_can_optionally_carry_frontmatter_fields() {
    let harness = common::tmpdir("deploy-fields");
    let source = write_command(&harness, "deploy", DEPLOY_COMMAND);

    let command_kind = builtin_kind::definition("command").expect("command is embedded");
    let member = Member::from_source(&command_kind, &source).unwrap();
    let unit = common::surface_unit(&member);
    let features = builtin_kind::features(&command_kind, &unit, &[]);

    // When frontmatter fields are present, they extract as context: `description`
    // is carried, but it is not required.
    assert_eq!(
        features.field("description"),
        Some(FeatureValue::scalar(
            ValueType::String,
            "Deploy the application to production."
        ))
    );
}

#[test]
fn a_command_whose_frontmatter_yaml_does_not_parse_loads_with_no_fields_set() {
    let harness = common::tmpdir("non-portable-command");
    let path = write_command(
        &harness,
        "deploy",
        "---\ndescription: Deploy: ship the release\n---\n# Deploy\n\nRun the release pipeline.\n",
    );
    let command_kind = builtin_kind::definition("command").expect("command is embedded");

    let member = Member::from_source(&command_kind, &path)
        .expect("the command loads as the runtime loads it");

    assert_eq!(member.id, "deploy");
    assert!(
        member.fields.is_empty(),
        "no field is set: {:?}",
        member.fields
    );
    assert!(member.body.contains("# Deploy"));
    assert!(member.forgiven.is_some());

    let run = common::check_in(&harness, &["."], Some("github"));
    let findings = run.findings();
    assert!(
        common::findings_for(&findings, "member.load-fault").is_empty(),
        "a command the runtime loads is never a load fault: {findings:#?}"
    );
    assert_eq!(
        common::findings_for(&findings, "member.non-portable-yaml").len(),
        1,
        "{findings:#?}"
    );
}
