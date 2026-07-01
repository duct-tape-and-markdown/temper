//! Composition — layer an optional author-declared `temper.toml` over the
//! embedded by-kind floor contracts.
//!
//! Implements `specs/40-composition.md` ("Decision: the author-declared contract
//! lives in `temper.toml`, layered"). `check` gates every harness against the
//! built-in contract for each artifact kind — the **floor** (`specs/20-surface.md`,
//! "contract selection is by artifact kind"). The floor needs no author input, but
//! a built-in is only `temper`'s curated default; `00-intent.md` law 2 (the author
//! declares; built-ins are overridable *data*) is only half-real until the author
//! can declare on top of it. This module is that other half — the optional,
//! project-root `temper.toml` that **layers over** the floor.
//!
//! ## What the layer does (this tier)
//!
//! A `temper.toml` carries a `[kind.<k>]` table per artifact kind it customizes.
//! Each does up to two things, both settled here:
//!
//! - **Bind** — name the kind's package explicitly (`package = "<name>"`), a
//!   *name*, not a path. A built-in name (`skill.anthropic`, `rule`) resolves to
//!   the embedded floor; any other name resolves to a project-authored package at
//!   `.temper/packages/<name>/PACKAGE.md` (PACKAGE-DOCUMENT's loader). A name that
//!   resolves to neither — no built-in, no project package — is a load error.
//!   Omitting `package` takes the kind's built-in floor implicitly.
//! - **Extend / override / flip** — an inline `[[kind.<k>.clause]]` array of the
//!   *same* closed-vocabulary clauses a bare contract carries. Each layered clause
//!   either **overrides** the floor clause with the same identity (its predicate
//!   [`key`](crate::contract::Predicate::key) and the field it
//!   [`target`](crate::contract::Predicate::target)s) — which is how a severity
//!   flip (`required` ⟷ `advisory`) and a parameter change are both expressed — or,
//!   when no floor clause shares that identity, **extends** the floor with it.
//!
//! ## The requirement roster
//!
//! A `temper.toml` may also carry top-level `[requirement.<name>]` tables — the
//! harness-contract tier (`specs/10-contracts.md`, "Requirements — the harness's
//! named obligations"; "Decision: role and requirement are one concept"). A
//! **requirement** is the harness's named obligation, one concept carrying every
//! facet: an optional authored `means`, optional typing (`kind` / `package`), an
//! optional `required` flag, the set-scope predicates (`count` / `unique` /
//! `membership`), an optional graph-scope `degree` bound, and an optional
//! `verified_by` verifier. Each parses into a typed [`Requirement`]; **every facet
//! is optional except the name** (`specs/10-contracts.md`, "all facets optional
//! except its name"). Fill is the artifact's opt-in `satisfies` alone — there is
//! **no contract-side name-`match` selector** (a name pattern is the contract
//! guessing, eradicated; `specs/45-governance.md`, "The set scope").
//!
//! `crate::coverage` gates referential coverage over the opt-in `satisfies` edges,
//! and `crate::roster`/`crate::graph` run the set-scope predicates and the
//! graph-scope `degree` bound over each requirement's **satisfier set** — the
//! artifacts of its `kind` that opt in via `satisfies`. A malformed requirement is a
//! load error here.
//!
//! ## Registering a custom kind (not defining it)
//!
//! A `[kind.<name>]` whose name is **not** a built-in ([`crate::kind::BUILTIN_KINDS`])
//! is a **custom-kind registration** (`specs/40-composition.md`, "Decision: a custom
//! kind is an authored `.temper/` artifact, registered in the assembly"): it binds the
//! kind's package (`package = "<name>"`, its whole require-side wiring, uniform with a
//! built-in) and *points at* the authored definition under `.temper/kinds/<name>/`. The
//! definition itself — the `governs` locus, the composed extraction, the relationships
//! — lives in that `KIND.md` artifact, loaded by [`crate::kind::CustomKind`], **not**
//! inline here. The fully-inline `[kind.<name>]` definition is retired: a `governs`
//! locus, an `[[kind.<name>.extraction]]` array, or a `[[kind.<name>.clause]]` contract
//! under a kind table is now a stray key, rejected at load ([`parse_kind_layer`]) — a
//! custom kind carries no clauses, and its definition is authored, not stuffed into the
//! assembly. So every `[kind.<name>]` table parses uniformly into a [`KindLayer`]
//! (package binding ⊕ clause overrides) here; whether the name registers a custom kind
//! is the caller's concern, resolved against [`crate::kind::BUILTIN_KINDS`] with the
//! definition loaded off disk.
//!
//! ## Relationships — a kind capability, not a standalone construct
//!
//! A reference is a **kind capability**, not a free-standing table: a kind declares
//! which of its references are edges under its own `[[kind.<name>.relationships]]`
//! array (`specs/15-kinds.md`, "The entity graph is a kind capability";
//! `specs/40-composition.md`, the `.relationships` surface). The owning kind
//! `<name>` is each edge's *source* (the implicit `from`); each relationship names
//! its reference `field` and its target `to` kind. Here they are gathered off every
//! `[kind.<name>]` table in the **assembly** — a built-in kind declares its edges
//! alongside its package binding. A **custom** kind declares its edges in its authored
//! `KIND.md` definition instead ([`crate::kind::CustomKind`]), the same [`Edge`] shape,
//! so both homes parse into the identical value [`crate::graph`] consumes; assembling
//! the graph and checking route resolution live there.
//!
//! ## Closed vocabulary, end to end
//!
//! The clause array is parsed by the *same* [`crate::contract`] parser a bare
//! contract uses ([`contract::parse_clauses`]), so a layered clause naming an
//! unknown predicate is rejected at load exactly as it is in a standalone contract
//! — the author layer earns no escape hatch the floor lacks. And the effective
//! contract (floor ⊕ layer) is run through *both* greens (admissibility +
//! conformance) in `check`, so an inadmissible override — an empty `enum`, say —
//! fails admissibility on the layered result, never slipping through because the
//! floor was clean.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use toml_edit::{DocumentMut, Table};

use crate::contract::{self, Clause, Contract, ContractError};

/// The author-declared layer parsed from a project-root `temper.toml`: a per-kind
/// set of package bindings and clause overrides to apply over the bound package.
#[derive(Debug, Clone)]
pub struct AuthorLayer {
    /// The source path, retained so a layering error (a bound package that
    /// resolves to nothing) can name the file it came from.
    path: PathBuf,
    /// The per-kind layers, keyed by artifact kind (`skill`, `rule`, …). A kind
    /// the author did not name falls through to the floor unchanged.
    kinds: BTreeMap<String, KindLayer>,
    /// The named requirements parsed from top-level `[requirement.<name>]` tables,
    /// keyed by requirement name — the harness's named obligations
    /// (`specs/10-contracts.md`, "Requirements — the harness's named obligations";
    /// "Decision: role and requirement are one concept"). Its own namespace,
    /// distinct from the `kind` map. Empty when the `temper.toml` declares none.
    /// `crate::coverage` gates coverage over the `satisfies` edges; `crate::roster`
    /// and `crate::graph` run the set-scope and `degree` checks over each
    /// requirement's satisfier set.
    requirements: BTreeMap<String, Requirement>,
    /// The declared edge relationships gathered off every kind's
    /// `[[kind.<name>.relationships]]` array, in declaration order — the reference
    /// syntax the harness reference graph is built from (`specs/15-kinds.md`, "The
    /// entity graph is a kind capability"; `specs/45-governance.md`, "The harness is
    /// a graph too — and references are declared edges"). Each edge's `from` is the
    /// owning kind that declared it. Empty when no kind declares any. Parse-only
    /// here; assembling the graph and checking route resolution live in
    /// [`crate::graph`].
    edges: Vec<Edge>,
}

/// A declared **edge relationship** — a kind capability, declared under its owning
/// kind's `[[kind.<name>.relationships]]` array (`specs/15-kinds.md`, "The entity
/// graph is a kind capability"; `specs/45-governance.md`, "The harness is a graph
/// too — and references are declared edges"): the reference is a *declared
/// structured field on the surface*, never grepped from prose (`(skill-ref-syntax)`
/// RESOLVED). The owning kind is the edge *source* (the implicit `from`); the
/// relationship names its reference `field` and the kind it resolves to (the edge
/// `to` target) — "a rule routes to a skill by a `routes_to` field."
/// [`crate::graph`] reads the field off each source artifact's
/// [`Features`](crate::extract::Features) into edges, then flags any route that
/// resolves to no artifact of the target kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    /// The reference field `F` read off each source artifact's frontmatter (via
    /// the `extra` catch-all) — the declared reference syntax (`routes_to`). Its
    /// scalar value (or each element of a list value) names the target artifact.
    pub field: String,
    /// The artifact kind that owns the reference field — the edge *source*
    /// (`rule`), the `[kind.<name>]` the relationship was declared under. Stored
    /// verbatim; a `from` naming an unmodeled kind simply yields no source
    /// artifacts, so the edge is inert (never a route to resolve).
    pub from: String,
    /// The artifact kind the reference resolves to — the edge *target* (`skill`).
    /// A route resolves when an artifact of this kind bears the named id; the
    /// target kind must be one `temper` models, else no route can resolve (a
    /// graph-admissibility concern, checked in [`crate::graph`]).
    pub to: String,
}

/// A named **requirement** — the harness's named obligation, one concept carrying
/// every facet, declared in a top-level `[requirement.<name>]` table
/// (`specs/10-contracts.md`, "Requirements — the harness's named obligations";
/// "Decision: role and requirement are one concept"). The earlier split into a
/// structural slot and a semantic obligation bridged by `filled_by` is retired:
/// kind-typing is a facet (`kind` / `package`), not a rival concept, and there is
/// no `filled_by` because there are not two things to bridge. Fill is the artifact's
/// opt-in `satisfies` alone — there is **no name-`match` selector** (a name pattern is
/// the contract guessing, eradicated). **Every facet is optional except the name**
/// (`specs/10-contracts.md`, "all facets optional except its name").
///
/// `temper` **never interprets `means`** — it is authored intent the surface carries
/// and organizes, never a thing the engine judges (no proxy; `00-intent.md` law 3).
/// What `check` gates is the decidable shadow: [`crate::coverage`] gates referential
/// coverage over the opt-in `satisfies` edges, and [`crate::roster`]/[`crate::graph`]
/// run the set-scope predicates and the graph-scope `degree` bound over the
/// requirement's **satisfier set** (the artifacts of its `kind` that opt in via
/// `satisfies`). `requirement.` is its own namespace, distinct from the `rule`
/// artifact kind (the Decision's closing note).
#[derive(Debug, Clone, PartialEq)]
pub struct Requirement {
    /// The requirement's name — the `[requirement.<name>]` table key.
    pub name: String,
    /// The authored *intent*, the why, stated in *meaning* not predicates — "the
    /// harness has a skill that maintains development standards". Carried verbatim
    /// and **never interpreted**: `temper` organizes it, never judges it
    /// (`00-intent.md` law 3). Optional — all facets except the name are.
    pub means: Option<String>,
    /// The artifact kind that may fill the requirement (`skill`, `command`, …),
    /// stored verbatim — the `kind` typing facet. Absent ⇒ **kind-blind**: any
    /// artifact that opts in fills it (`specs/10-contracts.md`, the typing facet).
    pub kind: Option<String>,
    /// The package the filling artifact must conform to — the `package` typing facet
    /// (`specs/10-contracts.md`, the typing facet). A package named **by name**,
    /// resolved against the built-in packages ∪ `.temper/packages/` (PACKAGE-BINDING's
    /// order via [`PackageResolver`]) — never inline clauses (clauses live only in
    /// packages). Composes with `kind`: the filler is checked by its own kind's bound
    /// package (conformance) *and* this named one, as a type implements several traits.
    /// Absent ⇒ no package constraint on the filler. Stored verbatim; whether it
    /// resolves is an admissibility check (`names a real package`).
    pub package: Option<String>,
    /// Whether an unfilled requirement is a gate-blocking violation. Absent in
    /// source ⇒ `false`: `temper` never fabricates a gate the author did not declare
    /// (`00-intent.md` law 4). Mutually exclusive with [`count`](Requirement::count):
    /// `required` is the ≥1-satisfier shorthand, `count` the general cardinality form.
    pub required: bool,
    /// An optional bound on the satisfier-set cardinality — the set-scope `count`
    /// predicate (`specs/45-governance.md`, "The set scope (the roster)"): the
    /// number of artifacts satisfying the requirement must land in `[min, max]`. Absent
    /// ⇒ `None` (no cardinality gate beyond `required`'s ≥1 one). The general form of
    /// `required`; the two are mutually exclusive.
    pub count: Option<CountBound>,
    /// The declared field names held unique across the requirement's satisfier set —
    /// the set-scope `unique` predicate (`specs/45-governance.md`, "The set scope
    /// (the roster)"): each named field's extracted scalar must not repeat across the
    /// satisfiers. Absent ⇒ empty (no uniqueness gate). Generalizes the kind-wide
    /// `unique-name` engine predicate from name-only over a whole kind to an arbitrary
    /// field over a requirement's opt-in satisfier subset. Checked in [`crate::roster`].
    pub unique: Vec<String>,
    /// An optional set-scope `membership` predicate (`specs/45-governance.md`, "The
    /// set scope (the roster)"): a declared field `F` of every artifact satisfying the
    /// requirement (S₁) must lie in the feature-set drawn from a *second* satisfier
    /// set (S₂) — "every agent's `model` is one of the approved set." Unlike the static
    /// field `enum`, the allowed set is corpus-*derived*. Absent ⇒ `None` (no
    /// membership gate). Orthogonal to `count`/`unique`/`required`; checked in
    /// [`crate::roster`].
    pub membership: Option<Membership>,
    /// An optional graph-scope `degree` bound (`specs/45-governance.md`, "The graph
    /// scope (the model)"): the in/out edge count of every artifact satisfying the
    /// requirement must land in the declared bound over the harness reference
    /// graph. Declared on the requirement (a set-scope home) but ranging over the
    /// *edge* graph, so it is checked in [`crate::graph`] — reusing the resolved arcs
    /// route resolution and `acyclic` assemble — not the set-scope [`crate::roster`].
    /// Absent ⇒ `None` (no degree gate). Orthogonal to `count`/`unique`/`membership`.
    pub degree: Option<DegreeBound>,
    /// An optional external verifier for the behavioral remainder (`verified_by`).
    /// Stored verbatim; whether it *resolves* is an admissibility check.
    pub verified_by: Option<String>,
}

/// An inclusive bound on the cardinality of a requirement's satisfier set — the
/// set-scope `count` predicate (`specs/45-governance.md`, "The set scope (the
/// roster)"). The number of artifacts satisfying the requirement must land in
/// `[min, max]`; "at most N agents" is `{ min = 0, max = N }`, "exactly one planner"
/// is `{ min = 1, max = 1 }`. An inverted `min > max` bound admits no cardinality and
/// is rejected as inadmissible (`crate::roster`), mirroring `range`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountBound {
    /// The inclusive lower bound on the satisfier-set size.
    pub min: usize,
    /// The inclusive upper bound on the satisfier-set size.
    pub max: usize,
}

/// The graph-scope `degree` predicate declared on a requirement — an optional
/// inclusive bound on the **incoming** and/or **outgoing** edge count of every
/// artifact satisfying the requirement over the harness reference graph
/// (`specs/45-governance.md`, "The graph scope (the model)"). Declared on the
/// requirement (a set-scope home) but ranging over the *edge* graph: "self-registering
/// artifact: zero incoming" is `degree = { incoming = { max = 0 } }`; "routed
/// artifact: at least one incoming" is `degree = { incoming = { min = 1 } }`. At
/// least one direction is present (an empty `degree` constrains nothing — rejected
/// at parse). Deciding a satisfier node's degree against the resolved arcs lives in
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
    /// the graph-scope `degree` check (`specs/45-governance.md`).
    #[must_use]
    pub fn admits(self, degree: usize) -> bool {
        self.min.is_none_or(|min| degree >= min) && self.max.is_none_or(|max| degree <= max)
    }
}

/// A set-scope `membership` predicate over a requirement's satisfier set (S₁) — the
/// constraint that a declared field `F` of every artifact satisfying the requirement
/// must lie in a *corpus-derived* set, not a static `enum`
/// (`specs/45-governance.md`, "The set scope (the roster)"). The allowed set is the
/// `source_feature` (G) extracted over the S₂ satisfier set — the artifacts of
/// `source_kind` that opt into the `source` requirement (R₂) — "every agent's `model`
/// is one of the approved set;" "a hook's binary is one the manifest declares," each
/// set an opt-in satisfier set. S₂ may name a different artifact kind than the
/// requirement's own, so the check ranges over the whole by-kind map. The field
/// names and source requirement are stored verbatim; deciding membership lives in
/// [`crate::roster`].
///
#[derive(Debug, Clone, PartialEq)]
pub struct Membership {
    /// The field `F` on each S₁ satisfier whose extracted scalar must be a member of
    /// the source set. A satisfier missing `F` carries no value to check.
    pub field: String,
    /// The source requirement `R₂` whose satisfier set (S₂) supplies the allowed
    /// values (`specs/45-governance.md`, "each set an opt-in satisfier set"): a
    /// `source_kind` artifact enters S₂ exactly when its `satisfies` names this
    /// requirement. Stored verbatim; the join lives in [`crate::roster`].
    pub source: String,
    /// The artifact kind S₂ is drawn from (`skill`, `manifest`, …). May differ from
    /// the requirement's own `kind`, so the allowed set can be drawn from another kind.
    pub source_kind: String,
    /// The feature `G` whose extracted scalars over the S₂ satisfiers form the allowed
    /// set. A source artifact missing `G` contributes nothing to the set.
    pub source_feature: String,
    /// An optional **typed reference** constraint (`conforms_to`,
    /// `specs/45-governance.md`, "The set scope (the roster)"): when set, S₂ is
    /// narrowed to the source artifacts that *also* conform to this **package**, so the
    /// reference resolves to the right *kind* of thing — "a reference to an agent of
    /// kind K conforming to package P." A package named **by name**, resolved through
    /// the same [`PackageResolver`] a requirement's own `package` is — never inline
    /// clauses. Absent ⇒ `None`: plain membership over every S₂ satisfier, unchanged.
    /// Deciding conformance lives in [`crate::roster`], reusing the resolve + validate
    /// machinery `conformance` runs.
    pub source_package: Option<String>,
}

/// Resolves a **bound package name** to its [`Contract`] in PACKAGE-BINDING's order
/// (`specs/20-surface.md`, "Decision: package binding is by artifact kind"): a
/// built-in package name resolves from the embedded set first; any other name loads
/// `<packages_dir>/<name>/PACKAGE.md`. It is the single order every by-name binding is
/// resolved through — a requirement's `package` typing facet and a `membership`'s
/// `conforms_to` typed reference — so packages **compose**: a filler is checked by its
/// kind's bound package *and* any package a requirement names.
#[derive(Debug, Clone)]
pub struct PackageResolver {
    /// The built-in packages, keyed by name (`skill.anthropic`, `rule`) — the embedded
    /// floor set a bound name resolves against before the on-disk one, matching
    /// [`AuthorLayer::layer_over`]'s kind-binding order.
    builtins: BTreeMap<String, Contract>,
    /// The `.temper/packages/` directory a non-built-in name loads its
    /// `<name>/PACKAGE.md` from.
    packages_dir: PathBuf,
}

impl PackageResolver {
    /// Assemble a resolver over the built-in package set (keyed by name) and the
    /// on-disk `.temper/packages/` directory a project-authored name resolves against.
    #[must_use]
    pub fn new(builtins: BTreeMap<String, Contract>, packages_dir: PathBuf) -> Self {
        Self {
            builtins,
            packages_dir,
        }
    }

    /// Resolve a bound package `name` to the [`Contract`] the engine validates a
    /// requirement's filler against (`specs/10-contracts.md`, the `package` typing
    /// facet's `conforms-to` half):
    ///
    /// - `Ok(Some)` — the name is a built-in package, or an on-disk
    ///   `<packages_dir>/<name>/PACKAGE.md`; built-ins win, matching the kind-binding
    ///   order.
    /// - `Ok(None)` — the name resolves to *neither*: a non-resolving binding, which is
    ///   admissibility's finding (`names a real package`), never a thrown error, so the
    ///   caller can skip conformance rather than double-report.
    /// - `Err` — an on-disk package exists but fails to load, bubbled as the
    ///   [`ContractError`].
    pub fn resolve(&self, name: &str) -> Result<Option<Contract>, ContractError> {
        if let Some(contract) = self.builtins.get(name) {
            return Ok(Some(contract.clone()));
        }
        let path = self.packages_dir.join(name).join("PACKAGE.md");
        if path.is_file() {
            return Ok(Some(Contract::load_package(&path)?));
        }
        Ok(None)
    }
}

/// One kind's customization: an optional bound package name and the clauses to layer
/// over that package.
#[derive(Debug, Clone)]
struct KindLayer {
    /// The explicitly bound package name, if the author named one — a *name*, not a
    /// path. Resolved against the built-in floor ∪ `.temper/packages/` at layering
    /// time (`AuthorLayer::layer_over`); `None` takes the kind's built-in floor.
    package: Option<String>,
    /// The override / extend clauses, in declaration order.
    clauses: Vec<Clause>,
}

/// Errors raised while loading or applying a `temper.toml`. Hard failures (an
/// unreadable or malformed file, a layer that binds a package resolving to nothing,
/// a clause outside the closed vocabulary) — distinct from a lint finding,
/// which the check engine collects rather than throws.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ComposeError {
    /// The `temper.toml` exists but could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::compose::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// The `temper.toml` is not valid TOML.
    #[error("failed to parse {path} as TOML")]
    #[diagnostic(code(temper::compose::toml))]
    Toml {
        /// The file that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
    },

    /// The top-level `kind` key is present but is not a table of per-kind layers.
    #[error("{path}: `kind` must be a table of per-kind contract layers")]
    #[diagnostic(code(temper::compose::kind_not_table))]
    KindRootNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
    },

    /// A `[kind.<k>]` entry is present but is not a table.
    #[error("{path}: `[kind.{kind}]` must be a table")]
    #[diagnostic(code(temper::compose::kind_layer_not_table))]
    KindLayerNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The artifact kind whose layer is malformed.
        kind: String,
    },

    /// A `[kind.<k>]` layer's `package` value is not a string.
    #[error("{path}: `[kind.{kind}]` `package` must be a string")]
    #[diagnostic(code(temper::compose::package_not_string))]
    PackageNotString {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The artifact kind whose `package` is mistyped.
        kind: String,
    },

    /// A `[kind.<k>]` layer binds a package name that resolves to nothing — neither
    /// the kind's built-in package (the embedded floor) nor a project package under
    /// `.temper/packages/`. A name is resolved against the union of those two sets
    /// (`specs/20-surface.md`, "Decision: package binding is by artifact kind"), so a
    /// name matching neither is rejected, never silently ignored.
    #[error(
        "{path}: `[kind.{kind}]` binds unknown package `{package}` (resolve the built-in `{builtin}` or a project package under `{packages_dir}`)"
    )]
    #[diagnostic(
        code(temper::compose::unknown_package),
        help(
            "bind the kind's built-in package by name, author `.temper/packages/<name>/PACKAGE.md`, or omit `package` to take the built-in floor"
        )
    )]
    UnknownPackage {
        /// The `temper.toml` that named the package.
        path: PathBuf,
        /// The artifact kind whose layer binds it.
        kind: String,
        /// The unresolved bound package name.
        package: String,
        /// The kind's built-in (floor) package name — the one embedded name the
        /// binding could have taken.
        builtin: String,
        /// The `.temper/packages/` directory the project-authored names resolve
        /// against — the other half of the resolution set, named so the author sees
        /// where a package would live.
        packages_dir: PathBuf,
    },

    /// A built-in `[kind.<k>]` contract layer carries a key outside its closed set
    /// (`package`, `clause`, `relationships`). A misspelled `pacakge` is rejected at
    /// parse rather than silently ignored — a typo that quietly disables a binding
    /// or a clause is the silent gap temper exists to catch, applied to
    /// temper's own parser (`specs/10-contracts.md`, "Decision: unknown keys are
    /// rejected, not ignored"). Mirrors the unknown-predicate reject one rung out to
    /// keys.
    #[error("{path}: `[kind.{kind}]` has unknown key `{key}`")]
    #[diagnostic(
        code(temper::compose::kind_unknown_key),
        help(
            "a built-in kind layer carries only `package`, `clause`, and `relationships` — a stray key is a typo, not an escape hatch"
        )
    )]
    KindUnknownKey {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The artifact kind whose layer carries the stray key.
        kind: String,
        /// The unrecognized key.
        key: String,
    },

    /// The top-level `requirement` key is present but is not a table of requirement
    /// definitions — its own namespace, distinct from the `kind` map.
    #[error("{path}: `requirement` must be a table of named requirements")]
    #[diagnostic(code(temper::compose::requirement_root_not_table))]
    RequirementRootNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
    },

    /// A top-level key other than `kind` or `requirement` — a typo, or the retired
    /// `[role.*]` surface the role/requirement consolidation hard-cut
    /// (`specs/10-contracts.md`, "Decision: role and requirement are one concept").
    /// Rejected, not ignored: silently dropping a stray root is the very gap temper
    /// exists to catch (`specs/10-contracts.md`, "Decision: unknown keys are rejected,
    /// not ignored"), applied one rung out to the document root.
    #[error(
        "{path}: unknown top-level key `{key}` (temper.toml carries only `kind` and `requirement`)"
    )]
    #[diagnostic(
        code(temper::compose::unknown_root_key),
        help(
            "a temper.toml declares `[kind.*]` layers/custom kinds and `[requirement.*]` obligations — the `[role.*]` surface was retired into `[requirement.*]`"
        )
    )]
    UnknownRootKey {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The unrecognized top-level key.
        key: String,
    },

    /// A `[requirement.<name>]` entry is present but is not a table.
    #[error("{path}: `[requirement.{name}]` must be a table")]
    #[diagnostic(code(temper::compose::requirement_not_table))]
    RequirementNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The requirement whose definition is malformed.
        name: String,
    },

    /// A `[requirement.<name>]` key has the wrong TOML type — `means` not a string,
    /// `required` not a boolean, or `kind`/`verified_by` not a string.
    #[error("{path}: `[requirement.{name}]` key `{key}` must be {expected}")]
    #[diagnostic(code(temper::compose::requirement_wrong_type))]
    RequirementWrongType {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The requirement whose key is mistyped.
        name: String,
        /// The mistyped key.
        key: &'static str,
        /// The type that was expected, for the message.
        expected: &'static str,
    },

    /// A `[requirement.<name>]`'s `count` bound is malformed — not an inline table,
    /// or its `min`/`max` are missing, non-integer, or negative. The matched-set
    /// cardinality bound is a pair of `usize` counts, never an open guess.
    #[error(
        "{path}: `[requirement.{name}]` `count` must be an inline table with non-negative integer `min` and `max` bounds"
    )]
    #[diagnostic(code(temper::compose::requirement_bad_count))]
    RequirementBadCount {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The requirement with the malformed count bound.
        name: String,
    },

    /// A `[requirement.<name>]`'s `unique` declaration is malformed — not an array,
    /// or an element that is not a string. The set-scope `unique` predicate names a
    /// list of declared field names, never an open guess.
    #[error(
        "{path}: `[requirement.{name}]` `unique` must be an array of declared field-name strings"
    )]
    #[diagnostic(code(temper::compose::requirement_bad_unique))]
    RequirementBadUnique {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The requirement with the malformed `unique` declaration.
        name: String,
    },

    /// A `[requirement.<name>]`'s `membership` declaration is malformed — not an
    /// inline table, or missing/mistyped one of its `field`, `kind`, `source`,
    /// `feature` strings. The set-scope `membership` predicate names a constrained
    /// field, a source kind, a source requirement, and a source feature; any
    /// miss collapses here, the way [`parse_count`] folds its malformations into one
    /// error.
    #[error(
        "{path}: `[requirement.{name}]` `membership` must be an inline table with `field`, `kind`, `source`, `feature` strings"
    )]
    #[diagnostic(code(temper::compose::requirement_bad_membership))]
    RequirementBadMembership {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The requirement with the malformed `membership` declaration.
        name: String,
    },

    /// A `[requirement.<name>]`'s `degree` declaration is malformed — not an inline
    /// table, naming neither direction, or a direction whose bound is not a
    /// `{ min?, max? }` table with non-negative integer, well-ordered endpoints. The
    /// graph-scope `degree` predicate names an optional incoming/outgoing edge-count
    /// bound; an empty declaration (constraining nothing) or an inverted `min > max`
    /// (satisfied by no node) is as malformed as a mistyped bound. Any miss collapses
    /// here, the way [`parse_count`] folds its malformations into one error.
    #[error(
        "{path}: `[requirement.{name}]` `degree` must be an inline table naming an `incoming` and/or `outgoing` bound, each a `{{ min?, max? }}` table of non-negative, well-ordered integers"
    )]
    #[diagnostic(code(temper::compose::requirement_bad_degree))]
    RequirementBadDegree {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The requirement with the malformed `degree` declaration.
        name: String,
    },

    /// A `[requirement.<name>]` declares *both* `required` and a `count` bound. The
    /// two are mutually exclusive: `required` is the single-filler shorthand, `count`
    /// the general cardinality form, so declaring both is ambiguous.
    #[error(
        "{path}: `[requirement.{name}]` declares both `required` and `count`; they are mutually exclusive (`count` is the general form of `required`'s single-filler bound)"
    )]
    #[diagnostic(code(temper::compose::requirement_count_and_required))]
    RequirementCountAndRequired {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The requirement declaring both.
        name: String,
    },

    /// A `[requirement.<name>]` carries a key outside its closed facet set. A
    /// misspelled `requird` is rejected at parse rather than silently ignored — a
    /// typo that quietly disables the gate it was meant to arm is the silent gap
    /// temper exists to catch (`specs/10-contracts.md`, "Decision: unknown keys are
    /// rejected, not ignored").
    #[error("{path}: `[requirement.{name}]` has unknown key `{key}`")]
    #[diagnostic(
        code(temper::compose::requirement_unknown_key),
        help(
            "a requirement carries only `means`, `kind`, `package`, `required`, `count`, `unique`, `membership`, `degree`, and `verified_by` — a stray key is a typo, not an escape hatch (inline clauses retired: clauses live only in packages)"
        )
    )]
    RequirementUnknownKey {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The requirement carrying the stray key.
        name: String,
        /// The unrecognized key.
        key: String,
    },

    /// A `[kind.<name>.relationships]` key is present but is not an array of
    /// `[[kind.<name>.relationships]]` reference tables.
    #[error(
        "{path}: `[kind.{kind}.relationships]` must be an array of `[[kind.{kind}.relationships]]` reference tables"
    )]
    #[diagnostic(code(temper::compose::relationships_not_array))]
    RelationshipsNotArray {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The kind whose relationships array is malformed.
        kind: String,
    },

    /// A `[[kind.<name>.relationships]]` declaration is malformed — missing or
    /// mistyped one of its `field`, `to` strings. A declared relationship names a
    /// reference field and a target kind (its owning kind is the source); any miss
    /// collapses here, the way [`parse_count`] folds its malformations into one
    /// error.
    #[error(
        "{path}: `[[kind.{kind}.relationships]]` #{index} must name a reference `field` and a `to` kind, both strings"
    )]
    #[diagnostic(code(temper::compose::bad_relationship))]
    BadRelationship {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The kind that owns the malformed relationship.
        kind: String,
        /// The zero-based position of the malformed relationship in declaration order.
        index: usize,
    },

    /// A layered clause is outside the closed vocabulary (or otherwise malformed).
    /// Bubbled verbatim from [`crate::contract`] so the author layer's clauses are
    /// held to the exact same closed-vocabulary contract as a bare one's. Covers a
    /// requirement's inline `[[clause]]`s too, since they reuse [`contract::parse_clauses`].
    #[error(transparent)]
    #[diagnostic(transparent)]
    Contract(#[from] ContractError),
}

impl AuthorLayer {
    /// Load the optional `temper.toml` at `path`. A missing file is not an error —
    /// it is the floor-only path — so absence returns `Ok(None)`, and the floor
    /// runs unchanged.
    pub fn load(path: &Path) -> Result<Option<Self>, ComposeError> {
        match fs::read_to_string(path) {
            Ok(src) => Ok(Some(Self::parse(&src, path)?)),
            Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(source) => Err(ComposeError::Io {
                path: path.to_path_buf(),
                source,
            }),
        }
    }

    /// Parse a `temper.toml` from source. `path` only labels diagnostics, so this
    /// is the seam tests drive without touching disk.
    pub fn parse(src: &str, path: &Path) -> Result<Self, ComposeError> {
        let doc = src
            .parse::<DocumentMut>()
            .map_err(|source| ComposeError::Toml {
                path: path.to_path_buf(),
                source,
            })?;

        // Unknown top-level keys are rejected, not ignored (`specs/10-contracts.md`,
        // "Decision: unknown keys are rejected, not ignored"). The two roots temper
        // models are `kind` and `requirement`; a stray root — a typo, or the retired
        // `[role.*]` surface (hard-cut by the role/requirement consolidation) — must
        // fail loudly rather than silently vanish, the very silent gap temper exists to
        // catch. An upgrading author who left a `[role.*]` table is told, not dropped.
        for (key, _) in doc.as_table().iter() {
            if !matches!(key, "kind" | "requirement") {
                return Err(ComposeError::UnknownRootKey {
                    path: path.to_path_buf(),
                    key: key.to_string(),
                });
            }
        }

        let mut kinds = BTreeMap::new();
        let mut edges = Vec::new();
        if let Some(item) = doc.as_table().get("kind") {
            let kind_table = item
                .as_table()
                .ok_or_else(|| ComposeError::KindRootNotTable {
                    path: path.to_path_buf(),
                })?;
            for (name, item) in kind_table.iter() {
                let table = item
                    .as_table()
                    .ok_or_else(|| ComposeError::KindLayerNotTable {
                        path: path.to_path_buf(),
                        kind: name.to_string(),
                    })?;
                // Relationships are a kind capability gathered off every kind table
                // (`specs/15-kinds.md`), the owning `name` each edge's source. A
                // built-in kind declares its edges here; a custom kind declares them in
                // its authored `KIND.md` (`crate::kind::CustomKind`) instead.
                edges.extend(parse_relationships(table, name, path)?);
                // Every `[kind.<name>]` parses uniformly into a built-in-shaped layer
                // (package binding ⊕ clause overrides). The inline custom-kind
                // definition is retired: a `governs`/`extraction`/`clause` key is a
                // stray key here (`parse_kind_layer`), and whether the name registers a
                // custom kind is resolved against `kind::BUILTIN_KINDS` by the caller,
                // its definition loaded off disk.
                kinds.insert(name.to_string(), parse_kind_layer(table, name, path)?);
            }
        }

        let mut requirements = BTreeMap::new();
        if let Some(item) = doc.as_table().get("requirement") {
            let requirement_table =
                item.as_table()
                    .ok_or_else(|| ComposeError::RequirementRootNotTable {
                        path: path.to_path_buf(),
                    })?;
            for (name, item) in requirement_table.iter() {
                let table = item
                    .as_table()
                    .ok_or_else(|| ComposeError::RequirementNotTable {
                        path: path.to_path_buf(),
                        name: name.to_string(),
                    })?;
                requirements.insert(name.to_string(), parse_requirement(table, name, path)?);
            }
        }

        Ok(Self {
            path: path.to_path_buf(),
            kinds,
            requirements,
            edges,
        })
    }

    /// The parsed requirements, keyed by requirement name — the harness's named
    /// obligations (`specs/10-contracts.md`, "Requirements — the harness's named
    /// obligations"). Empty when the `temper.toml` declares no `[requirement.<name>]`
    /// tables — a kind-only (or empty) layer carries an empty roster. `crate::coverage`
    /// gates coverage over the `satisfies` edges; `crate::roster` and `crate::graph`
    /// run the set-scope and `degree` checks over each requirement's satisfier set.
    #[must_use]
    pub fn requirements(&self) -> &BTreeMap<String, Requirement> {
        &self.requirements
    }

    /// The parsed edge relationships, in declaration order (by owning kind, then by
    /// each kind's `[[kind.<name>.relationships]]` order). Empty when no kind
    /// declares any. The declared reference syntax the harness reference graph is
    /// built from — [`crate::graph`] reads these into a directed graph and checks
    /// route resolution.
    #[must_use]
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// The names of every `[kind.<name>]` registered in the assembly, in name order.
    /// A caller separates built-in layers from custom-kind registrations by matching
    /// each name against [`crate::kind::BUILTIN_KINDS`]: a non-built-in name registers a
    /// **custom kind**, whose definition loads from `.temper/kinds/<name>/KIND.md`
    /// ([`crate::kind::CustomKind::load`]). Uniform — the assembly binds a package to
    /// every kind the same way (`specs/40-composition.md`, "Decision: a custom kind is
    /// an authored `.temper/` artifact, registered in the assembly").
    pub fn registered_kinds(&self) -> impl Iterator<Item = &str> {
        self.kinds.keys().map(String::as_str)
    }

    /// The package the `[kind.<name>]` registration binds by name, if it named one
    /// (`package = "<name>"`). `None` when the kind is unregistered or bound no explicit
    /// package — for a **custom** kind that means the caller defaults to the kind's own
    /// name (the `spec` kind is checked by the `spec` package), the natural convention
    /// (`specs/40-composition.md`, "its package — the require-side, always a bound
    /// package").
    #[must_use]
    pub fn kind_package(&self, kind: &str) -> Option<&str> {
        self.kinds
            .get(kind)
            .and_then(|layer| layer.package.as_deref())
    }

    /// The effective contract for `kind`: this layer's clauses for that kind applied
    /// over the **package it binds**. A kind the author did not name returns `floor`
    /// unchanged, so a `temper.toml` that customizes only some kinds leaves the rest
    /// exactly at the built-in floor.
    ///
    /// The bound package resolves against `floor` ∪ `.temper/packages/`
    /// (`specs/20-surface.md`, "Decision: package binding is by artifact kind"): an
    /// omitted `package`, or a name matching the kind's built-in (the `floor.name`),
    /// takes `floor`; any other name loads the project package at
    /// `<packages_dir>/<name>/PACKAGE.md` via [`Contract::load_package`]. A name that
    /// resolves to neither is a [`ComposeError::UnknownPackage`] load error.
    ///
    /// The layer's clauses then fold over that resolved base: each **overrides** the
    /// base clause sharing its identity (the predicate key plus targeted field) — the
    /// severity flip and the parameter change both land here — or, with a new
    /// identity, **extends** the base by appending.
    pub fn layer_over(
        &self,
        kind: &str,
        floor: Contract,
        packages_dir: &Path,
    ) -> Result<Contract, ComposeError> {
        let Some(layer) = self.kinds.get(kind) else {
            return Ok(floor);
        };

        // Resolve the bound package name to its base contract. An omitted name, or the
        // kind's own built-in name, takes the embedded floor; any other name loads the
        // project package under `.temper/packages/`. A name that names neither a
        // built-in nor an on-disk project package is an unknown-package load error.
        let base = match &layer.package {
            None => floor,
            Some(name) if name == &floor.name => floor,
            Some(name) => {
                let path = packages_dir.join(name).join("PACKAGE.md");
                if !path.is_file() {
                    return Err(ComposeError::UnknownPackage {
                        path: self.path.clone(),
                        kind: kind.to_string(),
                        package: name.clone(),
                        builtin: floor.name.clone(),
                        packages_dir: packages_dir.to_path_buf(),
                    });
                }
                Contract::load_package(&path)?
            }
        };

        let mut clauses = base.clauses;
        fold_clauses(&mut clauses, &layer.clauses);
        Ok(Contract {
            name: base.name,
            clauses,
            // Carry the base package's guidance through the fold: layering clauses
            // over a package overrides predicates, not the package's prose.
            guidance: base.guidance,
        })
    }

    /// Fold a gitignored **`temper-local.toml`** layer over this committed one — the
    /// committed-plus-local split the spec names (`specs/40-composition.md`, "a
    /// gitignored `temper-local.toml` layers over *it*"; the split Lefthook proves).
    /// `temper.toml` is committed project policy; `temper-local.toml` is a personal
    /// clause/severity override that layers on top, so the fold runs one file up from
    /// [`layer_over`] with the *same* clause semantics: per kind, a local clause
    /// **overrides** the base clause sharing its [`same_identity`] (the severity flip
    /// and parameter change both land here) or, with a fresh identity, **extends** the
    /// base with it. A kind only `local` names is carried in whole; a kind only the
    /// committed layer names is left untouched.
    ///
    /// **Scope: contract clauses/severity only.** Cross-file requirement-roster and
    /// relationship merge is out of this tier and under-specified — the committed
    /// layer's [`requirements`](Self::requirements) and [`edges`](Self::edges) pass
    /// through unchanged and any `local` carries of those are not merged here (a story that
    /// needs local requirements raises an open question, per the entry). A local
    /// `package` overrides the base's for that kind, since it names the same
    /// package-selecting facet the clauses layer over.
    #[must_use]
    pub fn fold_local(mut self, local: AuthorLayer) -> AuthorLayer {
        for (kind, local_layer) in local.kinds {
            match self.kinds.get_mut(&kind) {
                // The committed layer already customizes this kind: fold the local
                // clauses over its own, and let a local `package` override the base's.
                Some(base) => {
                    if local_layer.package.is_some() {
                        base.package = local_layer.package;
                    }
                    fold_clauses(&mut base.clauses, &local_layer.clauses);
                }
                // A kind only the local layer names: it layers straight over the
                // floor, so carry it in whole.
                None => {
                    self.kinds.insert(kind, local_layer);
                }
            }
        }
        self
    }
}

/// Fold an `overlay` of clauses over a `base` clause list in place, with a layer's
/// override/extend semantics: an overlay clause sharing a base clause's
/// [`same_identity`] (predicate key + targeted field) replaces it in place — a
/// severity flip or a parameter change — while one with a fresh identity is
/// appended. Shared by [`AuthorLayer::layer_over`] (layer over the floor) and
/// [`AuthorLayer::fold_local`] (local over the committed layer), so both files fold
/// clauses identically.
fn fold_clauses(base: &mut Vec<Clause>, overlay: &[Clause]) {
    for clause in overlay {
        match base
            .iter()
            .position(|existing| same_identity(existing, clause))
        {
            Some(index) => base[index] = clause.clone(),
            None => base.push(clause.clone()),
        }
    }
}

/// The effective contract for `kind` given an *optional* author layer: `floor`
/// unchanged when there is no layer, else [`AuthorLayer::layer_over`] applied. The
/// `Option` seam keeps the absent-`temper.toml` path — every existing test's path —
/// a verbatim pass-through of the floor. `packages_dir` is the `.temper/packages/`
/// directory a bound project-package name resolves against; it is untouched when the
/// layer binds no custom package (an omitted or built-in `package`).
pub fn effective(
    layer: Option<&AuthorLayer>,
    kind: &str,
    floor: Contract,
    packages_dir: &Path,
) -> Result<Contract, ComposeError> {
    match layer {
        Some(layer) => layer.layer_over(kind, floor, packages_dir),
        None => Ok(floor),
    }
}

/// Parse one `[kind.<k>]` table into its [`KindLayer`] — the optional bound
/// `package` name and the inline `[[clause]]` array, the latter through the shared
/// closed-vocabulary parser ([`contract::parse_clauses`]).
fn parse_kind_layer(table: &Table, kind: &str, path: &Path) -> Result<KindLayer, ComposeError> {
    // A built-in layer carries only `package` and `clause`; `relationships` is a kind
    // capability gathered off every kind table before this point (`parse_relationships`),
    // so it is admissible here too. Anything else is a typo, rejected — not dropped.
    for (key, _) in table.iter() {
        if !matches!(key, "package" | "clause" | "relationships") {
            return Err(ComposeError::KindUnknownKey {
                path: path.to_path_buf(),
                kind: kind.to_string(),
                key: key.to_string(),
            });
        }
    }
    let package = match table.get("package") {
        None => None,
        Some(item) => Some(
            item.as_str()
                .ok_or_else(|| ComposeError::PackageNotString {
                    path: path.to_path_buf(),
                    kind: kind.to_string(),
                })?
                .to_string(),
        ),
    };
    let clauses = contract::parse_clauses(table, path)?;
    Ok(KindLayer { package, clauses })
}

/// Parse one kind's `[[kind.<name>.relationships]]` array into typed [`Edge`]s, in
/// declaration order — a reference is a **kind capability** declared under its owning
/// kind, not a standalone construct (`specs/15-kinds.md`, "The entity graph is a kind
/// capability"; `specs/40-composition.md`, the `.relationships` surface). The owning
/// `kind` is each edge's source (the implicit `from`); each relationship names its
/// reference `field` and target `to` kind. Absent ⇒ an empty vec (this kind declares
/// no edges). The key must be an array-of-tables (`[[kind.<name>.relationships]]`);
/// anything else is [`ComposeError::RelationshipsNotArray`]. Each element parses
/// through [`parse_relationship`], so a malformed one is a single folded
/// [`ComposeError::BadRelationship`] naming its position.
fn parse_relationships(table: &Table, kind: &str, path: &Path) -> Result<Vec<Edge>, ComposeError> {
    let Some(item) = table.get("relationships") else {
        return Ok(Vec::new());
    };
    let array = item
        .as_array_of_tables()
        .ok_or_else(|| ComposeError::RelationshipsNotArray {
            path: path.to_path_buf(),
            kind: kind.to_string(),
        })?;
    let mut edges = Vec::with_capacity(array.len());
    for (index, relationship) in array.iter().enumerate() {
        edges.push(parse_relationship(relationship, kind, index, path)?);
    }
    Ok(edges)
}

/// Parse one `[[kind.<name>.relationships]]` table into a typed [`Edge`] — its
/// required `field` (reference syntax) and `to` (target kind), both strings, with
/// the owning `kind` filled in as the edge's `from` source. Any missing or mistyped
/// key collapses to a single [`ComposeError::BadRelationship`], the way
/// [`parse_count`] folds its malformations. The names are stored verbatim; whether
/// they are *sound* (a non-empty field, a modeled target kind) is a
/// graph-admissibility concern ([`crate::graph`]), not a parse one.
fn parse_relationship(
    table: &Table,
    kind: &str,
    index: usize,
    path: &Path,
) -> Result<Edge, ComposeError> {
    let bad = || ComposeError::BadRelationship {
        path: path.to_path_buf(),
        kind: kind.to_string(),
        index,
    };
    let field = relationship_str(table, "field").ok_or_else(bad)?;
    let to = relationship_str(table, "to").ok_or_else(bad)?;
    Ok(Edge {
        field,
        from: kind.to_string(),
        to,
    })
}

/// Read one required string key off a `[[kind.<name>.relationships]]` table: present
/// and a TOML string ⇒ `Some`, else `None` (which [`parse_relationship`] reports as a
/// single [`ComposeError::BadRelationship`]).
fn relationship_str(table: &Table, key: &str) -> Option<String> {
    Some(table.get(key)?.as_str()?.to_string())
}

/// Parse one `[requirement.<name>]` table into the unified typed [`Requirement`]
/// (`specs/10-contracts.md`, "Decision: role and requirement are one concept"). Every
/// facet is optional except the name: an authored `means` (carried verbatim, never
/// interpreted — `00-intent.md` law 3), the typing facets (`kind` / `package`), the
/// optional `required` flag (absent ⇒ `false`; `temper` never fabricates a gate the
/// author did not declare — law 4), the set-scope predicates (`count` / `unique` /
/// `membership`), the graph-scope `degree` bound, and the `verified_by` verifier.
/// Typing is `kind`/`package` **by name** — never inline clauses (clauses live only in
/// packages); a `contract = "<path>"` or a `[[requirement.<name>.clause]]` array is now
/// an unknown-key reject (below), the require-side vocabulary migration. Fill is by
/// opt-in `satisfies` alone — there is no `match` key. Each malformed facet is a load
/// error, mirroring `[kind.<k>]` parsing.
fn parse_requirement(table: &Table, name: &str, path: &Path) -> Result<Requirement, ComposeError> {
    // A requirement carries only the closed facet set below; a stray key (a misspelled
    // `mean`, a `requird`) is a typo that would silently drop the meaning or disable a
    // gate it was meant to arm, so it is rejected at parse, never ignored
    // (`specs/10-contracts.md`, "Decision: unknown keys are rejected, not ignored"). The
    // retired `contract`-as-bundle key and inline `clause` array fall here too — typing
    // is by-name `package`, and clauses live only in packages.
    for (key, _) in table.iter() {
        if !matches!(
            key,
            "means"
                | "kind"
                | "package"
                | "required"
                | "count"
                | "unique"
                | "membership"
                | "degree"
                | "verified_by"
        ) {
            return Err(ComposeError::RequirementUnknownKey {
                path: path.to_path_buf(),
                name: name.to_string(),
                key: key.to_string(),
            });
        }
    }

    let means = requirement_str(table, "means", name, path)?;
    let kind = requirement_str(table, "kind", name, path)?;
    let package = requirement_str(table, "package", name, path)?;
    // `required` and `count` are two ways to express the same dimension (satisfier-set
    // cardinality), so declaring both is ambiguous — reject it before parsing either.
    if table.contains_key("required") && table.contains_key("count") {
        return Err(ComposeError::RequirementCountAndRequired {
            path: path.to_path_buf(),
            name: name.to_string(),
        });
    }
    let required = parse_requirement_required(table, name, path)?;
    let count = parse_count(table, name, path)?;
    let unique = parse_unique(table, name, path)?;
    let membership = parse_membership(table, name, path)?;
    let degree = parse_degree(table, name, path)?;
    let verified_by = requirement_str(table, "verified_by", name, path)?;

    Ok(Requirement {
        name: name.to_string(),
        means,
        kind,
        package,
        required,
        count,
        unique,
        membership,
        degree,
        verified_by,
    })
}

/// The requirement's optional `count` bound: an inline `count = { min, max }` table
/// whose `min` and `max` are non-negative integers (`usize`). Absent ⇒ `None`. Any
/// malformation — not a table, a missing/mistyped/negative bound — collapses to
/// [`ComposeError::RequirementBadCount`], the way [`parse_membership`] folds its
/// malformations into one error. The bound is stored verbatim; whether `min > max` (an
/// unsatisfiable bound) is an *admissibility* concern, checked in [`crate::roster`].
fn parse_count(table: &Table, name: &str, path: &Path) -> Result<Option<CountBound>, ComposeError> {
    let Some(item) = table.get("count") else {
        return Ok(None);
    };
    let bad_count = || ComposeError::RequirementBadCount {
        path: path.to_path_buf(),
        name: name.to_string(),
    };
    let count_table = item.as_table_like().ok_or_else(bad_count)?;
    let min = count_bound(count_table, "min").ok_or_else(bad_count)?;
    let max = count_bound(count_table, "max").ok_or_else(bad_count)?;
    Ok(Some(CountBound { min, max }))
}

/// Read one `count` bound (`min`/`max`) off the inline table as a `usize`: present,
/// a TOML integer, and non-negative. Any miss — absent, a non-integer, or a
/// negative value (`usize` cannot hold one) — is `None`, which [`parse_count`]
/// reports as a single [`ComposeError::RequirementBadCount`].
fn count_bound(table: &dyn toml_edit::TableLike, key: &str) -> Option<usize> {
    table
        .get(key)?
        .as_integer()
        .and_then(|n| usize::try_from(n).ok())
}

/// The requirement's optional `degree` bound: an inline `degree = { incoming = { min?,
/// max? }, outgoing = { min?, max? } }` table naming a graph-scope in/out edge-count
/// bound (`specs/45-governance.md`, "The graph scope (the model)"). Absent ⇒ `None`.
/// At least one direction must be named, and each present direction is a `{ min?,
/// max? }` table with at least one non-negative, well-ordered integer endpoint —
/// "self-registering" is `{ incoming = { max = 0 } }`, "routed" is `{ incoming =
/// { min = 1 } }`. Any malformation — not a table, naming no direction, a mistyped or
/// negative endpoint, an endpoint-less or inverted bound — collapses to
/// [`ComposeError::RequirementBadDegree`], the way [`parse_count`] folds its
/// malformations into one error. The bound is stored verbatim; deciding a matched
/// node's degree against the resolved arcs lives in [`crate::graph`].
fn parse_degree(
    table: &Table,
    name: &str,
    path: &Path,
) -> Result<Option<DegreeBound>, ComposeError> {
    let Some(item) = table.get("degree") else {
        return Ok(None);
    };
    let bad = || ComposeError::RequirementBadDegree {
        path: path.to_path_buf(),
        name: name.to_string(),
    };
    let degree = item.as_table_like().ok_or_else(bad)?;
    let incoming = parse_edge_bound(degree, "incoming", name, path)?;
    let outgoing = parse_edge_bound(degree, "outgoing", name, path)?;
    // A `degree` naming neither direction constrains nothing — malformed, the way an
    // endpoint-less direction bound is.
    if incoming.is_none() && outgoing.is_none() {
        return Err(bad());
    }
    Ok(Some(DegreeBound { incoming, outgoing }))
}

/// Parse one direction (`incoming`/`outgoing`) of a `degree` bound: absent ⇒ `None`;
/// present ⇒ a `{ min?, max? }` table with at least one non-negative integer
/// endpoint and, if both, `min <= max`. An endpoint-less bound (admits every degree)
/// and an inverted bound (admits none) are both vacuous, so both fold into
/// [`ComposeError::RequirementBadDegree`] — mirroring [`parse_count`]'s single folded
/// error.
fn parse_edge_bound(
    table: &dyn toml_edit::TableLike,
    direction: &str,
    name: &str,
    path: &Path,
) -> Result<Option<EdgeBound>, ComposeError> {
    let Some(item) = table.get(direction) else {
        return Ok(None);
    };
    let bad = || ComposeError::RequirementBadDegree {
        path: path.to_path_buf(),
        name: name.to_string(),
    };
    let bound = item.as_table_like().ok_or_else(bad)?;
    let min = edge_endpoint(bound, "min", name, path)?;
    let max = edge_endpoint(bound, "max", name, path)?;
    match (min, max) {
        // Neither endpoint: the bound admits every degree — meaningless, so malformed.
        (None, None) => Err(bad()),
        // An inverted bound admits no degree at all — the author cannot have meant it.
        (Some(lo), Some(hi)) if lo > hi => Err(bad()),
        _ => Ok(Some(EdgeBound { min, max })),
    }
}

/// Read one optional `degree` endpoint (`min`/`max`) off a direction table as a
/// `usize`: absent ⇒ `Ok(None)`; a non-negative TOML integer ⇒ `Ok(Some)`; anything
/// else (a non-integer, or a negative value `usize` cannot hold) ⇒
/// [`ComposeError::RequirementBadDegree`]. Unlike [`count_bound`], absence is
/// distinguished from malformation so an omitted endpoint means "unbounded on that
/// side," not an error.
fn edge_endpoint(
    table: &dyn toml_edit::TableLike,
    key: &str,
    name: &str,
    path: &Path,
) -> Result<Option<usize>, ComposeError> {
    match table.get(key) {
        None => Ok(None),
        Some(item) => {
            let value = item
                .as_integer()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or_else(|| ComposeError::RequirementBadDegree {
                    path: path.to_path_buf(),
                    name: name.to_string(),
                })?;
            Ok(Some(value))
        }
    }
}

/// The requirement's optional `unique` field list: a `unique = ["field", …]` array of
/// declared field names, each held unique across the requirement's matched set by the
/// roster check (`specs/45-governance.md`, "The set scope (the roster)"). Absent ⇒
/// an empty vec (no uniqueness gate). Any malformation — not an array, or a
/// non-string element — collapses to [`ComposeError::RequirementBadUnique`], the way
/// [`parse_count`] folds its malformations into one error. The names are stored
/// verbatim; grouping the matched fillers by each is left to [`crate::roster`].
fn parse_unique(table: &Table, name: &str, path: &Path) -> Result<Vec<String>, ComposeError> {
    let Some(item) = table.get("unique") else {
        return Ok(Vec::new());
    };
    let bad_unique = || ComposeError::RequirementBadUnique {
        path: path.to_path_buf(),
        name: name.to_string(),
    };
    let array = item.as_array().ok_or_else(bad_unique)?;
    let mut fields = Vec::new();
    for value in array.iter() {
        fields.push(value.as_str().ok_or_else(bad_unique)?.to_string());
    }
    Ok(fields)
}

/// The requirement's optional `membership` predicate: an inline `membership = { field,
/// kind, source, feature }` table naming the constrained field `F` (on the
/// requirement's own satisfier set), the source artifact kind and the source
/// requirement `R₂` whose satisfier set is `S₂`, and the source feature `G` whose
/// values form the allowed set (`specs/45-governance.md`, "The set scope (the
/// roster)"; "each set an opt-in satisfier set"). Absent ⇒ `None`. Any malformation —
/// not a table, or a missing/mistyped string — collapses to
/// [`ComposeError::RequirementBadMembership`], the way [`parse_count`] folds its
/// malformations into one error. The field names and source requirement are stored
/// verbatim; deciding membership against the corpus is left to [`crate::roster`].
fn parse_membership(
    table: &Table,
    name: &str,
    path: &Path,
) -> Result<Option<Membership>, ComposeError> {
    let Some(item) = table.get("membership") else {
        return Ok(None);
    };
    let bad = || ComposeError::RequirementBadMembership {
        path: path.to_path_buf(),
        name: name.to_string(),
    };
    let membership = item.as_table_like().ok_or_else(bad)?;
    let field = membership_str(membership, "field").ok_or_else(bad)?;
    let source_kind = membership_str(membership, "kind").ok_or_else(bad)?;
    let source = membership_str(membership, "source").ok_or_else(bad)?;
    let source_feature = membership_str(membership, "feature").ok_or_else(bad)?;
    let source_package = parse_conforms_to(membership, name, path)?;
    Ok(Some(Membership {
        field,
        source,
        source_kind,
        source_feature,
        source_package,
    }))
}

/// The optional `conforms_to` constraint on a `membership`'s source set — the
/// **typed reference** (`specs/45-governance.md`, "The set scope (the roster)"): a
/// package named **by name** (`conforms_to = "<package>"`), resolved through the same
/// [`PackageResolver`] a requirement's own `package` is — so S₂ is narrowed to the
/// source artifacts that also conform to that package. Never inline clauses (clauses
/// live only in packages, the require-side vocabulary migration). Absent ⇒ `None`
/// (plain membership). A non-string value folds into
/// [`ComposeError::RequirementBadMembership`], the way every other membership miss does.
fn parse_conforms_to(
    table: &dyn toml_edit::TableLike,
    name: &str,
    path: &Path,
) -> Result<Option<String>, ComposeError> {
    let Some(item) = table.get("conforms_to") else {
        return Ok(None);
    };
    let package = item
        .as_str()
        .ok_or_else(|| ComposeError::RequirementBadMembership {
            path: path.to_path_buf(),
            name: name.to_string(),
        })?;
    Ok(Some(package.to_string()))
}

/// Read one required string key off an inline table-like (a `membership` field):
/// present and a TOML string ⇒ `Some`, else `None` (which [`parse_membership`]
/// reports as a single [`ComposeError::RequirementBadMembership`]).
fn membership_str(table: &dyn toml_edit::TableLike, key: &str) -> Option<String> {
    Some(table.get(key)?.as_str()?.to_string())
}

/// The requirement's optional `required` flag: absent ⇒ `false` (`temper` never
/// fabricates a gate the author did not declare); present-but-not-a-boolean ⇒ a
/// load error.
fn parse_requirement_required(
    table: &Table,
    name: &str,
    path: &Path,
) -> Result<bool, ComposeError> {
    match table.get("required") {
        None => Ok(false),
        Some(item) => item
            .as_bool()
            .ok_or_else(|| ComposeError::RequirementWrongType {
                path: path.to_path_buf(),
                name: name.to_string(),
                key: "required",
                expected: "a boolean",
            }),
    }
}

/// Read an optional string key off a `[requirement.<name>]` table: absent ⇒ `None`,
/// present-but-not-a-string ⇒ [`ComposeError::RequirementWrongType`].
fn requirement_str(
    table: &Table,
    key: &'static str,
    name: &str,
    path: &Path,
) -> Result<Option<String>, ComposeError> {
    match table.get(key) {
        None => Ok(None),
        Some(item) => Some(item.as_str().map(str::to_string).ok_or_else(|| {
            ComposeError::RequirementWrongType {
                path: path.to_path_buf(),
                name: name.to_string(),
                key,
                expected: "a string",
            }
        }))
        .transpose(),
    }
}

/// Whether two clauses address the same thing — the same predicate key and the
/// same targeted field (or both field-less). This is a clause's *layering
/// identity*: a layered clause sharing it overrides the floor clause, while a
/// clause with a fresh identity extends the floor.
fn same_identity(a: &Clause, b: &Clause) -> bool {
    a.predicate.key() == b.predicate.key() && a.predicate.target() == b.predicate.target()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::contract::{Predicate, Severity};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-compose-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A `.temper/packages/` directory for `layer_over` calls that bind no project
    /// package (an omitted or built-in `package`), where the path is never read.
    fn no_packages() -> &'static Path {
        Path::new(".temper/packages")
    }

    /// A small skill-shaped floor: a required `max_len` on `name`, a required
    /// `forbidden_keys`, and an advisory `max_lines`. Enough distinct identities to
    /// exercise override-vs-extend.
    fn floor() -> Contract {
        Contract {
            name: "skill.anthropic".to_string(),
            guidance: None,
            clauses: vec![
                Clause {
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::MaxLen {
                        field: "name".to_string(),
                        max: 64,
                    },
                },
                Clause {
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::ForbiddenKeys {
                        keys: vec!["globs".to_string()],
                    },
                },
                Clause {
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::MaxLines { max: 500 },
                },
            ],
        }
    }

    #[test]
    fn no_layer_for_a_kind_returns_the_floor_unchanged() {
        let toml = r#"
[kind.rule]
[[kind.rule.clause]]
severity = "advisory"
predicate = "max_lines"
max = 100
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        // The layer names only `rule`; the `skill` floor passes through verbatim.
        assert_eq!(
            layer.layer_over("skill", floor(), no_packages()).unwrap(),
            floor()
        );
    }

    #[test]
    fn a_severity_flip_overrides_the_matching_floor_clause_in_place() {
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "advisory"
predicate = "forbidden_keys"
keys = ["globs"]
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let effective = layer.layer_over("skill", floor(), no_packages()).unwrap();

        // Same identity (key + no field) ⇒ override in place, not append: the
        // clause count is unchanged and the order is preserved.
        assert_eq!(effective.clauses.len(), floor().clauses.len());
        assert_eq!(effective.clauses[1].severity, Severity::Advisory);
        assert_eq!(
            effective.clauses[1].predicate,
            Predicate::ForbiddenKeys {
                keys: vec!["globs".to_string()]
            }
        );
    }

    #[test]
    fn a_parameter_override_replaces_the_floor_clause_with_the_same_identity() {
        // Same predicate key *and* field (`max_len` on `name`) ⇒ the layered bound
        // replaces the floor's, in place.
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 32
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let effective = layer.layer_over("skill", floor(), no_packages()).unwrap();

        assert_eq!(effective.clauses.len(), floor().clauses.len());
        assert_eq!(
            effective.clauses[0].predicate,
            Predicate::MaxLen {
                field: "name".to_string(),
                max: 32
            }
        );
    }

    #[test]
    fn a_new_identity_extends_the_floor_by_appending() {
        // `min_len` on `name` shares no identity with any floor clause (the floor's
        // `max_len` on `name` is a different key) ⇒ appended, floor preserved.
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "required"
predicate = "min_len"
field = "name"
min = 1
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let effective = layer.layer_over("skill", floor(), no_packages()).unwrap();

        assert_eq!(effective.clauses.len(), floor().clauses.len() + 1);
        // The original floor clauses are untouched and the new clause is last.
        assert_eq!(&effective.clauses[..3], &floor().clauses[..]);
        assert_eq!(
            effective.clauses[3].predicate,
            Predicate::MinLen {
                field: "name".to_string(),
                min: 1
            }
        );
    }

    #[test]
    fn a_layered_clause_carries_and_overrides_its_guidance_string() {
        // The docs-channel `guidance` layers exactly as severity/predicate do
        // (`specs/50-distribution.md`): an *override* (same identity) replaces the
        // whole floor clause, so its guidance replaces the floor's; an *extend*
        // (fresh identity) appends its clause, guidance and all. A floor whose
        // `max_len` on `name` carries guidance, overridden by a layer clause that
        // both changes the bound and re-authors the guidance, plus a new `min_len`
        // clause carrying its own guidance.
        let mut floor = floor();
        floor.clauses[0].guidance = Some("floor: keep the name short".to_string());
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 32
guidance = "layer: names cap at 32"

[[kind.skill.clause]]
severity = "advisory"
predicate = "min_len"
field = "name"
min = 1
guidance = "a name is never empty"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let effective = layer.layer_over("skill", floor, no_packages()).unwrap();

        // The override replaced the floor clause's guidance in place.
        assert_eq!(
            effective.clauses[0].guidance.as_deref(),
            Some("layer: names cap at 32")
        );
        // The extended clause is appended carrying its own guidance.
        let appended = effective.clauses.last().unwrap();
        assert_eq!(
            appended.predicate,
            Predicate::MinLen {
                field: "name".to_string(),
                min: 1
            }
        );
        assert_eq!(appended.guidance.as_deref(), Some("a name is never empty"));
    }

    #[test]
    fn an_unknown_predicate_in_a_layered_clause_is_a_load_error() {
        // The shared closed-vocabulary parser rejects it at parse, exactly as it
        // does for a bare contract — the author layer has no escape hatch.
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "required"
predicate = "word_count"
field = "description"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::Contract(ContractError::UnknownPredicate { ref predicate, .. })
                if predicate == "word_count"
        ));
    }

    #[test]
    fn binding_the_builtin_package_by_name_is_the_default_made_explicit() {
        // Naming the kind's own built-in package resolves to the embedded floor and
        // changes nothing — the implicit default made visible.
        let toml = r#"
[kind.skill]
package = "skill.anthropic"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        assert_eq!(
            layer.layer_over("skill", floor(), no_packages()).unwrap(),
            floor()
        );
    }

    #[test]
    fn binding_a_package_that_resolves_to_nothing_is_a_load_error() {
        // A name that is neither the kind's built-in nor a `.temper/packages/`
        // project package resolves to nothing — an unknown-package load error naming
        // the whole resolution set, never a silent fall-through to the floor.
        let toml = r#"
[kind.skill]
package = "skill.cursor"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let err = layer
            .layer_over("skill", floor(), no_packages())
            .unwrap_err();
        assert!(matches!(
            err,
            ComposeError::UnknownPackage { ref package, ref builtin, .. }
                if package == "skill.cursor" && builtin == "skill.anthropic"
        ));
    }

    #[test]
    fn binding_a_project_package_resolves_it_from_the_packages_dir() {
        // A non-built-in name resolves to `.temper/packages/<name>/PACKAGE.md`
        // (PACKAGE-DOCUMENT's loader). The bound package *replaces* the floor as the
        // base, so the effective contract carries the project package's clauses, and a
        // layered clause still folds over that base.
        let packages = tmpdir("packages");
        let pkg_dir = packages.join("house-style");
        fs::create_dir_all(&pkg_dir).unwrap();
        fs::write(
            pkg_dir.join("PACKAGE.md"),
            "+++\n\
[[clause]]\n\
severity = \"required\"\n\
predicate = \"min_len\"\n\
field = \"description\"\n\
min = 20\n\
+++\n\
\n\
# House style\n\
\n\
The project's own skill package.\n",
        )
        .unwrap();

        let toml = r#"
[kind.skill]
package = "house-style"
[[kind.skill.clause]]
severity = "advisory"
predicate = "max_lines"
max = 120
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let effective = layer.layer_over("skill", floor(), &packages).unwrap();

        // Identity is the package's directory stem, not the floor's name.
        assert_eq!(effective.name, "house-style");
        // The base carries the project package's `min_len` clause — the floor's own
        // clauses are gone, replaced wholesale by the bound package.
        assert!(effective.clauses.iter().any(|c| matches!(
            &c.predicate,
            Predicate::MinLen { field, min: 20 } if field == "description"
        )));
        assert!(
            !effective
                .clauses
                .iter()
                .any(|c| matches!(c.predicate, Predicate::ForbiddenKeys { .. })),
            "the floor's clauses must not survive binding a different package"
        );
        // The layered clause folds over the resolved package base.
        assert!(
            effective
                .clauses
                .iter()
                .any(|c| matches!(c.predicate, Predicate::MaxLines { max: 120 }))
        );
        // The package body carries through as the effective contract's guidance.
        assert_eq!(
            effective.guidance.as_deref(),
            Some("\n# House style\n\nThe project's own skill package.\n")
        );
    }

    #[test]
    fn an_empty_temper_toml_and_an_absent_one_both_yield_the_floor() {
        // Present-but-declares-nothing parses to a layer with no kinds, so every
        // kind falls through to the floor — the same result as `effective(None,..)`.
        let layer = AuthorLayer::parse("# nothing here\n", Path::new("temper.toml")).unwrap();
        assert_eq!(
            layer.layer_over("skill", floor(), no_packages()).unwrap(),
            floor()
        );
        assert_eq!(
            effective(None, "skill", floor(), no_packages()).unwrap(),
            floor()
        );
        assert_eq!(
            effective(Some(&layer), "skill", floor(), no_packages()).unwrap(),
            floor()
        );
    }

    #[test]
    fn load_returns_none_for_an_absent_file_and_some_for_a_present_one() {
        let dir = tmpdir("load");
        let path = dir.join("temper.toml");
        // Absent ⇒ None (the floor-only path).
        assert!(AuthorLayer::load(&path).unwrap().is_none());

        // Present ⇒ Some, parsed from disk.
        fs::write(&path, "[kind.skill]\npackage = \"skill.anthropic\"\n").unwrap();
        let layer = AuthorLayer::load(&path)
            .unwrap()
            .expect("a present file loads");
        assert_eq!(
            layer.layer_over("skill", floor(), no_packages()).unwrap(),
            floor()
        );
    }

    #[test]
    fn a_non_table_kind_entry_is_a_load_error() {
        let err = AuthorLayer::parse("kind = 7\n", Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::KindRootNotTable { .. }));
    }

    // ---- requirement roster -----------------------------------------------

    #[test]
    fn a_full_requirement_table_parses_into_a_typed_requirement() {
        // Every facet present: a `means`, a `kind`, a by-name `package` binding, an
        // explicit `required`, and a `verified_by` verifier. Fill is by opt-in
        // `satisfies` — there is no `match` selector facet.
        let toml = r#"
[requirement.task-planning]
means = "the harness plans tasks"
kind = "skill"
package = "skill.anthropic"
required = true
verified_by = "tests/plan.rs"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer
            .requirements()
            .get("task-planning")
            .expect("the requirement parses into the roster");
        assert_eq!(
            requirement,
            &Requirement {
                name: "task-planning".to_string(),
                means: Some("the harness plans tasks".to_string()),
                kind: Some("skill".to_string()),
                package: Some("skill.anthropic".to_string()),
                required: true,
                count: None,
                unique: Vec::new(),
                membership: None,
                degree: None,
                verified_by: Some("tests/plan.rs".to_string()),
            }
        );
    }

    #[test]
    fn a_bare_requirement_parses_with_every_facet_absent() {
        // Every facet is optional except the name: a requirement that declares only a
        // `means` carries `None`/empty for the rest — the pure opt-in-coverage form.
        let toml = r#"
[requirement.dev-standards]
means = "the harness maintains dev standards"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("dev-standards").unwrap();
        assert_eq!(
            requirement,
            &Requirement {
                name: "dev-standards".to_string(),
                means: Some("the harness maintains dev standards".to_string()),
                kind: None,
                package: None,
                required: false,
                count: None,
                unique: Vec::new(),
                membership: None,
                degree: None,
                verified_by: None,
            }
        );
    }

    #[test]
    fn a_requirement_with_no_means_parses() {
        // `means` is optional too — a requirement may carry only structural facets
        // (`specs/10-contracts.md`, "all facets optional except its name").
        let toml = r#"
[requirement.linter]
kind = "rule"
required = true
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("linter").unwrap();
        assert_eq!(requirement.means, None);
        assert!(requirement.required);
    }

    #[test]
    fn a_package_binding_parses_onto_the_requirement() {
        // Typing is `package` **by name**: the filler must conform to the named package,
        // resolved by name through `PackageResolver` (never inline clauses — clauses
        // live only in packages). The name is stored verbatim; whether it resolves is an
        // admissibility check.
        let toml = r#"
[requirement.release-tool]
kind = "command"
package = "release-command"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer
            .requirements()
            .get("release-tool")
            .expect("the requirement parses");
        assert_eq!(requirement.package, Some("release-command".to_string()));
    }

    #[test]
    fn an_inline_clause_array_on_a_requirement_is_an_unknown_key() {
        // Inline clauses under a requirement retired — clauses live only in packages
        // (`specs/10-contracts.md`, the typing facet). A leftover `[[requirement.*.clause]]`
        // array is no longer a facet but an unknown `clause` key, rejected at parse
        // rather than silently dropped.
        let toml = r#"
[requirement.release-tool]
kind = "command"
[[requirement.release-tool.clause]]
severity = "required"
predicate = "required"
field = "description"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementUnknownKey { ref key, ref name, .. }
                if key == "clause" && name == "release-tool"
        ));
    }

    #[test]
    fn an_absent_required_flag_defaults_to_false() {
        // `temper` never fabricates a gate the author did not declare: an absent
        // `required` is `false`, not `true`.
        let toml = r#"
[requirement.linter]
kind = "skill"
package = "skill.anthropic"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("linter").unwrap();
        assert!(!requirement.required);
        assert_eq!(requirement.verified_by, None);
    }

    #[test]
    fn the_retired_contract_bundle_key_is_an_unknown_key() {
        // `contract = "<path>"` — a requirement adopting a contract bundle by path —
        // retired: typing is `package` by name (`specs/10-contracts.md`, the typing
        // facet). A leftover `contract` key is rejected at parse rather than silently
        // dropped, the require-side vocabulary migration.
        let toml = r#"
[requirement.linter]
kind = "skill"
contract = "contracts/skill.anthropic.toml"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementUnknownKey { ref key, ref name, .. }
                if key == "contract" && name == "linter"
        ));
    }

    #[test]
    fn a_match_key_is_rejected_as_an_unknown_key() {
        // The name-`match` selector is eradicated — fill is opt-in `satisfies` alone.
        // A leftover `match = {…}` is no longer a facet but an unknown key, rejected at
        // parse rather than silently dropped (`specs/10-contracts.md`, "Decision:
        // unknown keys are rejected, not ignored").
        let toml = r#"
[requirement.linter]
kind = "skill"
package = "skill.anthropic"
match = { name = "lint*" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementUnknownKey { ref name, ref key, .. }
                if name == "linter" && key == "match"
        ));
    }

    #[test]
    fn a_non_boolean_required_flag_is_a_load_error() {
        let toml = r#"
[requirement.linter]
kind = "skill"
package = "skill.anthropic"
required = "yes"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementWrongType {
                key: "required",
                ..
            }
        ));
    }

    #[test]
    fn a_non_table_requirement_root_is_a_load_error() {
        let err = AuthorLayer::parse("requirement = 7\n", Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RequirementRootNotTable { .. }));
    }

    #[test]
    fn a_count_bound_parses_into_a_typed_requirement() {
        // The set-scope `count` predicate: an inline `{ min, max }` table parses
        // into a `CountBound`, and (being the general form of `required`) no
        // `required` flag rides alongside it.
        let toml = r#"
[requirement.agents]
kind = "agent"
package = "skill.anthropic"
count = { min = 0, max = 3 }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("agents").unwrap();
        assert_eq!(requirement.count, Some(CountBound { min: 0, max: 3 }));
        assert!(!requirement.required);
    }

    #[test]
    fn a_non_table_count_is_a_load_error() {
        let toml = r#"
[requirement.agents]
kind = "agent"
package = "skill.anthropic"
count = 3
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementBadCount { ref name, .. } if name == "agents"
        ));
    }

    #[test]
    fn a_count_with_a_non_integer_bound_is_a_load_error() {
        // A `max` that is not a non-negative integer collapses to `RequirementBadCount`,
        // the way a malformed `membership` collapses to `RequirementBadMembership`.
        let toml = r#"
[requirement.agents]
kind = "agent"
package = "skill.anthropic"
count = { min = 0, max = "three" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RequirementBadCount { .. }));
    }

    #[test]
    fn a_count_missing_a_bound_is_a_load_error() {
        // Both `min` and `max` are required — the bound is a closed pair, never a
        // half-open guess.
        let toml = r#"
[requirement.agents]
kind = "agent"
package = "skill.anthropic"
count = { max = 3 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RequirementBadCount { .. }));
    }

    #[test]
    fn a_negative_count_bound_is_a_load_error() {
        // A negative `min` cannot be a `usize` cardinality — rejected, not floored.
        let toml = r#"
[requirement.agents]
kind = "agent"
package = "skill.anthropic"
count = { min = -1, max = 3 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RequirementBadCount { .. }));
    }

    #[test]
    fn declaring_both_required_and_count_is_a_load_error() {
        // The two express the same dimension (matched-set cardinality); declaring
        // both is ambiguous, so it is rejected before either is read.
        let toml = r#"
[requirement.agents]
kind = "agent"
package = "skill.anthropic"
required = true
count = { min = 0, max = 3 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementCountAndRequired { ref name, .. } if name == "agents"
        ));
    }

    #[test]
    fn a_unique_field_list_parses_into_a_typed_requirement() {
        // The set-scope `unique` predicate: a `unique = ["model"]` array parses into
        // `Requirement.unique`, the declared fields the roster holds unique across the set.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
unique = ["model"]
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("agents").unwrap();
        assert_eq!(requirement.unique, vec!["model".to_string()]);
    }

    #[test]
    fn an_absent_unique_defaults_to_an_empty_vec() {
        // `temper` never fabricates a gate the author did not declare: an absent
        // `unique` is no uniqueness gate, an empty vec.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("agents").unwrap();
        assert!(requirement.unique.is_empty());
    }

    #[test]
    fn a_non_array_unique_is_a_load_error() {
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
unique = "model"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementBadUnique { ref name, .. } if name == "agents"
        ));
    }

    #[test]
    fn a_unique_with_a_non_string_element_is_a_load_error() {
        // A non-string element collapses to `RequirementBadUnique`, the way a
        // malformed `count` bound collapses to `RequirementBadCount`.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
unique = ["model", 7]
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RequirementBadUnique { .. }));
    }

    #[test]
    fn a_membership_clause_parses_into_a_typed_requirement() {
        // The set-scope `membership` predicate: an inline `{ field, kind, source,
        // feature }` table parses into a `Membership`, naming the constrained field
        // F, the source kind and source requirement R₂ (whose satisfiers are S₂), and
        // the source feature G.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
membership = { field = "model", kind = "manifest", source = "approved-models", feature = "model" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("agents").unwrap();
        assert_eq!(
            requirement.membership,
            Some(Membership {
                field: "model".to_string(),
                source: "approved-models".to_string(),
                source_kind: "manifest".to_string(),
                source_feature: "model".to_string(),
                source_package: None,
            })
        );
    }

    #[test]
    fn a_membership_with_a_conforms_to_package_parses() {
        // The typed-reference form: `conforms_to` names a package **by name**, so S₂ is
        // narrowed to sources conforming to that package. It parses into
        // `source_package: Some(..)`, resolved through the same `PackageResolver` a
        // requirement's own `package` is — never inline clauses.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
membership = { field = "model", kind = "manifest", source = "approved-model", feature = "model", conforms_to = "approved-manifest" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("agents").unwrap();
        assert_eq!(
            requirement.membership.as_ref().unwrap().source_package,
            Some("approved-manifest".to_string())
        );
    }

    #[test]
    fn an_inline_conforms_to_clause_sub_table_is_a_load_error() {
        // Inline `conforms_to` clauses retired — `conforms_to` names a package by name,
        // never a clause-bearing sub-table (clauses live only in packages). A leftover
        // `[requirement.<name>.membership.conforms_to]` sub-table makes `conforms_to` a
        // non-string, so it folds into `RequirementBadMembership`.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"

[requirement.agents.membership]
field = "model"
kind = "manifest"
feature = "model"
source = "approved-model"

[[requirement.agents.membership.conforms_to.clause]]
severity = "required"
predicate = "required"
field = "model"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementBadMembership { ref name, .. } if name == "agents"
        ));
    }

    #[test]
    fn a_membership_with_a_malformed_conforms_to_is_a_load_error() {
        // `conforms_to` must be a package-name string; a bare number is not, so it
        // folds into `RequirementBadMembership`.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
membership = { field = "model", kind = "manifest", source = "approved-model", feature = "model", conforms_to = 7 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementBadMembership { ref name, .. } if name == "agents"
        ));
    }

    #[test]
    fn a_membership_names_its_source_requirement() {
        // S₂ is the satisfier set of a *named source requirement* (R₂) drawn over the
        // source kind — an opt-in satisfier set, not a name glob.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
membership = { field = "model", kind = "skill", source = "approved-model", feature = "model" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("agents").unwrap();
        assert_eq!(
            requirement.membership.as_ref().unwrap().source,
            "approved-model".to_string()
        );
    }

    #[test]
    fn an_absent_membership_defaults_to_none() {
        // `temper` never fabricates a gate the author did not declare: an absent
        // `membership` is no gate at all.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let requirement = layer.requirements().get("agents").unwrap();
        assert!(requirement.membership.is_none());
    }

    #[test]
    fn a_non_table_membership_is_a_load_error() {
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
membership = "model"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RequirementBadMembership { ref name, .. } if name == "agents"
        ));
    }

    #[test]
    fn a_membership_missing_a_required_key_is_a_load_error() {
        // `feature` (the source feature G) is required — its absence collapses to
        // `RequirementBadMembership`, the way a missing `count` bound collapses to
        // `RequirementBadCount`.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
membership = { field = "model", kind = "manifest", source = "approved-model" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RequirementBadMembership { .. }));
    }

    #[test]
    fn a_membership_with_a_non_string_source_is_a_load_error() {
        // The `source` names a requirement (a string); a non-string is malformed, so
        // the whole clause folds into `RequirementBadMembership`.
        let toml = r#"
[requirement.agents]
kind = "skill"
package = "skill.anthropic"
membership = { field = "model", kind = "manifest", source = 7, feature = "model" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RequirementBadMembership { .. }));
    }

    #[test]
    fn a_kind_only_temper_toml_carries_an_empty_roster() {
        // Customizing only `[kind.*]` leaves the requirement roster empty — and the
        // kind layer still works exactly as before.
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "advisory"
predicate = "max_lines"
max = 100
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        assert!(layer.requirements().is_empty());
    }

    // ---- edge relationships (a kind capability, parse-only) ----------------

    #[test]
    fn a_relationship_parses_into_a_typed_edge_with_the_owning_kind_as_source() {
        // A reference is a kind capability: a `[[kind.rule.relationships]]` naming
        // the reference field and the target kind parses into an `Edge` whose `from`
        // is the owning kind `rule`.
        let toml = r#"
[[kind.rule.relationships]]
field = "routes_to"
to = "skill"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        assert_eq!(
            layer.edges(),
            &[Edge {
                field: "routes_to".to_string(),
                from: "rule".to_string(),
                to: "skill".to_string(),
            }]
        );
    }

    #[test]
    fn relationships_parse_off_built_in_kind_layers_in_the_assembly() {
        // A built-in kind declares its edges in the assembly, alongside its package
        // binding — gathered off every kind table, each edge's `from` its owning kind. A
        // custom kind declares its edges in its authored `KIND.md` instead (tested in
        // `crate::kind`), the same `Edge` shape.
        let toml = r#"
[kind.rule]
package = "rule"
[[kind.rule.relationships]]
field = "routes_to"
to = "skill"

[kind.skill]
package = "skill.anthropic"
[[kind.skill.relationships]]
field = "routes_to"
to = "rule"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        // Both are built-in layers registered in the assembly.
        let registered: Vec<&str> = layer.registered_kinds().collect();
        assert_eq!(registered, vec!["rule", "skill"]);
        // Both relationships are gathered as edges, each `from` its owning kind.
        let edges: Vec<(&str, &str, &str)> = layer
            .edges()
            .iter()
            .map(|e| (e.from.as_str(), e.field.as_str(), e.to.as_str()))
            .collect();
        assert!(edges.contains(&("rule", "routes_to", "skill")));
        assert!(edges.contains(&("skill", "routes_to", "rule")));
    }

    #[test]
    fn multiple_relationships_on_one_kind_parse_in_declaration_order() {
        // One kind may declare several relationships; they arrive in declaration
        // order, each `from` the owning kind.
        let toml = r#"
[[kind.skill.relationships]]
field = "routes_to"
to = "skill"

[[kind.skill.relationships]]
field = "delegates_to"
to = "skill"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let fields: Vec<&str> = layer.edges().iter().map(|e| e.field.as_str()).collect();
        assert_eq!(fields, vec!["routes_to", "delegates_to"]);
        assert!(layer.edges().iter().all(|e| e.from == "skill"));
    }

    #[test]
    fn a_kind_declaring_no_relationships_yields_no_edges() {
        // `temper` never fabricates a gate the author did not declare: a kind with no
        // `relationships` array declares no edges.
        let toml = r#"
[requirement.planner]
kind = "skill"
package = "skill.anthropic"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        assert!(layer.edges().is_empty());
    }

    #[test]
    fn a_non_array_relationships_key_is_a_load_error() {
        let err = AuthorLayer::parse("[kind.rule]\nrelationships = 7\n", Path::new("temper.toml"))
            .unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RelationshipsNotArray { ref kind, .. } if kind == "rule"
        ));
    }

    #[test]
    fn a_relationship_missing_a_required_key_is_a_load_error() {
        // `to` (the target kind) is required — its absence collapses to
        // `BadRelationship`, the way a missing `count` bound collapses to
        // `RequirementBadCount`.
        let toml = r#"
[[kind.rule.relationships]]
field = "routes_to"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::BadRelationship { index: 0, ref kind, .. } if kind == "rule"
        ));
    }

    #[test]
    fn a_relationship_with_a_mistyped_key_is_a_load_error() {
        // A non-string `field` is not a reference syntax name — folded into
        // `BadRelationship`, the index naming which relationship was malformed.
        let toml = r#"
[[kind.skill.relationships]]
field = "routes_to"
to = "skill"

[[kind.skill.relationships]]
field = 7
to = "skill"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::BadRelationship { index: 1, ref kind, .. } if kind == "skill"
        ));
    }

    // ---- custom-kind registration (definition retired to KIND.md) ---------
    //
    // A custom kind is now *registered* in the assembly (`[kind.<name>]` binds a
    // package by name) and *defined* under `.temper/kinds/<name>/KIND.md`
    // (`crate::kind::CustomKind`) — the fully-inline `governs`/`extraction`
    // definition is retired to a stray-key reject here.

    #[test]
    fn a_custom_kind_registration_binds_a_package_by_name() {
        // The registration is uniform with a built-in binding: `[kind.spec] package =
        // "spec"` binds the require-side, and the definition lives in KIND.md, not
        // here. It parses into the registered-kinds set with its bound package.
        let toml = r#"
[kind.spec]
package = "spec"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let registered: Vec<&str> = layer.registered_kinds().collect();
        assert_eq!(registered, vec!["spec"]);
        assert_eq!(layer.kind_package("spec"), Some("spec"));
        // Separating custom from built-in is the caller's job, against BUILTIN_KINDS.
        assert!(!crate::kind::BUILTIN_KINDS.contains(&"spec"));
    }

    #[test]
    fn an_inline_governs_definition_is_a_retired_stray_key() {
        // The inline custom-kind definition is retired: a `governs` locus under a kind
        // table is no longer a declaration but a stray key, rejected at load exactly as
        // any other (`specs/40-composition.md`, "Decision: a custom kind is an authored
        // `.temper/` artifact"). The definition belongs in KIND.md.
        let toml = r#"
[kind.spec]
governs = { root = "specs", glob = "*.md" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(
            matches!(err, ComposeError::KindUnknownKey { ref key, ref kind, .. } if key == "governs" && kind == "spec"),
            "an inline `governs` is a retired stray key, got: {err:?}"
        );
    }

    #[test]
    fn an_inline_extraction_definition_is_a_retired_stray_key() {
        // Likewise the inline `[[kind.spec.extraction]]` array — the composed extractor
        // is authored in KIND.md, never stuffed into the assembly.
        let toml = r#"
[kind.spec]
package = "spec"
[[kind.spec.extraction]]
primitive = "line_count"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(
            matches!(err, ComposeError::KindUnknownKey { ref key, ref kind, .. } if key == "extraction" && kind == "spec"),
            "an inline `extraction` is a retired stray key, got: {err:?}"
        );
    }
}
