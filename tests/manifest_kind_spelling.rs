//! The gate resolves a manifest member's authored kind before corpus lookup
//! (`specs/architecture/15-kinds.md`, "Decision: kind identity carries a provider axis";
//! GATE-KIND-RESOLVE).
//!
//! A module-carried `[[member]]` stamps the **qualified** identity `<provider>.<name>`
//! (the SDK's `members.ts`), while the gate keys its corpus by the **bare** name the
//! dispatch loop reads (`corpus.get(&kind.name)`). These end-to-end checks pin the two
//! halves of the fix: a `claude-code.rule` member reaches the `rule` package and is
//! **checked** (not silently skipped under an unread qualified key), and a member whose
//! kind resolves to no built-in or custom definition surfaces a **loud finding** rather
//! than a silent `checked 0` (`.claude/rules/collaboration.md`, "a silent skip reads as
//! done").
//!
//! Driven across the real process boundary because the resolve happens inside `check`'s
//! gate and its effect is observable only in the rendered diagnostics + exit code.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A rule carrying Cursor `.mdc` keys (`globs`/`alwaysApply`) Claude Code silently
/// ignores — the exact mistake the rule contract's `required` `forbidden_keys` clause
/// catches. If the member reaches the `rule` package, `check` fires and exits non-zero.
const FORBIDDEN_KEY_RULE: &str = "---\n\
globs: \"**/*.rs\"\n\
alwaysApply: true\n\
---\n\
# Rust conventions\n\
\n\
This frontmatter loads nothing in Claude Code.\n";

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "manifest-kind-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// The outcome of a `check` run: whether it exited zero and its combined stdout+stderr.
struct CheckRun {
    ok: bool,
    output: String,
}

/// `temper import <root> --into <root>/.temper` — the persistent import that serializes
/// the manifest with **document-carried** members (baked features under a bare `kind`),
/// the carriage the SDK also emits. The member's `kind` is then re-spelled to exercise
/// the gate's resolve.
fn import(root: &Path) {
    let status = Command::new(BIN)
        .arg("import")
        .arg(root)
        .arg("--into")
        .arg(root.join(".temper"))
        .status()
        .unwrap();
    assert!(status.success(), "import should succeed: {status}");
}

/// Run `temper check` from `root` (so its `temper.toml` is discovered), capturing the
/// exit status and rendered diagnostics.
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

/// Import a one-rule harness (`FORBIDDEN_KEY_RULE`), then rewrite the member's authored
/// `kind` to `spelling` and strip the surface trees. Stripping is deliberate: it removes
/// the copy-tree fallback the gate reaches only when the manifest carries **no** corpus,
/// so the run judges the manifest member alone — a member left unread under a qualified
/// key would leave the gate with nothing to check, the exact silent skip under test.
fn manifest_with_rule_kind_spelled(label: &str, spelling: &str) -> PathBuf {
    let root = tmpdir(label);
    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), FORBIDDEN_KEY_RULE).unwrap();
    import(&root);

    let manifest_path = root.join("temper.toml");
    let manifest = fs::read_to_string(&manifest_path).unwrap();
    // `import` writes the built-in rule member under the bare `kind = "rule"`; re-spell it
    // to the identity the SDK stamps (or an unrecognized kind) to drive the resolve.
    let respelled = manifest.replace("kind = \"rule\"", &format!("kind = \"{spelling}\""));
    assert_ne!(
        manifest, respelled,
        "the imported manifest must carry a bare `kind = \"rule\"` to re-spell"
    );
    fs::write(&manifest_path, respelled).unwrap();

    fs::remove_dir_all(root.join(".claude")).unwrap();
    fs::remove_dir_all(root.join(".temper").join("rules")).unwrap();
    root
}

#[test]
fn a_qualified_claude_code_rule_member_is_checked_against_the_rule_package() {
    // The SDK stamps the qualified `claude-code.rule`; the gate must resolve it to the
    // bare `rule` slice so the member reaches the rule contract. With the surface stripped,
    // the only thing to check is this manifest member — so a red run proves it was
    // dispatched, not skipped.
    let root = manifest_with_rule_kind_spelled("qualified-rule", "claude-code.rule");
    let run = check_in(&root);

    assert!(
        !run.ok,
        "a `claude-code.rule` member carrying forbidden keys must fail — proof it reached \
         the rule contract, not an unread qualified key, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("forbidden_keys"),
        "the finding names the rule clause the qualified member tripped, got:\n{}",
        run.output
    );
    // The coverage note counts the member under its resolved built-in identity — checked,
    // never a silent zero.
    assert!(
        run.output.contains("claude-code.rule (1)"),
        "the resolved member is counted checked under its kind, got:\n{}",
        run.output
    );
}

#[test]
fn an_unrecognized_manifest_kind_is_a_loud_finding_not_a_silent_zero() {
    // A member kind resolving to no built-in or custom definition must be reported, never
    // dropped to a silent `checked 0` (`.claude/rules/collaboration.md`).
    let root = manifest_with_rule_kind_spelled("unknown-kind", "claude-code.widget");
    let run = check_in(&root);

    assert!(
        !run.ok,
        "an unrecognized manifest kind must fail the run, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("manifest.unknown-kind"),
        "the unrecognized kind surfaces as a loud finding, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("claude-code.widget"),
        "the finding names the unresolved kind so the author knows what to fix, got:\n{}",
        run.output
    );
}
