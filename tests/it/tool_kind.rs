//! The `tool` built-in kind: the one home of a command the harness runs, a root-hosted
//! embedded member that projects nothing and registers nothing. Driven through a real SDK
//! program's `emit` and the `check` that reads its lock.

use std::path::PathBuf;

use temper::drift::{self, EmitOptions};

use crate::common;

/// A program composing one tool of each variant and nothing that runs any of them.
const TOOLS_PROGRAM: &str = r#"
import { harness, tool } from "@dtmd/temper";

const program = harness({
  members: [tool(), tool({ system: "jq" }), tool({ file: "scripts/check.sh" })],
});

export default program;
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
            .any(|line| line.contains("tool (4 embedded)")),
        "every tool is a checked member, the fixture typecheck's included: {checked:#?}"
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
        vec![
            ("root", "check.sh"),
            ("root", "fixture-tsc"),
            ("root", "jq"),
            ("root", "temper")
        ],
        "the lock carries each tool as a root-hosted member, the fixture typecheck's included"
    );
    assert!(
        declarations.registrations.is_empty(),
        "a tool registers nothing"
    );
}

/// A program declaring a committed hook script as a `{ file }` tool, and the
/// `PostToolUse` handler whose command runs it naming that tool by `runs`.
const HOOK_SCRIPT_PROGRAM: &str = r#"
import { harness, tool } from "@dtmd/temper";
import { hook } from "@dtmd/temper/claude-code";

const check = tool({ file: ".claude/hooks/check.sh" });
const after = hook({
  event: "PostToolUse",
  matcher: "Edit",
  hooks: [{ type: "command", command: ".claude/hooks/check.sh", runs: check }],
});

export default harness({ members: [check, after] });
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
        common::findings_for(&findings, "root.resolves").is_empty(),
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
import { harness, tool } from "@dtmd/temper";
import { settings } from "@dtmd/temper/claude-code";

const line = tool({ file: ".claude/statusline.sh" });
const project = settings({
  name: "settings",
  statusLine: { type: "command", command: ".claude/statusline.sh", padding: 0, runs: line },
});

export default harness({ members: [line, project] });
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

/// A program composing the reference and one `{ package, bin }` tool that the reference
/// renders and nothing runs.
const PACKAGE_TOOL_PROGRAM: &str = r#"
import { harness, reference, tool } from "@dtmd/temper";

const program = harness({
  members: [reference(), tool({ package: "typescript", bin: "tsc" })],
});

export default program;
"#;

/// The tool's topic beside the reference's `SKILL.md`.
fn tool_topic(harness: &std::path::Path) -> String {
    std::fs::read_to_string(harness.join(".claude/skills/temper/kind-tool.md")).unwrap()
}

#[test]
fn a_package_bin_tool_reads_its_version_from_package_lock_json() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("tool-npm-lock", PACKAGE_TOOL_PROGRAM);
    common::write_sibling(
        &into,
        "package-lock.json",
        r#"{ "lockfileVersion": 3, "packages": {
  "": { "devDependencies": { "typescript": "5.9.3" } },
  "node_modules/typescript": { "version": "5.9.3" } } }
"#,
    );
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let topic = tool_topic(&harness);
    assert!(
        topic.contains("`root/tool/tsc`")
            && topic.contains("version 5.9.3, locked by `package-lock.json`"),
        "the reference shows the locked version: {topic}"
    );
    let narration = common::explain_in(&harness, "member:root/tool/tsc");
    assert!(
        narration.contains("`typescript` 5.9.3, locked by the program's `package-lock.json`"),
        "explain shows the locked version: {narration}"
    );
    let run = common::check_in(&harness, &["."], Some("github"));
    assert!(
        common::findings_for(&run.findings(), "tool.unlocked").is_empty(),
        "a locked package draws no finding: {}",
        run.output
    );
}

#[test]
fn explain_resolves_a_root_hosted_tool_by_its_address() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("tool-explain-address", PACKAGE_TOOL_PROGRAM);
    common::write_sibling(
        &into,
        "package-lock.json",
        r#"{ "lockfileVersion": 3, "packages": {
  "": { "devDependencies": { "typescript": "5.9.3" } },
  "node_modules/typescript": { "version": "5.9.3" } } }
"#,
    );
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let qualified = common::explain_in(&harness, "member:root/tool/tsc");
    for target in ["root/tool/tsc", "tsc"] {
        let narration = common::explain_in(&harness, target);
        assert!(
            narration.contains("`typescript` 5.9.3, locked by the program's `package-lock.json`"),
            "`explain {target}` narrates the tool: {narration}"
        );
        assert_eq!(
            narration, qualified,
            "`explain {target}` reads as the qualified spelling does"
        );
    }
}

#[test]
fn a_package_bin_tool_reads_its_version_from_pnpm_lock_yaml() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("tool-pnpm-lock", PACKAGE_TOOL_PROGRAM);
    common::declare_package_manager(&into, "pnpm@9.15.0");
    common::write_sibling(
        &into,
        "pnpm-lock.yaml",
        "lockfileVersion: '9.0'\n\nimporters:\n\n  .:\n    devDependencies:\n      typescript:\n        specifier: ^5.6.0\n        version: 5.9.3\n\npackages:\n\n  typescript@5.9.3:\n    resolution: {integrity: sha512-x}\n",
    );
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let topic = tool_topic(&harness);
    assert!(
        topic.contains("version 5.9.3, locked by `pnpm-lock.yaml`"),
        "the reference shows the locked version: {topic}"
    );
    let narration = common::explain_in(&harness, "member:root/tool/tsc");
    assert!(
        narration.contains("`typescript` 5.9.3, locked by the program's `pnpm-lock.yaml`"),
        "explain shows the locked version: {narration}"
    );
}

/// Assert the reference, `explain` and `check` all read `typescript` 5.9.3 from `lock`.
fn assert_locked_by(harness: &std::path::Path, lock: &str) {
    let topic = tool_topic(harness);
    assert!(
        topic.contains(&format!("version 5.9.3, locked by `{lock}`")),
        "the reference shows the locked version: {topic}"
    );
    let narration = common::explain_in(harness, "member:root/tool/tsc");
    assert!(
        narration.contains(&format!(
            "`typescript` 5.9.3, locked by the program's `{lock}`"
        )),
        "explain shows the locked version: {narration}"
    );
    let run = common::check_in(harness, &["."], Some("github"));
    assert!(
        common::findings_for(&run.findings(), "tool.unlocked").is_empty(),
        "a locked package draws no finding: {}",
        run.output
    );
}

#[test]
fn a_workspace_root_lockfile_above_the_program_locks_a_package_bin_tool() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("tool-root-lock", PACKAGE_TOOL_PROGRAM);
    common::declare_package_manager(&into, "pnpm@9.15.0");
    common::write_sibling(&harness, "pnpm-workspace.yaml", "packages:\n  - .temper\n");
    common::write_sibling(
        &harness,
        "pnpm-lock.yaml",
        "lockfileVersion: '9.0'\n\nimporters:\n\n  .:\n    devDependencies:\n      typescript:\n        specifier: ^4.0.0\n        version: 4.9.5\n\n  .temper:\n    devDependencies:\n      typescript:\n        specifier: ^5.6.0\n        version: 5.9.3\n\npackages:\n\n  typescript@5.9.3:\n    resolution: {integrity: sha512-x}\n",
    );
    drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert_locked_by(&harness, "pnpm-lock.yaml");

    // npm's workspace lock: the project's own `node_modules` before the hoisted root.
    let (harness, into) = common::wire_sdk_harness("tool-root-npm-lock", PACKAGE_TOOL_PROGRAM);
    common::write_sibling(&harness, "package.json", r#"{ "workspaces": [".temper"] }"#);
    common::write_sibling(
        &harness,
        "package-lock.json",
        r#"{ "lockfileVersion": 3, "packages": {
  "": { "workspaces": [".temper"] },
  "node_modules/typescript": { "version": "4.9.5" },
  ".temper/node_modules/typescript": { "version": "5.9.3" } } }
"#,
    );
    drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert_locked_by(&harness, "package-lock.json");
}

#[test]
fn a_lockfile_in_an_ancestor_outside_the_workspace_is_not_read() {
    common::ensure_sdk_built();
    let stray_npm = r#"{ "lockfileVersion": 3, "packages": {
  "node_modules/typescript": { "version": "5.9.3" } } }
"#;
    let stray_pnpm = "lockfileVersion: '9.0'\n\nimporters:\n\n  .temper:\n    devDependencies:\n      typescript:\n        specifier: ^5.6.0\n        version: 5.9.3\n";
    // No workspace above the program: a stray lock, a lock under a `package.json` with no
    // `workspaces`, and another manager's lone lock are each some other project's.
    let cases: [(&str, &[(&str, &str)]); 4] = [
        ("tool-stray-npm-lock", &[("package-lock.json", stray_npm)]),
        (
            "tool-stray-manifest-lock",
            &[
                ("package.json", r#"{ "name": "home" }"#),
                ("package-lock.json", stray_npm),
            ],
        ),
        (
            "tool-stray-yarn-lock",
            &[("yarn.lock", "# yarn lockfile v1\n")],
        ),
        // A workspace that does not include the program bounds the walk too.
        (
            "tool-excluding-workspace-lock",
            &[
                (
                    "pnpm-workspace.yaml",
                    "packages:\n  - 'packages/*'\n  - '!.temper'\n",
                ),
                ("pnpm-lock.yaml", stray_pnpm),
            ],
        ),
    ];
    for (label, files) in cases {
        let (harness, into) = common::wire_sdk_harness(label, PACKAGE_TOOL_PROGRAM);
        for (name, body) in files {
            common::write_sibling(&harness, name, body);
        }
        drift::emit_program(&into, EmitOptions::default())
            .unwrap_or_else(|err| panic!("{label}: an unrelated lock refuses nothing: {err:?}"));
        assert!(
            tool_topic(&harness).contains("no lockfile records it"),
            "{label}: the reference reads no lockfile"
        );
        let run = common::check_in(&harness, &["."], Some("github"));
        let findings = run.findings();
        let unlocked = common::findings_for(&findings, "tool.unlocked");
        assert!(
            unlocked.len() == 1 && unlocked[0].contains("carries no lockfile"),
            "{label}: the ancestor's lock is never read: {}",
            run.output
        );
    }
}

#[test]
fn a_pnpm_lockfile_with_a_leading_env_document_locks_a_package_bin_tool() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("tool-pnpm-env-lock", PACKAGE_TOOL_PROGRAM);
    common::declare_package_manager(&into, "pnpm@10.0.0");
    common::write_sibling(
        &into,
        "pnpm-lock.yaml",
        "---\nlockfileVersion: '9.0'\n\nimporters:\n\n  .:\n    configDependencies: {}\n    packageManagerDependencies:\n      pnpm:\n        specifier: 10.0.0\n        version: 10.0.0\n\npackages:\n\n  pnpm@10.0.0:\n    resolution: {integrity: sha512-y}\n\nsnapshots:\n\n  pnpm@10.0.0: {}\n\n---\nlockfileVersion: '9.0'\n\nsettings:\n  autoInstallPeers: true\n\nimporters:\n\n  .:\n    devDependencies:\n      typescript:\n        specifier: ^5.6.0\n        version: 5.9.3\n\npackages:\n\n  typescript@5.9.3:\n    resolution: {integrity: sha512-x}\n",
    );
    drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert_locked_by(&harness, "pnpm-lock.yaml");
}

#[test]
fn a_package_bin_tool_missing_from_the_lockfile_is_a_finding() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("tool-unlocked", PACKAGE_TOOL_PROGRAM);
    common::write_sibling(
        &into,
        "package-lock.json",
        r#"{ "lockfileVersion": 3, "packages": { "": {} } }
"#,
    );
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    assert!(
        tool_topic(&harness).contains("not in `package-lock.json`"),
        "the reference names the missing version"
    );
    let run = common::check_in(&harness, &["."], Some("github"));
    let findings = run.findings();
    let unlocked = common::findings_for(&findings, "tool.unlocked");
    assert!(
        unlocked.len() == 1
            && unlocked[0].starts_with("::warning")
            && unlocked[0].contains("root/tool/tsc")
            && unlocked[0].contains("package-lock.json"),
        "a package the lockfile does not record is an advisory finding: {}",
        run.output
    );
}

#[test]
fn a_package_manager_field_naming_pnpm_makes_the_tool_use_pnpm() {
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("tool-declared-pnpm", PACKAGE_TOOL_PROGRAM);
    common::declare_package_manager(&into, "pnpm@9.15.0");
    // npm's lockfile beside it records another version: the declared manager's is read.
    common::write_sibling(
        &into,
        "package-lock.json",
        r#"{ "lockfileVersion": 3, "packages": {
  "node_modules/typescript": { "version": "4.9.5" } } }
"#,
    );
    common::write_sibling(
        &into,
        "pnpm-lock.yaml",
        "lockfileVersion: '9.0'\n\nimporters:\n\n  .:\n    devDependencies:\n      typescript:\n        specifier: ^5.6.0\n        version: 5.9.3\n",
    );
    drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert_locked_by(&harness, "pnpm-lock.yaml");
}

#[test]
fn a_program_package_json_with_no_package_manager_field_refuses_the_tool_read() {
    common::ensure_sdk_built();
    let fieldless = "{ \"type\": \"module\" }\n";
    let (_harness, into) = common::wire_sdk_harness("tool-no-manager", PACKAGE_TOOL_PROGRAM);
    common::write_sibling(&into, "package.json", fieldless);
    let err = drift::emit_program(&into, EmitOptions::default()).unwrap_err();
    assert!(
        format!("{err:?}").contains("declares no `packageManager`"),
        "emit refuses naming the field: {err:?}"
    );

    let (harness_b, into_b) = common::wire_sdk_harness(
        "tool-no-manager-check",
        &PACKAGE_TOOL_PROGRAM.replace("reference(), ", ""),
    );
    drift::emit_program(&into_b, EmitOptions::default()).unwrap();
    common::write_sibling(&into_b, "package.json", fieldless);
    let run = common::check_in(&harness_b, &["."], Some("github"));
    assert!(
        !run.ok && run.output.contains("packageManager"),
        "check refuses naming the field: {}",
        run.output
    );
}

#[test]
fn an_unsupported_package_manager_refuses_naming_it() {
    common::ensure_sdk_built();
    let (_harness, into) = common::wire_sdk_harness("tool-yarn-manager", PACKAGE_TOOL_PROGRAM);
    common::declare_package_manager(&into, "yarn@4.5.0");
    common::write_sibling(&into, "yarn.lock", "# yarn lockfile v1\n");

    let err = drift::emit_program(&into, EmitOptions::default()).unwrap_err();
    assert!(
        format!("{err:?}").contains("yarn@4.5.0"),
        "emit refuses naming the manager: {err:?}"
    );

    // A program composing no reference still meets the refusal at `check`.
    let (harness_b, into_b) = common::wire_sdk_harness(
        "tool-yarn-manager-check",
        &PACKAGE_TOOL_PROGRAM.replace("reference(), ", ""),
    );
    drift::emit_program(&into_b, EmitOptions::default()).unwrap();
    common::declare_package_manager(&into_b, "yarn@4.5.0");
    let run = common::check_in(&harness_b, &["."], Some("github"));
    assert!(
        !run.ok && run.output.contains("yarn@4.5.0"),
        "check refuses naming the manager: {}",
        run.output
    );
    let narration = common::explain_in(&harness_b, "member:root/tool/tsc");
    assert!(
        narration.contains("yarn@4.5.0"),
        "explain refuses naming the manager: {narration}"
    );
}

/// A program whose only runner of its `{ package, bin }` compiler tool is its typecheck.
const TYPECHECK_ONLY_PROGRAM: &str = r#"
import { harness, input, tool } from "@dtmd/temper";


const tsc = tool({ package: "typescript", bin: "tsc" });

const program = harness({
  members: [tsc],
  typecheck: { tool: tsc, project: input(import.meta.url, "./tsconfig.json") },
});

export default program;
"#;

#[cfg(unix)]
#[test]
fn a_tool_reached_only_by_the_typecheck_draws_no_degree_finding() {
    let (harness, into) = common::wire_sdk_harness("tool-typecheck-only", TYPECHECK_ONLY_PROGRAM);
    common::wire_typescript_compiler(&into);
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let declarations = drift::read_declarations(&into).unwrap();
    assert!(
        declarations
            .runs
            .iter()
            .any(|row| row.member == "root" && row.tool == "root/tool/tsc"),
        "the lock carries the typecheck's edge as a runs row whose runner is the root: {:?}",
        declarations.runs
    );

    let run = common::check_in(&harness, &["."], Some("github"));
    let findings = run.findings();
    assert!(
        common::findings_for(&findings, "tool.degree").is_empty(),
        "the typecheck runs the tool, so the tool is run: {}",
        run.output
    );
    assert!(
        common::findings_for(&findings, "root.resolves").is_empty(),
        "the root's edge resolves: {}",
        run.output
    );
}

/// A program whose only runner of its tool is a telemetry requirement's tap, and whose
/// only hook content is that tap.
const TAP_ONLY_PROGRAM: &str = r#"
import { harness, telemetry, tool } from "@dtmd/temper";


const temper = tool();
const program = harness({
  members: [temper],
  require: {
    "skill-fires": { prose: "the skill fires", verifier: telemetry(["SkillInvoked"], { tool: temper }) },
  },
});

export default program;
"#;

#[test]
fn a_tool_reached_only_by_a_tap_draws_no_degree_finding() {
    let (harness, into) = common::wire_sdk_harness("tool-tap-only", TAP_ONLY_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let declarations = drift::read_declarations(&into).unwrap();
    assert!(
        declarations
            .runs
            .iter()
            .any(|row| row.member == "root/requirement/skill-fires"
                && row.tool == "root/tool/temper"),
        "the lock carries the tap's edge as a runs row whose runner is its requirement: {:?}",
        declarations.runs
    );

    let run = common::check_in(&harness, &["."], Some("github"));
    let findings = run.findings();
    assert!(
        common::findings_for(&findings, "tool.degree").is_empty(),
        "the tap runs the tool, so the tool is run: {}",
        run.output
    );
    assert!(
        common::findings_for(&findings, "root.resolves").is_empty(),
        "the requirement's edge resolves: {}",
        run.output
    );
}

/// A program whose only runner of its `{ file }` tool is a script verifier, beside a
/// script verifier that names no tool.
const SCRIPT_VERIFIER_PROGRAM: &str = r#"
import { harness, script, tool } from "@dtmd/temper";


const verify = tool({ file: "scripts/verify.sh" });

const program = harness({
  members: [verify],
  require: {
    "verified": { prose: "the script passes", verifier: script("scripts/verify.sh", { tool: verify }) },
    "tested": { prose: "the test passes", verifier: script("tests/held.rs") },
  },
});

export default program;
"#;

/// Wire and emit [`SCRIPT_VERIFIER_PROGRAM`] with both verifier paths present.
fn script_verifier_harness(label: &str) -> PathBuf {
    let (harness, into) = common::wire_sdk_harness(label, SCRIPT_VERIFIER_PROGRAM);
    common::write_sibling(&harness, "scripts/verify.sh", "#!/bin/sh\n");
    common::write_sibling(&harness, "tests/held.rs", "\n");
    drift::emit_program(&into, EmitOptions::default()).unwrap();
    harness
}

#[test]
fn a_script_verifier_naming_a_file_tool_leaves_that_tool_with_no_degree_finding() {
    let harness = script_verifier_harness("tool-script-verifier");

    let declarations = drift::read_declarations(&harness.join(".temper")).unwrap();
    assert!(
        declarations
            .runs
            .iter()
            .any(|row| row.member == "root/requirement/verified"
                && row.tool == "root/tool/verify.sh"),
        "the lock carries the verifier's edge as a runs row whose runner is its requirement: {:?}",
        declarations.runs
    );

    let run = common::check_in(&harness, &["."], Some("github"));
    let findings = run.findings();
    assert!(
        common::findings_for(&findings, "tool.degree").is_empty(),
        "the script verifier runs the tool, so the tool is run: {}",
        run.output
    );
    assert!(
        common::findings_for(&findings, "graph.route").is_empty(),
        "the requirement's edge resolves: {}",
        run.output
    );
}

#[test]
fn a_script_verifier_without_a_tool_is_still_accepted() {
    let harness = script_verifier_harness("tool-script-verifier-toolless");

    let declarations = drift::read_declarations(&harness.join(".temper")).unwrap();
    assert!(
        !declarations
            .runs
            .iter()
            .any(|row| row.member == "root/requirement/tested"),
        "a toolless verifier runs nothing: {:?}",
        declarations.runs
    );

    let run = common::check_in(&harness, &["."], Some("github"));
    assert!(
        common::findings_for(&run.findings(), "requirement.admissibility").is_empty(),
        "a toolless script verifier is admissible: {}",
        run.output
    );
}
