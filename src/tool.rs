//! The `tool` kind's engine-side facts: the leaves a tool states its command under, and
//! the version a `{ package, bin }` tool runs at — its program's lockfile's, never a
//! restatement.
//!
//! The program's `package.json` `packageManager` field names its manager, read and never
//! guessed: npm's `package-lock.json` or pnpm's `pnpm-lock.yaml` (through `gray_matter`'s
//! YAML engine) is read from the program's own directory, else the workspace root its
//! manager resolves — an ancestor no workspace of the program's names is never read. A
//! field that is absent, or names another manager, refuses by name. A package the
//! lockfile does not record — or no lockfile at all — is an advisory finding on the tool.
//!
//! The formats are external facts:
//! - `packageManager` (`<name>@<version>`, an optional `+<hash>` suffix) —
//!   <https://github.com/nodejs/corepack/blob/main/README.md> (retrieved 2026-11-26).
//! - `package-lock.json` (lockfileVersion 2/3: `packages["node_modules/<name>"]`, a
//!   `link: true` entry resolving to `packages[<resolved>]`; lockfileVersion 1:
//!   `dependencies[<name>].version`) — <https://docs.npmjs.com/cli/v10/configuring-npm/package-lock-json>
//!   (retrieved 2026-11-25); a workspace project's own `<path>/node_modules/<name>`
//!   before the hoisted root one.
//! - `pnpm-lock.yaml` (`importers[<path> or "."].{dependencies,devDependencies,optionalDependencies}
//!   [<name>].version`, a peer-resolution suffix `(…)` after the version; lockfile v5's
//!   top-level dependency maps of bare version strings) —
//!   <https://github.com/pnpm/spec/blob/master/lockfile/9.0.md> (retrieved 2026-11-25);
//!   a leading boot (env) document before the project's, each opened by a `---` line —
//!   <https://pnpm.io/lockfile> (retrieved 2026-11-25).

use std::path::{Path, PathBuf};

use gray_matter::Pod;
use gray_matter::engine::{Engine, YAML};
use serde_json::Value as JsonValue;

use crate::check::Diagnostic;
use crate::contract;
use crate::drift::NestedMemberRow;

/// The kind a tool is a member of.
pub const TOOL_KIND: &str = "tool";
/// The leaf a `{ file }` tool states its script's harness-relative path under.
pub const TOOL_FILE_LEAF: &str = "file";
/// The leaf a `{ package, bin }` tool states its package under.
pub const TOOL_PACKAGE_LEAF: &str = "package";
/// The leaf a `{ package, bin }` tool states its bin under.
pub const TOOL_BIN_LEAF: &str = "bin";
/// The leaf a `{ system }` tool states the PATH name it claims under.
pub const TOOL_SYSTEM_LEAF: &str = "system";

/// npm's lockfile.
pub const NPM_LOCKFILE: &str = "package-lock.json";
/// pnpm's lockfile.
pub const PNPM_LOCKFILE: &str = "pnpm-lock.yaml";
/// npm's publishable lockfile, which npm reads before `package-lock.json`: beside an npm
/// program it holds the packages in a file temper does not read, so it refuses by name.
const NPM_SHRINKWRAP: &str = "npm-shrinkwrap.json";

/// The `package.json` field declaring the program's package manager.
pub const PACKAGE_MANAGER_FIELD: &str = "packageManager";

/// A package manager the program's `packageManager` field names, and temper reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    /// npm, locking by `package-lock.json`.
    Npm,
    /// pnpm, locking by `pnpm-lock.yaml`.
    Pnpm,
}

impl PackageManager {
    /// The manager's name, as the field spells it before its `@`.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Pnpm => "pnpm",
        }
    }

    /// The lockfile the manager writes.
    #[must_use]
    pub fn lockfile(self) -> &'static str {
        match self {
            Self::Npm => NPM_LOCKFILE,
            Self::Pnpm => PNPM_LOCKFILE,
        }
    }

    /// The executable to spawn: `Command` does not consult `PATHEXT`, and Windows ships
    /// both managers as `.cmd` shims.
    #[must_use]
    pub fn program(self) -> &'static str {
        match (self, cfg!(windows)) {
            (Self::Npm, false) => "npm",
            (Self::Npm, true) => "npm.cmd",
            (Self::Pnpm, false) => "pnpm",
            (Self::Pnpm, true) => "pnpm.cmd",
        }
    }
}

/// The manager the `package.json` in `workspace` declares in its `packageManager` field.
///
/// # Errors
/// [`ToolLockError::NoManager`] when the manifest is absent or declares no field;
/// [`ToolLockError::UnsupportedManager`] when the field names a manager temper does not
/// read; [`ToolLockError::Read`] / [`ToolLockError::Malformed`] when the manifest cannot
/// be read or parsed.
pub fn declared_manager(workspace: &Path) -> Result<PackageManager, ToolLockError> {
    let path = workspace.join("package.json");
    let text = match crate::bytes::open_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(ToolLockError::NoManager { path });
        }
        Err(source) => return Err(ToolLockError::Read { path, source }),
    };
    let manifest: JsonValue =
        serde_json::from_str(&text).map_err(|error| ToolLockError::Malformed {
            path: path.clone(),
            reason: error.to_string(),
        })?;
    let Some(declared) = manifest
        .get(PACKAGE_MANAGER_FIELD)
        .and_then(JsonValue::as_str)
    else {
        return Err(ToolLockError::NoManager { path });
    };
    match declared.split('@').next().unwrap_or_default() {
        "npm" => Ok(PackageManager::Npm),
        "pnpm" => Ok(PackageManager::Pnpm),
        _ => Err(ToolLockError::UnsupportedManager {
            path,
            declared: declared.to_string(),
        }),
    }
}

/// The fixed-check id a `{ package, bin }` tool the lockfile does not record reports under.
pub const UNLOCKED_RULE: &str = "tool.unlocked";

/// Why a tool's lockfile could not be read.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ToolLockError {
    /// The program's `package.json` declares no `packageManager`.
    #[error(
        "`{}` declares no `packageManager`, so the program's package manager is unknown — temper never guesses it",
        path.display()
    )]
    #[diagnostic(
        code(temper::tool::no_package_manager),
        help(
            "add `\"packageManager\": \"npm@<version>\"` (or `pnpm@<version>`) to it, naming the manager that installs the program's packages"
        )
    )]
    NoManager {
        /// The program's `package.json`.
        path: PathBuf,
    },
    /// The program's `package.json` names a manager temper does not read.
    #[error(
        "`{}` declares the package manager `{declared}`, which temper does not read — a `{{ package, bin }}` tool's version is read from npm's or pnpm's lockfile",
        path.display()
    )]
    #[diagnostic(
        code(temper::tool::unsupported_package_manager),
        help(
            "install the program's packages with npm or pnpm and declare it in `packageManager`, or declare the tool as a `{{ file }}` or `{{ system }}` tool"
        )
    )]
    UnsupportedManager {
        /// The program's `package.json`.
        path: PathBuf,
        /// The field's value.
        declared: String,
    },
    /// The workspace's packages are held by a lockfile temper does not read.
    #[error(
        "the program's packages are locked by `{}`, which temper does not read — a `{{ package, bin }}` tool's version is read from `package-lock.json` or `pnpm-lock.yaml`",
        path.display()
    )]
    #[diagnostic(
        code(temper::tool::unsupported_lockfile),
        help(
            "install the program's packages with npm or pnpm, or declare the tool as a `{{ file }}` or `{{ system }}` tool"
        )
    )]
    Unsupported {
        /// The lockfile found.
        path: PathBuf,
    },
    /// A lockfile exists but cannot be read.
    #[error("failed to read the lockfile `{}`", path.display())]
    #[diagnostic(code(temper::tool::lockfile_read))]
    Read {
        /// The lockfile.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// A lockfile exists but does not parse as its format.
    #[error("the lockfile `{}` does not parse: {reason}", path.display())]
    #[diagnostic(code(temper::tool::lockfile_malformed))]
    Malformed {
        /// The lockfile.
        path: PathBuf,
        /// What failed.
        reason: String,
    },
}

/// A composed `{ package, bin }` tool, read off its lock row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageTool<'a> {
    /// The tool's own address, `root/tool/<key>`.
    pub address: String,
    /// The package whose bin it runs.
    pub package: &'a str,
    /// The bin it runs.
    pub bin: &'a str,
}

/// Every root-hosted `{ package, bin }` tool among `rows`, in row order.
pub fn package_tools(rows: &[NestedMemberRow]) -> Vec<PackageTool<'_>> {
    rows.iter()
        .filter(|row| row.kind == TOOL_KIND && row.host == contract::ROOT_OWNER)
        .filter_map(|row| {
            let leaf = |name: &str| {
                row.leaves
                    .get(name)
                    .map(String::as_str)
                    .filter(|value| !value.is_empty())
            };
            Some(PackageTool {
                address: crate::member_address::nested_address(&row.host, &row.kind, &row.key),
                package: leaf(TOOL_PACKAGE_LEAF)?,
                bin: leaf(TOOL_BIN_LEAF)?,
            })
        })
        .collect()
}

/// A parsed lockfile, as its format holds it.
#[derive(Debug)]
enum LockDocument {
    Npm(JsonValue),
    /// Every YAML document in the file, in order.
    Pnpm(Vec<Pod>),
}

/// The program's read lockfile: its file name, its parsed document, and the program's
/// path relative to the lockfile's directory.
#[derive(Debug)]
pub struct Lockfile {
    name: &'static str,
    document: LockDocument,
    /// `/`-separated; empty when the lockfile sits in the program's own workspace.
    importer: String,
}

impl Lockfile {
    /// The lockfile's file name (`package-lock.json` or `pnpm-lock.yaml`).
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The version this lockfile records for the program's direct dependency `package`,
    /// or `None` when it records none.
    #[must_use]
    pub fn version(&self, package: &str) -> Option<String> {
        match &self.document {
            LockDocument::Npm(document) => npm_version(document, &self.importer, package),
            LockDocument::Pnpm(documents) => documents
                .iter()
                .find_map(|document| pnpm_version(document, &self.importer, package)),
        }
    }
}

/// Read the lockfile of the program in `workspace` — the one its declared manager writes:
/// its own directory's, else the one at the workspace root the manager resolves for it —
/// the nearest ancestor that declares a workspace, read only when that declaration
/// includes the program. `None` when the program carries no lockfile and no ancestor's
/// workspace includes it: a lockfile in an ancestor that is no workspace root of the
/// program's locks some other project, and is never read.
///
/// # Errors
/// [`declared_manager`]'s refusals; [`ToolLockError::Unsupported`] when an npm program's
/// directory holds `npm-shrinkwrap.json`; [`ToolLockError::Read`] /
/// [`ToolLockError::Malformed`] when the read lockfile cannot be read or parsed.
pub fn read_lockfile(workspace: &Path) -> Result<Option<Lockfile>, ToolLockError> {
    // Absolute so a relative workspace still walks its real ancestors; not canonical, so
    // the importer path is the one the author's tree spells.
    let workspace = std::path::absolute(workspace).unwrap_or_else(|_| workspace.to_path_buf());
    let manager = declared_manager(&workspace)?;
    if let Some(lockfile) = read_lockfile_in(&workspace, &workspace, manager)? {
        return Ok(Some(lockfile));
    }
    for dir in workspace.ancestors().skip(1) {
        let Some(globs) = workspace_globs(dir) else {
            continue;
        };
        return if includes(&globs, &relative_path(dir, &workspace)) {
            read_lockfile_in(dir, &workspace, manager)
        } else {
            Ok(None)
        };
    }
    Ok(None)
}

/// pnpm's workspace declaration, beside the root's lockfile.
const PNPM_WORKSPACE: &str = "pnpm-workspace.yaml";

/// The workspace globs `dir` declares, when it declares a workspace at all: pnpm's
/// `pnpm-workspace.yaml` `packages` (absent, the root alone), then `package.json`'s
/// `workspaces` array — the field npm reads, and pnpm does not. Both lists join: a
/// directory declaring both is the root of either manager's workspace.
///
/// External facts: "A workspace must have a `pnpm-workspace.yaml` file in its root"; "If
/// the `packages` field is omitted, only the root package is included"; pnpm "reads the
/// workspace from `pnpm-workspace.yaml`, not from the `workspaces` field of the root
/// `package.json`" — <https://pnpm.io/workspaces>, <https://pnpm.io/pnpm-workspace_yaml>
/// (retrieved 2026-11-26). npm's `workspaces` is an array of folder paths or globs, and
/// from a directory inside a workspace "`prefix` is set to the root workspace" —
/// <https://docs.npmjs.com/cli/v10/using-npm/workspaces> (retrieved 2026-11-26).
///
/// A declaration that cannot be read or parsed declares nothing: an ancestor's broken
/// manifest is no workspace of the program's, so it bounds nothing and refuses nothing.
fn workspace_globs(dir: &Path) -> Option<Vec<String>> {
    let mut globs = None::<Vec<String>>;
    if let Ok(text) = crate::bytes::open_to_string(dir.join(PNPM_WORKSPACE)) {
        let packages = match YAML::parse(&text) {
            Pod::Hash(top) => match top.get("packages") {
                Some(Pod::Array(items)) => items
                    .iter()
                    .filter_map(|item| match item {
                        Pod::String(glob) => Some(glob.clone()),
                        _ => None,
                    })
                    .collect(),
                _ => Vec::new(),
            },
            _ => Vec::new(),
        };
        globs = Some(packages);
    }
    let manifest = crate::bytes::open_to_string(dir.join("package.json"))
        .ok()
        .and_then(|text| serde_json::from_str::<JsonValue>(&text).ok());
    if let Some(JsonValue::Array(items)) = manifest.as_ref().and_then(|m| m.get("workspaces")) {
        globs.get_or_insert_with(Vec::new).extend(
            items
                .iter()
                .filter_map(JsonValue::as_str)
                .map(str::to_string),
        );
    }
    globs
}

/// Whether the workspace `globs` include the project at `path` (`/`-separated, relative to
/// the root): a positive glob matches it and no `!`-negated one does.
fn includes(globs: &[String], path: &str) -> bool {
    let matches = |glob: &str| {
        let glob = glob.trim_start_matches("./").trim_end_matches('/');
        crate::glob::compile_glob(glob).is_some_and(|matcher| matcher.is_match(path))
    };
    let (negated, positive): (Vec<&String>, Vec<&String>) =
        globs.iter().partition(|glob| glob.starts_with('!'));
    positive.iter().any(|glob| matches(glob)) && !negated.iter().any(|glob| matches(&glob[1..]))
}

/// `workspace`'s path below `dir`, `/`-separated; empty when they are the same directory.
fn relative_path(dir: &Path, workspace: &Path) -> String {
    workspace
        .strip_prefix(dir)
        .unwrap_or(Path::new(""))
        .components()
        .map(|part| part.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// `path`'s text, a failure to read it the refusal naming it.
fn read_text(path: &Path) -> Result<String, ToolLockError> {
    crate::bytes::open_to_string(path).map_err(|source| ToolLockError::Read {
        path: path.to_path_buf(),
        source,
    })
}

/// `manager`'s lockfile in `dir` alone, for the program in `workspace` (`dir` or below it).
fn read_lockfile_in(
    dir: &Path,
    workspace: &Path,
    manager: PackageManager,
) -> Result<Option<Lockfile>, ToolLockError> {
    if manager == PackageManager::Npm && dir.join(NPM_SHRINKWRAP).is_file() {
        return Err(ToolLockError::Unsupported {
            path: dir.join(NPM_SHRINKWRAP),
        });
    }
    let name = manager.lockfile();
    let path = dir.join(name);
    if !path.is_file() {
        return Ok(None);
    }
    let text = read_text(&path)?;
    let malformed = |reason: String| ToolLockError::Malformed {
        path: path.clone(),
        reason,
    };
    let document = match manager {
        PackageManager::Npm => LockDocument::Npm(
            serde_json::from_str(&text).map_err(|error| malformed(error.to_string()))?,
        ),
        PackageManager::Pnpm => LockDocument::Pnpm(pnpm_documents(&text).map_err(malformed)?),
    };
    Ok(Some(Lockfile {
        name,
        document,
        importer: relative_path(dir, workspace),
    }))
}

/// Every document of a pnpm lockfile. `gray_matter`'s engine yields only a text's first
/// document, so the text is split at its `---` marker lines — pnpm's block-style output
/// indents every nested line, so a bare `---` line is only ever a document boundary.
fn pnpm_documents(text: &str) -> Result<Vec<Pod>, String> {
    let mut chunks = vec![String::new()];
    for line in text.lines() {
        if line.trim_end() == "---" {
            chunks.push(String::new());
        } else if let Some(chunk) = chunks.last_mut() {
            chunk.push_str(line);
            chunk.push('\n');
        }
    }
    let documents: Vec<Pod> = chunks
        .iter()
        .filter(|chunk| !chunk.trim().is_empty())
        // `gray_matter` collapses a YAML parse failure to `Null`, and a lockfile document
        // is never empty, so `Null` is the malformed signal.
        .map(|chunk| match YAML::parse(chunk) {
            Pod::Null => Err("not a YAML mapping".to_string()),
            document => Ok(document),
        })
        .collect::<Result<_, _>>()?;
    if documents.is_empty() {
        return Err("not a YAML mapping".to_string());
    }
    Ok(documents)
}

/// `package`'s version in an npm lockfile: v2/v3's `packages` map first — the importer's
/// own `node_modules` before the hoisted root one, and a linked package's entry carries
/// no version, so it resolves through to the linked folder's — then v1's `dependencies`.
fn npm_version(document: &JsonValue, importer: &str, package: &str) -> Option<String> {
    let string = |value: &JsonValue| value.as_str().map(str::to_string);
    if let Some(packages) = document.get("packages") {
        let hoisted = format!("node_modules/{package}");
        let entry = (!importer.is_empty())
            .then(|| packages.get(format!("{importer}/{hoisted}")))
            .flatten()
            .or_else(|| packages.get(&hoisted))?;
        if let Some(version) = entry.get("version").and_then(string) {
            return Some(version);
        }
        let resolved = entry
            .get("link")
            .and_then(JsonValue::as_bool)
            .filter(|link| *link)
            .and_then(|_| entry.get("resolved"))
            .and_then(JsonValue::as_str)?;
        return packages.get(resolved)?.get("version").and_then(string);
    }
    document
        .get("dependencies")?
        .get(package)?
        .get("version")
        .and_then(string)
}

/// `package`'s version in one pnpm lockfile document: the importer's dependency groups (v6+),
/// else the top-level groups (v5). A group entry is a mapping carrying `version`, or (v5)
/// the bare version string; a peer-resolution suffix is not part of the version.
fn pnpm_version(document: &Pod, importer: &str, package: &str) -> Option<String> {
    let Pod::Hash(top) = document else {
        return None;
    };
    let root = match top.get("importers") {
        Some(Pod::Hash(importers)) => {
            importers.get(if importer.is_empty() { "." } else { importer })
        }
        _ => Some(document),
    };
    let Some(Pod::Hash(root)) = root else {
        return None;
    };
    ["dependencies", "devDependencies", "optionalDependencies"]
        .iter()
        .find_map(|group| {
            let Some(Pod::Hash(group)) = root.get(*group) else {
                return None;
            };
            let version = match group.get(package)? {
                Pod::String(version) => version,
                Pod::Hash(entry) => match entry.get("version")? {
                    Pod::String(version) => version,
                    _ => return None,
                },
                _ => return None,
            };
            let version = version.split('(').next().unwrap_or(version);
            Some(version.to_string())
        })
}

/// The advisory finding for every `{ package, bin }` tool whose package the workspace's
/// lockfile does not record — or every one, where the workspace carries no lockfile.
///
/// # Errors
/// Propagates [`read_lockfile`]'s refusals: another manager's lockfile is never guessed.
pub fn unlocked_findings(
    rows: &[NestedMemberRow],
    workspace: &Path,
) -> Result<Vec<Diagnostic>, ToolLockError> {
    let tools = package_tools(rows);
    if tools.is_empty() {
        return Ok(Vec::new());
    }
    let lockfile = read_lockfile(workspace)?;
    Ok(tools
        .into_iter()
        .filter_map(|tool| {
            let message = match &lockfile {
                Some(lock) if lock.version(tool.package).is_some() => return None,
                Some(lock) => format!(
                    "its package `{}` is not in the program's `{}`, so its bin `{}` runs at no locked version",
                    tool.package,
                    lock.name(),
                    tool.bin
                ),
                None => format!(
                    "the program carries no lockfile, so its package `{}` (bin `{}`) runs at no locked version",
                    tool.package, tool.bin
                ),
            };
            Some(
                Diagnostic::warn(UNLOCKED_RULE, tool.address, message).with_guidance(Some(
                    "install the package with the program's package manager, so its lockfile records the version the tool runs at".to_string(),
                )),
            )
        })
        .collect())
}

/// `explain`'s version strand for the tool at `address`, empty when it is no composed
/// `{ package, bin }` tool.
///
/// # Errors
/// Propagates [`read_lockfile`]'s refusals.
pub fn version_strand(
    rows: &[NestedMemberRow],
    workspace: &Path,
    address: &str,
) -> Result<String, ToolLockError> {
    let Some(tool) = package_tools(rows)
        .into_iter()
        .find(|tool| tool.address == address)
    else {
        return Ok(String::new());
    };
    let lockfile = read_lockfile(workspace)?;
    Ok(match &lockfile {
        Some(lock) => match lock.version(tool.package) {
            Some(version) => format!(
                "Version: `{}` {version}, locked by the program's `{}` — the bin `{}` runs at it.\n",
                tool.package,
                lock.name(),
                tool.bin
            ),
            None => format!(
                "Version: none — `{}` is not in the program's `{}`.\n",
                tool.package,
                lock.name()
            ),
        },
        None => format!(
            "Version: none — the program carries no lockfile recording `{}`.\n",
            tool.package
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lock(name: &'static str, text: &str) -> Lockfile {
        let document = if name == NPM_LOCKFILE {
            LockDocument::Npm(serde_json::from_str(text).unwrap())
        } else {
            LockDocument::Pnpm(pnpm_documents(text).unwrap())
        };
        Lockfile {
            name,
            document,
            importer: String::new(),
        }
    }

    #[test]
    fn npm_reads_v3_packages_v1_dependencies_and_follows_a_link() {
        let v3 = lock(
            NPM_LOCKFILE,
            r#"{ "packages": {
                "node_modules/typescript": { "version": "5.9.3" },
                "node_modules/@dtmd/temper": { "resolved": "../sdk", "link": true },
                "../sdk": { "name": "@dtmd/temper", "version": "0.0.25" } } }"#,
        );
        assert_eq!(v3.version("typescript").as_deref(), Some("5.9.3"));
        assert_eq!(v3.version("@dtmd/temper").as_deref(), Some("0.0.25"));
        assert_eq!(v3.version("jq"), None);
        let v1 = lock(
            NPM_LOCKFILE,
            r#"{ "dependencies": { "typescript": { "version": "4.0.0" } } }"#,
        );
        assert_eq!(v1.version("typescript").as_deref(), Some("4.0.0"));
    }

    #[test]
    fn pnpm_reads_the_root_importer_and_v5_groups_dropping_peer_suffixes() {
        let v9 = lock(
            PNPM_LOCKFILE,
            "lockfileVersion: '9.0'\nimporters:\n\n  .:\n    devDependencies:\n      '@dtmd/temper':\n        specifier: ^0.0.25\n        version: 0.0.25(typescript@5.9.3)\n",
        );
        assert_eq!(v9.version("@dtmd/temper").as_deref(), Some("0.0.25"));
        assert_eq!(v9.version("typescript"), None);
        let v5 = lock(
            PNPM_LOCKFILE,
            "lockfileVersion: 5.4\ndependencies:\n  typescript: 4.9.5\n",
        );
        assert_eq!(v5.version("typescript").as_deref(), Some("4.9.5"));
    }
}
