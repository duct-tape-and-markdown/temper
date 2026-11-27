//! The engine stamp a lock records: the release version, marked when the build is not
//! that release. Shared verbatim with `build.rs`, which computes the stamp this binary
//! carries, so the build script and the tests read one spelling.

/// The mark an unreleased build appends to the release version it follows. Semver
/// build metadata, so the stamp still names the release line it was built on.
pub const UNRELEASED_MARK: &str = "+unreleased";

/// The stamp a build of `release` writes into every lock it emits: `release` itself for
/// a released build, `release` plus [`UNRELEASED_MARK`] for any other.
pub fn engine_stamp(release: &str, released: bool) -> String {
    if released {
        release.to_string()
    } else {
        format!("{release}{UNRELEASED_MARK}")
    }
}
