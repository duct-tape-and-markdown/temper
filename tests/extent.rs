//! `extent` — the render-side size budget, judged through the engine that decides it.
//!
//! Five properties, and they are the whole bargain: an each-grain budget fires on one
//! member's own rendered extent past the bound and holds within it; a whole-grain budget
//! fires on the selection's *summed* rendered extent; both grains measure in either unit
//! (lines, characters); measurement is render-side — a projection over the bound fires
//! even where the source body count would pass, the axis `max_lines` was retired for; and
//! the closed vocabulary refuses a lock still carrying the retired `max_lines`, and an
//! `extent` row naming an unknown unit, at load.

use temper::compose::{self, ClauseRowError};
use temper::contract::{self, Clause, Contract, ExtentUnit, Predicate, Severity as ClauseSeverity};
use temper::drift::{BoundRow, ClauseRow};
use temper::engine::{self, Selection, Selector};
use temper::extract::Features;

/// A member whose render-side extent is `(lines, chars)` and whose source body count is
/// `body_lines` — the three kept distinct on purpose so a test can drive the render/source
/// divergence directly.
fn member(id: &str, lines: usize, chars: usize, body_lines: usize) -> Features {
    Features {
        id: id.to_string(),
        fields: Default::default(),
        body_lines,
        rendered_lines: Some(lines),
        rendered_chars: Some(chars),
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

/// An `extent` clause at the declared grain, addressed as a lifted row would be.
fn clause(unit: ExtentUnit, max: usize, whole: bool) -> Clause {
    let predicate = Predicate::Extent { unit, max, whole };
    Clause {
        label: contract::clause_label(Some("skill"), predicate.key(), predicate.target()),
        severity: ClauseSeverity::Advisory,
        predicate,
        guidance: None,
        source: None,
    }
}

/// A contract binding one `extent` clause to `skill`.
fn contract(unit: ExtentUnit, max: usize, whole: bool) -> Contract {
    Contract {
        name: "skill".to_string(),
        guidance: None,
        clauses: vec![clause(unit, max, whole)],
    }
}

/// A kind selection over `members`, carrying the one whole-grain `extent` clause `judge`
/// decides.
fn selection<'a>(unit: ExtentUnit, max: usize, members: &'a [Features]) -> Selection<'a> {
    Selection {
        selector: Selector::Kind("skill".to_string()),
        clauses: vec![clause(unit, max, true)],
        members: members.iter().map(|f| ("skill", f)).collect(),
    }
}

#[test]
fn each_grain_fires_past_the_bound_and_holds_within_it_in_both_units() {
    // Lines: 501 rendered lines trips a 500-line budget; exactly 500 is "at most" and
    // holds.
    let over = member("over", 501, 0, 501);
    let at = member("at", 500, 0, 500);
    let lines = contract(ExtentUnit::Lines, 500, false);
    assert_eq!(
        engine::validate(&lines, std::slice::from_ref(&over)).len(),
        1
    );
    assert!(engine::validate(&lines, std::slice::from_ref(&at)).is_empty());

    // Characters: the same algebra over the character count — the second unit measures a
    // budget lines cannot express (a member of few long lines).
    let wide = member("wide", 1, 4001, 1);
    let chars = contract(ExtentUnit::Characters, 4000, false);
    assert_eq!(
        engine::validate(&chars, std::slice::from_ref(&wide)).len(),
        1
    );
    let narrow = member("narrow", 1, 4000, 1);
    assert!(engine::validate(&chars, std::slice::from_ref(&narrow)).is_empty());
}

#[test]
fn whole_grain_bounds_the_selections_summed_extent_in_both_units() {
    // Three members each under the line budget on their own, but their summed rendered
    // extent (120 + 120 + 120 = 360) exceeds the ambient 300-line ceiling — the budget the
    // grain axis gives for free.
    let members = [
        member("a", 120, 900, 120),
        member("b", 120, 900, 120),
        member("c", 120, 900, 120),
    ];
    let over_lines = selection(ExtentUnit::Lines, 300, &members);
    assert_eq!(engine::judge(&[over_lines]).len(), 1);

    // Summed under the bound holds.
    let under_lines = selection(ExtentUnit::Lines, 400, &members);
    assert!(engine::judge(&[under_lines]).is_empty());

    // The same summation in characters: 2700 total over a 2000-character ambient budget.
    let over_chars = selection(ExtentUnit::Characters, 2000, &members);
    assert_eq!(engine::judge(&[over_chars]).len(), 1);
    let under_chars = selection(ExtentUnit::Characters, 3000, &members);
    assert!(engine::judge(&[under_chars]).is_empty());
}

#[test]
fn an_extent_clause_bound_to_an_embedded_kind_judges_the_captured_span() {
    // 0035's load-bearing case: an embedded kind's members carry a rendered span captured
    // at emit, so `extent` is admissible over the embedded locus rather than fenced bodyless
    // — the fence that once let a hardcoded zero pass every budget. The judging that follows
    // is the file-side algebra unchanged: one `extent`, one type, decided over a captured
    // projection whether the member owns a file or is composed into a host body.
    let lines_budget = contract(ExtentUnit::Lines, 500, false);
    assert!(
        engine::admissibility(
            &lines_budget,
            &engine::Locus::Embedded("citation".to_string())
        )
        .is_empty(),
        "a captured span makes `extent` decidable over an embedded kind",
    );

    // Each grain, both units: a composed embedded member over its budget is a finding, under
    // it passes.
    let over_lines = member("over", 501, 0, 0);
    assert_eq!(
        engine::validate(&lines_budget, std::slice::from_ref(&over_lines)).len(),
        1
    );
    let under_lines = member("under", 500, 0, 0);
    assert!(engine::validate(&lines_budget, std::slice::from_ref(&under_lines)).is_empty());

    let chars_budget = contract(ExtentUnit::Characters, 4000, false);
    let over_chars = member("wide", 1, 4001, 0);
    assert_eq!(
        engine::validate(&chars_budget, std::slice::from_ref(&over_chars)).len(),
        1
    );
    let under_chars = member("narrow", 1, 4000, 0);
    assert!(engine::validate(&chars_budget, std::slice::from_ref(&under_chars)).is_empty());

    // Whole grain sums the captured spans across the population, both units — three members
    // each under the per-member budget whose summed span (360 lines, 2700 chars) overruns.
    let members = [
        member("a", 120, 900, 0),
        member("b", 120, 900, 0),
        member("c", 120, 900, 0),
    ];
    assert_eq!(
        engine::judge(&[selection(ExtentUnit::Lines, 300, &members)]).len(),
        1
    );
    assert!(engine::judge(&[selection(ExtentUnit::Lines, 400, &members)]).is_empty());
    assert_eq!(
        engine::judge(&[selection(ExtentUnit::Characters, 2000, &members)]).len(),
        1
    );
}

#[test]
fn render_side_measurement_catches_an_overrun_a_source_side_count_would_miss() {
    // A member whose authored body is short (10 source lines) but whose projection — an
    // include resolved, a render hook run — expands past the bound (600 rendered lines).
    // `extent` reads the rendered extent, so it fires; a source-side count over
    // `body_lines` would have passed, which is exactly why `max_lines` was retired.
    let expanded = member("expanded", 600, 0, 10);
    let budget = contract(ExtentUnit::Lines, 500, false);
    assert_eq!(
        engine::validate(&budget, std::slice::from_ref(&expanded)).len(),
        1,
        "extent measures the projection, not the authored body"
    );
}

#[test]
fn a_lock_carrying_the_retired_max_lines_predicate_refuses_at_load() {
    // The closed vocabulary's ordinary reject: `max_lines` is gone, so a lock still
    // carrying it fails to lift into a clause rather than degrading to a silent skip.
    let retired = row("max_lines", None, Some(200));
    assert!(matches!(
        compose::clause_from_row(&retired),
        Err(ClauseRowError::Predicate { predicate }) if predicate == "max_lines"
    ));
}

#[test]
fn an_extent_row_naming_an_unknown_unit_refuses_at_load() {
    // The unit is a closed set too: an `extent` row whose unit is outside it is no clause,
    // refused at load rather than measuring nothing. `tokens` is the tempting one 0035
    // rejected for instability.
    let bad_unit = row("extent", Some("tokens"), Some(200));
    assert!(matches!(
        compose::clause_from_row(&bad_unit),
        Err(ClauseRowError::Predicate { .. })
    ));

    // A well-formed `extent` row lifts cleanly, so the refusal above is the unit's doing,
    // not the predicate's.
    let good = row("extent", Some("lines"), Some(200));
    let lifted = compose::clause_from_row(&good).expect("a well-formed extent row lifts");
    assert!(matches!(
        lifted.predicate,
        Predicate::Extent {
            unit: ExtentUnit::Lines,
            max: 200,
            whole: false,
        }
    ));
}

/// A lock-shaped clause row for `predicate`, carrying an optional `unit` and `max` bound —
/// the columns `extent`/`max_lines` ride. Every other column is empty, the shape a
/// fieldless node-scope clause takes.
fn row(predicate: &str, unit: Option<&str>, max: Option<usize>) -> ClauseRow {
    ClauseRow {
        label: Some(format!("skill.{predicate}")),
        kind: Some("skill".to_string()),
        predicate: predicate.to_string(),
        field: None,
        severity: "advisory".to_string(),
        guidance: None,
        cite: None,
        count: None,
        target: None,
        degree: None,
        gate: None,
        value_type: None,
        shape: None,
        bound: max.map(|max| BoundRow {
            min: None,
            max: Some(max),
        }),
        unit: unit.map(str::to_string),
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
