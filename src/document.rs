//! The fenced document — the surface language's authored unit (`specs/architecture/20-surface.md`,
//! "The member document — the surface language").
//!
//! Every artifact in the surface is **one authored document**: a `+++`-fenced TOML
//! header over a markdown body, in a single file. Members (`SKILL.md`, `RULE.md`,
//! `SPEC.md`) and packages (`PACKAGE.md`) share this exact medium — a package is
//! "authored in the same medium as any member" (`specs/architecture/10-contracts.md`) — so the
//! split/parse/emit machinery lives here once, kind-agnostic, and every kind builds
//! its typed view on top.
//!
//! The header is held as a [`toml_edit::DocumentMut`] so a field patch is
//! **format-preserving**: comments, key order, and whitespace survive, the
//! co-authorship constraint the TOML-dialect Decision rests on (`specs/architecture/20-surface.md`)
//! — the human, the agent, and the tool all write the same file. The body is kept
//! **verbatim** (never re-rendered), and emit is **deterministic**: `parse` then
//! `emit` over an untouched document is byte-identical.
//!
//! This is foundation only — no pipeline is rewired here. Downstream, the member
//! and package kinds parse their source into a [`Document`], read the clause tables
//! out of its header, and patch it back format-preserving.

use miette::SourceSpan;
use serde_json::{Map as JsonMap, Value as JsonValue};
use toml_edit::{DocumentMut, Item, Table, Value};

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

    /// A `[requirement.<name>]` module published on a member header carries a key
    /// outside its closed facet set (`means`/`kind`/`package`/`required`) — the same
    /// posture as an unknown contract key (`specs/architecture/10-contracts.md`, "unknown keys are
    /// rejected"): a typo that would silently drop a published obligation is a hard
    /// load error, not a dropped key.
    #[error("published requirement `{name}` has unknown key `{key}`")]
    #[diagnostic(
        code(temper::document::requirement_unknown_key),
        help(
            "a member-published requirement carries only `means`, `kind`, `package`, and `required`"
        )
    )]
    RequirementUnknownKey {
        /// The published requirement carrying the stray key.
        name: String,
        /// The unrecognized key.
        key: String,
    },

    /// A `[requirement.<name>]` module published on a member header carries a facet
    /// of the wrong TOML type — `means`/`kind`/`package` not a string, or `required`
    /// not a boolean.
    #[error("published requirement `{name}` key `{key}` must be {expected}")]
    #[diagnostic(code(temper::document::requirement_wrong_type))]
    RequirementWrongType {
        /// The published requirement whose facet is mistyped.
        name: String,
        /// The mistyped facet.
        key: &'static str,
        /// The type that was expected, for the message.
        expected: &'static str,
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

    /// The parsed header, read-only. The clause tables (`specs/architecture/20-surface.md`) live
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

/// A `[satisfies.<requirement>]` clause module (`specs/architecture/20-surface.md`, "The member
/// document"): the member opts into filling `requirement`, carrying the optional
/// authored `rationale` — the *why*, first-class beside the link rather than
/// delegated and forgotten (`00-intent.md` law 7). Authored on the surface, never
/// imported; the coverage check reads only the requirement name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Satisfies {
    /// The declared requirement this member opts into filling.
    pub requirement: String,
    /// The authored rationale — never a decidable feature, so no check reads it.
    pub rationale: Option<String>,
}

impl Satisfies {
    /// A `satisfies` clause naming `requirement`, with no rationale.
    pub fn new(requirement: impl Into<String>) -> Self {
        Self {
            requirement: requirement.into(),
            rationale: None,
        }
    }
}

/// A `[requirement.<name>]` clause module **published** on a member header
/// (`specs/architecture/10-contracts.md`, "Decision: a requirement's publisher is any authored
/// surface document"): the member declares a named obligation — the demand side of
/// the fill edge, joined to the roster by another member's `satisfies`. It carries
/// the same facets the assembly roster's `[requirement.<name>]` does (`means`,
/// `kind`, `package`, `required`); the richer set-scope facets stay assembly-only.
/// Authored on the surface, never imported; one namespace with the assembly roster,
/// so a cross-publisher name collision is an admissibility finding (resolved in the
/// gate), never a shadow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedRequirement {
    /// The requirement's name — the `[requirement.<name>]` module key.
    pub name: String,
    /// The authored *intent*, the why. Carried verbatim and never interpreted
    /// (`00-intent.md` law 3).
    pub means: Option<String>,
    /// The artifact kind that may fill the requirement. Absent ⇒ kind-blind.
    pub kind: Option<String>,
    /// The package the filling artifact must conform to, named by name. Absent ⇒ no
    /// package constraint.
    pub package: Option<String>,
    /// Whether an unfilled requirement is gate-blocking. Absent ⇒ `false`.
    pub required: bool,
}

/// Get-or-create an *implicit* parent table under `key`, so its children render as
/// standalone `[key.<child>]` tables (never a bare `[key]` header). This is what
/// makes each clause module its own labelled `[table]` in the fenced header.
fn child_table<'a>(header: &'a mut DocumentMut, key: &'static str) -> &'a mut Table {
    let table = header.as_table_mut();
    if !table.contains_key(key) {
        let mut parent = Table::new();
        parent.set_implicit(true);
        table.insert(key, Item::Table(parent));
    }
    table
        .get_mut(key)
        .and_then(Item::as_table_mut)
        .expect("the just-inserted parent is a table")
}

/// Emit a `[clause.<field>]` module carrying `value = <val>` into `header`. Called
/// once per structured field the member carries, in the caller's order (which is
/// what makes projection deterministic).
pub fn add_clause(header: &mut DocumentMut, field: &str, val: Value) {
    let mut module = Table::new();
    module.insert("value", Item::Value(val));
    child_table(header, "clause").insert(field, Item::Table(module));
}

/// Emit a `[satisfies.<requirement>]` module (with its optional `rationale`).
pub fn add_satisfies(header: &mut DocumentMut, satisfies: &Satisfies) {
    let mut module = Table::new();
    if let Some(rationale) = &satisfies.rationale {
        module.insert("rationale", Item::Value(Value::from(rationale.clone())));
    }
    child_table(header, "satisfies").insert(&satisfies.requirement, Item::Table(module));
}

/// Emit a `[requirement.<name>]` module carrying the facets the member publishes
/// (`means`, `kind`, `package`, `required`). The demand-side mirror of
/// [`add_satisfies`]; an absent facet (`None`, or `required = false`) is omitted, so
/// emit is the exact inverse of [`requirements`] and the round-trip is symmetric.
pub fn add_requirement(header: &mut DocumentMut, requirement: &PublishedRequirement) {
    let mut module = Table::new();
    if let Some(means) = &requirement.means {
        module.insert("means", Item::Value(Value::from(means.clone())));
    }
    if let Some(kind) = &requirement.kind {
        module.insert("kind", Item::Value(Value::from(kind.clone())));
    }
    if let Some(package) = &requirement.package {
        module.insert("package", Item::Value(Value::from(package.clone())));
    }
    if requirement.required {
        module.insert("required", Item::Value(Value::from(true)));
    }
    child_table(header, "requirement").insert(&requirement.name, Item::Table(module));
}

/// Emit the generated `[provenance]` module — `source_path` + `import_hash`, the
/// drift anchor (`specs/architecture/20-surface.md`). Always last, so the authored clauses read
/// first and the generated lock trails them.
pub fn add_provenance(header: &mut DocumentMut, source_path: &str, import_hash: &str) {
    let mut module = Table::new();
    module.insert(
        "source_path",
        Item::Value(Value::from(source_path.to_string())),
    );
    module.insert(
        "import_hash",
        Item::Value(Value::from(import_hash.to_string())),
    );
    header
        .as_table_mut()
        .insert("provenance", Item::Table(module));
}

/// The `value` item of each `[clause.<field>]` module in `header`, in document
/// order — the structured fields the member carries (typed fields *and* the
/// verbatim-preserved unknown frontmatter keys). A module with no `value` key is
/// skipped rather than erroring: it names a field but carries nothing to read.
pub fn clauses(header: &DocumentMut) -> Vec<(String, &Item)> {
    header
        .get("clause")
        .and_then(Item::as_table)
        .map(|table| {
            table
                .iter()
                .filter_map(|(field, item)| {
                    item.as_table()
                        .and_then(|module| module.get("value"))
                        .map(|value| (field.to_string(), value))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Convert a header [`Item`] to a [`serde_json::Value`] — the inverse of the
/// built-in adapters' `json_to_toml_value`, so a `[clause.<field>]` `value` read by
/// [`clauses`] lands in a member's frontmatter the same shape a built-in's hand-
/// written parser produces (`specs/architecture/20-surface.md`). A JSON-null-unrepresentable item
/// (`Item::None`, a bare TOML `Datetime` is kept as its string form) yields `None`,
/// dropped rather than invented. Recurses through tables and arrays so a nested
/// clause value round-trips.
#[must_use]
pub fn item_to_json(item: &Item) -> Option<JsonValue> {
    match item {
        Item::Value(val) => value_to_json(val),
        Item::Table(table) => {
            let mut map = JsonMap::new();
            for (key, child) in table.iter() {
                if let Some(json) = item_to_json(child) {
                    map.insert(key.to_string(), json);
                }
            }
            Some(JsonValue::Object(map))
        }
        Item::ArrayOfTables(tables) => Some(JsonValue::Array(
            tables
                .iter()
                .map(|t| item_to_json(&Item::Table(t.clone())))
                .collect::<Option<Vec<_>>>()?,
        )),
        Item::None => None,
    }
}

/// Convert a header [`Value`] to a [`serde_json::Value`]; a TOML `Datetime` carries
/// as its string form (JSON has no datetime scalar).
fn value_to_json(val: &Value) -> Option<JsonValue> {
    Some(match val {
        Value::String(s) => JsonValue::from(s.value().clone()),
        Value::Integer(i) => JsonValue::from(*i.value()),
        Value::Float(f) => JsonValue::from(*f.value()),
        Value::Boolean(b) => JsonValue::from(*b.value()),
        Value::Datetime(d) => JsonValue::from(d.value().to_string()),
        Value::Array(array) => JsonValue::Array(array.iter().filter_map(value_to_json).collect()),
        Value::InlineTable(inline) => {
            let mut map = JsonMap::new();
            for (key, child) in inline.iter() {
                if let Some(json) = value_to_json(child) {
                    map.insert(key.to_string(), json);
                }
            }
            JsonValue::Object(map)
        }
    })
}

/// The `[satisfies.<requirement>]` modules in `header`, in document order.
pub fn satisfies(header: &DocumentMut) -> Vec<Satisfies> {
    header
        .get("satisfies")
        .and_then(Item::as_table)
        .map(|table| {
            table
                .iter()
                .map(|(requirement, item)| Satisfies {
                    requirement: requirement.to_string(),
                    rationale: item
                        .as_table()
                        .and_then(|module| module.get("rationale"))
                        .and_then(Item::as_str)
                        .map(str::to_string),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// The `[requirement.<name>]` modules a member **publishes** in its header, in
/// document order (`specs/architecture/10-contracts.md`, "Decision: a requirement's publisher is
/// any authored surface document") — the demand-side mirror of [`satisfies`]. A
/// module with no body is a bare (all-facets-absent) obligation, valid like a bare
/// assembly `[requirement.<name>]`.
///
/// # Errors
///
/// Rejects a module carrying a key outside the closed facet set
/// ([`DocumentError::RequirementUnknownKey`]) or a facet of the wrong TOML type
/// ([`DocumentError::RequirementWrongType`]) — a typo that would silently drop a
/// published obligation is a hard load error (`specs/architecture/10-contracts.md`, "unknown keys
/// are rejected").
pub fn requirements(header: &DocumentMut) -> Result<Vec<PublishedRequirement>, DocumentError> {
    let Some(table) = header.get("requirement").and_then(Item::as_table) else {
        return Ok(Vec::new());
    };
    let mut requirements = Vec::new();
    for (name, item) in table.iter() {
        let mut means = None;
        let mut kind = None;
        let mut package = None;
        let mut required = false;
        if let Some(module) = item.as_table() {
            for (key, value) in module.iter() {
                match key {
                    "means" => means = Some(requirement_str(value, name, "means")?),
                    "kind" => kind = Some(requirement_str(value, name, "kind")?),
                    "package" => package = Some(requirement_str(value, name, "package")?),
                    "required" => required = requirement_bool(value, name)?,
                    other => {
                        return Err(DocumentError::RequirementUnknownKey {
                            name: name.to_string(),
                            key: other.to_string(),
                        });
                    }
                }
            }
        }
        requirements.push(PublishedRequirement {
            name: name.to_string(),
            means,
            kind,
            package,
            required,
        });
    }
    Ok(requirements)
}

/// Read one string facet off a published `[requirement.<name>]` module, mapping a
/// non-string value to a precise [`DocumentError::RequirementWrongType`].
fn requirement_str(value: &Item, name: &str, key: &'static str) -> Result<String, DocumentError> {
    value
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| DocumentError::RequirementWrongType {
            name: name.to_string(),
            key,
            expected: "a string",
        })
}

/// Read the boolean `required` facet off a published `[requirement.<name>]` module,
/// mapping a non-boolean value to a precise [`DocumentError::RequirementWrongType`].
fn requirement_bool(value: &Item, name: &str) -> Result<bool, DocumentError> {
    value.as_bool().ok_or(DocumentError::RequirementWrongType {
        name: name.to_string(),
        key: "required",
        expected: "a boolean",
    })
}

/// The generated `[provenance]` module's `(source_path, import_hash)`, or `None`
/// when it is absent or missing either key — a surface missing what the tool always
/// writes is malformed, and the caller turns that `None` into a precise error.
pub fn provenance(header: &DocumentMut) -> Option<(String, String)> {
    let table = header.get("provenance").and_then(Item::as_table)?;
    let source_path = table.get("source_path").and_then(Item::as_str)?;
    let import_hash = table.get("import_hash").and_then(Item::as_str)?;
    Some((source_path.to_string(), import_hash.to_string()))
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
[future.annotation]          # a table no kind models — carried verbatim\n\
note = \"kept as-is\"\n\
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
        assert!(out.contains("# a table no kind models — carried verbatim"));
        // Key order is intact: name still precedes description.
        let name_at = out.find("[clause.name]").unwrap();
        let desc_at = out.find("[clause.description]").unwrap();
        assert!(name_at < desc_at, "table order must be preserved");
        // The old value is gone (it was the one thing patched).
        assert!(!out.contains("value = \"dev-standards\""));
        // The unrelated unmodelled table is untouched.
        assert!(out.contains("note = \"kept as-is\""));
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
        assert!(
            out.contains(
                "[future.annotation]          # a table no kind models — carried verbatim"
            )
        );
        assert!(out.contains("note = \"kept as-is\""));
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
    fn clause_modules_emit_as_labelled_tables_and_round_trip() {
        // Build a header from clause modules the way a member projector does: field
        // clauses, an authored satisfies with rationale, then provenance.
        let mut header = DocumentMut::new();
        add_clause(&mut header, "name", Value::from("dev-standards"));
        add_clause(&mut header, "allowed-tools", {
            let mut array = toml_edit::Array::new();
            array.push("Bash");
            Value::Array(array)
        });
        add_satisfies(
            &mut header,
            &Satisfies {
                requirement: "engineering-standards".to_string(),
                rationale: Some("the home for enforcement".to_string()),
            },
        );
        add_provenance(&mut header, "./SKILL.md", "abc123");
        let doc = Document::new(header, "# Body\n".to_string());
        let emitted = doc.emit();

        // Each clause is its own labelled `[table]`, never a bare `[clause]`.
        assert!(emitted.contains("[clause.name]\nvalue = \"dev-standards\""));
        assert!(emitted.contains("[clause.allowed-tools]\nvalue = [\"Bash\"]"));
        assert!(!emitted.contains("[clause]\n"));
        assert!(emitted.contains("[satisfies.engineering-standards]\nrationale ="));
        assert!(emitted.contains("[provenance]\nsource_path = \"./SKILL.md\""));

        // The readers recover exactly what was emitted, in order.
        let parsed = Document::parse(&emitted).unwrap();
        let read: Vec<String> = clauses(parsed.header())
            .into_iter()
            .map(|(field, _)| field)
            .collect();
        assert_eq!(read, vec!["name".to_string(), "allowed-tools".to_string()]);
        assert_eq!(
            satisfies(parsed.header()),
            vec![Satisfies {
                requirement: "engineering-standards".to_string(),
                rationale: Some("the home for enforcement".to_string()),
            }]
        );
        assert_eq!(
            provenance(parsed.header()),
            Some(("./SKILL.md".to_string(), "abc123".to_string()))
        );
        // Re-emitting a parsed document is byte-identical — deterministic round-trip.
        assert_eq!(parsed.emit(), emitted);
    }

    #[test]
    fn published_requirement_modules_emit_as_labelled_tables_and_round_trip() {
        // A member publishes two `[requirement.<name>]` modules: a fully-facetted one
        // and a bare (name-only) one. Each emits as its own labelled table and reads
        // back identically, the demand-side mirror of the satisfies round-trip.
        let mut header = DocumentMut::new();
        add_requirement(
            &mut header,
            &PublishedRequirement {
                name: "architecture".to_string(),
                means: Some("the corpus carries an architecture spec".to_string()),
                kind: Some("spec".to_string()),
                package: Some("spec.temper".to_string()),
                required: true,
            },
        );
        add_requirement(
            &mut header,
            &PublishedRequirement {
                name: "bare".to_string(),
                means: None,
                kind: None,
                package: None,
                required: false,
            },
        );
        let doc = Document::new(header, "# Body\n".to_string());
        let emitted = doc.emit();

        // Each is its own labelled `[requirement.<name>]` table.
        assert!(emitted.contains("[requirement.architecture]\nmeans ="));
        assert!(emitted.contains("kind = \"spec\""));
        assert!(emitted.contains("required = true"));
        // A bare requirement is an empty module — a name with no facets.
        assert!(emitted.contains("[requirement.bare]"));

        let parsed = Document::parse(&emitted).unwrap();
        assert_eq!(
            requirements(parsed.header()).unwrap(),
            vec![
                PublishedRequirement {
                    name: "architecture".to_string(),
                    means: Some("the corpus carries an architecture spec".to_string()),
                    kind: Some("spec".to_string()),
                    package: Some("spec.temper".to_string()),
                    required: true,
                },
                PublishedRequirement {
                    name: "bare".to_string(),
                    means: None,
                    kind: None,
                    package: None,
                    required: false,
                },
            ]
        );
        // Deterministic round-trip: re-emitting a parsed document is byte-identical.
        assert_eq!(parsed.emit(), emitted);
    }

    #[test]
    fn a_published_requirement_with_an_unknown_key_is_rejected() {
        // A facet outside the closed set (`count` is assembly-only) is a hard load
        // error, not a silently dropped key (`specs/architecture/10-contracts.md`, unknown keys
        // rejected).
        let raw = "+++\n[requirement.x]\ncount = 3\n+++\n# body\n";
        let doc = Document::parse(raw).unwrap();
        let err = requirements(doc.header()).unwrap_err();
        assert!(matches!(
            err,
            DocumentError::RequirementUnknownKey { ref name, ref key }
                if name == "x" && key == "count"
        ));
    }

    #[test]
    fn a_published_requirement_with_a_mistyped_facet_is_rejected() {
        // `required` must be a boolean; a string is a precise wrong-type error.
        let raw = "+++\n[requirement.x]\nrequired = \"yes\"\n+++\n# body\n";
        let doc = Document::parse(raw).unwrap();
        let err = requirements(doc.header()).unwrap_err();
        assert!(matches!(
            err,
            DocumentError::RequirementWrongType {
                key: "required",
                ..
            }
        ));
    }

    #[test]
    fn a_header_with_no_requirement_modules_reads_empty() {
        let doc = Document::parse(FIXTURE).unwrap();
        assert!(requirements(doc.header()).unwrap().is_empty());
    }

    #[test]
    fn item_to_json_converts_each_clause_value_kind_faithfully() {
        // The inverse of the built-in adapters' `json_to_toml_value`: a `[clause.<field>]`
        // `value` read by `clauses` lands JSON-kind-faithful, so a custom member's
        // frontmatter matches a built-in's hand-parsed shape (`specs/architecture/20-surface.md`).
        let raw = "+++\n\
[clause.name]\nvalue = \"demo\"\n\
[clause.priority]\nvalue = 7\n\
[clause.enabled]\nvalue = true\n\
[clause.tools]\nvalue = [\"Bash\", \"Read\"]\n\
[clause.meta]\nvalue = { team = \"core\" }\n\
+++\n# body\n";
        let doc = Document::parse(raw).unwrap();
        let by_field: std::collections::BTreeMap<String, JsonValue> = clauses(doc.header())
            .into_iter()
            .filter_map(|(field, value)| item_to_json(value).map(|json| (field, json)))
            .collect();

        assert_eq!(by_field["name"], JsonValue::String("demo".to_string()));
        assert_eq!(by_field["priority"], JsonValue::from(7));
        assert_eq!(by_field["enabled"], JsonValue::Bool(true));
        assert_eq!(by_field["tools"], serde_json::json!(["Bash", "Read"]));
        assert_eq!(by_field["meta"], serde_json::json!({ "team": "core" }));
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
