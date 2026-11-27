//! The program's module closure — every local module a real `emit` run loaded — is
//! fingerprinted into the lock's `module` family, owned by the root member, so editing a
//! module the program imports is drift even when no `input()` names it. An installed
//! package, whose real path sits under `node_modules`, is its lockfile's to pin, and a
//! module the repository's ignore rules exclude is no committed source; neither gets a
//! row. `check` judges the closure off the lock alone, with no Node in the loop.

use std::fs;
use std::path::Path;

use temper::drift::{self, EmitOptions};

use crate::common;

/// A one-rule program whose rule name rides a sibling module, beside an installed package.
const PROGRAM: &str = r#"
import { harness, text } from "@dtmd/temper";
import { rule } from "@dtmd/temper/claude-code";
import { RULE_NAME } from "./members.ts";
import { suffix } from "helper-pkg";

const program = harness({
  members: [
    rule({
      name: RULE_NAME,
      paths: [`src/**/*.${suffix}`],
      prose: text`
        # Host
      `,
    }),
  ],
});

export default program;
"#;

const MEMBERS_MODULE: &str = "export const RULE_NAME = \"host\";\n";

/// Wire [`PROGRAM`], its sibling module, and a real installed `helper-pkg` under the
/// program's own `node_modules`, then run one real emit.
fn emitted(label: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let (harness, into) = common::wire_sdk_harness(label, PROGRAM);
    fs::write(into.join("members.ts"), MEMBERS_MODULE).unwrap();
    common::write_sibling(
        &into,
        "node_modules/helper-pkg/package.json",
        "{ \"name\": \"helper-pkg\", \"type\": \"module\", \"main\": \"index.js\" }\n",
    );
    common::write_sibling(
        &into,
        "node_modules/helper-pkg/index.js",
        "export const suffix = \"rs\";\n",
    );
    drift::emit_program(&into, EmitOptions::default()).unwrap();
    (harness, into)
}

/// The lock's `[[declaration.module]]` rows, as `(member, source_path)`.
fn module_rows(into: &Path) -> Vec<(String, String)> {
    let doc: toml_edit::DocumentMut = fs::read_to_string(into.join("lock.toml"))
        .unwrap()
        .parse()
        .unwrap();
    doc["declaration"]
        .get("module")
        .and_then(|item| item.as_array_of_tables())
        .map(|rows| {
            rows.iter()
                .map(|row| {
                    (
                        row["member"].as_str().unwrap().to_string(),
                        row["source_path"].as_str().unwrap().to_string(),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

/// The `root.fresh` finding lines a github-reporter `check` run prints.
fn fresh_findings(harness: &Path) -> Vec<String> {
    let (findings, _ok) = common::check_harness(harness);
    findings
        .into_iter()
        .filter(|line| line.contains("root.fresh"))
        .collect()
}

#[test]
fn emit_writes_a_provenance_row_for_each_local_module_the_program_loaded() {
    let (_harness, into) = emitted("module-closure-rows");

    let rows = module_rows(&into);
    let paths: Vec<&str> = rows.iter().map(|(_, path)| path.as_str()).collect();
    assert!(
        paths.contains(&".temper/harness.ts") && paths.contains(&".temper/members.ts"),
        "the entry and the module it imports are both fingerprinted: {paths:?}"
    );
    assert!(
        rows.iter().all(|(member, _)| member == "root"),
        "every module row is the root member's: {rows:?}"
    );
    let mut sorted = paths.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), paths.len(), "one row per module: {paths:?}");

    // And the module family is a source dependency, never a declaration a lock round-trip
    // reads back.
    let lock = into.join("lock.toml");
    let declarations =
        drift::parse_declarations(&lock, &fs::read_to_string(&lock).unwrap()).unwrap();
    assert!(declarations.modules.is_empty());
}

#[test]
fn a_module_under_node_modules_gets_no_provenance_row() {
    let (_harness, into) = emitted("module-closure-node-modules");

    let rows = module_rows(&into);
    assert!(
        !rows.is_empty(),
        "the program's own modules are fingerprinted"
    );
    assert!(
        rows.iter().all(|(_, path)| !path.contains("node_modules")),
        "the installed package is its lockfile's to pin, never this lock's: {rows:?}"
    );
}

#[test]
fn a_moved_program_module_is_a_root_level_fresh_finding_with_the_re_emit_remedy() {
    let (harness, into) = emitted("module-closure-moved");

    // The edit changes no projection: only the module's own bytes move.
    fs::write(
        into.join("members.ts"),
        format!("// a refactor note\n{MEMBERS_MODULE}"),
    )
    .unwrap();
    // `check` judges off the lock alone — with the SDK unreachable, no Node run could
    // even import it.
    fs::remove_dir_all(into.join("node_modules")).unwrap();

    let fresh = fresh_findings(&harness);
    assert_eq!(
        fresh.len(),
        1,
        "one finding, for the one moved module: {fresh:?}"
    );
    assert!(
        fresh[0].contains(".temper/members.ts") && fresh[0].contains("`root`"),
        "the finding names the module and its root owner: {}",
        fresh[0]
    );
    assert!(
        fresh[0].contains("re-emit to reconcile"),
        "and the remedy is re-emitting the program: {}",
        fresh[0]
    );
}

#[test]
fn an_unchanged_program_module_closure_reports_no_fresh_finding() {
    let (harness, into) = emitted("module-closure-unchanged");
    assert!(
        !module_rows(&into).is_empty(),
        "the closure was fingerprinted, so its silence is a verdict"
    );

    assert!(
        fresh_findings(&harness).is_empty(),
        "a closure nobody touched reads fresh"
    );
}

/// A program importing a tracked sibling module and a gitignored one, beside a
/// `file:`-linked workspace package whose built `dist/` is gitignored.
const IGNORED_PROGRAM: &str = r#"
import { harness, text } from "@dtmd/temper";
import { rule } from "@dtmd/temper/claude-code";
import { RULE_NAME } from "./members.ts";
import { GENERATED } from "./generated/names.ts";
import { suffix } from "linked-pkg";

const program = harness({
  members: [
    rule({
      name: `${RULE_NAME}-${GENERATED}`,
      paths: [`src/**/*.${suffix}`],
      prose: text`
        # Host
      `,
    }),
  ],
});

export default program;
"#;

/// Wire [`IGNORED_PROGRAM`]: a tracked `members.ts`, a gitignored `generated/` module,
/// and `linked-pkg` living in the repository at `packages/linked-pkg`, reached through a
/// `node_modules` symlink, whose `dist/` the repository ignores. Then run one real emit.
fn emitted_with_ignored(label: &str) -> std::path::PathBuf {
    let (harness, into) = common::wire_sdk_harness(label, IGNORED_PROGRAM);
    fs::create_dir(harness.join(".git")).unwrap();
    common::write_sibling(
        &harness,
        ".gitignore",
        ".temper/node_modules/\n.temper/generated/\ndist/\n",
    );
    fs::write(into.join("members.ts"), MEMBERS_MODULE).unwrap();
    common::write_sibling(
        &into,
        "generated/names.ts",
        "export const GENERATED = \"gen\";\n",
    );
    let package = harness.join("packages/linked-pkg");
    common::write_sibling(
        &package,
        "package.json",
        "{ \"name\": \"linked-pkg\", \"type\": \"module\", \"main\": \"dist/index.js\" }\n",
    );
    common::write_sibling(&package, "dist/index.js", "export const suffix = \"rs\";\n");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&package, into.join("node_modules/linked-pkg")).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&package, into.join("node_modules/linked-pkg")).unwrap();
    drift::emit_program(&into, EmitOptions::default()).unwrap();
    into
}

#[test]
fn a_gitignored_module_the_program_loaded_gets_no_provenance_row() {
    let into = emitted_with_ignored("module-closure-gitignored");

    let paths: Vec<String> = module_rows(&into).into_iter().map(|(_, p)| p).collect();
    assert!(
        paths.iter().any(|p| p == ".temper/members.ts"),
        "a tracked sibling keeps its row: {paths:?}"
    );
    assert!(
        paths.iter().all(|p| !p.contains("generated")),
        "the gitignored module is no committed source: {paths:?}"
    );
}

#[test]
fn a_gitignored_module_reached_through_a_linked_package_gets_no_provenance_row() {
    let into = emitted_with_ignored("module-closure-linked");

    let paths: Vec<String> = module_rows(&into).into_iter().map(|(_, p)| p).collect();
    assert!(
        paths.iter().any(|p| p == ".temper/harness.ts"),
        "the program's own entry keeps its row: {paths:?}"
    );
    assert!(
        paths.iter().all(|p| !p.contains("linked-pkg")),
        "the linked package's ignored build output gets no row: {paths:?}"
    );
}

/// `lock_text` with every `[[declaration.module]]` row `drop` selects removed — the family
/// key too once nothing is left. The one home for the suites whose committed or snapshotted
/// lock text cannot carry a row: the SDK `common::vendor_sdk` links in by symlink is
/// fingerprinted by its real path, a `../`-spelled row naming this checkout and hashing
/// whatever `sdk/dist` the last build left, where an adopter's installed SDK sits under
/// `node_modules` and gets no row at all.
pub(crate) fn without_module_rows(lock_text: &str, drop: impl Fn(&str) -> bool) -> String {
    let mut doc: toml_edit::DocumentMut = lock_text.parse().unwrap();
    let Some(declaration) = doc
        .get_mut("declaration")
        .and_then(|item| item.as_table_mut())
    else {
        return lock_text.to_string();
    };
    let Some(rows) = declaration
        .get_mut("module")
        .and_then(|item| item.as_array_of_tables_mut())
    else {
        return lock_text.to_string();
    };
    rows.retain(|row| !drop(row["source_path"].as_str().unwrap()));
    if rows.is_empty() {
        declaration.remove("module");
    }
    doc.to_string()
}

/// Whether a module row names the vendored SDK — a path out of the harness root.
pub(crate) fn is_vendored_sdk(source_path: &str) -> bool {
    source_path.starts_with("../")
}
