//! The read family — `why` and `requirements`, two read-only traversals over the
//! requirement↔`satisfies` edge (`specs/architecture/20-surface.md`, "Decision: the CLI gains a
//! read family — `why` and `requirements`").
//!
//! [`why`] walks the edge **forward** (this member → the requirements it fills, with
//! their authored rationale → the package its kind binds → its resolved edges in and
//! out); [`requirements`] walks it in **reverse** (the roster → each requirement's
//! satisfier set + coverage state, and with a name the blast radius a removal would
//! strand). Both are *projections* over the data `check` already computes — the
//! opt-in `satisfies` bindings [`crate::coverage`] gates, and, for the edge walk, the
//! **gate's own resolved edge set** ([`crate::graph::resolved_edges`], relationships
//! over extracted features), never a private re-derivation off the `[edge.<target>]`
//! document clauses (READ-EDGE-UNIFY: one source of truth, so `why`'s edge narration
//! cannot disagree with `graph::check`). Neither adds engine semantics and neither ever
//! gates: they return narration, and `main` prints it and exits zero on every input
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
use crate::compose::{AuthorLayer, Edge, Requirement};
use crate::document::Satisfies;
use crate::extract::Features;
use crate::graph::{self, ResolvedEdge};

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

/// Project every opt-in artifact into the read family's [`Member`] view — the
/// [`Workspace`]'s built-in kinds (skills, then rules) followed by the caller-threaded
/// custom-kind members, each group name-sorted by its load, so every traversal below
/// is deterministic without a re-sort (READ-CUSTOM-SATISFIERS).
fn members(workspace: &Workspace, custom: &[CustomMember]) -> Vec<Member> {
    let mut members = Vec::new();
    for skill in &workspace.skills {
        members.push(Member {
            kind: "skill".to_string(),
            id: skill.id.clone(),
            satisfies: skill.satisfies.clone(),
        });
    }
    for rule in &workspace.rules {
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

/// The requirement roster the read family walks — the `[requirement.<name>]` tables
/// on the author layer, or an empty roster when the harness carries no `temper.toml`.
/// Borrowed from the layer so the name-sorted `BTreeMap` iteration order carries
/// through to the narration.
fn roster(layer: Option<&AuthorLayer>) -> &BTreeMap<String, Requirement> {
    // A shared empty roster so the floor-only path (no `temper.toml`) narrates a
    // zero-requirement roster rather than special-casing the absence at each call.
    static EMPTY: BTreeMap<String, Requirement> = BTreeMap::new();
    layer.map_or(&EMPTY, AuthorLayer::requirements)
}

/// The package the `kind`'s members are checked against — the author layer's explicit
/// binding, else the kind's built-in floor package (`specs/architecture/20-surface.md`, "Decision:
/// package binding is by artifact kind": skill → `skill.anthropic`, rule →
/// `rule.anthropic`).
fn bound_package(layer: Option<&AuthorLayer>, kind: &str) -> String {
    let floor = match kind {
        "rule" => builtin::RULE_PACKAGE,
        _ => builtin::SKILL_PACKAGE,
    };
    layer
        .and_then(|layer| layer.kind_package(kind))
        .unwrap_or(floor)
        .to_string()
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
#[must_use]
pub fn why(
    workspace: &Workspace,
    layer: Option<&AuthorLayer>,
    custom: &[CustomMember],
    by_kind: &BTreeMap<&str, &[Features]>,
    edges: &[Edge],
    member: &str,
) -> String {
    let members = members(workspace, custom);
    let roster = roster(layer);
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
        why_one(&mut out, member, roster, layer, &resolved);
    }
    out
}

/// Narrate one matched member into `out` — the full forward walk for a single
/// `(kind, id)` node.
fn why_one(
    out: &mut String,
    member: &Member,
    roster: &BTreeMap<String, Requirement>,
    layer: Option<&AuthorLayer>,
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

    // The package the member's kind binds — the governing contract its conformance is
    // checked against.
    let _ = writeln!(
        out,
        "Governing package: its `{}` kind binds the `{}` package, whose clauses check it.\n",
        member.kind,
        bound_package(layer, &member.kind),
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

/// `temper requirements [<name>]` — narrate the requirement roster. Without a name it
/// is the forward roster view: each requirement with its satisfier set and coverage
/// state. With a name it is the reverse walk over that one requirement: its satisfiers
/// and the blast radius a removal would strand (`specs/architecture/20-surface.md`, "Decision: the
/// CLI gains a read family"; the traversal payoff of `specs/architecture/30-landscapes.md` law 6).
/// A read, never a gate — the caller prints this and exits zero on every input.
#[must_use]
pub fn requirements(
    workspace: &Workspace,
    layer: Option<&AuthorLayer>,
    custom: &[CustomMember],
    name: Option<&str>,
) -> String {
    let members = members(workspace, custom);
    let roster = roster(layer);
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
        return "The assembly declares no requirements — the roster is empty. \
                Declare `[requirement.<name>]` in `temper.toml` to name an obligation.\n"
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
        let mut out = format!("No requirement named `{name}` is declared in the assembly.\n");
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
