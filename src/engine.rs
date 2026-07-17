//! The generic contract engine — evaluate a [`Contract`]'s clauses over
//! extracted [`Features`].
//!
//! Kills the heuristic rule
//! registry: rules no longer live in a hardcoded `all_rules()` registry with
//! the tool's opinions buried in `if` statements. Instead an author-declared
//! contract (a closed set of decidable clauses) is validated by *this* one
//! generic engine. The engine knows no artifact kind and no rule name — it reads
//! only the declared clauses and the deterministically-extracted features, so
//! there is nowhere to hardcode an opinion.
//!
//! For each artifact's [`Features`], [`validate`] evaluates every clause as a
//! decidable predicate and, on a false predicate, emits a [`check::Diagnostic`]:
//!
//! - **severity** is the clause's *declared* weight — `required` ⇒ [`Error`],
//!   `advisory` ⇒ [`Warn`] — never a tool-baked split.
//! - **rule** is the clause key (the predicate's TOML discriminator, e.g.
//!   `max_len`), so a finding names the clause that produced it.
//! - **artifact** is the features' `id`.
//!
//! ## The two grains
//!
//! A clause binds to a [`Selection`] and evaluates at one of two grains. [`validate`]
//! judges the member grain — one member's own [`Features`]. [`judge`] judges the
//! selection grain — the set predicates, over whatever selector picked the set
//! (`crate::graph::degree` reads the same selections, since a degree bound needs the
//! reference graph the members alone do not carry).
//!
//! ## The honest bound (`verified_by` philosophy)
//!
//! A predicate no judge decides never degrades to a working no-op: [`admissibility`]
//! **fences it**, so a hand-authored clause fails loudly instead of quietly deciding
//! nothing. One predicate is fenced — `dependency-exists`, which names no decidable
//! reference syntax or extractor, so no projection carries the fact it would range
//! over.
//!
//! [`Error`]: check::Severity::Error
//! [`Warn`]: check::Severity::Warn

use std::collections::{BTreeMap, BTreeSet};

use crate::check::{self, Diagnostic};
use crate::contract::{self, Clause, Contract, EdgeBound, ExtentUnit, Predicate};
use crate::extract::{FeatureValue, Features, ValueType};

/// Validate every artifact's [`Features`] against the contract's clauses,
/// collecting a [`Diagnostic`] per violation at the clause's declared severity.
///
/// The artifact slice is passed whole because cross-artifact clauses (e.g.
/// `unique-name`) decide over the set, not one unit — the whole-kind slice
/// [`crate::extract::Features`] is carried in.
///
/// This is the **member grain** only. A clause whose predicate ranges over the
/// selection is skipped here and judged by [`judge`] over the assembled corpus: this
/// loop sees one member of the kind's selection at a time, and two of the set
/// predicates reach past the kind's own members entirely (`membership` derives its
/// allowed set from another selection, `degree` counts arcs in the reference graph).
#[must_use]
pub fn validate(contract: &Contract, artifacts: &[Features]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for features in artifacts {
        for clause in &contract.clauses {
            if clause.predicate.ranges_over_selection() {
                continue;
            }
            for message in evaluate(contract, &clause.predicate, features, artifacts) {
                diagnostics.push(
                    Diagnostic::new(
                        severity_of(clause.severity),
                        &clause.label,
                        &features.id,
                        message,
                    )
                    // The clause's colocated guidance rides its own violation — the
                    // just-in-time teaching moment.
                    .with_guidance(clause.guidance.clone()),
                );
            }
        }
    }
    diagnostics
}

/// Validate a contract against **the definition** — the closed algebra itself —
/// returning an error-severity [`Diagnostic`] per inadmissible clause. This is
/// *admissibility*: the
/// contract earns trust the way a harness does, by passing a check, before it
/// is used to check anything.
///
/// Admissibility composes *on top* of loading, never re-doing it. Closed-
/// vocabulary rejection (an unknown predicate) and charset-range validity are
/// already enforced as load errors in [`crate::contract`]; a [`Contract`] that
/// reached this engine has cleared both. The only admissibility clause decidable
/// today over the current algebra is **list non-emptiness**: an `enum` or `deny`
/// with no values, or a `forbidden_keys` / `require_sections` with no entries, is
/// a vacuous clause that can never decide anything — inadmissible. (The
/// `pattern`-compiles and `verified_by`-resolves clauses the spec also names extend
/// this same pass when those primitives land.)
///
/// Every finding is [`check::Severity::Error`]: an inadmissible contract must
/// fail the run, exactly as a `required` conformance violation does — there is no
/// "advisory" admissibility, because a contract that cannot be trusted cannot be
/// used. The diagnostic's `artifact` is the contract's display label so a finding
/// names the contract it indicts.
///
/// `locus` is where the bound kind's members live. Most rules never read it; the
/// body-shaped fence does, because a predicate's decidability is a fact about the
/// selection's locus, not about the predicate alone ([`bodyless`]).
#[must_use]
pub fn admissibility(contract: &Contract, locus: &Locus) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for clause in &contract.clauses {
        for message in inadmissibilities(&clause.predicate, locus, &contract.clauses) {
            diagnostics.push(Diagnostic::error(&clause.label, &contract.name, message));
        }
    }
    diagnostics
}

/// Where a clause's selected members live — the one thing beyond the predicate a
/// vacuity rule reads.
///
/// The distinction is decidability, not taxonomy: a member at the document locus is
/// extracted from a file of its own, so every body-derived feature carries real bytes;
/// an embedded member is read off its host's declared surface and owns no document, so
/// the same features arrive empty and any predicate ranging over them returns one fixed
/// answer for every member.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Locus {
    /// The selection's members each own a document — the at-locus binding, where the
    /// full predicate vocabulary is decidable.
    Document,
    /// The members of the named embedded kind, folded from their hosts' bodies.
    Embedded(String),
}

/// The admissibility violations of a single clause's predicate at `locus` — empty when
/// the clause is well-formed over the definition. Three decidable checks live here
/// today: (1) the predicate has a judge at all, since one that does not could only ever
/// return [`Outcome::Indeterminate`] — a silent no-op; (2) a value/key/kind list is
/// non-empty — a list-bearing predicate with an empty list is vacuous (an `enum` over no
/// values admits nothing; `forbidden_keys` over no keys forbids nothing; a `type` over no
/// kinds admits no value the lattice can carry), which the author cannot have meant; and
/// (3) the predicate reads a feature the bound locus carries ([`bodyless`]).
///
/// `siblings` is the clause set this predicate's own clause sits in — the wider context
/// `closed-keys` needs, whose allow-list is those siblings' declared keys and which is
/// therefore vacuous or not depending on them rather than on itself.
///
/// `pub(crate)` so [`crate::roster::admissibility`] reuses the same per-predicate rules
/// for a requirement's own `clauses` — one definition of "vacuous", never a second copy
/// drifting beside it.
pub(crate) fn inadmissibilities(
    predicate: &Predicate,
    locus: &Locus,
    siblings: &[Clause],
) -> Vec<String> {
    let mut messages: Vec<String> = judgeless(predicate).into_iter().collect();
    messages.extend(bodyless(predicate, locus));
    messages.extend(unaddressable(predicate));
    messages.extend(vacuities(predicate, siblings));
    messages
}

/// The refusal when the clause's `field` falls outside the declared addressing subset,
/// else `None`.
///
/// The subset — name segments and `[*]` — is what keeps the RFC 9535 engine underneath
/// hidden mechanics rather than an author-facing pattern language, and a bound that is
/// not enforced is not a bound: a filter or a slice fails the contract here rather than
/// quietly evaluating.
///
/// A **presence** clause carries the one extra rule: its path must end in a name segment,
/// because presence is asked of a *key*. `required("plugins[*]")` names elements, so
/// there is no key to be absent and the clause could never fire.
fn unaddressable(predicate: &Predicate) -> Option<String> {
    let field = addressed_field(predicate)?;
    let path = match crate::address::FieldPath::parse(field) {
        Ok(path) => path,
        Err(refusal) => return Some(refusal),
    };
    if matches!(predicate, Predicate::Required { .. }) && path.split_leaf().is_none() {
        return Some(format!(
            "`required` clause on field `{field}` addresses an array's elements rather \
             than a key, so no key of it can be absent; a presence clause's path ends in \
             a name segment (`plugins[*].source`)"
        ));
    }
    None
}

/// The `field` this predicate addresses by an [`crate::address::FieldPath`] — the
/// per-member value predicates, the ones whose `field` is a path rather than a bare key.
///
/// `forbidden_keys` and `must_define` name *keys*, not paths, and the set predicates'
/// `field` is read by their own judges over a selection, so none of them lands here.
///
/// The match names every arm rather than defaulting: a bound that is not enforced is not
/// a bound, and a wildcard would admit the next field-carrying predicate with an
/// unaddressable path instead of asking its author this question at compile time.
fn addressed_field(predicate: &Predicate) -> Option<&str> {
    match predicate {
        Predicate::Required { field }
        | Predicate::Optional { field }
        | Predicate::Type { field, .. }
        | Predicate::MinLen { field, .. }
        | Predicate::MaxLen { field, .. }
        | Predicate::Range { field, .. }
        | Predicate::Enum { field, .. }
        | Predicate::Deny { field, .. }
        | Predicate::AllowedChars { field, .. }
        | Predicate::Shape { field, .. }
        | Predicate::GlobValid { field } => Some(field),
        // A key or a body marker, not a path — there is nothing for the addressing
        // subset to range over.
        Predicate::ForbiddenKeys { .. }
        | Predicate::MustDefine { .. }
        | Predicate::ClosedKeys
        | Predicate::Extent { .. }
        | Predicate::RequireSections { .. }
        | Predicate::SectionContains { .. }
        | Predicate::NameMatchesDir
        | Predicate::UniqueName
        | Predicate::DependencyExists
        // The set predicates' `field` is read by their own judges over a resolved
        // selection, never parsed as a member-local path.
        | Predicate::Count { .. }
        | Predicate::Unique { .. }
        | Predicate::Membership { .. }
        | Predicate::Degree { .. }
        | Predicate::Kind { .. }
        | Predicate::MentionReachable { .. }
        | Predicate::FormatPlacesEdges => None,
    }
}

/// The fence message when `predicate` reads a feature `locus` carries none of, else
/// `None`.
///
/// An embedded member is lifted from its host's declared surface — its leaves become
/// fields, and the body-derived headings/sections/source-directory are empty because there
/// is no document to read them from. A predicate ranging over one of those features
/// therefore returns the same answer over every member of the kind: `section_contains`
/// finds no section to indict, `require_sections` misses every named heading,
/// `name-matches-dir` has no directory to compare. Deciding nothing, they are inadmissible
/// where they are bound, rather than degrading to a check that never ran.
///
/// `extent` is *not* fenced here: a composed embedded member's rendered span is captured at
/// emit and rides its `nested_member` row, so the clause reads real data. A member no
/// format rendered carries no span, and its each-grain verdict is [`Outcome::Indeterminate`]
/// per member rather than a kind-wide fence.
///
/// The line is the feature read, not the predicate's family: `must_define` looks the
/// part but resolves as field presence, which an embedded member's leaves answer, so it
/// stays decidable and unfenced.
fn bodyless(predicate: &Predicate, locus: &Locus) -> Option<String> {
    let Locus::Embedded(kind) = locus else {
        return None;
    };
    let feature = match predicate {
        Predicate::RequireSections { .. } => "the body's headings",
        Predicate::SectionContains { .. } => "the body's sections",
        Predicate::NameMatchesDir => "the member's source directory",
        _ => return None,
    };
    Some(format!(
        "`{}` ranges over {feature}, which no member of embedded kind `{kind}` has: an \
         embedded member is read off its host's declared surface, never a document of \
         its own, so the clause decides the same thing over every member",
        predicate.key()
    ))
}

/// The fence message when no judge decides `predicate`, else `None`.
///
/// A predicate with no judge cannot decide its clause, and a clause that decides
/// nothing must fail admissibility rather than degrade to a working no-op.
fn judgeless(predicate: &Predicate) -> Option<String> {
    match predicate {
        Predicate::DependencyExists => Some(
            "`dependency-exists` is held back: it names no decidable reference \
             syntax or extractor, so it is inadmissible as a clause"
                .to_string(),
        ),
        _ => None,
    }
}

/// The clause's vacuity violations — a predicate that can never decide anything over
/// *any* selection, at any locus: an empty `enum` admits nothing and an inverted bound
/// excludes everything, whatever the clause binds to. The locus-dependent half of
/// vacuity is [`bodyless`]'s.
///
/// `siblings` is the clause's own clause set, which one predicate's vacuity is a fact
/// about: `closed-keys` reads its allow-list there, so a contract declaring no key at all
/// spells "every key is undeclared" rather than a closed schema.
fn vacuities(predicate: &Predicate, siblings: &[Clause]) -> Vec<String> {
    match predicate {
        // `closed-keys` declares the kind's own key set exhaustive; over no declared key
        // that is not a closed schema but a clause indicting every member's every key —
        // the empty-`forbidden_keys` refusal's mirror image, and as surely unmeant.
        Predicate::ClosedKeys if contract::declared_keys(siblings).is_empty() => {
            vec![
                "`closed-keys` clause declares the key set exhaustive on a contract that \
                 declares no `required` or `optional` key, so every key of every member \
                 would be undeclared"
                    .to_string(),
            ]
        }
        Predicate::Enum { field, values } if values.is_empty() => {
            vec![format!("`enum` clause on field `{field}` lists no values")]
        }
        Predicate::Deny { field, values } if values.is_empty() => {
            vec![format!("`deny` clause on field `{field}` lists no values")]
        }
        Predicate::ForbiddenKeys { keys } if keys.is_empty() => {
            vec!["`forbidden_keys` clause lists no keys".to_string()]
        }
        // A `type` clause over no kinds admits no value at all — every field the
        // lattice can carry fails it — so it is vacuous in the same way an inverted
        // `range` bound is, and the author cannot have meant it.
        Predicate::Type { field, kinds } if kinds.is_empty() => {
            vec![format!("`type` clause on field `{field}` lists no kinds")]
        }
        Predicate::RequireSections { sections } if sections.is_empty() => {
            vec!["`require_sections` clause lists no sections".to_string()]
        }
        // An empty `section_contains` marker is a substring of every body, so the
        // clause can never fire — vacuous, and inadmissible like an empty-list
        // clause. An empty *heading* prefix is not vacuous (it governs every
        // section, a meaningful "every section carries the marker"), so it stands.
        Predicate::SectionContains { heading, marker } if marker.is_empty() => {
            vec![format!(
                "`section_contains` clause on heading `{heading}` names an empty marker"
            )]
        }
        // An inverted bound (`min > max`) admits no value at all — a vacuous
        // clause the author cannot have meant, so the contract carrying it fails
        // admissibility.
        Predicate::Range { field, min, max } if min > max => {
            vec![format!(
                "`range` clause on field `{field}` has min {min} greater than max {max}"
            )]
        }
        // The node-set `count` bound is the same "reject min>max" rule as `range`,
        // over the satisfier-set's cardinality rather than a field's value.
        Predicate::Count { min, max } if min > max => {
            vec![format!(
                "`count` clause has min {min} greater than max {max}"
            )]
        }
        // An empty `target` names no requirement to draw the allowed set from — a
        // membership clause that can never resolve its source set.
        Predicate::Membership { field, target } if target.is_empty() => {
            vec![format!(
                "`membership` clause on field `{field}` names an empty target requirement"
            )]
        }
        // `degree` with neither direction bounded constrains nothing — vacuous, like
        // an empty-list predicate. A bounded direction whose own `min > max` is
        // likewise vacuous in that direction, the same inverted-bound rule as
        // `range`/`count`.
        Predicate::Degree { incoming, outgoing } => {
            let mut messages = Vec::new();
            if incoming.is_none() && outgoing.is_none() {
                messages.push("`degree` clause carries no incoming or outgoing bound".to_string());
            }
            for (label, bound) in [("incoming", incoming), ("outgoing", outgoing)] {
                if let Some(EdgeBound {
                    min: Some(min),
                    max: Some(max),
                }) = bound
                    && min > max
                {
                    messages.push(format!(
                        "`degree` clause's {label} bound has min {min} greater than max {max}"
                    ));
                }
            }
            messages
        }
        // An empty `kind` names no kind to narrow to — vacuous, like an empty-list
        // predicate: it can never decide anything over a real satisfier.
        Predicate::Kind { kind } if kind.is_empty() => {
            vec!["`kind` clause names an empty kind".to_string()]
        }
        // Both arguments are *field names*, not glob sets — an empty glob set on either
        // end is member data the judge reads (and is precisely what "unscoped source" /
        // "ungated target" mean), never a property of the clause. An empty field name
        // is the clause-level vacuity: an empty gate field reads no field, so every
        // target is ungated and the clause can never fire, and an empty scope field
        // makes every source unscoped, so it fires on target gating alone and decides
        // nothing about the pair. Neither can be what the author meant.
        Predicate::MentionReachable {
            scope_field,
            gate_field,
        } => {
            let mut messages = Vec::new();
            if scope_field.is_empty() {
                messages.push("`mention-reachable` clause names an empty scope field".to_string());
            }
            if gate_field.is_empty() {
                messages.push("`mention-reachable` clause names an empty gate field".to_string());
            }
            messages
        }
        _ => Vec::new(),
    }
}

/// A declared, decidable expression picking the set a contract binds to. Selectors are
/// atomic and do not compose: narrowing a selection is an each-grain clause over it
/// (`kind`), never a second selector, so a member outside the narrowing is a finding
/// rather than a silent exclusion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    /// Every member of a kind — the universal binding.
    Kind(String),
    /// The members whose satisfies edge targets a requirement — the existential
    /// binding.
    OptIn(String),
}

impl Selector {
    /// The kind or requirement name this selector picks by — the `artifact` a finding
    /// over the selection names.
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Selector::Kind(kind) => kind,
            Selector::OptIn(requirement) => requirement,
        }
    }

    /// The selection as a finding's subject, e.g. ``kind `skill` `` — `pub` so
    /// [`crate::graph::degree`]'s findings name their selection in the identical words
    /// the other set predicates' do.
    #[must_use]
    pub fn noun(&self) -> String {
        match self {
            Selector::Kind(kind) => format!("kind `{kind}`"),
            Selector::OptIn(requirement) => format!("requirement `{requirement}`"),
        }
    }
}

/// The set a contract binds to: the [`Selector`] that picked it, the members it
/// resolved to (each tagged with its own kind), and the clauses bound to it.
///
/// One algebra judges every selection — the quantifier is the clause's *grain*, never
/// the selector's, so there is no universal/existential machinery to keep in step.
pub struct Selection<'a> {
    /// The declared expression that picked the members.
    pub selector: Selector,
    /// The clauses bound to this selection, in declaration order.
    pub clauses: Vec<Clause>,
    /// The selected members, each tagged with its own kind label.
    pub members: Vec<(&'a str, &'a Features)>,
}

impl Selection<'_> {
    /// Every selected member's `field` value that is a scalar — the projection the
    /// whole-grain field predicates decide over. A member missing the field carries no
    /// value, so it contributes none.
    fn values<'f>(&'f self, field: &str) -> impl Iterator<Item = (&'f str, String)> {
        self.members.iter().filter_map(move |(_, features)| {
            let value = features.field(field)?;
            Some((features.id.as_str(), value.as_scalar()?.to_string()))
        })
    }
}

/// Judge every clause bound to each selection, at the **selection grain** — the one
/// algebra behind `count`/`unique`/`membership` (whole) and `kind` (each), whichever
/// selector picked the set. Each finding carries its clause's own declared severity and
/// names the selection it indicts.
///
/// `selections` is the whole declared set rather than one, because `membership` draws
/// its allowed values from a *second* selection — shaping that set is the target's own
/// job, never re-derived here.
///
/// Two grains are judged elsewhere off the same clause set: `degree` by
/// [`crate::graph::degree`], which needs the reference graph the members do not carry,
/// and the member-grain predicates by [`validate`], which reads one member's
/// [`Features`].
#[must_use]
pub fn judge(selections: &[Selection]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for selection in selections {
        for clause in &selection.clauses {
            match &clause.predicate {
                Predicate::Count { min, max } => {
                    diagnostics.extend(out_of_band(selection, clause, *min, *max));
                }
                Predicate::Unique { field } => {
                    diagnostics.extend(duplicates(selection, clause, field));
                }
                Predicate::Membership { field, target } => {
                    diagnostics.extend(out_of_set(selections, selection, clause, field, target));
                }
                Predicate::Kind { kind } => {
                    diagnostics.extend(wrong_kind(selection, clause, kind));
                }
                Predicate::Extent {
                    unit,
                    max,
                    whole: true,
                } => {
                    diagnostics.extend(over_budget(selection, clause, *unit, *max));
                }
                // `degree` binds to a selection too, but its judge needs the graph.
                // Every other predicate binds to a member, not a set.
                _ => {}
            }
        }
    }
    diagnostics
}

/// The whole-grain `count` finding when the selection's cardinality falls outside the
/// declared bound — naming the selection, the members, and the `[min, max]` it missed.
fn out_of_band(
    selection: &Selection,
    clause: &Clause,
    min: usize,
    max: usize,
) -> Option<Diagnostic> {
    if (min..=max).contains(&selection.members.len()) {
        return None;
    }
    let listed = if selection.members.is_empty() {
        String::new()
    } else {
        format!(
            " ({})",
            selection
                .members
                .iter()
                .map(|(_, features)| features.id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    Some(finding(
        selection,
        clause,
        format!(
            "{} selects {} member(s){listed}, outside its declared count bound [{min}, {max}]",
            selection.selector.noun(),
            selection.members.len(),
        ),
    ))
}

/// The whole-grain `extent` finding when the selection's **summed** rendered extent
/// exceeds the budget — the ambient-context ceiling ("everything always-on under N
/// lines") that falls out of the grain axis. Each member contributes its own rendered
/// extent in the declared unit; the finding names the selection, the total, and the bound.
fn over_budget(
    selection: &Selection,
    clause: &Clause,
    unit: ExtentUnit,
    max: usize,
) -> Option<Diagnostic> {
    // A member no format rendered (`None`) carries no rendered projection, so it is not
    // part of the population a rendered budget sums — dropped here the way `unique`/
    // `membership` skip a member missing the field, never read as a zero that pads the sum.
    let total: usize = selection
        .members
        .iter()
        .filter_map(|(_, features)| match unit {
            ExtentUnit::Lines => features.rendered_lines,
            ExtentUnit::Characters => features.rendered_chars,
        })
        .sum();
    if total <= max {
        return None;
    }
    Some(finding(
        selection,
        clause,
        format!(
            "{} has a summed rendered extent of {total} {} (max {max})",
            selection.selector.noun(),
            unit.name(),
        ),
    ))
}

/// The whole-grain `unique` findings for one declared `field` — one per value two or
/// more members share. A member missing the field carries no value to collide on, so it
/// is silently skipped: a missing field is no collision. Values are grouped in a
/// [`std::collections::BTreeMap`] so the finding set is stable across runs.
fn duplicates(selection: &Selection, clause: &Clause, field: &str) -> Vec<Diagnostic> {
    let mut by_value: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for (id, value) in selection.values(field) {
        by_value.entry(value).or_default().push(id);
    }
    by_value
        .into_iter()
        .filter(|(_, sharers)| sharers.len() > 1)
        .map(|(value, sharers)| {
            finding(
                selection,
                clause,
                format!(
                    "{} requires `{field}` unique across its selection, but {} members share `{field}` = `{value}` ({})",
                    selection.selector.noun(),
                    sharers.len(),
                    sharers.join(", ")
                ),
            )
        })
        .collect()
}

/// The whole-grain `membership` findings: build the allowed set from `field` over the
/// selection `target` names, then emit one finding per member whose own `field` scalar
/// is absent from it. A member missing `field` carries no value to check, so it is
/// silently skipped — a missing field is no violation, the way a missing `unique` field
/// is no collision.
///
/// The allowed set is corpus-*derived*, so a `target` with no members — or a `target` no
/// selector declares — yields the empty set, under which every valued member is
/// genuinely a non-member.
fn out_of_set(
    selections: &[Selection],
    selection: &Selection,
    clause: &Clause,
    field: &str,
    target: &str,
) -> Vec<Diagnostic> {
    let source = Selector::OptIn(target.to_string());
    let allowed: BTreeSet<String> = selections
        .iter()
        .filter(|other| other.selector == source)
        .flat_map(|other| other.values(field))
        .map(|(_, value)| value)
        .collect();

    selection
        .values(field)
        .filter(|(_, value)| !allowed.contains(value))
        .map(|(id, value)| {
            finding(
                selection,
                clause,
                format!(
                    "{} requires `{field}` of each member drawn from the `{field}` feature of the members satisfying `{target}`, but `{id}` declares `{field}` = `{value}`, which is not in that set",
                    selection.selector.noun(),
                ),
            )
        })
        .collect()
}

/// The each-grain `kind` findings — one per selected member whose actual kind is not the
/// declared one. A member outside the narrowing is a finding, never a silent exclusion
/// from the set `count`/`unique`/`membership` range over.
///
/// A member's *actual* kind is the selection's own context, never a fact [`Features`]
/// itself carries, so this predicate is judged here rather than in [`decide`]'s
/// per-`Features` table.
fn wrong_kind(selection: &Selection, clause: &Clause, kind: &str) -> Vec<Diagnostic> {
    selection
        .members
        .iter()
        .filter(|(actual, _)| *actual != kind)
        .map(|(actual, features)| {
            finding(
                selection,
                clause,
                format!(
                    "{} narrows its members to kind `{kind}`, but `{}` is kind `{actual}`",
                    selection.selector.noun(),
                    features.id,
                ),
            )
        })
        .collect()
}

/// One selection-grain finding: the clause's own declared severity, its address as the
/// rule id, the selection as the indicted artifact, and the clause's colocated guidance.
fn finding(selection: &Selection, clause: &Clause, message: String) -> Diagnostic {
    Diagnostic::new(
        severity_of(clause.severity),
        &clause.label,
        selection.selector.label(),
        message,
    )
    .with_guidance(clause.guidance.clone())
}

/// Evaluate one predicate over one artifact's features, returning a message per
/// violation (empty ⇒ the clause holds — see [`Outcome`]).
fn evaluate(
    contract: &Contract,
    predicate: &Predicate,
    features: &Features,
    all: &[Features],
) -> Vec<String> {
    match decide(contract, predicate, features, all) {
        Outcome::Holds => Vec::new(),
        Outcome::Violated(messages) => messages,
        // Unreachable on an admissible run: [`admissibility`] fences every producer
        // before conformance. The empty vec keeps that floor silent rather than
        // reporting a verdict no judge reached.
        Outcome::Indeterminate => Vec::new(),
    }
}

/// The result of testing a predicate against features. `Indeterminate` is the
/// honest third state for a clause whose backing feature the current projection
/// does not carry — distinct from `Holds`, so the engine never *claims* to have
/// checked what it could not.
enum Outcome {
    /// The predicate is true of the features.
    Holds,
    /// The predicate is false; each string is one violation to report.
    Violated(Vec<String>),
    /// The feature this predicate names is absent from the projection, so the
    /// clause cannot be decided here (no pass, no finding).
    Indeterminate,
}

impl Outcome {
    /// A single-message violation.
    fn violated(message: String) -> Self {
        Outcome::Violated(vec![message])
    }

    /// `Holds` when `ok`, else a single-message violation.
    fn check(ok: bool, message: impl FnOnce() -> String) -> Self {
        if ok {
            Outcome::Holds
        } else {
            Outcome::violated(message())
        }
    }
}

/// The decision table — one arm per primitive. Every arm is decidable *given the
/// feature it names*; the predicates ranging over a whole selection rather than one
/// member ([`Predicate::ranges_over_selection`]) return [`Outcome::Indeterminate`] here
/// rather than a fabricated pass, since their judges live elsewhere.
///
/// `contract` is the clause set the predicate was drawn from — the wider read `closed-keys`
/// needs, whose allow-list is its own siblings' declared keys. Reading past the one field
/// is evaluation cost, never a second category: the verdict is still one member's own, the
/// grain `validate` judges at.
fn decide(
    contract: &Contract,
    predicate: &Predicate,
    features: &Features,
    all: &[Features],
) -> Outcome {
    match predicate {
        // A value/presence predicate is the *only* owner of its field's
        // presence; the other field predicates stay silent when the field is
        // absent so one missing field yields one finding, not a cascade.
        //
        // Presence is asked of the path's *parent*, since an absent key locates no node
        // of its own: `plugins[*].source` fires once per entry that omits it.
        Predicate::Required { field } => match addressing(field) {
            None => Outcome::Indeterminate,
            Some(path) => {
                let absent: Vec<String> = features
                    .locate_presence(&path)
                    .into_iter()
                    .filter(|(_, present)| !present)
                    .map(|(address, _)| format!("required field `{address}` is absent"))
                    .collect();
                if absent.is_empty() {
                    Outcome::Holds
                } else {
                    Outcome::Violated(absent)
                }
            }
        },

        // `optional` records that a key is part of the declared schema; it is
        // always satisfied — its presence or absence is never a violation.
        Predicate::Optional { .. } => Outcome::Holds,

        // `type` compares the field's *preserved source kind* to the declared set,
        // holding when it is any member — a field an external format documents as
        // `string|array` is gated by the set, never by picking one of the two. An
        // absent field is the `required` clause's concern, so `type` stays silent on
        // absence (like the other field predicates).
        Predicate::Type { field, kinds } => addressed(features, field, |address, value| {
            let actual = value.kind();
            (!kinds.contains(&actual)).then(|| {
                format!(
                    "field `{address}` is `{}` but the contract declares `{}`",
                    actual.name(),
                    declared_kinds(kinds)
                )
            })
        }),

        Predicate::MinLen { field, min } => addressed(features, field, |address, value| {
            let len = value.as_scalar()?.chars().count();
            (len < *min).then(|| format!("field `{address}` is {len} characters (min {min})"))
        }),

        Predicate::MaxLen { field, max } => addressed(features, field, |address, value| {
            let len = value.as_scalar()?.chars().count();
            (len > *max).then(|| format!("field `{address}` is {len} characters (max {max})"))
        }),

        // `range` bounds a *numeric* field to `[min, max]`. It fires only when the
        // field is present, parsed as `integer`/`number`, and falls outside the
        // bound; it stays silent on absence (the `required` clause's concern) and
        // on a non-numeric kind (a `type` clause owns that mismatch) so one wrong
        // field yields one finding, not a cascade.
        Predicate::Range { field, min, max } => addressed(features, field, |address, value| {
            if !matches!(value.kind(), ValueType::Integer | ValueType::Number) {
                return None;
            }
            // The value type says numeric but the text would not parse — don't
            // fabricate a finding over a value we cannot read.
            let n = value.as_scalar()?.parse::<f64>().ok()?;
            (!(*min..=*max).contains(&n))
                .then(|| format!("field `{address}` value {n} is outside the range [{min}, {max}]"))
        }),

        Predicate::Enum { field, values } => addressed(features, field, |address, value| {
            let text = value.as_scalar()?;
            (!values.iter().any(|v| v == text)).then(|| {
                format!(
                    "field `{address}` value `{text}` is not one of [{}]",
                    values.join(", ")
                )
            })
        }),

        Predicate::Deny { field, values } => addressed(features, field, |address, value| {
            let text = value.as_scalar()?;
            values
                .iter()
                .any(|v| v == text)
                .then(|| format!("field `{address}` value `{text}` is denied"))
        }),

        // One finding per offending key, so each forbidden key points at itself.
        Predicate::ForbiddenKeys { keys } => {
            let present: Vec<String> = keys
                .iter()
                .filter(|key| features.has_field(key))
                .map(|key| format!("forbidden key `{key}` is present"))
                .collect();
            if present.is_empty() {
                Outcome::Holds
            } else {
                Outcome::Violated(present)
            }
        }

        // The deny-list's complement: every key the contract's own `required`/`optional`
        // siblings declare is the allow-list, and anything else on the member is a
        // finding — one per undeclared key, so each points at itself.
        Predicate::ClosedKeys => {
            let declared = contract::declared_keys(&contract.clauses);
            let undeclared: Vec<String> = features
                .fields
                .keys()
                .filter(|key| !declared.contains(key.as_str()))
                .map(|key| {
                    format!(
                        "key `{key}` is not one of the keys this contract declares, and the \
                         declared set is exhaustive"
                    )
                })
                .collect();
            if undeclared.is_empty() {
                Outcome::Holds
            } else {
                Outcome::Violated(undeclared)
            }
        }

        Predicate::AllowedChars { field, charset } => {
            addressed(features, field, |address, value| {
                let bad: BTreeSet<char> = value
                    .as_scalar()?
                    .chars()
                    .filter(|&c| !charset.allows(c))
                    .collect();
                (!bad.is_empty()).then(|| {
                    let rendered: String = bad.iter().collect();
                    format!("field `{address}` has characters outside the allowed set: {rendered}")
                })
            })
        }

        // The shape owns both its mechanics and its prose: the finding names the shape
        // and quotes what it demands, and the clause's guidance teaches past that.
        Predicate::Shape { field, shape } => addressed(features, field, |address, value| {
            let value = value.as_scalar()?;
            (!shape.admits(value)).then(|| {
                format!(
                    "field `{address}` does not hold the `{}` shape: {}",
                    shape.name(),
                    shape.demand()
                )
            })
        }),

        // `glob-valid` checks every glob the field carries parses under the one
        // shared `globset` surface (`crate::kind::compile_glob`, brace-aware). An
        // unparseable pattern matches nothing silently, so a dead scope becomes a
        // finding — one per offending glob, each naming itself. Silent on absence
        // (the `required` clause's concern).
        Predicate::GlobValid { field } => match addressing(field) {
            None => Outcome::Indeterminate,
            Some(path) => {
                let bad: Vec<String> = features
                    .locate(&path)
                    .into_iter()
                    .flat_map(|(address, value)| {
                        field_globs(&value)
                            .into_iter()
                            .filter(|glob| crate::kind::compile_glob(glob).is_none())
                            .map(|glob| {
                                format!(
                                    "field `{address}` glob `{glob}` does not parse under globset, so it silently matches nothing"
                                )
                            })
                            .collect::<Vec<String>>()
                    })
                    .collect();
                if bad.is_empty() {
                    Outcome::Holds
                } else {
                    Outcome::Violated(bad)
                }
            }
        },

        // `extent` at the **each** grain: one member's own rendered extent against the
        // bound, in the declared unit. Render-side off the projected body, never the
        // `line_count` primitive's source-side `body_lines`. The whole-grain form ranges
        // over the selection and is judged by [`judge`], so it never reaches this table.
        Predicate::Extent {
            unit,
            max,
            whole: false,
        } => {
            let measured = match unit {
                ExtentUnit::Lines => features.rendered_lines,
                ExtentUnit::Characters => features.rendered_chars,
            };
            // `None` is a member no format rendered — an embedded member read off a
            // layout host's source. It has no projection to budget, so its extent is
            // undecidable rather than a zero read as a pass.
            match measured {
                Some(measured) => Outcome::check(measured <= *max, || {
                    format!("rendered extent is {measured} {} (max {max})", unit.name())
                }),
                None => Outcome::Indeterminate,
            }
        }

        // `require_sections` decides over the extracted body headings: one
        // finding per named section with no matching heading.
        Predicate::RequireSections { sections } => {
            let missing: Vec<String> = sections
                .iter()
                .filter(|section| !features.headings.iter().any(|h| h == *section))
                .map(|section| format!("required section `{section}` is absent from the body"))
                .collect();
            if missing.is_empty() {
                Outcome::Holds
            } else {
                Outcome::Violated(missing)
            }
        }

        // `must_define` over a frontmatter marker (e.g. `disable-model-invocation`)
        // is decidable as field presence.
        Predicate::MustDefine { marker } => Outcome::check(features.has_field(marker), || {
            format!("marker `{marker}` is not defined")
        }),

        // `section_contains` decides over the extracted body sections: every
        // section whose heading *starts with* the declared prefix must carry the
        // declared marker (a substring of its body). One finding per bare section,
        // so each offending section points at itself.
        Predicate::SectionContains { heading, marker } => {
            let bare: Vec<String> = features
                .sections
                .iter()
                .filter(|section| section.heading.starts_with(heading.as_str()))
                .filter(|section| !section.body.contains(marker.as_str()))
                .map(|section| {
                    format!(
                        "section `{}` does not carry the required marker `{marker}`",
                        section.heading
                    )
                })
                .collect();
            if bare.is_empty() {
                Outcome::Holds
            } else {
                Outcome::Violated(bare)
            }
        }

        Predicate::NameMatchesDir => {
            let name = features.field("name");
            match (
                name.as_ref().and_then(FeatureValue::as_scalar),
                features.source_dir.as_deref(),
            ) {
                (Some(name), Some(dir)) => Outcome::check(name == dir, || {
                    format!("name `{name}` does not match its directory `{dir}`")
                }),
                // No name field, or no known source directory: nothing to compare.
                _ => Outcome::Holds,
            }
        }

        Predicate::UniqueName => {
            let shared = all.iter().filter(|other| other.id == features.id).count();
            Outcome::check(shared <= 1, || {
                format!(
                    "name `{}` is not unique ({shared} artifacts share it)",
                    features.id
                )
            })
        }

        // `format-places-edges` decides over the placement `emit` observed and lowered
        // into the member's own declaration row: one finding per omitted edge, so each
        // points at the field it left unrepresented. A member with no format of its own
        // is not a format to indict, and a format whose value carries no edge placed
        // everything there was to place — both hold, neither is a fabricated pass.
        Predicate::FormatPlacesEdges => {
            let Some(placements) = &features.edge_placements else {
                return Outcome::Holds;
            };
            let omitted: Vec<String> = placements
                .iter()
                .filter(|(_, placed)| !**placed)
                .map(|(field, _)| {
                    format!(
                        "the format renders no `{field}` edge, so it projects a contract the prose does not represent"
                    )
                })
                .collect();
            if omitted.is_empty() {
                Outcome::Holds
            } else {
                Outcome::Violated(omitted)
            }
        }

        // The defensive floor, never a fabricated pass, and never reached on a valid
        // run: [`admissibility`] fences `dependency-exists` before conformance, and
        // [`validate`] routes a set predicate to its own judge instead of this
        // per-member table. `dependency-exists` lights up here with no other engine
        // change once an extractor carries a declared-dependency model.
        Predicate::DependencyExists
        | Predicate::Count { .. }
        | Predicate::Unique { .. }
        | Predicate::Membership { .. }
        | Predicate::Degree { .. }
        | Predicate::Kind { .. }
        // `mention-reachable` is each-grain over the selection, but its verdict reads
        // the mention graph and the *target* member's gate field — neither is on the
        // member in hand — so `crate::graph::mention_reachable` judges it, exactly as
        // `degree`'s judge lives there.
        | Predicate::MentionReachable { .. }
        // Whole-grain `extent` sums the selection; [`judge`] decides it, not this
        // per-member table.
        | Predicate::Extent { whole: true, .. } => Outcome::Indeterminate,
    }
}

/// A `type` clause's declared set, rendered for a diagnostic: the lattice names in
/// lattice order, `|`-joined — the spelling the external formats themselves use for a
/// documented union (`string|array`).
///
/// A one-element set renders as the bare kind name, so a single-kind clause's finding
/// reads exactly as it did before the set widening.
fn declared_kinds(kinds: &BTreeSet<ValueType>) -> String {
    kinds
        .iter()
        .map(|kind| kind.name())
        .collect::<Vec<&str>>()
        .join("|")
}

/// The parsed addressing path a clause's `field` spells, or `None` when it falls outside
/// the declared subset.
///
/// `None` is unreachable on an admissible run — [`admissibility`] refuses an out-of-subset
/// path before any member is judged — so its callers land on
/// [`Outcome::Indeterminate`]: no pass, no finding, exactly as the other fenced
/// predicates do.
fn addressing(field: &str) -> Option<crate::address::FieldPath> {
    crate::address::FieldPath::parse(field).ok()
}

/// Judge `verdict` over every value the clause's `field` path addresses on this member —
/// one node for a path of name segments, one per element under a `[*]`, so an each-grain
/// clause indicts each offending element by its own address.
///
/// `verdict` returns the violation message, or `None` where the value is not this
/// predicate's to indict (a container under a scalar predicate, a non-numeric under
/// `range`). A path that addresses nothing yields no node, so the clause holds silently —
/// absence is the `required` clause's concern.
fn addressed(
    features: &Features,
    field: &str,
    verdict: impl Fn(&str, &FeatureValue) -> Option<String>,
) -> Outcome {
    let Some(path) = addressing(field) else {
        return Outcome::Indeterminate;
    };
    let messages: Vec<String> = features
        .locate(&path)
        .into_iter()
        .filter_map(|(address, value)| verdict(&address, &value))
        .collect();
    if messages.is_empty() {
        Outcome::Holds
    } else {
        Outcome::Violated(messages)
    }
}

/// The glob strings a field value carries — each element of a list, or a lone
/// scalar read as a single glob (a `paths` authored as one string). A map carries
/// none; a type mismatch there is the `type` clause's concern, not this one's.
fn field_globs(value: &FeatureValue) -> Vec<&str> {
    match value {
        FeatureValue::List(items) => items.iter().map(String::as_str).collect(),
        FeatureValue::Scalar { text, .. } => vec![text.as_str()],
        FeatureValue::Map => Vec::new(),
    }
}

/// Map a clause's *declared* severity onto the engine's diagnostic severity:
/// `required` blocks (`Error`), `advisory` reports (`Warn`). The engine never
/// chooses — it only translates what the author declared.
///
/// `pub` so an assembly-scope dial that shares the author's `required`/`advisory`
/// vocabulary — the reachability severity — maps through
/// the one translation, never a second copy that could drift.
#[must_use]
pub fn severity_of(severity: contract::Severity) -> check::Severity {
    match severity {
        contract::Severity::Required => check::Severity::Error,
        contract::Severity::Advisory => check::Severity::Warn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use serde_json::{Value as JsonValue, json};

    use crate::check::{Severity, any_error};
    use crate::contract::{Charset, Clause, Severity as ClauseSeverity};
    use crate::extract::ValueType;

    /// Build a `Features` with the given name-keyed scalar fields, body line
    /// count, and source directory.
    fn features(
        id: &str,
        fields: &[(&str, JsonValue)],
        body_lines: usize,
        source_dir: Option<&str>,
    ) -> Features {
        let fields = fields
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect::<BTreeMap<_, _>>();
        Features {
            id: id.to_string(),
            fields,
            body_lines,
            rendered_lines: Some(body_lines),
            rendered_chars: Some(0),
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: source_dir.map(str::to_string),
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            nested_members: Vec::new(),
            satisfies: Vec::new(),
            edge_placements: None,
        }
    }

    /// `features` with the given body headings — for the `require_sections`
    /// tests, which decide over headings rather than fields.
    fn features_with_headings(id: &str, headings: &[&str]) -> Features {
        let mut f = features(id, &[], 1, None);
        f.headings = headings.iter().map(|h| (*h).to_string()).collect();
        f
    }

    /// A `string` field value (the existing scalar predicates read only the text, so
    /// the source kind is incidental to these tests).
    fn scalar(text: &str) -> JsonValue {
        JsonValue::String(text.to_string())
    }

    /// A [`Clause`] over `predicate` at `severity`, addressed under `owner` — the
    /// `<owner>.<predicate>` spelling emit stamps a fieldless row with. The engine
    /// never derives a label, so a fixture supplies it exactly as a lifted row would.
    fn clause(owner: &str, severity: ClauseSeverity, predicate: Predicate) -> Clause {
        Clause {
            label: crate::contract::clause_label(Some(owner), predicate.key(), None),
            source: None,
            severity,
            predicate,
            guidance: None,
        }
    }

    /// A one-clause contract carrying `predicate` at `severity`.
    fn contract(severity: ClauseSeverity, predicate: Predicate) -> Contract {
        Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses: vec![clause("skill", severity, predicate)],
        }
    }

    /// The `[a-z0-9-]` charset, the `allowed_chars` workhorse.
    fn slug_charset() -> Charset {
        Charset {
            ranges: vec![('a', 'z'), ('0', '9')],
            chars: BTreeSet::from(['-']),
        }
    }

    /// Validate a single artifact against a single required clause and return the
    /// diagnostics — the common shape of the per-primitive tests below.
    fn run(predicate: Predicate, artifact: Features) -> Vec<Diagnostic> {
        validate(
            &contract(ClauseSeverity::Required, predicate),
            std::slice::from_ref(&artifact),
        )
    }

    #[test]
    fn required_fires_on_an_absent_field_and_is_silent_when_present() {
        let absent = features("demo", &[], 1, None);
        let diags = run(
            Predicate::Required {
                field: "name".to_string(),
            },
            absent,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.required");
        assert_eq!(diags[0].artifact, "demo");

        let present = features("demo", &[("name", scalar("demo"))], 1, None);
        assert!(
            run(
                Predicate::Required {
                    field: "name".to_string()
                },
                present
            )
            .is_empty()
        );
    }

    #[test]
    fn optional_never_fires() {
        let any = features("demo", &[], 1, None);
        assert!(
            run(
                Predicate::Optional {
                    field: "license".to_string()
                },
                any
            )
            .is_empty()
        );
    }

    #[test]
    fn type_fires_on_a_kind_mismatch_and_is_silent_on_match_and_absence() {
        let predicate = || Predicate::Type {
            field: "count".to_string(),
            kinds: BTreeSet::from([ValueType::Integer]),
        };

        // The field's preserved source kind differs from the declared one: fires.
        let mismatch = features("demo", &[("count", json!("7"))], 1, None);
        let diags = run(predicate(), mismatch);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.type");
        // The message names both the actual and the declared lattice kind.
        assert!(diags[0].message.contains("string"));
        assert!(diags[0].message.contains("integer"));

        // The kind matches the declaration: silent.
        let matched = features("demo", &[("count", json!(7))], 1, None);
        assert!(run(predicate(), matched).is_empty());

        // An absent field is the `required` clause's concern, not `type`'s.
        let absent = features("demo", &[], 1, None);
        assert!(run(predicate(), absent).is_empty());

        // A container kind is decided the same way — a list where a map is
        // declared fires.
        let container = Predicate::Type {
            field: "tags".to_string(),
            kinds: BTreeSet::from([ValueType::Map]),
        };
        let as_list = features("demo", &[("tags", json!(["a"]))], 1, None);
        assert_eq!(run(container, as_list).len(), 1);
    }

    #[test]
    fn type_over_a_set_holds_for_any_member_and_names_the_whole_set_when_it_fires() {
        // The `string|array` shape an external format documents: both forms hold.
        let union = || Predicate::Type {
            field: "skills".to_string(),
            kinds: BTreeSet::from([ValueType::String, ValueType::List]),
        };
        let as_string = features("demo", &[("skills", json!("./s/"))], 1, None);
        assert!(run(union(), as_string).is_empty());
        let as_list = features("demo", &[("skills", json!(["./s/"]))], 1, None);
        assert!(run(union(), as_list).is_empty());

        // A kind outside the set fires once, and the message names the whole declared
        // set — an author told only "not `string`" cannot see the other form is open.
        let as_bool = features("demo", &[("skills", json!(true))], 1, None);
        let diags = run(union(), as_bool);
        assert_eq!(diags.len(), 1);
        assert_eq!(
            diags[0].message,
            "field `skills` is `boolean` but the contract declares `string|list`"
        );

        // The set is a set: the author's write order is not a distinction it carries.
        assert_eq!(
            union(),
            Predicate::Type {
                field: "skills".to_string(),
                kinds: BTreeSet::from([ValueType::List, ValueType::String]),
            }
        );
    }

    #[test]
    fn a_one_element_set_reads_exactly_as_the_single_kind_clause_it_replaces() {
        // The widening's compatibility bar, in the one place an author sees it: the
        // finding's wording is the pre-set wording, not `declares `integer|``.
        let single = Predicate::Type {
            field: "count".to_string(),
            kinds: BTreeSet::from([ValueType::Integer]),
        };
        let wrong = features("demo", &[("count", json!("7"))], 1, None);
        let diags = run(single, wrong);
        assert_eq!(diags.len(), 1);
        assert_eq!(
            diags[0].message,
            "field `count` is `string` but the contract declares `integer`"
        );
    }

    #[test]
    fn max_len_fires_only_past_the_bound() {
        let predicate = || Predicate::MaxLen {
            field: "name".to_string(),
            max: 3,
        };
        let over = features("demo", &[("name", scalar("toolong"))], 1, None);
        let diags = run(predicate(), over);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.max_len");

        let within = features("demo", &[("name", scalar("ok"))], 1, None);
        assert!(run(predicate(), within).is_empty());
        // An absent field is the `required` clause's concern, not this one's.
        let absent = features("demo", &[], 1, None);
        assert!(run(predicate(), absent).is_empty());
    }

    #[test]
    fn range_fires_only_when_a_numeric_field_falls_outside_the_bound() {
        let predicate = || Predicate::Range {
            field: "score".to_string(),
            min: 0.0,
            max: 100.0,
        };

        // A numeric field past the upper bound fires once, naming the clause.
        let over = features("demo", &[("score", json!(150))], 1, None);
        let diags = run(predicate(), over);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.range");

        // Below the lower bound fires too — a fractional `number` is in scope.
        let under = features("demo", &[("score", json!(-0.5))], 1, None);
        assert_eq!(run(predicate(), under).len(), 1);

        // Within the inclusive bound (and exactly on each edge): silent.
        let within = features("demo", &[("score", json!(42))], 1, None);
        assert!(run(predicate(), within).is_empty());
        for edge in [0, 100] {
            let at_edge = features("demo", &[("score", json!(edge))], 1, None);
            assert!(run(predicate(), at_edge).is_empty(), "edge {edge} holds");
        }

        // An absent field is the `required` clause's concern, not this one's.
        let absent = features("demo", &[], 1, None);
        assert!(run(predicate(), absent).is_empty());

        // A non-numeric kind is a `type` clause's concern: `range` stays silent
        // rather than fire on a value it does not own — no cascade.
        let non_numeric = features("demo", &[("score", json!("150"))], 1, None);
        assert!(run(predicate(), non_numeric).is_empty());
    }

    #[test]
    fn an_inverted_range_is_inadmissible() {
        // A `min > max` bound admits no value — vacuous, so the contract carrying
        // it fails admissibility (an error, exit non-zero).
        let inverted = contract(
            ClauseSeverity::Required,
            Predicate::Range {
                field: "score".to_string(),
                min: 100.0,
                max: 0.0,
            },
        );
        let diags = admissibility(&inverted, &Locus::Document);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.range");
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].artifact, "skill");
        assert!(any_error(&diags));

        // A well-formed `min <= max` bound (equal endpoints included) is admissible.
        for (min, max) in [(0.0, 100.0), (5.0, 5.0)] {
            let ok = contract(
                ClauseSeverity::Required,
                Predicate::Range {
                    field: "score".to_string(),
                    min,
                    max,
                },
            );
            assert!(
                admissibility(&ok, &Locus::Document).is_empty(),
                "[{min}, {max}] is admissible"
            );
        }
    }

    // The node-set family's vacuity rules are exercised where a judge reaches them — a
    // requirement's own clauses, via `crate::roster::admissibility`.

    #[test]
    fn an_inverted_count_bound_is_inadmissible() {
        // `count`'s `min > max` is the same vacuous-bound rule as `range`, over the
        // satisfier-set's cardinality.
        let messages =
            inadmissibilities(&Predicate::Count { min: 3, max: 1 }, &Locus::Document, &[]);
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("min 3 greater than max 1"));

        // Equal endpoints included: a well-ordered bound is admissible.
        for (min, max) in [(0, 5), (2, 2)] {
            assert!(
                inadmissibilities(&Predicate::Count { min, max }, &Locus::Document, &[]).is_empty(),
                "[{min}, {max}] is admissible"
            );
        }
    }

    #[test]
    fn a_membership_clause_with_an_empty_target_is_inadmissible() {
        let empty_target = Predicate::Membership {
            field: "model".to_string(),
            target: String::new(),
        };
        let messages = inadmissibilities(&empty_target, &Locus::Document, &[]);
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("empty target"));

        let named = Predicate::Membership {
            field: "model".to_string(),
            target: "approved-models".to_string(),
        };
        assert!(inadmissibilities(&named, &Locus::Document, &[]).is_empty());
    }

    #[test]
    fn a_degree_clause_with_no_direction_is_inadmissible() {
        // A `degree` clause bounding neither direction constrains nothing —
        // vacuous, like an empty-list clause.
        let no_direction = Predicate::Degree {
            incoming: None,
            outgoing: None,
        };
        let messages = inadmissibilities(&no_direction, &Locus::Document, &[]);
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("no incoming or outgoing bound"));

        // At least one bounded direction is admissible — the routed (incoming) and
        // self-registering (outgoing-only) idioms both stand.
        let routed = Predicate::Degree {
            incoming: Some(EdgeBound {
                min: Some(1),
                max: None,
            }),
            outgoing: None,
        };
        assert!(inadmissibilities(&routed, &Locus::Document, &[]).is_empty());
    }

    #[test]
    fn a_degree_clause_with_an_inverted_direction_bound_is_inadmissible() {
        let inverted = Predicate::Degree {
            incoming: Some(EdgeBound {
                min: Some(5),
                max: Some(1),
            }),
            outgoing: None,
        };
        let messages = inadmissibilities(&inverted, &Locus::Document, &[]);
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("incoming"));
    }

    /// The set predicates, each in an otherwise well-formed shape so nothing but a
    /// fence could indict them.
    fn set_predicates() -> Vec<Predicate> {
        vec![
            Predicate::Count { min: 1, max: 3 },
            Predicate::Unique {
                field: "name".to_string(),
            },
            Predicate::Membership {
                field: "model".to_string(),
                target: "approved-models".to_string(),
            },
            Predicate::Degree {
                incoming: Some(EdgeBound {
                    min: Some(1),
                    max: None,
                }),
                outgoing: None,
            },
            Predicate::Kind {
                kind: "skill".to_string(),
            },
        ]
    }

    #[test]
    fn the_one_predicate_no_judge_decides_is_inadmissible_as_a_clause() {
        // `dependency-exists` names no decidable reference syntax or extractor, so no
        // projection carries the fact it would range over and it could only ever answer
        // `Indeterminate`. The fence makes the contract fail loud rather than let the
        // clause read as a green pass it never ran.
        let diags = admissibility(
            &contract(ClauseSeverity::Required, Predicate::DependencyExists),
            &Locus::Document,
        );
        assert_eq!(diags.len(), 1, "got: {diags:?}");
        assert_eq!(
            diags[0].rule, "skill.dependency-exists",
            "the finding must name the predicate"
        );
        assert_eq!(
            diags[0].severity,
            Severity::Error,
            "an inadmissible contract fails the run"
        );
    }

    #[test]
    fn a_set_predicate_is_admissible_as_a_clause_on_any_contract() {
        // The set predicates bind to a selection, and a kind's own contract declares
        // one — its whole member population. `judge` (and `crate::graph::degree`) judge
        // them over it, so there is nothing here for a fence to stand in for: a well-
        // formed set clause is admissible wherever it is declared, and only vacuity can
        // indict it.
        for predicate in set_predicates() {
            let diags = admissibility(
                &contract(ClauseSeverity::Required, predicate.clone()),
                &Locus::Document,
            );
            assert!(
                diags.is_empty(),
                "`{}` is judged over whatever selection its contract binds to, got: {diags:?}",
                predicate.key()
            );
        }
    }

    #[test]
    fn a_body_shaped_predicate_bound_to_an_embedded_kind_is_inadmissible() {
        // The lift leaves an embedded member's headings/sections/source-directory empty, so
        // each of these returns one fixed answer over every member of the kind:
        // `section_contains` finds no section to indict, `require_sections` misses every
        // named heading, `name-matches-dir` has no directory to compare. Bound where
        // nothing can decide them, they fail admissibility rather than read as checks that
        // ran. `extent` is not among them — a composed member's span is captured at emit —
        // and is tested separately.
        let embedded = Locus::Embedded("citation".to_string());
        for predicate in [
            Predicate::RequireSections {
                sections: vec!["Usage".to_string()],
            },
            Predicate::SectionContains {
                heading: "Usage".to_string(),
                marker: "example".to_string(),
            },
            Predicate::NameMatchesDir,
        ] {
            let key = predicate.key();
            let diags = admissibility(
                &contract(ClauseSeverity::Required, predicate.clone()),
                &embedded,
            );
            assert_eq!(diags.len(), 1, "`{key}` must be fenced, got: {diags:?}");
            assert_eq!(
                diags[0].rule,
                format!("skill.{key}"),
                "the finding names the predicate"
            );
            assert!(
                diags[0].message.contains("citation"),
                "the finding names the kind it is bound to, got: {}",
                diags[0].message
            );
            assert_eq!(
                diags[0].severity,
                Severity::Error,
                "an inadmissible contract fails the run"
            );

            let at_locus = admissibility(
                &contract(ClauseSeverity::Required, predicate),
                &Locus::Document,
            );
            assert!(
                at_locus.is_empty(),
                "`{key}` decides over a member that owns a document, got: {at_locus:?}"
            );
        }
    }

    #[test]
    fn a_leaf_reading_predicate_bound_to_an_embedded_kind_still_judges() {
        // The fence's line is the feature read, not the predicate's family.
        // `must_define` looks body-shaped but resolves as field presence, which an
        // embedded member's leaves answer — so it stays admissible, as do the field and
        // placement predicates the embedded dispatcher exists to run.
        let embedded = Locus::Embedded("citation".to_string());
        for predicate in [
            Predicate::MustDefine {
                marker: "source".to_string(),
            },
            Predicate::Required {
                field: "source".to_string(),
            },
            Predicate::FormatPlacesEdges,
        ] {
            let key = predicate.key();
            let diags = admissibility(&contract(ClauseSeverity::Required, predicate), &embedded);
            assert!(
                diags.is_empty(),
                "`{key}` decides over an embedded member's leaves, got: {diags:?}"
            );
        }
    }

    #[test]
    fn an_extent_clause_bound_to_an_embedded_kind_reads_the_captured_span() {
        // Admissible over an embedded kind now: a composed member's rendered span is
        // captured at emit and rides its row, so the bodyless fence no longer stands in.
        let embedded = Locus::Embedded("citation".to_string());
        let extent = Predicate::Extent {
            unit: ExtentUnit::Lines,
            max: 2,
            whole: false,
        };
        assert!(
            admissibility(
                &contract(ClauseSeverity::Required, extent.clone()),
                &embedded
            )
            .is_empty(),
            "a captured span makes `extent` decidable over an embedded member",
        );

        let carrier = contract(ClauseSeverity::Required, extent.clone());

        // A member carrying a captured span decides against the bound — over is a finding,
        // within holds — the same algebra a file member's extent runs.
        let mut over = features("over", &[], 0, None);
        over.rendered_lines = Some(3);
        assert!(matches!(
            decide(&carrier, &extent, &over, std::slice::from_ref(&over)),
            Outcome::Violated(_)
        ));
        let mut within = features("within", &[], 0, None);
        within.rendered_lines = Some(2);
        assert!(matches!(
            decide(&carrier, &extent, &within, std::slice::from_ref(&within)),
            Outcome::Holds
        ));

        // A member no format rendered — a layout host read off source — carries no span, so
        // its extent is undecidable rather than a zero read as a pass, the exact defect the
        // capture names. `Indeterminate`, distinct from the `Holds` a real zero would earn.
        let mut unrendered = features("unrendered", &[], 0, None);
        unrendered.rendered_lines = None;
        unrendered.rendered_chars = None;
        assert!(matches!(
            decide(
                &carrier,
                &extent,
                &unrendered,
                std::slice::from_ref(&unrendered)
            ),
            Outcome::Indeterminate
        ));
    }

    // ---- the selection grain ----------------------------------------------
    //
    // The same algebra the opt-in selection reaches (`crate::roster`'s cases), over the
    // *universal* binding: a kind's whole member population. The quantifier is the
    // clause's grain, so nothing about the judging is per-selector.

    /// A by-kind selection over `members` (each of kind `kind`), carrying one
    /// required-severity clause — the whole surface a kind's contract declares to bind a
    /// set predicate to its own population.
    fn kind_selection<'a>(
        kind: &'a str,
        members: &'a [Features],
        predicate: Predicate,
    ) -> Selection<'a> {
        Selection {
            selector: Selector::Kind(kind.to_string()),
            clauses: vec![clause(kind, ClauseSeverity::Required, predicate)],
            members: members.iter().map(|features| (kind, features)).collect(),
        }
    }

    #[test]
    fn count_is_whole_grain_over_a_by_kind_selection() {
        // The kind's population *is* the selection, so `count` bounds how many members
        // the kind has — one verdict over the set, never one per member.
        let members = [
            features("plan", &[], 1, None),
            features("ship", &[], 1, None),
        ];
        assert!(
            judge(&[kind_selection(
                "skill",
                &members,
                Predicate::Count { min: 1, max: 2 }
            )])
            .is_empty(),
            "two members inside a [1, 2] band is clean"
        );

        let diags = judge(&[kind_selection(
            "skill",
            &members,
            Predicate::Count { min: 1, max: 1 },
        )]);
        assert_eq!(diags.len(), 1, "one finding for the set, got: {diags:?}");
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, "skill.count");
        assert_eq!(diags[0].artifact, "skill");
        assert!(diags[0].message.contains("kind `skill`"));
        assert!(diags[0].message.contains("[1, 1]"));
        assert!(diags[0].message.contains("plan") && diags[0].message.contains("ship"));
    }

    #[test]
    fn an_empty_by_kind_selection_fires_its_count_floor() {
        // The case the member-grain loop structurally cannot reach: with no members
        // there is no artifact to iterate, yet a `count` floor of one is exactly what a
        // kind declares to demand the population exist.
        let diags = judge(&[kind_selection(
            "skill",
            &[],
            Predicate::Count { min: 1, max: 9 },
        )]);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.count");
        assert!(diags[0].message.contains("selects 0 member(s)"));
    }

    #[test]
    fn unique_is_whole_grain_over_a_by_kind_selection() {
        let shared = [
            features("plan", &[("model", scalar("opus"))], 1, None),
            features("ship", &[("model", scalar("opus"))], 1, None),
        ];
        let diags = judge(&[kind_selection(
            "skill",
            &shared,
            Predicate::Unique {
                field: "model".to_string(),
            },
        )]);
        assert_eq!(diags.len(), 1, "one finding per shared value");
        assert_eq!(diags[0].rule, "skill.unique");
        assert!(diags[0].message.contains("opus"));
        assert!(diags[0].message.contains("plan") && diags[0].message.contains("ship"));

        let distinct = [
            features("plan", &[("model", scalar("opus"))], 1, None),
            features("ship", &[("model", scalar("sonnet"))], 1, None),
        ];
        assert!(
            judge(&[kind_selection(
                "skill",
                &distinct,
                Predicate::Unique {
                    field: "model".to_string(),
                },
            )])
            .is_empty()
        );
    }

    #[test]
    fn membership_draws_a_by_kind_selections_allowed_set_from_an_opt_in_selection() {
        // The two selectors meet: the constrained selection is a kind's population, the
        // set its values are drawn from is a requirement's opt-in selection. `target`
        // resolves off the sibling selections, so neither binding needs to know the
        // other's machinery.
        let skills = [
            features("plan", &[("model", scalar("opus"))], 1, None),
            features("ship", &[("model", scalar("gpt"))], 1, None),
        ];
        let approved = [features(
            "approved-a",
            &[("model", scalar("opus"))],
            1,
            None,
        )];
        let selections = [
            kind_selection(
                "skill",
                &skills,
                Predicate::Membership {
                    field: "model".to_string(),
                    target: "approved-model".to_string(),
                },
            ),
            Selection {
                selector: Selector::OptIn("approved-model".to_string()),
                clauses: Vec::new(),
                members: approved.iter().map(|f| ("manifest", f)).collect(),
            },
        ];
        let diags = judge(&selections);
        assert_eq!(diags.len(), 1, "only `gpt` is outside the derived set");
        assert_eq!(diags[0].rule, "skill.membership");
        assert_eq!(diags[0].artifact, "skill");
        assert!(diags[0].message.contains("ship") && diags[0].message.contains("gpt"));
    }

    #[test]
    fn kind_is_each_grain_over_a_by_kind_selection() {
        // The narrowing clause over a selection whose members are all of the kind it
        // names holds of every one; naming another kind indicts every member, one
        // finding each.
        let members = [
            features("plan", &[], 1, None),
            features("ship", &[], 1, None),
        ];
        assert!(
            judge(&[kind_selection(
                "skill",
                &members,
                Predicate::Kind {
                    kind: "skill".to_string()
                }
            )])
            .is_empty()
        );

        let diags = judge(&[kind_selection(
            "skill",
            &members,
            Predicate::Kind {
                kind: "rule".to_string(),
            },
        )]);
        assert_eq!(diags.len(), 2, "each grain: one finding per member");
        assert!(diags.iter().all(|d| d.rule == "skill.kind"));
        assert!(diags[0].message.contains("`plan` is kind `skill`"));
    }

    #[test]
    fn a_selection_clause_carries_its_own_severity_and_guidance() {
        // The engine never decides error-versus-warning here either: an advisory set
        // clause reports, and its colocated guidance rides the violation.
        let mut selection = kind_selection("skill", &[], Predicate::Count { min: 1, max: 9 });
        selection.clauses[0].severity = ClauseSeverity::Advisory;
        selection.clauses[0].guidance = Some("ship at least one skill".to_string());
        let diags = judge(&[selection]);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Warn);
        assert!(!any_error(&diags));
        assert_eq!(
            diags[0].guidance.as_deref(),
            Some("ship at least one skill")
        );
    }

    #[test]
    fn a_member_grain_clause_on_a_selection_is_left_to_validate() {
        // The two judges split one contract: `judge` reads the clauses that range over
        // the selection and leaves the rest to `validate`, so a kind's whole contract
        // can ride the selection without double-reporting the member-grain half.
        let members = [features("plan", &[], 1, None)];
        let mut selection = kind_selection(
            "skill",
            &members,
            Predicate::Required {
                field: "model".to_string(),
            },
        );
        assert!(judge(std::slice::from_ref(&selection)).is_empty());

        // …and the member-grain judge is the mirror image: it skips the set clause.
        selection.clauses[0].predicate = Predicate::Count { min: 9, max: 9 };
        assert!(
            validate(
                &contract(
                    ClauseSeverity::Required,
                    Predicate::Count { min: 9, max: 9 }
                ),
                &members,
            )
            .is_empty(),
            "`validate` sees one member of the selection at a time — the set clause is \
             not its verdict to reach"
        );
    }

    #[test]
    fn every_predicate_is_judged_at_exactly_one_grain() {
        // The honest bound, whole: a predicate either ranges over the selection — where
        // `judge`/`crate::graph::degree` decide it and this per-member table must stay
        // out of the way — or it reaches a verdict here. A predicate that did neither
        // would read as a green pass it never ran.
        let demo = features("demo", &[("model", scalar("opus"))], 1, Some("demo"));
        for predicate in set_predicates() {
            assert!(
                predicate.ranges_over_selection(),
                "`{}` is judged over a selection",
                predicate.key()
            );
            assert!(
                matches!(
                    decide(
                        &contract(ClauseSeverity::Required, predicate.clone()),
                        &predicate,
                        &demo,
                        std::slice::from_ref(&demo),
                    ),
                    Outcome::Indeterminate
                ),
                "`{}` must not answer at the member grain — its selection is not one member",
                predicate.key()
            );
        }
    }

    #[test]
    fn no_admissible_predicate_reaches_indeterminate_at_conformance() {
        // The member-grain vocabulary, whole. Every predicate an admissible contract can
        // carry must reach a verdict over this projection; the set predicates are judged
        // over their selection instead (above), and the fenced one never reaches
        // conformance at all.
        let demo = features("demo", &[("model", scalar("opus"))], 1, Some("demo"));
        let vocabulary = vec![
            Predicate::Required {
                field: "model".to_string(),
            },
            Predicate::Optional {
                field: "model".to_string(),
            },
            Predicate::Type {
                field: "model".to_string(),
                kinds: BTreeSet::from([ValueType::String]),
            },
            Predicate::MinLen {
                field: "model".to_string(),
                min: 1,
            },
            Predicate::MaxLen {
                field: "model".to_string(),
                max: 9,
            },
            Predicate::Range {
                field: "model".to_string(),
                min: 0.0,
                max: 9.0,
            },
            Predicate::Enum {
                field: "model".to_string(),
                values: vec!["opus".to_string()],
            },
            Predicate::Deny {
                field: "model".to_string(),
                values: vec!["claude".to_string()],
            },
            Predicate::ForbiddenKeys {
                keys: vec!["globs".to_string()],
            },
            Predicate::AllowedChars {
                field: "model".to_string(),
                charset: slug_charset(),
            },
            Predicate::Extent {
                unit: ExtentUnit::Lines,
                max: 9,
                whole: false,
            },
            Predicate::RequireSections {
                sections: vec!["Usage".to_string()],
            },
            Predicate::MustDefine {
                marker: "model".to_string(),
            },
            Predicate::SectionContains {
                heading: "Decision".to_string(),
                marker: "Rejected".to_string(),
            },
            Predicate::NameMatchesDir,
            Predicate::UniqueName,
            Predicate::GlobValid {
                field: "model".to_string(),
            },
            Predicate::FormatPlacesEdges,
            Predicate::ClosedKeys,
        ];

        // Each predicate is judged inside a contract that also declares `model` — the demo's
        // one key. It is inert for every predicate but `closed-keys`, whose allow-list is
        // its siblings: alone it would be the vacuous clause admissibility refuses, which is
        // a fact about the contract rather than the verdict this test is after.
        let declares_model = clause(
            "skill",
            ClauseSeverity::Required,
            Predicate::Required {
                field: "model".to_string(),
            },
        );
        for predicate in vocabulary {
            let mut carrier = contract(ClauseSeverity::Required, predicate.clone());
            carrier.clauses.push(declares_model.clone());
            assert!(
                inadmissibilities(&predicate, &Locus::Document, &carrier.clauses).is_empty(),
                "`{}` is admissible here — the fenced set is tested above",
                predicate.key()
            );
            assert!(
                !matches!(
                    decide(&carrier, &predicate, &demo, std::slice::from_ref(&demo)),
                    Outcome::Indeterminate
                ),
                "`{}` reached conformance undecided — it would read as a green pass",
                predicate.key()
            );
        }
    }

    #[test]
    fn min_len_fires_only_below_the_bound() {
        let predicate = || Predicate::MinLen {
            field: "description".to_string(),
            min: 5,
        };
        let under = features("demo", &[("description", scalar("hi"))], 1, None);
        assert_eq!(run(predicate(), under).len(), 1);

        let ok = features("demo", &[("description", scalar("plenty"))], 1, None);
        assert!(run(predicate(), ok).is_empty());
    }

    #[test]
    fn enum_fires_off_the_permitted_set() {
        let predicate = || Predicate::Enum {
            field: "status".to_string(),
            values: vec!["draft".to_string(), "active".to_string()],
        };
        let bad = features("demo", &[("status", scalar("retired"))], 1, None);
        assert_eq!(run(predicate(), bad).len(), 1);

        let good = features("demo", &[("status", scalar("active"))], 1, None);
        assert!(run(predicate(), good).is_empty());
    }

    #[test]
    fn deny_fires_on_a_forbidden_value() {
        let predicate = || Predicate::Deny {
            field: "name".to_string(),
            values: vec!["anthropic".to_string(), "claude".to_string()],
        };
        let reserved = features("claude", &[("name", scalar("claude"))], 1, None);
        assert_eq!(run(predicate(), reserved).len(), 1);

        let fine = features("demo", &[("name", scalar("demo"))], 1, None);
        assert!(run(predicate(), fine).is_empty());
    }

    #[test]
    fn forbidden_keys_fire_once_per_present_key() {
        let predicate = || Predicate::ForbiddenKeys {
            keys: vec!["globs".to_string(), "alwaysApply".to_string()],
        };
        let legacy = features(
            "legacy",
            &[
                ("globs", scalar("**/*.rs")),
                ("alwaysApply", scalar("true")),
            ],
            1,
            None,
        );
        let diags = run(predicate(), legacy);
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.rule == "skill.forbidden_keys"));

        let clean = features("clean", &[("name", scalar("clean"))], 1, None);
        assert!(run(predicate(), clean).is_empty());
    }

    #[test]
    fn allowed_chars_fires_on_a_character_outside_the_set() {
        let predicate = || Predicate::AllowedChars {
            field: "name".to_string(),
            charset: slug_charset(),
        };
        let shouty = features("Demo_1", &[("name", scalar("Demo_1"))], 1, None);
        let diags = run(predicate(), shouty);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.allowed_chars");
        // The offending characters, deduped and sorted, ride in the message.
        assert!(diags[0].message.contains('D'));
        assert!(diags[0].message.contains('_'));

        let slug = features("demo-1", &[("name", scalar("demo-1"))], 1, None);
        assert!(run(predicate(), slug).is_empty());
    }

    #[test]
    fn glob_valid_fires_once_per_unparseable_glob_and_passes_valid_brace_globs() {
        let predicate = || Predicate::GlobValid {
            field: "paths".to_string(),
        };

        // A list whose entries are two unparseable globs (each an unclosed `[`) and
        // one valid one: one finding per broken entry, none for the valid one.
        let broken = features(
            "demo",
            &[("paths", json!(["src/**/*.rs", "[", "a[b"]))],
            1,
            None,
        );
        let diags = run(predicate(), broken);
        assert_eq!(diags.len(), 2, "one finding per unparseable glob");
        assert!(diags.iter().all(|d| d.rule == "skill.glob-valid"));

        // Brace expansion is in scope — a valid `{a,b}` alternation passes.
        let valid = features(
            "demo",
            &[("paths", json!(["src/**/*.{rs,toml}", "docs/*.md"]))],
            1,
            None,
        );
        assert!(run(predicate(), valid).is_empty());

        // A lone scalar is read as a single glob — a broken one fires.
        let scalar_bad = features("demo", &[("paths", scalar("["))], 1, None);
        assert_eq!(run(predicate(), scalar_bad).len(), 1);

        // An absent field is the `required` clause's concern, not this one's.
        let absent = features("demo", &[], 1, None);
        assert!(run(predicate(), absent).is_empty());
    }

    #[test]
    fn extent_each_grain_fires_only_past_the_budget() {
        let predicate = || Predicate::Extent {
            unit: ExtentUnit::Lines,
            max: 500,
            whole: false,
        };
        // The `features` helper mirrors its line count into the rendered extent.
        let long = features("demo", &[], 501, None);
        assert_eq!(run(predicate(), long).len(), 1);

        // Exactly at the bound is "at most max" — it holds.
        let at_bound = features("demo", &[], 500, None);
        assert!(run(predicate(), at_bound).is_empty());
    }

    #[test]
    fn must_define_fires_when_the_marker_is_absent() {
        let predicate = || Predicate::MustDefine {
            marker: "disable-model-invocation".to_string(),
        };
        let missing = features("demo", &[("name", scalar("demo"))], 1, None);
        assert_eq!(run(predicate(), missing).len(), 1);

        let defined = features(
            "demo",
            &[("disable-model-invocation", scalar("true"))],
            1,
            None,
        );
        assert!(run(predicate(), defined).is_empty());
    }

    #[test]
    fn name_matches_dir_fires_on_a_mismatch() {
        let mismatch = features("demo", &[("name", scalar("demo"))], 1, Some("other"));
        let diags = run(Predicate::NameMatchesDir, mismatch);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.name-matches-dir");

        let aligned = features("demo", &[("name", scalar("demo"))], 1, Some("demo"));
        assert!(run(Predicate::NameMatchesDir, aligned).is_empty());
    }

    #[test]
    fn unique_name_fires_for_each_colliding_artifact() {
        let a = features("dup", &[("name", scalar("dup"))], 1, None);
        let b = features("dup", &[("name", scalar("dup"))], 1, None);
        let c = features("solo", &[("name", scalar("solo"))], 1, None);
        let diags = validate(
            &contract(ClauseSeverity::Required, Predicate::UniqueName),
            &[a, b, c],
        );
        // Both `dup` artifacts report; `solo` does not.
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.artifact == "dup"));
    }

    #[test]
    fn require_sections_fires_per_missing_heading_and_is_silent_when_all_present() {
        let predicate = || Predicate::RequireSections {
            sections: vec!["Usage".to_string(), "Examples".to_string()],
        };

        // One finding per named section with no matching heading.
        let missing = features_with_headings("demo", &["Usage"]);
        let diags = run(predicate(), missing);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.require_sections");
        assert_eq!(diags[0].artifact, "demo");
        assert!(diags[0].message.contains("Examples"));

        // Every named heading present (order and extras are irrelevant): silent.
        let complete = features_with_headings("demo", &["Examples", "Intro", "Usage"]);
        assert!(run(predicate(), complete).is_empty());
    }

    #[test]
    fn dependency_exists_is_inadmissible() {
        // `dependency-exists` is held back — it names no decidable reference
        // syntax or extractor, so a hand-authored clause must fail admissibility
        // loudly rather than silently decide `Indeterminate`. The fence is mirrored on the full `pattern` primitive.
        let held = contract(ClauseSeverity::Required, Predicate::DependencyExists);
        let diags = admissibility(&held, &Locus::Document);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.dependency-exists");
        assert_eq!(diags[0].severity, Severity::Error);
        // The finding names the contract it indicts.
        assert_eq!(diags[0].artifact, "skill");
        assert!(any_error(&diags));

        // A clause's declared severity is irrelevant: it is inadmissible even
        // when marked advisory, because an inadmissible contract cannot be used.
        let advisory = contract(ClauseSeverity::Advisory, Predicate::DependencyExists);
        assert_eq!(admissibility(&advisory, &Locus::Document).len(), 1);
    }

    #[test]
    fn an_empty_enum_clause_is_inadmissible() {
        // A clause that lists no values can never decide anything — vacuous, so
        // the contract carrying it fails admissibility (an error, exit non-zero).
        let empty_enum = contract(
            ClauseSeverity::Required,
            Predicate::Enum {
                field: "status".to_string(),
                values: Vec::new(),
            },
        );
        let diags = admissibility(&empty_enum, &Locus::Document);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "skill.enum");
        assert_eq!(diags[0].severity, Severity::Error);
        // The finding names the contract it indicts.
        assert_eq!(diags[0].artifact, "skill");
        assert!(any_error(&diags));
    }

    #[test]
    fn an_empty_list_clause_of_every_list_kind_is_inadmissible() {
        // Each list-bearing predicate is inadmissible when its list is empty; the
        // finding's `rule` names the offending clause.
        for (predicate, key) in [
            (
                Predicate::Deny {
                    field: "name".to_string(),
                    values: Vec::new(),
                },
                "deny",
            ),
            (
                Predicate::ForbiddenKeys { keys: Vec::new() },
                "forbidden_keys",
            ),
            (
                Predicate::RequireSections {
                    sections: Vec::new(),
                },
                "require_sections",
            ),
            // A `type` over no kinds is the same vacuity: no value the lattice can
            // carry satisfies it, so it admits nothing at any locus.
            (
                Predicate::Type {
                    field: "keywords".to_string(),
                    kinds: BTreeSet::new(),
                },
                "type",
            ),
        ] {
            let diags = admissibility(
                &contract(ClauseSeverity::Required, predicate),
                &Locus::Document,
            );
            assert_eq!(diags.len(), 1, "{key} with an empty list should fire once");
            assert_eq!(diags[0].rule, format!("skill.{key}"));
            assert_eq!(diags[0].severity, Severity::Error);
        }
    }

    #[test]
    fn a_well_formed_contract_is_admissible() {
        // Non-empty lists, and the non-list primitives, carry nothing for
        // admissibility to reject — the multi-clause representative is admissible.
        let clauses = vec![
            clause(
                "skill",
                ClauseSeverity::Required,
                Predicate::Enum {
                    field: "status".to_string(),
                    values: vec!["draft".to_string(), "active".to_string()],
                },
            ),
            clause(
                "skill",
                ClauseSeverity::Required,
                Predicate::Deny {
                    field: "name".to_string(),
                    values: vec!["claude".to_string()],
                },
            ),
            clause(
                "skill",
                ClauseSeverity::Required,
                Predicate::ForbiddenKeys {
                    keys: vec!["globs".to_string()],
                },
            ),
            clause(
                "skill",
                ClauseSeverity::Advisory,
                Predicate::RequireSections {
                    sections: vec!["Usage".to_string()],
                },
            ),
            clause(
                "skill",
                ClauseSeverity::Required,
                Predicate::Required {
                    field: "name".to_string(),
                },
            ),
            clause("skill", ClauseSeverity::Required, Predicate::NameMatchesDir),
        ];
        let contract = Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses,
        };
        assert!(admissibility(&contract, &Locus::Document).is_empty());
    }

    #[test]
    fn declared_severity_maps_required_to_error_and_advisory_to_warn() {
        let violating = features("demo", &[], 1, None);
        let predicate = || Predicate::Required {
            field: "name".to_string(),
        };

        let required = validate(
            &contract(ClauseSeverity::Required, predicate()),
            std::slice::from_ref(&violating),
        );
        assert_eq!(required[0].severity, Severity::Error);

        let advisory = validate(
            &contract(ClauseSeverity::Advisory, predicate()),
            std::slice::from_ref(&violating),
        );
        assert_eq!(advisory[0].severity, Severity::Warn);
    }

    #[test]
    fn an_all_advisory_run_yields_no_error() {
        // Every clause advisory; the artifact violates all of them.
        let clauses = vec![
            clause(
                "skill",
                ClauseSeverity::Advisory,
                Predicate::Required {
                    field: "name".to_string(),
                },
            ),
            clause(
                "skill",
                ClauseSeverity::Advisory,
                Predicate::Extent {
                    unit: ExtentUnit::Lines,
                    max: 10,
                    whole: false,
                },
            ),
        ];
        let contract = Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses,
        };
        let violating = features("demo", &[], 99, None);

        let diags = validate(&contract, std::slice::from_ref(&violating));
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.severity == Severity::Warn));
        // The whole point of `advisory`: it reports without blocking the gate.
        assert!(!any_error(&diags));
    }

    #[test]
    fn a_conforming_artifact_against_a_multi_clause_contract_is_clean() {
        let clauses = vec![
            clause(
                "skill",
                ClauseSeverity::Required,
                Predicate::Required {
                    field: "name".to_string(),
                },
            ),
            clause(
                "skill",
                ClauseSeverity::Required,
                Predicate::MaxLen {
                    field: "name".to_string(),
                    max: 64,
                },
            ),
            clause(
                "skill",
                ClauseSeverity::Required,
                Predicate::AllowedChars {
                    field: "name".to_string(),
                    charset: slug_charset(),
                },
            ),
            clause(
                "skill",
                ClauseSeverity::Required,
                Predicate::ForbiddenKeys {
                    keys: vec!["globs".to_string()],
                },
            ),
            clause(
                "skill",
                ClauseSeverity::Advisory,
                Predicate::Extent {
                    unit: ExtentUnit::Lines,
                    max: 500,
                    whole: false,
                },
            ),
            clause("skill", ClauseSeverity::Required, Predicate::NameMatchesDir),
        ];
        let contract = Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses,
        };
        let conforming = features("demo", &[("name", scalar("demo"))], 12, Some("demo"));

        assert!(validate(&contract, std::slice::from_ref(&conforming)).is_empty());
    }
}
