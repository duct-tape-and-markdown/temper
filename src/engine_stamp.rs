//! The engine stamp a lock records: the release version, marked when the build is not
//! that release. Shared verbatim with `build.rs`, which computes the stamp this binary
//! carries, so the build script and the tests read one spelling.
//!
//! An unreleased build's mark carries a digest of the built-in facts it embeds
//! (`src/builtin_lock.toml`), so two unreleased builds of one release line stamp alike
//! exactly when their built-ins agree: a lock snapshotted before a built-in moved reads
//! as another engine's, and a lock emitted by a build whose built-ins never moved stays
//! byte-stable.

use sha2::{Digest, Sha256};

/// The mark an unreleased build appends to the release version it follows. Semver
/// build metadata, so the stamp still names the release line it was built on; the
/// built-in-facts digest follows it as a further dot-separated identifier.
pub const UNRELEASED_MARK: &str = "+unreleased";

/// How many hex digits of the built-in-facts digest the stamp carries — enough that two
/// distinct built-in snapshots never plausibly share one.
const DIGEST_HEX_LEN: usize = 12;

/// The digest of the embedded built-in facts an unreleased stamp carries: the leading
/// twelve lowercase hex digits of the SHA-256 of `builtin_facts`.
pub fn builtin_digest(builtin_facts: &[u8]) -> String {
    let hash = Sha256::digest(builtin_facts);
    let mut hex = String::with_capacity(DIGEST_HEX_LEN);
    for byte in hash.iter().take(DIGEST_HEX_LEN / 2) {
        hex.push_str(&format!("{byte:02x}"));
    }
    hex
}

/// The stamp a build of `release` writes into every lock it emits: `release` itself for
/// a released build; for any other, `release` plus [`UNRELEASED_MARK`] plus the
/// [`builtin_digest`] of the `builtin_facts` it embeds.
pub fn engine_stamp(release: &str, released: bool, builtin_facts: &[u8]) -> String {
    if released {
        release.to_string()
    } else {
        format!(
            "{release}{UNRELEASED_MARK}.{}",
            builtin_digest(builtin_facts)
        )
    }
}
