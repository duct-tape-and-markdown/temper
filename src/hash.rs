//! Shared content hashing — the single home for the SHA-256 hex that anchors
//! provenance and drift. Also home to the shared read+UTF-8-decode primitive all
//! formats use to load source files, with each format mapping the error to its own
//! vocabulary.
//! `source_hash` is the SHA-256 of an artifact's authored
//! source bytes; the drift engine re-hashes on-disk bytes and compares against
//! that anchor. Both compute the same lowercase hex here, over raw `&[u8]`, so
//! the hash stays kind-agnostic — no artifact typing is lost by sharing it.

use std::path::{Path, PathBuf};

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

/// Errors from reading a file and decoding it as UTF-8.
#[derive(Debug)]
pub(crate) enum ReadUtf8Error {
    /// File I/O failed.
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },
    /// File contents are not valid UTF-8.
    NotUtf8 {
        /// The file whose bytes could not be decoded.
        path: PathBuf,
        /// The decode error.
        source: std::string::FromUtf8Error,
    },
}

/// Read a file and decode it as UTF-8, returning both the raw bytes and decoded string.
/// Each format maps the error to its own error vocabulary at the call site.
pub(crate) fn read_utf8(path: &Path) -> Result<(Vec<u8>, String), ReadUtf8Error> {
    let bytes = std::fs::read(path).map_err(|source| ReadUtf8Error::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let string = String::from_utf8(bytes.clone()).map_err(|source| ReadUtf8Error::NotUtf8 {
        path: path.to_path_buf(),
        source,
    })?;
    Ok((bytes, string))
}
