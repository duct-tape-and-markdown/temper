//! Shared content hashing — the single home for the SHA-256 hex that anchors
//! provenance and drift (`specs/model/pipeline.md`, "The lock" and "Emit").
//! `source_hash` is the SHA-256 of an artifact's authored
//! source bytes; the drift engine re-hashes on-disk bytes and compares against
//! that anchor. Both compute the same lowercase hex here, over raw `&[u8]`, so
//! the hash stays kind-agnostic — no artifact typing is lost by sharing it.

use sha2::{Digest, Sha256};

/// Lowercase hex SHA-256 of `bytes`.
pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
