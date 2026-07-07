//! A minimal, dependency-free JSON text splicer.
//!
//! Locates the byte spans of object members and array elements within a JSON
//! document's *original* text, so a caller can replace or insert just the
//! bytes it owns — leaving every other byte (a human's key order, spacing,
//! indentation) untouched. `install`'s `.claude/settings.json` merge is the
//! sole consumer: it grafts the temper hook groups without re-serializing a
//! document it does not fully own.
//!
//! Every function here is only ever called over text `serde_json` has
//! already parsed successfully, so the byte scanning below assumes
//! well-formed JSON and never needs to report a parse error of its own.

use serde_json::Value as JsonValue;

/// A byte range `[start, end)` into the original document text.
pub(crate) type Span = (usize, usize);

/// One surgical change to apply to the original text: replace `span` (a
/// zero-length span is a pure insertion) with `replacement`.
pub(crate) struct Edit {
    pub(crate) span: Span,
    pub(crate) replacement: String,
}

/// Apply `edits` to `text` in one pass, producing the spliced document.
/// Edits may be given in any order and must not overlap — each is resolved
/// against the original `text`, so earlier edits never shift later spans.
pub(crate) fn apply_edits(text: &str, mut edits: Vec<Edit>) -> String {
    edits.sort_by_key(|edit| edit.span.0);
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0;
    for edit in edits {
        out.push_str(&text[cursor..edit.span.0]);
        out.push_str(&edit.replacement);
        cursor = edit.span.1;
    }
    out.push_str(&text[cursor..]);
    out
}

/// One member of a JSON object, in source order: its key and its value's span.
pub(crate) struct Member {
    pub(crate) key: String,
    pub(crate) value_span: Span,
}

/// A JSON object's structure in the source text: its own span (including
/// braces) and its members in source order.
pub(crate) struct ObjectShape {
    pub(crate) span: Span,
    pub(crate) members: Vec<Member>,
}

/// Locate the JSON object beginning at `start` (the byte offset of its `{`),
/// returning its span and members.
pub(crate) fn object_shape(text: &str, start: usize) -> ObjectShape {
    let bytes = text.as_bytes();
    let mut i = skip_ws(bytes, start + 1);
    let mut members = Vec::new();
    while bytes[i] != b'}' {
        let key_start = i;
        let key_end = skip_string(bytes, key_start);
        let key: String = serde_json::from_str(&text[key_start..key_end]).unwrap_or_default();
        i = skip_ws(bytes, key_end);
        i = skip_ws(bytes, i + 1); // the ':'
        let value_start = i;
        let value_end = skip_value(bytes, value_start);
        members.push(Member {
            key,
            value_span: (value_start, value_end),
        });
        i = skip_ws(bytes, value_end);
        if bytes[i] == b',' {
            i = skip_ws(bytes, i + 1);
        }
    }
    ObjectShape {
        span: (start, i + 1),
        members,
    }
}

/// A JSON array's structure in the source text: its own span (including
/// brackets) and each element's span, in source order.
pub(crate) struct ArrayShape {
    pub(crate) span: Span,
    pub(crate) elements: Vec<Span>,
}

/// Locate the JSON array beginning at `start` (the byte offset of its `[`).
pub(crate) fn array_shape(text: &str, start: usize) -> ArrayShape {
    let bytes = text.as_bytes();
    let mut i = skip_ws(bytes, start + 1);
    let mut elements = Vec::new();
    while bytes[i] != b']' {
        let value_start = i;
        let value_end = skip_value(bytes, value_start);
        elements.push((value_start, value_end));
        i = skip_ws(bytes, value_end);
        if bytes[i] == b',' {
            i = skip_ws(bytes, i + 1);
        }
    }
    ArrayShape {
        span: (start, i + 1),
        elements,
    }
}

fn skip_ws(bytes: &[u8], mut i: usize) -> usize {
    while bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    i
}

/// The offset just past the closing quote of the JSON string beginning at `start`.
fn skip_string(bytes: &[u8], start: usize) -> usize {
    let mut i = start + 1;
    while bytes[i] != b'"' {
        i += if bytes[i] == b'\\' { 2 } else { 1 };
    }
    i + 1
}

/// The offset just past the JSON value beginning at `start`.
fn skip_value(bytes: &[u8], start: usize) -> usize {
    match bytes[start] {
        b'"' => skip_string(bytes, start),
        b'{' => skip_bracketed(bytes, start, b'{', b'}'),
        b'[' => skip_bracketed(bytes, start, b'[', b']'),
        _ => {
            let mut i = start;
            while !matches!(bytes[i], b',' | b'}' | b']') && !bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            i
        }
    }
}

/// The offset just past the matching `close` for the `open` bracket at `start`,
/// skipping over nested strings so a brace/bracket inside a string is never
/// mistaken for structure.
fn skip_bracketed(bytes: &[u8], start: usize, open: u8, close: u8) -> usize {
    let mut depth: i32 = 0;
    let mut i = start;
    loop {
        if bytes[i] == b'"' {
            i = skip_string(bytes, i);
            continue;
        }
        if bytes[i] == open {
            depth += 1;
        } else if bytes[i] == close {
            depth -= 1;
            if depth == 0 {
                return i + 1;
            }
        }
        i += 1;
    }
}

/// Re-indent a `serde_json::to_string_pretty` fragment for insertion at
/// `base_indent`: every line but the first gets `base_indent` prefixed, so a
/// freshly built value lands at the right nesting depth in the spliced text.
fn reindent(pretty: &str, base_indent: &str) -> String {
    let mut lines = pretty.lines();
    let mut out = lines.next().unwrap_or_default().to_string();
    for line in lines {
        out.push('\n');
        out.push_str(base_indent);
        out.push_str(line);
    }
    out
}

/// Pretty-print `value` (`serde_json`'s 2-space convention) and reindent it
/// for insertion at `base_indent`.
pub(crate) fn pretty_at(value: &JsonValue, base_indent: &str) -> String {
    reindent(
        &serde_json::to_string_pretty(value).expect("a JsonValue serializes infallibly"),
        base_indent,
    )
}

/// The edit that inserts a new `key: value` member into the object described
/// by `shape`, appended after any existing members (or as the object's sole
/// entry if empty) at `level` — the number of two-space indent units the
/// member's own line sits at.
pub(crate) fn insert_member(
    shape: &ObjectShape,
    key: &str,
    value: &JsonValue,
    level: usize,
) -> Edit {
    let indent = "  ".repeat(level);
    let value_text = pretty_at(value, &indent);
    let key_text = serde_json::to_string(key).expect("a plain key serializes infallibly");
    match shape.members.last() {
        Some(last) => Edit {
            span: (last.value_span.1, last.value_span.1),
            replacement: format!(",\n{indent}{key_text}: {value_text}"),
        },
        None => {
            // An empty object's interior carries nothing worth preserving.
            let outer_indent = "  ".repeat(level.saturating_sub(1));
            Edit {
                span: (shape.span.0 + 1, shape.span.1 - 1),
                replacement: format!("\n{indent}{key_text}: {value_text}\n{outer_indent}"),
            }
        }
    }
}

/// The edit that appends `value` to the array described by `shape`, at
/// `level` (the number of two-space indent units the new element sits at).
pub(crate) fn append_element(shape: &ArrayShape, value: &JsonValue, level: usize) -> Edit {
    let indent = "  ".repeat(level);
    let value_text = pretty_at(value, &indent);
    match shape.elements.last() {
        Some(&(_, last_end)) => Edit {
            span: (last_end, last_end),
            replacement: format!(",\n{indent}{value_text}"),
        },
        None => {
            let outer_indent = "  ".repeat(level.saturating_sub(1));
            Edit {
                span: (shape.span.0 + 1, shape.span.1 - 1),
                replacement: format!("\n{indent}{value_text}\n{outer_indent}"),
            }
        }
    }
}
