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
//! - [`acyclic`] — **the reference graph has no cycle** (`specs/45-governance.md`,
//!   "The graph scope (the model)"): resolve the same declared edges into an
//!   artifact-level graph and DFS for a cycle. A circular import loads nothing, so a
//!   cycle is a true positive that earns the hard gate. Like route resolution it is
//!   intrinsic to the declared edges — always-on over `layer.edges()`, no opt-in.
//!
//! - [`degree`] — **a matched node's in/out edge count is bounded**
//!   (`specs/45-governance.md`, "The graph scope (the model)"; the worked example
//!   "self-registering vs routed"): a role declares a `degree` bound and `temper`
//!   checks every artifact its `match` selects has an incoming/outgoing edge count
//!   inside it — "self-registering: zero incoming," "routed: at least one incoming."
//!   Unlike route resolution and `acyclic`, `degree` is **opt-in, per-role**: it runs
//!   only for roles that declare a bound. It is *declared* at the set scope (on the
//!   role) but *ranges over* the edge graph, so it reuses the same resolved arcs
//!   [`acyclic`] assembles ([`resolved_arcs`]) and selects nodes by the *same*
//!   [`roster::matches`] selector the roster scope uses.
//!
//! Nodes are the artifacts across every kind (their [`Features::id`]); the edges are
//! the declared references between them.
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
use crate::compose::{Edge, EdgeBound, Role};
use crate::extract::{FeatureValue, Features};
use crate::roster;

/// The diagnostic `rule` id every route-resolution finding reports under — the
/// referential clause of the harness graph (`specs/45-governance.md`, "The harness
/// is a graph too"): a declared reference must resolve to a real artifact.
const GRAPH_ROUTE_RULE: &str = "graph.route";

/// The diagnostic `rule` id every graph-admissibility finding reports under — the
/// edge declaration is itself checked (`specs/10-contracts.md`, "Decision: the
/// contract is itself checked — admissibility") before the graph judges the harness.
const GRAPH_ADMISSIBILITY_RULE: &str = "graph.admissibility";

/// The diagnostic `rule` id the acyclicity finding reports under — the graph-scope
/// `acyclic` predicate (`specs/45-governance.md`, "The graph scope (the model)"): the
/// reference graph has no cycle (a circular import loads nothing).
const GRAPH_ACYCLIC_RULE: &str = "graph.acyclic";

/// The diagnostic `rule` id every degree finding reports under — the graph-scope
/// `degree` predicate (`specs/45-governance.md`, "The graph scope (the model)"): a
/// matched node's in/out edge count must land in the role's declared bound.
const GRAPH_DEGREE_RULE: &str = "graph.degree";

/// A node in the artifact-level reference graph: `(kind, id)`. An artifact id is
/// unique only *within* a kind (a rule and a skill may share a name), and an edge
/// resolves only within its target kind, so the kind is part of the identity —
/// otherwise a same-named rule and skill would collapse into one node and forge or
/// mask a cycle.
type Node = (String, String);

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

/// Check **acyclicity** over the harness reference graph (`specs/45-governance.md`,
/// "The graph scope (the model)"): build the artifact-level directed graph by
/// resolving each declared [`Edge`]'s reference field to real target ids — the same
/// arc resolution [`check`] performs — and return an error-severity [`Diagnostic`]
/// naming a cycle if the reference graph is not acyclic.
///
/// A cycle among declared references is a circular import that loads nothing, so the
/// finding is a true positive that earns the hard gate. `acyclic` is *intrinsic* to
/// the declared edges — it runs always-on over `layer.edges()`, no per-role opt-in,
/// exactly like route resolution.
///
/// Only **resolved** arcs enter the graph: an [`is_admissible`]-failing edge is
/// skipped (as in [`check`]), and a dangling reference (one naming no artifact of the
/// target kind) loads nothing, so it contributes no arc — it can neither forge a
/// cycle nor mask one, leaving that dangling finding to [`check`]. Nodes are keyed by
/// `(kind, id)` so a same-named artifact of a different kind is a distinct node.
///
/// At most one finding is returned — a cycle is a fatal structural fault, and naming
/// one closed chain suffices to fail the run and orient the author. The chain is
/// canonicalized (rotated to start at its least node) so the finding is stable
/// regardless of which node the traversal happened to enter from.
#[must_use]
pub fn acyclic(edges: &[Edge], by_kind: &BTreeMap<&str, &[Features]>) -> Vec<Diagnostic> {
    // Build adjacency over resolved reference arcs — the same graph `degree` reads.
    let adjacency = resolved_arcs(edges, by_kind);

    // A three-color DFS: a back edge to a node still on the current path (`Gray`)
    // closes a cycle. Roots iterate in sorted node order (BTreeMap keys), and each
    // node's neighbours in sorted order (BTreeSet), so the first cycle found is
    // deterministic across runs.
    let mut color: BTreeMap<Node, Color> = BTreeMap::new();
    let mut path: Vec<Node> = Vec::new();
    for root in adjacency.keys() {
        if color.get(root).copied().unwrap_or(Color::White) != Color::White {
            continue;
        }
        if let Some(cycle) = find_cycle(root, &adjacency, &mut color, &mut path) {
            return vec![cycle_diagnostic(&canonical_cycle(&cycle))];
        }
    }
    Vec::new()
}

/// Check the graph-scope **`degree`** predicate over the harness reference graph
/// (`specs/45-governance.md`, "The graph scope (the model)"; the worked example
/// "self-registering vs routed"): for each role that declares a
/// [`DegreeBound`](crate::compose::DegreeBound), compute the incoming/outgoing edge
/// count of every artifact its `match` selects
/// over the *resolved* reference arcs and return an error-severity [`Diagnostic`] per
/// matched node whose degree falls outside the declared bound.
///
/// `degree` is declared at the **set scope** (on a role) but ranges over the **edge
/// graph**, so it is checked here rather than in [`crate::roster`]: it reuses the
/// same [`resolved_arcs`] [`acyclic`] and [`check`] assemble, and selects a role's
/// nodes by the *same* [`roster::matches`] selector the roster scope uses — never a
/// second matcher that could disagree. Only **resolved** arcs count toward a degree:
/// a dangling reference loads nothing (route resolution owns that finding), and an
/// inadmissible edge is skipped, exactly as in [`acyclic`].
///
/// Unlike route resolution and `acyclic`, `degree` is **opt-in, per-role** — it runs
/// only for roles carrying a bound, so a roster declaring none (or an absent
/// `temper.toml`) does no graph work here. A node is `(kind, id)`, the role's
/// `artifact` kind paired with each matched id; incoming degree is how many distinct
/// nodes point at it, outgoing how many distinct nodes it points at (an arc is a
/// distinct `(source, target)` pair, deduped like [`acyclic`]'s adjacency). Roles
/// iterate in name order and each kind's candidates arrive name-sorted, so the
/// finding set is stable across runs.
#[must_use]
pub fn degree(
    roles: &BTreeMap<String, Role>,
    edges: &[Edge],
    by_kind: &BTreeMap<&str, &[Features]>,
) -> Vec<Diagnostic> {
    // Opt-in: with no role declaring a bound, the graph is never even assembled.
    if roles.values().all(|role| role.degree.is_none()) {
        return Vec::new();
    }

    let adjacency = resolved_arcs(edges, by_kind);
    // Incoming degree per node: how many distinct nodes point at it. Built once by
    // inverting the resolved arcs; a node absent from the map has in-degree zero.
    let mut incoming: BTreeMap<&Node, usize> = BTreeMap::new();
    for targets in adjacency.values() {
        for target in targets {
            *incoming.entry(target).or_default() += 1;
        }
    }

    let mut diagnostics = Vec::new();
    for role in roles.values() {
        let Some(bound) = &role.degree else {
            continue;
        };
        let candidates = by_kind.get(role.artifact.as_str()).copied().unwrap_or(&[]);
        for features in candidates {
            if !roster::matches(&role.selector, features) {
                continue;
            }
            // The node is `(kind, id)`: an id is unique only within a kind, and an arc
            // resolves only within its target kind, so degree is per-`(kind, id)` too.
            let node = (role.artifact.clone(), features.id.clone());
            let in_degree = incoming.get(&node).copied().unwrap_or(0);
            let out_degree = adjacency.get(&node).map_or(0, BTreeSet::len);

            if let Some(edge_bound) = bound.incoming
                && !edge_bound.admits(in_degree)
            {
                diagnostics.push(out_of_degree(
                    role,
                    &features.id,
                    Direction::Incoming,
                    in_degree,
                    edge_bound,
                ));
            }
            if let Some(edge_bound) = bound.outgoing
                && !edge_bound.admits(out_degree)
            {
                diagnostics.push(out_of_degree(
                    role,
                    &features.id,
                    Direction::Outgoing,
                    out_degree,
                    edge_bound,
                ));
            }
        }
    }
    diagnostics
}

/// A degree direction — which side of a node's edges a [`DegreeBound`] constrains,
/// named in the finding so the author knows whether an incoming or outgoing count
/// broke the bound.
#[derive(Clone, Copy)]
enum Direction {
    /// Edges *pointing at* the node — how many nodes reference it.
    Incoming,
    /// Edges *from* the node — how many nodes it references.
    Outgoing,
}

impl Direction {
    /// The word the finding uses for this direction.
    fn label(self) -> &'static str {
        match self {
            Direction::Incoming => "incoming",
            Direction::Outgoing => "outgoing",
        }
    }
}

/// The finding for a matched node whose `degree` in one direction falls outside its
/// role's declared bound — naming the role, the artifact, the direction, the actual
/// edge count, and the `[min, max]` bound it missed (an open endpoint rendered `∞`).
fn out_of_degree(
    role: &Role,
    artifact: &str,
    direction: Direction,
    actual: usize,
    bound: EdgeBound,
) -> Diagnostic {
    let min = bound.min.map_or_else(|| "0".to_string(), |n| n.to_string());
    let max = bound.max.map_or_else(|| "∞".to_string(), |n| n.to_string());
    Diagnostic::error(
        GRAPH_DEGREE_RULE,
        artifact,
        format!(
            "role `{}` bounds `{}` {} degree to [{min}, {max}], but `{artifact}` has {actual}",
            role.name,
            role.artifact,
            direction.label(),
        ),
    )
}

/// Build the artifact-level directed graph over **resolved** reference arcs — the
/// shared foundation [`acyclic`] and [`degree`] both range over. Each admissible edge
/// contributes, for every source artifact of its `from` kind, an arc to each named
/// target that resolves to a real artifact of its `to` kind. An [`is_admissible`]-
/// failing edge is skipped (as in [`check`]), and a dangling reference (naming no
/// artifact of the target kind) loads nothing, so it contributes no arc — route
/// resolution owns that finding. Nodes are keyed `(kind, id)` so a same-named
/// artifact of a different kind is a distinct node. Arcs dedupe in the [`BTreeSet`],
/// so a target named twice (e.g. by a list field) is one arc.
fn resolved_arcs(
    edges: &[Edge],
    by_kind: &BTreeMap<&str, &[Features]>,
) -> BTreeMap<Node, BTreeSet<Node>> {
    let mut adjacency: BTreeMap<Node, BTreeSet<Node>> = BTreeMap::new();
    for edge in edges {
        if !is_admissible(edge, by_kind) {
            continue;
        }
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
                // A dangling reference loads nothing — no arc.
                if targets.contains(target) {
                    adjacency
                        .entry((edge.from.clone(), source.id.clone()))
                        .or_default()
                        .insert((edge.to.clone(), target.to_string()));
                }
            }
        }
    }
    adjacency
}

/// DFS coloring for cycle detection: `White` unvisited, `Gray` on the current path,
/// `Black` fully explored (no cycle reachable through it).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

/// Depth-first search from `node` for a back edge. On finding a neighbour still
/// `Gray` (on the current path), returns the closed cycle: the path suffix from that
/// neighbour to `node`, plus the neighbour again to close the ring. Returns `None`
/// when the subtree rooted at `node` holds no cycle.
fn find_cycle(
    node: &Node,
    adjacency: &BTreeMap<Node, BTreeSet<Node>>,
    color: &mut BTreeMap<Node, Color>,
    path: &mut Vec<Node>,
) -> Option<Vec<Node>> {
    color.insert(node.clone(), Color::Gray);
    path.push(node.clone());
    if let Some(neighbours) = adjacency.get(node) {
        for next in neighbours {
            match color.get(next).copied().unwrap_or(Color::White) {
                Color::White => {
                    if let Some(cycle) = find_cycle(next, adjacency, color, path) {
                        return Some(cycle);
                    }
                }
                Color::Gray => {
                    // A back edge to a node on the current path closes a cycle. The
                    // node is on `path` by the invariant that `Gray` ⇔ on the path.
                    let start = path
                        .iter()
                        .position(|n| n == next)
                        .expect("a Gray node is on the current DFS path");
                    let mut cycle = path[start..].to_vec();
                    cycle.push(next.clone());
                    return Some(cycle);
                }
                Color::Black => {}
            }
        }
    }
    path.pop();
    color.insert(node.clone(), Color::Black);
    None
}

/// Canonicalize a closed cycle (`[a, …, a]`) so its rendering is stable regardless of
/// which node the traversal entered from: drop the closing repeat, rotate the ring to
/// begin at its least node, then re-close it.
fn canonical_cycle(cycle: &[Node]) -> Vec<Node> {
    // `cycle` is closed: its last element repeats its first. The ring is the rest.
    let ring = &cycle[..cycle.len().saturating_sub(1)];
    let pivot = ring
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map_or(0, |(index, _)| index);
    let mut rotated: Vec<Node> = ring[pivot..]
        .iter()
        .chain(&ring[..pivot])
        .cloned()
        .collect();
    if let Some(first) = rotated.first().cloned() {
        rotated.push(first);
    }
    rotated
}

/// The finding for a cyclic reference graph — naming the closed chain of `<kind>
/// \`<id>\`` nodes so the author can see exactly which references form the circle.
fn cycle_diagnostic(cycle: &[Node]) -> Diagnostic {
    let chain = cycle
        .iter()
        .map(|(kind, id)| format!("{kind} `{id}`"))
        .collect::<Vec<_>>()
        .join(" → ");
    // Name the finding after the ring's least node — the same node the chain starts
    // at — so the `artifact` is stable and points into the cycle.
    let artifact = cycle.first().map_or_else(String::new, |(_, id)| id.clone());
    Diagnostic::error(
        GRAPH_ACYCLIC_RULE,
        artifact,
        format!("the harness reference graph contains a cycle: {chain}"),
    )
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
            satisfies: Vec::new(),
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

    /// A `routes_to` edge from `skill` back to `rule` — the return arc that closes a
    /// `rule → skill → rule` cycle.
    fn skill_to_rule_edge() -> Edge {
        Edge {
            field: "routes_to".to_string(),
            from: "skill".to_string(),
            to: "rule".to_string(),
        }
    }

    #[test]
    fn an_acyclic_reference_graph_is_clean() {
        // `rule style → skill standards`, with no return arc — a DAG, so `acyclic`
        // has nothing to report.
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(acyclic(&edges, &by_kind).is_empty());
    }

    #[test]
    fn a_self_loop_fires_an_acyclic_error() {
        // A `rule → rule` edge whose source routes to itself: the shortest cycle. It
        // fires an error naming the artifact under the `graph.acyclic` rule.
        let edges = [Edge {
            field: "routes_to".to_string(),
            from: "rule".to_string(),
            to: "rule".to_string(),
        }];
        let rules = [node("style", Some("style"))];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);
        let diags = acyclic(&edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, GRAPH_ACYCLIC_RULE);
        assert_eq!(diags[0].artifact, "style");
        assert!(diags[0].message.contains("cycle"));
        assert!(diags[0].message.contains("style"));
    }

    #[test]
    fn a_multi_node_cycle_fires_an_acyclic_error() {
        // `rule style → skill standards → rule style`: two edges close a circle across
        // two kinds. One finding naming the whole chain.
        let edges = [routes_to_edge(), skill_to_rule_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [node("standards", Some("style"))];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = acyclic(&edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, GRAPH_ACYCLIC_RULE);
        assert!(diags[0].message.contains("cycle"));
        assert!(diags[0].message.contains("style"));
        assert!(diags[0].message.contains("standards"));
    }

    #[test]
    fn a_dangling_reference_does_not_forge_a_cycle() {
        // `rule style` routes to two skills: `standards` resolves, `absent` dangles.
        // The dangling arc loads nothing, and the resolving arc is acyclic — clean.
        // (Route resolution owns the dangling `absent` finding, not `acyclic`.)
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
        assert!(acyclic(&edges, &by_kind).is_empty());
    }

    #[test]
    fn a_dangling_reference_does_not_mask_a_real_cycle() {
        // `rule style` routes to `standards` (resolves) and `absent` (dangles), and
        // `skill standards` routes back to `style` — a real `style → standards →
        // style` cycle. The dangling arc must not suppress it.
        let mut style = node("style", None);
        style.fields.insert(
            "routes_to".to_string(),
            FeatureValue::List(vec!["standards".to_string(), "absent".to_string()]),
        );
        let edges = [routes_to_edge(), skill_to_rule_edge()];
        let rules = [style];
        let skills = [node("standards", Some("style"))];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = acyclic(&edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, GRAPH_ACYCLIC_RULE);
        assert!(diags[0].message.contains("style"));
        assert!(diags[0].message.contains("standards"));
    }

    #[test]
    fn an_inadmissible_edge_is_skipped_by_acyclic() {
        // The target kind `agent` is not modeled — the edge is inadmissible, so
        // `acyclic` skips it exactly as `check` does. Even a self-naming source over
        // it forges no cycle, because the arc never resolves.
        let edge = edge("[[kind.rule.relationships]]\nfield = \"routes_to\"\nto = \"agent\"\n");
        let rules = [node("style", Some("style"))];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);
        assert!(acyclic(std::slice::from_ref(&edge), &by_kind).is_empty());
    }

    /// Parse a `temper.toml` fragment's `[role.<name>]` tables into the typed roster
    /// the [`degree`] check reads — the parse foundation is the only constructor for a
    /// [`Role`]'s `degree` bound, so the graph tests drive it.
    fn roles(toml: &str) -> BTreeMap<String, crate::compose::Role> {
        AuthorLayer::parse(toml, Path::new("temper.toml"))
            .unwrap()
            .roles()
            .clone()
    }

    /// A role selecting the skill `standards` and declaring a `degree` bound `clause`
    /// (an inline `{ … }` body), over an inline contract so the role is admissible.
    /// The graph the degree check ranges over is the caller's `edges`/`by_kind`.
    fn degree_role(clause: &str) -> BTreeMap<String, crate::compose::Role> {
        roles(&format!(
            "[role.gate]\n\
             artifact = \"skill\"\n\
             match = {{ name = \"standards\" }}\n\
             degree = {{ {clause} }}\n\
             [[role.gate.clause]]\n\
             severity = \"required\"\n\
             predicate = \"required\"\n\
             field = \"name\"\n"
        ))
    }

    #[test]
    fn a_self_registering_bound_passes_when_the_node_is_not_pointed_at() {
        // `incoming = { max = 0 }`: the skill `standards` must not be pointed at. No
        // rule routes to it (the only rule routes nowhere), so its incoming degree is
        // zero — inside the bound, clean.
        let roles = degree_role("incoming = { max = 0 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", None)];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(degree(&roles, &edges, &by_kind).is_empty());
    }

    #[test]
    fn a_self_registering_bound_fires_when_the_node_is_pointed_at() {
        // The rule `style` routes to `standards`, so the skill has incoming degree 1 —
        // outside `incoming = { max = 0 }`. A self-registering artifact must not be
        // reached: an error naming the role, the artifact, and the direction.
        let roles = degree_role("incoming = { max = 0 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(&roles, &edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, GRAPH_DEGREE_RULE);
        assert_eq!(diags[0].artifact, "standards");
        assert!(diags[0].message.contains("gate"));
        assert!(diags[0].message.contains("incoming"));
    }

    #[test]
    fn a_routed_bound_passes_when_the_node_is_reachable() {
        // `incoming = { min = 1 }`: the skill `standards` must be reachable. The rule
        // `style` routes to it, so its incoming degree is 1 — inside the open-above
        // bound, clean.
        let roles = degree_role("incoming = { min = 1 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(degree(&roles, &edges, &by_kind).is_empty());
    }

    #[test]
    fn a_routed_bound_fires_when_the_node_is_unreachable() {
        // No rule routes to `standards`, so its incoming degree is zero — outside
        // `incoming = { min = 1 }`. A routed artifact must be reachable: an error.
        let roles = degree_role("incoming = { min = 1 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", None)];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(&roles, &edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, GRAPH_DEGREE_RULE);
        assert_eq!(diags[0].artifact, "standards");
        assert!(diags[0].message.contains("incoming"));
    }

    #[test]
    fn an_outgoing_bound_reads_the_matched_node_out_degree() {
        // Degree bounds both directions: the rule `style` (selected by an `outgoing`
        // bound) routes to one skill, so its out-degree is 1 — outside `{ max = 0 }`.
        let roles = roles(
            "[role.gate]\n\
             artifact = \"rule\"\n\
             match = { name = \"style\" }\n\
             degree = { outgoing = { max = 0 } }\n\
             [[role.gate.clause]]\n\
             severity = \"required\"\n\
             predicate = \"required\"\n\
             field = \"routes_to\"\n",
        );
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(&roles, &edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].artifact, "style");
        assert!(diags[0].message.contains("outgoing"));
    }

    #[test]
    fn a_roster_declaring_no_degree_bound_does_no_graph_work() {
        // `degree` is opt-in, per-role: a role with no bound is silent even over a
        // graph that would violate one — `temper` never fabricates a gate the author
        // did not declare (`00-intent.md` law 4).
        let roles = roles(
            "[role.gate]\n\
             artifact = \"skill\"\n\
             match = { name = \"standards\" }\n\
             contract = \"contracts/skill.toml\"\n",
        );
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(degree(&roles, &edges, &by_kind).is_empty());
    }

    #[test]
    fn a_dangling_reference_does_not_count_toward_degree() {
        // The rule routes to `absent`, which resolves to no skill — a dangling arc
        // that loads nothing, so `standards` has incoming degree zero and a routed
        // `{ min = 1 }` bound fires. The dangling reference neither forges nor masks a
        // degree, exactly as it neither forges nor masks a cycle.
        let roles = degree_role("incoming = { min = 1 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("absent"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(&roles, &edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].artifact, "standards");
    }
}
