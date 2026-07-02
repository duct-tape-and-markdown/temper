//! The embedded built-in kind std-lib.
//!
//! specs/15-kinds.md, "A built-in kind is an adapter". `temper` ships the read-side
//! definitions of the known-harness kinds (`skill`, `rule`) as first-party product
//! source in a `kinds/<name>/KIND.md` tree at the repo root — the same medium and
//! schema as a project's own `.temper/kinds/<name>/KIND.md`, differing only in where
//! it sources from (temper-maintained vs author-maintained; ownership, not
//! mechanism). `build.rs` walks that tree and generates the [`BUILTIN_KINDS`] table
//! this module re-exports, so a built-in kind resolves from the embedded set with no
//! on-disk configuration.
//!
//! A built-in kind's definition parses into a [`CustomKind`] through the very same
//! [`CustomKind::from_header`] path a project-authored `KIND.md` does, and is
//! validated as any kind is — this only sources the header from embedded product
//! data instead of `.temper/kinds/`.

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::document::Document;
use crate::kind::{CustomKind, KindError};

// The generated `pub static BUILTIN_KINDS: &[(&str, &str)]` — every embedded kind as
// `(name, KIND.md source)`, sorted by name. `build.rs` writes it into `$OUT_DIR` from
// the `kinds/` tree; including it here re-exports it as
// `crate::builtin_kind::BUILTIN_KINDS`.
include!(concat!(env!("OUT_DIR"), "/builtin_kinds.rs"));

/// The embedded `KIND.md` source for a built-in kind, or `None` if no kind of that
/// name is embedded.
#[must_use]
pub fn source(name: &str) -> Option<&'static str> {
    BUILTIN_KINDS
        .iter()
        .find(|(embedded, _)| *embedded == name)
        .map(|(_, src)| *src)
}

/// Parse the named built-in kind's embedded `KIND.md` into its [`CustomKind`], or
/// `None` if no kind of that name is embedded. The `+++`-fenced header is parsed
/// through the same [`CustomKind::from_header`] a project-authored definition is; the
/// synthetic `kinds/<name>/KIND.md` path labels any diagnostic.
///
/// # Errors
///
/// Returns a [`KindError`] when the embedded `KIND.md` is not a well-formed fenced
/// document, or its header is not an admissible kind definition (a bad `governs`, an
/// out-of-vocabulary extraction primitive, a stray key).
pub fn definition(name: &str) -> Result<Option<CustomKind>, KindError> {
    match source(name) {
        None => Ok(None),
        Some(src) => Ok(Some(parse(name, src)?)),
    }
}

/// Parse every embedded built-in kind into a `name → CustomKind` map — the built-in
/// read-side set, the mirror of [`crate::builtin::contracts`] on the require-side.
///
/// # Errors
///
/// Returns a [`KindError`] if any embedded `KIND.md` fails to parse into an
/// admissible [`CustomKind`].
pub fn definitions() -> Result<BTreeMap<String, CustomKind>, KindError> {
    let mut map = BTreeMap::new();
    for (name, src) in BUILTIN_KINDS {
        map.insert((*name).to_string(), parse(name, src)?);
    }
    Ok(map)
}

/// Parse one embedded `KIND.md` source into a [`CustomKind`] named `name`. The
/// synthetic `kinds/<name>/KIND.md` path gives diagnostics the same shape the on-disk
/// loader's carry.
fn parse(name: &str, src: &str) -> Result<CustomKind, KindError> {
    let path = PathBuf::from("kinds").join(name).join("KIND.md");
    let document = Document::parse(src).map_err(|source| KindError::Document {
        path: path.clone(),
        source,
    })?;
    CustomKind::from_header(document.header().as_table(), name, &path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::Edge;
    use crate::kind::{Governs, Primitive};

    #[test]
    fn skill_definition_matches_the_hand_authored_kind() {
        let skill = definition("skill").unwrap().expect("skill is embedded");

        assert_eq!(skill.name, "skill");
        assert_eq!(
            skill.governs,
            Governs {
                root: ".claude/skills".to_string(),
                glob: "*/SKILL.md".to_string(),
            }
        );
        // The composed extractor mirrors `kinds/skill/KIND.md`: the four documented
        // frontmatter fields, then the markdown-structure primitives, in order.
        assert_eq!(
            skill.extraction.primitives(),
            &[
                Primitive::Field {
                    key: "name".to_string()
                },
                Primitive::Field {
                    key: "description".to_string()
                },
                Primitive::Field {
                    key: "version".to_string()
                },
                Primitive::Field {
                    key: "license".to_string()
                },
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]
        );
        // The built-in `skill` kind declares no relationships.
        assert_eq!(skill.relationships, Vec::<Edge>::new());
    }

    #[test]
    fn rule_definition_matches_the_hand_authored_kind() {
        let rule = definition("rule").unwrap().expect("rule is embedded");

        assert_eq!(rule.name, "rule");
        assert_eq!(
            rule.governs,
            Governs {
                root: ".claude/rules".to_string(),
                glob: "*.md".to_string(),
            }
        );
        assert_eq!(
            rule.extraction.primitives(),
            &[
                Primitive::Field {
                    key: "paths".to_string()
                },
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
            ]
        );
        assert_eq!(rule.relationships, Vec::<Edge>::new());
    }

    #[test]
    fn an_unknown_kind_name_is_none() {
        assert!(definition("spec").unwrap().is_none());
        assert!(source("spec").is_none());
    }

    #[test]
    fn definitions_carries_exactly_the_two_built_in_kinds() {
        let all = definitions().unwrap();
        assert_eq!(all.keys().collect::<Vec<_>>(), vec!["rule", "skill"]);
    }
}
