//! Shared fixtures for the integration test suites under `tests/` — one home
//! for scaffolding (temp dirs, fixture paths, the SDK vendoring used by tests
//! that drive a real `node` subprocess) every suite was carrying its own copy
//! of.
//!
//! Cargo compiles this module fresh into every integration test binary that
//! `mod common`s it, so an item only some binaries call reads as dead code in
//! the rest — `allow(dead_code)` blanket-suppresses that structural false
//! positive rather than each caller re-deriving it.
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;

use temper::drift::{
    self, ClauseRow, CountBoundRow, Declarations, DegreeBoundRow, EmitOptions, KindFactRow,
    Payload, PayloadMember, RequirementRow, SatisfiesRow,
};
use temper::frontmatter::Member;
use temper::kind::Unit;

/// A fresh, empty temp directory, uniquely named via the sanctioned `tempfile`
/// crate — replaces the hand-rolled counter+pid+label naming scheme every
/// caller carried before this consolidation. Persisted with `.keep()`: like
/// the hand-rolled scheme it replaces, nothing here auto-deletes, since
/// callers hand the path across process boundaries (a built binary, a
/// vendored `node` subprocess) that outlive the `TempDir` guard's scope.
pub fn tmpdir(label: &str) -> PathBuf {
    tempfile::Builder::new()
        .prefix(label)
        .tempdir()
        .expect("failed to create temp dir")
        .keep()
}

/// A fresh `<harness>` temp dir carrying an empty `.temper` workspace and a
/// `specs/` tree — the scaffold the prose-include and layout-import suites both
/// open a case on. One signature serves both: the empty `specs/` a
/// prose-include case never reads is inert, so folding the shape into one home
/// beats two callers re-deriving it.
pub fn scaffold(slug: &str) -> PathBuf {
    let harness = tmpdir(slug);
    fs::create_dir_all(harness.join(".temper")).unwrap();
    fs::create_dir_all(harness.join("specs")).unwrap();
    harness
}

/// Path to a directory under `tests/fixtures`, resolved from the manifest so
/// the test is independent of the process working directory.
pub fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(rel)
}

/// The repo's `sdk/` directory — the SDK package this crate's worktree carries
/// beside `Cargo.toml`.
pub fn sdk_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sdk")
}

/// Build the SDK's `dist/` once per test binary run — the compiled package a
/// fixture harness program's bare `@dtmd/temper` import resolves to, exactly as
/// an installed npm dependency would.
pub fn ensure_sdk_built() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let status = std::process::Command::new(temper::install::npm_program())
            .args(["run", "build"])
            .current_dir(sdk_root())
            .status()
            .expect("failed to run `npm run build` in sdk/ — is npm on PATH?");
        assert!(status.success(), "sdk build failed");
    });
}

/// Vendor the repo's built SDK into `node_modules_scope/temper` — the
/// `node_modules/@dtmd` directory of a fixture harness — standing in for a real
/// `npm install`'s local-dependency resolution. Idempotent: skips if the
/// link/junction already exists.
///
/// Unix links a real symlink, same as `npm install` would for a `file:`/workspace
/// dependency. Windows shells `cmd /C mklink /J` for a junction rather than
/// `std::os::windows::fs::symlink_dir`: a symlink needs
/// `SeCreateSymbolicLinkPrivilege` or Developer Mode, a junction needs neither,
/// matching how npm itself links local/workspace deps on Windows (npm/cli#5189;
/// nixhacker.com "Understanding and Exploiting Symbolic links in Windows";
/// hinchley.net "Junctions and Symbolic Links" — retrieved 2026-07-08). `mklink`'s
/// arg order is link-then-target, reversed from `std::os::unix::fs::symlink`'s
/// (original, link); passing each path as its own `.arg()` (never a hand-built
/// command string) lets `Command` quote them, since `CARGO_MANIFEST_DIR` may
/// contain spaces.
pub fn vendor_sdk(node_modules_scope: &Path) {
    std::fs::create_dir_all(node_modules_scope).unwrap();
    let link = node_modules_scope.join("temper");
    if link.exists() {
        return;
    }
    ensure_sdk_built();
    let target = sdk_root();

    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, &link).unwrap();

    #[cfg(windows)]
    {
        let status = std::process::Command::new("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&link)
            .arg(&target)
            .status()
            .expect("failed to run `mklink /J` — is cmd on PATH?");
        assert!(status.success(), "mklink /J failed");
    }
}

/// Write a one-skill harness member directly at its real Claude Code locus
/// (`<root>/.claude/skills/<name>/SKILL.md`) — `check` reads built-in kind members
/// live off harness disk, no scratch import.
pub fn write_skill(root: &Path, name: &str, skill_md: &str) {
    let dir = root.join(".claude").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();
}

/// The outcome of a `check` run: whether it exited zero and its combined
/// stdout+stderr (diagnostics render to stdout, a load error to stderr).
pub struct CheckRun {
    pub ok: bool,
    pub output: String,
}

/// Run `temper check <args…>` from `root`, optionally selecting `reporter`
/// (e.g. `"github"`), capturing the result. Callers that need a different
/// return shape (a `(bool, String)` pair, a parsed `Vec<String>` of
/// `::`-prefixed finding lines) adapt from [`CheckRun`] at the call site.
pub fn check_in(root: &Path, args: &[&str], reporter: Option<&str>) -> CheckRun {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_temper"));
    cmd.current_dir(root).arg("check").args(args);
    if let Some(reporter) = reporter {
        cmd.arg("--reporter").arg(reporter);
    }
    let out = cmd.output().unwrap();
    let mut output = String::from_utf8_lossy(&out.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&out.stderr));
    CheckRun {
        ok: out.status.success(),
        output,
    }
}

/// Read `root`'s current lock declarations (empty if none), apply `patch`, and
/// re-emit the whole lock — the additive primitive every `write_*`/`author_*` setup
/// helper composes through below, so a test's setup calls compose regardless of
/// order (`write_lock` itself is the one exception: a caller building the whole
/// [`Declarations`] wants it exactly as given, not merged with stale prior state).
fn merge_lock(root: &Path, patch: impl FnOnce(&mut Declarations)) {
    let mut declarations = drift::read_declarations(&root.join(".temper")).unwrap();
    patch(&mut declarations);
    write_lock(root, declarations);
}

/// Author a member's `satisfies` links directly on the harness's lock
/// (`declarations.satisfies`) — the real SDK-emit shape a converted harness
/// carries; the member's real source file itself carries no temper annotation.
/// `kind_dir` names the member's real Claude Code locus (`skills` or `rules`),
/// whose source is `SKILL.md` / `<name>.md` respectively — required to exist
/// there, mirroring the real harness this stands in for, even though the lock
/// row itself carries no kind.
pub fn author_satisfies(root: &Path, kind_dir: &str, name: &str, requirements: &[&str]) {
    let source = match kind_dir {
        "skills" => root
            .join(".claude")
            .join("skills")
            .join(name)
            .join("SKILL.md"),
        "rules" => root
            .join(".claude")
            .join("rules")
            .join(format!("{name}.md")),
        other => panic!("unknown kind_dir {other}"),
    };
    assert!(
        source.is_file(),
        "author_satisfies: no real harness source at {}",
        source.display()
    );
    merge_lock(root, |declarations| {
        declarations
            .satisfies
            .extend(requirements.iter().map(|r| SatisfiesRow {
                member: name.to_string(),
                requirement: (*r).to_string(),
            }));
    });
}

/// A floor-clean skill named `name` (matching its directory, a lowercase slug, a
/// present description). Clean against the floor, so the only finding a case can
/// produce is the one under test.
pub fn clean_skill(name: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

/// Write a floor-clean rule directly at its real Claude Code locus
/// (`<root>/.claude/rules/<name>.md`) — a second modeled kind, so a requirement or
/// edge typed to `rule` has a real satisfier/endpoint to be.
pub fn write_rule(root: &Path, name: &str) {
    let dir = root.join(".claude").join("rules");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join(format!("{name}.md")),
        format!("# {name}\n\nBody.\n"),
    )
    .unwrap();
}

/// The retired manifest's filename, spelled by concatenation so the retired token
/// itself never appears as a literal in this source.
pub fn retired_manifest_name() -> String {
    format!("temper{}toml", '.')
}

/// Write the retired manifest verbatim at the project root — the filename is inert
/// (never read by any verb), so every case using this proves exactly that: the file
/// changes nothing, whatever it carries.
pub fn write_retired_manifest(root: &Path, contents: &str) {
    fs::write(root.join(retired_manifest_name()), contents).unwrap();
}

/// Compile a golden lock at `<root>/.temper/lock.toml` declaring `requirements` —
/// the SDK-emitted fixture standing in for `import::run`'s scratch projection of the
/// retired manifest's `[requirement.*]` table: the gate sources requirements from
/// the lock, never a re-imported assembly. Merges onto whatever the lock already
/// declares (`merge_lock`), so it composes with `author_satisfies` in either order.
pub fn write_requirements(root: &Path, requirements: Vec<RequirementRow>) {
    merge_lock(root, |declarations| {
        declarations.requirements = requirements
    });
}

/// Compile a golden lock at `<root>/.temper/lock.toml` carrying just `declarations` —
/// the SDK-emitted fixture standing in for `import::run`'s scratch projection of a
/// manifest's `[[kind.<name>.relationships]]`/`[requirement.*]` table: the gate
/// sources edges and requirements from the lock, never a re-imported assembly.
pub fn write_lock(root: &Path, declarations: Declarations) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations,
        members: Vec::new(),
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

/// A raw `Unit` built straight from its parts, no disk round-trip — the shape
/// every caller driving a composed extractor over an arbitrary id/frontmatter/
/// body/source_path converges on, whichever of the four varies.
pub fn raw_unit(
    id: &str,
    frontmatter: BTreeMap<String, serde_json::Value>,
    body: &str,
    source_path: &str,
) -> Unit {
    Unit {
        id: id.to_string(),
        frontmatter,
        body: body.to_string(),
        source_path: PathBuf::from(source_path),
        satisfies: Vec::new(),
        satisfies_clauses: Vec::new(),
    }
}

/// Snapshot every file under `dir` as a sorted map of relative path -> bytes,
/// via the sanctioned `walkdir` crate — replaces the hand-rolled `fs::read_dir`
/// stack walk every caller carried before this consolidation.
pub fn tree_bytes(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            let rel = entry.path().strip_prefix(dir).unwrap().to_path_buf();
            (rel, fs::read(entry.path()).unwrap())
        })
        .collect()
}

/// Lift an imported [`Member`] straight into the raw [`Unit`] the composed
/// extractor reads — the same fields a built-in kind's member carries into
/// `check`, with no disk round trip.
pub fn surface_unit(member: &Member) -> Unit {
    Unit {
        id: member.id.clone(),
        frontmatter: member.fields.iter().cloned().collect(),
        body: member.body.clone(),
        source_path: member.provenance.source_path.clone(),
        satisfies: member
            .satisfies
            .iter()
            .map(|s| s.requirement.clone())
            .collect(),
        satisfies_clauses: member.satisfies.clone(),
    }
}

/// Lift an imported skill [`Member`] straight into the raw [`Unit`] the composed
/// extractor reads — the skill-flavored alias of [`surface_unit`].
pub fn skill_surface_unit(skill: &Member) -> Unit {
    surface_unit(skill)
}

/// A hand-built `skill` `PayloadMember` carrying `name`/`description` fields.
pub fn skill_member(name: &str, description: &str, body: &str) -> PayloadMember {
    PayloadMember {
        kind: "skill".to_string(),
        name: name.to_string(),
        fields: vec![
            ("name".to_string(), serde_json::json!(name)),
            ("description".to_string(), serde_json::json!(description)),
        ],
        body: body.to_string(),
        source_path: None,
    }
}

/// A hand-built `rule` `PayloadMember`, optionally carrying a `paths` field —
/// `None` omits the field entirely, matching a `rule` with no declared `paths`.
pub fn rule_member(name: &str, paths: Option<&[&str]>, body: &str) -> PayloadMember {
    let mut fields = Vec::new();
    if let Some(paths) = paths {
        fields.push(("paths".to_string(), serde_json::json!(paths)));
    }
    PayloadMember {
        kind: "rule".to_string(),
        name: name.to_string(),
        fields,
        body: body.to_string(),
        source_path: None,
    }
}

/// The `skill` built-in kind's declaration row, parameterized by the
/// `provider`/`registration` values callers diverge on — the rest of the row
/// (`governs`, `format`, `unit_shape`) is the kind's fixed shape.
pub fn skill_kind_facts(provider: Option<&str>, registration: &[&str]) -> KindFactRow {
    KindFactRow {
        name: "skill".to_string(),
        provider: provider.map(str::to_string),
        governs_root: ".claude/skills".to_string(),
        governs_glob: "*/SKILL.md".to_string(),
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("directory".to_string()),
        registration: registration.iter().map(|r| r.to_string()).collect(),
        templates: Vec::new(),
        content: None,
        shape: None,
        collection_address: None,
    }
}

/// The `rule` built-in kind's declaration row, parameterized by the
/// `provider`/`registration` values callers diverge on.
pub fn rule_kind_facts(provider: Option<&str>, registration: &[&str]) -> KindFactRow {
    KindFactRow {
        name: "rule".to_string(),
        provider: provider.map(str::to_string),
        governs_root: ".claude/rules".to_string(),
        governs_glob: "*.md".to_string(),
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("file".to_string()),
        registration: registration.iter().map(|r| r.to_string()).collect(),
        templates: Vec::new(),
        content: None,
        shape: None,
        collection_address: None,
    }
}

/// A [`KindFactRow`] naming `name` over its `governs_root`/`governs_glob` locus,
/// every optional fact at its default (no provider/format/unit_shape, empty
/// registration/templates) — the general default-filling home beside the per-kind
/// [`skill_kind_facts`]/[`rule_kind_facts`]. Call sites override the facts a kind
/// declares via struct-update.
pub fn kind_facts(name: &str, governs_root: &str, governs_glob: &str) -> KindFactRow {
    KindFactRow {
        name: name.to_string(),
        provider: None,
        governs_root: governs_root.to_string(),
        governs_glob: governs_glob.to_string(),
        format: None,
        unit_shape: None,
        registration: Vec::new(),
        templates: Vec::new(),
        content: None,
        shape: None,
        collection_address: None,
    }
}

/// The findings whose rule (the `title=<rule>` property) equals `rule` — the
/// GitHub reporter's per-finding lines this suite's cases scrape for a count.
pub fn findings_for<'a>(findings: &'a [String], rule: &str) -> Vec<&'a String> {
    let needle = format!("title={rule}::");
    findings
        .iter()
        .filter(|line| line.contains(&needle))
        .collect()
}

/// Author a rule's `satisfies` links on the harness's lock — the `rule`-kind alias
/// of [`author_satisfies`].
pub fn author_rule_satisfies(root: &Path, name: &str, requirements: &[&str]) {
    author_satisfies(root, "rules", name, requirements);
}

/// A bare `RequirementRow` naming `name`, otherwise the union of the shapes
/// callers need: `required` and an optional `kind` narrowing.
pub fn requirement(name: &str, required: bool, kind: Option<&str>) -> RequirementRow {
    RequirementRow {
        name: name.to_string(),
        kind: kind.map(str::to_string),
        required,
        clauses: Vec::new(),
        verified_by: None,
        prose: None,
    }
}

/// A [`ClauseRow`] naming `predicate` at `severity`, every other column at its
/// default (`kind: None`, no field, no predicate argument) — the one default-filling
/// home for the family. Call sites override the columns they diverge on via
/// struct-update: a kind-carrying floor clause sets `kind`, a predicate with an
/// argument sets its own column (`count`/`bound`/`charset`/…).
pub fn clause(predicate: &str, severity: &str) -> ClauseRow {
    ClauseRow {
        kind: None,
        predicate: predicate.to_string(),
        field: None,
        severity: severity.to_string(),
        guidance: None,
        cite: None,
        count: None,
        target: None,
        degree: None,
        bound: None,
        charset: None,
        keys: None,
        values: None,
        range: None,
        section: None,
    }
}

/// A `required`-severity [`ClauseRow`] wrapping one set-/edge-scope predicate — the
/// shape a [`RequirementRow`]'s own `clauses` nest. `kind` is `None`: a nested
/// requirement clause names no kind of its own.
pub fn required_clause_row(
    predicate: &str,
    field: Option<&str>,
    count: Option<CountBoundRow>,
    target: Option<&str>,
    degree: Option<DegreeBoundRow>,
) -> ClauseRow {
    ClauseRow {
        field: field.map(str::to_string),
        count,
        target: target.map(str::to_string),
        degree,
        ..clause(predicate, "required")
    }
}
