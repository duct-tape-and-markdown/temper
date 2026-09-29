//! Build script — stamps this build's commit for the long `--version` form.
//!
//! The built-in std-lib (packages and kinds) is authored directly as Rust data
//! (`src/builtin.rs`, `src/builtin_kind.rs`), so there is no tree to walk and embed
//! here. The script's one job is provenance.
//!
//! The engine's version string is the release tag's, identical in every build of that
//! tag and in every artifact it stamps, with build provenance riding the long
//! `--version` form alone. So the commit lands in an env var that `src/main.rs` reads
//! with `option_env!` for that form only — never in `CARGO_PKG_VERSION`, where a
//! suffix would make a tarball build and a git build of one tag disagree.
//!
//! Every step is best-effort, because a crates.io tarball carries no git metadata: a
//! missing `git`, a non-repository tree, or any failed call emits nothing, the long
//! form degrades to the short one, and the build still succeeds.

use std::path::Path;
use std::process::Command;

fn main() {
    let Some(commit) = git(&["rev-parse", "--short=12", "HEAD"]) else {
        // No git metadata — a released tarball. Emit nothing, and declare no inputs,
        // so Cargo keeps its default "re-run when a package file changes".
        return;
    };

    // Re-run when HEAD moves; without these the stamp is computed once and then goes
    // silently stale for the rest of the target dir's life. `git rev-parse --git-path`
    // resolves the per-worktree and common git dirs, so these are the real files even
    // in a linked worktree, where `.git` is a file rather than a directory.
    for input in [
        // Attached, this holds the ref name and changes only on checkout; detached, it
        // holds the commit itself and changes on every move.
        git(&["rev-parse", "--git-path", "HEAD"]),
        // The loose ref HEAD points at, when it is attached and unpacked.
        git(&["symbolic-ref", "-q", "HEAD"]).and_then(|r| git(&["rev-parse", "--git-path", &r])),
        // ...and the packed file, when that same ref lives there instead.
        git(&["rev-parse", "--git-path", "packed-refs"]),
    ]
    .into_iter()
    .flatten()
    {
        // A declared path that does not exist re-runs the script on every build, so
        // only the ones actually on disk are worth declaring.
        if Path::new(&input).exists() {
            println!("cargo::rerun-if-changed={input}");
        }
    }

    println!("cargo::rustc-env=TEMPER_BUILD_COMMIT={commit}");
}

/// Run `git` with `args`, returning its trimmed stdout — or `None` if the binary is
/// missing, the call failed, or the output was empty or not UTF-8.
fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?.trim().to_string();
    (!text.is_empty()).then_some(text)
}
