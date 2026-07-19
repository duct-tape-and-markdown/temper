//! The layout-document reader — the typed reading of a `layout`-content document's
//! heading tree into fields, edges, members, and prose.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::extract;

/// A declared **layout** — the ordered regions a `layout`-content kind's body is read as,
/// each one of the three corpus primitives ([`LayoutRegion`]). The regions are the
/// declared template; matching them against a member's actual heading tree is the
/// reader's job, not this fact's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    /// The layout's regions, in declared document order.
    pub regions: Vec<LayoutRegion>,
}

/// One region of a [`Layout`] — a single corpus primitive over the body's heading tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutRegion {
    /// **prose** — a verbatim span, or, when `import` is declared, a reference resolving
    /// to a file's contents (fingerprinted, refusing when dangling — the resolver's job).
    Prose {
        /// The import reference this region resolves from, when it imports a file's
        /// contents rather than carrying its own verbatim words.
        import: Option<String>,
    },
    /// **field section** — a heading whose span fills the named field `slot`.
    Field {
        /// The field slot the heading's span fills.
        slot: String,
    },
    /// **member collection** — a heading whose child headings are each one member of the
    /// named kind; a member's identity is its slugged child heading, or `key` when an
    /// explicit one is declared (surviving a retitle).
    Collection {
        /// The child member kind each child heading instantiates.
        member_kind: String,
        /// An explicit identity key overriding the slugged heading, when declared.
        key: Option<String>,
    },
}

/// The typed reading of a `layout`-content document: what emit derives its
/// declaration rows from and the gate reads its fields off, all off the one authored
/// source. Field sections fill named [`fields`](LayoutReading::fields); member
/// collections yield [`members`](LayoutReading::members) carrying slugged-heading (or
/// explicit-key) identity; verbatim prose regions land in
/// [`prose`](LayoutReading::prose), in document order. The document is never written
/// back — it is the source, read (`pipeline.md`, "Emit").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LayoutReading {
    /// Each field section's named slot filled by its heading's verbatim span.
    pub fields: BTreeMap<String, String>,
    /// Each **edge** section's slot mapped to its parsed address entries, in document
    /// order. A field section whose slot the kind marks as an edge field carries
    /// addresses, not a verbatim span, so its entries land here rather than in
    /// [`fields`](LayoutReading::fields) — `satisfies` among them.
    pub edges: BTreeMap<String, Vec<String>>,
    /// Each member collection's members, in document order — one per child heading of
    /// a collection heading.
    pub members: Vec<LayoutMember>,
    /// Each verbatim prose region's span, in document order (an importing prose region
    /// is out of this reader's scope and lands empty).
    pub prose: Vec<String>,
}

/// One member read off a layout document's member collection: its child kind, its
/// identity (the slugged heading, or the explicit key when the collection declares
/// one — surviving a retitle of the heading), the authored heading it was read from,
/// and its own leaves (the immediate deeper sub-headings' spans, keyed by slug).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMember {
    /// The child kind this member instantiates — the collection region's `member_kind`.
    pub member_kind: String,
    /// The member's identity: the slugged child heading, or the explicit key's value
    /// when the collection declares a `key` (stable across a heading retitle).
    pub key: String,
    /// The authored child heading this member was read from — its identity source when
    /// no explicit key overrides, kept so a rename surfaces as a move.
    pub heading: String,
    /// The member's own prose leaves — its immediate deeper sub-headings' spans, keyed
    /// by the slug of each sub-heading.
    pub leaves: BTreeMap<String, String>,
}

/// The failures a layout read surfaces loud — each naming the file and the heading at
/// fault, never a degraded empty reading (invariant 6: loud or nothing). An unfilled
/// region is not among them: a region states what may appear, never what must, so it
/// reads empty. What refuses is malformed structure the reader cannot place.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum LayoutError {
    /// A collection declares an explicit identity `key`, but a member carries no
    /// sub-heading of that name to read the key from.
    #[error(
        "{path}: collection member `{heading}` has no `{key}` sub-heading for its explicit key"
    )]
    #[diagnostic(code(temper::layout::missing_key))]
    MissingKey {
        /// The layout document at fault.
        path: PathBuf,
        /// The member heading missing the key sub-heading.
        heading: String,
        /// The declared key sub-heading name.
        key: String,
    },

    /// The document carries a top-level heading the layout's regions never admit —
    /// structure no primitive covers (`representation.md`: "two kinds, or it is prose").
    #[error("{path}: heading `{heading}` fits no declared layout region")]
    #[diagnostic(code(temper::layout::unadmitted))]
    Unadmitted {
        /// The layout document at fault.
        path: PathBuf,
        /// The unadmitted heading.
        heading: String,
    },
}

impl Layout {
    /// Read `body` under this declared layout, off the document at `source_path` (named
    /// only in diagnostics). The regions map in document order onto the body's top-level
    /// headings ([`extract::body_heading_tree`]): a field section fills its slot with
    /// the heading's verbatim span — unless the slot is one of `edge_fields`, when its
    /// entries are parsed as addresses into [`edges`](LayoutReading::edges) instead — a
    /// member collection turns each child heading into a member, and a verbatim prose
    /// region takes the document preamble. A region the document has no heading for reads
    /// empty — a region states what may appear, never what must — so one layout serves a
    /// tree ranging from prose-only to fully membered. An explicit key with no sub-heading,
    /// or a top-level heading no region admits, still refuses loud ([`LayoutError`]).
    ///
    /// # Errors
    ///
    /// Returns a [`LayoutError`] naming the file and heading when the document carries
    /// structure the declared layout cannot place.
    pub fn read(
        &self,
        body: &str,
        source_path: &Path,
        edge_fields: &BTreeSet<String>,
    ) -> Result<LayoutReading, LayoutError> {
        let tree = extract::body_heading_tree(body);
        let mut reading = LayoutReading::default();
        // The cursor into the document's top-level headings the field/collection regions
        // consume in order; a preamble taken once by the first verbatim prose region.
        let mut cursor = 0;
        let mut preamble_taken = false;
        for region in &self.regions {
            match region {
                LayoutRegion::Prose { import } => {
                    // An importing prose region resolves elsewhere (LAYOUT-PROSE-IMPORT);
                    // a verbatim one takes the document preamble, once. The blank lines
                    // that separate regions are structure, trimmed off the captured span.
                    let span = if import.is_none() && !preamble_taken {
                        preamble_taken = true;
                        extract::body_preamble(body).trim().to_string()
                    } else {
                        String::new()
                    };
                    reading.prose.push(span);
                }
                LayoutRegion::Field { slot } => {
                    // No heading left for the slot: it reads absent, not loud.
                    let Some(node) = next_heading(&tree, &mut cursor) else {
                        continue;
                    };
                    if edge_fields.contains(slot) {
                        reading
                            .edges
                            .insert(slot.clone(), parse_edge_entries(&node.body));
                    } else {
                        reading
                            .fields
                            .insert(slot.clone(), node.body.trim().to_string());
                    }
                }
                LayoutRegion::Collection { member_kind, key } => {
                    // No heading left for the collection: it reads with zero members.
                    let Some(node) = next_heading(&tree, &mut cursor) else {
                        continue;
                    };
                    for child in &node.children {
                        reading.members.push(read_collection_member(
                            child,
                            member_kind,
                            key,
                            source_path,
                        )?);
                    }
                }
            }
        }
        // Every top-level heading the regions did not consume is structure no primitive
        // admits — loud, never silently dropped.
        if let Some(extra) = tree.get(cursor) {
            return Err(LayoutError::Unadmitted {
                path: source_path.to_path_buf(),
                heading: extra.heading.clone(),
            });
        }
        Ok(reading)
    }
}

/// The next unconsumed top-level heading a field/collection region binds to, advancing
/// the cursor — or `None` when the document has run out of headings, leaving the region
/// to read empty.
fn next_heading<'a>(
    tree: &'a [extract::HeadingNode],
    cursor: &mut usize,
) -> Option<&'a extract::HeadingNode> {
    let node = tree.get(*cursor)?;
    *cursor += 1;
    Some(node)
}

/// Read one collection member off its child heading `node`: its identity is the
/// slugged heading, or — when the collection declares an explicit `key` — the slug of
/// the value under the member's `key` sub-heading, which a heading retitle leaves
/// untouched. Its leaves are the member's immediate sub-headings' spans, keyed by slug.
fn read_collection_member(
    node: &extract::HeadingNode,
    member_kind: &str,
    key: &Option<String>,
    source_path: &Path,
) -> Result<LayoutMember, LayoutError> {
    let mut leaves = BTreeMap::new();
    for child in &node.children {
        leaves.insert(slugify(&child.heading), child.body.trim().to_string());
    }
    let identity = match key {
        None => slugify(&node.heading),
        Some(key) => {
            let slug = slugify(key);
            let value = leaves.get(&slug).ok_or_else(|| LayoutError::MissingKey {
                path: source_path.to_path_buf(),
                heading: node.heading.clone(),
                key: key.clone(),
            })?;
            slugify(value)
        }
    };
    Ok(LayoutMember {
        member_kind: member_kind.to_string(),
        key: identity,
        heading: node.heading.clone(),
        leaves,
    })
}

/// Parse an edge slot's heading span into its ordered address entries: one entry per
/// non-empty line, a leading markdown list marker (`-`/`*`/`+`) and surrounding
/// whitespace stripped. The authored form a `satisfies` (or other edge-field) section
/// lists its targets as — bare requirement names for `satisfies`, `kind:name` addresses
/// for a declared relationship.
fn parse_edge_entries(span: &str) -> Vec<String> {
    span.lines()
        .map(|line| line.trim().trim_start_matches(['-', '*', '+']).trim())
        .filter(|entry| !entry.is_empty())
        .map(str::to_string)
        .collect()
}

/// Slugify a heading (or explicit-key value) into a stable identity token: ASCII
/// alphanumerics lower-cased, every other run collapsed to a single `-`, leading and
/// trailing `-` trimmed. The identity a member collection keys on when no explicit key
/// overrides — `## Baked projection` → `baked-projection`.
fn slugify(text: &str) -> String {
    let mut out = String::new();
    let mut pending_sep = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_sep && !out.is_empty() {
                out.push('-');
            }
            pending_sep = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            pending_sep = true;
        }
    }
    out
}
