//! The harness reference graph — route resolution over declared edges.
//!
//! The harness is a graph: skills and rules pointing at each other through
//! **declared** reference fields, read off [`Features`], never grepped from a body.
//! Nodes are `(kind, id)`
//! across every kind; edges are the [`Edge`] relationships declared on the surface.
//! Five checks range over it: [`check`] (route resolution — a reference resolves to a
//! real target), [`admissibility`] (each edge names its field and a modeled target
//! kind, checked before the graph is trusted), [`acyclic`] (no circular import),
//! [`degree`] (a satisfier node's in/out count lands in a requirement's bound), and
//! [`reachable`]. The first four
//! range over one resolved-edge enumeration ([`resolved_edges`]), shared with
//! `crate::read`'s narration so gate and read never disagree (READ-EDGE-UNIFY).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use crate::check::{Diagnostic, Severity};
use crate::compose::{Edge, Requirement};
use crate::contract::{EdgeBound, Predicate};
use crate::engine::{self, Selection};
use crate::extract::{FeatureValue, Features};
use crate::kind::Registration;

/// The diagnostic `rule` id every route-resolution finding reports under.
const GRAPH_ROUTE_RULE: &str = "graph.route";

/// The diagnostic `rule` id every graph-admissibility finding reports under.
const GRAPH_ADMISSIBILITY_RULE: &str = "graph.admissibility";

/// The diagnostic `rule` id the acyclicity finding reports under.
const GRAPH_ACYCLIC_RULE: &str = "graph.acyclic";

/// The diagnostic `rule` id every reachability finding reports under.
const GRAPH_REACHABLE_RULE: &str = "graph.reachable";

/// The diagnostic `rule` id every unbacked-pointer directive finding reports under.
const GRAPH_DIRECTIVE_UNBACKED_RULE: &str = "graph.directive-unbacked";

/// The reference `field` a directive-produced [`ResolvedEdge`] records — the
/// `at-import` syntax that observed it, not a frontmatter field. Lets a reader
/// tell a directive edge from a declared reference edge in the one resolved-edge set.
const DIRECTIVE_FIELD: &str = "at-import";

/// The reference `field` a mention-produced [`ResolvedEdge`] records — an authored
/// `n`, not a frontmatter field. Lets a reader tell a mention edge from a declared
/// reference edge in the one resolved-edge set.
const MENTION_FIELD: &str = "mention";

/// The reference `field` a layout-prose-import [`ResolvedEdge`] records — a declared
/// include of a file's contents, not a frontmatter field. Lets a reader tell an import
/// edge from a declared reference edge in the one resolved-edge set.
const IMPORT_FIELD: &str = "import";

/// The maximum import-recursion depth reachability propagates a live importer's
/// liveness across — the `at-import` grammar is recursion-capped at five hops
/// (code.claude.com/docs/en/memory, retrieved 2026-07-02), so an import chain
/// deeper than this loads nothing at runtime and cannot carry liveness either.
const MAX_IMPORT_HOPS: usize = 5;

/// A node in the artifact-level reference graph: `(kind, id)`. An id is unique only
/// *within* a kind and an edge resolves only within its target kind, so the kind is
/// part of the identity — else a same-named rule and skill collapse into one node and
/// forge or mask a cycle.
///
/// Exposed so the read family (`crate::read`) keys a member's resolved in/out
/// edges on the *same* `(kind, id)` node the gate does (READ-EDGE-UNIFY), and so the
/// directive classing ([`classify_directives`]) names the endpoints of the edges it
/// yields.
pub type Node = (String, String);

/// The distinguished **world** node — the harness runtime and repo `temper` observes
/// but does not govern. Registration
/// facts are its edges *into* members; [`reachable`]
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
/// which declared reference produced the edge. Also the type [`classify_directives`]
/// yields a member-class directive occurrence as, so an observed `@import` edge enters
/// the same enumeration a declared reference edge does.
#[derive(Clone)]
pub struct ResolvedEdge {
    /// The source node `(kind, id)` carrying the reference.
    pub from: Node,
    /// The reference field the edge was declared under (`routes_to`).
    pub field: String,
    /// The target node `(kind, id)` the reference resolved to.
    pub to: Node,
}

/// Check **route resolution** over the harness reference graph:
/// for each declared [`Edge`], read its reference field
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

        let sources = by_kind.get(edge.from.as_str()).copied().unwrap_or(&[]);
        for source in sources {
            for target in edge_targets(source, &edge.field) {
                // The finding names the *authored* spelling, never the normalized
                // identity — the author has to find what they wrote.
                let resolved = target_identity(&target, &edge.to)
                    .is_some_and(|(kind, identity)| resolves(by_kind, kind, identity));
                if !resolved {
                    diagnostics.push(dangling(edge, source.id.as_str(), &target));
                }
            }
        }
    }
    diagnostics
}

/// Validate the declared edges against **the definition** — admissibility:
/// each edge earns trust
/// *before* the graph judges the harness. Every finding is [`Diagnostic::error`] and
/// names the edge.
///
/// Two decidable clauses: **(a)** the reference `field` is
/// non-empty — an empty field names no reference syntax; **(b)** every kind the `to`
/// set declares is one `temper` models — an unmodeled element has no artifacts, so the
/// routes it would take can never resolve, making the fault the declaration's. The
/// finding names the offending *element*, not the whole set, and [`check`] skips the
/// edge.
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
                    "edge from `{}` to {} declares an empty reference field, which names no reference syntax",
                    edge.from,
                    render_target_kinds(&edge.to)
 ),
 ));
        }

        // (b) Every declared target kind is one `temper` models — else the routes it
        // would take can never resolve.
        for kind in &edge.to {
            if !by_kind.contains_key(kind.as_str()) {
                diagnostics.push(Diagnostic::error(
                    GRAPH_ADMISSIBILITY_RULE,
                    edge_id(edge),
 format!(
                        "edge `{}` targets kind `{kind}`, which `temper` does not model — no route can ever resolve",
 edge.field
 ),
 ));
            }
        }
    }
    diagnostics
}

/// Check **acyclicity** over the harness reference graph: build the artifact-level
/// graph from the same resolved arcs [`check`] uses and return an error-severity
/// [`Diagnostic`] naming a cycle if one exists. A cycle is a circular import that
/// loads nothing — a true positive.
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

/// Check the **`degree`** predicate over every declared [`Selection`]: for each `degree`
/// clause bound to one, return a [`Diagnostic`] — at the clause's own declared severity
/// — per selected member whose in/out edge count over the resolved arcs falls outside
/// the bound.
///
/// `degree` is the one set predicate this module judges rather than
/// [`engine::judge`]: the clause is each-grain over the selection's members and
/// whole-grain over each member's own **by-incidence** selection — the edges at it,
/// filtered by direction — which is the graph, not a fact the members carry. It reuses
/// [`acyclic`]'s [`resolved_arcs`], so only **resolved** arcs count (a dangling
/// reference loads nothing; an inadmissible edge is skipped).
///
/// Unlike route resolution and `acyclic`, `degree` is **opt-in** — selections declaring
/// no `degree` clause do no graph work. A node is `(kind, id)`, so a selection whose
/// members span kinds keys each by its *own* label. Selections, their clauses, and their
/// members all arrive in the caller's order, which is stable across runs.
///
/// `mention_edges` folds the already-resolved mention edges into the same adjacency —
/// a mention is obligation-free by default (no shipped clause counts it), but an
/// authored `degree` clause may range over it exactly as it does a declared reference
/// edge.
#[must_use]
pub fn degree(
    selections: &[Selection],
    edges: &[Edge],
    mention_edges: &[ResolvedEdge],
    by_kind: &BTreeMap<&str, &[Features]>,
) -> Vec<Diagnostic> {
    let any_degree_clause = selections.iter().any(|selection| {
        selection
            .clauses
            .iter()
            .any(|clause| matches!(clause.predicate, Predicate::Degree { .. }))
    });
    if !any_degree_clause {
        return Vec::new();
    }

    let mut adjacency = resolved_arcs(edges, by_kind);
    for edge in mention_edges {
        adjacency
            .entry(edge.from.clone())
            .or_default()
            .insert(edge.to.clone());
    }
    // Incoming degree per node, built once by inverting the resolved arcs; a node
    // absent from the map has in-degree zero.
    let mut incoming: BTreeMap<&Node, usize> = BTreeMap::new();
    for targets in adjacency.values() {
        for target in targets {
            *incoming.entry(target).or_default() += 1;
        }
    }

    let mut diagnostics = Vec::new();
    for selection in selections {
        for clause in &selection.clauses {
            let Predicate::Degree {
                incoming: incoming_bound,
                outgoing: outgoing_bound,
            } = &clause.predicate
            else {
                continue;
            };
            for (kind, features) in &selection.members {
                let node = ((*kind).to_string(), features.id.clone());
                let in_degree = incoming.get(&node).copied().unwrap_or(0);
                let out_degree = adjacency.get(&node).map_or(0, BTreeSet::len);

                if let Some(edge_bound) = incoming_bound
                    && !edge_bound.admits(in_degree)
                {
                    diagnostics.push(out_of_degree(
                        selection,
                        clause,
                        &features.id,
                        Direction::Incoming,
                        in_degree,
                        *edge_bound,
                    ));
                }
                if let Some(edge_bound) = outgoing_bound
                    && !edge_bound.admits(out_degree)
                {
                    diagnostics.push(out_of_degree(
                        selection,
                        clause,
                        &features.id,
                        Direction::Outgoing,
                        out_degree,
                        *edge_bound,
                    ));
                }
            }
        }
    }
    diagnostics
}

/// Check the **`mention-reachable`** predicate over every declared [`Selection`]: for
/// each clause bound to one, return a [`Diagnostic`] — at the clause's own declared
/// severity — per selected member whose authored mention cannot fire where its target
/// can be invoked.
///
/// A target is **gated** when its own `gate_field` carries globs: the harness removes it
/// from every invocation channel until it reads a file the gate matches. The trigger is
/// that field carrying a non-empty value — never the target's kind, and never a
/// `paths-match` registration lookup: a gate is a field a kind *documents*, and a kind
/// may gate while declaring no `paths-match` channel at all (a skill's `paths` is
/// exactly that — it gates every channel and adds no registration entry,
/// `sdk/src/builtins.ts`). Reading the registration set instead would select rules and
/// never skills, so the rule→skill mention this check exists for could never fire.
///
/// Two diagnoses, one invariant (`specs/decisions/0028-…`): a **scoped** source whose
/// scope globs are not contained in the target's gate, and an **unscoped** source
/// mentioning a gated target. Silent otherwise — an ungated target imposes nothing, and
/// a mention resolving to no composed member is [`route_mentions`]'s verdict, never
/// double-reported here.
///
/// **Declared leniency:** containment is *literal* — every source glob must appear
/// verbatim in the target's gate set. True glob-set containment is undecidable, so this
/// errs toward firing on a semantically contained narrower glob rather than staying
/// silent; a clause naming the predicate therefore ships at advisory severity. This is
/// the same leniency *direction* [`dead_registration`] takes with an uncompilable glob:
/// neither cries wolf about a pattern it cannot decide.
///
/// Like `degree`, this is **opt-in** — selections declaring no `mention-reachable`
/// clause do no graph work — and ranges over the same unified enumeration `degree`
/// folds: the declared field edges resolved to real targets ([`resolved_edges`]) plus
/// the already-resolved mention and import edges. A reference to a gated target can fire
/// where that target cannot be invoked whatever locus declared it, so a rendering claim
/// carried on a field edge is judged rather than dropped for riding a family this check
/// once read alone.
///
/// An edge a **body-carried** member declares is judged under its *host*'s scope, not
/// the embedded member's own: the embedded source keys to `(embedded-kind, key)`, so
/// `embedded_hosts` maps it back to `(host-kind, host-id)` and the host's `scope_field`
/// gates it. This mirrors the target-side [`target_identity`] seam — a member's
/// reference set is the union of the edges its fields *and* its embedded members
/// declare, so a body-carried consult is the host's citation to scope.
#[must_use]
pub fn mention_reachable(
    selections: &[Selection],
    edges: &[Edge],
    mention_edges: &[ResolvedEdge],
    by_kind: &BTreeMap<&str, &[Features]>,
    embedded_hosts: &BTreeMap<Node, Node>,
) -> Vec<Diagnostic> {
    let any_clause = selections.iter().any(|selection| {
        selection
            .clauses
            .iter()
            .any(|clause| matches!(clause.predicate, Predicate::MentionReachable { .. }))
    });
    if !any_clause {
        return Vec::new();
    }
    let mut all_edges = resolved_edges(edges, by_kind);
    all_edges.extend(mention_edges.iter().cloned());

    let mut diagnostics = Vec::new();
    for selection in selections {
        for clause in &selection.clauses {
            let Predicate::MentionReachable {
                scope_field,
                gate_field,
            } = &clause.predicate
            else {
                continue;
            };
            for (kind, features) in &selection.members {
                let source = ((*kind).to_string(), features.id.clone());
                for edge in all_edges.iter().filter(|edge| {
                    edge.from == source || embedded_hosts.get(&edge.from) == Some(&source)
                }) {
                    // A mention whose target composes no member has no gate to read —
                    // `route_mentions` owns that verdict, so this clause stays silent.
                    let Some(target) = member_at(&edge.to, by_kind) else {
                        continue;
                    };
                    let gate = declared_globs(target, gate_field);
                    if gate.is_empty() {
                        continue;
                    }
                    let scope = declared_globs(features, scope_field);
                    let message = if scope.is_empty() {
                        format!(
                            "`{}` is unscoped, but its mention of `{}:{}` is actionable only \
                             inside that member's `{gate_field}` gate ({}): scope \
                             `{}`'s `{scope_field}` to the gate, or ungate the target",
                            features.id,
                            edge.to.0,
                            edge.to.1,
                            quoted(&gate),
                            features.id,
                        )
                    } else if let Some(uncovered) = uncontained(&scope, &gate) {
                        format!(
                            "`{}`'s `{scope_field}` glob `{uncovered}` is not in the \
                             `{gate_field}` gate of the member it mentions, `{}:{}` ({}), so \
                             the mention can fire where that member cannot be invoked",
                            features.id,
                            edge.to.0,
                            edge.to.1,
                            quoted(&gate),
                        )
                    } else {
                        continue;
                    };
                    diagnostics.push(
                        Diagnostic::new(
                            engine::severity_of(clause.severity),
                            &clause.label,
                            &features.id,
                            message,
                        )
                        .with_guidance(clause.guidance.clone()),
                    );
                }
            }
        }
    }
    diagnostics
}

/// The member a resolved node addresses, or `None` when the corpus composes none of
/// that kind and id — a bare requirement target (which carries no gate to read) or a
/// dangler [`route_mentions`] owns. The same corpus lookup [`edge_resolves`] makes to
/// decide a mention's route, read here for the target's own features.
fn member_at<'f>(node: &Node, by_kind: &BTreeMap<&str, &'f [Features]>) -> Option<&'f Features> {
    let (kind, name) = node;
    by_kind
        .get(kind.as_str())?
        .iter()
        .find(|features| features.id == *name)
}

/// The first source glob that appears in no gate glob **verbatim**, or `None` when the
/// gate literally contains every one. The declared leniency: literal containment, since
/// true glob-set containment is undecidable.
fn uncontained(scope: &[String], gate: &[String]) -> Option<String> {
    scope
        .iter()
        .find(|glob| !gate.contains(glob))
        .map(ToString::to_string)
}

/// A glob set rendered for a finding — backticked and comma-joined, in declaration
/// order.
fn quoted(globs: &[String]) -> String {
    globs
        .iter()
        .map(|glob| format!("`{glob}`"))
        .collect::<Vec<String>>()
        .join(", ")
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

/// The finding for a selected member whose `degree` in one direction falls outside its
/// clause's bound — naming the selection, the direction, the actual count, and the
/// `[min, max]` bound (an open endpoint rendered `∞`).
fn out_of_degree(
    selection: &Selection,
    clause: &crate::contract::Clause,
    artifact: &str,
    direction: Direction,
    actual: usize,
    bound: EdgeBound,
) -> Diagnostic {
    let min = bound.min.map_or_else(|| "0".to_string(), |n| n.to_string());
    let max = bound.max.map_or_else(|| "∞".to_string(), |n| n.to_string());
    Diagnostic::new(
        engine::severity_of(clause.severity),
        &clause.label,
        artifact,
        format!(
            "{} bounds {} degree to [{min}, {max}], but `{artifact}` has {actual}",
            selection.selector.noun(),
            direction.label(),
        ),
    )
    .with_guidance(clause.guidance.clone())
}

/// Check the graph-scope **`reachable`** predicate: a member is reachable when its own
/// inbound registration edge from the [`world`] node is live **or a reachable member
/// imports it** — the closure over the observed directive edges. A member's own edge is
/// live iff **any one channel** of its kind's declared registration set is live — user
/// invocation and description trigger are channels, not rivals (`builtins.md`, "The
/// shipped kinds"). Return a finding only when *every* channel is provably dead — a
/// `description-trigger` field that is blank (the harness loads nothing) or a
/// `paths-match` glob set matching no file in `repo_files` (the harness activates it
/// never) — *and* no live importer reaches the member. Each channel's dead criterion is
/// an exact fact at check time.
///
/// `registrations` maps a kind to the declared [`Registration`] **set** its definition
/// carries; `by_kind` is the same corpus map the other predicates read; `repo_files` is
/// the repo file-set the `paths-match` globs are tested against; `edges` is the observed
/// member→member directive edge set ([`classify_directives`]'s `edges`) reachability
/// closes over. All are **parameters**, not graph dependencies, so the blast radius
/// stays this module and the predicate is pure and testable. A kind that declares no
/// registration contributes no entry to `registrations` and is not subject to a *finding*,
/// but its members are unconditionally live and so can carry liveness across an import
/// edge (a memory member that imports a rule); an `always`/`user-invoked` channel is
/// unconditionally live and an `event` channel carries no repo-decidable dead criterion
/// the spec names, so neither ever contributes a dead reason. Liveness propagates along a
/// directive edge from a live importer to its target, hop-capped at [`MAX_IMPORT_HOPS`]
/// as the format documents, so the target inherits the importer's liveness
/// conditionally. Members iterate in the corpus's candidate order under each name-sorted
/// kind, so findings are stable.
///
/// `severity` is the **assembly's** declaration: whether a dead edge
/// gates, and at what weight, is the assembly's dial like `degree`, never a member's own
/// clause — a deliberate work-in-progress dead edge stays the author's call.
#[must_use]
pub fn reachable(
    registrations: &BTreeMap<&str, Vec<Registration>>,
    by_kind: &BTreeMap<&str, &[Features]>,
    repo_files: &[String],
    edges: &[ResolvedEdge],
    severity: Severity,
) -> Vec<Diagnostic> {
    let world = world();
    // The reachability closure: every member reachable from the world — own registration
    // live, or reached along a directive edge from a live importer within the hop cap.
    let live = live_members(registrations, by_kind, repo_files, edges);
    let mut diagnostics = Vec::new();
    for (kind, channels) in registrations {
        let members = by_kind.get(kind).copied().unwrap_or(&[]);
        for member in members {
            // Fire only when every channel is dead *and* no live importer reaches the
            // member — conditional inheritance: a dead-own member imported by a reachable
            // one is live, so it stays silent.
            if let Some(reason) = dead_channel_set(channels, member, repo_files) {
                let node = ((*kind).to_string(), member.id.clone());
                if !live.contains(&node) {
                    diagnostics.push(unreachable(&world, kind, &member.id, &reason, severity));
                }
            }
        }
    }
    diagnostics
}

/// The members whose reachability **dies** if the node `removed` is deleted or renamed:
/// every member live now that is no longer live
/// once `removed`, and every directive edge touching it, is excised from the graph. The
/// blast radius the graph promises, read over the
/// same [`live_members`] closure [`reachable`] stands on so the read agrees with the gate
/// (READ-EDGE-UNIFY). `removed` itself is excluded — a removed member is trivially gone,
/// not orphaned. Returned in sorted `(kind, id)` order for a stable narration.
///
/// A member drops out only through the *import* path: its own registration is dead and its
/// sole live route was a directive edge from `removed` (or a chain through it), so
/// re-running the closure without `removed` — and without any directive edge into or out
/// of it — leaves it unreached. A member with a live own registration never drops, so this
/// is silent unless `removed` was carrying another's liveness.
#[must_use]
pub(crate) fn reachability_orphaned(
    removed: &Node,
    registrations: &BTreeMap<&str, Vec<Registration>>,
    by_kind: &BTreeMap<&str, &[Features]>,
    repo_files: &[String],
    edges: &[ResolvedEdge],
) -> Vec<Node> {
    let live_before = live_members(registrations, by_kind, repo_files, edges);

    // The graph with `removed` excised: its kind loses the member, and every directive
    // edge touching it (in or out) goes with it. Owned so the reduced corpus outlives
    // the borrowed `by_kind` view the closure reads.
    let mut owned: BTreeMap<&str, Vec<Features>> = BTreeMap::new();
    for (&kind, members) in by_kind {
        let kept: Vec<Features> = members
            .iter()
            .filter(|features| !(kind == removed.0 && features.id == removed.1))
            .cloned()
            .collect();
        owned.insert(kind, kept);
    }
    let reduced_by_kind: BTreeMap<&str, &[Features]> = owned
        .iter()
        .map(|(&kind, members)| (kind, members.as_slice()))
        .collect();
    let reduced_edges: Vec<ResolvedEdge> = edges
        .iter()
        .filter(|edge| edge.from != *removed && edge.to != *removed)
        .map(|edge| ResolvedEdge {
            from: edge.from.clone(),
            field: edge.field.clone(),
            to: edge.to.clone(),
        })
        .collect();
    let live_after = live_members(registrations, &reduced_by_kind, repo_files, &reduced_edges);

    live_before
        .into_iter()
        .filter(|node| node != removed && !live_after.contains(node))
        .collect()
}

/// The set of members reachable from the [`world`] node — the closure the [`reachable`]
/// predicate consults. Seeds every member whose **own** registration edge is live (its
/// kind declares no registration ⇒ unconditionally live, or [`dead_channel_set`] finds at
/// least one channel of its declared set live), then propagates liveness along the
/// observed directive `edges` from a live
/// importer to its target, breadth-first and capped at [`MAX_IMPORT_HOPS`] hops (the
/// `at-import` recursion depth the format documents) — a target reached within the cap
/// of a live importer inherits its liveness.
fn live_members(
    registrations: &BTreeMap<&str, Vec<Registration>>,
    by_kind: &BTreeMap<&str, &[Features]>,
    repo_files: &[String],
    edges: &[ResolvedEdge],
) -> BTreeSet<Node> {
    // Seed: every member whose own world-edge is live. A kind absent from `registrations`
    // declares no registration, so its members load unconditionally and seed the closure —
    // `by_kind` carries every kind, so an always-live importer is in scope.
    let mut live: BTreeSet<Node> = BTreeSet::new();
    for (kind, members) in by_kind {
        for member in *members {
            let own_live = match registrations.get(kind) {
                None => true,
                Some(channels) => dead_channel_set(channels, member, repo_files).is_none(),
            };
            if own_live {
                live.insert(((*kind).to_string(), member.id.clone()));
            }
        }
    }

    // Propagate along directive edges, one hop per round, capped at the format's import
    // recursion depth. Each newly-live node expands exactly once, the round after it
    // goes live, so a chain longer than the cap carries no liveness — as the runtime's
    // recursion cap loads nothing past it.
    let mut frontier: BTreeSet<Node> = live.iter().cloned().collect();
    for _ in 0..MAX_IMPORT_HOPS {
        let mut next: BTreeSet<Node> = BTreeSet::new();
        for edge in edges {
            if frontier.contains(&edge.from) && !live.contains(&edge.to) {
                live.insert(edge.to.clone());
                next.insert(edge.to.clone());
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    live
}

/// Whether every channel of a member's declared [`Registration`] **set** is provably
/// dead, and why — `Some(reason)` joins each dead channel's own reason for the finding,
/// `None` leaves the member reachable because at least one channel is live (or the set
/// is empty — nothing to evaluate, the caller's job to treat as unconditionally live).
/// The member's world edge is live iff any one channel is, so this only fires when
/// [`dead_registration`] finds every channel in `channels` dead.
fn dead_channel_set(
    channels: &[Registration],
    member: &Features,
    repo_files: &[String],
) -> Option<String> {
    if channels.is_empty() {
        return None;
    }
    let reasons: Vec<String> = channels
        .iter()
        .filter_map(|channel| dead_registration(channel, member, repo_files))
        .collect();
    (reasons.len() == channels.len()).then(|| reasons.join("; "))
}

/// Whether one declared registration **channel** is **provably dead** on its own, and
/// why — `Some(reason)` names the dead channel for the finding, `None` leaves it live.
/// Only three channels the spec makes decidable can die here: a blank
/// `description-trigger` field, a `paths-match` field whose *present* globs match no
/// file (an absent/blank `paths` field is unconditional loading, never dead), and an
/// `enablement` entry the harness is documented not to load. `always`/`user-invoked`
/// (unconditionally live), `event`, and `connection`/`registry` (no repo-decidable
/// criterion) never do.
fn dead_registration(
    registration: &Registration,
    member: &Features,
    repo_files: &[String],
) -> Option<String> {
    match registration {
        Registration::Always
        | Registration::UserInvoked
        | Registration::Event { .. }
        | Registration::Connection
        | Registration::Registry => None,
        // The gate rides the member's declared enablement field, never a channel of its
        // own: only the documented `false` is dead. Any other value — the enablement the
        // harness writes, or a shape whose semantics no source documents — stays live, so
        // an entry the format admits is never called dead on a guess.
        Registration::Enablement => matches!(
            member.field(crate::kind::ENABLEMENT_FIELD),
            Some(FeatureValue::Scalar {
                kind: crate::extract::ValueType::Boolean,
                text,
            }) if text == "false"
        )
        .then(|| {
            format!(
                "its `{}` field is `false`, so the harness does not load the plugin",
                crate::kind::ENABLEMENT_FIELD
            )
        }),
        Registration::DescriptionTrigger { field } => field_is_blank(member, field).then(|| {
            format!("its `{field}` description-trigger field is blank, so the harness has nothing to load")
 }),
        Registration::PathsMatch { field } => {
            // An absent/blank field is unconditional loading, not a dead edge:
 // only a *present* glob set that
            // matches nothing is provably dead.
            let globs = declared_globs(member, field);
            // A glob `globset` cannot compile is treated as matching, so the gate never
            // cries wolf on a `paths-match` pattern it failed to understand.
            let dead = !globs.is_empty()
                && !globs.iter().any(|glob| {
                    crate::kind::compile_glob(glob)
                        .is_none_or(|matcher| repo_files.iter().any(|file| matcher.is_match(file)))
                });
            dead.then(|| {
                format!("its `{field}` globs match no file in the repository, so the harness activates it never")
 })
 }
 }
}

/// Whether a member's registration field is **blank** — absent, or a scalar whose text is
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

/// The registration globs a member declares on `field`: a scalar names one glob, a list
/// names each of several, and an absent field or a map (which carries no glob) names
/// none. Read off [`Features`] — a declared field, never grepped. Declaring none is
/// *not* a dead edge: an absent/blank `paths` field falls back to unconditional loading,
/// so the caller only tests for the dead edge
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

/// The finding for a member whose inbound registration edge from the [`world`] node is
/// dead — naming the world, the member (kind + id), and the dead-edge reason, at the
/// assembly-declared `severity`.
fn unreachable(world: &Node, kind: &str, id: &str, reason: &str, severity: Severity) -> Diagnostic {
    Diagnostic::new(
        severity,
        GRAPH_REACHABLE_RULE,
        id,
        format!(
            "the registration edge from the {} node to {kind} `{id}` is dead — {reason}",
            world.0
        ),
    )
}

/// One member the directive classing ranges over: its `(kind, id)` identity, the
/// provenance `source_path` that is the join key between world paths and members,
/// and its extracted `at-import` target
/// occurrences in document order. The caller builds it off the units the features were
/// extracted from — the full path the decidable [`Features`] view drops — carrying
/// *every* member (a directive may point at a member that itself imports nothing) with
/// its `directives` (empty for a kind composing no `directives` primitive).
pub struct DirectiveMember {
    /// The member's kind name (`skill`, `memory`, a custom kind).
    pub kind: String,
    /// The member's id — the `Features::id`, named in a finding and an edge endpoint.
    pub id: String,
    /// The provenance source path the member was imported from — the classing join key.
    pub source_path: PathBuf,
    /// The member's extracted `at-import` occurrences: raw target strings in document
    /// order (`Features::directives`).
    pub directives: Vec<String>,
}

/// The outcome of classifying a corpus's directive occurrences:
/// the
/// member→member edges the member-class occurrences resolved to, and the
/// unbacked-pointer findings for the occurrences that resolved to neither a member nor
/// a repo file.
pub struct DirectiveClassing {
    /// The member-class occurrences as resolved edges — each an observed import from
    /// one member to another, of the same [`ResolvedEdge`] type the declared-edge
    /// enumeration yields, so it enters the one resolved-edge set the graph predicates
    /// read. Reachability closing over them is a later slice.
    pub edges: Vec<ResolvedEdge>,
    /// The unbacked-pointer findings — one per occurrence resolving to nothing, keyed
    /// to the importing member.
    pub findings: Vec<Diagnostic>,
}

/// Classify each member's extracted `at-import` directive occurrences against the
/// landscape: resolve every target
/// relative to the importing member's file directory (an absolute target as-is;
/// code.claude.com/docs/en/memory, retrieved 2026-07-16) and sort it into one of three
/// classes — a **member** (the resolved path is another member's provenance
/// `source_path`, yielding a member→member [`ResolvedEdge`]), a **backed repo file**
/// (the path is present in `repo_files`, a one-way boundary edge that neither errors
/// nor enters the member graph), or **nothing** (an *unbacked pointer* — the importing
/// member's finding, the silent-context-loss failure class made author-time).
///
/// `members` carries every member so the provenance index is complete — a target may
/// point at a member that imports nothing. `repo_files` is the repo file-set
/// [`reachable`] also reads. Members and their targets iterate in the caller's order,
/// so the edge and finding sets are stable. Member class beats repo-file class: a
/// member *is* a repo file, and the stronger classification (it enters the graph) wins.
#[must_use]
pub fn classify_directives(
    members: &[DirectiveMember],
    repo_files: &[String],
) -> DirectiveClassing {
    // The provenance index — normalized `source_path` → node — the join between a
    // resolved target path and the member it names.
    let index: BTreeMap<PathBuf, Node> = members
        .iter()
        .map(|member| {
            (
                normalize_path(&member.source_path),
                (member.kind.clone(), member.id.clone()),
            )
        })
        .collect();
    // The repo file-set, normalized the identical way so a resolved target joins it.
    let repo: BTreeSet<PathBuf> = repo_files
        .iter()
        .map(|file| normalize_path(Path::new(file)))
        .collect();

    let mut edges = Vec::new();
    let mut findings = Vec::new();
    for member in members {
        for target in &member.directives {
            let resolved = resolve_directive_target(&member.source_path, target);
            if let Some(to) = index.get(&resolved) {
                edges.push(ResolvedEdge {
                    from: (member.kind.clone(), member.id.clone()),
                    field: DIRECTIVE_FIELD.to_string(),
                    to: to.clone(),
                });
            } else if !repo.contains(&resolved) {
                // Neither a member nor a repo file: an unbacked pointer that loads
                // nothing. A backed repo file is a one-way boundary edge — no finding,
                // no member edge.
                findings.push(unbacked_pointer(&member.id, target));
            }
        }
    }
    DirectiveClassing { edges, findings }
}

/// Resolve a directive target against the importing member's file: an absolute target
/// as authored, a relative one joined onto the importing file's directory,
/// then lexically normalized so `.`/`..`
/// segments join the index cleanly.
fn resolve_directive_target(importing: &Path, target: &str) -> PathBuf {
    let target = Path::new(target);
    let joined = if target.is_absolute() {
        target.to_path_buf()
    } else {
        importing
            .parent()
            .map_or_else(|| target.to_path_buf(), |dir| dir.join(target))
    };
    normalize_path(&joined)
}

/// Lexically normalize a path — drop `.` and resolve `..` against a preceding normal
/// segment — **without touching disk**: a provenance path need not exist under the
/// check CWD, and both the index keys and a resolved target must normalize the identical
/// way to join. A leading `..` with nothing to pop is kept, so an out-of-tree target
/// stays distinct rather than silently rooting.
pub(crate) fn normalize_path(path: &Path) -> PathBuf {
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

/// The finding for an **unbacked pointer** — a directive occurrence resolving to
/// neither a member nor a repo file: the
/// importing member imports a path that loads nothing, the silent-context-loss failure
/// class caught at author-time. Mirrors [`dangling`]/[`unreachable`]: an error naming
/// the importing member and the dead target.
fn unbacked_pointer(importing: &str, target: &str) -> Diagnostic {
    Diagnostic::error(
        GRAPH_DIRECTIVE_UNBACKED_RULE,
        importing,
        format!(
            "`{importing}` imports `@{target}`, which resolves to no member and no repository file — an unbacked pointer that loads nothing"
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
        let sources = by_kind.get(edge.from.as_str()).copied().unwrap_or(&[]);
        for source in sources {
            for target in edge_targets(source, &edge.field) {
                let Some((kind, identity)) = target_identity(&target, &edge.to) else {
                    continue;
                };
                // A dangling reference loads nothing — no resolved edge.
                if resolves(by_kind, kind, identity) {
                    resolved.push(ResolvedEdge {
                        from: (edge.from.clone(), source.id.clone()),
                        field: edge.field.clone(),
                        to: (kind.to_string(), identity.to_string()),
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

/// One authored **mention**, ready to enter the resolved-edge graph — the citing
/// member's own address and the address its `n` names, both already resolved at
/// emit (`crate::main`'s conversion off the lock's `mention` declaration rows, the
/// mirror of [`compose::Edge`]'s own lift off the assembly fact rows).
pub struct MentionDeclaration {
    /// The citing member's own `kind:name` address.
    pub member: String,
    /// The address the mention names.
    pub target: String,
}

/// The reserved kind a bare requirement name resolves under — distinct from [`world`]
/// and every artifact kind, so a requirement-targeted mention binds a node the
/// degree/explain traversals range over and route resolution ([`route_mentions`])
/// resolves against the roster rather than the by-kind corpus. Exposed so the read family
/// renders a requirement-targeted mention as its bare name, never a `requirement:name`
/// address no author wrote (READ-EDGE-UNIFY).
pub const REQUIREMENT_KIND: &str = "requirement";

/// Parse an address a mention may name into its graph [`Node`]: `kind:name` parses
/// into that member's node; a bare name (no `:`) addresses a requirement under the
/// reserved [`REQUIREMENT_KIND`].
fn node_from_address(address: &str) -> Node {
    match address.split_once(':') {
        Some((kind, name)) => (kind.to_string(), name.to_string()),
        None => (REQUIREMENT_KIND.to_string(), address.to_string()),
    }
}

/// Lift the lock's `mention` rows into [`ResolvedEdge`]s by parsing both addresses
/// ([`node_from_address`]) — the mention-family mirror of [`resolved_edges`]. The parse
/// alone binds no verdict: `emit` refuses a mention naming no declared kind before a byte
/// is written, but a mention naming a declared kind with no composed member *defers* — its
/// row rides the lock, and [`route_mentions`] owns the dangling verdict at `check` against
/// the discovered corpus. Every mention lands here regardless, obligation-free by default
/// (`specs/model/contract.md`) until a `degree` clause opts in to counting it.
#[must_use]
pub fn resolved_mention_edges(mentions: &[MentionDeclaration]) -> Vec<ResolvedEdge> {
    mentions
        .iter()
        .map(|mention| ResolvedEdge {
            from: node_from_address(&mention.member),
            field: MENTION_FIELD.to_string(),
            to: node_from_address(&mention.target),
        })
        .collect()
}

/// Whether a lifted reference edge resolves against the **discovered corpus**. Only a
/// mention route-resolves at `check`: a member `kind:name` target resolves when the
/// corpus carries a member of that kind and name; a bare requirement name resolves when
/// the roster declares it. An import ([`IMPORT_FIELD`]) or any other lifted edge already
/// resolved at emit and never dangles here, so it always resolves.
fn edge_resolves(
    edge: &ResolvedEdge,
    by_kind: &BTreeMap<&str, &[Features]>,
    requirements: &BTreeMap<String, Requirement>,
) -> bool {
    if edge.field != MENTION_FIELD {
        return true;
    }
    let (kind, name) = &edge.to;
    if kind == REQUIREMENT_KIND {
        requirements.contains_key(name)
    } else {
        by_kind
            .get(kind.as_str())
            .is_some_and(|members| members.iter().any(|features| features.id == *name))
    }
}

/// Split the lifted mention/import edges into those that resolve against the discovered
/// corpus and those that dangle ([`edge_resolves`]). The read verbs narrate the two
/// halves differently — a resolved mention as an edge, a dangling one as the gate's route
/// finding — over the exact split [`route_mentions`] fires on, so read never disagrees
/// with the gate (READ-EDGE-UNIFY).
#[must_use]
pub fn partition_mentions(
    mentions: &[ResolvedEdge],
    by_kind: &BTreeMap<&str, &[Features]>,
    requirements: &BTreeMap<String, Requirement>,
) -> (Vec<ResolvedEdge>, Vec<ResolvedEdge>) {
    mentions
        .iter()
        .cloned()
        .partition(|edge| edge_resolves(edge, by_kind, requirements))
}

/// Check **route resolution** over the authored mention edges: a mention whose target —
/// a member `kind:name` or a bare requirement name — is absent from the discovered corpus
/// returns an error-severity [`Diagnostic`] naming the citing member and the dangling
/// target. `emit` defers such a mention (a declared kind with no composed member rides the
/// lock), so `check` owns the verdict here, at the same corpus `implemented-by` resolution
/// reads; a mention naming no declared kind refused earlier, at emit. Edges iterate in the
/// lock's order, so the finding set is stable.
#[must_use]
pub fn route_mentions(
    mentions: &[ResolvedEdge],
    by_kind: &BTreeMap<&str, &[Features]>,
    requirements: &BTreeMap<String, Requirement>,
) -> Vec<Diagnostic> {
    mentions
        .iter()
        .filter(|edge| !edge_resolves(edge, by_kind, requirements))
        .map(dangling_mention)
        .collect()
}

/// One layout prose-import edge the lock already resolved at emit — the importing
/// layout member's own `kind:name` address and the `kind:name` address its import
/// region resolved to (another member, path-resolved against raw disk at emit). The
/// [`ImportDeclaration`] mirror of [`MentionDeclaration`]: an import whose target is a
/// plain repository file, not a member, carries no address and reaches the graph as no
/// edge (its content dependency is still fingerprinted in the lock — `crate::drift`).
pub struct ImportDeclaration {
    /// The importing layout member's own `kind:name` address.
    pub member: String,
    /// The `kind:name` address the import resolved to.
    pub target: String,
}

/// Fold the lock's already-resolved layout-import rows into [`ResolvedEdge`]s — the
/// import-locus mirror of [`resolved_mention_edges`]: no dangling check runs here (a
/// dangling import never reaches the lock, `emit` refuses before a byte is written),
/// just the address parse [`node_from_address`] on both ends. A path-resolved edge
/// resolved once, at emit, so it lands in the one resolved-edge enumeration the gate and
/// every read verb range over exactly as a mention does.
#[must_use]
pub fn resolved_import_edges(imports: &[ImportDeclaration]) -> Vec<ResolvedEdge> {
    imports
        .iter()
        .map(|import| ResolvedEdge {
            from: node_from_address(&import.member),
            field: IMPORT_FIELD.to_string(),
            to: node_from_address(&import.target),
        })
        .collect()
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
/// every kind its target set declares is one `temper` models. The predicate [`check`]
/// gates on to skip an unsound declaration, kept in lockstep with the clauses
/// [`admissibility`] reports so the two never disagree.
fn is_admissible(edge: &Edge, by_kind: &BTreeMap<&str, &[Features]>) -> bool {
    !edge.field.is_empty()
        && edge
            .to
            .iter()
            .all(|kind| by_kind.contains_key(kind.as_str()))
}

/// The target artifact names an `edge`'s reference field carries on one source
/// artifact: a scalar field names one target, a list field names each of several,
/// and an absent field (or a map, which carries no name) names none. Read off
/// [`Features`] — a declared field, never grepped prose.
fn edge_targets(source: &Features, field: &str) -> Vec<String> {
    match source.field(field) {
        None | Some(FeatureValue::Map) => Vec::new(),
        Some(FeatureValue::List(items)) => items,
        Some(FeatureValue::Scalar { text, .. }) => vec![text],
    }
}

/// The `(kind, identity)` one authored edge leaf resolves to within the edge's declared
/// target set — the single spelling normalizer both [`check`] and [`resolved_edges`]
/// compare through, so an edge resolves on one path whatever grain declared it. `None`
/// is a leaf the declaration cannot address at all, which dangles under its authored
/// name.
///
/// A field edge is declared at any grain, and the grains spell their leaf differently: a
/// frontmatter field carries a bare identity, while an embedded member's edge field
/// carries the target's full `kind:name` address. Which of the declared kinds a leaf
/// names is read off **the written text alone**, never inferred from the member
/// population — a bare name stays unresolvable against a multi-kind declaration even
/// when exactly one kind happens to hold a member of that name, since the answer would
/// otherwise flip as members come and go.
///
/// - a **one-element** set resolves a bare identity within its one kind, and takes a
///   `kind:name` address whose kind is that element as the same identity. Any other
///   spelling is carried whole as the identity, so it dangles under its authored name
///   rather than being cross-attributed to a same-named member of the target kind.
/// - a **multi-element** set resolves only a `kind:name` whose kind is one of its
///   elements. A bare name, or an address naming an undeclared kind, resolves to
///   nothing.
fn target_identity<'a>(target: &'a str, to: &'a [String]) -> Option<(&'a str, &'a str)> {
    if let [only] = to {
        let identity = target
            .strip_prefix(only.as_str())
            .and_then(|rest| rest.strip_prefix(':'))
            .unwrap_or(target);
        return Some((only.as_str(), identity));
    }
    let (kind, identity) = target.split_once(':')?;
    to.iter()
        .find(|declared| declared.as_str() == kind)
        .map(|declared| (declared.as_str(), identity))
}

/// Whether `identity` names a real member of `kind` in the corpus — the one membership
/// test route resolution runs, over the same map [`check`] and [`resolved_edges`] read.
fn resolves(by_kind: &BTreeMap<&str, &[Features]>, kind: &str, identity: &str) -> bool {
    by_kind
        .get(kind)
        .copied()
        .unwrap_or(&[])
        .iter()
        .any(|features| features.id == identity)
}

/// An edge's declared target set, rendered for a diagnostic: one kind reads as its own
/// name, several as an `or`-joined list — the authored address names one of them.
fn render_target_kinds(to: &[String]) -> String {
    let rendered: Vec<String> = to.iter().map(|kind| format!("`{kind}`")).collect();
    match rendered.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} or {last}", rest.join(", ")),
    }
}

/// A stable identity for an edge in a diagnostic — `<from>.<field>` (e.g.
/// `rule.routes_to`).
fn edge_id(edge: &Edge) -> String {
    format!("{}.{}", edge.from, edge.field)
}

/// The finding for a route that resolves to no artifact — naming the source, the
/// reference field, the dangling target, and the declared target kinds.
fn dangling(edge: &Edge, source: &str, target: &str) -> Diagnostic {
    Diagnostic::error(
        GRAPH_ROUTE_RULE,
        source,
        format!(
            "`{source}` `{}` routes to `{target}`, which resolves to no {} artifact",
            edge.field,
            render_target_kinds(&edge.to)
        ),
    )
}

/// Render a mention target [`Node`] as the author wrote it: a member as its `kind:name`
/// address, a requirement as its bare name.
fn render_target(node: &Node) -> String {
    let (kind, name) = node;
    if kind == REQUIREMENT_KIND {
        name.clone()
    } else {
        format!("{kind}:{name}")
    }
}

/// The finding for a mention whose target resolves to no member or requirement in the
/// discovered corpus — naming the citing member and the dangling target
/// ([`route_mentions`]).
fn dangling_mention(edge: &ResolvedEdge) -> Diagnostic {
    let (from_kind, from_id) = &edge.from;
    let target = render_target(&edge.to);
    let resolves_against = if edge.to.0 == REQUIREMENT_KIND {
        "requirement"
    } else {
        "member"
    };
    Diagnostic::error(
        GRAPH_ROUTE_RULE,
        format!("{from_kind}:{from_id}"),
        format!(
            "`{from_kind}:{from_id}` mentions `{target}`, which resolves to no {resolves_against} in the discovered corpus",
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use serde_json::{Value as JsonValue, json};

    use crate::check::Severity;
    use crate::compose::Edge;
    use crate::contract::{Clause, Severity as ClauseSeverity};
    use crate::roster;

    /// A `Features` carrying a name (its `id`) and, optionally, a `routes_to`
    /// reference field — a scalar naming one target.
    fn node(name: &str, routes_to: Option<&str>) -> Features {
        let mut fields = BTreeMap::new();
        if let Some(target) = routes_to {
            fields.insert(
                "routes_to".to_string(),
                JsonValue::String(target.to_string()),
            );
        }
        Features {
            id: name.to_string(),
            fields,
            body_lines: 1,
            rendered_lines: Some(1),
            rendered_chars: Some(0),
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: Some(name.to_string()),
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            nested_members: Vec::new(),
            satisfies: Vec::new(),
            edge_placements: None,
        }
    }

    /// The `routes_to` edge every case shares: a rule points at a skill.
    fn routes_to_edge() -> Edge {
        Edge {
            field: "routes_to".to_string(),
            from: "rule".to_string(),
            to: vec!["skill".to_string()],
        }
    }

    /// A `routes_to` edge naming an unmodeled target kind — every case below that
    /// needs one names `agent`, a kind `by_kind` never carries.
    fn routes_to_agent_edge() -> Edge {
        Edge {
            field: "routes_to".to_string(),
            from: "rule".to_string(),
            to: vec!["agent".to_string()],
        }
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
        style
            .fields
            .insert("routes_to".to_string(), json!(["standards", "absent"]));
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
        let edge = routes_to_agent_edge();
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
            to: vec!["skill".to_string()],
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
            to: vec!["rule".to_string()],
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
            to: vec!["rule".to_string()],
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
        style
            .fields
            .insert("routes_to".to_string(), json!(["standards", "absent"]));
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
        style
            .fields
            .insert("routes_to".to_string(), json!(["standards", "absent"]));
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
        let edge = routes_to_agent_edge();
        let rules = [node("style", Some("style"))];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);
        assert!(acyclic(std::slice::from_ref(&edge), &by_kind).is_empty());
    }

    /// A bare `gate` requirement, optionally typed to `kind`, declaring a required
    /// `degree` clause (or none) — the roster the [`degree`] check reads. The
    /// satisfier nodes are whichever candidates' `satisfies` names `gate`; `kind:
    /// None` is the kind-blind case, ranging over every modeled kind's opt-ins.
    fn gate_requirement(
        kind: Option<&str>,
        degree: Option<Predicate>,
    ) -> BTreeMap<String, crate::compose::Requirement> {
        let clauses = degree
            .into_iter()
            .map(|predicate| Clause {
                label: crate::contract::clause_label(
                    Some(&crate::contract::requirement_owner("gate")),
                    predicate.key(),
                    None,
                ),
                severity: ClauseSeverity::Required,
                predicate,
                guidance: None,
                source: None,
            })
            .collect();
        BTreeMap::from([(
            "gate".to_string(),
            crate::compose::Requirement {
                name: "gate".to_string(),
                prose: None,
                kind: kind.map(str::to_string),
                required: false,
                clauses,
                verifier: None,
            },
        )])
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
        let requirements = gate_requirement(
            Some("skill"),
            Some(Predicate::Degree {
                incoming: Some(EdgeBound {
                    min: None,
                    max: Some(0),
                }),
                outgoing: None,
            }),
        );
        let edges = [routes_to_edge()];
        let rules = [node("style", None)];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(
            degree(
                &roster::selections(&requirements, &by_kind),
                &edges,
                &[],
                &by_kind
            )
            .is_empty()
        );
    }

    #[test]
    fn a_self_registering_bound_fires_when_the_node_is_pointed_at() {
        // The rule `style` routes to `standards`, so the skill has incoming degree 1 —
        // outside `incoming = { max = 0 }`. A self-registering artifact must not be
        // reached: an error naming the requirement, the artifact, and the direction.
        let requirements = gate_requirement(
            Some("skill"),
            Some(Predicate::Degree {
                incoming: Some(EdgeBound {
                    min: None,
                    max: Some(0),
                }),
                outgoing: None,
            }),
        );
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(
            &roster::selections(&requirements, &by_kind),
            &edges,
            &[],
            &by_kind,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, "requirement.gate.degree");
        assert_eq!(diags[0].artifact, "standards");
        assert!(diags[0].message.contains("gate"));
        assert!(diags[0].message.contains("incoming"));
    }

    #[test]
    fn a_routed_bound_passes_when_the_node_is_reachable() {
        // `incoming = { min = 1 }`: the skill `standards` must be reachable. The rule
        // `style` routes to it, so its incoming degree is 1 — inside the open-above
        // bound, clean.
        let requirements = gate_requirement(
            Some("skill"),
            Some(Predicate::Degree {
                incoming: Some(EdgeBound {
                    min: Some(1),
                    max: None,
                }),
                outgoing: None,
            }),
        );
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(
            degree(
                &roster::selections(&requirements, &by_kind),
                &edges,
                &[],
                &by_kind
            )
            .is_empty()
        );
    }

    #[test]
    fn a_routed_bound_fires_when_the_node_is_unreachable() {
        // No rule routes to `standards`, so its incoming degree is zero — outside
        // `incoming = { min = 1 }`. A routed artifact must be reachable: an error.
        let requirements = gate_requirement(
            Some("skill"),
            Some(Predicate::Degree {
                incoming: Some(EdgeBound {
                    min: Some(1),
                    max: None,
                }),
                outgoing: None,
            }),
        );
        let edges = [routes_to_edge()];
        let rules = [node("style", None)];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(
            &roster::selections(&requirements, &by_kind),
            &edges,
            &[],
            &by_kind,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "requirement.gate.degree");
        assert_eq!(diags[0].artifact, "standards");
        assert!(diags[0].message.contains("incoming"));
    }

    #[test]
    fn a_kind_blind_requirements_degree_bound_ranges_over_every_modeled_kind() {
        // No `kind` at all: `gate`'s satisfier is the *rule* `style` (a kind-blind
        // requirement is filled by opt-ins of any modeled kind), and its incoming
        // bound must still range over it rather than being skipped.
        let requirements = gate_requirement(
            None,
            Some(Predicate::Degree {
                incoming: Some(EdgeBound {
                    min: Some(1),
                    max: None,
                }),
                outgoing: None,
            }),
        );
        let edges = [routes_to_edge()];
        let rules = [satisfying(node("style", None), "gate")];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(
            &roster::selections(&requirements, &by_kind),
            &edges,
            &[],
            &by_kind,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "requirement.gate.degree");
        assert_eq!(diags[0].artifact, "style");
        assert!(diags[0].message.contains("incoming"));
    }

    #[test]
    fn an_outgoing_bound_reads_the_satisfier_node_out_degree() {
        // Degree bounds both directions: the rule `style` (a `gate` satisfier under an
        // `outgoing` bound) routes to one skill, so its out-degree is 1 — outside
        // `{ max = 0 }`.
        let requirements = gate_requirement(
            Some("rule"),
            Some(Predicate::Degree {
                incoming: None,
                outgoing: Some(EdgeBound {
                    min: None,
                    max: Some(0),
                }),
            }),
        );
        let edges = [routes_to_edge()];
        let rules = [satisfying(node("style", Some("standards")), "gate")];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(
            &roster::selections(&requirements, &by_kind),
            &edges,
            &[],
            &by_kind,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].artifact, "style");
        assert!(diags[0].message.contains("outgoing"));
    }

    #[test]
    fn a_roster_declaring_no_degree_bound_does_no_graph_work() {
        // `degree` is opt-in, per-requirement: a requirement with no bound is silent over a
        // graph that would violate one — `temper` never fabricates a gate the author
        // did not declare.
        let requirements = gate_requirement(Some("skill"), None);
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("standards"))];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        assert!(
            degree(
                &roster::selections(&requirements, &by_kind),
                &edges,
                &[],
                &by_kind
            )
            .is_empty()
        );
    }

    #[test]
    fn a_paths_match_glob_dies_only_when_no_repo_file_matches_it() {
        // `reachable` leans on `crate::kind::compile_glob` to decide a `paths-match`
        // glob dead — `**/` crossing segments and a flat `*` staying within one segment
        // are exercised directly on that shared surface (`kind::tests`), so this proves
        // only the wiring: `dead_registration` reports dead exactly when every declared
        // glob matches nothing in `repo_files`.
        let channel = Registration::PathsMatch {
            field: "paths".to_string(),
        };
        let mut fields = BTreeMap::new();
        fields.insert(
            "paths".to_string(),
            JsonValue::String("**/*.rs".to_string()),
        );
        let member = Features {
            id: "rust".to_string(),
            fields,
            body_lines: 1,
            rendered_lines: Some(1),
            rendered_chars: Some(0),
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: None,
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            nested_members: Vec::new(),
            satisfies: Vec::new(),
            edge_placements: None,
        };

        assert!(dead_registration(&channel, &member, &["src/a/foo.rs".to_string()]).is_none());
        assert!(dead_registration(&channel, &member, &["foo.md".to_string()]).is_some());
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
        let requirements = gate_requirement(
            Some("skill"),
            Some(Predicate::Degree {
                incoming: Some(EdgeBound {
                    min: Some(1),
                    max: None,
                }),
                outgoing: None,
            }),
        );
        let edges = [routes_to_edge()];
        let rules = [node("style", Some("absent"))];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let diags = degree(
            &roster::selections(&requirements, &by_kind),
            &edges,
            &[],
            &by_kind,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].artifact, "standards");
    }

    #[test]
    fn a_mention_edge_counts_toward_degree() {
        // No declared-reference edge touches `standards` at all — only a mention
        // (obligation-free by default) points at it. A `degree` clause is a contract
        // that opts in to counting it: `incoming = { min = 1 }` is satisfied by the
        // mention alone.
        let requirements = gate_requirement(
            Some("skill"),
            Some(Predicate::Degree {
                incoming: Some(EdgeBound {
                    min: Some(1),
                    max: None,
                }),
                outgoing: None,
            }),
        );
        let rules = [node("style", None)];
        let skills = [satisfying(node("standards", None), "gate")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let mention_edges = resolved_mention_edges(&[MentionDeclaration {
            member: "rule:style".to_string(),
            target: "skill:standards".to_string(),
        }]);
        assert!(
            degree(
                &roster::selections(&requirements, &by_kind),
                &[],
                &mention_edges,
                &by_kind
            )
            .is_empty()
        );
    }
}
