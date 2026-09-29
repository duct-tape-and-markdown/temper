//! Shared content hashing — the single home for the SHA-256 hex that anchors
//! provenance and drift. Also home to the shared read+UTF-8-decode primitive all
//! formats use to load source files, with each format mapping the error to its own
//! vocabulary, and to `canonicalize_eol` — the one EOL normalizer — which the drift
//! engine applies before re-hashing so a CRLF-filtered checkout reads clean against
//! an LF baseline, and which the projection writer applies through
//! [`canonicalize_eol_str`] so emitted bytes are LF-uniform whatever the source's
//! own convention.
//! Every caller computes the same lowercase hex here, over raw `&[u8]`, so the
//! hash stays kind-agnostic — no artifact typing is lost by sharing it.

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

/// Canonicalize line endings in raw bytes: CRLF (b'\r' b'\n') collapses to LF (b'\n'),
/// and lone CR (b'\r' not followed by b'\n') becomes LF.
/// Used for drift comparisons: a checkout git-filtered to CRLF reads clean against an LF baseline.
pub fn canonicalize_eol(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\r' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                // CRLF -> LF
                out.push(b'\n');
                i += 2;
            } else {
                // Lone CR -> LF
                out.push(b'\n');
                i += 1;
            }
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

/// [`canonicalize_eol`] over text, for callers holding a `&str` rather than raw bytes —
/// the projection writer, which writes LF uniformly regardless of the source's own
/// convention. The decode back cannot fail: CR and LF are ASCII, so rewriting them
/// touches no multi-byte sequence and UTF-8 validity carries through byte-for-byte.
pub fn canonicalize_eol_str(text: &str) -> String {
    String::from_utf8(canonicalize_eol(text.as_bytes()))
        .expect("canonicalizing ASCII line endings preserves UTF-8 validity")
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

/// Read a file and decode it as UTF-8. Each format maps the error to its own error
/// vocabulary at the call site.
pub(crate) fn read_utf8(path: &Path) -> Result<String, ReadUtf8Error> {
    let bytes = std::fs::read(path).map_err(|source| ReadUtf8Error::Io {
        path: path.to_path_buf(),
        source,
    })?;
    String::from_utf8(bytes).map_err(|source| ReadUtf8Error::NotUtf8 {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalize_eol_folds_every_convention_onto_lf() {
        // The one rule, over both faces: a CRLF pair collapses to a single LF, and a
        // lone CR (old Mac style) becomes LF too.
        assert_eq!(canonicalize_eol(b"a\r\nb\rc\n"), b"a\nb\nc\n");
        assert_eq!(canonicalize_eol_str("a\r\nb\rc\n"), "a\nb\nc\n");

        // A trailing lone CR has no successor to inspect and still folds.
        assert_eq!(canonicalize_eol_str("a\r"), "a\n");
    }

    #[test]
    fn canonicalize_eol_leaves_lf_text_identical() {
        let lf = "---\ntitle: t\n---\n\nBody.\n";
        assert_eq!(canonicalize_eol(lf.as_bytes()), lf.as_bytes());
        assert_eq!(canonicalize_eol_str(lf), lf);
    }

    #[test]
    fn canonicalize_eol_preserves_multi_byte_utf8_around_a_cr() {
        // The `&str` face's decode-back rests on CR being ASCII: multi-byte scalars on
        // either side of a folded line ending survive intact.
        let crlf = "é—\r\n日本\rπ";
        assert_eq!(canonicalize_eol_str(crlf), "é—\n日本\nπ");
        assert_eq!(canonicalize_eol(crlf.as_bytes()), "é—\n日本\nπ".as_bytes());
    }
}
