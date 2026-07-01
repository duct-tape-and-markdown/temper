//! The harness reference graph — route resolution over declared edges.
//!
//! Implements `specs/45-governance.md` ("The harness is a graph too — and
//! references are declared edges"). The **harness** is a graph: skills and rules
//! pointing at each other. To govern its shape `temper` needs the edges — and an
//! edge is a **declared field on the surface**, never grepped from prose
//! (`(skill-ref-syntax)` RESOLVED: the field is the truth, the prose is payload).
//! A rule routes to a skill through a structured field (`routes_to: standards`),
//! authored on the composition surface (`specs/20-surface.md`); `temper` reads the
//! field into the graph.
//!
//! ## What this tier does
//!
//! Given the [`Edge`] relationships declared on the author layer
//! ([`AuthorLayer::edges`](crate::compose::AuthorLayer::edges)) and the by-kind
//! [`Features`] of the whole corpus:
//!
//! - [`check`] — **route resolution**: for each declared edge (source kind `from`,
//!   reference field `F`, target kind `to`), read `F` off every source artifact and
//!   emit an error-severity [`Diagnostic`] for any route whose named target resolves
//!   to no artifact of the target kind. This is the *referential decidable check*
//!   (`specs/10-contracts.md`, the referential primitive: a reference resolves over
//!   a **declared syntax**, never prose-grep) — every violation is a true positive,
//!   so it earns the hard gate.
//! - [`admissibility`] — **the edge declaration is itself checked**
//!   (`specs/10-contracts.md`, "Decision: the contract is itself checked"): before
//!   the graph is trusted to judge the harness, each edge must *name its reference
//!   field and target kind* (`specs/45-governance.md`). An edge with an empty field
//!   names no reference syntax; one whose target kind `temper` does not model can
//!   *never* resolve — its every route would dangle, so the fault is the declaration,
//!   not the artifacts. An inadmissible edge is reported here and **skipped** by
//!   [`check`], so a single unsound declaration does not forge a route finding on
//!   every source artifact.
//!
//! Nodes are the artifacts across every kind (their [`Features::id`]); the edges are
//! the declared references between them. `degree`/`acyclic` (`specs/45-governance.md`)
//! are the next graph-scope predicates — they read the same edges this tier assembles.
//!
//! ## Only declared fields, never grepped prose
//!
//! The reference is read off [`Features`] — the deterministically-extracted feature
//! set (`specs/30-landscapes.md`, the extraction soundness boundary), the `extra`
//! catch-all surfacing the author's `routes_to` field like any other. A scalar field
//! names one target; a list field names several. `temper` never scans a body for
//! names or paths — the unsound prose-grep the referential rule forbids
//! (`specs/10-contracts.md`).

use std::collections::{BTreeMap, BTreeSet};

use crate::check::Diagnostic;
use crate::compose::Edge;
use crate::extract::{FeatureValue, Features};

/// The diagnostic `rule` id every route-resolution finding reports under — the
/// referential clause of the harness graph (`specs/45-governance.md`, "The harness
/// is a graph too"): a declared reference must resolve to a real artifact.
const GRAPH_ROUTE_RULE: &str = "graph.route";

/// The diagnostic `rule` id every graph-admissibility finding reports under — the
/// edge declaration is itself checked (`specs/10-contracts.md`, "Decision: the
/// contract is itself checked — admissibility") before the graph judges the harness.
const GRAPH_ADMISSIBILITY_RULE: &str = "graph.admissibility";

/// Build the harness reference graph and check **route resolution** over it
/// (`specs/45-governance.md`, "The harness is a graph too"): for each declared
/// [`Edge`], read its reference field off every source artifact and return an
/// error-severity [`Diagnostic`] for any route that resolves to no artifact of the
/// target kind.
///
/// `by_kind` maps an artifact kind (`skill`, `rule`, …) to the workspace
/// [`Features`] of that kind — the whole corpus, since an edge's source and target
/// kinds may differ. An edge that fails [`admissibility`] (an empty reference field,
/// or a target kind `temper` does not model) is **skipped** here, so its single
/// declaration fault is reported once by admissibility rather than forged into a
/// route finding on every source artifact. Sources iterate in candidate order (each
/// kind's slice is name-sorted), so the finding set is stable across runs.
#[must_use]
pub fn check(edges: &[Edge], by_kind: &BTreeMap<&str, &[Features]>) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for edge in edges {
        // A declaration that is not itself admissible cannot soundly judge the
        // harness — admissibility owns that finding, so skip it here rather than
        // dangle every source's route off an unsound edge.
        if !is_admissible(edge, by_kind) {
            continue;
        }

        // The nodes reachable as targets: the ids of every artifact of the target
        // kind. A route resolves exactly when its named target is one of these.
        let targets: BTreeSet<&str> = by_kind
            .get(edge.to.as_str())
            .copied()
            .unwrap_or(&[])
            .iter()
            .map(|features| features.id.as_str())
            .collect();

        let sources = by_kind.get(edge.from.as_str()).copied().unwrap_or(&[]);
        for source in sources {
            for target in edge_targets(source, &edge.field) {
                if !targets.contains(target) {
                    diagnostics.push(dangling(edge, source.id.as_str(), target));
                }
            }
        }
    }
    diagnostics
}

/// Validate the declared edge relationships against **the definition** —
/// admissibility (`specs/10-contracts.md`, "Decision: the contract is itself
/// checked"). Each edge earns trust by passing a check *before* the graph is used to
/// judge the harness; every finding is [`Diagnostic::error`] (an inadmissible edge
/// cannot be trusted, so it must fail the run) and names the edge it indicts.
///
/// Two decidable clauses, mirroring the spec's requirement that a declared edge
/// relationship *name its reference field and target kind*
/// (`specs/45-governance.md`, "The harness is a graph too"):
///
/// - **(a) the reference field is named** — a non-empty `field`. An empty field
///   names no reference syntax; no artifact declares an empty-named field, so the
///   edge could never carry a route.
/// - **(b) the target kind is one `temper` models** — `edge.to` is a key of
///   `by_kind`. A target kind `temper` does not model has no artifacts, so *every*
///   route over the edge would dangle — the fault is the declaration, not the
///   sources, so it is reported once here (and [`check`] skips the edge). This
///   mirrors the roster's "a required role over an unmodeled kind can never be
///   filled" admissibility clause ([`crate::roster`]).
///
/// `by_kind` is the same corpus map [`check`] reads — admissibility uses only its
/// *keys* (the modeled kinds), never the artifacts.
#[must_use]
pub fn admissibility(edges: &[Edge], by_kind: &BTreeMap<&str, &[Features]>) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for edge in edges {
        // (a) The reference field is named.
        if edge.field.is_empty() {
            diagnostics.push(Diagnostic::error(
                GRAPH_ADMISSIBILITY_RULE,
                edge_id(edge),
                format!(
                    "edge from `{}` to `{}` declares an empty reference field, which names no reference syntax",
                    edge.from, edge.to
                ),
            ));
        }

        // (b) The target kind is one `temper` models — else no route can resolve.
        if !by_kind.contains_key(edge.to.as_str()) {
            diagnostics.push(Diagnostic::error(
                GRAPH_ADMISSIBILITY_RULE,
                edge_id(edge),
                format!(
                    "edge `{}` targets kind `{}`, which `temper` does not model — no route can ever resolve",
                    edge.field, edge.to
                ),
            ));
        }
    }
    diagnostics
}

/// Whether an [`Edge`] is admissible: its reference field is named (non-empty) and
/// its target kind is one `temper` models. The predicate [`check`] gates on to skip
/// an unsound declaration, kept in lockstep with the clauses [`admissibility`]
/// reports so the two never disagree.
fn is_admissible(edge: &Edge, by_kind: &BTreeMap<&str, &[Features]>) -> bool {
    !edge.field.is_empty() && by_kind.contains_key(edge.to.as_str())
}

/// The target artifact names an `edge`'s reference field carries on one source
/// artifact: a scalar field names one target, a list field names each of several,
/// and an absent field (or a map, which carries no name) names none. Read off
/// [`Features`] — a declared field, never grepped prose.
fn edge_targets<'a>(source: &'a Features, field: &str) -> Vec<&'a str> {
    match source.field(field) {
        None | Some(FeatureValue::Map) => Vec::new(),
        Some(FeatureValue::List(items)) => items.iter().map(String::as_str).collect(),
        Some(value @ FeatureValue::Scalar { .. }) => value.as_scalar().into_iter().collect(),
    }
}

/// A stable identity for an edge in a diagnostic — `<from>.<field>` (e.g.
/// `rule.routes_to`), naming the source kind and the reference field so a reader
/// knows which declaration is indicted.
fn edge_id(edge: &Edge) -> String {
    format!("{}.{}", edge.from, edge.field)
}

/// The finding for a route that resolves to no artifact — naming the source
/// artifact carrying the route, the reference field, the dangling target, and the
/// target kind no artifact of which bears that id.
fn dangling(edge: &Edge, source: &str, target: &str) -> Diagnostic {
    Diagnostic::error(
        GRAPH_ROUTE_RULE,
        source,
        format!(
            "`{source}` `{}` routes to `{target}`, which resolves to no `{}` artifact",
            edge.field, edge.to
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::check::Severity;
    use crate::compose::{AuthorLayer, Edge};
    use crate::extract::Kind;
    use std::path::Path;

    /// A `Features` carrying a name (its `id`) and, optionally, a `routes_to`
    /// reference field — a scalar naming one target.
    fn node(name: &str, routes_to: Option<&str>) -> Features {
        let mut fields = BTreeMap::new();
        if let Some(target) = routes_to {
            fields.insert(
                "routes_to".to_string(),
                FeatureValue::scalar(Kind::String, target),
            );
        }
        Features {
            id: name.to_string(),
            fields,
            body_lines: 1,
            headings: Vec::new(),
            source_dir: Some(name.to_string()),
            companions: Vec::new(),
        }
    }

    /// The `routes_to` edge every case shares: a rule points at a skill.
    fn routes_to_edge() -> Edge {
        Edge {
            field: "routes_to".to_string(),
            from: "rule".to_string(),
            to: "skill".to_string(),
        }
    }

    /// Parse the first edge out of a `temper.toml` fragment — the parse foundation
    /// (a kind's `[[kind.<name>.relationships]]` array) is the only constructor for
    /// an [`Edge`], so the graph tests drive it.
    fn edge(toml: &str) -> Edge {
        AuthorLayer::parse(toml, Path::new("temper.toml"))
            .unwrap()
            .edges()[0]
            .clone()
    }

    #[test]
    fn a_resolving_route_is_clean() {
        // The rule `style` routes to the skill `standards`, which exists — so the
        // route resolves and nothing fires.
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(check(&edges, &by_kind).is_empty());
    }

    #[test]
    fn a_dangling_route_fires_a_route_resolution_error() {
        // The rule routes to `absent`, which names no skill — a dangling route, an
        // error naming the source, the field, the target, and the target kind.
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("absent"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = check(&edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, GRAPH_ROUTE_RULE);
        assert_eq!(diags[0].artifact, "style");
        assert!(diags[0].message.contains("absent"));
        assert!(diags[0].message.contains("routes_to"));
        assert!(diags[0].message.contains("skill"));
    }

    #[test]
    fn a_source_declaring_no_reference_field_carries_no_route() {
        // A rule with no `routes_to` field declares no edge — `temper` never invents
        // a route the author did not author, so nothing fires.
        let edges = [routes_to_edge()];
        let rules = [node("style", None)];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(check(&edges, &by_kind).is_empty());
    }

    #[test]
    fn a_route_resolves_only_within_the_target_kind() {
        // A rule named the same as the route target exists, but the edge targets
        // `skill` — a same-named *rule* does not satisfy it, so the route dangles.
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards")), node("standards", None)];
        let skills: [Features; 0] = [];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = check(&edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].artifact, "style");
    }

    #[test]
    fn a_list_reference_field_names_several_targets() {
        // A `routes_to` list names two targets; one resolves and one dangles, so a
        // single finding fires for the dangling element only.
        let mut style = node("style", None);
        style.fields.insert(
            "routes_to".to_string(),
            FeatureValue::List(vec!["standards".to_string(), "absent".to_string()]),
        );
        let edges = [routes_to_edge()];
        let rules = [style];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = check(&edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("absent"));
    }

    #[test]
    fn an_edge_over_an_unmodeled_target_kind_is_inadmissible_and_skipped() {
        // The target kind `agent` is not modeled (`by_kind` has only `rule`): every
        // route would dangle, so the fault is the declaration. Admissibility reports
        // it once, and `check` skips the edge rather than flag every source.
        let edge = edge("[[kind.rule.relationships]]\nfield = \"routes_to\"\nto = \"agent\"\n");
        let rules = [node("style", Some("whatever"))];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);

        let admit = admissibility(std::slice::from_ref(&edge), &by_kind);
        assert_eq!(admit.len(), 1);
        assert_eq!(admit[0].severity, Severity::Error);
        assert_eq!(admit[0].rule, GRAPH_ADMISSIBILITY_RULE);
        assert!(admit[0].message.contains("agent"));
        assert!(admit[0].message.contains("does not model"));

        // `check` skips the inadmissible edge — no per-source route finding.
        assert!(check(std::slice::from_ref(&edge), &by_kind).is_empty());
    }

    #[test]
    fn an_edge_with_an_empty_reference_field_is_inadmissible() {
        // An empty `field` names no reference syntax — admissibility rejects it, and
        // `check` skips it (no field to read off any source).
        let edge = Edge {
            field: String::new(),
            from: "rule".to_string(),
            to: "skill".to_string(),
        };
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let admit = admissibility(std::slice::from_ref(&edge), &by_kind);
        assert_eq!(admit.len(), 1);
        assert_eq!(admit[0].rule, GRAPH_ADMISSIBILITY_RULE);
        assert!(admit[0].message.contains("empty reference field"));
        assert!(check(std::slice::from_ref(&edge), &by_kind).is_empty());
    }

    #[test]
    fn a_well_formed_edge_over_a_modeled_kind_is_admissible() {
        // A named field and a modeled target kind — nothing for admissibility to
        // reject.
        let edges = [routes_to_edge()];
        let rules: [Features; 0] = [];
        let skills: [Features; 0] = [];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(admissibility(&edges, &by_kind).is_empty());
    }

    #[test]
    fn a_source_over_an_unmodeled_from_kind_is_silent() {
        // The edge's `from` kind has no artifacts in the corpus — no sources, so no
        // routes to resolve. Not an inadmissibility (the author may model that kind
        // later); just silent, mirroring a non-required role over an unmodeled kind.
        let edges = [routes_to_edge()];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        assert!(admissibility(&edges, &by_kind).is_empty());
        assert!(check(&edges, &by_kind).is_empty());
    }
}
