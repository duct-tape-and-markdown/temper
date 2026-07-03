//! The harness reference graph — route resolution over declared edges
//! (`specs/architecture/45-governance.md`, "The harness is a graph too").
//!
//! The harness is a graph: skills and rules pointing at each other through
//! **declared** reference fields, read off [`Features`], never grepped from a body
//! (`specs/architecture/10-contracts.md`, the referential primitive). Nodes are `(kind, id)`
//! across every kind; edges are the [`Edge`] relationships declared on the surface.
//! Five checks range over it: [`check`] (route resolution — a reference resolves to a
//! real target), [`admissibility`] (each edge names its field and a modeled target
//! kind, checked before the graph is trusted), [`acyclic`] (no circular import),
//! [`degree`] (a satisfier node's in/out count lands in a requirement's bound), and
//! [`reachable`] (a member's inbound activation edge from the distinguished [`world`]
//! node is live, `specs/architecture/45-governance.md`, "The world is a node"). The first four
//! range over one resolved-edge enumeration ([`resolved_edges`]), shared with
//! `crate::read`'s narration so gate and read never disagree (READ-EDGE-UNIFY).

use std::collections::{BTreeMap, BTreeSet};

use regex::Regex;

use crate::check::{Diagnostic, Severity};
use crate::compose::{Edge, EdgeBound, Requirement};
use crate::extract::{FeatureValue, Features};
use crate::kind::Activation;
use crate::roster;

/// The diagnostic `rule` id every route-resolution finding reports under.
const GRAPH_ROUTE_RULE: &str = "graph.route";

/// The diagnostic `rule` id every graph-admissibility finding reports under.
const GRAPH_ADMISSIBILITY_RULE: &str = "graph.admissibility";

/// The diagnostic `rule` id the acyclicity finding reports under.
const GRAPH_ACYCLIC_RULE: &str = "graph.acyclic";

/// The diagnostic `rule` id every degree finding reports under.
const GRAPH_DEGREE_RULE: &str = "graph.degree";

/// The diagnostic `rule` id every reachability finding reports under.
const GRAPH_REACHABLE_RULE: &str = "graph.reachable";

/// A node in the artifact-level reference graph: `(kind, id)`. An id is unique only
/// *within* a kind and an edge resolves only within its target kind, so the kind is
/// part of the identity — else a same-named rule and skill collapse into one node and
/// forge or mask a cycle.
///
/// `pub(crate)` so the read family (`crate::read`) keys a member's resolved in/out
/// edges on the *same* `(kind, id)` node the gate does (READ-EDGE-UNIFY).
pub(crate) type Node = (String, String);

/// The distinguished **world** node — the harness runtime and repo `temper` observes
/// but does not govern (`specs/architecture/45-governance.md`, "The world is a node — reachability
/// is a predicate"). Activation facts are its edges *into* members; [`reachable`]
/// decides whether the edge the world would use to reach a given member is live. Keyed
/// like any [`Node`] under a reserved `world` kind no artifact kind collides with, so a
/// follow-on gate can place it in the same `(kind, id)` graph the other predicates
/// range over.
pub(crate) fn world() -> Node {
    ("world".to_string(), "world".to_string())
}

/// A **resolved edge** — a `(from, field, to)` triple over `(kind, id)` [`Node`]s,
/// both endpoints naming a real artifact. The element type of [`resolved_edges`], the
/// one arc-resolution enumeration [`resolved_arcs`] folds into adjacency and
/// `crate::read` narrates per node, so gate and read range over one identical edge set
/// (READ-EDGE-UNIFY). Retains the reference `field` an arc drops, so a reader can see
/// which declared reference produced the edge.
pub(crate) struct ResolvedEdge {
    /// The source node `(kind, id)` carrying the reference.
    pub from: Node,
    /// The reference field the edge was declared under (`routes_to`).
    pub field: String,
    /// The target node `(kind, id)` the reference resolved to.
    pub to: Node,
}

/// Check **route resolution** over the harness reference graph
/// (`specs/architecture/45-governance.md`): for each declared [`Edge`], read its reference field
/// off every source artifact and return an error-severity [`Diagnostic`] for any
/// route that resolves to no artifact of the target kind.
///
/// `by_kind` maps each kind to the whole corpus of that kind, since an edge's source
/// and target kinds may differ. An [`admissibility`]-failing edge is **skipped** so
/// its one declaration fault is not forged into a route finding on every source.
/// Sources iterate in candidate (name-sorted) order, so the finding set is stable.
#[must_use]
pub fn check(edges: &[Edge], by_kind: &BTreeMap<&str, &[Features]>) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for edge in edges {
        // Inadmissible edges are admissibility's finding to own — skip here rather
        // than dangle every source's route off an unsound declaration.
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
                if !targets.contains(target) {
                    diagnostics.push(dangling(edge, source.id.as_str(), target));
                }
            }
        }
    }
    diagnostics
}

/// Validate the declared edges against **the definition** — admissibility
/// (`specs/architecture/10-contracts.md`, "the contract is itself checked"): each edge earns trust
/// *before* the graph judges the harness. Every finding is [`Diagnostic::error`] and
/// names the edge.
///
/// Two decidable clauses (`specs/architecture/45-governance.md`): **(a)** the reference `field` is
/// non-empty — an empty field names no reference syntax; **(b)** the target kind is
/// one `temper` models — an unmodeled `to` has no artifacts, so every route over the
/// edge would dangle, making the fault the declaration's, reported once here while
/// [`check`] skips the edge.
///
/// `by_kind` is the same corpus map [`check`] reads; admissibility uses only its keys.
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

/// Check **acyclicity** over the harness reference graph (`specs/architecture/45-governance.md`,
/// "The graph scope"): build the artifact-level graph from the same resolved arcs
/// [`check`] uses and return an error-severity [`Diagnostic`] naming a cycle if one
/// exists. A cycle is a circular import that loads nothing — a true positive.
///
/// Only **resolved** arcs enter: an inadmissible edge is skipped and a dangling
/// reference loads nothing, so neither forges nor masks a cycle (that dangling finding
/// is [`check`]'s). Nodes are keyed `(kind, id)`. At most one finding — a cycle is
/// fatal, and naming one closed chain suffices; the chain is canonicalized (rotated to
/// its least node) so the finding is stable regardless of the traversal's entry node.
#[must_use]
pub fn acyclic(edges: &[Edge], by_kind: &BTreeMap<&str, &[Features]>) -> Vec<Diagnostic> {
    let adjacency = resolved_arcs(edges, by_kind);

    // Three-color DFS: a back edge to a node still on the current path (`Gray`) closes
    // a cycle. Roots and neighbours iterate in sorted order (BTreeMap/BTreeSet), so the
    // first cycle found is deterministic across runs.
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

/// Check the graph-scope **`degree`** predicate (`specs/architecture/45-governance.md`, "The graph
/// scope"; worked example "self-registering vs routed"): for each requirement
/// declaring a [`DegreeBound`](crate::compose::DegreeBound), return an error-severity
/// [`Diagnostic`] per satisfier node whose in/out edge count over the resolved arcs
/// falls outside the bound.
///
/// Declared at the **set scope** (on a requirement) but ranging over the **edge
/// graph**, so it lives here: it reuses [`acyclic`]'s [`resolved_arcs`] and the same
/// opt-in [`roster::is_satisfier`] join the roster scope uses, never a second selector
/// that could disagree. Only **resolved** arcs count (a dangling reference loads
/// nothing; an inadmissible edge is skipped), exactly as in [`acyclic`].
///
/// Unlike route resolution and `acyclic`, `degree` is **opt-in, per-requirement** — a
/// roster declaring no bound does no graph work. A node is `(kind, id)`, so a
/// requirement declaring no `kind` cannot identify its nodes and is skipped.
/// Requirements iterate in name order over name-sorted candidates, so findings are
/// stable across runs.
#[must_use]
pub fn degree(
    requirements: &BTreeMap<String, Requirement>,
    edges: &[Edge],
    by_kind: &BTreeMap<&str, &[Features]>,
) -> Vec<Diagnostic> {
    // Opt-in: with no requirement declaring a bound, the graph is never assembled.
    if requirements
        .values()
        .all(|requirement| requirement.degree.is_none())
    {
        return Vec::new();
    }

    let adjacency = resolved_arcs(edges, by_kind);
    // Incoming degree per node, built once by inverting the resolved arcs; a node
    // absent from the map has in-degree zero.
    let mut incoming: BTreeMap<&Node, usize> = BTreeMap::new();
    for targets in adjacency.values() {
        for target in targets {
            *incoming.entry(target).or_default() += 1;
        }
    }

    let mut diagnostics = Vec::new();
    for requirement in requirements.values() {
        let Some(bound) = &requirement.degree else {
            continue;
        };
        // `degree` needs a declared `kind` to range over; a kind-blind requirement
        // can't identify its nodes and is skipped — `temper` never fabricates a gate
        // the author did not fully declare.
        let Some(kind) = &requirement.kind else {
            continue;
        };
        let candidates = by_kind.get(kind.as_str()).copied().unwrap_or(&[]);
        for features in candidates {
            if !roster::is_satisfier(&requirement.name, features) {
                continue;
            }
            let node = (kind.clone(), features.id.clone());
            let in_degree = incoming.get(&node).copied().unwrap_or(0);
            let out_degree = adjacency.get(&node).map_or(0, BTreeSet::len);

            if let Some(edge_bound) = bound.incoming
                && !edge_bound.admits(in_degree)
            {
                diagnostics.push(out_of_degree(
                    requirement,
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
                    requirement,
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

/// A degree direction — which side of a node's edges a [`DegreeBound`] constrains.
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
/// requirement's bound — naming the requirement, kind, direction, actual count, and
/// the `[min, max]` bound (an open endpoint rendered `∞`).
fn out_of_degree(
    requirement: &Requirement,
    artifact: &str,
    direction: Direction,
    actual: usize,
    bound: EdgeBound,
) -> Diagnostic {
    let min = bound.min.map_or_else(|| "0".to_string(), |n| n.to_string());
    let max = bound.max.map_or_else(|| "∞".to_string(), |n| n.to_string());
    let kind = requirement.kind.as_deref().unwrap_or("any");
    Diagnostic::error(
        GRAPH_DEGREE_RULE,
        artifact,
        format!(
            "requirement `{}` bounds `{kind}` {} degree to [{min}, {max}], but `{artifact}` has {actual}",
            requirement.name,
            direction.label(),
        ),
    )
}

/// Check the graph-scope **`reachable`** predicate (`specs/architecture/45-governance.md`, "The
/// world is a node — reachability is a predicate"): for each member of a kind that
/// declares an [`Activation`], return a finding when the inbound activation edge from
/// the [`world`] node is provably dead — a `description-trigger` member whose named
/// field is blank (the harness loads nothing) or a `paths-match` member whose globs
/// match no file in `repo_files` (the harness activates it never). Each is an exact
/// fact at check time.
///
/// `activations` maps a kind to the single [`Activation`] its definition declares;
/// `by_kind` is the same corpus map the other predicates read; `repo_files` is the
/// repo file-set the `paths-match` globs are tested against — a **parameter**, not a
/// graph dependency, so the blast radius stays this module and the predicate is pure
/// and testable. A kind that declares no activation contributes no entry to
/// `activations` and is not subject; an `always` edge is unconditionally live and an
/// `event` edge carries no repo-decidable dead criterion the spec names, so neither
/// fires. Members iterate in the corpus's candidate order under each name-sorted kind,
/// so findings are stable.
///
/// `severity` is the **assembly's** declaration (`specs/architecture/45-governance.md`, "The world
/// is a node" — resolved `reachability-gate-mechanism` option b): whether a dead edge
/// gates, and at what weight, is the assembly's dial like `degree`, never a member's or
/// a package clause's — a deliberate work-in-progress dead edge stays the author's call.
#[must_use]
pub fn reachable(
    activations: &BTreeMap<&str, Activation>,
    by_kind: &BTreeMap<&str, &[Features]>,
    repo_files: &[String],
    severity: Severity,
) -> Vec<Diagnostic> {
    let world = world();
    let mut diagnostics = Vec::new();
    for (kind, activation) in activations {
        let members = by_kind.get(kind).copied().unwrap_or(&[]);
        for member in members {
            if let Some(reason) = dead_activation(activation, member, repo_files) {
                diagnostics.push(unreachable(&world, kind, &member.id, &reason, severity));
            }
        }
    }
    diagnostics
}

/// Whether a member's declared [`Activation`] edge from the world is **provably dead**,
/// and why — `Some(reason)` names the dead edge for the finding, `None` leaves the
/// member reachable. Only the two edges the spec makes decidable can die here: a blank
/// `description-trigger` field and a `paths-match` field whose *present* globs match no
/// file (an absent/blank `paths` field is unconditional loading, never dead).
/// `always` (unconditionally live) and `event` (no repo-decidable criterion) never do.
fn dead_activation(
    activation: &Activation,
    member: &Features,
    repo_files: &[String],
) -> Option<String> {
    match activation {
        Activation::Always | Activation::Event { .. } => None,
        Activation::DescriptionTrigger { field } => field_is_blank(member, field).then(|| {
            format!("its `{field}` description-trigger field is blank, so the harness has nothing to load")
        }),
        Activation::PathsMatch { field } => {
            // An absent/blank field is unconditional loading, not a dead edge
            // (specs/architecture/15-kinds.md paths-match bullet): only a *present* glob set that
            // matches nothing is provably dead.
            let globs = declared_globs(member, field);
            let dead = !globs.is_empty()
                && !globs.iter().any(|glob| glob_matches_any(glob, repo_files));
            dead.then(|| {
                format!("its `{field}` globs match no file in the repository, so the harness activates it never")
            })
        }
    }
}

/// Whether a member's activation field is **blank** — absent, or a scalar whose text is
/// empty or all whitespace. A blank `description-trigger` field means the harness has
/// nothing to load, so the edge is dead. A list/map value carries content and is never
/// blank (a `description` is a scalar; a container there is another finding's to own).
fn field_is_blank(member: &Features, field: &str) -> bool {
    match member.field(field) {
        None => true,
        Some(FeatureValue::Scalar { text, .. }) => text.trim().is_empty(),
        Some(FeatureValue::List(_) | FeatureValue::Map) => false,
    }
}

/// The activation globs a member declares on `field`: a scalar names one glob, a list
/// names each of several, and an absent field or a map (which carries no glob) names
/// none. Read off [`Features`] — a declared field, never grepped. Declaring none is
/// *not* a dead edge: an absent/blank `paths` field falls back to unconditional loading
/// (specs/architecture/15-kinds.md paths-match bullet), so the caller only tests for the dead edge
/// once at least one glob is present.
fn declared_globs(member: &Features, field: &str) -> Vec<String> {
    match member.field(field) {
        None | Some(FeatureValue::Map) => Vec::new(),
        Some(FeatureValue::Scalar { text, .. }) => {
            let glob = text.trim();
            if glob.is_empty() {
                Vec::new()
            } else {
                vec![glob.to_string()]
            }
        }
        Some(FeatureValue::List(items)) => items
            .iter()
            .map(|glob| glob.trim().to_string())
            .filter(|glob| !glob.is_empty())
            .collect(),
    }
}

/// Whether `glob` matches at least one path in `files`, decided over a regex compiled
/// from the glob (the sanctioned `regex` crate — `specs/architecture/45-governance.md` leaves
/// zero-match globs to this module). A glob `temper` cannot compile is treated as
/// matching, so the gate never cries wolf on a pattern it failed to understand — though
/// [`glob_to_regex`] is total, so that branch is defensive only.
fn glob_matches_any(glob: &str, files: &[String]) -> bool {
    match Regex::new(&glob_to_regex(glob)) {
        Ok(regex) => files.iter().any(|file| regex.is_match(file)),
        Err(_) => true,
    }
}

/// Compile a path glob to an anchored regex: `**` crosses directory separators (`**/`
/// matching any number of leading segments, including none), a single `*` and `?` stay
/// within one segment, and every other character matches literally (a regex
/// metacharacter escaped). Total — every input yields a valid pattern.
fn glob_to_regex(glob: &str) -> String {
    let chars: Vec<char> = glob.chars().collect();
    let mut regex = String::from("^");
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '*' if i + 1 < chars.len() && chars[i + 1] == '*' => {
                // `**` crosses `/`: `**/` (a segment wildcard) matches any number of
                // leading path segments, including none; a trailing `**` any suffix.
                i += 1;
                if i + 1 < chars.len() && chars[i + 1] == '/' {
                    regex.push_str("(?:.*/)?");
                    i += 1;
                } else {
                    regex.push_str(".*");
                }
            }
            '*' => regex.push_str("[^/]*"),
            '?' => regex.push_str("[^/]"),
            c => {
                if is_regex_meta(c) {
                    regex.push('\\');
                }
                regex.push(c);
            }
        }
        i += 1;
    }
    regex.push('$');
    regex
}

/// Whether `c` is a regex metacharacter that must be escaped to match literally. `*`
/// and `?` are consumed as glob wildcards before this is reached, so they are absent.
fn is_regex_meta(c: char) -> bool {
    matches!(
        c,
        '.' | '+' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '^' | '$' | '\\'
    )
}

/// The finding for a member whose inbound activation edge from the [`world`] node is
/// dead — naming the world, the member (kind + id), and the dead-edge reason, at the
/// assembly-declared `severity` (`specs/architecture/45-governance.md`).
fn unreachable(world: &Node, kind: &str, id: &str, reason: &str, severity: Severity) -> Diagnostic {
    Diagnostic::new(
        severity,
        GRAPH_REACHABLE_RULE,
        id,
        format!(
            "the activation edge from the {} node to {kind} `{id}` is dead — {reason}",
            world.0
        ),
    )
}

/// Enumerate every **resolved** reference edge: for each admissible edge, each source
/// of its `from` kind, and each named target that resolves to a real artifact of its
/// `to` kind, one [`ResolvedEdge`]. The single arc-resolution primitive —
/// [`resolved_arcs`] folds it into adjacency for [`acyclic`]/[`degree`] and
/// `crate::read` filters it per node — so gate and read narrate the *same* edges
/// (READ-EDGE-UNIFY). An inadmissible edge is skipped and a dangling reference yields
/// no edge (route resolution owns that). Sources iterate in name-sorted order for a
/// stable enumeration; a target named twice yields two edges, deduped into one arc by
/// [`resolved_arcs`].
pub(crate) fn resolved_edges(
    edges: &[Edge],
    by_kind: &BTreeMap<&str, &[Features]>,
) -> Vec<ResolvedEdge> {
    let mut resolved = Vec::new();
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
                // A dangling reference loads nothing — no resolved edge.
                if targets.contains(target) {
                    resolved.push(ResolvedEdge {
                        from: (edge.from.clone(), source.id.clone()),
                        field: edge.field.clone(),
                        to: (edge.to.clone(), target.to_string()),
                    });
                }
            }
        }
    }
    resolved
}

/// Build the artifact-level directed graph over **resolved** arcs — the shared
/// foundation [`acyclic`] and [`degree`] range over — by folding [`resolved_edges`]
/// into `(kind, id)`-keyed adjacency. Arcs dedupe in the [`BTreeSet`], so a target
/// named twice is one arc. Deriving it from the same [`resolved_edges`] the read family
/// consumes keeps the gate's checks and `temper why` in lockstep.
fn resolved_arcs(
    edges: &[Edge],
    by_kind: &BTreeMap<&str, &[Features]>,
) -> BTreeMap<Node, BTreeSet<Node>> {
    let mut adjacency: BTreeMap<Node, BTreeSet<Node>> = BTreeMap::new();
    for ResolvedEdge { from, to, .. } in resolved_edges(edges, by_kind) {
        adjacency.entry(from).or_default().insert(to);
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
                    // A back edge closes a cycle; the node is on `path` by the
                    // invariant Gray ⇔ on the current path.
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
    // Name the finding after the ring's least node (the chain's start) so `artifact`
    // is stable and points into the cycle.
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
/// `rule.routes_to`).
fn edge_id(edge: &Edge) -> String {
    format!("{}.{}", edge.from, edge.field)
}

/// The finding for a route that resolves to no artifact — naming the source, the
/// reference field, the dangling target, and the target kind.
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
            sections: Vec::new(),
            source_dir: Some(name.to_string()),
            directives: Vec::new(),
            satisfies: Vec::new(),
            published_requirements: Vec::new(),
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
        // later); just silent, mirroring a non-required requirement over an unmodeled kind.
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

    /// Parse a `temper.toml` fragment's `[requirement.<name>]` tables into the typed
    /// roster the [`degree`] check reads — the parse foundation is the only constructor
    /// for a [`Requirement`]'s `degree` bound, so the graph tests drive it.
    fn requirements(toml: &str) -> BTreeMap<String, crate::compose::Requirement> {
        AuthorLayer::parse(toml, Path::new("temper.toml"))
            .unwrap()
            .requirements()
            .clone()
    }

    /// A requirement whose satisfier nodes are the skills opting into `gate`, declaring
    /// a `degree` bound `clause` (an inline `{ … }` body). The graph the degree check
    /// ranges over is the caller's `edges`/`by_kind`; the satisfier nodes are the skills
    /// whose `satisfies` names `gate`. No `package` is needed — the degree check reads
    /// the edge graph, not a contract.
    fn degree_requirement(clause: &str) -> BTreeMap<String, crate::compose::Requirement> {
        requirements(&format!(
            "[requirement.gate]\n\
             kind = \"skill\"\n\
             degree = {{ {clause} }}\n"
        ))
    }

    /// A node that opts into the named requirement via `satisfies` — the degree tests'
    /// way to place a node in a requirement's satisfier set.
    fn satisfying(mut features: Features, requirement: &str) -> Features {
        features.satisfies.push(requirement.to_string());
        features
    }

    #[test]
    fn a_self_registering_bound_passes_when_the_node_is_not_pointed_at() {
        // `incoming = { max = 0 }`: the skill `standards` must not be pointed at. No
        // rule routes to it (the only rule routes nowhere), so its incoming degree is
        // zero — inside the bound, clean.
        let requirements = degree_requirement("incoming = { max = 0 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", None)];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(degree(&requirements, &edges, &by_kind).is_empty());
    }

    #[test]
    fn a_self_registering_bound_fires_when_the_node_is_pointed_at() {
        // The rule `style` routes to `standards`, so the skill has incoming degree 1 —
        // outside `incoming = { max = 0 }`. A self-registering artifact must not be
        // reached: an error naming the requirement, the artifact, and the direction.
        let requirements = degree_requirement("incoming = { max = 0 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(&requirements, &edges, &by_kind);
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
        let requirements = degree_requirement("incoming = { min = 1 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(degree(&requirements, &edges, &by_kind).is_empty());
    }

    #[test]
    fn a_routed_bound_fires_when_the_node_is_unreachable() {
        // No rule routes to `standards`, so its incoming degree is zero — outside
        // `incoming = { min = 1 }`. A routed artifact must be reachable: an error.
        let requirements = degree_requirement("incoming = { min = 1 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", None)];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(&requirements, &edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, GRAPH_DEGREE_RULE);
        assert_eq!(diags[0].artifact, "standards");
        assert!(diags[0].message.contains("incoming"));
    }

    #[test]
    fn an_outgoing_bound_reads_the_satisfier_node_out_degree() {
        // Degree bounds both directions: the rule `style` (a `gate` satisfier under an
        // `outgoing` bound) routes to one skill, so its out-degree is 1 — outside
        // `{ max = 0 }`.
        let requirements = requirements(
            "[requirement.gate]\n\
             kind = \"rule\"\n\
             degree = { outgoing = { max = 0 } }\n",
        );
        let edges = [routes_to_edge()];
        let rules = [satisfying(node("style", Some("standards")), "gate")];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(&requirements, &edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].artifact, "style");
        assert!(diags[0].message.contains("outgoing"));
    }

    #[test]
    fn a_roster_declaring_no_degree_bound_does_no_graph_work() {
        // `degree` is opt-in, per-requirement: a requirement with no bound is silent over a
        // graph that would violate one — `temper` never fabricates a gate the author
        // did not declare (`00-intent.md` law 4).
        let requirements = requirements(
            "[requirement.gate]\n\
             kind = \"skill\"\n\
             package = \"skill.anthropic\"\n",
        );
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(degree(&requirements, &edges, &by_kind).is_empty());
    }

    #[test]
    fn glob_to_regex_matches_common_path_globs_within_and_across_segments() {
        // `**/` matches any number of leading segments including none, `*` stays within
        // one segment, and a literal `.` is escaped — the semantics `reachable` leans on
        // to decide a `paths-match` glob dead.
        let recursive = Regex::new(&glob_to_regex("**/*.rs")).unwrap();
        assert!(recursive.is_match("foo.rs"));
        assert!(recursive.is_match("src/a/foo.rs"));
        assert!(!recursive.is_match("foo.md"));

        let subtree = Regex::new(&glob_to_regex("src/**")).unwrap();
        assert!(subtree.is_match("src/graph.rs"));
        assert!(subtree.is_match("src/a/b.rs"));
        assert!(!subtree.is_match("tests/graph.rs"));

        // A single `*` does not cross a `/`.
        let flat = Regex::new(&glob_to_regex("*.md")).unwrap();
        assert!(flat.is_match("README.md"));
        assert!(!flat.is_match("docs/README.md"));
    }

    #[test]
    fn the_world_node_is_a_stable_reserved_identity() {
        // The distinguished world node keys under a reserved `world` kind, so a
        // reachability finding can name the edge's source without colliding with any
        // artifact kind.
        assert_eq!(world(), ("world".to_string(), "world".to_string()));
    }

    #[test]
    fn a_dangling_reference_does_not_count_toward_degree() {
        // The rule routes to `absent`, which resolves to no skill — a dangling arc
        // that loads nothing, so `standards` has incoming degree zero and a routed
        // `{ min = 1 }` bound fires. The dangling reference neither forges nor masks a
        // degree, exactly as it neither forges nor masks a cycle.
        let requirements = degree_requirement("incoming = { min = 1 }");
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("absent"))];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(&requirements, &edges, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].artifact, "standards");
    }
}
