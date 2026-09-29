//! Shared fixtures for the integration suites under `tests/it/` — one home for
//! scaffolding (temp dirs, fixture paths, the SDK vendoring used by tests that
//! drive a real `node` subprocess) every suite would otherwise carry its own
//! copy of.
//!
//! The suites compile as one crate, so this module is compiled once with every
//! caller in view. Dead-code analysis is therefore exact here and carries no
//! blanket `allow`: a helper no suite calls is real dead scaffolding, and the
//! clippy gate fails on it.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Once, OnceLock};

use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

use temper::builtin_kind;
use temper::check::Diagnostic;
use temper::compose::{self, EnforcementMode};
use temper::drift::{
    self, AssemblyFactRow, ClauseRow, CollectionAddressRow, CountBoundRow, Declarations,
    DegreeBoundRow, EmitOptions, KindFactRow, LayoutRegionRow, LayoutRow, MentionRow, Payload,
    PayloadMember, RegistrationRow, RequirementRow, SatisfiesRow,
};
use temper::extract::Features;
use temper::frontmatter::Member;
use temper::import;
use temper::json_manifest::Manifest;
use temper::kind::{CustomKind, Unit};
use temper::tap::{TapEvent, TapRecord};

/// The fixed prefix every test binary's per-run fixture parent is named under, so
/// a sweep of all fixture debris this suite ever wrote is one `rm -rf
/// $TMPDIR/temper-fixtures-*`.
const RUN_PARENT_PREFIX: &str = "temper-fixtures-";

/// The one per-run parent directory every [`tmpdir`] nests under, created once per
/// test binary. Two things ride on the nesting: fixture debris is sweepable by the
/// one prefix above (persisted dirs otherwise accumulate at the temp root
/// monotonically), and a fixture that re-roots a walk above itself is bounded to
/// this binary's own fixtures instead of the whole temp root.
fn run_parent() -> &'static Path {
    static RUN_PARENT: OnceLock<PathBuf> = OnceLock::new();
    RUN_PARENT.get_or_init(|| {
        tempfile::Builder::new()
            .prefix(RUN_PARENT_PREFIX)
            .tempdir()
            .expect("failed to create the per-run fixture parent")
            .keep()
    })
}

/// A fresh, empty temp directory under this run's parent ([`run_parent`]), uniquely
/// named via the sanctioned `tempfile` crate — replaces the hand-rolled
/// counter+pid+label naming scheme every caller carried before this consolidation.
/// Persisted with `.keep()`: like the hand-rolled scheme it replaces, nothing here
/// auto-deletes, since callers hand the path across process boundaries (a built
/// binary, a vendored `node` subprocess) that outlive the `TempDir` guard's scope.
pub fn tmpdir(label: &str) -> PathBuf {
    tempfile::Builder::new()
        .prefix(label)
        .tempdir_in(run_parent())
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

/// A fresh `<harness>`/`<harness>/.temper` pair — `drift::emit` derives the projection
/// root from the workspace dir's parent, matching the seam's own topology: `.temper/`
/// sits beside `.claude/`. The pair-returning sibling of [`scaffold`], for the emit
/// suites that hold both paths.
pub fn workspace(label: &str) -> (PathBuf, PathBuf) {
    let harness = tmpdir(label);
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    (harness, into)
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
/// `npm install`'s local-dependency resolution. Idempotent: skips *linking* when
/// the link/junction already exists, and never skips the build — the link points
/// at `sdk/`, so its existence says nothing about whether `sdk/dist` holds the
/// bytes the caller is about to read. [`ensure_sdk_built`] therefore runs first
/// and unconditionally, which also makes a caller with a surviving link join that
/// `Once` instead of racing a sibling test's `rm -rf dist && tsc`.
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
    ensure_sdk_built();
    std::fs::create_dir_all(node_modules_scope).unwrap();
    let link = node_modules_scope.join("temper");
    if link.exists() {
        return;
    }
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

/// Wire a fixture harness under `<harness>/.temper/harness.ts` carrying `program`,
/// with a `node_modules/@dtmd/temper` resolving to the repo's own built SDK — the
/// stand-in for a real consumer's installed dependency. Returns the harness root and
/// its `.temper` directory. The seam every real-SDK test drives is the same; only the
/// authored fixture program differs, so it stays the caller's.
pub fn wire_sdk_harness(label: &str, program: &str) -> (PathBuf, PathBuf) {
    let harness = tmpdir(label);
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    fs::write(into.join("harness.ts"), program).unwrap();
    vendor_sdk(&into.join("node_modules").join("@dtmd"));
    (harness, into)
}

/// Write `body` at `<root>/<rel>`, creating the parent directories — the
/// create-parents-then-write primitive every locus writer below composes, and the
/// direct route for a file at no modeled locus at all (a plain repo file an
/// `@import` resolves to: the backing set is the whole repo, so a resolving target
/// need not itself be a harness member).
pub fn write_sibling(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

/// Write a one-skill harness member directly at its real Claude Code locus
/// (`<root>/.claude/skills/<name>/SKILL.md`) — `check` reads built-in kind members
/// live off harness disk, no scratch import.
pub fn write_skill(root: &Path, name: &str, skill_md: &str) {
    write_sibling(root, &format!(".claude/skills/{name}/SKILL.md"), skill_md);
}

/// Write a plugin manifest at its real Claude Code locus
/// (`<root>/.claude-plugin/plugin.json`) — never a layout invented for the test's
/// convenience.
pub fn write_plugin_json(root: &Path, body: &str) {
    write_sibling(root, ".claude-plugin/plugin.json", body);
}

/// Write a marketplace catalog at its real Claude Code locus
/// (`<root>/.claude-plugin/marketplace.json`) — never a layout invented for the
/// test's convenience.
pub fn write_marketplace_json(root: &Path, body: &str) {
    write_sibling(root, ".claude-plugin/marketplace.json", body);
}

/// Write an MCP manifest at its real Claude Code locus (`<root>/.mcp.json`, the harness
/// root itself) — never a layout invented for the test's convenience.
pub fn write_mcp_json(root: &Path, body: &str) {
    write_sibling(root, ".mcp.json", body);
}

/// Write settings at their real Claude Code locus (`<root>/.claude/settings.json`)
/// — never a layout invented for the test's convenience.
pub fn write_settings(root: &Path, body: &str) {
    write_sibling(root, ".claude/settings.json", body);
}

/// A manifest kind's read, end to end over `harness`: discover the kind's manifest
/// files off its own `governs` locus, then infer each one's collection members at the
/// kind's declared address. One `Manifest` per discovered file, so a caller still
/// asserts how many manifests were read and what surfaced in each.
///
/// `LocalOverride::Honored` is the engine's own walk (`compose::manifest_units`), so a
/// test reads what `check` reads. A fresh `Discovery` per call is deliberate here and
/// the reason the count-pinned suites (`tests/it/check_cost.rs`) keep their own hoisted
/// walk instead of calling through.
pub fn manifest_members(harness: &Path, kind: &CustomKind) -> Vec<Manifest> {
    let disc = import::Discovery::new(harness);
    let files = import::discover_kind_files(
        &disc,
        kind,
        kind.governs.as_ref().unwrap(),
        import::LocalOverride::Honored,
    );
    Manifest::read_kind(&files, kind).unwrap()
}

/// A manifest kind's members projected through the shared read-time fold — the same
/// `Features` a clause and the reachability gate range over, flattened across every
/// manifest [`manifest_members`] read. Each member's unit takes its source from the
/// manifest it was read off, exactly as `compose::manifest_units` does.
///
/// Named apart from [`features`], which builds a `Features` fixture from nothing.
pub fn kind_features(harness: &Path, kind: &CustomKind) -> Vec<Features> {
    let address = kind.collection_address.clone().unwrap();
    let mut features = Vec::new();
    for manifest in manifest_members(harness, kind) {
        let source = &manifest.provenance.source_path;
        for member in &manifest.members {
            features.push(builtin_kind::features(
                kind,
                &member.to_unit(&address, source),
                &[],
            ));
        }
    }
    features
}

/// The outcome of a `check` run: whether it exited zero and its combined
/// stdout+stderr (diagnostics render to stdout, a load error to stderr).
/// `stdout` keeps that stream alone, for the case asserting *which* stream a
/// finding reaches — a distinction `output` deliberately erases.
pub struct CheckRun {
    pub ok: bool,
    pub output: String,
    pub stdout: String,
}

/// The `title=` every announcement line carries, and the one thing that tells an
/// announcement apart from a disclosure note — both ride `::notice`, since neither is
/// a problem, so the command alone cannot separate them.
const ANNOUNCE_TITLE: &str = "title=temper.announce::";

impl CheckRun {
    /// Returns the github-reporter finding lines — each one `::error`/`::warning`/
    /// `::notice …`, in output order. A `::notice` finding is a disclosure note (what
    /// the gate checked); an announced input rides `::notice` too but is not a finding,
    /// so it is excluded here ([`CheckRun::announcements`] is its reader).
    pub fn findings(&self) -> Vec<String> {
        self.workflow_commands(|line| {
            (line.starts_with("::error") || line.starts_with("::warning"))
                || (line.starts_with("::notice") && !line.contains(ANNOUNCE_TITLE))
        })
    }

    /// Returns the github-reporter announcement lines — each one `::notice …`, naming
    /// one input that judged the run beyond the committed harness.
    pub fn announcements(&self) -> Vec<String> {
        self.workflow_commands(|line| line.starts_with("::notice") && line.contains(ANNOUNCE_TITLE))
    }

    /// The output's workflow-command lines the predicate selects, in output order.
    fn workflow_commands(&self, keep: impl Fn(&str) -> bool) -> Vec<String> {
        self.output
            .lines()
            .filter(|line| keep(line))
            .map(str::to_string)
            .collect()
    }
}

/// Run `temper check <args…>` from `root`, optionally selecting `reporter`
/// (e.g. `"github"`), capturing the result. Every `check` run reaches the binary
/// here — whatever its reporter, arg shape, or working directory — never a
/// re-spelled `Command`, and every reader of a github run's finding lines
/// reaches [`CheckRun::findings`] rather than re-spelling the filter; the
/// one-shot `--harness <root>` arg shape composes it through
/// [`check_harness_in`], and the GitHub-reporter projection of that shape one
/// step further still through [`check_harness`]. The one run this cannot express
/// is a caller driving a command string that carries the verb itself, since
/// `check` is prepended here.
pub fn check_in(root: &Path, args: &[&str], reporter: Option<&str>) -> CheckRun {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_temper"));
    cmd.current_dir(root).arg("check").args(args);
    if let Some(reporter) = reporter {
        cmd.arg("--reporter").arg(reporter);
    }
    let out = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let mut output = stdout.clone();
    output.push_str(&String::from_utf8_lossy(&out.stderr));
    CheckRun {
        ok: out.status.success(),
        output,
        stdout,
    }
}

/// Run `temper check --harness <harness>` from `harness` itself — the one-shot
/// wedge, no on-ramp step and no workspace — under `reporter`.
///
/// This is the only home for that arg shape. It stays fixed to exactly this
/// shape rather than growing a parameter per permutation: a run adding a flag,
/// passing a workspace positional, or working from some other directory is a
/// different run and reaches [`check_in`] directly.
pub fn check_harness_in(harness: &Path, reporter: Option<&str>) -> CheckRun {
    check_in(harness, &["--harness", harness.to_str().unwrap()], reporter)
}

/// Run the one-shot harness gate under the github reporter, returning `(finding
/// lines, exit success)`. Each finding is one `::error`/`::warning …` line.
pub fn check_harness(harness: &Path) -> (Vec<String>, bool) {
    let run = check_harness_in(harness, Some("github"));
    (run.findings(), run.ok)
}

/// Run `temper explain <target>` from `root`, returning stdout and stderr
/// concatenated. The combined stream is the contract, not a convenience: `explain`
/// narrates to stdout but refuses to stderr, so a stdout-only reader watching for a
/// string's *absence* reads a refusal as agreement. Every `explain` run over the real
/// binary reaches the verb here — the four private copies this replaced disagreed on
/// exactly that, two of them stdout-only.
pub fn explain_in(root: &Path, target: &str) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_temper"))
        .current_dir(root)
        .arg("explain")
        .arg(target)
        .output()
        .unwrap();
    let mut narration = String::from_utf8_lossy(&out.stdout).into_owned();
    narration.push_str(&String::from_utf8_lossy(&out.stderr));
    narration
}

/// Run `temper guard <root>` from inside `root` with `payload` on stdin, returning the
/// exit code and the guard's combined output. Mirrors the existing `check_*` family: the
/// one home for guard driver scaffolding, consolidating what install.rs and cli.rs were
/// re-implementing independently. Its `.`-argument twin is [`run_guard_from_root`]: the
/// two differ in exactly one variable, how the root is spelled.
pub fn run_guard(root: &Path, payload: &str) -> (Option<i32>, String) {
    run_guard_spawned_in(root, root.as_os_str(), payload)
}

/// Run `temper guard .` from inside `root` — the invocation shape `install` writes into
/// every settings.json guard hook command, where the argument is a bare `.` and the
/// harness root is the process's working directory.
pub fn run_guard_from_root(root: &Path, payload: &str) -> (Option<i32>, String) {
    run_guard_spawned_in(root, std::ffi::OsStr::new("."), payload)
}

/// A `PreToolUse` payload announcing a bare `Write` to `file_path` — the input half of the
/// two runners above, and the one home for the payload's key names.
///
/// The keys are a provider fact: `tool_name` and `tool_input` on the event envelope
/// (`code.claude.com/docs/en/hooks`, "PreToolUse input", retrieved 2026-09-25), and
/// `file_path` inside it (`code.claude.com/docs/en/tools-reference`, retrieved
/// 2026-09-07). Its two siblings carry the same envelope over the wider `tool_input` the
/// same reference documents: [`guard_write_payload_with_content`] and
/// [`guard_edit_payload`].
pub fn guard_write_payload(file_path: &str) -> String {
    serde_json::json!({
        "tool_name": "Write",
        "tool_input": { "file_path": file_path },
    })
    .to_string()
}

/// A `PreToolUse` `Write` payload landing whole-file `content` at `file_path` — the shape
/// the manifest guard reads a pending manifest's members off, a partial `Edit` carrying no
/// full manifest.
pub fn guard_write_payload_with_content(file_path: &str, content: &str) -> String {
    serde_json::json!({
        "tool_name": "Write",
        "tool_input": { "file_path": file_path, "content": content },
    })
    .to_string()
}

/// A `PreToolUse` `Edit` payload — the partial shape carrying replacement strings rather
/// than the whole file, which the guard reconstructs against the on-disk manifest.
pub fn guard_edit_payload(file_path: &str, old_string: &str, new_string: &str) -> String {
    serde_json::json!({
        "tool_name": "Edit",
        "tool_input": {
            "file_path": file_path,
            "old_string": old_string,
            "new_string": new_string,
        },
    })
    .to_string()
}

/// One provenance row of a guard fixture's lock — the four columns `emit`'s own roll-up
/// writes (`name`, `source_path`, `source_hash`, `emit_hash`), plus the kind whose
/// `[[<kind>]]` array the row lands in. No public type carries this shape: the engine's
/// `RollupEntry` is `pub(crate)`, and its hashes are exactly what a fixture needs to lie
/// about — a row at a fingerprint no file's bytes can produce is drifted by construction.
struct GuardLockMember {
    kind: String,
    name: String,
    source_path: String,
    source_hash: String,
    emit_hash: String,
}

/// A hand-authored lock at `<root>/.temper/lock.toml` — the third half of every guard
/// case, beside [`run_guard`] and the payload builders above, and the one home for the
/// lock vocabulary every guard fixture declares its rows in.
///
/// Hand-rendered on purpose. [`write_lock`] is not the home for this: it runs
/// `drift::emit`, which writes real projections at real fingerprints — the opposite of a
/// guard fixture, whose whole point is a declared row whose file is absent or drifted
/// (engineering.md, "A seam gate reads what the real writer wrote": a hand fixture is the
/// tool for the input a real writer cannot produce).
///
/// What keeps the hand spelling honest is [`GuardLock::write`]'s read-back: the rendered
/// bytes go straight through the engine's own reader and must lift to exactly the typed
/// rows declared here, and the mode must resolve to the one asked for. A column this home
/// spells differently from `src/` therefore fails every guard fixture at once, rather than
/// writing a lock that declares nothing while the warn arms stay green on the default.
pub struct GuardLock {
    mode: String,
    members: Vec<GuardLockMember>,
    declarations: Declarations,
}

impl GuardLock {
    /// A lock declaring `mode` as the assembly family's enforcement fact and nothing
    /// else. `mode` is a string, never a typed [`EnforcementMode`]: the vocabulary
    /// refusal `compose::mode_from_declarations` raises is a guard case of its own, and a
    /// typed mode could not express the value it refuses.
    pub fn declaring(mode: &str) -> Self {
        Self {
            mode: mode.to_string(),
            members: Vec::new(),
            declarations: Declarations {
                assembly: vec![AssemblyFactRow {
                    fact: "mode".to_string(),
                    value: Some(mode.to_string()),
                    from: None,
                    field: None,
                    to: None,
                }],
                ..Declarations::default()
            },
        }
    }

    /// Declare one emit-owned member of `kind` — its name, the projection path it owns,
    /// and the two fingerprints, in the roll-up's own column order.
    #[must_use]
    pub fn member(
        mut self,
        kind: &str,
        name: &str,
        source_path: &str,
        source_hash: &str,
        emit_hash: &str,
    ) -> Self {
        self.members.push(GuardLockMember {
            kind: kind.to_string(),
            name: name.to_string(),
            source_path: source_path.to_string(),
            source_hash: source_hash.to_string(),
            emit_hash: emit_hash.to_string(),
        });
        self
    }

    /// Declare one kind fact row — the container case's `[[declaration.kind]]`.
    #[must_use]
    pub fn kind_row(mut self, row: KindFactRow) -> Self {
        self.declarations.kinds.push(row);
        self
    }

    /// Declare one contract clause row. A guard fixture spells the `label` itself: the
    /// lock-shaped row is what `compose::root_contract`'s rows-or-default rule reads.
    #[must_use]
    pub fn clause_row(mut self, row: ClauseRow) -> Self {
        self.declarations.clauses.push(row);
        self
    }

    /// Declare one registration member — the identity and collection address a lock row
    /// carries. `fields` stays empty: they live in the projected manifest, never a second
    /// copy the engine reads back.
    #[must_use]
    pub fn registration(mut self, kind: &str, key: &str, manifest: &str, key_path: &str) -> Self {
        self.declarations.registrations.push(RegistrationRow {
            kind: kind.to_string(),
            key: key.to_string(),
            manifest: manifest.to_string(),
            key_path: key_path.to_string(),
            fields: Vec::new(),
        });
        self
    }

    /// Render the lock at `<root>/.temper/lock.toml`, then read the bytes back through
    /// the engine's own reader and assert they lift to exactly what was declared.
    pub fn write(self, root: &Path) {
        let workspace = root.join(temper::WORKSPACE_DIR);
        fs::create_dir_all(&workspace).unwrap();
        let path = workspace.join(temper::LOCK_FILENAME);
        let text = self.render();
        fs::write(&path, &text).unwrap();

        // Parsed off the rendered text rather than re-read from disk: `read_declarations`
        // increments the hoisting counters `check_cost` pins deltas on.
        let lifted = drift::parse_declarations(&path, &text)
            .unwrap_or_else(|err| panic!("the guard fixture lock must lift: {err}\n{text}"));
        assert_eq!(
            lifted, self.declarations,
            "the rendered lock must lift back to the rows it declares — a column this \
             writer spells differently from `src/` declares nothing:\n{text}"
        );

        // The mode is a declared fact, not a default: an absent or unreadable `mode` fact
        // resolves to `EnforcementMode::default()`, which is exactly the silence this
        // read-back exists to break. The match is exhaustive, so a fourth mode refuses to
        // compile until this home answers it.
        let resolved = match compose::mode_from_declarations(&lifted) {
            Ok(EnforcementMode::Note) => "note",
            Ok(EnforcementMode::Warn) => "warn",
            Ok(EnforcementMode::Block) => "block",
            Err(refusal) => {
                // Only a fixture declaring a mode outside the closed vocabulary reaches
                // here, and its whole point is that the refusal names the value.
                assert!(
                    format!("{refusal}").contains(&self.mode),
                    "the vocabulary refusal must name the declared mode `{}`, got: {refusal}",
                    self.mode
                );
                return;
            }
        };
        assert_eq!(
            resolved, self.mode,
            "the lock must declare its enforcement mode, not inherit the default:\n{text}"
        );
    }

    /// The lock's bytes: the per-kind provenance arrays first, then the declaration
    /// families under an implicit `[declaration]` table — the layout `emit`'s own
    /// roll-up writes.
    fn render(&self) -> String {
        let mut doc = DocumentMut::new();

        let mut by_kind: BTreeMap<&str, ArrayOfTables> = BTreeMap::new();
        for row in &self.members {
            let mut table = Table::new();
            table["name"] = value(row.name.clone());
            table["source_path"] = value(row.source_path.clone());
            table["source_hash"] = value(row.source_hash.clone());
            table["emit_hash"] = value(row.emit_hash.clone());
            by_kind.entry(row.kind.as_str()).or_default().push(table);
        }
        for (kind, tables) in by_kind {
            doc[kind] = Item::ArrayOfTables(tables);
        }

        let mut declaration = Table::new();
        // Implicit: only the `[[declaration.<family>]]` sub-headers render.
        declaration.set_implicit(true);
        insert_declaration_family(
            &mut declaration,
            "kind",
            self.declarations.kinds.iter().map(kind_fact_table),
        );
        insert_declaration_family(
            &mut declaration,
            "clause",
            self.declarations.clauses.iter().map(clause_table),
        );
        insert_declaration_family(
            &mut declaration,
            "assembly",
            self.declarations.assembly.iter().map(assembly_fact_table),
        );
        insert_declaration_family(
            &mut declaration,
            "registration",
            self.declarations
                .registrations
                .iter()
                .map(registration_table),
        );
        doc["declaration"] = Item::Table(declaration);

        doc.to_string()
    }
}

/// Attach one declaration family's rows under `[[declaration.<family>]]`, writing
/// nothing when the family is empty (an empty array vanishes on the toml round-trip).
fn insert_declaration_family(table: &mut Table, family: &str, rows: impl Iterator<Item = Table>) {
    let tables: ArrayOfTables = rows.collect();
    if !tables.is_empty() {
        table[family] = Item::ArrayOfTables(tables);
    }
}

/// The `fact`/`value` columns of one assembly fact — the enforcement mode's whole
/// spelling, and the one the guard's every arm rests on.
fn assembly_fact_table(row: &AssemblyFactRow) -> Table {
    let mut table = Table::new();
    table["fact"] = value(row.fact.clone());
    if let Some(cell) = &row.value {
        table["value"] = value(cell.clone());
    }
    table
}

/// The kind-fact columns a guard fixture declares. The rest of [`KindFactRow`] is not
/// rendered: a caller setting one fails [`GuardLock::write`]'s read-back loudly, which is
/// the signal to widen this home rather than hand-spell a lock beside it.
fn kind_fact_table(row: &KindFactRow) -> Table {
    let mut table = Table::new();
    table["name"] = value(row.name.clone());
    for (column, cell) in [
        ("governs_root", &row.governs_root),
        ("governs_glob", &row.governs_glob),
        ("format", &row.format),
        ("unit_shape", &row.unit_shape),
    ] {
        if let Some(cell) = cell {
            table[column] = value(cell.clone());
        }
    }
    table
}

/// The clause columns a guard fixture declares — the same partial rendering (and the
/// same read-back backstop) [`kind_fact_table`] takes.
fn clause_table(row: &ClauseRow) -> Table {
    let mut table = Table::new();
    for (column, cell) in [("label", &row.label), ("kind", &row.kind)] {
        if let Some(cell) = cell {
            table[column] = value(cell.clone());
        }
    }
    table["predicate"] = value(row.predicate.clone());
    if let Some(field) = &row.field {
        table["field"] = value(field.clone());
    }
    table["severity"] = value(row.severity.clone());
    table
}

/// A registration row's four identity-and-address columns.
fn registration_table(row: &RegistrationRow) -> Table {
    let mut table = Table::new();
    table["kind"] = value(row.kind.clone());
    table["key"] = value(row.key.clone());
    table["manifest"] = value(row.manifest.clone());
    table["key_path"] = value(row.key_path.clone());
    table
}

/// The finding a `warn`-mode run surfaced in-band, read out of the `hookSpecificOutput`
/// envelope on stdout — the one channel a hook exiting zero reaches the model's context
/// through, stderr on exit 0 landing in the debug log alone.
///
/// `event` is the `hookEventName` the envelope must stamp: `PreToolUse` for the guard,
/// `PostToolUse` for the post-edit warn. Panics unless `output` is exactly that
/// envelope, so every warn arm asserts the placement and not merely the text: a finding
/// on the wrong stream, or under the wrong event name, is a finding the session never
/// sees — an envelope naming any other event is rejected whole.
pub fn guard_in_band(output: &str, event: &str) -> String {
    let payload: serde_json::Value = serde_json::from_str(output.trim()).unwrap_or_else(|err| {
        panic!("warn must emit the hook envelope on stdout: {err}, got: {output}")
    });
    let hook = &payload["hookSpecificOutput"];
    assert_eq!(
        hook["hookEventName"], event,
        "the envelope must stamp the firing event, got: {output}"
    );
    hook["additionalContext"]
        .as_str()
        .unwrap_or_else(|| panic!("the finding must ride additionalContext, got: {output}"))
        .to_string()
}

/// Run `temper guard <root_arg>` with the working directory set to `cwd`, returning the
/// exit code and stdout-then-stderr concatenated — the same combined-stream contract
/// [`check_in`] sets, for the same reason: the guard splits its channels by mode (`warn`
/// injects in-band on stdout, `block` writes stderr), so a single-stream reader watching
/// for a finding's *absence* reads the other mode's silence as agreement.
fn run_guard_spawned_in(
    cwd: &Path,
    root_arg: &std::ffi::OsStr,
    payload: &str,
) -> (Option<i32>, String) {
    use std::io::Write;
    let mut child = Command::new(env!("CARGO_BIN_EXE_temper"))
        .arg("guard")
        .arg(root_arg)
        .current_dir(cwd)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    // Tolerate a closed pipe if the child has already exited (e.g., on corrupt locks).
    let _ = child.stdin.take().unwrap().write_all(payload.as_bytes());
    let out = child.wait_with_output().unwrap();
    let mut output = String::from_utf8_lossy(&out.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&out.stderr));
    (out.status.code(), output)
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
/// there, mirroring the real harness this stands in for. The lock row addresses
/// the filler by its `kind:name` label, the qualified shape the SDK emits.
pub fn author_satisfies(root: &Path, kind_dir: &str, name: &str, requirements: &[&str]) {
    let (source, kind) = match kind_dir {
        "skills" => (
            root.join(".claude")
                .join("skills")
                .join(name)
                .join("SKILL.md"),
            "skill",
        ),
        "rules" => (
            root.join(".claude")
                .join("rules")
                .join(format!("{name}.md")),
            "rule",
        ),
        other => panic!("unknown kind_dir {other}"),
    };
    assert!(
        source.is_file(),
        "author_satisfies: no real harness source at {}",
        source.display()
    );
    let address = format!("{kind}:{name}");
    merge_lock(root, |declarations| {
        declarations
            .satisfies
            .extend(requirements.iter().map(|r| SatisfiesRow {
                member: address.clone(),
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
    write_sibling(
        root,
        &format!(".claude/rules/{name}.md"),
        &format!("# {name}\n\nBody.\n"),
    );
}

/// A floor-clean rule document, optionally scoped by `paths` and optionally carrying a
/// `routes_to` reference field — the one home for all three spellings the suites build:
/// the scoped rule (a mention's source), the routing rule (a declared field edge the
/// graph reads), and both at once (a reference riding a field, under a scope).
///
/// `paths` `None` is the unscoped rule, which the harness loads always. `routes_to` is
/// not a floor-forbidden rule key, so the document stays floor-clean either way and the
/// only finding a routing case can produce is the graph one.
pub fn scoped_routing_rule(paths: Option<&str>, routes_to: Option<&str>) -> String {
    let scope = paths.map_or_else(String::new, |glob| format!("paths: [\"{glob}\"]\n"));
    let route = routes_to.map_or_else(String::new, |target| format!("routes_to: {target}\n"));
    format!(
        "---\n\
         {scope}{route}---\n\
         # Style\n\
         \n\
         Prefer the standards skill.\n"
    )
}

/// The unrouted spelling of [`scoped_routing_rule`] — a floor-clean rule carrying a
/// scope and no field edge, the form callers that name only a scope spell.
pub fn scoped_rule(paths: Option<&str>) -> String {
    scoped_routing_rule(paths, None)
}

/// A floor-clean skill, optionally gated by `paths` — a mention's target. `None` is
/// the ungated skill, invocable with no file read first.
pub fn gated_skill(name: &str, paths: Option<&str>) -> String {
    let gate = paths.map_or_else(String::new, |glob| format!("paths: [\"{glob}\"]\n"));
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         {gate}---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

/// Write a harness of one rule and one skill straight at their real Claude Code locus
/// — the rule under `.claude/rules/<rule>.md`, the skill under
/// `.claude/skills/<skill>/SKILL.md` — no scratch import. `check` reads built-in kind
/// members live off harness disk.
pub fn write_rule_skill_harness(
    root: &Path,
    rule_name: &str,
    rule_md: &str,
    skill_name: &str,
    skill_md: &str,
) {
    write_sibling(root, &format!(".claude/rules/{rule_name}.md"), rule_md);
    write_skill(root, skill_name, skill_md);
}

/// An `edge` assembly fact declaring one target kind — the lock row a
/// `[[kind.<from>.relationships]]` table projects. A custom kind carries its declared
/// edges only here, never on its kind-fact row, so this is the one place the gate and
/// emit learn a field is a relationship.
pub fn edge(from: &str, field: &str, to: &str) -> AssemblyFactRow {
    edge_to_set(from, field, &[to])
}

/// An `edge` assembly fact over a declared target *set* — the general row [`edge`] is
/// the one-element case of, and the column carrying the non-empty set of kinds a field
/// may resolve into.
pub fn edge_to_set(from: &str, field: &str, to: &[&str]) -> AssemblyFactRow {
    AssemblyFactRow {
        fact: "edge".to_string(),
        value: None,
        from: Some(from.to_string()),
        field: Some(field.to_string()),
        to: Some(to.iter().map(|kind| (*kind).to_string()).collect()),
    }
}

/// A `mention` declaration row — the lock family a deferred discovery-locus mention
/// rides. `emit` writes it whether or not the target is a composed value; `check` folds
/// it into the resolved-edge set and resolves it against the discovered corpus.
pub fn mention(member: &str, target: &str) -> MentionRow {
    MentionRow {
        member: member.to_string(),
        target: target.to_string(),
    }
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

/// An inert [`Features`] carrying nothing but `id` — the base every integration fixture
/// starts from, so a test spells only the columns it varies via struct update:
/// `Features { body_lines: 1, ..common::features(id) }`. The crate-side twin
/// (`test_support::features`) is the same base for in-`src` fixtures; the two spell one
/// shape, and a fourteenth column lands in two places rather than fourteen.
///
/// The rendered extents are `Some(0)`, not `None`: an `extent` clause reads the `Some`,
/// and a fixture that wants the undecidable case says so by overriding.
pub fn features(id: &str) -> Features {
    Features {
        id: id.to_string(),
        fields: BTreeMap::new(),
        body_lines: 0,
        rendered_lines: Some(0),
        rendered_chars: Some(0),
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: None,
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: Vec::new(),
        edge_placements: None,
    }
}

/// A manifest-shaped member over [`features`]: `fields` is the retained parse, exactly
/// as the `json-document` read face hands it over. Panics unless the fixture is a JSON
/// object, the only shape a manifest member's retained parse ever takes.
pub fn parsed_features(fields: serde_json::Value) -> Features {
    let serde_json::Value::Object(fields) = fields else {
        unreachable!("the fixture is a JSON object")
    };
    Features {
        fields: fields.into_iter().collect(),
        ..features("acme-tools")
    }
}

/// A tap record naming `identity` under `event`, written at `version` — a `version` below
/// `TAP_RECORD_VERSION` exercises the reader's older-version toleration. The base every
/// tap fixture starts from, so a test spells only the columns it varies via struct update:
/// `TapRecord { session: "a".to_string(), ..common::tap_record(..) }`.
///
/// `ts` is empty: `tap::append` stamps it unconditionally, so only a record serialized
/// without going through the writer ever reads the value this base gives it.
pub fn tap_record(version: u32, event: TapEvent, identity: &str) -> TapRecord {
    TapRecord {
        version,
        session: "sess".to_string(),
        event,
        identity: identity.to_string(),
        ts: String::new(),
        reason: None,
        raw_path: None,
        trigger_path: None,
        parent_path: None,
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

/// Copy every file under `from` into `to`, preserving relative layout — the walk
/// that lets a test drive a committed in-tree harness without writing into the
/// checkout. Same `walkdir` shape as [`tree_bytes`], under the same `is_file()`
/// filter, and here that filter is load-bearing: `walkdir` does not follow links
/// and a link's file type is not `file`, so a vendored
/// `node_modules/@dtmd/temper` is neither copied nor descended.
pub fn copy_tree(from: &Path, to: &Path) {
    for entry in walkdir::WalkDir::new(from) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        let dest = to.join(entry.path().strip_prefix(from).unwrap());
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(entry.path(), &dest).unwrap();
    }
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
        host: None,
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
        host: None,
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
        governs_root: Some(".claude/skills".to_string()),
        governs_glob: Some("*/SKILL.md".to_string()),
        commitment: None,
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("directory".to_string()),
        registration: registration.iter().map(|r| r.to_string()).collect(),
        templates: Vec::new(),
        content: None,
        shape: None,
        leaves: Vec::new(),
        collection_address: None,
        guidance: None,
        cite: None,
    }
}

/// The `rule` built-in kind's declaration row, parameterized by the
/// `provider`/`registration` values callers diverge on.
pub fn rule_kind_facts(provider: Option<&str>, registration: &[&str]) -> KindFactRow {
    KindFactRow {
        name: "rule".to_string(),
        provider: provider.map(str::to_string),
        governs_root: Some(".claude/rules".to_string()),
        governs_glob: Some("*.md".to_string()),
        commitment: None,
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("file".to_string()),
        registration: registration.iter().map(|r| r.to_string()).collect(),
        templates: Vec::new(),
        content: None,
        shape: None,
        leaves: Vec::new(),
        collection_address: None,
        guidance: None,
        cite: None,
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
        governs_root: Some(governs_root.to_string()),
        governs_glob: Some(governs_glob.to_string()),
        commitment: None,
        format: None,
        unit_shape: None,
        registration: Vec::new(),
        templates: Vec::new(),
        content: None,
        shape: None,
        leaves: Vec::new(),
        collection_address: None,
        guidance: None,
        cite: None,
    }
}

/// The `rule` built-in kind's locus row with **every** optional fact left at its
/// default — no format, no unit shape, no registration. Deliberately not
/// [`rule_kind_facts`], which declares the real kind's `yaml-frontmatter`/`file` facts:
/// a plain markdown, field-less projection is what these callers pin, so the emitted
/// artifact is the authored body verbatim and any extra byte would show.
pub fn bare_rule_kind_facts() -> KindFactRow {
    kind_facts("rule", ".claude/rules", "*.md")
}

/// A `hook` registration kind fact: fields-only (no body slot), keyed at
/// `settings.json`'s `hooks.<Event>` — the shape an SDK-declared registration kind's row
/// carries into the lock.
pub fn hook_kind_facts() -> KindFactRow {
    KindFactRow {
        shape: Some("fields".to_string()),
        collection_address: Some(CollectionAddressRow {
            manifest: "settings.json".to_string(),
            key_path: "hooks.<Event>".to_string(),
            entry_shape: Some("group-array(hooks;matcher)".to_string()),
        }),
        ..kind_facts("hook", ".claude", "settings.json")
    }
}

/// A layout kind governing a single lone `.md` document under `specs/`, carrying the
/// given ordered region rows — the layout host the layout suites build a member of. Its
/// kind facts ride [`kind_facts`], overriding only `content`.
pub fn layout_kind_facts(name: &str, regions: Vec<LayoutRegionRow>) -> KindFactRow {
    KindFactRow {
        content: Some(LayoutRow { regions }),
        ..kind_facts(name, "specs", &format!("{name}.md"))
    }
}

/// A `field` region row filling `slot` — an edge slot when `slot` is one of the kind's
/// edge fields, an ordinary field section otherwise.
pub fn field_region(slot: &str) -> LayoutRegionRow {
    LayoutRegionRow {
        region: "field".to_string(),
        import: None,
        slot: Some(slot.to_string()),
        member_kind: None,
        key: None,
    }
}

/// The `intent` layout's regions in wire form — a leading verbatim prose region, an
/// `intent` field section, and an `invariant` member collection. The `_row` suffix is
/// load-bearing: a suite's own `intent_layout` builds the engine
/// [`Layout`](temper::layout::Layout) this is the wire form of, and the bare name would
/// read as its twin.
pub fn intent_layout_row() -> LayoutRow {
    LayoutRow {
        regions: vec![
            LayoutRegionRow {
                region: "prose".to_string(),
                import: None,
                slot: None,
                member_kind: None,
                key: None,
            },
            field_region("intent"),
            LayoutRegionRow {
                region: "collection".to_string(),
                import: None,
                slot: None,
                member_kind: Some("invariant".to_string()),
                key: None,
            },
        ],
    }
}

/// A layout member of `kind`, its document already on disk (a source, never projected).
pub fn layout_member(kind: &str) -> PayloadMember {
    PayloadMember {
        kind: kind.to_string(),
        name: kind.to_string(),
        host: None,
        fields: Vec::new(),
        body: String::new(),
        source_path: None,
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
        verifier: None,
        prose: None,
    }
}

/// A [`ClauseRow`] naming `predicate` at `severity`, every other column at its
/// default (`kind: None`, no field, no predicate argument) — the one default-filling
/// home for the family. Call sites override the columns they diverge on via
/// struct-update: a kind-carrying floor clause sets `kind`, a predicate with an
/// argument sets its own column (`count`/`bound`/`charset`/…).
///
/// Payload-shaped: `label` is `None`, the state emit's own stamp overwrites. A test
/// that wants a *lock*-shaped row spells the label it is asserting about.
pub fn clause(predicate: &str, severity: &str) -> ClauseRow {
    ClauseRow {
        unit: None,
        label: None,
        kind: None,
        predicate: predicate.to_string(),
        field: None,
        severity: severity.to_string(),
        guidance: None,
        cite: None,
        count: None,
        target: None,
        degree: None,
        fields: None,
        gate: None,
        value_type: None,
        shape: None,
        bound: None,
        charset: None,
        keys: None,
        values: None,
        range: None,
        section: None,
        sections: None,
        guard_predicate: None,
        body: None,
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
        unit: None,
        field: field.map(str::to_string),
        count,
        target: target.map(str::to_string),
        degree,
        ..clause(predicate, "required")
    }
}

/// An engine [`temper::contract::Clause`] on `predicate` at `severity`, owned by `kind`
/// and addressed exactly as the shipped stamper addresses a lifted row: the predicate's
/// own key and target fill the label's trailing segments, so a finding's `rule` is the
/// address a real lock stamps. Engine-typed, unlike the row-shaped [`clause`]: a
/// [`ClauseRow`] is the wire form and cannot be handed to the engine.
///
/// `guidance` and `source` are `None` — a proof asserting over either builds its own
/// clause.
pub fn labelled_clause(
    kind: &str,
    severity: temper::contract::Severity,
    predicate: temper::contract::Predicate,
) -> temper::contract::Clause {
    temper::contract::Clause {
        label: temper::contract::clause_label(Some(kind), predicate.key(), predicate.target()),
        severity,
        predicate,
        guidance: None,
        source: None,
    }
}

/// An engine [`temper::contract::Contract`] over `kind` binding `clauses` — the host a
/// proof that judges predicates in isolation hangs [`labelled_clause`] results off.
pub fn clause_contract(
    kind: &str,
    clauses: Vec<temper::contract::Clause>,
) -> temper::contract::Contract {
    temper::contract::Contract {
        name: kind.to_string(),
        guidance: None,
        clauses,
    }
}

/// An engine [`temper::contract::Contract`] over `kind` binding exactly one `required`
/// clause on `predicate` — the shape a proof that judges one predicate in isolation
/// builds.
pub fn one_clause_contract(
    kind: &str,
    predicate: temper::contract::Predicate,
) -> temper::contract::Contract {
    clause_contract(
        kind,
        vec![labelled_clause(
            kind,
            temper::contract::Severity::Required,
            predicate,
        )],
    )
}

/// The findings a one-clause contract over `predicate` fires against `features` — the
/// judge every proof that isolates one predicate runs, composed once here rather than
/// per suite.
pub fn one_clause_findings(
    kind: &str,
    predicate: temper::contract::Predicate,
    features: &Features,
) -> Vec<Diagnostic> {
    temper::engine::validate(
        &one_clause_contract(kind, predicate),
        std::slice::from_ref(features),
    )
}

/// The admissibility verdict on a one-clause contract over `predicate`, at the document
/// locus — the locus every caller judges under, so it is fixed here rather than threaded.
/// A proof needing another locus calls [`temper::engine::admissibility`] directly.
pub fn one_clause_admissibility(
    kind: &str,
    predicate: temper::contract::Predicate,
) -> Vec<Diagnostic> {
    temper::engine::admissibility(
        &one_clause_contract(kind, predicate),
        &temper::engine::Locus::Document,
    )
}

/// The shipped root default's own `fresh` clause — the value `gate` threads into the
/// staleness judges, read off the embedded lock rather than hand-built so an assertion
/// measures the label and severity a real `check` reports under.
pub fn fresh_clause() -> temper::contract::Clause {
    temper::builtin::root_contract()
        .clauses
        .into_iter()
        .find(|clause| clause.predicate == temper::contract::Predicate::Fresh)
        .expect("the shipped root default binds `fresh`")
}

/// A built-in kind's shipped floor `Contract`, resolved off the embedded built-in lock
/// exactly as the shipped `check` resolves it — so an assertion measures the clauses the
/// tool ships, never a hand-built mirror of them. Panics by kind name: every kind a test
/// names here is embedded, so a missing floor is the test's own typo.
pub fn builtin_floor(kind: &str) -> temper::contract::Contract {
    temper::builtin::contract(kind)
        .unwrap_or_else(|| panic!("built-in kind `{kind}` ships an embedded floor"))
}

/// Each finding's message.
pub fn messages(diagnostics: &[Diagnostic]) -> Vec<&str> {
    diagnostics.iter().map(|d| d.message.as_str()).collect()
}

/// An engine [`temper::engine::Selection`] over `selector` binding exactly one clause
/// on `predicate` at `severity`, optionally teaching through `guidance`, resolved to
/// `members` — the host every proof that drives a *set* judge over one declared clause
/// builds. The clause is labelled through [`labelled_clause`], with the owner segment
/// derived from the selector rather than hand-spelled: [`temper::engine::Selector::Root`]
/// owns [`temper::contract::ROOT_OWNER`], `Kind(k)` owns `k`.
///
/// # Panics
///
/// On [`temper::engine::Selector::OptIn`]: a requirement's clause owner is
/// [`temper::contract::requirement_owner`], a different join, and no proof here binds a
/// clause to an opt-in selection. A proof needing that owner builds its own selection,
/// the same fence [`one_clause_admissibility`] states for its locus.
pub fn one_clause_selection<'a>(
    selector: temper::engine::Selector,
    severity: temper::contract::Severity,
    predicate: temper::contract::Predicate,
    guidance: Option<&str>,
    members: Vec<(&'a str, &'a Features)>,
) -> temper::engine::Selection<'a> {
    let owner = match &selector {
        temper::engine::Selector::Kind(kind) => kind.clone(),
        temper::engine::Selector::Root => temper::contract::ROOT_OWNER.to_string(),
        temper::engine::Selector::OptIn(requirement) => {
            panic!("a clause on requirement `{requirement}` owns a different label join")
        }
    };
    temper::engine::Selection {
        selector,
        clauses: vec![temper::contract::Clause {
            guidance: guidance.map(str::to_string),
            ..labelled_clause(&owner, severity, predicate)
        }],
        members,
    }
}

/// The root selection binding one `reachable` clause at `required` — the opt-in the
/// judge locates before it walks anything, and the declaration its findings report
/// under. `members` stays empty: the predicate ranges over `by_kind`.
pub fn root_reachable_binding() -> Vec<temper::engine::Selection<'static>> {
    vec![one_clause_selection(
        temper::engine::Selector::Root,
        temper::contract::Severity::Required,
        temper::contract::Predicate::Reachable,
        None,
        Vec::new(),
    )]
}
