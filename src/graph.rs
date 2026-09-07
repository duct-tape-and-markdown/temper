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
//! [`reachable`]. The first four range over one resolved-edge enumeration ([`resolved_edges`]),
//! computed once per `gate()` invocation and shared with `crate::read`'s narration
//! so gate and read never disagree (READ-EDGE-UNIFY).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::builtin_kind::MAX_IMPORT_HOPS;
use crate::check::{Diagnostic, Severity};
use crate::compose::{Edge, Requirement};
use crate::contract::{EdgeBound, Predicate};
use crate::engine::{self, Selection};
use crate::extract::{FeatureValue, Features};
use crate::kind::Registration;
use crate::member_address::{
    embedded_source_host, parse_host_address, parse_leaf_address, parse_nested_address,
};
use crate::read::resolve_leaf;

thread_local! {
    /// Per-thread count of resolved-edge computations. Incremented each time the
    /// edge-resolution walk is computed via [`resolved_edges`], pinning that
    /// whole-input work hoists per `gate()` invocation (computed once and shared
    /// across check, acyclic, degree, and mention_reachable) rather than recomputing it
    /// per check.
    static RESOLVED_EDGES_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Per-thread count of resolved-edge computations. The walk is single-threaded on
/// its caller's thread, so this counts one run's computations in isolation.
#[must_use]
pub fn resolved_edges_count() -> usize {
    RESOLVED_EDGES_COUNT.with(|c| c.get())
}

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
pub(crate) const DIRECTIVE_FIELD: &str = "at-import";

/// The reference `field` a mention-produced [`ResolvedEdge`] records — an authored
/// `n`, not a frontmatter field. Lets a reader tell a mention edge from a declared
/// reference edge in the one resolved-edge set.
const MENTION_FIELD: &str = "mention";

/// The reference `field` a layout-prose-import [`ResolvedEdge`] records — a declared
/// include of a file's contents, not a frontmatter field. Lets a reader tell an import
/// edge from a declared reference edge in the one resolved-edge set.
const IMPORT_FIELD: &str = "import";

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
fn world() -> Node {
    ("world".to_string(), "world".to_string())
}

/// A **resolved edge** — a `(from, field, to)` triple over `(kind, id)` [`Node`]s,
/// both endpoints naming a real artifact. The element type of [`ResolvedEdgesResult::resolved`], the
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

/// The outcome of computing the resolved edges over declared references: the arcs
/// that form real member→member relationships and the dangling-route diagnostics
/// for references that resolve to no artifact. Computed once per `gate()` invocation
/// to avoid recomputation across [`check`], [`acyclic`], [`degree`], and
/// [`mention_reachable`].
#[derive(Clone)]
pub struct ResolvedEdgesResult {
    /// The **resolved** references — each an arc from one member to another over a
    /// declared field. Admissible edges whose targets resolve to real artifacts.
    pub resolved: Vec<ResolvedEdge>,
    /// The route-resolution findings — one per dangling reference, named to the
    /// importing member.
    pub dangling_diagnostics: Vec<Diagnostic>,
}

/// Check **route resolution** over the harness reference graph:
/// for each declared [`Edge`], read its reference field
/// off every source artifact and return an error-severity [`Diagnostic`] for any
/// route that resolves to no artifact of the target kind.
///
/// This is a thin wrapper over [`resolved_edges`] that extracts the dangling
/// diagnostics from the shared computation, avoiding recomputation of the
/// edge-resolution walk across multiple checks.
#[must_use]
pub fn check(edges: &[Edge], by_kind: &BTreeMap<&str, &[Features]>) -> Vec<Diagnostic> {
    resolved_edges(edges, by_kind).dangling_diagnostics
}

/// Validate the declared edges against **the definition** — admissibility:
/// each edge earns trust
/// *before* the graph judges the harness. Every finding is [`Diagnostic::error`] and
/// names the edge.
///
/// Three decidable clauses: **(a)** the reference `field` is
/// non-empty — an empty field names no reference syntax; **(b)** every kind the `to`
/// set declares is one `temper` models — an unmodeled element has no artifacts, so the
/// routes it would take can never resolve, making the fault the declaration's. The
/// finding names the offending *element*, not the whole set, and [`check`] skips the
/// edge. **(c)** no two edges share a `(from, field)` slot: `contract.md` ("edge") gives
/// a field ONE target set, so two rows spelling one slot are a malformed lock — reported
/// once per slot ([`coincident_slots`]), naming the count and each distinct `to` set,
/// and both arms skipped, since resolving either is a guess between two declarations.
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

    // (c) One `(from, field)` slot is declared by one edge. Two rows spelling it name no
    // single edge, so the fix is the merge, not a choice between them.
    for rows in coincident_slots(edges).into_values() {
        let Some(first) = rows.first() else { continue };
        // Each distinct target set the coincident rows declare, in declaration order —
        // identical rows collapse to one, which is exactly what the author needs to see.
        let mut targets: Vec<String> = Vec::new();
        for edge in &rows {
            let rendered = render_target_kinds(&edge.to);
            if !targets.contains(&rendered) {
                targets.push(rendered);
            }
        }
        let slot = edge_id(first);
        let message = format!(
            "`{slot}` is declared by {} `edge` facts, targeting {} — a field's target is one \
             set of kinds, so two rows spelling one slot name no single edge; merge them into \
             one `edge` fact whose `to` set lists every target kind",
            rows.len(),
            targets.join(" and "),
        );
        diagnostics.push(Diagnostic::error(GRAPH_ADMISSIBILITY_RULE, slot, message));
    }

    diagnostics
}

/// The `(from, field)` **slots** more than one [`Edge`] spells, each with its declaring
/// rows in declaration order — the coincidence [`admissibility`]'s clause (c) refuses and
/// [`resolved_edges`] skips. `contract.md` ("edge") gives a field one target set, so a
/// slot two rows spell names no single edge.
///
/// Keyed on the slot **alone**, whatever each row's `to` set says: two identical rows are
/// equally malformed, doubling every [`ResolvedEdge`] the slot yields and inflating
/// [`degree`]'s in/out counts with a phantom arc. One pass over an already-loaded slice —
/// never a walk over members, so the resolution cost is unmoved.
fn coincident_slots(edges: &[Edge]) -> BTreeMap<(&str, &str), Vec<&Edge>> {
    let mut slots: BTreeMap<(&str, &str), Vec<&Edge>> = BTreeMap::new();
    for edge in edges {
        slots
            .entry((edge.from.as_str(), edge.field.as_str()))
            .or_default()
            .push(edge);
    }
    slots.retain(|_, rows| rows.len() > 1);
    slots
}

/// Check **acyclicity** over the harness reference graph: build the artifact-level
/// graph from **resolved** arcs and return an error-severity [`Diagnostic`] naming
/// a cycle if one exists. A cycle is a circular import that loads nothing — a true
/// positive.
///
/// Takes a pre-computed slice of resolved arcs (from [`ResolvedEdgesResult::resolved`])
/// computed once per `gate()` invocation to avoid recomputation across multiple checks.
/// Inadmissible edges and dangling references don't enter this slice, so neither forges
/// nor masks a cycle (the dangling finding belongs to [`check`]). Nodes are keyed
/// `(kind, id)`. At most one finding — a cycle is fatal, and naming one closed chain
/// suffices; the chain is canonicalized (rotated to its least node) so the finding is
/// stable regardless of the traversal's entry node.
#[must_use]
pub fn acyclic(resolved: &[ResolvedEdge]) -> Vec<Diagnostic> {
    let adjacency = resolved_arcs(resolved);

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

/// Whether any selection carries a clause matching the given predicate test — an
/// opt-in guard both [`degree`] and [`mention_reachable`] consult to avoid unnecessary
/// graph work when no clause declares the predicate.
fn any_clause_of(selections: &[Selection], matches: impl Fn(&Predicate) -> bool) -> bool {
    selections.iter().any(|selection| {
        selection
            .clauses
            .iter()
            .any(|clause| matches(&clause.predicate))
    })
}

/// Check the **`degree`** predicate over every declared [`Selection`]: for each `degree`
/// clause bound to one, return a [`Diagnostic`] — at the clause's own declared severity
/// — per selected member whose in/out edge count over the resolved arcs falls outside
/// the bound.
///
/// `degree` is the one set predicate this module judges rather than
/// [`engine::judge`]: the clause is each-grain over the selection's members and
/// whole-grain over each member's own **by-incidence** selection — the edges at it,
/// filtered by direction — which is the graph, not a fact the members carry. Takes a
/// pre-computed slice of resolved arcs (from [`ResolvedEdgesResult::resolved`])
/// computed once per `gate()` invocation to avoid recomputation.
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
    resolved: &[ResolvedEdge],
    mention_edges: &[ResolvedEdge],
) -> Vec<Diagnostic> {
    if !any_clause_of(selections, |predicate| {
        matches!(predicate, Predicate::Degree { .. })
    }) {
        return Vec::new();
    }

    let mut adjacency = resolved_arcs(resolved);
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

/// The host member an edge's source belongs to, or `None` when the source is no embedded
/// member: read off the source's **own address** when it carries its host
/// ([`embedded_source_host`](crate::member_address::embedded_source_host)), else off the
/// caller's `(kind, key)` index — the spelling a declaration row uses when it names an
/// embedded member by key alone.
fn edge_host(from: &Node, embedded_hosts: &BTreeMap<Node, Node>) -> Option<Node> {
    if let Some((_, (host_kind, host_name))) = embedded_source_host(&from.1) {
        return Some((host_kind.to_string(), host_name.to_string()));
    }
    embedded_hosts.get(from).cloned()
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
/// folds: the declared field edges resolved to real targets (pre-computed and passed in)
/// plus the already-resolved mention and import edges. A reference to a gated target can
/// fire where that target cannot be invoked whatever locus declared it, so a rendering
/// claim carried on a field edge is judged rather than dropped for riding a family this
/// check once read alone.
///
/// An edge a **body-carried** member declares is judged under its *host*'s scope, not
/// the embedded member's own ([`edge_host`]): a member's reference set is the union of the
/// edges its fields *and* its embedded members declare, so a body-carried consult is the
/// host's citation to scope. `embedded_hosts` is the caller's `(embedded-kind, key)` →
/// `(host-kind, host-id)` index, read for a source spelled without its host.
#[must_use]
pub fn mention_reachable(
    selections: &[Selection],
    resolved: &[ResolvedEdge],
    mention_edges: &[ResolvedEdge],
    by_kind: &BTreeMap<&str, &[Features]>,
    embedded_hosts: &BTreeMap<Node, Node>,
) -> Vec<Diagnostic> {
    if !any_clause_of(selections, |predicate| {
        matches!(predicate, Predicate::MentionReachable { .. })
    }) {
        return Vec::new();
    }
    let mut all_edges = resolved.to_vec();
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
                    edge.from == source
                        || edge_host(&edge.from, embedded_hosts).as_ref() == Some(&source)
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
    member_named(by_kind.get(kind.as_str())?, name)
}

/// The first source glob whose resolved path set is not contained in the gate's resolved
/// path set, or `None` when every scope glob is a subset of the gate globs. Containment
/// is a structural-subset check over resolved path sets via `globset`: for each scope
/// glob, representative paths are tested against the gate glob set; if all representatives
/// match at least one gate glob, the scope glob is contained. True glob-language containment
/// is undecidable, so this check errs toward firing on a semantically contained narrower
/// glob rather than staying silent (same leniency `dead_registration` takes with an
/// uncompilable glob). A clause naming the predicate therefore ships at advisory severity.
fn uncontained(scope: &[String], gate: &[String]) -> Option<String> {
    for scope_glob in scope {
        if !is_scope_contained_in_gate(scope_glob, gate) {
            return Some(scope_glob.clone());
        }
    }
    None
}

/// Whether a scope glob's resolved path set is contained in the gate glob set's resolved
/// path sets. Generates representative paths from the scope glob and tests each against
/// the gate glob set; if all representatives match at least one gate glob, the scope is
/// contained in the gate.
fn is_scope_contained_in_gate(scope_glob: &str, gate_globs: &[String]) -> bool {
    // Compile the gate globs; an uncompilable gate glob is treated as matching (the same
    // leniency `dead_registration` takes).
    let compiled_gates: Vec<_> = gate_globs
        .iter()
        .filter_map(|g| crate::glob::compile_glob(g))
        .collect();
    if compiled_gates.is_empty() {
        // All gate globs failed to compile; treat as matching.
        return true;
    }

    // Generate representative paths from the scope glob. If the scope glob cannot compile,
    // treat it as contained (the leniency gate takes).
    let representatives = representative_paths_for_glob(scope_glob);
    if representatives.is_empty() {
        return true;
    }

    // Test each representative path: it must match at least one gate glob.
    representatives.iter().all(|path| {
        compiled_gates
            .iter()
            .any(|gate_matcher| gate_matcher.is_match(path))
    })
}

/// Generate representative paths that a glob pattern would match. These are used to
/// test whether a scope glob is a subset of a gate glob set. For each `**` segment,
/// we generate multiple test paths covering zero and multiple directory levels. Brace
/// alternations are expanded into one witness per alternative, and character classes
/// are concretized to a representative character.
///
/// # Implementation note
///
/// This function and its helpers (`process_glob_segment`, `expand_braces`,
/// `concretize_segment`, `concretize_char_class`, `is_in_char_class`) form a
/// hand-rolled brace-and-charset glob parser. Subset-containment witness synthesis
/// requires the parsed brace-alternative and character-class AST that `globset`'s
/// public API does not expose — the crate provides only the original glob string and
/// compiled regex string, not the AST structure needed to consolidate witnesses
/// against the gate set.
fn representative_paths_for_glob(glob: &str) -> Vec<String> {
    // Split the glob into segments to understand its structure.
    let segments: Vec<&str> = glob.split('/').collect();

    // Track path variants as we build them; start with one empty variant
    let mut path_variants: Vec<Vec<String>> = vec![vec![]];

    for (i, segment) in segments.iter().enumerate() {
        if *segment == "**" {
            // For `**`, generate multiple depth examples for each existing variant
            let mut new_variants = Vec::new();

            for variant in path_variants {
                // Collect the prefix (parts before **)
                let prefix = if variant.is_empty() {
                    String::new()
                } else {
                    variant.join("/") + "/"
                };

                // Collect the suffix (parts after **)
                let suffix_parts: Vec<&str> = segments[i + 1..].to_vec();
                let has_suffix = !suffix_parts.is_empty();

                if has_suffix {
                    // ** followed by more segments: generate examples with different depths
                    let suffix_paths = representative_paths_for_glob(&suffix_parts.join("/"));
                    for suffix in suffix_paths {
                        // Zero directories (** matches nothing)
                        new_variants.push(vec![format!("{}{}", prefix, suffix)]);

                        // One directory level
                        new_variants.push(vec![format!("{}dir/{}", prefix, suffix)]);

                        // Two directory levels
                        new_variants.push(vec![format!("{}dir1/dir2/{}", prefix, suffix)]);
                    }
                } else {
                    // ** at the end: generate paths of various depths
                    new_variants.push(vec![format!("{}file", prefix)]);
                    new_variants.push(vec![format!("{}dir/file", prefix)]);
                    new_variants.push(vec![format!("{}dir1/dir2/file", prefix)]);
                }
            }

            // Convert single-element variant vecs to their string values
            return new_variants.into_iter().map(|v| v.join("")).collect();
        } else {
            // Process the segment to handle braces, character classes, and wildcards
            let segment_variants = process_glob_segment(segment);

            // Combine existing path variants with new segment variants
            let mut new_variants = Vec::new();
            for existing_variant in &path_variants {
                for segment_variant in &segment_variants {
                    let mut new_variant = existing_variant.clone();
                    new_variant.push(segment_variant.clone());
                    new_variants.push(new_variant);
                }
            }
            path_variants = new_variants;
        }
    }

    // Convert path variants to strings by joining segments with '/'
    if path_variants.is_empty() {
        vec![glob.to_string()]
    } else {
        path_variants.into_iter().map(|v| v.join("/")).collect()
    }
}

/// Process a single glob segment to expand braces and concretize character classes.
/// Returns a list of processed variants; normally one, but multiple for brace groups.
fn process_glob_segment(segment: &str) -> Vec<String> {
    // Expand brace groups like {a,b,c} into separate alternatives
    if let Some(expanded) = expand_braces(segment) {
        return expanded
            .into_iter()
            .flat_map(|alt| process_glob_segment(&alt))
            .collect();
    }

    // No braces; handle wildcards and character classes
    let concrete = concretize_segment(segment);
    vec![concrete]
}

/// Expand a brace group {a,b,...} into its alternatives, or return None if no braces.
fn expand_braces(segment: &str) -> Option<Vec<String>> {
    let start = segment.find('{')?;
    let end = segment.find('}')?;
    if start >= end {
        return None;
    }

    let prefix = &segment[..start];
    let suffix = &segment[end + 1..];
    let content = &segment[start + 1..end];

    let alternatives: Vec<&str> = content.split(',').collect();
    if alternatives.is_empty() {
        return None;
    }

    let expanded: Vec<String> = alternatives
        .into_iter()
        .map(|alt| format!("{}{}{}", prefix, alt, suffix))
        .collect();

    Some(expanded)
}

/// Concretize a glob segment by replacing wildcards and character classes with
/// representative characters.
fn concretize_segment(segment: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = segment.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '*' => {
                result.push_str("file");
                i += 1;
            }
            '?' => {
                result.push('x');
                i += 1;
            }
            '[' => {
                // Find the closing bracket
                if let Some(close_idx) = chars[i..].iter().position(|&c| c == ']') {
                    let close_idx = i + close_idx;
                    let class_str: String = chars[i..=close_idx].iter().collect();

                    // Concretize the character class
                    let representative = concretize_char_class(&class_str);
                    result.push_str(&representative);
                    i = close_idx + 1;
                } else {
                    // Unclosed bracket; just pass it through
                    result.push(chars[i]);
                    i += 1;
                }
            }
            c => {
                result.push(c);
                i += 1;
            }
        }
    }

    result
}

/// Concretize a character class like `[abc]`, `[a-z]`, or `[!abc]` to a representative.
fn concretize_char_class(class: &str) -> String {
    if !class.starts_with('[') || !class.ends_with(']') {
        return class.to_string();
    }

    let content = &class[1..class.len() - 1];

    if content.is_empty() {
        return class.to_string();
    }

    // Check if it's a negated class [!...]
    if let Some(negated) = content.strip_prefix('!') {
        // Find a character NOT in the negated set
        for test_char in ['0', '1', 'a', 'b', 'x', 'y', 'z', '_', '-'] {
            if !negated.contains(test_char) && !is_in_char_class(test_char, negated) {
                return test_char.to_string();
            }
        }
        // Fallback: just pick '0'
        "0".to_string()
    } else {
        // Positive character class [abc] or [a-z]
        // Try common character classes first
        if content == "0-9" {
            "0".to_string()
        } else if content == "a-z" {
            "a".to_string()
        } else if content == "A-Z" {
            "A".to_string()
        } else if let Some(dash_idx) = content.find('-') {
            if dash_idx > 0 {
                content.chars().next().unwrap().to_string()
            } else {
                // Otherwise, pick the first character in the class
                content
                    .chars()
                    .next()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "a".to_string())
            }
        } else {
            // Otherwise, pick the first character in the class
            content
                .chars()
                .next()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "a".to_string())
        }
    }
}

/// Check if a character is in a character class specification (like "a-z" or "abc").
fn is_in_char_class(ch: char, class: &str) -> bool {
    let mut chars = class.chars().peekable();
    while let Some(c) = chars.next() {
        if chars.peek() == Some(&'-') {
            let start = c;
            chars.next(); // consume '-'
            if let Some(end) = chars.next()
                && ch >= start
                && ch <= end
            {
                return true;
            }
        } else if c == ch {
            return true;
        }
    }
    false
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

/// A degree direction — which side of a node's edges a degree bound constrains.
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
        Registration::Enablement { field } => matches!(
            member.field(field),
            Some(FeatureValue::Scalar {
                kind: crate::extract::ValueType::Boolean,
                text,
            }) if text == "false"
        )
        .then(|| {
            format!(
                "its `{field}` field is `false`, so the harness does not load the plugin"
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
                    crate::glob::compile_glob(glob)
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

/// The glob strings a value carries: each element of a list, or a lone scalar
/// read as a single glob. A map carries none; a type mismatch there is another
/// clause's concern, not this one's. Each glob is trimmed, and blank/whitespace-only
/// entries are dropped — a whitespace-padded glob would compile as a literal-space
/// pattern silently matching nothing, and is better judged trimmed.
pub(crate) fn extract_globs(value: &FeatureValue) -> Vec<String> {
    match value {
        FeatureValue::List(items) => items
            .iter()
            .map(|glob| glob.trim().to_string())
            .filter(|glob| !glob.is_empty())
            .collect(),
        FeatureValue::Scalar { text, .. } => {
            let glob = text.trim();
            if glob.is_empty() {
                Vec::new()
            } else {
                vec![glob.to_string()]
            }
        }
        FeatureValue::Map => Vec::new(),
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
        Some(value) => extract_globs(&value),
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
                crate::path::normalize_path(&member.source_path),
                (member.kind.clone(), member.id.clone()),
            )
        })
        .collect();
    // The repo file-set, normalized the identical way so a resolved target joins it.
    let repo: BTreeSet<PathBuf> = repo_files
        .iter()
        .map(|file| crate::path::normalize_path(Path::new(file)))
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
    crate::path::normalize_path(&joined)
}

/// The finding for an **unbacked pointer** — a directive occurrence resolving to
/// neither a member nor a repo file: the
/// importing member imports a path that loads nothing, the silent-context-loss failure
/// class caught at author-time. Mirrors [`dangling`]/[`unreachable()`]: an error naming
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

/// Enumerate every **resolved** reference edge and the **dangling** routes that
/// resolve to no artifact: the single arc-resolution pass computed once per `gate()`
/// invocation and shared across [`check`], [`acyclic`], [`degree`], and
/// [`mention_reachable`], avoiding recomputation. For each admissible edge, each
/// source of its `from` kind, and each named target, yields either a [`ResolvedEdge`]
/// (when the target resolves to a real artifact of its `to` kind) or a dangling
/// diagnostic (when it resolves to nothing). The resolved half feeds [`resolved_arcs`]
/// into adjacency for [`acyclic`]/[`degree`] and `crate::read` filters per node so
/// gate and read narrate the same edges (READ-EDGE-UNIFY). Sources and targets iterate
/// in name-sorted order for a stable enumeration; a target named twice yields two
/// edges, deduped into one arc by [`resolved_arcs`].
#[must_use]
pub fn resolved_edges(
    edges: &[Edge],
    by_kind: &BTreeMap<&str, &[Features]>,
) -> ResolvedEdgesResult {
    RESOLVED_EDGES_COUNT.with(|c| c.set(c.get() + 1));
    let mut resolved = Vec::new();
    let mut dangling_diagnostics = Vec::new();
    // The whole-set half of admissibility, computed once here because `is_admissible`'s
    // per-edge signature cannot see a coincidence.
    let coincident = coincident_slots(edges);
    for edge in edges {
        // Skipped exactly as clauses (a)/(b) are: an inadmissible edge, and either arm of
        // a coincident slot — resolving one of two declarations is a guess, and the guess
        // forges a dangling finding for every reference the other arm's `to` set misses.
        if !is_admissible(edge, by_kind)
            || coincident.contains_key(&(edge.from.as_str(), edge.field.as_str()))
        {
            continue;
        }
        let sources = by_kind.get(edge.from.as_str()).copied().unwrap_or(&[]);
        for source in sources {
            for target in edge_targets(source, &edge.field) {
                let membership = target_identity(&target, &edge.to).map(|(kind, identity)| {
                    (kind, identity, resolve_target(by_kind, kind, identity))
                });
                match membership {
                    Some((kind, identity, Membership::One(_))) => {
                        resolved.push(ResolvedEdge {
                            from: (edge.from.clone(), source.id.clone()),
                            field: edge.field.clone(),
                            to: (kind.to_string(), identity.to_string()),
                        });
                    }
                    Some((_, _, Membership::Ambiguous(hosts))) => {
                        dangling_diagnostics.push(ambiguous_route(
                            edge,
                            source.id.as_str(),
                            &target,
                            &hosts,
                        ));
                    }
                    Some((_, _, Membership::Missing)) | None => {
                        dangling_diagnostics.push(dangling(edge, source.id.as_str(), &target));
                    }
                }
            }
        }
    }
    ResolvedEdgesResult {
        resolved,
        dangling_diagnostics,
    }
}

/// Build the artifact-level directed graph over **resolved** arcs — the shared
/// foundation [`acyclic`] and [`degree`] range over — by folding pre-computed resolved
/// arcs into `(kind, id)`-keyed adjacency. Arcs dedupe in the [`BTreeSet`], so a target
/// named twice is one arc. Deriving it from the same [`resolved_edges`] the read family
/// consumes keeps the gate's checks and `temper why` in lockstep.
fn resolved_arcs(resolved: &[ResolvedEdge]) -> BTreeMap<Node, BTreeSet<Node>> {
    let mut adjacency: BTreeMap<Node, BTreeSet<Node>> = BTreeMap::new();
    for ResolvedEdge { from, to, .. } in resolved {
        adjacency
            .entry(from.clone())
            .or_default()
            .insert(to.clone());
    }
    adjacency
}

/// One authored **mention**, ready to enter the resolved-edge graph — the citing
/// member's own address and the address its `n` names, both already resolved at
/// emit (`crate::main`'s conversion off the lock's `mention` declaration rows, the
/// mirror of [`compose::Edge`](crate::compose::Edge)'s own lift off the assembly fact rows).
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
///
/// Reserved among *node* kinds, not among corpus kinds: a corpus may declare a nested
/// kind spelled `requirement` (this one's members are the roster's, which has no kind at
/// all), so the two are told apart by the node's id — `reserved_node`.
pub const REQUIREMENT_KIND: &str = "requirement";

/// The reserved kind an embedded-leaf address resolves under — distinct from [`world`],
/// [`REQUIREMENT_KIND`], and every artifact kind, so an embedded-leaf mention binds a node
/// route resolution ([`route_mentions`]) resolves against the embedded leaves. Used with
/// the full leaf address (e.g., `member/kind/key/child-path`) as the node's id. Reserved
/// among node kinds on the same terms as [`REQUIREMENT_KIND`], and told from a corpus
/// kind of the same name by [`reserved_node`].
const EMBEDDED_LEAF_KIND: &str = "embedded";

/// Parse an address a mention may name into its graph [`Node`], reading the **grammar**
/// rather than the characters: a four-segment leaf address
/// (`<member>/<kind>/<key>/<child-path>`, [`parse_leaf_address`]) is an embedded leaf
/// under the reserved [`EMBEDDED_LEAF_KIND`]; a three-segment nested-member address
/// (`<host-address>/<kind>/<key>`, [`parse_nested_address`]) is that member's own node —
/// kind read off the address's second segment, id the whole address, the identity
/// [`target_identity`] already binds a declared edge to, so the two reference families
/// cannot disagree about which node one address names; otherwise `kind:name` parses into
/// that member's node and a bare name (no `:`) addresses a requirement under the reserved
/// [`REQUIREMENT_KIND`].
fn node_from_address(address: &str) -> Node {
    if parse_leaf_address(address).is_some() {
        return (EMBEDDED_LEAF_KIND.to_string(), address.to_string());
    }
    if let Some(nested) = parse_nested_address(address) {
        return (nested.kind.to_string(), address.to_string());
    }
    match parse_host_address(address) {
        Some((kind, name)) => (kind.to_string(), name.to_string()),
        None => (REQUIREMENT_KIND.to_string(), address.to_string()),
    }
}

/// Whether a [`Node`] is one of the two **reserved** ones [`node_from_address`] mints for
/// an address that names no member — a leaf under [`EMBEDDED_LEAF_KIND`], a bare
/// requirement name under [`REQUIREMENT_KIND`]. Read off the grammar, never the kind
/// string alone: a corpus is free to declare a nested kind *named* `requirement` or
/// `embedded`, and such a member's node carries its whole address as its id, which no
/// reserved node ever does.
fn reserved_node(node: &Node) -> Option<&str> {
    if parse_nested_address(&node.1).is_some() {
        return None;
    }
    (node.0 == EMBEDDED_LEAF_KIND || node.0 == REQUIREMENT_KIND).then_some(node.0.as_str())
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

/// The route finding a lifted reference edge earns against the **discovered corpus**, or
/// `None` when it resolves. Only a mention route-resolves at `check`: a member
/// `kind:name` target resolves when the corpus carries a member of that kind and name; an
/// embedded-leaf target resolves when the leaf exists in the corpus's embedded leaves; a
/// bare requirement name resolves when the roster declares it. An import
/// ([`IMPORT_FIELD`]) or any other lifted edge already resolved at emit and never dangles
/// here, so it always resolves.
///
/// The verdict and its wording live together so [`edge_resolves`] and [`route_mentions`]
/// cannot come to disagree about *why* a mention failed to resolve: a bare key several
/// hosts carry is refused as ambiguous, naming them, where a name no member bears dangles.
fn mention_finding(
    edge: &ResolvedEdge,
    by_kind: &BTreeMap<&str, &[Features]>,
    requirements: &BTreeMap<String, Requirement>,
) -> Option<Diagnostic> {
    if edge.field != MENTION_FIELD {
        return None;
    }
    let (kind, name) = &edge.to;
    // The reserved node kinds are read through [`reserved_node`], not off the kind string:
    // a nested member's node carries its declared kind, which a corpus may spell
    // `requirement` — and its identity is its whole address, which resolves at member
    // grain below exactly as a `kind:name` target does.
    match reserved_node(&edge.to) {
        Some(EMBEDDED_LEAF_KIND) => {
            let resolved = parse_leaf_address(name)
                .is_some_and(|parsed| resolve_leaf(by_kind, &parsed).is_some());
            return (!resolved).then(|| dangling_mention(edge));
        }
        Some(_) => return (!requirements.contains_key(name)).then(|| dangling_mention(edge)),
        None => {}
    }
    match resolve_target(by_kind, kind, name) {
        Membership::One(_) => None,
        Membership::Ambiguous(hosts) => Some(ambiguous_mention(edge, &hosts)),
        Membership::Missing => Some(dangling_mention(edge)),
    }
}

/// Whether a lifted reference edge resolves against the discovered corpus — the boolean
/// face of [`mention_finding`], which the read family's resolved/dangling split reads.
fn edge_resolves(
    edge: &ResolvedEdge,
    by_kind: &BTreeMap<&str, &[Features]>,
    requirements: &BTreeMap<String, Requirement>,
) -> bool {
    mention_finding(edge, by_kind, requirements).is_none()
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
/// a member `kind:name`, an embedded leaf `<member>/<kind>/<key>/<child-path>`, or a bare
/// requirement name — is absent from the discovered corpus returns an error-severity
/// [`Diagnostic`] naming the citing member and the dangling target. `emit` defers such a
/// mention (a declared kind with no composed member rides the lock), so `check` owns the
/// verdict here, at the same corpus `implemented-by` resolution reads; a mention naming no
/// declared kind refused earlier, at emit. Edges iterate in the lock's order, so the
/// finding set is stable.
#[must_use]
pub fn route_mentions(
    mentions: &[ResolvedEdge],
    by_kind: &BTreeMap<&str, &[Features]>,
    requirements: &BTreeMap<String, Requirement>,
) -> Vec<Diagnostic> {
    mentions
        .iter()
        .filter_map(|edge| mention_finding(edge, by_kind, requirements))
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
///
/// Per-edge only: admissibility's clause (c) is a fact about the edge *set*, invisible
/// from this signature, so [`resolved_edges`] carries that skip alongside this one off
/// [`coincident_slots`].
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
/// - a **nested member's own address** — `<host-address>/<kind>/<key>` — names its kind
///   in its second segment, so a multi-element set takes it exactly as it takes a
///   `kind:name`: by the kind the address spells, with the whole address carried on as the
///   identity [`resolve_target`] matches ([`parse_nested_address`] rules on the grammar, the
///   `/<leaf>` tail included).
fn target_identity<'a>(target: &'a str, to: &'a [String]) -> Option<(&'a str, &'a str)> {
    if let [only] = to {
        let identity = target
            .strip_prefix(only.as_str())
            .and_then(|rest| rest.strip_prefix(':'))
            .unwrap_or(target);
        return Some((only.as_str(), identity));
    }

    if let Some(nested) = parse_nested_address(target)
        && to.iter().any(|declared| declared.as_str() == nested.kind)
    {
        return Some((nested.kind, target));
    }

    // Fall back to bare address format: kind:name
    let (kind, identity) = parse_host_address(target)?;
    to.iter()
        .find(|declared| declared.as_str() == kind)
        .map(|declared| (declared.as_str(), identity))
}

/// The verdict an authored name gets against the corpus's members of `kind` — the one
/// membership test route resolution runs, over the same map [`check`] and
/// [`resolved_edges`] read. A kind the corpus composes nothing of answers
/// [`Membership::Missing`], exactly as a kind whose members none bear the name does.
fn resolve_target<'f>(
    by_kind: &BTreeMap<&str, &'f [Features]>,
    kind: &str,
    identity: &str,
) -> Membership<'f> {
    by_kind.get(kind).map_or(Membership::Missing, |members| {
        member_lookup(members, identity)
    })
}

/// What an authored name resolves to among one kind's members — the three-way verdict
/// [`member_lookup`] returns, so a name no member can own is refused rather than
/// collapsed onto whichever member the scan reached first.
enum Membership<'f> {
    /// Exactly one member answers the name.
    One(&'f Features),
    /// The name is a **bare key** these hosts each carry a member of the kind under —
    /// listed by host address, sorted and deduplicated. It names no one member.
    Ambiguous(Vec<&'f str>),
    /// No member of the kind answers the name.
    Missing,
}

/// The member of `members` an authored name addresses — the one membership lookup
/// [`resolve_target`], [`mention_finding`] and [`member_at`] all read, so route resolution
/// and the target's own features can never disagree about which member a name meant.
///
/// A member's identity **is** its address, so a name resolves by equality — an embedded
/// member's `<host-address>/<kind>/<key>` included, whose host segment is the whole of
/// what tells two same-keyed members under different hosts apart.
///
/// A **bare key** also names an embedded member: the short form the corpus writes, which
/// resolves when exactly one host carries it. Carried by *several* hosts it names nothing
/// — `representation.md` ("member") makes resolution total, an address naming exactly one
/// thing or the verb refusing — so the carriers come back as [`Membership::Ambiguous`]
/// for the caller to refuse by, naming every one of them.
fn member_lookup<'f>(members: &'f [Features], identity: &str) -> Membership<'f> {
    if let Some(features) = members.iter().find(|features| features.id == identity) {
        return Membership::One(features);
    }
    let carriers: Vec<(&'f str, &'f Features)> = members
        .iter()
        .filter_map(|features| {
            let nested = parse_nested_address(&features.id)?;
            (nested.key == identity).then_some((nested.host, features))
        })
        .collect();
    let hosts: BTreeSet<&'f str> = carriers.iter().map(|(host, _)| *host).collect();
    match (carriers.first(), hosts.len()) {
        (None, _) | (Some(_), 0) => Membership::Missing,
        // One host carrying the key twice is a malformed lock refused at admissibility
        // (`crate::admissibility`'s `nested_member_coincidence`), not an ambiguity to
        // re-decide here.
        (Some((_, only)), 1) => Membership::One(only),
        (Some(_), _) => Membership::Ambiguous(hosts.into_iter().collect()),
    }
}

/// The member of `members` an authored name addresses, or `None` when the name resolves
/// to no single member — the [`Option`] face of [`member_lookup`] for the readers that
/// carry no diagnostic to raise. An ambiguous bare key answers `None`: it names no one
/// member, and the refusal is raised where the route findings are built.
fn member_named<'f>(members: &'f [Features], identity: &str) -> Option<&'f Features> {
    match member_lookup(members, identity) {
        Membership::One(features) => Some(features),
        Membership::Ambiguous(_) | Membership::Missing => None,
    }
}

/// Each embedded member's `(kind, key)` node keyed to its **host**'s node — the index
/// `mention_reachable` needs to judge a body-carried citation under its host's scope,
/// since an embedded-carried edge keys its source to the embedded member, never the host
/// (the source-side twin of the target-side `target_identity` seam).
///
/// Built off each member's own identity — an embedded member's `Features::id` **is** its
/// `<host-address>/<kind>/<key>` address — so this index and the two membership lookups
/// beside it read one grammar through one parser.
///
/// A `(kind, key)` **two different hosts carry** maps to no host: the short spelling is
/// ambiguous between them, and an ambiguous address names nothing rather than whichever
/// carrier happened to be indexed last. The edge it would have scoped stays keyed to the
/// embedded member alone, exactly as a member whose address names no host does; the
/// reference spelling that ambiguity is *authored* in refuses loud at resolution
/// ([`resolved_edges`], [`route_mentions`]).
#[must_use]
pub fn embedded_hosts_by_key(by_kind: &BTreeMap<&str, &[Features]>) -> BTreeMap<Node, Node> {
    let mut hosts: BTreeMap<Node, Option<Node>> = BTreeMap::new();
    for features in by_kind.values().flat_map(|members| members.iter()) {
        let Some(((source_kind, source_key), (host_kind, host_name))) =
            embedded_source_host(&features.id)
        else {
            continue;
        };
        let source: Node = (source_kind.to_string(), source_key.to_string());
        let host: Node = (host_kind.to_string(), host_name.to_string());
        hosts
            .entry(source)
            .and_modify(|carrier| {
                if carrier.as_ref() != Some(&host) {
                    *carrier = None;
                }
            })
            .or_insert(Some(host));
    }
    hosts
        .into_iter()
        .filter_map(|(source, host)| Some((source, host?)))
        .collect()
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

/// The refusal a **bare key more than one host carries** reads as — the tail both route
/// findings share, so one ambiguity has one wording wherever it is spelled.
///
/// It is the SDK's own, which ships this bar at the other end of the seam
/// (`sdk/src/emit.ts`, `resolvedTargetFacts`): name every carrier host, and point at the
/// whole `<host-address>/<kind>/<key>` spelling that tells them apart. Two hosts sharing a
/// `(kind, key)` are legal — their addresses differ in the host segment — so the refusal
/// is the *reference*'s, never the declaration's.
fn ambiguous_bare_key(hosts: &[&str]) -> String {
    let carriers: Vec<String> = hosts.iter().map(|host| format!("`{host}`")).collect();
    format!(
        "a bare key {} hosts carry ({}) — a nested member's address composes through its \
         host, so spell the whole `<host-address>/<kind>/<key>` \
         (specs/model/representation.md, \"member\")",
        hosts.len(),
        carriers.join(", "),
    )
}

/// The finding for a declared reference whose target is a bare key several hosts carry —
/// the [`dangling`] twin for a route that resolves to *too many* members rather than none
/// ([`ambiguous_bare_key`]).
fn ambiguous_route(edge: &Edge, source: &str, target: &str, hosts: &[&str]) -> Diagnostic {
    Diagnostic::error(
        GRAPH_ROUTE_RULE,
        source,
        format!(
            "`{source}` `{}` routes to `{target}`, {}",
            edge.field,
            ambiguous_bare_key(hosts)
        ),
    )
}

/// Render a mention target [`Node`] as the author wrote it: a top-level member as its
/// `kind:name` address, a requirement as its bare name, and a nested member — or an
/// embedded leaf — as the whole address that is already its id.
fn render_target(node: &Node) -> String {
    let (kind, name) = node;
    if reserved_node(node).is_some() || parse_nested_address(name).is_some() {
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
    let resolves_against = if reserved_node(&edge.to) == Some(REQUIREMENT_KIND) {
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

/// The finding for a mention whose target is a bare key several hosts carry — the
/// [`dangling_mention`] twin, in the same wording the declared-reference family refuses
/// with ([`ambiguous_bare_key`]).
fn ambiguous_mention(edge: &ResolvedEdge, hosts: &[&str]) -> Diagnostic {
    let (from_kind, from_id) = &edge.from;
    let target = render_target(&edge.to);
    Diagnostic::error(
        GRAPH_ROUTE_RULE,
        format!("{from_kind}:{from_id}"),
        format!(
            "`{from_kind}:{from_id}` mentions `{target}`, {}",
            ambiguous_bare_key(hosts)
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
            fields,
            body_lines: 1,
            rendered_lines: Some(1),
            source_dir: Some(name.to_string()),
            ..crate::test_support::features(name)
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

    /// A second `routes_to` edge off `rule`, spelling the SAME slot as
    /// [`routes_to_edge`] with a different target set — the coincidence clause (c)
    /// refuses.
    fn routes_to_command_edge() -> Edge {
        Edge {
            field: "routes_to".to_string(),
            from: "rule".to_string(),
            to: vec!["command".to_string()],
        }
    }

    /// The corpus every coincidence case reads: one routing rule, the skill its
    /// `routes_to` really names, and an empty `command` kind — so both declared target
    /// kinds are modeled and clause (b) stays silent.
    fn coincidence_corpus() -> ([Features; 1], [Features; 1], [Features; 0]) {
        (
            [node("style", Some("standards"))],
            [node("standards", None)],
            [],
        )
    }

    #[test]
    fn one_edge_over_the_coincidence_corpus_resolves_and_is_silent() {
        // Non-vacuity for the two cases below: with ONE row for the slot, the same
        // fixture resolves `style` → `standards` and draws no finding at all. The
        // refusal below is a real narrowing, never an empty read.
        let (rules, skills, commands) = coincidence_corpus();
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([
            ("rule", &rules[..]),
            ("skill", &skills[..]),
            ("command", &commands[..]),
        ]);
        let edges = [routes_to_edge()];

        assert!(admissibility(&edges, &by_kind).is_empty());
        assert!(check(&edges, &by_kind).is_empty());
        assert_eq!(
            resolved_edges(&edges, &by_kind).resolved.len(),
            1,
            "the one declared row resolves its reference to the real skill"
        );
    }

    #[test]
    fn two_edges_sharing_a_slot_are_inadmissible_once_and_both_skipped() {
        // `rule.routes_to` declared twice with different target sets: one slot, two
        // declarations. Admissibility reports the slot ONCE — naming the count, both
        // target sets, and the merge that fixes it — and neither arm reaches
        // `resolved_edges`, so the resolving arm forges no arc and the `command` arm
        // forges no dangling finding.
        let (rules, skills, commands) = coincidence_corpus();
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([
            ("rule", &rules[..]),
            ("skill", &skills[..]),
            ("command", &commands[..]),
        ]);
        let edges = [routes_to_edge(), routes_to_command_edge()];

        let admit = admissibility(&edges, &by_kind);
        assert_eq!(
            admit.len(),
            1,
            "one finding per coincident slot, not per row"
        );
        assert_eq!(admit[0].severity, Severity::Error);
        assert_eq!(admit[0].rule, GRAPH_ADMISSIBILITY_RULE);
        assert_eq!(admit[0].artifact, "rule.routes_to");
        assert!(admit[0].message.contains("`rule.routes_to`"));
        assert!(admit[0].message.contains("2 `edge` facts"));
        assert!(admit[0].message.contains("`skill` and `command`"));
        assert!(
            admit[0].message.contains("merge"),
            "the message names the edit, not just the state: {}",
            admit[0].message
        );

        let resolution = resolved_edges(&edges, &by_kind);
        assert!(
            resolution.resolved.is_empty(),
            "neither arm resolves — the refusal replaces the arcs, it does not join them"
        );
        assert!(
            resolution.dangling_diagnostics.is_empty(),
            "no route finding is forged from either `to` set"
        );
        assert!(check(&edges, &by_kind).is_empty());
    }

    #[test]
    fn two_identical_edge_rows_are_the_same_coincidence() {
        // The slot is keyed on `(from, field)` ALONE: two byte-identical rows are equally
        // malformed, since resolving both doubles every arc and inflates `degree`'s
        // counts with a phantom. One finding, naming the single target set once.
        let (rules, skills, commands) = coincidence_corpus();
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([
            ("rule", &rules[..]),
            ("skill", &skills[..]),
            ("command", &commands[..]),
        ]);
        let edges = [routes_to_edge(), routes_to_edge()];

        let admit = admissibility(&edges, &by_kind);
        assert_eq!(admit.len(), 1);
        assert_eq!(admit[0].artifact, "rule.routes_to");
        assert!(admit[0].message.contains("2 `edge` facts"));
        assert!(
            admit[0].message.contains("targeting `skill` —"),
            "one distinct target set reads once, not twice: {}",
            admit[0].message
        );
        assert!(resolved_edges(&edges, &by_kind).resolved.is_empty());
        assert!(check(&edges, &by_kind).is_empty());
    }

    #[test]
    fn two_edges_over_different_fields_of_one_kind_are_not_coincident() {
        // The slot is `(from, field)`, not `from`: a kind declaring two DIFFERENT
        // reference fields is the ordinary case, and both resolve.
        let mut style = node("style", Some("standards"));
        style.fields.insert(
            "cites".to_string(),
            JsonValue::String("standards".to_string()),
        );
        let rules = [style];
        let skills = [node("standards", None)];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let edges = [
            routes_to_edge(),
            Edge {
                field: "cites".to_string(),
                from: "rule".to_string(),
                to: vec!["skill".to_string()],
            },
        ];

        assert!(admissibility(&edges, &by_kind).is_empty());
        assert_eq!(resolved_edges(&edges, &by_kind).resolved.len(), 2);
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        assert!(acyclic(&resolved).is_empty());
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        let diags = acyclic(&resolved);
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        let diags = acyclic(&resolved);
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        assert!(acyclic(&resolved).is_empty());
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        let diags = acyclic(&resolved);
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
        let edges = [routes_to_agent_edge()];
        let rules = [node("style", Some("style"))];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        assert!(acyclic(&resolved).is_empty());
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        assert!(degree(&roster::selections(&requirements, &by_kind), &resolved, &[]).is_empty());
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        let diags = degree(&roster::selections(&requirements, &by_kind), &resolved, &[]);
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        assert!(degree(&roster::selections(&requirements, &by_kind), &resolved, &[]).is_empty());
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        let diags = degree(&roster::selections(&requirements, &by_kind), &resolved, &[]);
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        let diags = degree(&roster::selections(&requirements, &by_kind), &resolved, &[]);
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        let diags = degree(&roster::selections(&requirements, &by_kind), &resolved, &[]);
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        assert!(degree(&roster::selections(&requirements, &by_kind), &resolved, &[]).is_empty());
    }

    #[test]
    fn a_paths_match_glob_dies_only_when_no_repo_file_matches_it() {
        // `reachable` leans on `crate::glob::compile_glob` to decide a `paths-match`
        // glob dead — `**/` crossing segments and a flat `*` staying within one segment
        // are exercised directly on that shared surface (`glob::tests`), so this proves
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
            fields,
            body_lines: 1,
            rendered_lines: Some(1),
            ..crate::test_support::features("rust")
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
        let resolved = resolved_edges(&edges, &by_kind).resolved;
        let diags = degree(&roster::selections(&requirements, &by_kind), &resolved, &[]);
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
                &mention_edges
            )
            .is_empty()
        );
    }
}
