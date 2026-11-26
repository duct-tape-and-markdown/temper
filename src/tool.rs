//! The `tool` kind's engine-side facts: the leaves a tool states its command under, and
//! the version a `{ package, bin }` tool runs at — its program's lockfile's, never a
//! restatement (`specs/model/authoring.md`, "The SDK"; decision 0104).
//!
//! Two lockfiles are read, both from the program's workspace (where its `package.json`
//! sits and its package manager lays out `node_modules/.bin`): npm's
//! `package-lock.json` and pnpm's `pnpm-lock.yaml`, the latter through `gray_matter`'s
//! YAML engine, the sanctioned crate's reader. Any other lockfile there, with neither of
//! those beside it, refuses loudly by name: a version temper cannot read is never guessed
//! and never silently skipped. A package the lockfile does not record — or a workspace
//! carrying no lockfile at all — is an advisory finding on the tool.
//!
//! The lockfile formats are external facts:
//! - `package-lock.json` (lockfileVersion 2/3: `packages["node_modules/<name>"]`, a
//!   `link: true` entry resolving to `packages[<resolved>]`; lockfileVersion 1:
//!   `dependencies[<name>].version`) — <https://docs.npmjs.com/cli/v10/configuring-npm/package-lock-json>
//!   (retrieved 2026-11-25).
//! - `pnpm-lock.yaml` (`importers["."].{dependencies,devDependencies,optionalDependencies}
//!   [<name>].version`, a peer-resolution suffix `(…)` after the version; lockfile v5's
//!   top-level dependency maps of bare version strings) —
//!   <https://github.com/pnpm/spec/blob/master/lockfile/9.0.md> (retrieved 2026-11-25).

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
/// The lockfiles temper reads, in the order it looks for them.
const READ_LOCKFILES: [&str; 2] = [NPM_LOCKFILE, PNPM_LOCKFILE];
/// The other package managers' lockfiles: present without a read one beside them, the
/// program's packages are held by a file temper cannot read, so it refuses by name.
const UNREAD_LOCKFILES: [&str; 4] = ["yarn.lock", "bun.lock", "bun.lockb", "npm-shrinkwrap.json"];

/// The fixed-check id a `{ package, bin }` tool the lockfile does not record reports under.
pub const UNLOCKED_RULE: &str = "tool.unlocked";

/// Why a tool's lockfile could not be read.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ToolLockError {
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
    Pnpm(Pod),
}

/// The workspace's read lockfile: its file name and its parsed document.
#[derive(Debug)]
pub struct Lockfile {
    name: &'static str,
    document: LockDocument,
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
            LockDocument::Npm(document) => npm_version(document, package),
            LockDocument::Pnpm(document) => pnpm_version(document, package),
        }
    }
}

/// Read the lockfile in `workspace`: `package-lock.json`, else `pnpm-lock.yaml`. `None`
/// when the workspace carries no lockfile at all.
///
/// # Errors
/// [`ToolLockError::Unsupported`] when only another manager's lockfile is there, naming
/// it; [`ToolLockError::Read`] / [`ToolLockError::Malformed`] when the read lockfile
/// cannot be read or parsed.
pub fn read_lockfile(workspace: &Path) -> Result<Option<Lockfile>, ToolLockError> {
    for name in READ_LOCKFILES {
        let path = workspace.join(name);
        if !path.is_file() {
            continue;
        }
        let text = std::fs::read_to_string(&path).map_err(|source| ToolLockError::Read {
            path: path.clone(),
            source,
        })?;
        let malformed = |reason: String| ToolLockError::Malformed {
            path: path.clone(),
            reason,
        };
        let document = if name == NPM_LOCKFILE {
            LockDocument::Npm(
                serde_json::from_str(&text).map_err(|error| malformed(error.to_string()))?,
            )
        } else {
            // `gray_matter` collapses a YAML parse failure to `Null`, and a lockfile is
            // never empty, so `Null` is the malformed signal.
            match YAML::parse(&text) {
                Pod::Null => return Err(malformed("not a YAML mapping".to_string())),
                document => LockDocument::Pnpm(document),
            }
        };
        return Ok(Some(Lockfile { name, document }));
    }
    if let Some(name) = UNREAD_LOCKFILES
        .iter()
        .find(|name| workspace.join(name).is_file())
    {
        return Err(ToolLockError::Unsupported {
            path: workspace.join(name),
        });
    }
    Ok(None)
}

/// `package`'s version in an npm lockfile: v2/v3's `packages` map first (a linked
/// package's entry carries no version, so it resolves through to the linked folder's),
/// then v1's `dependencies`.
fn npm_version(document: &JsonValue, package: &str) -> Option<String> {
    let string = |value: &JsonValue| value.as_str().map(str::to_string);
    if let Some(packages) = document.get("packages") {
        let entry = packages.get(format!("node_modules/{package}"))?;
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

/// `package`'s version in a pnpm lockfile: the root importer's dependency groups (v6+),
/// else the top-level groups (v5). A group entry is a mapping carrying `version`, or (v5)
/// the bare version string; a peer-resolution suffix is not part of the version.
fn pnpm_version(document: &Pod, package: &str) -> Option<String> {
    let Pod::Hash(top) = document else {
        return None;
    };
    let root = match top.get("importers") {
        Some(Pod::Hash(importers)) => importers.get("."),
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
                    "install the package with the program's package manager (npm or pnpm), so its lockfile records the version the tool runs at".to_string(),
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
            LockDocument::Pnpm(YAML::parse(text))
        };
        Lockfile { name, document }
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
