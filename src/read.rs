//! The read family — one CLI verb, [`explain`], over four traversals of the
//! requirement↔`satisfies` edge and the graph `check` already carries
//! (`specs/architecture/20-surface.md`, "Decision: one read verb — `explain`").
//!
//! [`explain`] resolves its one positional target across three namespaces — member,
//! requirement, leaf-grain address (`(explain-target-disambiguation)`, ruled
//! 2026-07-04) — and dispatches to whichever of the four traversals below answer that
//! species: [`why`] walks the edge **forward** (this member → the requirements it
//! fills, with their authored rationale → the package its kind binds → its resolved
//! edges in and out); [`requirements`] walks it in **reverse** (the roster → each
//! requirement's satisfier set + coverage state, and with a name the blast radius a
//! removal would strand); [`impact`] narrates the **blast radius of a removal** — what
//! strands if a member is removed or renamed: the requirements it is the sole satisfier
//! of (left unfilled), the `satisfies` links onto demands it alone publishes (left
//! dangling), the `@import` directive edges that point at it (left unbacked), and the
//! members whose reachability was carried only through it (gone dead) — or, at leaf
//! grain, a leaf's citations reported separately from its (nonexistent) fallout;
//! [`context`] emits the **declared neighborhood** — a member's genre slots or a leaf's
//! siblings, the citers, and the requirements satisfied. All are *projections* over the
//! data `check` already computes — the opt-in `satisfies` bindings [`crate::coverage`]
//! gates, and, for the edge walk, the **gate's own resolved edge set**
//! ([`crate::graph::resolved_edges`], relationships over extracted features), never a
//! private re-derivation off the `[edge.<target>]` document clauses (READ-EDGE-UNIFY:
//! one source of truth, so `why`'s edge narration cannot disagree with `graph::check`).
//! None adds engine semantics and none ever gates: `explain` returns narration, and
//! `main` prints it and exits zero on every input, ambiguous or unknown targets included
//! (the read family is not the gate; a reporting verb whose exit code CI trusts is
//! exactly what the Decision rejects).
//!
//! The output is a **teaching surface**, not a table dump (`specs/architecture/50-distribution.md`,
//! "the gate teaches"): full sentences over the author's own artifacts, in the
//! corpus's vocabulary. The narration is derived, never persisted.
//!
//! ## Scope: every opt-in kind, built-in and custom
//!
//! This tier reads the members [`Workspace`] carries — the built-in opt-in kinds
//! (skill ⊕ rule) — **and** the custom-kind members the caller threads in as
//! [`CustomMember`]s (READ-CUSTOM-SATISFIERS): temper's own `spec`s, or any consumer's
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

use crate::builtin;
use crate::check::Workspace;
use crate::compose::{Edge, Requirement};
use crate::document::Satisfies;
use crate::extract::{Features, LeafAddress};
use crate::graph::{self, ResolvedEdge};
use crate::kind::Activation;

/// A member as the read family sees it: its kind, its id, and the requirements it opts
/// into filling (each with its authored rationale). Built off the typed [`Workspace`]
/// artifacts — the `satisfies` the surface language authors on each member document
/// (`specs/architecture/20-surface.md`, "The member document"), which the decidable
/// [`crate::extract::Features`] view drops the rationale from but the read family needs
/// whole. Edges are **not** carried here: `why` narrates the gate's resolved edge set
/// ([`crate::graph::resolved_edges`]) keyed on the member's `(kind, id)` node, never the
/// `[edge.<target>]` document clauses (READ-EDGE-UNIFY).
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
/// threads these in — the [`Workspace`] holds only the built-in kinds, so a custom
/// member's satisfiers (loaded off [`crate::kind::Unit::satisfies_clauses`]) reach the
/// read family here rather than through the workspace. Kept whole with rationale,
/// which the decidable [`Features`] view drops.
pub struct CustomMember {
    /// The custom kind's registered name (`spec`, `adr`, …) — the edge node's kind
    /// and what the narration prints.
    pub kind: String,
    /// The member id (its surface directory name).
    pub id: String,
    /// The rationale-preserving `satisfies` clauses this member authors.
    pub satisfies: Vec<Satisfies>,
}

/// A declared **citation** — a one-way edge from a member (the citer) to a genre-value
/// [`LeafAddress`] it names (`specs/architecture/45-governance.md`, "the mention is the readmitted
/// one-way annotation class"; address grain). **Obligation-free**: the obligation graph
/// ignores it, coverage never counts it, and `impact` reports it as a *citation, never
/// fallout* — deleting or rewording the cited leaf is never blocked, the citer is told.
/// **Resolution-checked** against the manifest's serialized genre values: `impact` reports
/// a citation only for a leaf that resolves, exactly the referential guarantee a mention
/// carries.
///
/// The floor carries no producer yet — floor leaves carry no mentions (interpolation stays
/// an altitude feature, `specs/architecture/20-surface.md`, "Genre values"), so today's caller
/// threads an empty set and the leaf-grain report names zero citers. The reporting shape is
/// ready for the altitude's serialized mentions; the mechanism is proven in unit tests here.
pub struct Citation {
    /// The kind of the citer — part of its node identity, and what the narration prints.
    pub from_kind: String,
    /// The member id that declares the citation (the citer).
    pub from: String,
    /// The leaf address the citation targets — resolved against the serialized genre
    /// values before it is reported.
    pub target: LeafAddress,
}

/// Project every opt-in artifact into the read family's [`Member`] view — the
/// [`Workspace`]'s built-in kinds (skills, then rules) followed by the caller-threaded
/// custom-kind members, each group name-sorted by its load, so every traversal below
/// is deterministic without a re-sort (READ-CUSTOM-SATISFIERS).
fn members(workspace: &Workspace, custom: &[CustomMember]) -> Vec<Member> {
    let mut members = Vec::new();
    for skill in workspace.skills() {
        members.push(Member {
            kind: "skill".to_string(),
            id: skill.id.clone(),
            satisfies: skill.satisfies.clone(),
        });
    }
    for rule in workspace.rules() {
        members.push(Member {
            kind: "rule".to_string(),
            id: rule.id.clone(),
            satisfies: rule.satisfies.clone(),
        });
    }
    for member in custom {
        members.push(Member {
            kind: member.kind.clone(),
            id: member.id.clone(),
            satisfies: member.satisfies.clone(),
        });
    }
    members
}

/// The floor the `kind`'s members are checked against — the kind's real built-in
/// floor resolved by its bare row label through [`builtin::floor_package`]: `skill` →
/// `skill.anthropic`, `rule` → `rule.anthropic`, `memory` → `memory.anthropic`, so a
/// `memory` member is bound to its own floor rather than mis-narrated as
/// `skill.anthropic`. A kind that genuinely ships no floor (a custom kind with no
/// binding) falls back to its own name (`specs/architecture/40-composition.md`,
/// "Binding is implicit — a floor is a clause array": there is no package-to-kind
/// table, so an unbound kind is named for itself rather than a fabricated default).
fn bound_package(kind: &str) -> String {
    builtin::floor_package(kind).map_or_else(|| kind.to_string(), str::to_string)
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
    /// A leaf address (`<member>/<genre>/<key>/<field-path>`) — dispatches to
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

/// `temper explain <target>` — the one read verb (`specs/architecture/20-surface.md`, "Decision:
/// one read verb — `explain`"): resolve `target`'s [`Species`] and dispatch to whichever
/// of the four traversals answer it, so the single verb answers every read question
/// `why`/`requirements`/`impact`/`context` used to split across four CLI spellings. A
/// read, never a gate — the caller prints this and exits zero on every input, an
/// ambiguous or unrecognized target included.
///
/// `assembly` and `roster` mirror [`impact`]'s own split (the assembly's own
/// `[requirement.*]` roster vs. the composed namespace `check` gates); `edges` is the
/// declared relationship set [`why`]'s edge walk resolves; `activations`,
/// `repo_files`, and `directive_edges` are the exact reachability/directive inputs
/// [`impact`]'s blast radius ranges over; `citations` are the declared one-way edges a
/// leaf-grain answer reports separately from fallout. Every one is the identical input
/// the gate's own predicates range over (READ-EDGE-UNIFY), so `explain` cannot disagree
/// with a green `check`.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn explain(
    workspace: &Workspace,
    custom: &[CustomMember],
    assembly: &BTreeMap<String, Requirement>,
    roster: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    edges: &[Edge],
    activations: &BTreeMap<&str, Activation>,
    repo_files: &[String],
    directive_edges: &[ResolvedEdge],
    citations: &[Citation],
    target: &str,
) -> String {
    match resolve(by_kind, roster, target) {
        Species::Member(name) => {
            let mut out = why(workspace, custom, roster, by_kind, edges, name);
            out.push('\n');
            out.push_str(&impact(
                assembly,
                roster,
                by_kind,
                activations,
                repo_files,
                directive_edges,
                citations,
                name,
            ));
            out.push('\n');
            out.push_str(&context(by_kind, citations, name));
            out
        }
        Species::Requirement(name) => requirements(workspace, custom, roster, Some(name)),
        Species::Leaf(address) => {
            let mut out = impact(
                assembly,
                roster,
                by_kind,
                activations,
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
             leaf-grain addresses (`<member>/<genre>/<key>/<field-path>`); check the \
             name, or `import` the harness first.\n"
        ),
    }
}

/// `temper why <member>` — narrate everything that holds `member` in place: the
/// requirements it `satisfies` (each with its authored rationale and the requirement's
/// own `means`), the package its kind binds, and its resolved edges in and out
/// (`specs/architecture/20-surface.md`, "Decision: the CLI gains a read family"). A read, never a
/// gate — the caller prints this and exits zero on every input, including a name no
/// member bears.
///
/// The edge walk ranges over the **gate's own resolved edge set** — `by_kind` (the
/// by-kind [`Features`] corpus) and `edges` (the declared `[[kind.<name>.relationships]]`
/// set) are the *same* two the `check` arm builds, and `why` runs them through the
/// identical [`graph::resolved_edges`] the gate's `check`/`acyclic`/`degree` range over.
/// So `why`'s edge narration cannot disagree with the gate (READ-EDGE-UNIFY): a
/// `routes_to` edge the gate resolves is the exact edge `why` narrates, and a member
/// with no resolved edge stays silent.
///
/// The `roster` is the **composed** requirement namespace `check` gates — the assembly
/// `[requirement.*]` unioned with every member's published `[requirement.*]`
/// (`specs/architecture/10-contracts.md`, "a requirement's publisher is any authored surface
/// document"; built by the caller through the gate's own `union_published_requirements`,
/// READ-VERBS-PUBLISHED-DEMANDS). Ranging over it — not the assembly roster alone — is
/// why a `satisfies` link to a member-published demand narrates as filled, matching a
/// green `check` rather than misreporting the join as dangling.
#[must_use]
pub fn why(
    workspace: &Workspace,
    custom: &[CustomMember],
    roster: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    edges: &[Edge],
    member: &str,
) -> String {
    let members = members(workspace, custom);
    // The resolved edge set the gate ranges over — computed once, filtered per matched
    // node below. One source of truth: the exact arcs `graph::check` resolves.
    let resolved = graph::resolved_edges(edges, by_kind);

    let matches: Vec<&Member> = members.iter().filter(|m| m.id == member).collect();
    if matches.is_empty() {
        return format!(
            "No member named `{member}` is in the surface. `why` reads the authored \
             surface's members — skills, rules, and every custom kind's members; check \
             the name, or `import` the harness first.\n"
        );
    }

    let mut out = String::new();
    for (index, member) in matches.iter().enumerate() {
        // A blank line between multiple same-named members (a skill and a rule may
        // share a name), each narrated in full under its own kind.
        if index > 0 {
            out.push('\n');
        }
        why_one(&mut out, member, roster, &resolved);
    }
    out
}

/// Narrate one matched member into `out` — the full forward walk for a single
/// `(kind, id)` node.
fn why_one(
    out: &mut String,
    member: &Member,
    roster: &BTreeMap<String, Requirement>,
    resolved: &[ResolvedEdge],
) {
    let _ = writeln!(
        out,
        "Member `{}` ({}) — everything that holds it in place:\n",
        member.id, member.kind
    );

    // Forward walk: the requirements this member fills, each with its authored
    // rationale (the *why*, law 7) and the requirement's own `means`.
    if member.satisfies.is_empty() {
        let _ = writeln!(
            out,
            "It fills no requirements — it opts into no `satisfies` link, so it is \
             governed by its kind's floor alone.\n"
        );
    } else {
        let _ = writeln!(out, "Requirements it satisfies:");
        for satisfies in &member.satisfies {
            narrate_filled(out, satisfies, roster);
        }
        out.push('\n');
    }

    // The floor the member's kind binds — the governing contract its conformance is
    // checked against.
    let _ = writeln!(
        out,
        "Governing floor: its `{}` kind binds the `{}` floor, whose clauses check it.\n",
        member.kind,
        bound_package(&member.kind),
    );

    // The edges in and out — the member's node in the **gate's resolved edge set**
    // (`crate::graph::resolved_edges`), not a re-derivation off the `[edge.*]` document
    // clauses (READ-EDGE-UNIFY). A dangling reference resolves to no node, so it appears
    // in neither list — route resolution is the gate's finding to report, not `why`'s.
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
/// its authored rationale, and — resolving the link — the requirement's own `means`
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
            if let Some(means) = &requirement.means {
                let _ = writeln!(out, "      It means: \"{means}\".");
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

/// `temper impact <member>` — narrate the deterministic **blast radius** of removing or
/// renaming `member` (`specs/architecture/20-surface.md`, the `impact` CLI bullet): the graph
/// payoff `00-intent.md` promises, given a verb. Four strands, each read off the graph
/// data `check` already carries — no second build, no new engine semantics:
///
/// 1. **Requirements left unfilled** — a requirement `member` satisfies whose *only*
///    satisfier is `member`, so removing it drops coverage to zero (an error for a
///    `required` one, silent for an advisory).
/// 2. **`satisfies` left dangling** — a requirement `member` alone **publishes**
///    (`specs/architecture/10-contracts.md`, a publisher); removing its publisher drops the demand
///    from the namespace, so every *other* member's `satisfies` onto it dangles.
/// 3. **Directive edges left unbacked** — an `@import` from another member that resolves
///    to `member`'s file (`specs/architecture/15-kinds.md`, "Directives"); removing the file leaves
///    that import backing nothing, the silent-context-loss class made author-time.
/// 4. **Reachability that dies with it** — a member live now only because `member`
///    imports it (its own activation dead); removing `member` unreaches it
///    ([`graph::reachability_orphaned`], the same closure the gate's `reachable` runs).
///
/// The family gains **leaf grain** (`specs/architecture/20-surface.md`, "The family gains leaf
/// grain"): a `target` naming a genre-value leaf — the `<member>/<genre>/<key>/<field-path>`
/// address — dispatches to [`impact_leaf`], which resolves the leaf against the manifest's
/// serialized genre values and reports its **citations separately from fallout** (a leaf is
/// obligation-free; `specs/architecture/45-governance.md`, address grain). A `target` with no `/` is
/// a bare member name and takes the member-grain path below, unchanged.
///
/// A read, never a gate: the caller prints this and exits zero on every input, a name no
/// member or leaf bears included. `assembly` is the assembly's own `[requirement.*]` roster
/// (to tell a demand `member` alone publishes from one the assembly also carries); `roster`
/// is the **composed** namespace `check` gates; `by_kind`, `activations`, `repo_files`,
/// and `directive_edges` are the exact graph inputs the gate's predicates range over
/// (READ-EDGE-UNIFY), so the read cannot disagree with a green `check`. `by_kind` also
/// carries each member's serialized genre values, the leaf-grain surface; `citations` are
/// the declared one-way edges naming a leaf.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn impact(
    assembly: &BTreeMap<String, Requirement>,
    roster: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    activations: &BTreeMap<&str, Activation>,
    repo_files: &[String],
    directive_edges: &[ResolvedEdge],
    citations: &[Citation],
    target: &str,
) -> String {
    // A `/`-bearing target is a leaf address (member ids never carry a slash), so it
    // dispatches to leaf grain; a bare name stays the member-grain blast radius below.
    if target.contains('/') {
        return impact_leaf(by_kind, citations, target);
    }

    // Every `(kind, id)` node bearing the name — a skill and a rule may share one, each
    // with its own blast radius. Sorted, since `by_kind` is a `BTreeMap` over name-sorted
    // slices.
    let matches: Vec<(&str, &Features)> = by_kind
        .iter()
        .flat_map(|(&kind, members)| members.iter().map(move |features| (kind, features)))
        .filter(|(_, features)| features.id == target)
        .collect();

    if matches.is_empty() {
        return format!(
            "No member named `{target}` is in the surface. `impact` reads the authored \
             surface's members — skills, rules, and every custom kind's members; check \
             the name, or `import` the harness first.\n"
        );
    }

    let mut out = String::new();
    for (index, (kind, features)) in matches.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        impact_one(
            &mut out,
            kind,
            features,
            assembly,
            roster,
            by_kind,
            activations,
            repo_files,
            directive_edges,
        );
    }
    out
}

/// A parsed **leaf address** — the `<member>/<genre>/<key>/<field-path>` spelling `impact`
/// accepts to name a single genre-value leaf (`specs/architecture/20-surface.md`, "leaf addresses
/// are structural and keyed"). The three identity segments are `/`-separated; the field
/// path keeps its own dots (`rejected.baked-projection.because`), so it is the whole
/// remainder after the third slash — `splitn(4, '/')`, never a plain split that would
/// mangle a dotted collection path.
struct ParsedLeaf<'a> {
    member: &'a str,
    genre: &'a str,
    key: &'a str,
    field_path: &'a str,
}

/// Parse a `/`-bearing `target` into its four leaf-address segments, or `None` when a
/// segment is empty (a malformed address the caller reports as such). Keyed and structural
/// — the address rides the shape the author already wrote, stable under content edits.
fn parse_leaf_address(target: &str) -> Option<ParsedLeaf<'_>> {
    let mut parts = target.splitn(4, '/');
    let member = parts.next()?;
    let genre = parts.next()?;
    let key = parts.next()?;
    let field_path = parts.next()?;
    if member.is_empty() || genre.is_empty() || key.is_empty() || field_path.is_empty() {
        return None;
    }
    Some(ParsedLeaf {
        member,
        genre,
        key,
        field_path,
    })
}

/// Resolve a parsed leaf address against the manifest's **serialized genre values**
/// ([`Features::genre_leaves`]) — the tier-1, offline read the leaf-grain `impact` stands
/// on. Returns the matched leaf's kind and authored value, or `None` when no member's genre
/// value carries that `(genre, key, field-path)`. Ranges over every kind's members, since a
/// leaf may live in any genre-bearing kind.
fn resolve_leaf<'a>(
    by_kind: &BTreeMap<&str, &'a [Features]>,
    parsed: &ParsedLeaf<'_>,
) -> Option<(String, &'a str)> {
    for (&kind, members) in by_kind {
        for features in *members {
            if features.id != parsed.member {
                continue;
            }
            for (address, value) in features.genre_leaves() {
                if address.genre == parsed.genre
                    && address.key == parsed.key
                    && address.field_path == parsed.field_path
                {
                    return Some((kind.to_string(), value));
                }
            }
        }
    }
    None
}

/// `temper impact <leaf-address>` — narrate a genre-value leaf at **leaf grain**
/// (`specs/architecture/20-surface.md`, "The family gains leaf grain"): resolve the leaf against
/// the manifest's serialized genre values, then report its **citations separately from
/// fallout** (`specs/architecture/45-governance.md`, address grain). A leaf is obligation-free — its
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
             `<member>/<genre>/<key>/<field-path>` — the member, the genre value's genre and \
             key, and the field path within it (`chosen`, or `rejected.baked-projection.because`).\n"
        );
    };

    let Some((kind, value)) = resolve_leaf(by_kind, &parsed) else {
        return format!(
            "No leaf `{target}` is in the surface's serialized genre values. `impact` reads \
             leaf grain off the manifest's `[[member.genre]]` tables — check the member, \
             genre, key, and field path; a document carrying no genre values is not \
             represented at leaf grain.\n"
        );
    };

    let ParsedLeaf {
        member,
        genre,
        key,
        field_path,
    } = parsed;

    let mut out = String::new();
    let _ = writeln!(out, "Leaf `{target}` ({kind}) — leaf grain:\n");
    let _ = writeln!(
        out,
        "It is the `{field_path}` leaf of the `{genre}` value `{key}` in member `{member}`.",
    );
    let _ = writeln!(out, "Authored value: \"{value}\"\n");

    // Citations — the declared one-way edges naming this leaf, resolution-checked (we only
    // reach here for a leaf that resolves) and obligation-free. Reported *separately* from
    // fallout: a citation is never fallout (`specs/architecture/45-governance.md`). The shared
    // helper is reused by `context` at the same grain.
    narrate_citers(&mut out, citations, member, genre, key, field_path);
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

    // Coverage — the leaf-grain answer names what it cannot see (`specs/architecture/20-surface.md`,
    // "both disclose coverage"): the disclosure ships WITH the found answer, not only in the
    // not-found error, so an incomplete answer never wears complete clothes (law 1).
    disclose_coverage(&mut out, by_kind);

    out
}

/// Narrate the **citations** naming a leaf — the declared one-way edges (obligation-free,
/// resolution-checked) both `impact` and `context` report at leaf grain
/// (`specs/architecture/45-governance.md`, address grain). Shared so the two verbs cannot disagree on
/// what cites a leaf, exactly as the edge walk shares the gate's resolved set (READ-EDGE-UNIFY).
fn narrate_citers(
    out: &mut String,
    citations: &[Citation],
    member: &str,
    genre: &str,
    key: &str,
    field_path: &str,
) {
    let citers: Vec<&Citation> = citations
        .iter()
        .filter(|citation| {
            citation.target.member == member
                && citation.target.genre == genre
                && citation.target.key == key
                && citation.target.field_path == field_path
        })
        .collect();
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

/// Append the shared **coverage disclosure** every leaf-grain answer closes with
/// (`specs/architecture/20-surface.md`, "both disclose coverage … every leaf-grain answer names what
/// it cannot see"): the count of members carrying no serialized genre leaves — the documents that
/// carry no genre values a leaf-grain read cannot represent. Under the gradient a mixed-posture
/// corpus is the standing state, not an edge case, so an answer hiding its blind spot erodes the verb
/// exactly as a false gate-block erodes the gate (law 1). Shared by `impact`'s and `context`'s
/// leaf-grain answers, so the disclosure is one wording that ships WITH the verb, never after.
fn disclose_coverage(out: &mut String, by_kind: &BTreeMap<&str, &[Features]>) {
    let leafless = by_kind
        .values()
        .flat_map(|members| members.iter())
        .filter(|features| features.genre_leaves().is_empty())
        .count();
    let (noun, verb) = if leafless == 1 {
        ("document", "carries")
    } else {
        ("documents", "carry")
    };
    let _ = writeln!(
        out,
        "Coverage: {leafless} {noun} {verb} no genre values — not represented at leaf grain \
         (carrying no serialized genre leaves)."
    );
}

/// `temper context <address>` — emit the **declared neighborhood** of a member or a genre-value
/// leaf (`specs/architecture/20-surface.md`, "`context <address>` emits a member's or leaf's declared
/// neighborhood"): its genre slot, its siblings, the members that cite it, and the requirements its
/// member satisfies — the pre-edit context bundle for the primary author. Consumes only the
/// manifest's serialized genre values (`by_kind`) and declared citations: offline, tier-1, no
/// runtime.
///
/// A `/`-bearing `address` is a leaf (`<member>/<genre>/<key>/<field-path>`) reported at leaf grain
/// ([`context_leaf`]); a bare name is a member reported whole ([`context_member`]). Both are
/// leaf-grain answers, so both close with the shared [`disclose_coverage`] — a mixed-posture corpus
/// is the standing state, and an answer hiding what it cannot see erodes the verb (law 1).
///
/// A read, never a gate: an unresolved or ill-formed address is narrated plainly and the caller
/// still exits zero.
#[must_use]
pub fn context(
    by_kind: &BTreeMap<&str, &[Features]>,
    citations: &[Citation],
    address: &str,
) -> String {
    if address.contains('/') {
        context_leaf(by_kind, citations, address)
    } else {
        context_member(by_kind, citations, address)
    }
}

/// Narrate a genre-value leaf's neighborhood: its genre slot and authored value, its **siblings**
/// (the other leaves of the same genre value), the members that **cite** it, and the requirements
/// its member **satisfies** — then the shared coverage disclosure. Resolved against the manifest's
/// serialized genre values exactly as [`impact_leaf`] resolves them, so the two verbs agree on what
/// a leaf's neighborhood is.
fn context_leaf(
    by_kind: &BTreeMap<&str, &[Features]>,
    citations: &[Citation],
    address: &str,
) -> String {
    let Some(parsed) = parse_leaf_address(address) else {
        return format!(
            "`{address}` is not a well-formed leaf address. A leaf address is \
             `<member>/<genre>/<key>/<field-path>` — the member, the genre value's genre and \
             key, and the field path within it (`chosen`, or `rejected.baked-projection.because`).\n"
        );
    };

    let Some((kind, value)) = resolve_leaf(by_kind, &parsed) else {
        return format!(
            "No leaf `{address}` is in the surface's serialized genre values. `context` reads \
             leaf grain off the manifest's `[[member.genre]]` tables — check the member, genre, \
             key, and field path; a document carrying no genre values is not represented at leaf \
             grain.\n"
        );
    };

    let ParsedLeaf {
        member,
        genre,
        key,
        field_path,
    } = parsed;

    let mut out = String::new();
    let _ = writeln!(
        out,
        "Leaf `{address}` ({kind}) — its declared neighborhood:\n"
    );
    let _ = writeln!(
        out,
        "Genre slot: the `{field_path}` leaf of the `{genre}` value `{key}` in member `{member}`."
    );
    let _ = writeln!(out, "Authored value: \"{value}\"\n");

    // Siblings — the other leaves of the same genre value (same member, genre, key), the
    // co-resident context an author editing this leaf wants beside it.
    let siblings = sibling_leaves(by_kind, member, genre, key, field_path);
    if siblings.is_empty() {
        let _ = writeln!(
            out,
            "Siblings: none — it is the only leaf of the `{genre}` value `{key}`."
        );
    } else {
        let _ = writeln!(
            out,
            "Siblings (the other leaves of the `{genre}` value `{key}`):"
        );
        for (sibling_path, sibling_value) in siblings {
            let _ = writeln!(out, "  • `{sibling_path}` — \"{sibling_value}\"");
        }
    }
    out.push('\n');

    // Citers — the same declared one-way edges `impact` reports at leaf grain (shared helper).
    narrate_citers(&mut out, citations, member, genre, key, field_path);
    out.push('\n');

    // Satisfied requirements — the demands the member the leaf lives in opts into filling.
    narrate_satisfied(&mut out, by_kind, member);
    out.push('\n');

    // Coverage — the leaf-grain answer names what it cannot see (law 1), shipping WITH the verb.
    disclose_coverage(&mut out, by_kind);

    out
}

/// Narrate a member's neighborhood: the genre slots it carries (each genre value and its leaf
/// field paths), the members that cite any of its leaves, and the requirements it satisfies — then
/// the shared coverage disclosure. A member name bearing no `/` takes this path; every `(kind, id)`
/// node bearing the name is narrated, each under its own kind.
fn context_member(
    by_kind: &BTreeMap<&str, &[Features]>,
    citations: &[Citation],
    member: &str,
) -> String {
    let matches: Vec<(&str, &Features)> = by_kind
        .iter()
        .flat_map(|(&kind, members)| members.iter().map(move |features| (kind, features)))
        .filter(|(_, features)| features.id == member)
        .collect();

    if matches.is_empty() {
        return format!(
            "No member named `{member}` is in the surface. `context` reads the authored \
             surface's members — skills, rules, and every custom kind's members; check the \
             name, or `import` the harness first.\n"
        );
    }

    let mut out = String::new();
    for (index, (kind, features)) in matches.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        context_member_one(&mut out, kind, features, by_kind, citations);
    }

    // Coverage — a member's neighborhood is a leaf-grain answer too (it enumerates the member's
    // genre slots), so it names what it cannot see, once at the end.
    disclose_coverage(&mut out, by_kind);

    out
}

/// Narrate one matched member's neighborhood into `out` — its genre slots, the citations naming
/// any of its leaves, and the requirements it satisfies.
fn context_member_one(
    out: &mut String,
    kind: &str,
    features: &Features,
    by_kind: &BTreeMap<&str, &[Features]>,
    citations: &[Citation],
) {
    let _ = writeln!(
        out,
        "Member `{}` ({kind}) — its declared neighborhood:\n",
        features.id
    );

    // Genre slots — each genre value the member carries, with its leaf field paths (the leaves
    // an author addresses under this member at leaf grain).
    if features.genres.is_empty() {
        let _ = writeln!(
            out,
            "Genre slots: none — it carries no genre value, so it holds no leaf at leaf grain."
        );
    } else {
        let _ = writeln!(out, "Genre slots (the genre values it carries):");
        for value in &features.genres {
            let fields: Vec<String> = value
                .addressed_leaves()
                .into_iter()
                .map(|(field_path, _)| field_path)
                .collect();
            let _ = writeln!(
                out,
                "  • `{}` value `{}` — leaves: {}",
                value.genre,
                value.key,
                fields.join(", ")
            );
        }
    }
    out.push('\n');

    // Citers — every declared one-way edge naming a leaf in this member.
    let citers: Vec<&Citation> = citations
        .iter()
        .filter(|citation| citation.target.member == features.id)
        .collect();
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
                target.genre,
                target.key,
                target.field_path
            );
        }
    }
    out.push('\n');

    // Satisfied requirements — the demands this member opts into filling.
    narrate_satisfied(out, by_kind, &features.id);
}

/// The other leaves of the genre value `(member, genre, key)` — every serialized leaf of that
/// value except the one at `field_path`, paired with its authored value in the manifest's stable
/// order. The co-resident siblings `context` reports beside an addressed leaf.
fn sibling_leaves<'a>(
    by_kind: &BTreeMap<&str, &'a [Features]>,
    member: &str,
    genre: &str,
    key: &str,
    field_path: &str,
) -> Vec<(String, &'a str)> {
    let mut out = Vec::new();
    for members in by_kind.values() {
        for features in *members {
            if features.id != member {
                continue;
            }
            for (address, value) in features.genre_leaves() {
                if address.genre == genre && address.key == key && address.field_path != field_path
                {
                    out.push((address.field_path, value));
                }
            }
        }
    }
    out
}

/// Narrate the requirements the member `member` opts into filling — read off each matching
/// member's serialized `satisfies` (`Features::satisfies`), the manifest-only view the leaf-grain
/// read stands on. A member fills only the demands it names, so an empty set is stated plainly.
fn narrate_satisfied(out: &mut String, by_kind: &BTreeMap<&str, &[Features]>, member: &str) {
    let satisfied: Vec<&str> = by_kind
        .values()
        .flat_map(|members| members.iter())
        .filter(|features| features.id == member)
        .flat_map(|features| features.satisfies.iter().map(String::as_str))
        .collect();
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

/// Narrate one matched node's blast radius into `out` — the four strands for a single
/// `(kind, id)`.
#[allow(clippy::too_many_arguments)]
fn impact_one(
    out: &mut String,
    kind: &str,
    features: &Features,
    assembly: &BTreeMap<String, Requirement>,
    roster: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    activations: &BTreeMap<&str, Activation>,
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

    // (2) Demands it alone publishes — removing its publisher strands every other
    // member's `satisfies` onto them.
    let mut dangling: Vec<(String, Vec<(String, String)>)> = Vec::new();
    for published in &features.published_requirements {
        // Another publisher (the assembly, or a second member) keeps the demand in the
        // namespace, so removing this one strands nothing.
        if sole_publisher(&published.name, kind, &features.id, assembly, by_kind) {
            let stranded = other_satisfiers(by_kind, &published.name, kind, &features.id);
            dangling.push((published.name.clone(), stranded));
        }
    }
    if dangling.is_empty() {
        let _ = writeln!(
            out,
            "`satisfies` left dangling: none — it publishes no requirement that another \
             member fills and no other surface publishes."
        );
    } else {
        let _ = writeln!(
            out,
            "`satisfies` left dangling (it alone publishes these demands, so removing it \
             leaves each opt-in resolving to nothing):"
        );
        for (name, stranded) in &dangling {
            if stranded.is_empty() {
                let _ = writeln!(
                    out,
                    "  • `{name}` — no member fills it today, so nothing dangles yet, but the \
                     demand leaves the namespace with `{}`.",
                    features.id
                );
            } else {
                for (satisfier_kind, satisfier_id) in stranded {
                    let _ = writeln!(
                        out,
                        "  • `{satisfier_id}` ({satisfier_kind}) fills `{name}`, which only \
                         `{}` publishes — its `satisfies` link would dangle.",
                        features.id
                    );
                }
            }
        }
    }
    out.push('\n');

    // (3) `@import` directive edges that point at this member's file — removing the file
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
                DIRECTIVE_FIELD_LABEL
            );
        }
    }
    out.push('\n');

    // (4) Members reachable now only because this one carried their liveness across an
    // import — removing it unreaches them.
    let orphaned =
        graph::reachability_orphaned(&node, activations, by_kind, repo_files, directive_edges);
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
                "  • `{orphan_id}` ({orphan_kind}) — its own activation is dead, and removing \
                 `{}` leaves no live importer to reach it.",
                features.id
            );
        }
    }
}

/// The `@import` syntax label a directive-produced edge is narrated under — the mirror of
/// `graph`'s private `DIRECTIVE_FIELD` (`at-import`), so `impact`'s narration names the
/// edge the same way the gate records it.
const DIRECTIVE_FIELD_LABEL: &str = "at-import";

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

/// Whether the member `(kind, id)` is the **only** publisher of the demand `name` —
/// no assembly `[requirement.<name>]` and no other member publishing it. When true,
/// removing the member drops `name` from the namespace and strands its satisfiers; when
/// false, another surface keeps the demand alive.
fn sole_publisher(
    name: &str,
    kind: &str,
    id: &str,
    assembly: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
) -> bool {
    if assembly.contains_key(name) {
        return false;
    }
    !by_kind.iter().any(|(&other_kind, members)| {
        members.iter().any(|features| {
            !(other_kind == kind && features.id == id)
                && features
                    .published_requirements
                    .iter()
                    .any(|published| published.name == name)
        })
    })
}

/// The members that satisfy `name` other than `(kind, id)` — the opt-in links a removal
/// of `name`'s sole publisher would strand, as `(kind, id)` pairs in the corpus's stable
/// order.
fn other_satisfiers(
    by_kind: &BTreeMap<&str, &[Features]>,
    name: &str,
    kind: &str,
    id: &str,
) -> Vec<(String, String)> {
    by_kind
        .iter()
        .flat_map(|(&member_kind, members)| {
            members.iter().map(move |features| (member_kind, features))
        })
        .filter(|(member_kind, features)| {
            !(*member_kind == kind && features.id == id)
                && features.satisfies.iter().any(|req| req == name)
        })
        .map(|(member_kind, features)| (member_kind.to_string(), features.id.clone()))
        .collect()
}

/// `temper requirements [<name>]` — narrate the requirement roster. Without a name it
/// is the forward roster view: each requirement with its satisfier set and coverage
/// state. With a name it is the reverse walk over that one requirement: its satisfiers
/// and the blast radius a removal would strand (`specs/architecture/20-surface.md`, "Decision: the
/// CLI gains a read family"; the traversal payoff of `specs/architecture/30-landscapes.md` law 6).
/// A read, never a gate — the caller prints this and exits zero on every input.
///
/// The `roster` is the **composed** requirement namespace `check` gates (assembly ∪
/// member-published, READ-VERBS-PUBLISHED-DEMANDS), built by the caller through the
/// gate's own union — so `requirements` lists every published obligation, not the
/// assembly's `[requirement.*]` alone.
#[must_use]
pub fn requirements(
    workspace: &Workspace,
    custom: &[CustomMember],
    roster: &BTreeMap<String, Requirement>,
    name: Option<&str>,
) -> String {
    let members = members(workspace, custom);
    match name {
        Some(name) => requirement_detail(&members, roster, name),
        None => roster_overview(&members, roster),
    }
}

/// The forward roster view — every requirement, its satisfier set, and its coverage
/// state, in name order (`specs/architecture/10-contracts.md`, the coverage gate's vocabulary:
/// `required` + unfilled is an error, advisory unfilled never gates).
fn roster_overview(members: &[Member], roster: &BTreeMap<String, Requirement>) -> String {
    if roster.is_empty() {
        return "No requirements are published — the roster is empty. Declare one in \
                the SDK program's `harness()` assembly, or publish one on a member, \
                to name an obligation.\n"
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
        let satisfiers = satisfiers_of(members, &requirement.name);
        let _ = writeln!(
            out,
            "  • `{}` — {}",
            requirement.name,
            coverage_state(requirement.required, satisfiers.len())
        );
        if let Some(means) = &requirement.means {
            let _ = writeln!(out, "      It means: \"{means}\".");
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
/// gate ("removing a load-bearing entity surfaces its blast radius",
/// `specs/architecture/30-landscapes.md` law 6).
fn requirement_detail(
    members: &[Member],
    roster: &BTreeMap<String, Requirement>,
    name: &str,
) -> String {
    let satisfiers = satisfiers_of(members, name);

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
    if let Some(means) = &requirement.means {
        let _ = writeln!(&mut out, "  It means: \"{means}\".");
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
    for (member, satisfies) in &satisfiers {
        let rationale = satisfies.rationale.as_deref().map_or_else(
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
/// opts into it, paired with the opt-in link (for its authored rationale). The same
/// opt-in join [`crate::roster::is_satisfier`] and [`crate::coverage`] use, so the read
/// agrees with the gate. Members arrive name-sorted (skills then rules), so the set is
/// stable across runs.
fn satisfiers_of<'a>(members: &'a [Member], name: &str) -> Vec<(&'a Member, &'a Satisfies)> {
    members
        .iter()
        .filter_map(|member| {
            member
                .satisfies
                .iter()
                .find(|satisfies| satisfies.requirement == name)
                .map(|satisfies| (member, satisfies))
        })
        .collect()
}

/// The coverage-state clause for a requirement given whether it is `required` and how
/// many members satisfy it — the vocabulary the coverage gate reports in
/// (`specs/architecture/10-contracts.md`): a `required` requirement with no satisfier is unfilled,
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

#[cfg(test)]
mod impact_tests {
    //! Unit proofs of the four `impact` strands over hand-built graph inputs — the
    //! directive and reachability strands especially, which need an *importer* kind
    //! (a custom kind composing a `directives` primitive and an activation) the built-in
    //! skill/rule fixtures the e2e drives don't carry. The requirement strands are also
    //! e2e-proven in `tests/read_verbs.rs`.

    use super::*;
    use crate::document::PublishedRequirement;
    use crate::extract::{FeatureValue, GenreValue, Kind};

    /// A member's [`Features`] as `impact` reads them: its id, the requirements it opts
    /// into, the demands it publishes, and its `description` field (a blank one is a dead
    /// description-trigger world-edge). Body-derived features are inert here — `impact`
    /// reads the join, publish, directive, and activation data, all set explicitly.
    fn feature(
        id: &str,
        satisfies: &[&str],
        published: &[&str],
        description: Option<&str>,
    ) -> Features {
        let mut fields = BTreeMap::new();
        if let Some(text) = description {
            fields.insert(
                "description".to_string(),
                FeatureValue::scalar(Kind::String, text),
            );
        }
        Features {
            id: id.to_string(),
            fields,
            body_lines: 1,
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: Some(id.to_string()),
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            genres: Vec::new(),
            satisfies: satisfies.iter().map(|s| (*s).to_string()).collect(),
            published_requirements: published
                .iter()
                .map(|name| PublishedRequirement {
                    name: (*name).to_string(),
                    means: None,
                    kind: None,
                    required: true,
                })
                .collect(),
        }
    }

    /// A `required`/advisory requirement with everything else defaulted — the roster
    /// entry the coverage strand reads.
    fn req(name: &str, required: bool) -> Requirement {
        Requirement {
            name: name.to_string(),
            means: None,
            kind: None,
            required,
            clauses: Vec::new(),
            verified_by: None,
        }
    }

    /// A `(kind, id)` → `(kind, id)` `at-import` directive edge.
    fn directive(from: (&str, &str), to: (&str, &str)) -> ResolvedEdge {
        ResolvedEdge {
            from: (from.0.to_string(), from.1.to_string()),
            field: DIRECTIVE_FIELD_LABEL.to_string(),
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
        let empty = BTreeMap::new();
        let skills = [
            feature("solo", &["r1"], &[], Some("d")),
            feature("pair-a", &["r2"], &[], Some("d")),
            feature("pair-b", &["r2"], &[], Some("d")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let activations = BTreeMap::new();

        let solo = impact(
            &empty,
            &roster,
            &by_kind,
            &activations,
            &[],
            &[],
            &[],
            "solo",
        );
        assert!(
            solo.contains("Requirements left unfilled (it is the only member filling them):"),
            "{solo}"
        );
        assert!(solo.contains("`r1` — required"), "{solo}");
        assert!(solo.contains("fails the gate"), "{solo}");

        let pair = impact(
            &empty,
            &roster,
            &by_kind,
            &activations,
            &[],
            &[],
            &[],
            "pair-a",
        );
        assert!(
            pair.contains("Requirements left unfilled: none"),
            "a non-sole satisfier strands no requirement: {pair}"
        );
    }

    #[test]
    fn removing_a_sole_publisher_dangles_every_satisfying_link() {
        // `publisher` alone publishes `p`, which `filler` satisfies. Removing the
        // publisher drops `p` from the namespace, so `filler`'s `satisfies` dangles.
        let empty_assembly = BTreeMap::new();
        let roster = BTreeMap::from([("p".to_string(), req("p", true))]);
        let skills = [
            feature("publisher", &[], &["p"], Some("d")),
            feature("filler", &["p"], &[], Some("d")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let activations = BTreeMap::new();

        let out = impact(
            &empty_assembly,
            &roster,
            &by_kind,
            &activations,
            &[],
            &[],
            &[],
            "publisher",
        );
        assert!(out.contains("`satisfies` left dangling"), "{out}");
        assert!(
            out.contains("`filler` (skill) fills `p`, which only `publisher` publishes"),
            "{out}"
        );

        // The same demand also declared by the assembly keeps a second publisher, so
        // removing `publisher` strands nothing.
        let assembly = BTreeMap::from([("p".to_string(), req("p", true))]);
        let out = impact(
            &assembly,
            &roster,
            &by_kind,
            &activations,
            &[],
            &[],
            &[],
            "publisher",
        );
        assert!(out.contains("`satisfies` left dangling: none"), "{out}");
    }

    #[test]
    fn removing_an_imported_member_unbacks_the_import() {
        // `hub` `@import`s `leaf`; removing `leaf`'s file leaves that import backing
        // nothing — an unbacked pointer, the silent-context-loss class made author-time.
        let empty = BTreeMap::new();
        let docs = [
            feature("hub", &[], &[], Some("d")),
            feature("leaf", &[], &[], Some("d")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("doc", &docs[..])]);
        let activations = BTreeMap::new();
        let edges = [directive(("doc", "hub"), ("doc", "leaf"))];

        let out = impact(
            &empty,
            &empty,
            &by_kind,
            &activations,
            &[],
            &edges,
            &[],
            "leaf",
        );
        assert!(out.contains("Directive edges left unbacked"), "{out}");
        assert!(
            out.contains("`hub` (doc) imports it via `@at-import`"),
            "{out}"
        );

        // `hub` imports but is not imported, so nothing points *at* it.
        let out = impact(
            &empty,
            &empty,
            &by_kind,
            &activations,
            &[],
            &edges,
            &[],
            "hub",
        );
        assert!(out.contains("Directive edges left unbacked: none"), "{out}");
    }

    #[test]
    fn removing_a_live_importer_unreaches_its_dead_dependent() {
        // `leaf` has a blank `description` — its own description-trigger world-edge is
        // dead — but `hub` (live) `@import`s it, carrying its liveness. Removing `hub`
        // leaves `leaf` with no live importer, so its reachability dies.
        let empty = BTreeMap::new();
        let docs = [
            feature("hub", &[], &[], Some("present")),
            feature("leaf", &[], &[], Some("")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("doc", &docs[..])]);
        let activations = BTreeMap::from([(
            "doc",
            Activation::DescriptionTrigger {
                field: "description".to_string(),
            },
        )]);
        let edges = [directive(("doc", "hub"), ("doc", "leaf"))];

        let out = impact(
            &empty,
            &empty,
            &by_kind,
            &activations,
            &[],
            &edges,
            &[],
            "hub",
        );
        assert!(out.contains("Reachability that dies with it"), "{out}");
        assert!(
            out.contains("`leaf` (doc) — its own activation is dead"),
            "{out}"
        );

        // Removing `leaf` orphans nobody — it imports nothing.
        let out = impact(
            &empty,
            &empty,
            &by_kind,
            &activations,
            &[],
            &edges,
            &[],
            "leaf",
        );
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
        let activations = BTreeMap::new();
        let out = impact(
            &empty,
            &empty,
            &by_kind,
            &activations,
            &[],
            &[],
            &[],
            "ghost",
        );
        assert!(
            out.contains("No member named `ghost` is in the surface"),
            "{out}"
        );
    }

    /// A genre-bearing member for the leaf-grain proofs — one `decision` value with a
    /// `chosen` prose leaf, the serialized shape `impact` reads at leaf grain. The e2e
    /// drives carry genres only through a custom kind, so the leaf strand is unit-proven
    /// here beside the directive/reachability strands.
    fn genre_member(id: &str) -> Features {
        let mut features = feature(id, &[], &[], Some("d"));
        features.genres = vec![GenreValue {
            genre: "decision".to_string(),
            key: "surface-authority".to_string(),
            leaves: BTreeMap::from([(
                "chosen".to_string(),
                "the surface is canonical".to_string(),
            )]),
            collections: BTreeMap::new(),
        }];
        features
    }

    #[test]
    fn a_leaf_address_reports_citations_separately_from_fallout() {
        // `impact` on a leaf address resolves the leaf against the serialized genre values,
        // reports the citing one-way edge under its own heading (never fallout), and states
        // the leaf is obligation-free — deleting or rewording it is never blocked.
        let empty = BTreeMap::new();
        let activations = BTreeMap::new();
        let members = [genre_member("20-surface")];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
        let citations = [Citation {
            from_kind: "spec".to_string(),
            from: "45-governance".to_string(),
            target: LeafAddress {
                member: "20-surface".to_string(),
                genre: "decision".to_string(),
                key: "surface-authority".to_string(),
                field_path: "chosen".to_string(),
            },
        }];

        let out = impact(
            &empty,
            &empty,
            &by_kind,
            &activations,
            &[],
            &[],
            &citations,
            "20-surface/decision/surface-authority/chosen",
        );

        // Resolved against the manifest and reported at leaf grain.
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
        let activations = BTreeMap::new();
        let members = [genre_member("20-surface")];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);

        let out = impact(
            &empty,
            &empty,
            &by_kind,
            &activations,
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
        let activations = BTreeMap::new();
        let members = [genre_member("20-surface")];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);

        let missing = impact(
            &empty,
            &empty,
            &by_kind,
            &activations,
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
            &empty,
            &by_kind,
            &activations,
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
