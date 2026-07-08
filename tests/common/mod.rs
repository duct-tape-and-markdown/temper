//! Shared fixtures for tests that drive a real `node` subprocess against the
//! repo's own built SDK (`tests/emit.rs`, `tests/install.rs`,
//! `tests/builtin_lock_frozen.rs`) — one home for scaffolding those suites
//! were each carrying a byte-identical copy of.

use std::path::{Path, PathBuf};
use std::sync::Once;

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
        let status = std::process::Command::new("npm")
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
