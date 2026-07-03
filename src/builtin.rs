//! The embedded built-in package std-lib.
//!
//! `temper` ships a set of first-party packages — the curated Anthropic contracts
//! for the built-in artifact kinds (`skill.anthropic`, `rule.anthropic`). Their
//! authoritative home is the `packages/<name>/PACKAGE.md` product tree at the repo
//! root (`specs/architecture/10-contracts.md`, "Decision: a package is project-authorable, not
//! vendor-privileged — and is itself a kind"): the *same* `PACKAGE.md` is authored
//! as product source and shipped **embedded** by the build. `build.rs` walks that
//! tree and generates the [`BUILTIN_PACKAGES`] table this module re-exports, so a
//! built-in name resolves from the embedded set with no on-disk configuration — a
//! consumer's assembly binds it *by name* exactly as a stranger's would, and never
//! carries a copy.
//!
//! A built-in is a [`Contract`] like any other — it *carries* one — so it is parsed
//! through the same [`Contract::parse_package`] loader a project-authored
//! `.temper/packages/<name>/PACKAGE.md` is, and validated by the same admissibility
//! check before it is trusted to gate a harness.

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::contract::{Contract, ContractError};

// The generated `pub static BUILTIN_PACKAGES: &[(&str, &str)]` — every embedded
// package as `(name, PACKAGE.md source)`, sorted by name. `build.rs` writes it into
// `$OUT_DIR` from the `packages/` tree; including it here re-exports it as
// `crate::builtin::BUILTIN_PACKAGES`.
include!(concat!(env!("OUT_DIR"), "/builtin_packages.rs"));

/// The built-in package temper ships as the floor for the `claude-code.skill` kind —
/// Anthropic's documented skill contract (`specs/architecture/10-contracts.md`, "named for its
/// source"). temper's own **published** binding names the *qualified* kind identity
/// (`specs/architecture/15-kinds.md`, "a published package binds a qualified kind name"): publication
/// is where the consumer's assembly is unknowable, so a bare binding would be a latent
/// collision. The package's own name stays short — the kind axis it binds is what
/// qualifies, resolved through the embedded set (`crate::builtin_kind::qualified`).
pub const SKILL_PACKAGE: &str = "skill.anthropic";

/// The built-in package temper ships as the floor for the `claude-code.rule` kind —
/// Anthropic's documented rule contract, bound to the qualified kind identity exactly
/// as [`SKILL_PACKAGE`] is (`specs/architecture/15-kinds.md`). Renamed from the bare `rule`
/// (`specs/architecture/10-contracts.md`, "named for its source": the clauses are equally
/// Anthropic-sourced).
pub const RULE_PACKAGE: &str = "rule.anthropic";

/// The embedded `PACKAGE.md` source for a built-in package, or `None` if no package
/// of that name is embedded.
#[must_use]
pub fn source(name: &str) -> Option<&'static str> {
    BUILTIN_PACKAGES
        .iter()
        .find(|(embedded, _)| *embedded == name)
        .map(|(_, src)| *src)
}

/// Parse the named built-in package into its [`Contract`], or `None` if no package
/// of that name is embedded. The synthetic `packages/<name>/PACKAGE.md` path gives
/// [`Contract::parse_package`] the right parent-directory stem, so the parsed
/// contract's display name derives to `<name>` exactly as the on-disk loader's does.
pub fn contract(name: &str) -> Result<Option<Contract>, ContractError> {
    match source(name) {
        None => Ok(None),
        Some(src) => {
            let path = PathBuf::from("packages").join(name).join("PACKAGE.md");
            Contract::parse_package(src, &path).map(Some)
        }
    }
}

/// Parse every embedded built-in package into a `name → Contract` map — the
/// built-in set a by-name package binding resolves against before a project's own
/// `.temper/packages/` (`specs/architecture/20-surface.md`, "Decision: package binding is by
/// artifact kind"; the resolution order [`crate::compose::PackageResolver`] runs).
pub fn contracts() -> Result<BTreeMap<String, Contract>, ContractError> {
    let mut map = BTreeMap::new();
    for (name, src) in BUILTIN_PACKAGES {
        let path = PathBuf::from("packages").join(name).join("PACKAGE.md");
        map.insert((*name).to_string(), Contract::parse_package(src, &path)?);
    }
    Ok(map)
}
