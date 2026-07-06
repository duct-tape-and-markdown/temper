//! The harness assembly's domain types — [`Requirement`], [`Edge`], [`Reachability`],
//! [`Authority`], and the set-scope predicate shapes ([`CountBound`], [`DegreeBound`],
//! [`EdgeBound`], [`Membership`]) — and [`effective`], which composes the lock's
//! per-clause severity overrides onto the embedded by-kind floor
//! (`specs/architecture/20-surface.md`, "The lock and drift — one vocabulary").
//!
//! There is no reader in this module: every value here is populated from the lock's
//! declaration rows (`crate::drift::Declarations`), the sole producer since `emit`
//! compiles the SDK program. These are the shared shapes the gate lifts lock rows
//! into and [`crate::roster`]/[`crate::graph`]/[`crate::coverage`] range over —
//! the manifest era's reader (`TEMPER-TOML-ZERO`) retired with this file's parser.

use std::collections::BTreeMap;

use crate::contract::{self, Contract};
use crate::drift::ClauseRow;

/// The assembly's declared **surface-authority posture** — how firmly the surface owns its
/// projections (`specs/architecture/20-surface.md`, "surface authority is a declared posture,
/// never a baked stance"): a closed vocabulary the author declares, never a stance
/// temper bakes in. Defaults to [`Shared`](Authority::Shared).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Authority {
    /// Direct on-disk edits stay first-class — `re-add` reconciles, guards inform and
    /// route. The default: temper fabricates no enforcement the author did not ask for
    /// (`00-intent.md` law 4).
    #[default]
    Shared,
    /// The author opts into enforcement — the managed-by note and the guard hook's
    /// write-boundary block (the consumers' concern, not this slice's).
    Surface,
}

/// A declared **edge relationship** — a kind capability declared on the owning kind's
/// members (`specs/architecture/15-kinds.md`). The owning kind is the edge *source*
/// (the implicit `from`); the relationship names its reference `field` and the target
/// `to` kind. [`crate::graph`] reads the field off each source artifact into edges,
/// then flags any route that resolves to no artifact of the target kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    /// The reference field read off each source artifact's frontmatter (via the
    /// `extra` catch-all). Its scalar value (or each element of a list value) names
    /// the target artifact.
    pub field: String,
    /// The artifact kind that owns the reference field — the edge *source*. A `from`
    /// naming an unmodeled kind yields no source artifacts, so the edge is inert.
    pub from: String,
    /// The artifact kind the reference resolves to — the edge *target*. The target
    /// kind must be one `temper` models, else no route can resolve (a
    /// graph-admissibility concern, [`crate::graph`]).
    pub to: String,
}

/// A named **requirement** — the harness's named obligation, declared in the
/// assembly's `[requirement.<name>]` (or lifted from a member's own published
/// obligation, `specs/architecture/10-contracts.md`). **Every facet is optional
/// except the name.** Fill is by the artifact's opt-in `satisfies` alone — there is
/// no name-`match` selector.
///
/// `temper` **never interprets `means`** — it is authored intent the surface carries,
/// never a thing the engine judges (`00-intent.md` law 3). The decidable shadow is
/// what `check` gates: [`crate::coverage`] over the `satisfies` edges,
/// [`crate::roster`]/[`crate::graph`] over the **satisfier set** (the artifacts of
/// its `kind` that opt in via `satisfies`).
#[derive(Debug, Clone, PartialEq)]
pub struct Requirement {
    /// The requirement's name.
    pub name: String,
    /// The authored *intent*, stated in meaning not predicates. Carried verbatim and
    /// **never interpreted** (`00-intent.md` law 3).
    pub means: Option<String>,
    /// The artifact kind that may fill the requirement — the `kind` typing facet.
    /// Absent ⇒ **kind-blind**: any artifact that opts in fills it.
    pub kind: Option<String>,
    /// The package the filling artifact must conform to — the `package` typing facet.
    /// A package named **by name**, resolved through [`PackageResolver`] — never
    /// inline clauses. Composes with `kind`: the filler is checked by its own kind's
    /// bound package *and* this named one. Absent ⇒ no package constraint.
    pub package: Option<String>,
    /// Whether an unfilled requirement is a gate-blocking violation. Absent ⇒ `false`
    /// (`temper` never fabricates a gate the author did not declare — `00-intent.md`
    /// law 4). Mutually exclusive with [`count`](Requirement::count): `required` is
    /// the ≥1-satisfier shorthand, `count` the general cardinality form.
    pub required: bool,
    /// The set-scope `count` predicate (`specs/architecture/45-governance.md`): the satisfier-set
    /// size must land in `[min, max]`. Absent ⇒ `None`. The general form of
    /// `required`; the two are mutually exclusive.
    pub count: Option<CountBound>,
    /// The set-scope `unique` predicate (`specs/architecture/45-governance.md`): each named field's
    /// extracted scalar must not repeat across the satisfiers. Absent ⇒ empty (no
    /// uniqueness gate). Checked in [`crate::roster`].
    pub unique: Vec<String>,
    /// The set-scope `membership` predicate (`specs/architecture/45-governance.md`): a declared
    /// field of every satisfier (S₁) must lie in a *corpus-derived* set drawn from a
    /// second satisfier set (S₂). Absent ⇒ `None`. Checked in [`crate::roster`].
    pub membership: Option<Membership>,
    /// The graph-scope `degree` bound (`specs/architecture/45-governance.md`): the in/out edge
    /// count of every satisfier must land in the declared bound. Declared on the
    /// requirement but ranging over the *edge* graph, so checked in [`crate::graph`],
    /// not [`crate::roster`]. Absent ⇒ `None`.
    pub degree: Option<DegreeBound>,
    /// An optional external verifier for the behavioral remainder (`verified_by`).
    /// Stored verbatim; whether it *resolves* is an admissibility check.
    pub verified_by: Option<String>,
}

/// An inclusive `[min, max]` bound on the cardinality of a requirement's satisfier
/// set — the set-scope `count` predicate (`specs/architecture/45-governance.md`). An inverted
/// `min > max` bound admits nothing and is rejected as inadmissible
/// ([`crate::roster`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountBound {
    /// The inclusive lower bound on the satisfier-set size.
    pub min: usize,
    /// The inclusive upper bound on the satisfier-set size.
    pub max: usize,
}

/// The graph-scope `degree` predicate — an inclusive bound on the **incoming** and/or
/// **outgoing** edge count of every satisfier over the harness reference graph
/// (`specs/architecture/45-governance.md`). At least one direction is present (an empty `degree`
/// constrains nothing — rejected at parse). Decided against the resolved arcs in
/// [`crate::graph`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DegreeBound {
    /// The bound on a satisfier node's incoming edge count (how many nodes point at
    /// it). Absent ⇒ `None` (incoming degree is unconstrained).
    pub incoming: Option<EdgeBound>,
    /// The bound on a satisfier node's outgoing edge count (how many nodes it points
    /// at). Absent ⇒ `None` (outgoing degree is unconstrained).
    pub outgoing: Option<EdgeBound>,
}

/// An inclusive `[min, max]` bound on a node's edge count in one direction, each
/// endpoint optional so the single-sided cases the worked example needs are
/// expressible: absent `min` ⇒ no lower bound (0), absent `max` ⇒ unbounded above
/// (the routed "≥ 1" case). At least one endpoint is present — an endpoint-less
/// bound admits every degree, and an inverted `min > max` admits none; both are
/// vacuous clauses the author cannot have meant, so both are rejected at parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeBound {
    /// The inclusive lower bound on the edge count. `None` ⇒ no lower bound.
    pub min: Option<usize>,
    /// The inclusive upper bound on the edge count. `None` ⇒ unbounded above.
    pub max: Option<usize>,
}

impl EdgeBound {
    /// Whether `degree` lands inside this inclusive bound — `min <= degree <= max`
    /// with an absent endpoint imposing no limit on that side. The decidable core of
    /// the graph-scope `degree` check (`specs/architecture/45-governance.md`).
    #[must_use]
    pub fn admits(self, degree: usize) -> bool {
        self.min.is_none_or(|min| degree >= min) && self.max.is_none_or(|max| degree <= max)
    }
}

/// A set-scope `membership` predicate over a requirement's satisfier set (S₁): a
/// declared field of every satisfier must lie in a *corpus-derived* set, not a static
/// `enum` (`specs/architecture/45-governance.md`). The allowed set is `source_feature` extracted
/// over the S₂ satisfier set — the `source_kind` artifacts that opt into the `source`
/// requirement (R₂). S₂ may name a different kind than the requirement's own, so the
/// check ranges over the whole by-kind map. Decided in [`crate::roster`].
#[derive(Debug, Clone, PartialEq)]
pub struct Membership {
    /// The field on each S₁ satisfier whose extracted scalar must be a member of the
    /// source set. A satisfier missing it carries no value to check.
    pub field: String,
    /// The source requirement `R₂` whose satisfier set (S₂) supplies the allowed
    /// values: a `source_kind` artifact enters S₂ when its `satisfies` names this.
    pub source: String,
    /// The artifact kind S₂ is drawn from. May differ from the requirement's own
    /// `kind`, so the allowed set can be drawn from another kind.
    pub source_kind: String,
    /// The feature whose extracted scalars over the S₂ satisfiers form the allowed
    /// set. A source artifact missing it contributes nothing.
    pub source_feature: String,
    /// An optional **typed reference** constraint (`conforms_to`): when set, S₂ is
    /// narrowed to the source artifacts that also conform to this **package**, named
    /// by name and resolved through [`PackageResolver`]. Absent ⇒ `None` (plain
    /// membership). Conformance is decided in [`crate::roster`].
    pub source_package: Option<String>,
}

/// The assembly's graph-scope **`reachable`** opt-in — declared in the assembly's
/// `[reachability]` table (`specs/architecture/45-governance.md`, "The world is a node —
/// reachability is a predicate"; resolved `reachability-gate-mechanism` option b).
/// Presence is the opt-in: absent, the [`crate::graph::reachable`] predicate never
/// runs (like `degree`, temper fabricates no gate the author did not declare). Its
/// `severity` is the dial a provably-dead world→member activation edge is emitted at —
/// the assembly's call, since the graph scope is the assembly's and a deliberate
/// work-in-progress dead edge (a blank-description skill) must stay the author's to
/// weigh, never a member's or a package clause's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reachability {
    /// The severity a dead activation edge gates at, in the author's `required` /
    /// `advisory` dial — mapped to the diagnostic severity through the one translation
    /// clauses use ([`crate::engine::severity_of`]).
    pub severity: contract::Severity,
}

/// Resolves a **bound package name** to its [`Contract`] against the embedded
/// built-in set (`specs/architecture/20-surface.md`). The single order every by-name
/// binding resolves through (a requirement's `package`, a `membership`'s
/// `conforms_to`), so packages **compose**. There is no on-disk `PACKAGE.md` to
/// fall back to (`specs/architecture/15-kinds.md`, "Decision: field typing lives
/// in the SDK — there is no kind file format"): a project's own package is
/// SDK-authored, not yet a path this resolver reads.
#[derive(Debug, Clone)]
pub struct PackageResolver {
    /// The built-in packages, keyed by name — the embedded floor set a bound name
    /// resolves against.
    builtins: BTreeMap<String, Contract>,
}

impl PackageResolver {
    /// Assemble a resolver over the built-in package set, keyed by name.
    #[must_use]
    pub fn new(builtins: BTreeMap<String, Contract>) -> Self {
        Self { builtins }
    }

    /// Resolve a bound package `name` to the [`Contract`] the engine validates a
    /// requirement's filler against (`specs/architecture/10-contracts.md`, the `package` typing
    /// facet's `conforms-to` half): `Some` when `name` is a built-in package; `None`
    /// when it resolves to neither, which is admissibility's finding (`names a real
    /// package`), never a thrown error, so the caller can skip conformance rather
    /// than double-report.
    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<Contract> {
        self.builtins.get(name).cloned()
    }
}

/// The effective contract for `kind`: the embedded `floor` with each clause's
/// severity overridden by a matching row in the lock's declared `clauses`
/// (`specs/architecture/20-surface.md`, "The lock and drift — one vocabulary": the
/// gate's per-kind contract sources its overrides from the lock's `ClauseRow`
/// family, never a manifest `[kind.*]` layer). A row overrides the floor clause
/// sharing its identity (predicate key + targeted field); a row naming no matching
/// floor clause contributes nothing — a `ClauseRow` carries no predicate parameter
/// beyond `field`, so it can flip an existing clause's severity but never declare a
/// wholly new one. A row whose `severity` is outside the closed vocabulary leaves
/// the floor's own severity untouched, the same tolerant read the rest of the lock
/// takes over hand-editable state.
#[must_use]
pub fn effective(clauses: &[ClauseRow], kind: &str, mut floor: Contract) -> Contract {
    // A caller may pass the qualified floor identity (`claude-code.skill`) while a
    // `ClauseRow.kind` is always the bare name (`sdk/src/kind.ts`, `key: facts.name`)
    // — resolve to the bare component, the way a bare kind lookup always has.
    let bare = kind.rsplit('.').next().unwrap_or(kind);
    for clause in &mut floor.clauses {
        let key = clause.predicate.key();
        let target = clause.predicate.target();
        let overriding = clauses
            .iter()
            .find(|row| row.kind == bare && row.predicate == key && row.field.as_deref() == target);
        if let Some(severity) = overriding.and_then(|row| severity_from_label(&row.severity)) {
            clause.severity = severity;
        }
    }
    floor
}

/// Parse a lock clause row's `severity` label into the typed [`contract::Severity`]
/// — the closed `required`/`advisory` vocabulary a bare contract's own clauses
/// declare. An out-of-vocabulary label is `None`.
fn severity_from_label(label: &str) -> Option<contract::Severity> {
    match label {
        "required" => Some(contract::Severity::Required),
        "advisory" => Some(contract::Severity::Advisory),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::contract::{Clause, Predicate, Severity};

    /// A small skill-shaped floor: a required `max_len` on `name`, a required
    /// `forbidden_keys`, and an advisory `max_lines`. Enough distinct identities to
    /// exercise override-vs-extend.
    fn floor() -> Contract {
        Contract {
            name: "skill.anthropic".to_string(),
            guidance: None,
            clauses: vec![
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::MaxLen {
                        field: "name".to_string(),
                        max: 64,
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::ForbiddenKeys {
                        keys: vec!["globs".to_string()],
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::MaxLines { max: 500 },
                },
            ],
        }
    }

    #[test]
    fn effective_with_no_clause_rows_yields_the_floor_unchanged() {
        assert_eq!(effective(&[], "skill", floor()), floor());
    }

    #[test]
    fn effective_overrides_a_floor_clauses_severity_by_matching_identity() {
        // A row sharing the floor's `forbidden_keys` identity (predicate key, no
        // targeted field) flips its severity in place — the lock's `ClauseRow`
        // family is the sole source `effective` composes from, never a manifest
        // `[kind.*]` layer.
        let row = ClauseRow {
            kind: "skill".to_string(),
            predicate: "forbidden_keys".to_string(),
            field: None,
            severity: "advisory".to_string(),
        };
        let contract = effective(&[row], "skill", floor());
        assert_eq!(contract.clauses.len(), floor().clauses.len());
        assert_eq!(contract.clauses[1].severity, Severity::Advisory);
    }

    #[test]
    fn effective_ignores_a_row_naming_a_different_kind() {
        let row = ClauseRow {
            kind: "rule".to_string(),
            predicate: "forbidden_keys".to_string(),
            field: None,
            severity: "advisory".to_string(),
        };
        assert_eq!(effective(&[row], "skill", floor()), floor());
    }

    #[test]
    fn effective_ignores_a_row_with_no_matching_floor_clause() {
        // The row names a predicate/field pair the floor doesn't carry — a
        // `ClauseRow` carries no predicate parameter beyond `field`, so there is
        // nothing to reconstruct a wholly new clause from; it contributes nothing.
        let row = ClauseRow {
            kind: "skill".to_string(),
            predicate: "min_len".to_string(),
            field: Some("name".to_string()),
            severity: "required".to_string(),
        };
        assert_eq!(effective(&[row], "skill", floor()), floor());
    }

    #[test]
    fn effective_ignores_a_row_with_an_out_of_vocabulary_severity() {
        let row = ClauseRow {
            kind: "skill".to_string(),
            predicate: "forbidden_keys".to_string(),
            field: None,
            severity: "blocking".to_string(),
        };
        assert_eq!(effective(&[row], "skill", floor()), floor());
    }

    #[test]
    fn effective_resolves_a_qualified_kind_identity_to_its_bare_component() {
        // A caller may pass the floor's qualified identity (`claude-code.skill`); a
        // `ClauseRow.kind` is always bare, so the override still applies.
        let row = ClauseRow {
            kind: "skill".to_string(),
            predicate: "forbidden_keys".to_string(),
            field: None,
            severity: "advisory".to_string(),
        };
        let contract = effective(&[row], "claude-code.skill", floor());
        assert_eq!(contract.clauses[1].severity, Severity::Advisory);
    }
}
