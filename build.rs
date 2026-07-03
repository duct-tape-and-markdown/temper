//! Build script — embed the built-in std-lib (packages and kinds) into the binary.
//!
//! `temper`'s built-in std-lib lives in first-class product trees at the repo root:
//! `packages/<name>/PACKAGE.md` (the require-side contracts) and
//! `kinds/<name>/KIND.md` (the read-side kind definitions) — *product* source,
//! human-maintained from cited upstream sources on the product's release cadence
//! (`specs/10-contracts.md`, "a package is project-authorable, not vendor-
//! privileged"; `specs/15-kinds.md`, "A built-in kind is an adapter"). This script
//! walks both trees at compile time and generates `builtin_packages.rs` /
//! `builtin_kinds.rs` that embed each document verbatim via `include_str!`, so the
//! shipped binary carries the std-lib with no on-disk configuration and the *same*
//! document is authored as product source and shipped embedded — one artifact, no
//! mirror.
//!
//! Each embed is auto-detected: every directory under the tree that carries the
//! marker file is embedded, keyed by the directory name (`skill.anthropic`,
//! `rule.anthropic`; `skill`, `rule`). Adding a built-in needs no edit here or to
//! `Cargo.toml` — drop a `<name>/<MARKER>` and it ships. Keeping the embed in the
//! build script (not a `Cargo.toml` `include`) keeps it off the manifest so it
//! never collides with unrelated packaging metadata.
//!
//! The *kinds* walk additionally tolerates the nested provider layout
//! (`specs/15-kinds.md`, "Decision: kind identity carries a provider axis"): a
//! provider-level directory whose subdirectories carry `KIND.md` keys each
//! `<provider>.<name>` qualified, beside a flat `<name>/KIND.md` keyed bare. It is
//! the build-side predecessor to the human file-move — dormant while the kinds tree
//! is flat, so `kinds/claude-code/skill/KIND.md` embeds rather than being silently
//! skipped once the move lands. The packages walk stays flat: a package name is its
//! own namespace.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is always set by cargo");
    let manifest_dir = Path::new(&manifest_dir);
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is always set by cargo");
    let out_dir = Path::new(&out_dir);

    embed(
        &manifest_dir.join("packages"),
        "PACKAGE.md",
        "BUILTIN_PACKAGES",
        "package",
        false,
        &out_dir.join("builtin_packages.rs"),
    );
    embed(
        &manifest_dir.join("kinds"),
        "KIND.md",
        "BUILTIN_KINDS",
        "kind",
        true,
        &out_dir.join("builtin_kinds.rs"),
    );
}

/// Walk a `<tree>/<name>/<marker>` product tree and generate a `dest` file
/// declaring `pub static <table>: &[(&str, &str)]` — `(key, marker source)` per
/// embedded document, sorted by key. Auto-detected: any directory carrying the
/// marker file is embedded, so adding a built-in needs no edit here.
///
/// When `nested` (the kinds walk only), a directory that carries no marker directly
/// is treated as a provider-level directory: its marker-carrying subdirectories embed
/// keyed `<provider>.<name>` qualified (`specs/15-kinds.md`). A directory carrying the
/// marker itself always keys bare, so the flat and nested layouts coexist.
fn embed(tree: &Path, marker: &str, table: &str, noun: &str, nested: bool, dest: &Path) {
    // Re-run when a built-in is added, removed, or edited, so the embedded std-lib
    // never drifts from the authored source.
    println!("cargo:rerun-if-changed={}", tree.display());

    let mut entries: Vec<(String, PathBuf)> = Vec::new();
    if let Ok(listing) = fs::read_dir(tree) {
        for entry in listing {
            let entry = entry.expect("reading a std-lib tree dir entry");
            let dir = entry.path();
            if !dir.is_dir() {
                continue;
            }
            let name = dir_name(&dir);
            let doc = dir.join(marker);
            if doc.is_file() {
                // A directory directly carrying the marker keys bare — the flat layout
                // both trees share; today's kinds tree is entirely flat.
                println!("cargo:rerun-if-changed={}", doc.display());
                entries.push((name, doc));
            } else if nested {
                // No marker here: a provider-level directory. Descend one level and key
                // each carrier `<provider>.<name>`. Dormant until the human file-move
                // populates `kinds/<provider>/<name>/`.
                collect_qualified(&dir, &name, marker, &mut entries);
            }
        }
    }
    // Sorted so the generated table (and every diagnostic ranging over it) is
    // stable across machines and runs.
    entries.sort();

    // Emit `<table>: &[(&str, &str)]` — each entry the name paired with its marker
    // source, embedded byte-for-byte. Absolute paths so the `include_str!` resolves
    // regardless of where the generated file is included.
    let mut code = format!(
        "/// Every embedded built-in {noun}: `(key, {marker} source)`, sorted by\n\
         /// key. Generated by `build.rs` from the `{tree}` product tree.\n\
         pub static {table}: &[(&str, &str)] = &[\n",
        tree = tree
            .file_name()
            .and_then(|name| name.to_str())
            .expect("a std-lib tree name is valid UTF-8"),
    );
    for (name, path) in &entries {
        let path = path
            .to_str()
            .expect("a std-lib path under the manifest dir is valid UTF-8");
        writeln!(code, "    ({name:?}, include_str!({path:?})),").expect("writing to a String");
    }
    code.push_str("];\n");

    fs::write(dest, code).expect("writing a generated std-lib embed");
}

/// Collect the marker-carrying subdirectories of a provider-level directory, keying
/// each `<provider>.<name>` (`specs/15-kinds.md`, "Placement mirrors identity"). Only
/// the kinds walk descends here; a package name is its own namespace, so the packages
/// walk never calls this.
fn collect_qualified(
    provider_dir: &Path,
    provider: &str,
    marker: &str,
    entries: &mut Vec<(String, PathBuf)>,
) {
    let Ok(listing) = fs::read_dir(provider_dir) else {
        return;
    };
    for entry in listing {
        let entry = entry.expect("reading a provider-level std-lib dir entry");
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }
        let doc = dir.join(marker);
        if !doc.is_file() {
            continue;
        }
        let name = dir_name(&dir);
        println!("cargo:rerun-if-changed={}", doc.display());
        entries.push((format!("{provider}.{name}"), doc));
    }
}

/// The UTF-8 file name of a std-lib directory — the bare kind/package name, or a
/// provider name in the nested kinds layout.
fn dir_name(dir: &Path) -> String {
    dir.file_name()
        .and_then(|stem| stem.to_str())
        .expect("a std-lib directory name is valid UTF-8")
        .to_string()
}
