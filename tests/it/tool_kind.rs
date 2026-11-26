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
    common::write_sibling(&harness, "scripts/check.sh", "#!/bin/sh\n");
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
    common::write_sibling(&harness, "scripts/check.sh", "#!/bin/sh\n");
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let written: Vec<PathBuf> = common::tree_bytes(&harness)
        .into_keys()
        .filter(|path| !path.starts_with(".temper") && !path.starts_with("scripts"))
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

/// A program declaring a committed hook script as a `{ file }` tool, and the
/// `PostToolUse` handler whose command runs it naming that tool by `runs`.
const HOOK_SCRIPT_PROGRAM: &str = r#"
import { emit, harness, tool } from "@dtmd/temper";
import { hook } from "@dtmd/temper/claude-code";

const check = tool({ file: ".claude/hooks/check.sh" });
const after = hook({
  name: "PostToolUse",
  matcher: "Edit",
  hooks: [{ type: "command", command: ".claude/hooks/check.sh", runs: check }],
});

process.stdout.write(emit(harness({ members: [check, after] })).seam);
"#;

#[test]
fn a_hook_handler_reaches_the_file_tool_it_runs() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("tool-hook-script", HOOK_SCRIPT_PROGRAM);
    common::write_sibling(&harness, ".claude/hooks/check.sh", "#!/bin/sh\nexit 0\n");
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    // The edge is temper's: the projected handler carries the documented fields alone.
    let settings = std::fs::read_to_string(harness.join(".claude/settings.json")).unwrap();
    assert!(
        !settings.contains("\"runs\""),
        "the `runs` edge never reaches settings.json: {settings}"
    );
    let lock = std::fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        lock.contains("[[declaration.runs]]")
            && lock.contains("member = \"hook:PostToolUse:Edit/handler/0\"")
            && lock.contains("tool = \"root/tool/check.sh\""),
        "the lock carries the handler's edge to its tool: {lock}"
    );
    assert!(
        lock.contains("source_path = \".claude/hooks/check.sh\""),
        "the tool's file is a fingerprinted input: {lock}"
    );

    let run = common::check_in(&harness, &["."], Some("github"));
    let findings = run.findings();
    assert!(
        common::findings_for(&findings, "tool.degree").is_empty(),
        "the handler's edge reaches the tool, so the tool is run: {}",
        run.output
    );
    assert!(
        common::findings_for(&findings, "graph.route").is_empty(),
        "the edge resolves: {}",
        run.output
    );
}

#[test]
fn a_handler_naming_no_tool_member_refuses_at_emit() {
    common::ensure_sdk_built();
    let program = HOOK_SCRIPT_PROGRAM.replace("members: [check, after]", "members: [after]");
    let (harness, into) = common::wire_sdk_harness("tool-hook-uncomposed", &program);
    common::write_sibling(&harness, ".claude/hooks/check.sh", "#!/bin/sh\n");
    let err = drift::emit_program(&into, EmitOptions::default()).unwrap_err();
    assert!(
        format!("{err:?}").contains("does not compose"),
        "a `runs` edge to a tool the harness does not compose refuses: {err:?}"
    );
}

#[test]
fn a_file_tool_naming_a_missing_script_refuses_at_emit() {
    common::ensure_sdk_built();
    let (_harness, into) = common::wire_sdk_harness("tool-hook-missing", HOOK_SCRIPT_PROGRAM);
    assert!(
        drift::emit_program(&into, EmitOptions::default()).is_err(),
        "a `{{ file }}` tool's script is a declared input, so a missing one refuses"
    );
}

/// A program whose committed `settings` status line runs a script declared as a
/// `{ file }` tool, naming it by `runs`.
const STATUS_LINE_PROGRAM: &str = r#"
import { emit, harness, tool } from "@dtmd/temper";
import { settings } from "@dtmd/temper/claude-code";

const line = tool({ file: ".claude/statusline.sh" });
const project = settings({
  name: "settings",
  statusLine: { type: "command", command: ".claude/statusline.sh", padding: 0, runs: line },
});

process.stdout.write(emit(harness({ members: [line, project] })).seam);
"#;

#[test]
fn the_settings_status_line_reaches_the_file_tool_it_runs() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("tool-status-line", STATUS_LINE_PROGRAM);
    common::write_sibling(&harness, ".claude/statusline.sh", "#!/bin/sh\necho ok\n");
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let settings = std::fs::read_to_string(harness.join(".claude/settings.json")).unwrap();
    assert!(
        settings.contains("\"statusLine\"") && !settings.contains("\"runs\""),
        "the status line projects its documented fields alone: {settings}"
    );
    let lock = std::fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        lock.contains("member = \"settings:settings\"")
            && lock.contains("tool = \"root/tool/statusline.sh\""),
        "the lock carries the status line's edge to its tool: {lock}"
    );

    let run = common::check_in(&harness, &["."], Some("github"));
    let findings = run.findings();
    assert!(
        common::findings_for(&findings, "tool.degree").is_empty(),
        "the status line's edge reaches the tool: {}",
        run.output
    );
    assert!(
        common::findings_for(&findings, "coverage.unclaimed-entry")
            .iter()
            .all(|line| !line.contains("statusline.sh")),
        "the script is the tool's input, so it is claimed: {}",
        run.output
    );
}
