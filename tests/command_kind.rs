//! The `command` built-in kind: the skill surface's legacy file placement
//! (`specs/builtins.md`, "The shipped kinds").
//!
//! Discovery folds a lone `.claude/commands/*.md` file into a `command` member with
//! file-stem identity (like `rule` — no `name` field required for identity) and the
//! skill's field schema (imported, not re-derived): both invocation channels, the
//! same declared fields `skill` extracts. Driven at the crate-public API a real
//! `import`/`check` read takes — `import::discover_kind_files`, `Member::from_source`,
//! and the generic surface loader `builtin_kind::features` — over a fixture
//! mirroring the real Claude Code layout (`.claude/rules/rust.md`, "Harness-input
//! fixtures mirror the real Claude Code layout").

use std::fs;
use std::path::PathBuf;

mod common;

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

/// Write a command member at `<root>/.claude/commands/<stem>.md` — the real Claude
/// Code locus (`.claude/commands/*.md`), never a layout invented for the test.
fn write_command(root: &std::path::Path, stem: &str, body: &str) -> PathBuf {
    let dir = root.join(".claude").join("commands");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{stem}.md"));
    fs::write(&path, body).unwrap();
    path
}

#[test]
fn discovery_over_the_embedded_governs_finds_the_command_file() {
    let harness = common::tmpdir("discover");
    write_command(&harness, "coordinate", DEPLOY_COMMAND);
    write_command(&harness, "deploy", DEPLOY_COMMAND);

    let command_kind = builtin_kind::definition("command")
        .unwrap()
        .expect("command is embedded");
    let found = import::discover_kind_files(
        &import::Discovery::new(&harness),
        &command_kind,
        command_kind.governs.as_ref().unwrap(),
        import::LocalOverride::Honored,
    )
    .unwrap();

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

    let command_kind = builtin_kind::definition("command")
        .unwrap()
        .expect("command is embedded");
    let member = Member::from_source(&command_kind, &source).unwrap();

    // File-stem identity — like `rule`, not the `name`-field identity a directory-
    // shaped `skill` carries.
    assert_eq!(member.id, "deploy");
}

#[test]
fn a_command_member_registers_on_both_documented_invocation_channels() {
    let command_kind = builtin_kind::definition("command")
        .unwrap()
        .expect("command is embedded");

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
fn a_command_member_extracts_the_skills_declared_field_schema() {
    let harness = common::tmpdir("deploy-schema");
    let source = write_command(&harness, "deploy", DEPLOY_COMMAND);

    let command_kind = builtin_kind::definition("command")
        .unwrap()
        .expect("command is embedded");
    let member = Member::from_source(&command_kind, &source).unwrap();
    let unit = common::surface_unit(&member);
    let features = builtin_kind::features(&command_kind, &unit, &[]);

    // The skill's field schema by import: `description` extracts exactly as it does
    // off a `SKILL.md`.
    assert_eq!(
        features.field("description"),
        Some(FeatureValue::scalar(
            ValueType::String,
            "Deploy the application to production."
        ))
    );
}
