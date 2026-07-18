//! `shape` — the clause naming one engine-implemented form from a closed set, judged
//! through the engine that decides it.
//!
//! Four properties, and the closure is the load-bearing one: each shipped shape decides
//! its documented rule over a real member; an unknown shape name is **refused at load**
//! rather than skipped into a contract that checks less than it says; and no surface
//! anywhere accepts an author-written pattern — the whole point of naming a shape instead
//! of spelling one.

use temper::check::{Diagnostic, Severity};
use temper::contract::{self, Clause, Contract, Predicate, Severity as ClauseSeverity, Shape};
use temper::drift::ClauseRow;
use temper::engine;
use temper::extract::Features;

mod common;

/// A skill-shaped member carrying `name` and `description` — the two fields the shipped
/// shape clauses range over.
fn skill(name: &str, description: &str) -> Features {
    Features {
        id: name.to_string(),
        fields: [
            ("name".to_string(), name.into()),
            ("description".to_string(), description.into()),
        ]
        .into_iter()
        .collect(),
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

/// A contract of one `shape` clause on `field`, addressed as a lifted row would be.
fn shaped(field: &str, shape: Shape) -> Contract {
    let predicate = Predicate::Shape {
        field: field.to_string(),
        shape,
    };
    Contract {
        name: "skill".to_string(),
        guidance: None,
        clauses: vec![Clause {
            label: contract::clause_label(Some("skill"), predicate.key(), predicate.target()),
            severity: ClauseSeverity::Required,
            predicate,
            guidance: None,
            source: None,
        }],
    }
}

/// Each finding's message.
fn messages(diagnostics: &[Diagnostic]) -> Vec<&str> {
    diagnostics.iter().map(|d| d.message.as_str()).collect()
}

#[test]
fn a_hyphen_that_leads_trails_or_doubles_is_a_finding() {
    let contract = shaped("name", Shape::HyphenPlacement);
    for name in ["-pdf", "pdf-", "pdf--processing"] {
        let member = skill(name, "Extracts text from PDFs. Use when handling PDFs.");
        let diagnostics = engine::validate(&contract, std::slice::from_ref(&member));
        assert_eq!(
            messages(&diagnostics),
            vec![
                "field `name` does not hold the `hyphen-placement` shape: a hyphen may not \
                 lead, trail, or double — the value is segments joined by single hyphens"
            ],
            "`{name}` misplaces its hyphen"
        );
        // The finding names the clause that produced it, at the declared severity.
        assert_eq!(diagnostics[0].rule, "skill.shape.name");
        assert_eq!(diagnostics[0].severity, Severity::Error);
    }
}

#[test]
fn a_clean_slug_holds_the_hyphen_shape() {
    let contract = shaped("name", Shape::HyphenPlacement);
    // The spec's own valid examples, plus the hyphenless case and the empty value that
    // `min_len` — never this shape — is the clause for.
    for name in [
        "pdf-processing",
        "data-analysis",
        "code-review",
        "helper",
        "",
    ] {
        let member = skill(name, "Does a thing. Use when the thing is wanted.");
        assert!(
            engine::validate(&contract, std::slice::from_ref(&member)).is_empty(),
            "`{name}` places every hyphen legally"
        );
    }
}

#[test]
fn a_description_carrying_an_xml_tag_is_a_finding() {
    let contract = shaped("description", Shape::NoXmlTags);
    for description in [
        "Reviews code. <instructions>Use when reviewing.</instructions>",
        "Renders a page.<br/> Use when rendering.",
        "Fills forms. Use when the user says <thing href=\"x\">.",
    ] {
        let member = skill("code-review", description);
        let diagnostics = engine::validate(&contract, std::slice::from_ref(&member));
        assert_eq!(
            messages(&diagnostics),
            vec![
                "field `description` does not hold the `no-xml-tags` shape: the value \
                 carries no XML tag"
            ],
            "`{description}` carries a tag"
        );
        assert_eq!(diagnostics[0].rule, "skill.shape.description");
    }
}

#[test]
fn prose_spelling_a_comparison_is_not_a_tag() {
    // The declared leniency runs this way on purpose: a tag is read as well-formed, so
    // ordinary prose never forges a finding. A clause that gates cannot false-fire.
    let contract = shaped("description", Shape::NoXmlTags);
    for description in [
        "Compares sizes. Use when x < y and y > z.",
        "Diffs numbers. Use when 3<4 is asserted.",
        "Fetches a page. Use when the user pastes <https://example.com>.",
        "Extracts text and tables from PDF files. Use when working with PDFs.",
    ] {
        let member = skill("pdf-processing", description);
        assert!(
            engine::validate(&contract, std::slice::from_ref(&member)).is_empty(),
            "`{description}` spells no tag"
        );
    }
}

#[test]
fn a_lock_row_naming_a_shape_the_enum_does_not_carry_is_refused_at_load() {
    let mut row = common::clause("shape", "required");
    row.field = Some("name".to_string());

    // The closure is the safety property: a name the engine does not implement is no
    // predicate at all, so the row is rejected rather than skipped into a contract that
    // silently checks less than its own rows say.
    for unknown in ["kebab-case", "semver", "no-xml-tag", ""] {
        row.shape = Some(unknown.to_string());
        assert_eq!(
            contract::predicate_from_row(&row),
            None,
            "`{unknown}` names no member of the closed set"
        );
    }

    // A row naming no shape at all is refused the same way — the argument is not optional.
    row.shape = None;
    assert_eq!(contract::predicate_from_row(&row), None);

    // And the two the engine does implement lift, so the refusal is the enum's edge and
    // not a decoder that rejects everything.
    for (name, shape) in [
        ("hyphen-placement", Shape::HyphenPlacement),
        ("no-xml-tags", Shape::NoXmlTags),
    ] {
        row.shape = Some(name.to_string());
        assert_eq!(
            contract::predicate_from_row(&row),
            Some(Predicate::Shape {
                field: "name".to_string(),
                shape,
            })
        );
    }
}

/// Refusal is the *clause's*, not merely the predicate decoder's: an unknown shape fails
/// the whole row's lift loudly, which is what keeps a bad lock from loading at all.
#[test]
fn a_row_naming_an_unknown_shape_fails_the_clause_lift_loudly() {
    let mut row: ClauseRow = common::clause("shape", "required");
    row.label = Some("skill.shape.name".to_string());
    row.field = Some("name".to_string());
    row.shape = Some("kebab-case".to_string());

    assert!(
        temper::compose::clause_from_row(&row).is_err(),
        "an unknown shape is a load error, never a dropped clause"
    );
}

/// No surface accepts an author-written pattern. The vocabulary carries no `pattern`
/// predicate key, and a `shape` row's argument is a *name* the engine looks up — there is
/// nowhere to spell an expression, which is precisely what a closed enum buys.
#[test]
fn no_clause_surface_accepts_an_author_written_pattern() {
    let mut row = common::clause("pattern", "required");
    row.field = Some("name".to_string());
    assert_eq!(
        contract::predicate_from_row(&row),
        None,
        "`pattern` is no key of the closed vocabulary"
    );

    // Nor does the `shape` key admit one through its own argument: a regex handed to the
    // shape column is just an unknown name, refused with every other one.
    let mut smuggled = common::clause("shape", "required");
    smuggled.field = Some("name".to_string());
    smuggled.shape = Some("^[a-z]+$".to_string());
    assert_eq!(contract::predicate_from_row(&smuggled), None);

    // The engine's own expressions stay engine-side: a shape names them, never carries
    // them, so the two shipped shapes' arguments are their names alone.
    assert_eq!(Shape::HyphenPlacement.name(), "hyphen-placement");
    assert_eq!(Shape::NoXmlTags.name(), "no-xml-tags");
    assert_eq!(Shape::from_name("^[a-z]+$"), None);
}

/// The lock is not the only closed door: the SDK's `shape()` ctor takes the same closed
/// set, and the generated `Shape` union is the type an author is checked against. This
/// pins the seam the two sides share — a member added on one side alone is a build error
/// on the other.
#[test]
fn the_generated_seam_union_is_the_engines_own_closed_set() {
    let generated = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("sdk/src/generated/Shape.ts"),
    )
    .expect("the seam exports the shape union");

    assert!(
        generated.contains(
            r#"export type Shape = "hyphen-placement" | "no-xml-tags" | "leading-dot-slash";"#
        ),
        "the SDK's shape vocabulary is the engine's enum, generated across — got:\n{generated}"
    );
}
