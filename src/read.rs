//! The read family — one CLI verb, [`explain`], over four traversals of the
//! requirement↔`satisfies` edge and the graph `check` already carries.
//!
//! [`explain`] resolves its one positional target across three namespaces — member,
//! requirement, leaf-grain address (`(explain-target-disambiguation)`, ruled
//! 2026-07-04) — and dispatches to whichever of the four traversals below answer that
//! species: [`why`] walks the edge **forward** (this member → the requirements it
//! fills, with their authored rationale → the default contract its kind binds → its resolved
//! edges in and out); [`requirements`] walks it in **reverse** (the roster → each
//! requirement's satisfier set + coverage state, and with a name the blast radius a
//! removal would strand); [`impact`] narrates the **blast radius of a removal** — what
//! strands if a member is removed or renamed: the requirements it is the sole satisfier
//! of (left unfilled), the `@import` directive edges that point at it (left unbacked),
//! and the members whose reachability was carried only through it (gone dead) — or, at
//! leaf grain, a leaf's citations reported separately from its (nonexistent) fallout;
//! [`context`] emits the **declared neighborhood** — a member's nested members or a
//! leaf's siblings, the citers, and the requirements satisfied. All are *projections* over the
//! data `check` already computes — the opt-in `satisfies` bindings [`crate::coverage`]
//! gates, and, for the edge walk, the **gate's own resolved edge set**
//! ([`crate::graph::resolved_edges`], relationships over extracted features), never a
//! private re-derivation (READ-EDGE-UNIFY: one source of truth, so `why`'s edge
//! narration cannot disagree with `graph::check`).
//! None adds engine semantics and none ever gates: `explain` returns narration, and
//! `main` prints it and exits zero on every input, ambiguous or unknown targets included
//! (the read family is not the gate; a reporting verb whose exit code CI trusts is
//! exactly what the Decision rejects).
//!
//! The output is a **teaching surface**, not a table dump: full sentences over the author's own artifacts, in the
//! corpus's vocabulary. The narration is derived, never persisted.
//!
//! ## Scope: every opt-in kind, built-in and custom
//!
//! This tier reads every opt-in kind's members — built-in (skill ⊕ rule) and custom
//! alike — the caller threads in as [`CustomMember`]s (READ-CUSTOM-SATISFIERS):
//! temper's own `spec`s, or any consumer's
//! custom kind whose member fills a requirement. The decidable
//! [`crate::extract::Features`] drops the `satisfies` rationale, so a custom member
//! arrives carrying its rationale-preserving [`crate::document::Satisfies`] clauses
//! ([`crate::kind::Unit::satisfies_clauses`]) instead — the *why* the read family
//! narrates whole. So a custom member filling a requirement is no longer silently
//! absent from either verb; the roster the read family narrates agrees with the gate.
//! Edge narration already ranges over every kind (it reads the gate's resolved edge
//! set, READ-EDGE-UNIFY), so only the `satisfies` walk widens here.

use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::{Path, PathBuf};

use crate::builtin_kind;
use crate::compose::{self, Edge, Requirement};
use crate::contract::Contract;
use crate::document::Satisfies;
use crate::drift;
use crate::extract::{self, Features, MemberAddress};
use crate::graph::{self, DIRECTIVE_FIELD, ResolvedEdge};
use crate::kind::{self, CustomKind, Registration};
use crate::tap;
use crate::tap::TapRecord;
use crate::telemetry;

/// A member as the read family sees it: its kind, its id, and the requirements it opts
/// into filling (each with its authored rationale) — the caller-threaded
/// [`CustomMember`] listing's `satisfies`, which the decidable
/// [`crate::extract::Features`] view drops the rationale from but the read family needs
/// whole. Edges are **not** carried here: `why` narrates the gate's resolved edge set
/// ([`crate::graph::resolved_edges`]) keyed on the member's `(kind, id)` node, never
/// re-derived here (READ-EDGE-UNIFY).
#[derive(Clone)]
struct Member {
    /// The artifact kind (`skill`, `rule`, or a custom kind's name) — part of the
    /// identity, since an id is unique only within a kind. Owned rather than
    /// `&'static`, because a custom kind's name is authored, not a built-in literal.
    kind: String,
    /// The member id (a skill's/rule's name, a custom member's id), the node the
    /// traversals key on.
    id: String,
    /// The requirements this member opts into filling, with their authored rationale.
    satisfies: Vec<Satisfies>,
}

/// A custom-kind member as the read family needs it (READ-CUSTOM-SATISFIERS): its
/// kind name, its id, and its rationale-carrying `satisfies` clauses. The caller
/// threads every opt-in kind's members in this way (loaded off
/// [`crate::kind::Unit::satisfies_clauses`]) — built-in and custom alike. Kept whole
/// with rationale, which the decidable [`Features`] view drops.
pub struct CustomMember {
    /// The custom kind's registered name (`spec`, `adr`, …) — the edge node's kind
    /// and what the narration prints.
    pub kind: String,
    /// The member id (its surface directory name).
    pub id: String,
    /// The rationale-preserving `satisfies` clauses this member authors.
    pub satisfies: Vec<Satisfies>,
}

/// A declared **citation** — a one-way edge from a member (the citer) to a nested
/// member's [`MemberAddress`] it names. **Obligation-free**: the
/// obligation graph ignores it, coverage never counts it, and `impact` reports it as a
/// *citation, never fallout* — deleting or rewording the cited leaf is never blocked,
/// the citer is told. **Resolution-checked** against the lock's serialized nested-member
/// leaves: `impact` reports a citation only for a leaf that resolves, exactly the
/// referential guarantee a mention carries.
///
/// The floor carries no producer yet — floor leaves carry no mentions,
/// so today's
/// caller threads an empty set and the leaf-grain report names zero citers; the mechanism
/// is proven in unit tests here.
pub struct Citation {
    /// The kind of the citer — part of its node identity, and what the narration prints.
    pub from_kind: String,
    /// The member id that declares the citation (the citer).
    pub from: String,
    /// The leaf address the citation targets — resolved against the serialized
    /// nested-member leaves before it is reported.
    pub target: MemberAddress,
}

/// Project every caller-threaded opt-in member into the read family's [`Member`]
/// view, name-sorted by its load, so every traversal below is deterministic without
/// a re-sort (READ-CUSTOM-SATISFIERS).
fn members(custom: &[CustomMember]) -> Vec<Member> {
    custom
        .iter()
        .map(|member| Member {
            kind: member.kind.clone(),
            id: member.id.clone(),
            satisfies: member.satisfies.clone(),
        })
        .collect()
}

/// Build an id-keyed index of members from the by_kind corpus — maps each member id
/// to the list of (kind, features) pairs bearing that id. Enables O(1) lookups by
/// member id instead of repeated linear scans of the full corpus.
fn build_member_index<'a>(
    by_kind: &BTreeMap<&'a str, &'a [Features]>,
) -> BTreeMap<&'a str, Vec<(&'a str, &'a Features)>> {
    let mut index = BTreeMap::new();
    for (&kind, features_slice) in by_kind {
        for features in *features_slice {
            index
                .entry(features.id.as_str())
                .or_insert_with(Vec::new)
                .push((kind, features));
        }
    }
    index
}

/// The target species `explain <target>` resolves a positional string into
/// (`(explain-target-disambiguation)`, ruled 2026-07-04): an explicit `member:`/
/// `requirement:`/`address:` qualifier always wins outright (an explicit spelling is
/// never re-checked for ambiguity); absent one, a `/`-bearing target is always a leaf
/// address (a member or requirement name never carries a slash), and a bare name is
/// checked against both the member corpus and the requirement roster.
enum Species<'a> {
    /// A member id — dispatches to [`why`] (what holds it in place) and [`impact`] and
    /// [`context`] at member grain (its blast radius and its neighborhood).
    Member(&'a str),
    /// A requirement name — dispatches to [`requirements`] alone, whose reverse walk
    /// already carries coverage and blast radius.
    Requirement(&'a str),
    /// A leaf address (`<member>/<kind>/<key>/<child-path>`) — dispatches to
    /// [`impact`] and [`context`] at leaf grain (citations vs. fallout, and the leaf's
    /// neighborhood).
    Leaf(&'a str),
    /// The bare name matches both a member and a requirement — `explain` never
    /// guesses, so the caller must retry with one of the listed qualified spellings.
    Ambiguous(Vec<String>),
    /// The bare name matches no member, no requirement, and carries no leaf-address
    /// slash — a clean "nothing by this name" read, not a namespace preference.
    NotFound(&'a str),
}

/// Resolve `target` into its [`Species`] over the same corpus [`explain`]'s caller
/// already assembled for `check` — `by_kind` (every opt-in kind's [`Features`]) for
/// member existence, `roster` (the composed requirement namespace) for requirement
/// existence. A bare name in both is `Ambiguous`; a bare name in neither, absent a `/`,
/// is `NotFound` — `explain` never silently prefers one namespace over the other.
fn resolve<'a>(
    by_kind: &BTreeMap<&str, &[Features]>,
    roster: &BTreeMap<String, Requirement>,
    target: &'a str,
) -> Species<'a> {
    if let Some(name) = target.strip_prefix("member:") {
        return Species::Member(name);
    }
    if let Some(name) = target.strip_prefix("requirement:") {
        return Species::Requirement(name);
    }
    if let Some(address) = target.strip_prefix("address:") {
        return Species::Leaf(address);
    }
    if target.contains('/') {
        return Species::Leaf(target);
    }

    let is_member = by_kind
        .values()
        .flat_map(|members| members.iter())
        .any(|features| features.id == target);
    let is_requirement = roster.contains_key(target);

    match (is_member, is_requirement) {
        (true, true) => Species::Ambiguous(vec![
            format!("member:{target}"),
            format!("requirement:{target}"),
        ]),
        (true, false) => Species::Member(target),
        (false, true) => Species::Requirement(target),
        (false, false) => Species::NotFound(target),
    }
}

/// `temper explain <target>` — the one read verb: resolve `target`'s [`Species`] and dispatch to whichever
/// of the four traversals answer it, so the single verb answers every read question
/// `why`/`requirements`/`impact`/`context` used to split across four CLI spellings. A
/// read, never a gate — the caller prints this and exits zero on every input, an
/// ambiguous or unrecognized target included.
///
/// `roster` is the requirement namespace `check` gates; `contracts` are the resolved
/// contracts the gate judges with, keyed by kind, so [`why`] can name a member's
/// clauses by the addresses their findings print; `edges` is the
/// declared relationship set [`why`]'s edge walk resolves; `mention_edges` is the
/// already-resolved mention edge set the same walk folds in, so a member's only
/// outgoing reference being a mention still narrates rather than reading "it points at
/// no member"; `registrations`,
/// `repo_files`, and `directive_edges` are the exact reachability/directive inputs
/// [`impact`]'s blast radius ranges over; `citations` are the declared one-way edges a
/// leaf-grain answer reports separately from fallout. Every one is the identical input
/// the gate's own predicates range over (READ-EDGE-UNIFY), so `explain` cannot disagree
/// with a green `check`.
///
/// `tap_records` and `tap_older_version` are the per-machine tap log's readout — the
/// evidence [`field`] narrates for a member target. They join to members through the
/// same `by_kind` corpus above, never a private read, so a green `check` and a field
/// strand cannot disagree.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn explain(
    custom: &[CustomMember],
    roster: &BTreeMap<String, Requirement>,
    contracts: &BTreeMap<String, Contract>,
    by_kind: &BTreeMap<&str, &[Features]>,
    edges: &[Edge],
    mention_edges: &[ResolvedEdge],
    registrations: &BTreeMap<&str, Vec<Registration>>,
    repo_files: &[String],
    directive_edges: &[ResolvedEdge],
    citations: &[Citation],
    tap_records: &[TapRecord],
    tap_older_version: usize,
    target: &str,
) -> String {
    match resolve(by_kind, roster, target) {
        Species::Member(name) => {
            let member_index = build_member_index(by_kind);
            let mut out = why_impl(
                custom,
                roster,
                contracts,
                by_kind,
                edges,
                mention_edges,
                &member_index,
                name,
            );
            out.push('\n');
            out.push_str(&impact_impl(
                roster,
                by_kind,
                registrations,
                repo_files,
                directive_edges,
                citations,
                &member_index,
                name,
            ));
            out.push('\n');
            out.push_str(&context_impl(by_kind, citations, &member_index, name));
            // The field strand is evidence, not a gate: an absent/empty log narrates
            // nothing (an empty string), so it is only joined when the tap log carried
            // records to narrate.
            let field_strand = telemetry::field(tap_records, tap_older_version, by_kind, name);
            if !field_strand.is_empty() {
                out.push('\n');
                out.push_str(&field_strand);
            }
            out
        }
        Species::Requirement(name) => requirements(custom, roster, by_kind, Some(name)),
        Species::Leaf(address) => {
            let mut out = impact(
                roster,
                by_kind,
                registrations,
                repo_files,
                directive_edges,
                citations,
                address,
            );
            out.push('\n');
            out.push_str(&context(by_kind, citations, address));
            out
        }
        Species::Ambiguous(spellings) => format!(
            "`{target}` names more than one thing in the surface. `explain` never \
             guesses which you mean — retry with one of its qualified spellings:\n{}\n",
            spellings
                .iter()
                .map(|spelling| format!("  • `{spelling}`"))
                .collect::<Vec<_>>()
                .join("\n")
        ),
        Species::NotFound(name) => format!(
            "No member, requirement, or leaf address named `{name}` is in the surface. \
             `explain` reads the authored surface's members, its requirement roster, and \
             leaf-grain addresses (`<member>/<kind>/<key>/<child-path>`); check the \
             name.\n"
        ),
    }
}

/// `explain`'s **why** strand — narrate everything that holds `member` in place: the
/// requirements it `satisfies` (each with its authored rationale and the requirement's
/// own `prose`), the default contract its kind binds, and its resolved edges in and out.
/// A read, never a
/// gate — the caller prints this and exits zero on every input, including a name no
/// member bears.
///
/// The edge walk ranges over the **gate's own resolved edge set** — `by_kind` (the
/// by-kind [`Features`] corpus) and `edges` (the declared `[[kind.<name>.relationships]]`
/// set) are the *same* two the `check` arm builds, and `why` runs them through the
/// identical [`graph::resolved_edges`] the gate's `check`/`acyclic`/`degree` range over,
/// folding in `mention_edges` — the already-resolved mention edges `graph::degree` also
/// ranges over — so a member whose only outgoing reference is a mention narrates it
/// rather than reading "it points at no member".
/// So `why`'s edge narration cannot disagree with the gate (READ-EDGE-UNIFY): a
/// `routes_to` edge the gate resolves is the exact edge `why` narrates, and a member
/// with no resolved edge stays silent.
///
/// The `roster` is the requirement namespace `check` gates, so a `satisfies` link
/// narrates as filled exactly when a green `check` counts it covered.
#[must_use]
pub fn why(
    custom: &[CustomMember],
    roster: &BTreeMap<String, Requirement>,
    contracts: &BTreeMap<String, Contract>,
    by_kind: &BTreeMap<&str, &[Features]>,
    edges: &[Edge],
    mention_edges: &[ResolvedEdge],
    member: &str,
) -> String {
    let member_index = build_member_index(by_kind);
    why_impl(
        custom,
        roster,
        contracts,
        by_kind,
        edges,
        mention_edges,
        &member_index,
        member,
    )
}

/// Narrate the default contract `kind` binds, and every clause in it **by address** —
/// the same label each clause's findings print, so the author who wants to dial one
/// reads its name here and spells it straight into the dial rather than guessing at it.
///
/// The clause is named, never explained: its predicate and severity are the finding's to
/// state and its guidance's to teach. A kind the caller threaded no contract for narrates
/// the binding alone — a read never gates, so an unmodeled kind is silence, not a finding.
fn narrate_governing_contract(
    out: &mut String,
    contracts: &BTreeMap<String, Contract>,
    kind: &str,
) {
    let _ = writeln!(
        out,
        "Governing default contract: its `{kind}` kind binds the `{kind}` default \
         contract, whose clauses check it.",
    );
    let Some(contract) = contracts.get(kind) else {
        out.push('\n');
        return;
    };
    if contract.clauses.is_empty() {
        let _ = writeln!(
            out,
            "That contract declares no clauses, so nothing about this member's own \
             surface is checked.\n"
        );
        return;
    }
    let _ = writeln!(
        out,
        "Its clauses, by the address each one's findings report under:"
    );
    for clause in &contract.clauses {
        let _ = writeln!(out, "  • `{}`", clause.label);
    }
    out.push('\n');
}

/// Implementation of [`why`] using a pre-built member index.
#[must_use]
#[allow(clippy::too_many_arguments)]
fn why_impl(
    custom: &[CustomMember],
    roster: &BTreeMap<String, Requirement>,
    contracts: &BTreeMap<String, Contract>,
    by_kind: &BTreeMap<&str, &[Features]>,
    edges: &[Edge],
    mention_edges: &[ResolvedEdge],
    member_index: &BTreeMap<&str, Vec<(&str, &Features)>>,
    member: &str,
) -> String {
    let mut resolved = graph::resolved_edges(edges, by_kind).resolved;
    let (resolved_mentions, dangling_mentions) =
        graph::partition_mentions(mention_edges, by_kind, roster);
    resolved.extend(resolved_mentions);

    let mut matches: Vec<Member> = members(custom)
        .into_iter()
        .filter(|m| m.id == member)
        .collect();
    if let Some(indexed_members) = member_index.get(member) {
        for (kind, features) in indexed_members {
            if !matches.iter().any(|m| m.kind == *kind) {
                matches.push(Member {
                    kind: kind.to_string(),
                    id: member.to_string(),
                    satisfies: features
                        .satisfies
                        .iter()
                        .cloned()
                        .map(Satisfies::new)
                        .collect(),
                });
            }
        }
    }

    if matches.is_empty() {
        return format!(
            "No member named `{member}` is in the surface. `why` reads the authored \
             surface's members — skills, rules, and every custom kind's members; check \
             the name.\n"
        );
    }

    let mut out = String::new();
    for (index, member_obj) in matches.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        why_one(
            &mut out,
            member_obj,
            roster,
            contracts,
            &resolved,
            &dangling_mentions,
        );
    }
    out
}

/// Narrate one matched member into `out` — the full forward walk for a single
/// `(kind, id)` node.
fn why_one(
    out: &mut String,
    member: &Member,
    roster: &BTreeMap<String, Requirement>,
    contracts: &BTreeMap<String, Contract>,
    resolved: &[ResolvedEdge],
    dangling_mentions: &[ResolvedEdge],
) {
    let _ = writeln!(
        out,
        "Member `{}` ({}) — everything that holds it in place:\n",
        member.id, member.kind
    );

    // Forward walk: the requirements this member fills, each with its authored
    // rationale and the requirement's own `prose`.
    if member.satisfies.is_empty() {
        let _ = writeln!(
            out,
            "It fills no requirements — it opts into no `satisfies` link, so it is \
             governed by its kind's default contract alone.\n"
        );
    } else {
        let _ = writeln!(out, "Requirements it satisfies:");
        for satisfies in &member.satisfies {
            narrate_filled(out, satisfies, roster);
        }
        out.push('\n');
    }

    narrate_governing_contract(out, contracts, &member.kind);

    // The edges in and out — the member's node in the **gate's resolved edge set**
    // (`crate::graph::resolved_edges`), not a private re-derivation (READ-EDGE-UNIFY).
    // A dangling declared reference resolves to no node, so it appears in neither list —
    // route resolution is the gate's finding to report, not `why`'s. A dangling *mention*
    // is narrated below its own way: the gate's `route_mentions` verdict, surfaced as
    // teaching rather than silently dropped.
    let node: (String, String) = (member.kind.clone(), member.id.clone());

    let outgoing: Vec<&ResolvedEdge> = resolved.iter().filter(|edge| edge.from == node).collect();
    if outgoing.is_empty() {
        let _ = writeln!(
            out,
            "Edges out: it points at no member (it declares no resolved reference)."
        );
    } else {
        let _ = writeln!(
            out,
            "Edges out (the resolved references it declares, the exact set the gate ranges over):"
        );
        for edge in outgoing {
            let (to_kind, to_id) = &edge.to;
            let _ = writeln!(
                out,
                "  • it points at `{to_id}` ({to_kind}) via its `{}` field",
                edge.field
            );
        }
    }

    // Dangling mentions out: a deferred mention whose target is absent from the discovered
    // corpus. It fired the gate's `graph.route` finding, so it reads as dangling here, not
    // as a resolved edge.
    for edge in dangling_mentions.iter().filter(|edge| edge.from == node) {
        let (to_kind, to_id) = &edge.to;
        let target = if to_kind == graph::REQUIREMENT_KIND {
            format!("requirement `{to_id}`")
        } else {
            format!("`{to_id}` ({to_kind})")
        };
        let _ = writeln!(
            out,
            "  • it mentions {target}, which resolves to nothing in the surface — a \
             dangling mention `check` reports."
        );
    }

    let incoming: Vec<&ResolvedEdge> = resolved.iter().filter(|edge| edge.to == node).collect();
    if incoming.is_empty() {
        let _ = writeln!(out, "Edges in: no member points at it.");
    } else {
        let _ = writeln!(out, "Edges in (the resolved references that point at it):");
        for edge in incoming {
            let (from_kind, from_id) = &edge.from;
            let _ = writeln!(
                out,
                "  • `{from_id}` ({from_kind}) points at it via its `{}` field",
                edge.field
            );
        }
    }
}

/// Narrate one `satisfies` link of a member's forward walk: the requirement it fills,
/// its authored rationale, and — resolving the link — the requirement's own `prose`
/// and whether it is required, or that the link dangles when no such requirement is
/// declared (the same referential fault [`crate::coverage`] gates, surfaced as teaching).
fn narrate_filled(out: &mut String, satisfies: &Satisfies, roster: &BTreeMap<String, Requirement>) {
    let rationale = satisfies.rationale.as_deref().map_or_else(
        || "no rationale authored".to_string(),
        |r| format!("\"{r}\""),
    );
    let _ = writeln!(out, "  • `{}` — {rationale}", satisfies.requirement);

    match roster.get(&satisfies.requirement) {
        Some(requirement) => {
            if let Some(prose) = &requirement.prose {
                let _ = writeln!(out, " It means: \"{prose}\".");
            }
            let obligation = if requirement.required {
                "It is required — at least one member must fill it."
            } else {
                "It is advisory — leaving it unfilled never gates."
            };
            let _ = writeln!(out, "      {obligation}");
        }
        None => {
            let _ = writeln!(
                out,
                "      This link dangles: no requirement `{}` is declared, so it is a \
                 silent no-op the gate reports.",
                satisfies.requirement
            );
        }
    }
}

/// `explain`'s **impact** strand — narrate the deterministic **blast radius** of removing or
/// renaming `member`: the graph
/// payoff promised, given a verb. Three strands, each read off the graph
/// data `check` already carries — no second build, no new engine semantics:
///
/// 1. **Requirements left unfilled** — a requirement `member` satisfies whose *only*
///    satisfier is `member`, so removing it drops coverage to zero (an error for a
///    `required` one, silent for an advisory).
/// 2. **Directive edges left unbacked** — an `@import` from another member that
///    resolves to `member`'s file; removing the file leaves that import backing
///    nothing, the silent-context-loss class made author-time.
/// 3. **Reachability that dies with it** — a member live now only because `member`
///    imports it (its own registration dead); removing `member` unreaches it
///    ([`graph::reachability_orphaned`], the same closure the gate's `reachable` runs).
///
/// The family gains **leaf grain**: a `target` naming a nested member's leaf — the `<member>/<kind>/<key>/<child-path>`
/// address — dispatches to [`impact_leaf`], which resolves the leaf against the lock's
/// serialized nested-member leaves and reports its **citations separately from fallout**. A `target` with no `/` is
/// a bare member name and takes the member-grain path below, unchanged.
///
/// A read, never a gate: the caller prints this and exits zero on every input, a name no
/// member or leaf bears included. `roster` is the namespace `check` gates; `by_kind`,
/// `registrations`, `repo_files`, and `directive_edges` are the exact graph inputs the
/// gate's predicates range over (READ-EDGE-UNIFY), so the read cannot disagree with a
/// green `check`. `by_kind` also carries each member's serialized nested-member leaves,
/// the leaf-grain surface; `citations` are the declared one-way edges naming a leaf.
#[must_use]
#[allow(clippy::too_many_arguments)]
fn impact(
    roster: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    registrations: &BTreeMap<&str, Vec<Registration>>,
    repo_files: &[String],
    directive_edges: &[ResolvedEdge],
    citations: &[Citation],
    target: &str,
) -> String {
    let member_index = build_member_index(by_kind);
    impact_impl(
        roster,
        by_kind,
        registrations,
        repo_files,
        directive_edges,
        citations,
        &member_index,
        target,
    )
}

/// Implementation of [`impact`] using a pre-built member index.
#[must_use]
#[allow(clippy::too_many_arguments)]
fn impact_impl(
    roster: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    registrations: &BTreeMap<&str, Vec<Registration>>,
    repo_files: &[String],
    directive_edges: &[ResolvedEdge],
    citations: &[Citation],
    member_index: &BTreeMap<&str, Vec<(&str, &Features)>>,
    target: &str,
) -> String {
    if target.contains('/') {
        return impact_leaf(by_kind, citations, target);
    }

    let matches: Vec<(&str, &Features)> = member_index
        .get(target)
        .map(|v| v.iter().map(|(k, f)| (*k, *f)).collect())
        .unwrap_or_default();

    if matches.is_empty() {
        return format!(
            "No member named `{target}` is in the surface. `impact` reads the authored \
             surface's members — skills, rules, and every custom kind's members; check \
             the name.\n"
        );
    }

    let mut out = String::new();
    for (index, (kind, features)) in matches.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        impact_one_impl(
            &mut out,
            kind,
            features,
            roster,
            by_kind,
            member_index,
            registrations,
            repo_files,
            directive_edges,
        );
    }
    out
}

/// A parsed **leaf address** — the `<member>/<kind>/<key>/<child-path>` spelling `impact`
/// accepts to name a single nested member's leaf. The three identity segments are `/`-separated; the child
/// path keeps its own dots (`rejected.baked-projection.because`), so it is the whole
/// remainder after the third slash — `splitn(4, '/')`, never a plain split that would
/// mangle a dotted collection path.
struct ParsedLeaf<'a> {
    member: &'a str,
    kind: &'a str,
    key: &'a str,
    child_path: &'a str,
}

/// Parse a `/`-bearing `target` into its four leaf-address segments, or `None` when a
/// segment is empty (a malformed address the caller reports as such). Keyed and structural
/// — the address rides the shape the author already wrote, stable under content edits.
fn parse_leaf_address(target: &str) -> Option<ParsedLeaf<'_>> {
    let mut parts = target.splitn(4, '/');
    let member = parts.next()?;
    let kind = parts.next()?;
    let key = parts.next()?;
    let child_path = parts.next()?;
    if member.is_empty() || kind.is_empty() || key.is_empty() || child_path.is_empty() {
        return None;
    }
    Some(ParsedLeaf {
        member,
        kind,
        key,
        child_path,
    })
}

/// Resolve a parsed leaf address against the lock's **serialized nested-member leaves**
/// ([`Features::embedded_leaves`]) — the tier-1, offline read the leaf-grain `impact` stands
/// on. Returns the matched leaf's outer kind and authored value, or `None` when no
/// member's nested member carries that `(kind, key, child-path)`. Ranges over every
/// outer kind's members, since a leaf may live in any nested-member-bearing kind.
fn resolve_leaf<'a>(
    by_kind: &BTreeMap<&str, &'a [Features]>,
    parsed: &ParsedLeaf<'_>,
) -> Option<(String, &'a str)> {
    for (&outer_kind, members) in by_kind {
        for features in *members {
            if features.id != parsed.member {
                continue;
            }
            for (address, value) in features.embedded_leaves() {
                if address.kind == parsed.kind
                    && address.key == parsed.key
                    && address.child_path == parsed.child_path
                {
                    return Some((outer_kind.to_string(), value));
                }
            }
        }
    }
    None
}

/// `explain`'s **impact** strand at a leaf address — narrate a nested member's leaf at
/// **leaf grain**:
/// resolve the
/// leaf against the lock's serialized nested-member leaves, then report its **citations separately
/// from fallout**. A leaf is obligation-free — its
/// citations neither gate nor block a rewrite — so the fallout line states exactly that,
/// distinct from the citation list a join/reachability member-grain report would carry.
///
/// A read, never a gate: an ill-formed or unresolved address is narrated plainly and the
/// caller still exits zero (the read family is never the gate).
fn impact_leaf(
    by_kind: &BTreeMap<&str, &[Features]>,
    citations: &[Citation],
    target: &str,
) -> String {
    let Some(parsed) = parse_leaf_address(target) else {
        return format!(
            "`{target}` is not a well-formed leaf address. A leaf address is \
             `<member>/<kind>/<key>/<child-path>` — the member, the nested member's kind and \
             key, and the child path within it (`chosen`, or `rejected.baked-projection.because`).\n"
        );
    };

    let Some((outer_kind, value)) = resolve_leaf(by_kind, &parsed) else {
        return format!(
            "No leaf `{target}` is in the surface's serialized nested-member leaves. `impact` \
             reads leaf grain off the lock's serialized nested members — check the member, \
             kind, key, and child path; a document carrying no nested members is not \
             represented at leaf grain.\n"
        );
    };

    let ParsedLeaf {
        member,
        kind,
        key,
        child_path,
    } = parsed;

    let mut out = String::new();
    let _ = writeln!(out, "Leaf `{target}` ({outer_kind}) — leaf grain:\n");
    let _ = writeln!(
        out,
        "It is the `{child_path}` leaf of the `{kind}` member `{key}` in member `{member}`.",
    );
    let _ = writeln!(out, "Authored value: \"{value}\"\n");

    // Citations — the declared one-way edges naming this leaf, resolution-checked (we only
    // reach here for a leaf that resolves) and obligation-free. Reported *separately* from
    // fallout: a citation is never fallout. The shared
    // helper is reused by `context` at the same grain.
    narrate_citers(&mut out, citations, member, kind, key, child_path);
    out.push('\n');

    // Fallout — a leaf carries none: deleting or rewording it is never blocked by its
    // citations, the whole point of the obligation-free annotation class.
    let _ = writeln!(
        out,
        "Fallout: none — a leaf is obligation-free. Deleting or rewording it is never \
         blocked by its citations; the citer is told (`45-governance.md`, address grain), \
         which is the point."
    );
    out.push('\n');

    // Coverage — the leaf-grain answer names what it cannot see: the disclosure ships WITH the found answer, not only in the
    // not-found error, so an incomplete answer never wears complete clothes.
    disclose_coverage(&mut out, by_kind);

    out
}

/// Select citations matching a predicate — the shared filter both `narrate_citers` and
/// `context_member_one` use to identify relevant citations. Extracted so the two call sites
/// cannot disagree on what cites a leaf or member, exactly as the edge walk shares the
/// gate's resolved set (READ-EDGE-UNIFY).
fn select_citers<F>(citations: &[Citation], predicate: F) -> Vec<&Citation>
where
    F: Fn(&Citation) -> bool,
{
    citations
        .iter()
        .filter(|citation| predicate(citation))
        .collect()
}

/// Narrate the **citations** naming a leaf — the declared one-way edges (obligation-free,
/// resolution-checked) both `impact` and `context` report at leaf grain.
/// Shared so the two verbs cannot disagree on
/// what cites a leaf, exactly as the edge walk shares the gate's resolved set (READ-EDGE-UNIFY).
fn narrate_citers(
    out: &mut String,
    citations: &[Citation],
    member: &str,
    kind: &str,
    key: &str,
    child_path: &str,
) {
    let citers = select_citers(citations, |citation| {
        citation.target.member == member
            && citation.target.kind == kind
            && citation.target.key == key
            && citation.target.child_path == child_path
    });
    if citers.is_empty() {
        let _ = writeln!(
            out,
            "Citations (declared one-way edges that name this leaf — obligation-free): none \
             — no member cites it."
        );
    } else {
        let _ = writeln!(
            out,
            "Citations (declared one-way edges that name this leaf — obligation-free):"
        );
        for citation in citers {
            let _ = writeln!(
                out,
                "  • `{}` ({}) cites it — a resolved citation, obligation-free.",
                citation.from, citation.from_kind
            );
        }
    }
}

/// Append the shared **coverage disclosure** every leaf-grain answer closes with:
/// the count of members carrying no serialized nested-member leaves —
/// the documents that carry no nested members a leaf-grain read cannot represent. Under
/// the gradient a mixed-posture corpus is the standing state, not an edge case, so an
/// answer hiding its blind spot erodes the verb exactly as a false gate-block erodes the
/// gate. Shared by `impact`'s and `context`'s leaf-grain answers, so the
/// disclosure is one wording that ships WITH the verb, never after.
fn disclose_coverage(out: &mut String, by_kind: &BTreeMap<&str, &[Features]>) {
    let leafless = by_kind
        .values()
        .flat_map(|members| members.iter())
        .filter(|features| features.embedded_leaves().is_empty())
        .count();
    let (noun, verb) = if leafless == 1 {
        ("document", "carries")
    } else {
        ("documents", "carry")
    };
    let _ = writeln!(
        out,
        "Coverage: {leafless} {noun} {verb} no nested members — not represented at leaf grain \
         (carrying no serialized nested-member leaves)."
    );
}

/// `explain`'s **context** strand — emit the **declared neighborhood** of a member or a
/// nested member's leaf:
/// its nested-member slot, its siblings, the members that cite it, and the requirements
/// its member satisfies — the pre-edit context bundle for the primary author. Consumes
/// only the lock's serialized nested-member leaves (`by_kind`) and declared citations:
/// offline, tier-1, no runtime.
///
/// A `/`-bearing `address` is a leaf (`<member>/<kind>/<key>/<child-path>`) reported at leaf grain
/// ([`context_leaf`]); a bare name is a member reported whole ([`context_member`]). Both are
/// leaf-grain answers, so both close with the shared [`disclose_coverage`] — a mixed-posture corpus
/// is the standing state, and an answer hiding what it cannot see erodes the verb.
///
/// A read, never a gate: an unresolved or ill-formed address is narrated plainly and the caller
/// still exits zero.
#[must_use]
fn context(by_kind: &BTreeMap<&str, &[Features]>, citations: &[Citation], address: &str) -> String {
    let member_index = build_member_index(by_kind);
    context_impl(by_kind, citations, &member_index, address)
}

/// Implementation of [`context`] using a pre-built member index.
fn context_impl(
    by_kind: &BTreeMap<&str, &[Features]>,
    citations: &[Citation],
    member_index: &BTreeMap<&str, Vec<(&str, &Features)>>,
    address: &str,
) -> String {
    if address.contains('/') {
        context_leaf(by_kind, citations, address)
    } else {
        context_member_impl(by_kind, citations, member_index, address)
    }
}

/// Narrate a nested member's leaf neighborhood: its nested-member slot and authored
/// value, its **siblings** (the other leaves of the same nested member), the members
/// that **cite** it, and the requirements its member **satisfies** — then the shared
/// coverage disclosure. Resolved against the lock's serialized nested-member leaves
/// exactly as [`impact_leaf`] resolves them, so the two verbs agree on what a leaf's
/// neighborhood is.
fn context_leaf(
    by_kind: &BTreeMap<&str, &[Features]>,
    citations: &[Citation],
    address: &str,
) -> String {
    let Some(parsed) = parse_leaf_address(address) else {
        return format!(
            "`{address}` is not a well-formed leaf address. A leaf address is \
             `<member>/<kind>/<key>/<child-path>` — the member, the nested member's kind and \
             key, and the child path within it (`chosen`, or `rejected.baked-projection.because`).\n"
        );
    };

    let Some((outer_kind, value)) = resolve_leaf(by_kind, &parsed) else {
        return format!(
            "No leaf `{address}` is in the surface's serialized nested-member leaves. \
             `context` reads leaf grain off the lock's serialized nested members — check the \
             member, kind, key, and child path; a document carrying no nested members is not \
             represented at leaf grain.\n"
        );
    };

    let ParsedLeaf {
        member,
        kind,
        key,
        child_path,
    } = parsed;

    let mut out = String::new();
    let _ = writeln!(
        out,
        "Leaf `{address}` ({outer_kind}) — its declared neighborhood:\n"
    );
    let _ = writeln!(
        out,
        "Nested-member slot: the `{child_path}` leaf of the `{kind}` member `{key}` in member `{member}`."
    );
    let _ = writeln!(out, "Authored value: \"{value}\"\n");

    // Siblings — the other leaves of the same nested member (same member, kind, key),
    // the co-resident context an author editing this leaf wants beside it.
    let siblings = sibling_leaves(by_kind, member, kind, key, child_path);
    if siblings.is_empty() {
        let _ = writeln!(
            out,
            "Siblings: none — it is the only leaf of the `{kind}` member `{key}`."
        );
    } else {
        let _ = writeln!(
            out,
            "Siblings (the other leaves of the `{kind}` member `{key}`):"
        );
        for (sibling_path, sibling_value) in siblings {
            let _ = writeln!(out, "  • `{sibling_path}` — \"{sibling_value}\"");
        }
    }
    out.push('\n');

    // Citers — the same declared one-way edges `impact` reports at leaf grain (shared helper).
    narrate_citers(&mut out, citations, member, kind, key, child_path);
    out.push('\n');

    // Satisfied requirements — the demands the member the leaf lives in opts into filling.
    narrate_satisfied(&mut out, by_kind, member);
    out.push('\n');

    // Coverage — the leaf-grain answer names what it cannot see, shipping WITH the verb.
    disclose_coverage(&mut out, by_kind);

    out
}

/// Implementation of [`context_member`] using a pre-built member index.
fn context_member_impl(
    by_kind: &BTreeMap<&str, &[Features]>,
    citations: &[Citation],
    member_index: &BTreeMap<&str, Vec<(&str, &Features)>>,
    member: &str,
) -> String {
    let matches: Vec<(&str, &Features)> = member_index
        .get(member)
        .map(|v| v.iter().map(|(k, f)| (*k, *f)).collect())
        .unwrap_or_default();

    if matches.is_empty() {
        return format!(
            "No member named `{member}` is in the surface. `context` reads the authored \
             surface's members — skills, rules, and every custom kind's members; check the \
             name.\n"
        );
    }

    let mut out = String::new();
    for (index, (kind, features)) in matches.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        context_member_one_impl(&mut out, kind, features, by_kind, citations, member_index);
    }

    disclose_coverage(&mut out, by_kind);

    out
}

/// Implementation of [`context_member_one`] using a pre-built member index.
fn context_member_one_impl(
    out: &mut String,
    kind: &str,
    features: &Features,
    _by_kind: &BTreeMap<&str, &[Features]>,
    citations: &[Citation],
    member_index: &BTreeMap<&str, Vec<(&str, &Features)>>,
) {
    let _ = writeln!(
        out,
        "Member `{}` ({kind}) — its declared neighborhood:\n",
        features.id
    );

    if features.nested_members.is_empty() {
        let _ = writeln!(
            out,
            "Nested members: none — it carries no nested member, so it holds no leaf at leaf \
             grain."
        );
    } else {
        let _ = writeln!(out, "Nested members (the embedded members it carries):");
        for nested in &features.nested_members {
            let fields: Vec<String> = nested
                .addressed_leaves()
                .into_iter()
                .map(|(child_path, _)| child_path)
                .collect();
            let _ = writeln!(
                out,
                "  • `{}` member `{}` — leaves: {}",
                nested.kind,
                nested.key,
                fields.join(", ")
            );
        }
    }
    out.push('\n');

    let citers = select_citers(citations, |citation| citation.target.member == features.id);
    if citers.is_empty() {
        let _ = writeln!(
            out,
            "Citations (declared one-way edges naming any of its leaves — obligation-free): none \
             — no member cites it."
        );
    } else {
        let _ = writeln!(
            out,
            "Citations (declared one-way edges naming any of its leaves — obligation-free):"
        );
        for citation in citers {
            let target = &citation.target;
            let _ = writeln!(
                out,
                "  • `{}` ({}) cites `{}/{}/{}/{}` — a resolved citation, obligation-free.",
                citation.from,
                citation.from_kind,
                target.member,
                target.kind,
                target.key,
                target.child_path
            );
        }
    }
    out.push('\n');

    narrate_satisfied_impl(out, member_index, &features.id);
}

/// The other leaves of the nested member `(member, kind, key)` — every serialized leaf of
/// that member except the one at `child_path`, paired with its authored value in the
/// lock's stable order. The co-resident siblings `context` reports beside an addressed
/// leaf.
fn sibling_leaves<'a>(
    by_kind: &BTreeMap<&str, &'a [Features]>,
    member: &str,
    kind: &str,
    key: &str,
    child_path: &str,
) -> Vec<(String, &'a str)> {
    let mut out = Vec::new();
    for members in by_kind.values() {
        for features in *members {
            if features.id != member {
                continue;
            }
            for (address, value) in features.embedded_leaves() {
                if address.kind == kind && address.key == key && address.child_path != child_path {
                    out.push((address.child_path, value));
                }
            }
        }
    }
    out
}

/// Narrate the requirements the member `member` opts into filling — read off each matching
/// member's serialized `satisfies` (`Features::satisfies`), the lock-only view the leaf-grain
/// read stands on. A member fills only the demands it names, so an empty set is stated plainly.
fn narrate_satisfied(out: &mut String, by_kind: &BTreeMap<&str, &[Features]>, member: &str) {
    let member_index = build_member_index(by_kind);
    narrate_satisfied_impl(out, &member_index, member);
}

/// Implementation of [`narrate_satisfied`] using a pre-built member index.
fn narrate_satisfied_impl(
    out: &mut String,
    member_index: &BTreeMap<&str, Vec<(&str, &Features)>>,
    member: &str,
) {
    let satisfied: Vec<&str> = member_index
        .get(member)
        .map(|entries| {
            entries
                .iter()
                .flat_map(|(_, features)| features.satisfies.iter().map(String::as_str))
                .collect()
        })
        .unwrap_or_default();
    if satisfied.is_empty() {
        let _ = writeln!(
            out,
            "Satisfied requirements: none — member `{member}` opts into no `satisfies` link."
        );
    } else {
        let _ = writeln!(
            out,
            "Satisfied requirements (the demands member `{member}` fills):"
        );
        for name in satisfied {
            let _ = writeln!(out, "  • `{name}`");
        }
    }
}

/// The count of members opting into the requirement named `name`, across every kind —
/// the same opt-in join coverage counts, read off [`Features::satisfies`] so `impact`
/// agrees with a green `check`.
fn count_satisfiers(by_kind: &BTreeMap<&str, &[Features]>, name: &str) -> usize {
    by_kind
        .values()
        .flat_map(|members| members.iter())
        .filter(|features| features.satisfies.iter().any(|req| req == name))
        .count()
}

/// Narrate one matched node's blast radius into `out` — uses the member index for count_satisfiers.
#[allow(clippy::too_many_arguments)]
/// Implementation of [`impact_one`] using a pre-built member index.
fn impact_one_impl(
    out: &mut String,
    kind: &str,
    features: &Features,
    roster: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    _member_index: &BTreeMap<&str, Vec<(&str, &Features)>>,
    registrations: &BTreeMap<&str, Vec<Registration>>,
    repo_files: &[String],
    directive_edges: &[ResolvedEdge],
) {
    let _ = writeln!(
        out,
        "Member `{}` ({kind}) — the blast radius if it is removed or renamed:\n",
        features.id
    );

    // (1) Requirements it is the sole satisfier of — removing it drops them to zero.
    let sole: Vec<&Requirement> = features
        .satisfies
        .iter()
        .filter_map(|name| roster.get(name))
        .filter(|requirement| count_satisfiers(by_kind, &requirement.name) == 1)
        .collect();
    if sole.is_empty() {
        let _ = writeln!(
            out,
            "Requirements left unfilled: none — every requirement it fills has another \
             satisfier, so its removal drops no requirement to zero coverage."
        );
    } else {
        let _ = writeln!(
            out,
            "Requirements left unfilled (it is the only member filling them):"
        );
        for requirement in sole {
            if requirement.required {
                let _ = writeln!(
                    out,
                    "  • `{}` — required, so removing `{}` leaves it unfilled and fails the gate.",
                    requirement.name, features.id
                );
            } else {
                let _ = writeln!(
                    out,
                    "  • `{}` — advisory, so removing `{}` leaves it unfilled but never gates.",
                    requirement.name, features.id
                );
            }
        }
    }
    out.push('\n');

    // (2) `@import` directive edges that point at this member's file — removing the file
    // unbacks each.
    let node = (kind.to_string(), features.id.clone());
    let unbacked: Vec<&ResolvedEdge> = directive_edges
        .iter()
        .filter(|edge| edge.to == node)
        .collect();
    if unbacked.is_empty() {
        let _ = writeln!(
            out,
            "Directive edges left unbacked: none — no member `@import`s it, so removing it \
             leaves no import pointing at nothing."
        );
    } else {
        let _ = writeln!(
            out,
            "Directive edges left unbacked (members that `@import` it — removing its file \
             leaves each import loading nothing):"
        );
        for edge in unbacked {
            let (from_kind, from_id) = &edge.from;
            let _ = writeln!(
                out,
                "  • `{from_id}` ({from_kind}) imports it via `@{}` — the import would be unbacked.",
                DIRECTIVE_FIELD
            );
        }
    }
    out.push('\n');

    // (3) Members reachable now only because this one carried their liveness across an
    // import — removing it unreaches them.
    let orphaned =
        graph::reachability_orphaned(&node, registrations, by_kind, repo_files, directive_edges);
    if orphaned.is_empty() {
        let _ = writeln!(
            out,
            "Reachability that dies with it: none — no member depends on it to reach the \
             harness, so removing it leaves every other member as reachable as before."
        );
    } else {
        let _ = writeln!(
            out,
            "Reachability that dies with it (members live now only because it imports them):"
        );
        for (orphan_kind, orphan_id) in orphaned {
            let _ = writeln!(
                out,
                "  • `{orphan_id}` ({orphan_kind}) — its own registration is dead, and removing \
                 `{}` leaves no live importer to reach it.",
                features.id
            );
        }
    }
}

/// `explain`'s **requirements** strand — narrate the requirement roster. Without a name it
/// is the forward roster view: each requirement with its satisfier set and coverage
/// state. With a name it is the reverse walk over that one requirement: its satisfiers
/// and the blast radius a removal would strand.
/// A read, never a gate — the caller prints this and exits zero on every input.
///
/// The `roster` is the requirement namespace `check` gates. `by_kind` is the same
/// decidable corpus the gate's own selections count satisfiers from
/// (REQUIREMENT-GATE), so `explain` cannot report a requirement unfilled that `check`
/// counts as covered.
#[must_use]
fn requirements(
    custom: &[CustomMember],
    roster: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    name: Option<&str>,
) -> String {
    let members = members(custom);
    match name {
        Some(name) => requirement_detail(&members, by_kind, roster, name),
        None => roster_overview(&members, by_kind, roster),
    }
}

/// The forward roster view — every requirement, its satisfier set, and its coverage
/// state, in name order.
fn roster_overview(
    members: &[Member],
    by_kind: &BTreeMap<&str, &[Features]>,
    roster: &BTreeMap<String, Requirement>,
) -> String {
    if roster.is_empty() {
        return "No requirements are published — the roster is empty. Declare one in \
                the SDK program's `harness()` assembly to name an obligation.\n"
            .to_string();
    }

    let mut out = String::new();
    let plural = if roster.len() == 1 {
        "requirement"
    } else {
        "requirements"
    };
    let _ = writeln!(out, "The requirement roster ({} {plural}):\n", roster.len());
    for requirement in roster.values() {
        let satisfiers = satisfiers_of(members, by_kind, &requirement.name);
        let _ = writeln!(
            out,
            "  • `{}` — {}",
            requirement.name,
            coverage_state(requirement.required, satisfiers.len())
        );
        if let Some(prose) = &requirement.prose {
            let _ = writeln!(out, " It means: \"{prose}\".");
        }
        for (member, _) in &satisfiers {
            let _ = writeln!(out, "      ← `{}` ({})", member.id, member.kind);
        }
    }
    out
}

/// The reverse walk over one named requirement: its satisfiers (with the rationale
/// each authored) and the blast radius a removal would strand — the members whose
/// `satisfies` link would dangle, and, for a `required` requirement resting on a
/// single satisfier, that removing that one member leaves it unfilled and fails the
/// gate.
fn requirement_detail(
    members: &[Member],
    by_kind: &BTreeMap<&str, &[Features]>,
    roster: &BTreeMap<String, Requirement>,
    name: &str,
) -> String {
    let satisfiers = satisfiers_of(members, by_kind, name);

    let Some(requirement) = roster.get(name) else {
        // An undeclared name is not an error here — it is a read. Narrate that it is
        // undeclared, and if any member opts into it anyway, that those links dangle.
        let mut out =
            format!("No requirement named `{name}` is published in the composed roster.\n");
        if !satisfiers.is_empty() {
            let _ = writeln!(
                &mut out,
                "\nYet {} member(s) opt into it, so each `satisfies` link dangles \
                 (a silent no-op the gate reports):",
                satisfiers.len()
            );
            for (member, _) in &satisfiers {
                let _ = writeln!(&mut out, "  • `{}` ({})", member.id, member.kind);
            }
        }
        return out;
    };

    let mut out = format!("Requirement `{name}`:\n\n");
    if let Some(prose) = &requirement.prose {
        let _ = writeln!(&mut out, "  It means: \"{prose}\".");
    }
    let _ = writeln!(
        &mut out,
        "  {}\n",
        coverage_state(requirement.required, satisfiers.len())
    );

    if satisfiers.is_empty() {
        let _ = writeln!(&mut out, "  No member satisfies it.");
        return out;
    }

    let _ = writeln!(&mut out, "  Satisfied by:");
    for (member, rationale) in &satisfiers {
        let rationale = rationale.as_deref().map_or_else(
            || "no rationale authored".to_string(),
            |r| format!("\"{r}\""),
        );
        let _ = writeln!(
            &mut out,
            "    • `{}` ({}) — {rationale}",
            member.id, member.kind
        );
    }

    // Blast radius: removing the requirement strands every satisfier's opt-in link.
    let _ = writeln!(
        &mut out,
        "\n  Blast radius — removing `{name}` would strand {} `satisfies` link(s):",
        satisfiers.len()
    );
    for (member, _) in &satisfiers {
        let _ = writeln!(
            &mut out,
            "    • `{}` ({}) would dangle",
            member.id, member.kind
        );
    }

    // A required requirement resting on a single satisfier is load-bearing the other
    // direction: removing *that member* leaves the requirement unfilled and fails the gate.
    if requirement.required && satisfiers.len() == 1 {
        let (member, _) = &satisfiers[0];
        let _ = writeln!(
            &mut out,
            "\n  `{name}` is required and rests on a single satisfier — removing \
             `{}` ({}) would leave it unfilled, failing the gate.",
            member.id, member.kind
        );
    }

    out
}

/// The satisfier set of the requirement named `name` — every member whose `satisfies`
/// opts into it, paired with its authored rationale when one is available. The
/// rationale-carrying custom listing (`members`) is unioned with `by_kind` —
/// the same opt-in join the roster's own selection is built on — so a
/// satisfier `check` counts toward coverage is never missing here just because the
/// custom listing lags `by_kind` (a `required` requirement with a satisfier
/// locked on disk narrating as unfilled was exactly that drift, REQUIREMENT-GATE). A
/// `(kind, id)` the custom listing already carries (with its rationale) is
/// not duplicated from `by_kind`, whose decidable `Features::satisfies` carries none.
fn satisfiers_of(
    members: &[Member],
    by_kind: &BTreeMap<&str, &[Features]>,
    name: &str,
) -> Vec<(Member, Option<String>)> {
    let mut satisfiers: Vec<(Member, Option<String>)> = members
        .iter()
        .filter_map(|member| {
            member
                .satisfies
                .iter()
                .find(|satisfies| satisfies.requirement == name)
                .map(|satisfies| (member.clone(), satisfies.rationale.clone()))
        })
        .collect();

    for (&kind, features_slice) in by_kind {
        for features in *features_slice {
            if features.satisfies.iter().any(|req| req == name)
                && !satisfiers
                    .iter()
                    .any(|(member, _)| member.kind == kind && member.id == features.id)
            {
                satisfiers.push((
                    Member {
                        kind: kind.to_string(),
                        id: features.id.clone(),
                        satisfies: Vec::new(),
                    },
                    None,
                ));
            }
        }
    }

    satisfiers
}

/// The coverage-state clause for a requirement given whether it is `required` and how
/// many members satisfy it — the vocabulary the coverage gate reports in:
/// a `required` requirement with no satisfier is unfilled,
/// which `check` reports as an error; an advisory one is never a gate.
fn coverage_state(required: bool, satisfier_count: usize) -> String {
    match (required, satisfier_count) {
        (true, 0) => {
            "required, and unfilled — no member opts in, which `check` reports as an error"
                .to_string()
        }
        (true, count) => format!("required, filled by {count} member(s)"),
        (false, 0) => "advisory, and unfilled — never a gate".to_string(),
        (false, count) => format!("advisory, filled by {count} member(s)"),
    }
}

/// Assemble the corpus for the explain verb and dispatch to [`explain`] with the target:
/// read the workspace, discover all features, build contracts, rosters, and edge sets
/// exactly as `gate` does (READ-EDGE-UNIFY), so a read cannot disagree with the gate.
/// A read, never a gate — the caller prints the result and exits zero on every input.
///
/// # Errors
///
/// Propagates filesystem and corpus-assembly errors.
pub fn explain_target(target: &str) -> miette::Result<String> {
    const DEFAULT_WORKSPACE: &str = "./.temper";
    let workspace = PathBuf::from(DEFAULT_WORKSPACE);
    let harness_root = Path::new(".");
    // One ignore-honoring walk per flavor, shared across every kind and nested host this
    // read discovers ([`crate::import::Discovery`]).
    let discovery = crate::import::Discovery::new(harness_root);

    // The assembly's own declared facts, read first: the corpus below walks each
    // kind's governs locus off *this*.
    // No layers: `explain` narrates what this corpus declares. A policy layer is the
    // invocation's own, and the invocation here is a read.
    // Parse the lock document once for reuse across source-dependency checks, hoisting
    // the read/parse operation per the cost doctrine (engineering.md, "Cost scale is hoisted").
    let lock_doc = drift::read_lock_document(&workspace)?;
    // Empty cache for early assembly; will build a proper cache after lock_family returns.
    let empty_cache: compose::ManifestCache = BTreeMap::new();
    let compose::LockFamily { declarations, .. } = compose::assemble_lock_family(
        &discovery,
        &drift::read_declarations(&workspace)?,
        &[],
        &empty_cache,
    )?;

    // Build a shared manifest cache for this explain invocation.
    let manifest_cache = compose::build_manifest_cache(&discovery, &declarations)?;

    // Every embedded built-in kind's discovered features — the same generic loop
    // `gate`'s two-greens runs, not a hardcoded skill/rule pair
    // (MEMORY-ENTERS-REQUIREMENT-CORPUS), so a memory member's declared `satisfies`
    // reaches `explain` exactly as it reaches the gate's roster/graph/coverage tiers.
    let builtin_defs = builtin_kind::definitions();
    let builtin_units_and_features = compose::builtin_units_and_features_by_kind(
        &builtin_defs,
        &discovery,
        &declarations,
        &manifest_cache,
    )?;
    let builtin_features: BTreeMap<String, Vec<extract::Features>> = builtin_units_and_features
        .iter()
        .map(|(k, uaf)| (k.clone(), uaf.features.clone()))
        .collect();

    // Each kind's resolved contract, lifted exactly as `gate` lifts it, so the clause
    // addresses `explain` narrates are the ones a finding prints (READ-EDGE-UNIFY).
    let mut contracts: BTreeMap<String, Contract> = BTreeMap::new();
    for kind in builtin_defs.values() {
        contracts.insert(
            kind.name.clone(),
            compose::builtin_contract(&declarations.clauses, &kind.name)?,
        );
    }

    // Every lock-declared kind that is not a built-in — the same synthesis `gate` runs
    // (READ-EDGE-UNIFY), so a read cannot disagree with the gate about which kinds and
    // members exist.
    let mut custom_kinds: Vec<compose::CustomKindEntry> = Vec::new();
    let mut custom_units_and_features: Vec<(CustomKind, compose::KindUnitsAndFeatures)> =
        Vec::new();
    let mut custom_members: Vec<CustomMember> = Vec::new();
    let (custom_rows, _collisions) = compose::partition_kind_rows(&declarations, &builtin_defs)?;
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row)?;
        contracts.insert(
            row.name.clone(),
            compose::default_contract_from_rows(&declarations.clauses, &row.name)?,
        );
        let uaf = compose::kind_units_and_features(
            &custom_kind,
            &discovery,
            &declarations,
            &manifest_cache,
        )?;
        let features = &uaf.features;
        let units = &uaf.units;
        for unit in units {
            custom_members.push(CustomMember {
                kind: custom_kind.name.clone(),
                id: unit.id.clone(),
                satisfies: unit.satisfies_clauses.clone(),
            });
        }
        custom_kinds.push((custom_kind.clone(), features.clone()));
        custom_units_and_features.push((custom_kind, uaf));
    }
    let embedded_features = compose::embedded_features_by_kind(&declarations);
    let by_kind = compose::assemble_by_kind(&builtin_features, &custom_kinds, &embedded_features);

    // The one requirement namespace: the assembly's declared `[requirement.*]`
    // roster — a custom-kind member has no channel of its own to publish one (the
    // pre-0016 own-path surface that once carried it is retired).
    let roster: BTreeMap<String, compose::Requirement> = declarations
        .requirements
        .iter()
        .map(|row| Ok((row.name.clone(), drift::requirement_from_row(row)?)))
        .collect::<Result<_, compose::ClauseRowError>>()?;
    let assembly_edges = drift::edges_from_declarations(&declarations)?;
    // Authored mentions (route-resolved at check) and layout prose imports (path-resolved
    // at emit) both lift off the lock; `why` route-resolves them against this same corpus,
    // narrating a dangling mention as the gate's route finding rather than a resolved edge,
    // so a read cannot disagree with the gate (READ-EDGE-UNIFY).
    let mut mention_edges = drift::mention_edges_from_declarations(&declarations);
    mention_edges.extend(drift::import_edges_from_doc(&lock_doc)?);

    // The world's inbound registration channel set into each built-in kind — the same
    // derivation the gate's `reachable` runs, keyed by bare kind name to join `by_kind`.
    let mut registrations: BTreeMap<&str, Vec<kind::Registration>> = BTreeMap::new();
    for def in builtin_defs.values() {
        if !def.registration.is_empty() {
            registrations.insert(def.name.as_str(), def.registration.clone());
        }
    }

    let repo_files = compose::repo_file_set(Path::new("."));
    let directive_members = compose::directive_members_from_resolved(
        &builtin_units_and_features,
        &custom_units_and_features,
    );
    let directive_edges = crate::graph::classify_directives(&directive_members, &repo_files).edges;

    // Citations — the declared one-way edges naming a leaf; the floor carries no
    // producer yet, so the set is empty.
    let citations: Vec<Citation> = Vec::new();

    // The per-machine tap log at the workspace log path — the evidence the field strand
    // narrates. An absent log yields an empty readout (never an error), so the strand
    // narrates none.
    let readout = tap::read_log(&workspace)?;

    Ok(explain(
        &custom_members,
        &roster,
        &contracts,
        &by_kind,
        &assembly_edges,
        &mention_edges,
        &registrations,
        &repo_files,
        &directive_edges,
        &citations,
        &readout.records,
        readout.older_version,
        target,
    ))
}

#[cfg(test)]
mod impact_tests {
    //! Unit proofs of the four `impact` strands over hand-built graph inputs — the
    //! directive and reachability strands especially, which need an *importer* kind
    //! (a custom kind composing a `directives` primitive and a registration) the built-in
    //! skill/rule fixtures the e2e drives don't carry. The requirement strands are also
    //! e2e-proven in `tests/read_verbs.rs`.

    use super::*;
    use crate::extract::EmbeddedMember;

    /// A member's [`Features`] as `impact` reads them: its id, the requirements it opts
    /// into, and its `description` field (a blank one is a dead description-trigger
    /// world-edge). Body-derived features are inert here — `impact` reads the join,
    /// directive, and registration data, all set explicitly.
    fn feature(id: &str, satisfies: &[&str], description: Option<&str>) -> Features {
        let mut fields = BTreeMap::new();
        if let Some(text) = description {
            fields.insert(
                "description".to_string(),
                serde_json::Value::String(text.to_string()),
            );
        }
        Features {
            id: id.to_string(),
            fields,
            body_lines: 1,
            rendered_lines: Some(1),
            rendered_chars: Some(0),
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: Some(id.to_string()),
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            nested_members: Vec::new(),
            satisfies: satisfies.iter().map(|s| (*s).to_string()).collect(),
            edge_placements: None,
        }
    }

    /// A `required`/advisory requirement with everything else defaulted — the roster
    /// entry the coverage strand reads.
    fn req(name: &str, required: bool) -> Requirement {
        Requirement {
            name: name.to_string(),
            prose: None,
            kind: None,
            required,
            clauses: Vec::new(),
            verifier: None,
        }
    }

    /// A `(kind, id)` → `(kind, id)` `at-import` directive edge.
    fn directive(from: (&str, &str), to: (&str, &str)) -> ResolvedEdge {
        ResolvedEdge {
            from: (from.0.to_string(), from.1.to_string()),
            field: DIRECTIVE_FIELD.to_string(),
            to: (to.0.to_string(), to.1.to_string()),
        }
    }

    #[test]
    fn a_sole_satisfier_removal_leaves_its_required_requirement_unfilled() {
        // `solo` is the only member filling the required `r1`; removing it drops coverage
        // to zero, a gate failure. `r2` has two satisfiers, so `pair-a`'s removal strands
        // nothing there.
        let roster = BTreeMap::from([
            ("r1".to_string(), req("r1", true)),
            ("r2".to_string(), req("r2", true)),
        ]);
        let skills = [
            feature("solo", &["r1"], Some("d")),
            feature("pair-a", &["r2"], Some("d")),
            feature("pair-b", &["r2"], Some("d")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let registrations = BTreeMap::new();

        let solo = impact(&roster, &by_kind, &registrations, &[], &[], &[], "solo");
        assert!(
            solo.contains("Requirements left unfilled (it is the only member filling them):"),
            "{solo}"
        );
        assert!(solo.contains("`r1` — required"), "{solo}");
        assert!(solo.contains("fails the gate"), "{solo}");

        let pair = impact(&roster, &by_kind, &registrations, &[], &[], &[], "pair-a");
        assert!(
            pair.contains("Requirements left unfilled: none"),
            "a non-sole satisfier strands no requirement: {pair}"
        );
    }

    #[test]
    fn removing_an_imported_member_unbacks_the_import() {
        // `hub` `@import`s `leaf`; removing `leaf`'s file leaves that import backing
        // nothing — an unbacked pointer, the silent-context-loss class made author-time.
        let empty = BTreeMap::new();
        let docs = [
            feature("hub", &[], Some("d")),
            feature("leaf", &[], Some("d")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("doc", &docs[..])]);
        let registrations = BTreeMap::new();
        let edges = [directive(("doc", "hub"), ("doc", "leaf"))];

        let out = impact(&empty, &by_kind, &registrations, &[], &edges, &[], "leaf");
        assert!(out.contains("Directive edges left unbacked"), "{out}");
        assert!(
            out.contains("`hub` (doc) imports it via `@at-import`"),
            "{out}"
        );

        // `hub` imports but is not imported, so nothing points *at* it.
        let out = impact(&empty, &by_kind, &registrations, &[], &edges, &[], "hub");
        assert!(out.contains("Directive edges left unbacked: none"), "{out}");
    }

    #[test]
    fn removing_a_live_importer_unreaches_its_dead_dependent() {
        // `leaf` has a blank `description` — its own description-trigger world-edge is
        // dead — but `hub` (live) `@import`s it, carrying its liveness. Removing `hub`
        // leaves `leaf` with no live importer, so its reachability dies.
        let empty = BTreeMap::new();
        let docs = [
            feature("hub", &[], Some("present")),
            feature("leaf", &[], Some("")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("doc", &docs[..])]);
        let registrations = BTreeMap::from([(
            "doc",
            vec![Registration::DescriptionTrigger {
                field: "description".to_string(),
            }],
        )]);
        let edges = [directive(("doc", "hub"), ("doc", "leaf"))];

        let out = impact(&empty, &by_kind, &registrations, &[], &edges, &[], "hub");
        assert!(out.contains("Reachability that dies with it"), "{out}");
        assert!(
            out.contains("`leaf` (doc) — its own registration is dead"),
            "{out}"
        );

        // Removing `leaf` orphans nobody — it imports nothing.
        let out = impact(&empty, &by_kind, &registrations, &[], &edges, &[], "leaf");
        assert!(
            out.contains("Reachability that dies with it: none"),
            "{out}"
        );
    }

    #[test]
    fn an_unknown_member_is_a_clean_read() {
        // A name no member bears is not an error — `impact` names it absent and the
        // caller still exits zero (the read family is never a gate).
        let empty = BTreeMap::new();
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::new();
        let registrations = BTreeMap::new();
        let out = impact(&empty, &by_kind, &registrations, &[], &[], &[], "ghost");
        assert!(
            out.contains("No member named `ghost` is in the surface"),
            "{out}"
        );
    }

    /// A nested-member-bearing member for the leaf-grain proofs — one `decision`
    /// member with a `chosen` prose leaf, the serialized shape `impact` reads at leaf
    /// grain. The e2e drives carry nested members only through a custom kind, so the
    /// leaf strand is unit-proven here beside the directive/reachability strands.
    fn nested_member(id: &str) -> Features {
        let mut features = feature(id, &[], Some("d"));
        features.nested_members = vec![EmbeddedMember {
            kind: "decision".to_string(),
            key: "surface-authority".to_string(),
            leaves: BTreeMap::from([(
                "chosen".to_string(),
                "the surface is canonical".to_string(),
            )]),
            members: Vec::new(),
        }];
        features
    }

    #[test]
    fn a_leaf_address_reports_citations_separately_from_fallout() {
        // `impact` on a leaf address resolves the leaf against the serialized
        // nested-member leaves, reports the citing one-way edge under its own heading
        // (never fallout), and states the leaf is obligation-free — deleting or
        // rewording it is never blocked.
        let empty = BTreeMap::new();
        let registrations = BTreeMap::new();
        let members = [nested_member("20-surface")];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
        let citations = [Citation {
            from_kind: "spec".to_string(),
            from: "45-governance".to_string(),
            target: MemberAddress {
                member: "20-surface".to_string(),
                kind: "decision".to_string(),
                key: "surface-authority".to_string(),
                child_path: "chosen".to_string(),
            },
        }];

        let out = impact(
            &empty,
            &by_kind,
            &registrations,
            &[],
            &[],
            &citations,
            "20-surface/decision/surface-authority/chosen",
        );

        // Resolved against the lock and reported at leaf grain.
        assert!(
            out.contains("Leaf `20-surface/decision/surface-authority/chosen` (spec)"),
            "{out}"
        );
        assert!(
            out.contains("Authored value: \"the surface is canonical\""),
            "{out}"
        );
        // Citations precede — and are distinct from — fallout.
        let citations_at = out.find("Citations (").expect("a citations heading");
        let fallout_at = out.find("Fallout:").expect("a fallout heading");
        assert!(
            citations_at < fallout_at,
            "citations are reported separately from fallout: {out}"
        );
        assert!(out.contains("`45-governance` (spec) cites it"), "{out}");
        // Obligation-free: the leaf carries no gating fallout and a rewrite is never blocked.
        assert!(out.contains("Fallout: none"), "{out}");
        assert!(out.contains("never blocked by its citations"), "{out}");
    }

    #[test]
    fn a_leaf_with_no_citation_names_zero_citers() {
        // Absent any citing edge, the leaf still resolves and reports — the citations
        // heading names none, the floor's standing state (floor leaves carry no mentions).
        let empty = BTreeMap::new();
        let registrations = BTreeMap::new();
        let members = [nested_member("20-surface")];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);

        let out = impact(
            &empty,
            &by_kind,
            &registrations,
            &[],
            &[],
            &[],
            "20-surface/decision/surface-authority/chosen",
        );
        assert!(out.contains("none — no member cites it"), "{out}");
        assert!(out.contains("Fallout: none"), "{out}");
    }

    #[test]
    fn an_unresolved_or_malformed_leaf_address_is_a_clean_read() {
        // Both an address naming no live leaf and an ill-formed one are reads, not gates —
        // narrated plainly so the caller still exits zero.
        let empty = BTreeMap::new();
        let registrations = BTreeMap::new();
        let members = [nested_member("20-surface")];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);

        let missing = impact(
            &empty,
            &by_kind,
            &registrations,
            &[],
            &[],
            &[],
            "20-surface/decision/surface-authority/rejected",
        );
        assert!(
            missing.contains("No leaf `20-surface/decision/surface-authority/rejected`"),
            "{missing}"
        );

        let malformed = impact(
            &empty,
            &by_kind,
            &registrations,
            &[],
            &[],
            &[],
            "20-surface/decision",
        );
        assert!(
            malformed.contains("is not a well-formed leaf address"),
            "{malformed}"
        );
    }
}
