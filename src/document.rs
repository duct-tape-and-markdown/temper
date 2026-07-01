//! The fenced document — the surface language's authored unit (`specs/20-surface.md`,
//! "The member document — the surface language").
//!
//! Every artifact in the surface is **one authored document**: a `+++`-fenced TOML
//! header over a markdown body, in a single file. Members (`SKILL.md`, `RULE.md`,
//! `SPEC.md`) and packages (`PACKAGE.md`) share this exact medium — a package is
//! "authored in the same medium as any member" (`specs/10-contracts.md`) — so the
//! split/parse/emit machinery lives here once, kind-agnostic, and every kind builds
//! its typed view on top.
//!
//! The header is held as a [`toml_edit::DocumentMut`] so a field patch is
//! **format-preserving**: comments, key order, and whitespace survive, the
//! co-authorship constraint the TOML-dialect Decision rests on (`specs/20-surface.md`)
//! — the human, the agent, and the tool all write the same file. The body is kept
//! **verbatim** (never re-rendered), and emit is **deterministic**: `parse` then
//! `emit` over an untouched document is byte-identical.
//!
//! This is foundation only — no pipeline is rewired here. Downstream, the member
//! and package kinds parse their source into a [`Document`], read the clause tables
//! out of its header, and patch it back format-preserving.

use miette::SourceSpan;
use toml_edit::DocumentMut;

/// The literal fence line that opens and closes a surface header. A line is a
/// fence when its content (trailing whitespace stripped) is exactly this.
const FENCE: &str = "+++";

/// A surface-language document: a `+++`-fenced TOML header over a markdown body.
///
/// The header is a format-preserving [`DocumentMut`] (patch it with
/// [`Document::header_mut`]); the body is carried verbatim. The exact fence lines
/// are retained so [`Document::emit`] reproduces the source byte-for-byte —
/// including an unusual line ending or trailing whitespace on the fence itself.
#[derive(Debug, Clone)]
pub struct Document {
    /// The opening fence line, including its terminator, exactly as parsed.
    open_fence: String,
    /// The parsed header. Format-preserving: `header.to_string()` reproduces the
    /// header text between the fences byte-for-byte until it is patched.
    header: DocumentMut,
    /// The closing fence line, including its terminator, exactly as parsed.
    close_fence: String,
    /// Everything after the closing fence line, verbatim (trailing bytes intact).
    body: String,
}

/// Errors raised while parsing a [`Document`]. Hard failures over malformed input —
/// distinct from a lint `Diagnostic`, which is a finding the engine collects. Each
/// carries the source text and a labelled span so `miette` renders a precise,
/// pointed error rather than a bare message.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum DocumentError {
    /// The document does not begin with a `+++` header fence.
    #[error("document has no opening `+++` header fence")]
    #[diagnostic(
        code(temper::document::missing_fence),
        help("a surface document must begin with a line containing only `+++`")
    )]
    MissingFence {
        /// The full source text, for the rendered diagnostic.
        #[source_code]
        src: String,
        /// Points at where the opening fence was expected.
        #[label("expected `+++` on the first line")]
        at: SourceSpan,
    },

    /// The opening fence is never matched by a closing `+++` line.
    #[error("document header fence is opened but never closed")]
    #[diagnostic(
        code(temper::document::unterminated_fence),
        help("add a closing `+++` line after the TOML header")
    )]
    UnterminatedFence {
        /// The full source text, for the rendered diagnostic.
        #[source_code]
        src: String,
        /// Points at the unmatched opening fence.
        #[label("this header fence is never closed")]
        at: SourceSpan,
    },

    /// The text between the fences is not valid TOML.
    #[error("document header is not valid TOML")]
    #[diagnostic(code(temper::document::bad_header))]
    BadHeader {
        /// The full source text, for the rendered diagnostic.
        #[source_code]
        src: String,
        /// Points at the offending span within the header (absolute in `src`).
        #[label("{message}")]
        at: SourceSpan,
        /// The parser's message, surfaced on the label.
        message: String,
        /// The underlying `toml_edit` parse error (boxed — it is large, and this
        /// keeps `DocumentError` small enough to return by value).
        #[source]
        source: Box<toml_edit::TomlError>,
    },
}

impl Document {
    /// Assemble a document from an already-parsed header and a verbatim body,
    /// using the canonical `+++\n` fences the tool emits.
    ///
    /// The parse/emit path retains the source's own fence lines; this constructor
    /// is for building a document from scratch (a fresh member the tool authors),
    /// where the canonical fence is the right default.
    pub fn new(header: DocumentMut, body: String) -> Self {
        Self {
            open_fence: format!("{FENCE}\n"),
            header,
            close_fence: format!("{FENCE}\n"),
            body,
        }
    }

    /// Parse a `+++`-fenced document: split the opening fence, take the TOML header
    /// up to the closing fence, and keep the remainder as the verbatim body.
    ///
    /// The header is parsed into a format-preserving [`DocumentMut`]. The fence
    /// lines are retained verbatim so [`Document::emit`] round-trips byte-for-byte.
    /// Errors are precise, never panics: a missing or unterminated fence, or a
    /// malformed header, each report a labelled span into `raw`.
    pub fn parse(raw: &str) -> Result<Self, DocumentError> {
        // The opening fence is the first line, whitespace-tolerant on its content.
        let open_fence = raw.split_inclusive('\n').next().unwrap_or("");
        if !is_fence(open_fence) {
            let len = open_fence.trim_end_matches('\n').len();
            return Err(DocumentError::MissingFence {
                src: raw.to_string(),
                at: SourceSpan::from((0, len)),
            });
        }
        let header_offset = open_fence.len();
        let rest = &raw[header_offset..];

        // Scan for the closing fence on its own line; the header is everything
        // before it, the body everything after.
        let mut offset = 0;
        for line in rest.split_inclusive('\n') {
            if is_fence(line) {
                let header_src = &rest[..offset];
                let close_fence = line;
                let body = &rest[offset + line.len()..];
                let header = parse_header(header_src, header_offset, raw)?;
                return Ok(Self {
                    open_fence: open_fence.to_string(),
                    header,
                    close_fence: close_fence.to_string(),
                    body: body.to_string(),
                });
            }
            offset += line.len();
        }

        Err(DocumentError::UnterminatedFence {
            src: raw.to_string(),
            at: SourceSpan::from((0, open_fence.trim_end_matches('\n').len())),
        })
    }

    /// Emit the document to its authored form: the opening fence, the header, the
    /// closing fence, then the body — reproducing the parse input byte-for-byte
    /// when nothing has been patched (deterministic round-trip). A header patch
    /// re-emits format-preserving; the fences and body are untouched.
    pub fn emit(&self) -> String {
        let mut out = String::with_capacity(
            self.open_fence.len() + self.close_fence.len() + self.body.len() + 64,
        );
        out.push_str(&self.open_fence);
        out.push_str(&self.header.to_string());
        out.push_str(&self.close_fence);
        out.push_str(&self.body);
        out
    }

    /// The parsed header, read-only. The clause tables (`specs/20-surface.md`) live
    /// here as TOML tables a kind reads its typed view out of.
    pub fn header(&self) -> &DocumentMut {
        &self.header
    }

    /// The header for a format-preserving patch. Mutating a value through this
    /// keeps comments, key order, and surrounding whitespace intact (`toml_edit`).
    pub fn header_mut(&mut self) -> &mut DocumentMut {
        &mut self.header
    }

    /// The verbatim markdown body below the header.
    pub fn body(&self) -> &str {
        &self.body
    }
}

/// Whether `line` is a `+++` fence: its content, with any trailing newline and
/// trailing whitespace stripped, is exactly `+++`. Trailing whitespace or a `\r`
/// (CRLF) is tolerated on the fence and preserved verbatim by the caller.
fn is_fence(line: &str) -> bool {
    line.strip_suffix('\n').unwrap_or(line).trim_end() == FENCE
}

/// Parse the header text into a [`DocumentMut`], mapping a TOML error to a
/// [`DocumentError::BadHeader`] whose span is absolute within the whole document
/// (`header_offset` shifts the parser's header-relative span into `raw`).
fn parse_header(
    header_src: &str,
    header_offset: usize,
    raw: &str,
) -> Result<DocumentMut, DocumentError> {
    header_src
        .parse::<DocumentMut>()
        .map_err(|source| DocumentError::BadHeader {
            src: raw.to_string(),
            at: header_span(&source, header_offset, header_src.len()),
            message: source.message().to_string(),
            source: Box::new(source),
        })
}

/// Locate a TOML parse error within the whole document. `toml_edit` reports a span
/// relative to the header; shift it by `header_offset`. When the parser gives no
/// span, fall back to the whole header block.
fn header_span(err: &toml_edit::TomlError, header_offset: usize, header_len: usize) -> SourceSpan {
    match err.span() {
        Some(span) => SourceSpan::from((header_offset + span.start, span.len())),
        None => SourceSpan::from((header_offset, header_len)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml_edit::value;

    /// A representative surface document: a `+++`-fenced header with comments, a
    /// blank line, dotted clause tables in a deliberate order, an unknown table,
    /// and a body whose trailing bytes must survive intact.
    const FIXTURE: &str = "+++\n\
# every clause governing this member\n\
[clause.name]\n\
value = \"dev-standards\"\n\
\n\
[clause.description]\n\
value = \"Maintains development standards.\"\n\
\n\
[edge.lint-runner]           # an authored relationship\n\
relation = \"depends-on\"\n\
+++\n\
# Dev standards\n\
\n\
The body, with a trailing space.   \n\
Last line, no newline.";

    #[test]
    fn parse_then_emit_is_byte_identical() {
        let doc = Document::parse(FIXTURE).unwrap();
        assert_eq!(doc.emit(), FIXTURE);
    }

    #[test]
    fn body_is_verbatim() {
        let doc = Document::parse(FIXTURE).unwrap();
        assert_eq!(
            doc.body(),
            "# Dev standards\n\nThe body, with a trailing space.   \nLast line, no newline."
        );
    }

    #[test]
    fn empty_header_round_trips() {
        let raw = "+++\n+++\n# just a body\n";
        let doc = Document::parse(raw).unwrap();
        assert!(doc.header().as_table().is_empty());
        assert_eq!(doc.body(), "# just a body\n");
        assert_eq!(doc.emit(), raw);
    }

    #[test]
    fn header_patch_preserves_comments_order_and_whitespace() {
        let mut doc = Document::parse(FIXTURE).unwrap();

        // Patch one field's value through the format-preserving header.
        doc.header_mut()["clause"]["name"]["value"] = value("renamed");
        let out = doc.emit();

        // The patched value landed.
        assert!(out.contains("value = \"renamed\""));
        // Comments — both the leading one and the inline one — survive.
        assert!(out.contains("# every clause governing this member"));
        assert!(out.contains("# an authored relationship"));
        // Key order is intact: name still precedes description.
        let name_at = out.find("[clause.name]").unwrap();
        let desc_at = out.find("[clause.description]").unwrap();
        assert!(name_at < desc_at, "table order must be preserved");
        // The old value is gone (it was the one thing patched).
        assert!(!out.contains("value = \"dev-standards\""));
        // The unrelated edge clause is untouched.
        assert!(out.contains("\"depends-on\""));
        // Whitespace and everything else is intact: only the one value changed,
        // so the whole document differs from the source by exactly that string.
        assert_eq!(out, FIXTURE.replace("dev-standards", "renamed"));
    }

    #[test]
    fn unknown_header_tables_are_preserved_verbatim() {
        // A table no kind models today is carried through untouched, even across a
        // patch to an unrelated field.
        let mut doc = Document::parse(FIXTURE).unwrap();
        doc.header_mut()["clause"]["description"]["value"] = value("changed");
        let out = doc.emit();
        assert!(out.contains("[edge.lint-runner]           # an authored relationship"));
        assert!(out.contains("relation = \"depends-on\""));
    }

    #[test]
    fn missing_opening_fence_is_a_precise_error() {
        let err = Document::parse("# no fence here\nbody\n").unwrap_err();
        assert!(matches!(err, DocumentError::MissingFence { .. }));
    }

    #[test]
    fn empty_input_is_a_missing_fence_error() {
        let err = Document::parse("").unwrap_err();
        assert!(matches!(err, DocumentError::MissingFence { .. }));
    }

    #[test]
    fn unterminated_fence_is_a_precise_error() {
        let err = Document::parse("+++\nname = \"x\"\nno closing fence\n").unwrap_err();
        assert!(matches!(err, DocumentError::UnterminatedFence { .. }));
    }

    #[test]
    fn bare_opening_fence_is_unterminated_not_a_panic() {
        let err = Document::parse("+++").unwrap_err();
        assert!(matches!(err, DocumentError::UnterminatedFence { .. }));
    }

    #[test]
    fn malformed_header_toml_is_a_precise_error() {
        let err = Document::parse("+++\nnot = = valid\n+++\nbody\n").unwrap_err();
        let DocumentError::BadHeader { at, .. } = err else {
            panic!("expected a BadHeader error, got {err:?}");
        };
        // The span points into the header region (past the opening fence), never
        // at offset zero — the error is located, not generic.
        assert!(at.offset() >= "+++\n".len());
    }

    #[test]
    fn new_uses_canonical_fences_and_round_trips() {
        let mut header = DocumentMut::new();
        header["name"] = value("fresh");
        let doc = Document::new(header, "# body\n".to_string());
        let emitted = doc.emit();
        assert_eq!(emitted, "+++\nname = \"fresh\"\n+++\n# body\n");
        // Re-parsing a freshly-authored document round-trips it.
        assert_eq!(Document::parse(&emitted).unwrap().emit(), emitted);
    }
}
