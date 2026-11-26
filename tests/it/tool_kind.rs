//! The `tool` built-in kind: the one home of a command the harness runs, a root-hosted
//! embedded member that projects nothing and registers nothing. Driven through a real SDK
//! program's `emit` and the `check` that reads its lock.

use std::path::PathBuf;

use temper::drift::{self, EmitOptions};

use crate::common;

/// A program composing one tool of each variant and nothing that runs any of them.
const TOOLS_PROGRAM: &str = r#"
import { emit, harness, tool } from "@dtmd/temper";

const program = harness({
  members: [tool(), tool({ system: "jq" }), tool({ file: "scripts/check.sh" })],
});

process.stdout.write(emit(program).seam);
"#;

#[test]
fn a_tool_nothing_runs_draws_the_advisory_degree_finding() {
    let (harness, into) = common::wire_sdk_harness("tool-unreached", TOOLS_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let run = common::check_in(&harness, &["."], Some("github"));
    let findings = run.findings();
    let degree = common::findings_for(&findings, "tool.degree");
    for address in ["root/tool/temper", "root/tool/jq", "root/tool/check.sh"] {
        assert!(
            degree
                .iter()
                .any(|line| line.starts_with("::warning") && line.contains(address)),
            "an unreached tool draws the advisory degree finding at `{address}`: {}",
            run.output
        );
    }
    assert_eq!(degree.len(), 3, "{}", run.output);
    let checked = common::findings_for(&findings, "coverage.checked");
    assert!(
        checked
            .iter()
            .any(|line| line.contains("tool (3 embedded)")),
        "every tool is a checked member: {checked:#?}"
    );
}

#[test]
fn a_tool_member_projects_no_file() {
    let (harness, into) = common::wire_sdk_harness("tool-projects-nothing", TOOLS_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let written: Vec<PathBuf> = common::tree_bytes(&harness)
        .into_keys()
        .filter(|path| !path.starts_with(".temper"))
        .collect();
    assert_eq!(
        written,
        vec![PathBuf::from(".claude/rules/temper-sdk.md")],
        "only the counsel rule projects; a tool owns no file"
    );

    let declarations = drift::read_declarations(&into).unwrap();
    let tools: Vec<(&str, &str)> = declarations
        .nested_members
        .iter()
        .filter(|row| row.kind == "tool")
        .map(|row| (row.host.as_str(), row.key.as_str()))
        .collect();
    assert_eq!(
        tools,
        vec![("root", "check.sh"), ("root", "jq"), ("root", "temper")],
        "the lock carries each tool as a root-hosted member"
    );
    assert!(
        declarations.registrations.is_empty(),
        "a tool registers nothing"
    );
}
