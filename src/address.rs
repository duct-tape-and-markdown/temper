//! Field addressing — the path a clause's `field` spells, and its declared bound.
//!
//! A clause names the value it decides over by an **addressing path**: a run of name
//! segments (`owner.name`) with `[*]`, the each-grain over an array's elements
//! (`plugins[*].source`). That surface is a deliberately small subset of RFC 9535, and
//! the subset is the point: [`FieldPath::parse`] refuses anything beyond it — a filter,
//! a slice, an index, a recursive descent — so the RFC engine underneath stays hidden
//! mechanics and never becomes an author-facing pattern language.
//!
//! Evaluation itself is [`serde_json_path`]'s: a parsed path compiles to the normalized
//! RFC 9535 query its segments mean, and locating is the crate's. Nothing here walks a
//! JSON tree by hand.

use std::fmt::Write as _;
use std::path::{Component, Path, PathBuf};

use serde_json::Value as JsonValue;
use serde_json_path::JsonPath;

/// One step of an addressing path.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Step {
    /// A name segment — the key it addresses on the object in hand.
    Name(String),
    /// `[*]` — each element of the array in hand, the each-grain.
    EachElement,
}

/// A clause's parsed **addressing path**: the steps the author spelled, and the RFC 9535
/// query they compile to.
///
/// Construct through [`FieldPath::parse`], which is also the admissibility gate — a
/// `FieldPath` in hand is inside the declared subset by construction.
#[derive(Debug, Clone)]
pub struct FieldPath {
    /// The author's own spelling, the form every diagnostic names.
    spelling: String,
    steps: Vec<Step>,
    query: JsonPath,
}

impl PartialEq for FieldPath {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps
    }
}

impl Eq for FieldPath {}

impl FieldPath {
    /// Parse `spelling` into an addressing path, or return the refusal message naming why
    /// it falls outside the declared subset.
    ///
    /// # Errors
    ///
    /// Returns the diagnostic message when `spelling` uses anything beyond name segments
    /// and `[*]`, or when it is empty.
    pub fn parse(spelling: &str) -> Result<FieldPath, String> {
        let steps = steps_of(spelling)?;
        let rendered = query_of(&steps);
        let query = JsonPath::parse(&rendered).map_err(|error| {
            format!(
                "field path `{spelling}` does not compile to a query over the member's \
                 fields: {error}"
            )
        })?;
        Ok(FieldPath {
            spelling: spelling.to_string(),
            steps,
            query,
        })
    }

    /// Whether this path is a **lone name segment** — the flat field name that is the
    /// whole surface every clause spelled before the subset widened, and the only shape a
    /// JSON Schema property can carry.
    #[must_use]
    pub fn is_bare_name(&self) -> bool {
        matches!(self.steps.as_slice(), [Step::Name(_)])
    }

    /// This path's author-facing spelling.
    #[must_use]
    #[allow(dead_code)]
    fn spelling(&self) -> &str {
        &self.spelling
    }

    /// The **top-level key** this path is rooted at — `owner` for `owner.name`, `plugins`
    /// for `plugins[*].source`, the name itself for a bare name.
    ///
    /// Every admissible path opens on a name segment (a bracket selector with nothing
    /// before it is refused), so this is always `Some` for a parsed path; the `Option` is
    /// the grammar's own shape rather than a case a caller can hit.
    #[must_use]
    pub fn head_name(&self) -> Option<&str> {
        match self.steps.first()? {
            Step::Name(name) => Some(name),
            Step::EachElement => None,
        }
    }

    /// The path's trailing name segment paired with the path to its **parent** — the
    /// decomposition a presence check needs, since a key that is absent locates no node
    /// to ask about.
    ///
    /// `None` when the path ends in `[*]`, which names elements rather than a key.
    #[must_use]
    pub fn split_leaf(&self) -> Option<(FieldPath, &str)> {
        let (Step::Name(leaf), parent) = self.steps.split_last()? else {
            return None;
        };
        let steps = parent.to_vec();
        let rendered = query_of(&steps);
        // The parent of an admissible path is admissible: it is this path's own steps,
        // one fewer, so the query it renders is one this crate already parsed.
        let query = JsonPath::parse(&rendered).ok()?;
        Some((
            FieldPath {
                spelling: parent_spelling(&self.spelling, leaf),
                steps,
                query,
            },
            leaf.as_str(),
        ))
    }

    /// Every node this path locates in `root`, each paired with the concrete address it
    /// resolved to (`plugins[0].source`) — one node for a path of name segments, one per
    /// element under each `[*]`, in document order.
    ///
    /// Empty when the path resolves nowhere: a missing key, or a value with no such reach
    /// met before the leaf. Absent, never errored — the same silence a misspelled flat
    /// field name has always had.
    #[must_use]
    pub fn locate<'v>(&self, root: &'v JsonValue) -> Vec<(String, &'v JsonValue)> {
        self.query
            .query_located(root)
            .into_iter()
            .map(|node| (render_address(node.location()), node.node()))
            .collect()
    }
}

/// One located node's address in the author's own spelling — `plugins[0].source`, never
/// the RFC's normalized `$['plugins'][0]['source']`. The engine is hidden mechanics; a
/// finding that leaked its notation would make it the author's problem.
fn render_address(location: &serde_json_path::NormalizedPath) -> String {
    let mut out = String::new();
    for element in location.iter() {
        match element {
            serde_json_path::PathElement::Name(name) => {
                if !out.is_empty() {
                    out.push('.');
                }
                out.push_str(name);
            }
            serde_json_path::PathElement::Index(index) => {
                let _ = write!(out, "[{index}]");
            }
        }
    }
    out
}

/// The parent path's spelling — `spelling` with its trailing `leaf` name segment (and the
/// `.` that separated it) cut. Empty for a lone name, which is the root itself.
fn parent_spelling(spelling: &str, leaf: &str) -> String {
    let head = &spelling[..spelling.len() - leaf.len()];
    head.strip_suffix('.').unwrap_or(head).to_string()
}

/// Tokenize `spelling` into its steps, or return the refusal naming what put it outside
/// the declared subset.
fn steps_of(spelling: &str) -> Result<Vec<Step>, String> {
    let chars: Vec<char> = spelling.chars().collect();
    let mut steps = Vec::new();
    let mut i = 0;
    loop {
        // A name segment runs to the next `.` or `[`, so those two characters are the
        // whole grammar's punctuation and a name is whatever the format allows as a key
        // (`disable-model-invocation` is one).
        let start = i;
        while i < chars.len() && chars[i] != '.' && chars[i] != '[' {
            i += 1;
        }
        let name: String = chars[start..i].iter().collect();
        if name.is_empty() {
            return Err(empty_segment(spelling, &chars, i));
        }
        if name == "*" {
            return Err(refusal(
                spelling,
                "spells a wildcard as a bare `*` segment; the subset's each-grain over an \
                 array's elements is `[*]`",
            ));
        }
        steps.push(Step::Name(name));

        // Zero or more `[*]` — an array of arrays grains twice.
        while i < chars.len() && chars[i] == '[' {
            if chars.get(i + 1) == Some(&'*') && chars.get(i + 2) == Some(&']') {
                steps.push(Step::EachElement);
                i += 3;
            } else {
                return Err(refusal(
                    spelling,
                    "uses a bracket selector other than `[*]`; an index, a slice, a \
                     filter, and a quoted name are all outside the subset",
                ));
            }
        }

        match chars.get(i) {
            None => return Ok(steps),
            Some('.') => i += 1,
            Some(_) => {
                return Err(refusal(
                    spelling,
                    "runs a name segment straight onto a `[*]`; separate the two with a `.`",
                ));
            }
        }
    }
}

/// The refusal for an empty name segment at `i` — recursive descent when a second `.`
/// produced it, else a malformed path.
fn empty_segment(spelling: &str, chars: &[char], i: usize) -> String {
    if chars.get(i) == Some(&'.') {
        return refusal(
            spelling,
            "uses recursive descent (`..`), which is outside the subset",
        );
    }
    if chars.get(i) == Some(&'[') {
        return refusal(
            spelling,
            "opens a bracket selector with no name segment before it; a path is rooted at \
             a field the member declares",
        );
    }
    refusal(spelling, "has an empty name segment")
}

/// One out-of-subset diagnostic, in the words every such finding shares: what the path
/// did, and the bound it stepped past.
fn refusal(spelling: &str, because: &str) -> String {
    format!(
        "field path `{spelling}` {because}. A clause addresses a field by name segments \
         (`owner.name`) and `[*]`, each element of an array (`plugins[*].source`) — \
         nothing else"
    )
}

/// The RFC 9535 query `steps` mean: each name segment a double-quoted name selector (so a
/// key the shorthand cannot spell — `disable-model-invocation` — needs no special case),
/// each `[*]` the wildcard selector.
fn query_of(steps: &[Step]) -> String {
    let mut query = String::from("$");
    for step in steps {
        match step {
            // `serde_json` writes the RFC's own double-quoted string literal: `"` and `\`
            // escaped, control characters as `\uXXXX`.
            Step::Name(name) => {
                let _ = write!(query, "[{}]", JsonValue::String(name.clone()));
            }
            Step::EachElement => query.push_str("[*]"),
        }
    }
    query
}

/// Lexically normalize a path — drop `.` and resolve `..` against a preceding normal
/// segment — **without touching disk**: a provenance path need not exist under the
/// check CWD, and both the index keys and a resolved target must normalize the identical
/// way to join. A leading `..` with nothing to pop is kept, so an out-of-tree target
/// stays distinct rather than silently rooting.
#[must_use]
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut out: Vec<Component> = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir if matches!(out.last(), Some(Component::Normal(_))) => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The addresses and values `spelling` locates in `root`.
    fn locate(spelling: &str, root: &JsonValue) -> Vec<(String, JsonValue)> {
        FieldPath::parse(spelling)
            .expect("the path is inside the subset")
            .locate(root)
            .into_iter()
            .map(|(address, value)| (address, value.clone()))
            .collect()
    }

    #[test]
    fn a_bare_name_locates_the_top_level_field_and_nothing_else() {
        let root = json!({"name": "demo", "owner": {"name": "acme"}});
        assert_eq!(
            locate("name", &root),
            vec![("name".to_string(), json!("demo"))]
        );
        // A key the RFC's name shorthand cannot spell is an ordinary name segment here.
        let hyphenated = json!({"disable-model-invocation": true});
        assert_eq!(
            locate("disable-model-invocation", &hyphenated),
            vec![("disable-model-invocation".to_string(), json!(true))]
        );
    }

    #[test]
    fn a_name_path_walks_into_a_nested_object() {
        let root = json!({"owner": {"name": "acme", "email": "a@b.c"}});
        assert_eq!(
            locate("owner.name", &root),
            vec![("owner.name".to_string(), json!("acme"))]
        );
    }

    #[test]
    fn each_element_grains_over_an_array_and_addresses_each_by_index() {
        let root = json!({"plugins": [{"source": "./a"}, {"source": "./b"}, {}]});
        assert_eq!(
            locate("plugins[*].source", &root),
            vec![
                ("plugins[0].source".to_string(), json!("./a")),
                ("plugins[1].source".to_string(), json!("./b")),
            ]
        );
    }

    #[test]
    fn an_unresolved_path_locates_nothing_rather_than_erroring() {
        let root = json!({"owner": "acme", "plugins": []});
        // A missing key, a scalar met before the leaf, and an empty array each locate no
        // node — absent, never errored.
        for spelling in ["absent", "owner.name", "plugins[*].source"] {
            assert!(
                locate(spelling, &root).is_empty(),
                "{spelling} locates none"
            );
        }
    }

    #[test]
    fn everything_past_the_subset_is_refused_with_the_bound_named() {
        for spelling in [
            "plugins[0].source",
            "plugins[1:2]",
            "plugins[?@.source]",
            "plugins['name']",
            "plugins[*",
            "owner..name",
            "owner.",
            ".owner",
            "",
            "plugins.*",
            "plugins[*]source",
            "[0]",
        ] {
            let refusal = FieldPath::parse(spelling).expect_err("outside the subset");
            assert!(
                refusal.contains("`owner.name`") && refusal.contains("`plugins[*].source`"),
                "the refusal names the subset, got: {refusal}"
            );
        }
    }

    #[test]
    fn split_leaf_names_the_parent_and_the_key_a_presence_check_asks_for() {
        let bare = FieldPath::parse("name").expect("inside the subset");
        let (parent, leaf) = bare.split_leaf().expect("a name path splits");
        assert_eq!(leaf, "name");
        assert_eq!(parent.spelling(), "");
        // The parent of a bare name is the field map itself, so it locates the root.
        let root = json!({"name": "demo"});
        assert_eq!(parent.locate(&root).len(), 1);

        let nested = FieldPath::parse("plugins[*].source").expect("inside the subset");
        let (parent, leaf) = nested.split_leaf().expect("a name path splits");
        assert_eq!(leaf, "source");
        assert_eq!(parent.spelling(), "plugins[*]");

        // A path ending in `[*]` names elements, not a key — nothing to ask presence of.
        let each = FieldPath::parse("plugins[*]").expect("inside the subset");
        assert!(each.split_leaf().is_none());
    }

    #[test]
    fn a_bare_name_is_distinguished_from_a_path_that_steps_deeper() {
        assert!(FieldPath::parse("name").expect("parses").is_bare_name());
        for spelling in ["owner.name", "plugins[*]", "plugins[*].source"] {
            assert!(
                !FieldPath::parse(spelling).expect("parses").is_bare_name(),
                "{spelling} steps past the top level"
            );
        }
    }
}
