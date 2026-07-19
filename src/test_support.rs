//! Shared scaffolding for in-src `#[cfg(test)] mod tests` blocks.

use std::path::PathBuf;

use crate::kind::{CustomKind, Extraction, Format, Governs, Primitive, Unit, UnitShape};

/// A fresh, empty temp directory, uniquely named via the sanctioned `tempfile`
/// crate rather than a hand-rolled counter+pid scheme.
pub(crate) fn tmpdir(label: &str) -> PathBuf {
    tempfile::Builder::new()
        .prefix(label)
        .tempdir()
        .expect("failed to create temp dir")
        .keep()
}

/// A synthetic directory-shaped `yaml-frontmatter` kind with declared fields
/// matching the real skill kind's schema: `name`, `description`, `license` in
/// declaration order. Unknown keys are preserved sorted, so tests driving the
/// frontmatter adapter verify the projection without depending on the real
/// `builtin_kind::definition`.
#[allow(dead_code)]
pub(crate) fn skill_kind() -> CustomKind {
    CustomKind {
        format: Some(Format::YamlFrontmatter),
        unit_shape: Some(UnitShape::Directory),
        ..CustomKind::new(
            "test-skill",
            Governs {
                root: ".".to_string(),
                glob: "*/*".to_string(),
            },
            Extraction::new(vec![
                Primitive::Field {
                    key: "name".to_string(),
                },
                Primitive::Field {
                    key: "description".to_string(),
                },
                Primitive::Field {
                    key: "license".to_string(),
                },
            ]),
        )
    }
}

/// A synthetic file-shaped `yaml-frontmatter` kind with a declared `paths` field,
/// matching the real rule kind's schema. Tests driving the frontmatter adapter
/// verify the projection without depending on the real `builtin_kind::definition`.
#[allow(dead_code)]
pub(crate) fn rule_kind() -> CustomKind {
    CustomKind {
        format: Some(Format::YamlFrontmatter),
        unit_shape: Some(UnitShape::File),
        ..CustomKind::new(
            "test-rule",
            Governs {
                root: ".".to_string(),
                glob: "*".to_string(),
            },
            Extraction::new(vec![Primitive::Field {
                key: "paths".to_string(),
            }]),
        )
    }
}

/// Lift an imported [`Member`](crate::frontmatter::Member) straight into the raw [`Unit`]
/// the composed extractor reads — the same fields a built-in kind's member carries
/// into `check`, with no disk round trip.
#[allow(dead_code)]
pub(crate) fn surface_unit(member: &crate::frontmatter::Member) -> Unit {
    Unit {
        id: member.id.clone(),
        frontmatter: member.fields.iter().cloned().collect(),
        body: member.body.clone(),
        source_path: member.provenance.source_path.clone(),
        satisfies: member
            .satisfies
            .iter()
            .map(|s| s.requirement.clone())
            .collect(),
        satisfies_clauses: member.satisfies.clone(),
    }
}
