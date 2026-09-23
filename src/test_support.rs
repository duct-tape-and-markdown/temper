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

/// An inert [`Features`](crate::extract::Features) carrying nothing but `id` — the base
/// every in-src fixture starts from, so a test spells only the columns it varies via
/// struct update: `Features { body_lines: 1, ..test_support::features(id) }`.
///
/// The rendered extents are `Some(0)`, not `None`: an `extent` clause reads the `Some`,
/// and a fixture that wants the undecidable case says so by overriding.
pub(crate) fn features(id: &str) -> crate::extract::Features {
    crate::extract::Features {
        id: id.to_string(),
        fields: std::collections::BTreeMap::new(),
        body_lines: 0,
        rendered_lines: Some(0),
        rendered_chars: Some(0),
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: None,
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: Vec::new(),
        edge_placements: None,
    }
}

/// A [`KindFactRow`](crate::drift::KindFactRow) naming `name` and declaring nothing else
/// — the other fourteen columns absent — so an in-src fixture spells only the facts its
/// case exercises via struct update:
/// `KindFactRow { unit_shape: Some("file".into()), ..test_support::kind_fact_row("skill") }`.
///
/// The in-src home of the shape `tests/common`'s `kind_facts` holds for the integration
/// suites; a case declaring a `governs` locus overrides the two locus columns.
pub(crate) fn kind_fact_row(name: &str) -> crate::drift::KindFactRow {
    crate::drift::KindFactRow {
        name: name.to_string(),
        provider: None,
        governs_root: None,
        governs_glob: None,
        commitment: None,
        format: None,
        unit_shape: None,
        registration: Vec::new(),
        templates: Vec::new(),
        content: None,
        shape: None,
        leaves: Vec::new(),
        collection_address: None,
        guidance: None,
        cite: None,
    }
}

/// A [`ClauseRow`](crate::drift::ClauseRow) carrying `predicate` at `severity`, every
/// other column at its default — the base an in-src fixture struct-updates with the one
/// argument column its case exercises (`bound`/`section`/`count`/…).
///
/// Payload-shaped: `label` is `None`, the stamp emit itself writes. A case asserting
/// about a *lock*-shaped row spells the label it judges.
pub(crate) fn clause_row(predicate: &str, severity: &str) -> crate::drift::ClauseRow {
    crate::drift::ClauseRow {
        label: None,
        kind: None,
        predicate: predicate.to_string(),
        field: None,
        severity: severity.to_string(),
        guidance: None,
        cite: None,
        count: None,
        target: None,
        degree: None,
        fields: None,
        gate: None,
        value_type: None,
        shape: None,
        bound: None,
        unit: None,
        charset: None,
        keys: None,
        values: None,
        range: None,
        section: None,
        sections: None,
        guard_predicate: None,
        body: None,
    }
}
