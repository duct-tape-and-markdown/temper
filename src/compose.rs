//! Composition — layer an optional author-declared `temper.toml` over the
//! embedded by-kind floor contracts (`specs/architecture/40-composition.md`).
//!
//! A `temper.toml` carries per-kind `[kind.<k>]` layers (bind a package by name,
//! then override/extend its clauses), top-level `[requirement.<name>]` obligations
//! (`specs/architecture/10-contracts.md`), and per-kind `[[kind.<name>.relationships]]` edges
//! (`specs/architecture/15-kinds.md`). Clauses parse through the same closed-vocabulary
//! [`crate::contract`] parser a bare contract uses, so the author layer earns no
//! escape hatch the floor lacks. A non-built-in `[kind.<name>]` *registers* a custom
//! kind whose definition lives in `.temper/kinds/<name>/KIND.md`
//! ([`crate::kind::CustomKind`]); [`crate::coverage`], [`crate::roster`], and
//! [`crate::graph`] consume the parsed requirements and edges.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use toml_edit::{Array, ArrayOfTables, DocumentMut, InlineTable, Item, Table, Value, value};

use crate::contract::{self, Clause, Contract, ContractError};
use crate::document::{self, PublishedRequirement};
use crate::extract::{
    self, FeatureValue, Features, FencedBlock, GenreCollections, GenreValue, Kind, Section,
};

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
    /// keyed by name (`specs/architecture/10-contracts.md`). Its own namespace, distinct from the
    /// `kind` map; empty when the `temper.toml` declares none.
    requirements: BTreeMap<String, Requirement>,
    /// The declared edge relationships gathered off every kind's
    /// `[[kind.<name>.relationships]]` array, in declaration order
    /// (`specs/architecture/15-kinds.md`). Each edge's `from` is the owning kind that declared it;
    /// empty when no kind declares any. Parse-only here — [`crate::graph`] assembles
    /// the graph.
    edges: Vec<Edge>,
    /// The assembly's graph-scope reachability opt-in, parsed from the top-level
    /// `[reachability]` table (`specs/architecture/45-governance.md`). `None` ⇒ the assembly did
    /// not opt in, so [`crate::graph::reachable`] does not run. Its own assembly-scope
    /// declaration, distinct from the `kind`/`requirement` maps.
    reachability: Option<Reachability>,
    /// The assembly's declared **surface-authority posture**, parsed from the top-level
    /// `authority` key (`specs/architecture/20-surface.md`, "surface authority is a declared
    /// posture"). Absent ⇒ [`Authority::Shared`], the default. Inert here — parsed and
    /// exposed only; the install-wired enforcement artifacts (INSTALL-GUARD-ARTIFACTS)
    /// are the consumers.
    authority: Authority,
    /// The **serialized member features** parsed from the top-level `[[member]]` tables
    /// (`specs/architecture/20-surface.md`, "Topology"): the generated-canonical, pre-extracted
    /// form of every imported member. Its own root, distinct from the authored
    /// `kind`/`requirement` maps — `import` regenerates it whole while patching the
    /// author's bindings/requirements format-preserving. Empty when the manifest carries
    /// none. Inert on the read side until MANIFEST-GATE-READ flips the gate to consume it
    /// in place of the `.temper/` copy tree; parsed here so a re-import and the gate load
    /// round-trip the whole manifest without choking on the emitted section.
    members: Vec<ManifestMember>,
    /// The **in-place members** parsed from the `source`-bearing `[[member]]` tables
    /// (`specs/architecture/20-surface.md`, "In-place"): the harness landscape files that
    /// *are* their own members, live-extracted at check time. Its own list, distinct from
    /// the pre-extracted `members` above — a `[[member]]` table routes here when it
    /// carries a `source` path, there when it bakes features. Empty when the manifest
    /// declares no in-place members (any pre-`init` or altitude-only manifest).
    inplace: Vec<InPlaceMember>,
}

/// The assembly's **surface-authority posture** — how firmly the surface owns its
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

/// A declared **edge relationship** — a kind capability declared under its owning
/// kind's `[[kind.<name>.relationships]]` array (`specs/architecture/15-kinds.md`). The owning
/// kind is the edge *source* (the implicit `from`); the relationship names its
/// reference `field` and the target `to` kind. [`crate::graph`] reads the field off
/// each source artifact into edges, then flags any route that resolves to no
/// artifact of the target kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    /// The reference field read off each source artifact's frontmatter (via the
    /// `extra` catch-all). Its scalar value (or each element of a list value) names
    /// the target artifact.
    pub field: String,
    /// The artifact kind that owns the reference field — the edge *source*, the
    /// `[kind.<name>]` the relationship was declared under. A `from` naming an
    /// unmodeled kind yields no source artifacts, so the edge is inert.
    pub from: String,
    /// The artifact kind the reference resolves to — the edge *target*. The target
    /// kind must be one `temper` models, else no route can resolve (a
    /// graph-admissibility concern, [`crate::graph`]).
    pub to: String,
}

/// One **member's serialized features** in the manifest — a `[[member]]` table
/// (`specs/architecture/20-surface.md`, "Topology": member features serialize into the
/// manifest, the pre-extracted form the gate reads). It pairs the bare `kind` a member
/// is checked under with the deterministically-extracted [`Features`] `import` (or the
/// altitude's `emit`) baked in: the frontmatter fields, the body facts (line count,
/// headings, sections, source dir, directives, fenced blocks), and the representation edges
/// (`satisfies`, published requirements). Generated-canonical — regenerated whole each
/// `import`, never hand-tended — so the whole `member` root is re-emitted, distinct from
/// the hand-authored bindings/requirements the tool patches format-preserving.
#[derive(Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct ManifestMember {
    /// The bare kind name the member is checked under (`skill`, `rule`, a custom kind's
    /// name) — the key `check` groups members by (`assemble_by_kind`), so the manifest
    /// carries it explicitly rather than nesting members under a per-kind table.
    pub kind: String,
    /// The member's deterministically-extracted [`Features`] — the exact value the gate
    /// consumes, so a serialized member round-trips to the same features a live
    /// extraction yields.
    pub features: Features,
}

/// One **in-place member** declared in the manifest (`specs/architecture/20-surface.md`,
/// "In-place — the landscape file itself is the member"): a `[[member]]` table that
/// carries a `source` path instead of pre-extracted features. The harness file at
/// `source` *is* the member — its features are **live-extracted** at check time (no
/// projection, no provenance, no drift; the file is its own source), and only the joins
/// it participates in — `satisfies` and published requirements — are declared here,
/// because the harness format is not temper's to annotate. On a fresh `init` a member
/// arrives **unrecognized** (empty joins); recognition accrues member-by-member.
#[derive(Debug, Clone, PartialEq)]
pub struct InPlaceMember {
    /// The bare kind name the member is checked under (`skill`, `rule`).
    pub kind: String,
    /// The member id — its surface name, the id a live extraction yields.
    pub name: String,
    /// The landscape file that *is* the member, a slash path relative to the harness
    /// root (`.claude/rules/x.md`). The gate reads and extracts it live.
    pub source: String,
    /// The requirements this member opts into filling, declared in the assembly (the
    /// harness file carries no temper annotation). Empty until recognition accrues.
    pub satisfies: Vec<String>,
    /// The requirements this member publishes, declared in the assembly. Empty until
    /// recognition accrues.
    pub published: Vec<PublishedRequirement>,
}

/// A named **requirement** — the harness's named obligation, declared in a top-level
/// `[requirement.<name>]` table (`specs/architecture/10-contracts.md`). **Every facet is optional
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
    /// The requirement's name — the `[requirement.<name>]` table key.
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

/// The assembly's graph-scope **`reachable`** opt-in — declared in a top-level
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

/// Resolves a **bound package name** to its [`Contract`] in PACKAGE-BINDING's order
/// (`specs/architecture/20-surface.md`): a built-in name resolves from the embedded set first, any
/// other name loads `<packages_dir>/<name>/PACKAGE.md`. The single order every by-name
/// binding resolves through (a requirement's `package`, a `membership`'s
/// `conforms_to`), so packages **compose**.
#[derive(Debug, Clone)]
pub struct PackageResolver {
    /// The built-in packages, keyed by name — the embedded floor set a bound name
    /// resolves against before the on-disk one, matching [`AuthorLayer::layer_over`]'s
    /// kind-binding order.
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
    /// requirement's filler against (`specs/architecture/10-contracts.md`, the `package` typing
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
    /// the kind's built-in floor nor a project package under `.temper/packages/`
    /// (`specs/architecture/20-surface.md`). A name matching neither is rejected, never silently
    /// ignored.
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

    /// A built-in `[kind.<k>]` layer carries a key outside its closed set (`package`,
    /// `clause`, `relationships`). Rejected, not ignored — a typo that quietly
    /// disables a binding or clause is the silent gap temper exists to catch
    /// (`specs/architecture/10-contracts.md`, unknown keys rejected).
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

    /// A top-level key other than `kind`, `requirement`, `reachability`, `authority`, or
    /// `member` — a typo, or the retired `[role.*]` surface. Rejected, not ignored:
    /// silently dropping a stray root is the gap temper exists to catch
    /// (`specs/architecture/10-contracts.md`, unknown keys rejected).
    #[error(
        "{path}: unknown top-level key `{key}` (temper.toml carries only `kind`, `requirement`, `reachability`, `authority`, and `member`)"
    )]
    #[diagnostic(
        code(temper::compose::unknown_root_key),
        help(
            "a temper.toml declares `[kind.*]` layers/custom kinds, `[requirement.*]` obligations, the assembly-scope `[reachability]` opt-in, the `authority` posture, and the emitted `[[member]]` feature tables — the `[role.*]` surface was retired into `[requirement.*]`"
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

    /// A `[requirement.<name>]`'s `count` bound is malformed — not an inline table, or
    /// its `min`/`max` are missing, non-integer, or negative.
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

    /// A `[requirement.<name>]`'s `unique` declaration is malformed — not an array, or
    /// an element that is not a string.
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

    /// A `[requirement.<name>]`'s `membership` declaration is malformed — not an inline
    /// table, or missing/mistyped one of its `field`, `kind`, `source`, `feature`
    /// strings. Any miss collapses here, the way [`parse_count`] folds its
    /// malformations into one error.
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
    /// `{ min?, max? }` table with non-negative, well-ordered endpoints. An empty
    /// declaration and an inverted `min > max` are as malformed as a mistyped bound;
    /// any miss collapses here, the way [`parse_count`] folds its malformations.
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

    /// A `[requirement.<name>]` carries a key outside its closed facet set. Rejected,
    /// not ignored — a typo that quietly disables the gate it was meant to arm is the
    /// silent gap temper exists to catch (`specs/architecture/10-contracts.md`, unknown keys
    /// rejected).
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
    /// mistyped one of its `field`, `to` strings. Any miss collapses here, the way
    /// [`parse_count`] folds its malformations into one error.
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

    /// The top-level `[reachability]` declaration is malformed — not a table, missing
    /// its `severity`, a `severity` outside the `required`/`advisory` dial, or carrying
    /// a stray key. The assembly's graph-scope opt-in carries exactly one key; any miss
    /// folds here, the way [`parse_degree`] folds a degree bound's malformations
    /// (`specs/architecture/45-governance.md`).
    #[error(
        "{path}: `[reachability]` must be a table declaring a `severity` of `required` or `advisory` and no other key"
    )]
    #[diagnostic(code(temper::compose::bad_reachability))]
    BadReachability {
        /// The malformed `temper.toml`.
        path: PathBuf,
    },

    /// The top-level `authority` posture is outside its closed vocabulary — a value
    /// other than `shared` or `surface`, or not a string (`specs/architecture/20-surface.md`).
    /// A closed vocabulary, like `[reachability]`'s `severity`: an unknown value is a
    /// typo, rejected at load, never silently defaulted.
    #[error(
        "{path}: `authority` must be a string of `shared` or `surface` (absent ⇒ `shared`, the default)"
    )]
    #[diagnostic(code(temper::compose::bad_authority))]
    BadAuthority {
        /// The malformed `temper.toml`.
        path: PathBuf,
    },

    /// The top-level `member` key is present but is not an array of `[[member]]`
    /// feature tables — its own root, distinct from the `kind`/`requirement` maps
    /// (`specs/architecture/20-surface.md`, "Topology").
    #[error("{path}: `member` must be an array of `[[member]]` feature tables")]
    #[diagnostic(code(temper::compose::member_root_not_array))]
    MemberRootNotArray {
        /// The malformed `temper.toml`.
        path: PathBuf,
    },

    /// A `[[member]]` table is malformed — missing or mistyped its required `kind`/`name`
    /// strings, or carrying a `headings`/`directives`/`satisfies`/`section`/`fenced`/`published`
    /// facet of the wrong TOML shape. Any miss folds here, the way [`parse_count`] folds a
    /// count bound's malformations. A generated section should never be malformed; a
    /// hand-edited one that is fails loudly, never silently degrading a member's features.
    #[error(
        "{path}: `[[member]]` #{index} is malformed (each carries a `kind` and `name` string, an optional `line_count`/`source_dir`, string-array `headings`/`directives`/`satisfies`, a `[member.field]` table, and `[[member.section]]`/`[[member.fenced]]`/`[[member.published]]` tables)"
    )]
    #[diagnostic(code(temper::compose::bad_member))]
    BadMember {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The zero-based position of the malformed member in declaration order.
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

        // Unknown top-level keys are rejected, not ignored (`specs/architecture/10-contracts.md`) —
        // the roots temper models are `kind`, `requirement`, the assembly-scope
        // `reachability` opt-in (`specs/architecture/45-governance.md`), and the `authority`
        // posture (`specs/architecture/20-surface.md`).
        for (key, _) in doc.as_table().iter() {
            if !matches!(
                key,
                "kind" | "requirement" | "reachability" | "authority" | "member"
            ) {
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
                // Relationships are a kind capability gathered off every kind table,
                // the owning `name` each edge's source (`specs/architecture/15-kinds.md`).
                edges.extend(parse_relationships(table, name, path)?);
                // Every `[kind.<name>]` parses uniformly into a package binding ⊕
                // clause overrides; whether the bare name resolves to a built-in kind
                // (a layer) or registers a custom one is the caller's concern, routed
                // through provider resolution (`crate::builtin_kind::definition`).
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

        // The assembly's graph-scope reachability opt-in — its own root, parsed like a
        // `[requirement.<name>]`'s `degree` bound (`specs/architecture/45-governance.md`).
        let reachability = match doc.as_table().get("reachability") {
            None => None,
            Some(item) => {
                let table = item
                    .as_table_like()
                    .ok_or_else(|| ComposeError::BadReachability {
                        path: path.to_path_buf(),
                    })?;
                Some(parse_reachability(table, path)?)
            }
        };

        // The assembly's surface-authority posture — a top-level string, closed vocabulary
        // (`specs/architecture/20-surface.md`). Absent ⇒ the `Shared` default; an unknown value is a
        // load error, like `[reachability]`'s `severity`.
        let authority = parse_authority(doc.as_table().get("authority"), path)?;

        // The emitted member-features root — the generated-canonical, pre-extracted form
        // (`specs/architecture/20-surface.md`, "Topology"). Its own array root, parsed like the
        // lock's roll-up rows: each `[[member]]` table into a typed [`ManifestMember`].
        // A `[[member]]` table carrying a `source` path is **in-place** — the landscape
        // file is the member, features live-extracted at check (`specs/architecture/20-surface.md`);
        // one baking features is document/module-carried, pre-extracted here. One array,
        // two carriages, routed by the presence of `source`.
        let mut members = Vec::new();
        let mut inplace = Vec::new();
        if let Some(item) = doc.as_table().get("member") {
            let array =
                item.as_array_of_tables()
                    .ok_or_else(|| ComposeError::MemberRootNotArray {
                        path: path.to_path_buf(),
                    })?;
            for (index, table) in array.iter().enumerate() {
                if table.contains_key("source") {
                    inplace.push(parse_inplace_member(table, index, path)?);
                } else {
                    members.push(parse_member(table, index, path)?);
                }
            }
        }

        Ok(Self {
            path: path.to_path_buf(),
            kinds,
            requirements,
            edges,
            reachability,
            authority,
            members,
            inplace,
        })
    }

    /// The parsed requirements, keyed by name. Empty when the `temper.toml` declares
    /// no `[requirement.<name>]` tables.
    #[must_use]
    pub fn requirements(&self) -> &BTreeMap<String, Requirement> {
        &self.requirements
    }

    /// The parsed edge relationships, in declaration order (by owning kind, then by
    /// each kind's `[[kind.<name>.relationships]]` order). Empty when no kind declares
    /// any. [`crate::graph`] reads these into a directed graph and checks route
    /// resolution.
    #[must_use]
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// The assembly's graph-scope reachability opt-in, if it declared a top-level
    /// `[reachability]` table (`specs/architecture/45-governance.md`). `None` ⇒ the
    /// [`crate::graph::reachable`] predicate does not run — read off the layer exactly
    /// as `degree` reads [`requirements`](Self::requirements), the opt-in the gate
    /// dispatches on.
    #[must_use]
    pub fn reachability(&self) -> Option<Reachability> {
        self.reachability
    }

    /// The assembly's declared surface-authority posture (`specs/architecture/20-surface.md`).
    /// [`Authority::Shared`] when the `temper.toml` declares no `authority` key — the
    /// default. Inert this slice: exposed for the install-wired consumers
    /// (INSTALL-GUARD-ARTIFACTS), read by no gate yet.
    #[must_use]
    pub fn authority(&self) -> Authority {
        self.authority
    }

    /// The manifest's serialized member features, in declaration order
    /// (`specs/architecture/20-surface.md`, "Topology"). Empty when the manifest declares no
    /// `[[member]]` tables. The gate reads these pre-extracted features as its corpus in
    /// place of a live extraction over the authored sources ([`member_corpus`](Self::member_corpus));
    /// exposed whole for a re-import round-trip and any reader that ranges the raw list.
    #[must_use]
    pub fn members(&self) -> &[ManifestMember] {
        &self.members
    }

    /// The manifest's **in-place members**, in declaration order
    /// (`specs/architecture/20-surface.md`, "In-place"). Empty when the manifest declares none.
    /// The gate resolves each against the harness root and **live-extracts** its features
    /// from the landscape file — no projection, no drift — in place of the pre-extracted
    /// [`member_corpus`](Self::member_corpus) it reads for document/module carriage.
    #[must_use]
    pub fn inplace_members(&self) -> &[InPlaceMember] {
        &self.inplace
    }

    /// The manifest's serialized member features grouped by **bare kind name** — the
    /// gate's pre-extracted corpus (`specs/architecture/20-surface.md`, "a module-carried member
    /// arrives pre-extracted… its features are its declared typed fields, bounded by the
    /// same closed vocabulary via the manifest schema"). Every carriage serializes
    /// identically into `[[member]]`, so the gate ranges this in place of a live
    /// extraction over the `.temper/` copy tree, reading no language runtime.
    ///
    /// Empty when the manifest declares no `[[member]]` tables — the floor-manifest case
    /// (a hand-written assembly not yet carrying its members, temper's own dogfood
    /// included), where the gate falls back to extracting the authored sources. Members
    /// arrive kind-then-id sorted from `import`/`emit`, so each kind's slice stays
    /// name-sorted for a stable diagnostic set.
    #[must_use]
    pub fn member_corpus(&self) -> BTreeMap<String, Vec<Features>> {
        let mut corpus: BTreeMap<String, Vec<Features>> = BTreeMap::new();
        for member in &self.members {
            corpus
                .entry(member.kind.clone())
                .or_default()
                .push(member.features.clone());
        }
        corpus
    }

    /// The names of every `[kind.<name>]` registered in the assembly, in name order — the
    /// **bare** names the author writes. A caller separates built-in layers from custom-kind
    /// registrations by resolving each bare name through provider resolution
    /// ([`crate::builtin_kind::definition`]): a name resolving to an embedded built-in is a
    /// contract layer, one resolving to none registers a **custom kind** whose definition
    /// loads from `.temper/kinds/<name>/KIND.md` ([`crate::kind::CustomKind::load`]), and two
    /// providers under one bare name is a load error (`specs/architecture/15-kinds.md`).
    pub fn registered_kinds(&self) -> impl Iterator<Item = &str> {
        self.kinds.keys().map(String::as_str)
    }

    /// The package the `[kind.<name>]` registration binds by name, if it named one.
    /// `None` when the kind is unregistered or bound no explicit package — for a custom
    /// kind the caller then defaults to the kind's own name (`specs/architecture/40-composition.md`).
    #[must_use]
    pub fn kind_package(&self, kind: &str) -> Option<&str> {
        self.kinds
            .get(kind)
            .and_then(|layer| layer.package.as_deref())
    }

    /// The effective contract for `kind`: this layer's clauses applied over the
    /// **package it binds**. A kind the author did not name returns `floor` unchanged.
    ///
    /// The bound package resolves against `floor` ∪ `.temper/packages/`
    /// (`specs/architecture/20-surface.md`): an omitted `package`, or the kind's built-in name,
    /// takes `floor`; any other name loads `<packages_dir>/<name>/PACKAGE.md`, and one
    /// resolving to neither is a [`ComposeError::UnknownPackage`]. The layer's clauses
    /// then fold over that base — **override** the base clause sharing an identity, else
    /// **extend** by appending.
    pub fn layer_over(
        &self,
        kind: &str,
        floor: Contract,
        packages_dir: &Path,
    ) -> Result<Contract, ComposeError> {
        // The assembly keys its `[kind.<name>]` layer by the **bare** name the author
        // writes (`skill`), while a caller may pass the *qualified* floor identity
        // (`claude-code.skill`, `specs/architecture/15-kinds.md`) — resolve to the bare component so a
        // qualified floor still finds its bare layer. A bare or provider-less name is
        // its own last dotted component, so this is identity for `skill`/`spec`.
        let lookup = kind.rsplit('.').next().unwrap_or(kind);
        let Some(layer) = self.kinds.get(lookup) else {
            return Ok(floor);
        };

        // An omitted name, or the kind's own built-in name, takes the embedded floor;
        // any other name loads the project package under `.temper/packages/`, and one
        // resolving to neither is an unknown-package load error.
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

    /// Fold a gitignored **`temper-local.toml`** layer over this committed one
    /// (`specs/architecture/40-composition.md`) — committed project policy, then a personal
    /// clause/severity override on top, with the *same* override/extend clause
    /// semantics [`layer_over`] uses. A kind only `local` names is carried in whole; a
    /// kind only the committed layer names is left untouched.
    ///
    /// **Scope: contract clauses/severity only.** Cross-file requirement-roster,
    /// relationship, and reachability merge is out of this tier and under-specified —
    /// the committed layer's [`requirements`](Self::requirements), [`edges`](Self::edges),
    /// and [`reachability`](Self::reachability) pass through unchanged (a story needing a
    /// local override raises an open question). A local `package` overrides the base's
    /// for that kind.
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
                // A kind only the local layer names layers straight over the floor.
                None => {
                    self.kinds.insert(kind, local_layer);
                }
            }
        }
        self
    }

    /// An empty layer labelled by `path` — no bindings, requirements, edges, or members.
    /// The base the temper-owned assembly-fact artifacts merge into when they sit beside a
    /// `temper.toml` too thin to have parsed into a layer (or none at all), so the roster
    /// still gates.
    #[must_use]
    pub fn empty(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            kinds: BTreeMap::new(),
            requirements: BTreeMap::new(),
            edges: Vec::new(),
            reachability: None,
            authority: Authority::default(),
            members: Vec::new(),
            inplace: Vec::new(),
        }
    }

    /// Fold the temper-owned assembly-fact artifacts (`roster.toml`/`bindings.toml`) into
    /// this layer as the **assembly source** (`specs/architecture/20-surface.md`, "the bindings,
    /// the roster — are emitted as small committed temper-owned artifacts"): the SDK emits
    /// a members-only `temper.toml` and lands the roster + bindings beside it, so the gate
    /// reads its requirements and kind bindings from the artifacts rather than only the
    /// manifest layer.
    ///
    /// A requirement or binding the hand-written `temper.toml` **already** declares wins —
    /// the migration-era inline spelling takes precedence, so the artifacts only *fill*
    /// what the manifest layer left absent (for an SDK members-only manifest, that is the
    /// whole roster and every binding). `bindings` is keyed by bare kind name and folds in
    /// as a package-only `[kind.<name>]` layer, the same shape a hand-written binding
    /// parses into.
    pub fn merge_assembly(
        &mut self,
        requirements: BTreeMap<String, Requirement>,
        bindings: BTreeMap<String, String>,
    ) {
        for (name, requirement) in requirements {
            self.requirements.entry(name).or_insert(requirement);
        }
        for (kind, package) in bindings {
            self.kinds.entry(kind).or_insert_with(|| KindLayer {
                package: Some(package),
                clauses: Vec::new(),
            });
        }
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
/// `Option` seam keeps the absent-`temper.toml` path a verbatim pass-through of the
/// floor. `packages_dir` is untouched when the layer binds no custom package.
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
    // `relationships` is admissible here too — it is gathered before this point
    // (`parse_relationships`). Anything outside the closed set is a typo, rejected.
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
/// declaration order, the owning `kind` each edge's source (`specs/architecture/15-kinds.md`).
/// Absent ⇒ an empty vec. The key must be an array-of-tables, else
/// [`ComposeError::RelationshipsNotArray`]; a malformed element is a single folded
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
/// required `field` and `to`, both strings, with the owning `kind` as the edge's
/// `from`. Any missing or mistyped key collapses to a single
/// [`ComposeError::BadRelationship`]. Whether the names are *sound* is a
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

/// Parse one `[requirement.<name>]` table into the typed [`Requirement`]
/// (`specs/architecture/10-contracts.md`). Every facet is optional except the name. Typing is
/// `kind`/`package` **by name** — never inline clauses; a `contract`/`clause`/`match`
/// key is an unknown-key reject (below). Fill is by opt-in `satisfies` alone. Each
/// malformed facet is a load error.
fn parse_requirement(table: &Table, name: &str, path: &Path) -> Result<Requirement, ComposeError> {
    // The closed facet set below; a stray key is a typo that would silently drop
    // meaning or disable a gate, so it is rejected, not ignored (`specs/architecture/10-contracts.md`,
    // unknown keys rejected). The retired `contract` bundle key and inline `clause`
    // array fall here too.
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

/// The requirement's optional `count` bound: an inline `count = { min, max }` table of
/// non-negative integers. Absent ⇒ `None`; any malformation collapses to
/// [`ComposeError::RequirementBadCount`]. Stored verbatim — whether `min > max` (an
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

/// Read one `count` bound (`min`/`max`) off the inline table as a `usize`. Any miss —
/// absent, non-integer, or negative — is `None`, which [`parse_count`] reports as a
/// single [`ComposeError::RequirementBadCount`].
fn count_bound(table: &dyn toml_edit::TableLike, key: &str) -> Option<usize> {
    table
        .get(key)?
        .as_integer()
        .and_then(|n| usize::try_from(n).ok())
}

/// The requirement's optional `degree` bound: an inline `degree = { incoming, outgoing
/// }` table of graph-scope in/out edge-count bounds (`specs/architecture/45-governance.md`). Absent
/// ⇒ `None`. At least one direction must be named, each a `{ min?, max? }` table with
/// at least one non-negative, well-ordered endpoint. Any malformation collapses to
/// [`ComposeError::RequirementBadDegree`]. Decided against the resolved arcs in
/// [`crate::graph`].
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
/// present ⇒ a `{ min?, max? }` table with at least one endpoint and, if both,
/// `min <= max`. An endpoint-less bound (admits every degree) and an inverted one
/// (admits none) are both vacuous, so both fold into
/// [`ComposeError::RequirementBadDegree`].
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
/// `usize`: absent ⇒ `Ok(None)`; non-negative integer ⇒ `Ok(Some)`; anything else ⇒
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

/// Parse the top-level `[reachability]` declaration into a typed [`Reachability`]
/// (`specs/architecture/45-governance.md`, "The world is a node — reachability is a predicate"):
/// the assembly's graph-scope opt-in, carrying the one `severity` key — the author's
/// `required` / `advisory` dial. A closed vocabulary of one key, so any stray key, or a
/// missing / mistyped / out-of-vocabulary `severity`, folds into
/// [`ComposeError::BadReachability`], the way [`parse_degree`] folds its bound's
/// malformations. Presence is the opt-in; the caller has already established the table.
fn parse_reachability(
    table: &dyn toml_edit::TableLike,
    path: &Path,
) -> Result<Reachability, ComposeError> {
    let bad = || ComposeError::BadReachability {
        path: path.to_path_buf(),
    };
    // A stray key is a typo that would silently disable the dial, so it is rejected,
    // not ignored (`specs/architecture/10-contracts.md`, unknown keys rejected).
    for (key, _) in table.iter() {
        if key != "severity" {
            return Err(bad());
        }
    }
    let severity = match table.get("severity").and_then(|item| item.as_str()) {
        Some("required") => contract::Severity::Required,
        Some("advisory") => contract::Severity::Advisory,
        _ => return Err(bad()),
    };
    Ok(Reachability { severity })
}

/// Parse the top-level `authority` posture into a typed [`Authority`]
/// (`specs/architecture/20-surface.md`, "surface authority is a declared posture"): a closed
/// vocabulary of `shared`/`surface`. Absent (`None`) ⇒ [`Authority::Shared`], the
/// default (temper bakes in no stance the author did not declare); any other value —
/// an unknown string or a non-string — is a [`ComposeError::BadAuthority`], the way
/// `[reachability]`'s `severity` rejects an out-of-vocabulary value.
fn parse_authority(item: Option<&toml_edit::Item>, path: &Path) -> Result<Authority, ComposeError> {
    let Some(item) = item else {
        return Ok(Authority::Shared);
    };
    match item.as_str() {
        Some("shared") => Ok(Authority::Shared),
        Some("surface") => Ok(Authority::Surface),
        _ => Err(ComposeError::BadAuthority {
            path: path.to_path_buf(),
        }),
    }
}

/// The requirement's optional `unique` field list: a `unique = ["field", …]` array of
/// declared field names (`specs/architecture/45-governance.md`). Absent ⇒ an empty vec; any
/// malformation collapses to [`ComposeError::RequirementBadUnique`]. Grouping the
/// matched fillers by each is left to [`crate::roster`].
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
/// kind, source, feature }` table (`specs/architecture/45-governance.md`). Absent ⇒ `None`; any
/// malformation collapses to [`ComposeError::RequirementBadMembership`]. Deciding
/// membership against the corpus is left to [`crate::roster`].
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

/// The optional `conforms_to` constraint on a `membership`'s source set — a **typed
/// reference** naming a package by name (`specs/architecture/45-governance.md`), so S₂ is narrowed
/// to the source artifacts that also conform to it. Absent ⇒ `None` (plain
/// membership). A non-string value folds into
/// [`ComposeError::RequirementBadMembership`].
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

/// Re-emit the manifest's `[[member]]` root whole into `doc` from the freshly-extracted
/// `members`, replacing any prior member section and leaving every other root — the
/// hand-authored `kind`/`requirement`/`reachability`/`authority` declarations and their
/// comments — untouched (`specs/architecture/20-surface.md`, "a hand-written manifest is patched
/// format-preserving… an emitted [section] is re-emitted whole"). The member root is
/// **generated-canonical**: it carries nothing of the human's, so it is rebuilt each
/// import rather than merged. An empty `members` drops the root entirely (an empty
/// array-of-tables would vanish on the toml round-trip anyway).
pub fn write_manifest_members(doc: &mut DocumentMut, members: &[ManifestMember]) {
    write_members(doc, members, &[]);
}

/// Serialize a member slice into the standalone `[[member]]` manifest text — a fresh
/// document written whole, the exact byte form the interchange goldens pin
/// (`contract/`, `specs/architecture/50-distribution.md`, "both implementations tested
/// against one golden set"). The single producer of the goldens: `tests/contract_fixtures.rs`
/// byte-matches this against each committed golden, and the SDK's serializer is held to the
/// same bytes.
#[must_use]
pub fn manifest_members_to_string(members: &[ManifestMember]) -> String {
    let mut doc = DocumentMut::new();
    write_manifest_members(&mut doc, members);
    doc.to_string()
}

/// Re-emit the `[[member]]` root whole from both carriages: the pre-extracted
/// `extracted` members (document/module — features baked) and the `inplace` members
/// (the landscape file is the member — a `source` path, joins only, no features). The
/// generated-canonical superset of [`write_manifest_members`]: `init` writes the in-place
/// scan, `init --lift` writes the mixed set as one member migrates into a richer carriage
/// (`specs/architecture/20-surface.md`, "adoption is a gradient"). The root is regenerated whole
/// — see [`write_manifest_members`] for why removing before inserting keeps a re-write
/// byte-stable. An empty pair drops the root entirely.
pub fn write_members(
    doc: &mut DocumentMut,
    extracted: &[ManifestMember],
    inplace: &[InPlaceMember],
) {
    doc.as_table_mut().remove("member");
    if extracted.is_empty() && inplace.is_empty() {
        return;
    }
    let mut tables = ArrayOfTables::new();
    for member in extracted {
        tables.push(member_to_table(member));
    }
    for member in inplace {
        tables.push(inplace_member_to_table(member));
    }
    doc["member"] = Item::ArrayOfTables(tables);
}

/// Serialize one in-place member into its `[[member]]` table: the bare `kind`, `name`,
/// the `source` path that IS the member, and its declared join edges (`satisfies`,
/// `[[member.published]]`). No feature facets — the gate live-extracts those from the
/// landscape file. Each empty join is omitted so an unrecognized member stays terse.
fn inplace_member_to_table(member: &InPlaceMember) -> Table {
    let mut table = Table::new();
    table["kind"] = value(member.kind.clone());
    table["name"] = value(member.name.clone());
    table["source"] = value(member.source.clone());
    if !member.satisfies.is_empty() {
        table["satisfies"] = value(str_array(&member.satisfies));
    }
    if !member.published.is_empty() {
        let mut published = ArrayOfTables::new();
        for requirement in &member.published {
            published.push(published_to_table(requirement));
        }
        table["published"] = Item::ArrayOfTables(published);
    }
    table
}

/// Serialize one member's features into its `[[member]]` table: the bare `kind`, the
/// `name` (id), the body facts (`line_count`, `source_dir`, `headings`, `directives`),
/// the fill edges (`satisfies`), the `[member.field]` frontmatter table, and the
/// `[[member.section]]`/`[[member.fenced]]`/`[[member.published]]` sub-tables. Every
/// empty/absent facet is omitted so a member with no sections (or no unknown keys) stays
/// terse, and an absent facet round-trips back to the same empty default.
fn member_to_table(member: &ManifestMember) -> Table {
    let features = &member.features;
    let mut table = Table::new();
    table["kind"] = value(member.kind.clone());
    table["name"] = value(features.id.clone());
    // A `usize` line count cannot exceed `i64::MAX` for any real body; the saturating
    // fallback keeps the emit total rather than panicking on the impossible case.
    table["line_count"] = value(i64::try_from(features.body_lines).unwrap_or(i64::MAX));
    if let Some(source_dir) = &features.source_dir {
        table["source_dir"] = value(source_dir.clone());
    }
    if !features.headings.is_empty() {
        table["headings"] = value(str_array(&features.headings));
    }
    if !features.directives.is_empty() {
        table["directives"] = value(str_array(&features.directives));
    }
    if !features.satisfies.is_empty() {
        table["satisfies"] = value(str_array(&features.satisfies));
    }
    if !features.fields.is_empty() {
        let mut fields = Table::new();
        for (key, feature) in &features.fields {
            if let Some(val) = feature_to_value(feature) {
                fields[key.as_str()] = Item::Value(val);
            }
        }
        table["field"] = Item::Table(fields);
    }
    if !features.sections.is_empty() {
        let mut sections = ArrayOfTables::new();
        for section in &features.sections {
            let mut entry = Table::new();
            entry["heading"] = value(section.heading.clone());
            entry["body"] = value(section.body.clone());
            sections.push(entry);
        }
        table["section"] = Item::ArrayOfTables(sections);
    }
    if !features.fenced_blocks.is_empty() {
        let mut fenced = ArrayOfTables::new();
        for block in &features.fenced_blocks {
            let mut entry = Table::new();
            entry["info"] = value(block.info.clone());
            entry["content"] = value(block.content.clone());
            fenced.push(entry);
        }
        table["fenced"] = Item::ArrayOfTables(fenced);
    }
    if !features.genres.is_empty() {
        let mut genres = ArrayOfTables::new();
        for genre in &features.genres {
            genres.push(genre_to_table(genre));
        }
        table["genre"] = Item::ArrayOfTables(genres);
    }
    if !features.published_requirements.is_empty() {
        let mut published = ArrayOfTables::new();
        for requirement in &features.published_requirements {
            published.push(published_to_table(requirement));
        }
        table["published"] = Item::ArrayOfTables(published);
    }
    table
}

/// Serialize a published requirement into a `[[member.published]]` table — the same
/// facets a header `[requirement.<name>]` module carries (`means`, `kind`, `package`,
/// `required`), each optional facet omitted when absent so it round-trips to `None`.
fn published_to_table(requirement: &PublishedRequirement) -> Table {
    let mut table = Table::new();
    table["name"] = value(requirement.name.clone());
    if let Some(means) = &requirement.means {
        table["means"] = value(means.clone());
    }
    if let Some(kind) = &requirement.kind {
        table["kind"] = value(kind.clone());
    }
    if let Some(package) = &requirement.package {
        table["package"] = value(package.clone());
    }
    if requirement.required {
        table["required"] = value(true);
    }
    table
}

/// Serialize one [`GenreValue`] into a `[[member.genre]]` table — the genre value
/// serialized **whole** (`specs/architecture/15-kinds.md`): its `genre`/`key` identity, its
/// prose **leaves** as a `[member.genre.leaves]` string table, and its sibling
/// **collections** as `[member.genre.collections.<collection>.<entry>]` keyed sub-tables
/// of string leaves (`specs/architecture/20-surface.md`, "leaf addresses are structural and
/// keyed"). Every leaf is a string; keys are named at every level, never positional, so
/// the manifest carries the same structural address extraction produced — a
/// document-carried and a (future) module-carried genre value serialize identically. Each
/// empty facet is omitted so a leaf-only value stays terse and round-trips to the same
/// empty default.
fn genre_to_table(genre: &GenreValue) -> Table {
    let mut table = Table::new();
    table["genre"] = value(genre.genre.clone());
    table["key"] = value(genre.key.clone());
    if !genre.leaves.is_empty() {
        table["leaves"] = Item::Table(leaf_table(&genre.leaves));
    }
    if !genre.collections.is_empty() {
        let mut collections = Table::new();
        for (name, entries) in &genre.collections {
            let mut collection = Table::new();
            for (entry_key, leaves) in entries {
                collection[entry_key.as_str()] = Item::Table(leaf_table(leaves));
            }
            collections[name.as_str()] = Item::Table(collection);
        }
        table["collections"] = Item::Table(collections);
    }
    table
}

/// A TOML [`Table`] of prose leaves — field name → authored string, the shape a genre
/// value's top-level leaves and each collection entry's leaves both serialize as.
fn leaf_table(leaves: &BTreeMap<String, String>) -> Table {
    let mut table = Table::new();
    for (field, text) in leaves {
        table[field.as_str()] = value(text.clone());
    }
    table
}

/// Serialize one [`FeatureValue`] into a TOML scalar/array/inline-table, or `None` for a
/// value TOML cannot carry (a `null` scalar — dropped exactly as the surface projection
/// drops it). A scalar re-emits in its parsed source kind so the reload re-infers the
/// same [`Kind`]; a list re-emits as a string array (features stringify list elements);
/// a map has no payload, so it re-emits as an empty inline table.
fn feature_to_value(feature: &FeatureValue) -> Option<Value> {
    match feature {
        FeatureValue::Scalar {
            kind: Kind::String,
            text,
        } => Some(Value::from(text.clone())),
        FeatureValue::Scalar {
            kind: Kind::Integer,
            text,
        } => Some(
            text.parse::<i64>()
                .map_or_else(|_| Value::from(text.clone()), Value::from),
        ),
        FeatureValue::Scalar {
            kind: Kind::Number,
            text,
        } => Some(
            text.parse::<f64>()
                .map_or_else(|_| Value::from(text.clone()), Value::from),
        ),
        FeatureValue::Scalar {
            kind: Kind::Boolean,
            text,
        } => Some(Value::from(text == "true")),
        // A `null`, `list`, or `map` *kind* on a `Scalar` cannot occur by construction
        // (the extractor keys those to `List`/`Map`); a null scalar TOML cannot carry.
        FeatureValue::Scalar { .. } => None,
        FeatureValue::List(items) => Some(Value::Array(str_array(items))),
        FeatureValue::Map => Some(Value::InlineTable(InlineTable::new())),
    }
}

/// A TOML string [`Array`] over borrowed strings — the shape `headings`/`directives`/
/// `satisfies`/a list field re-emit as.
fn str_array(items: &[String]) -> Array {
    let mut array = Array::new();
    for item in items {
        array.push(item.as_str());
    }
    array
}

/// Parse one `[[member]]` table into a typed [`ManifestMember`], reconstructing the exact
/// [`Features`] a live extraction yields — the inverse of [`member_to_table`], so a
/// serialized member round-trips. Any missing/mistyped facet folds into a single
/// [`ComposeError::BadMember`] naming its position.
fn parse_member(table: &Table, index: usize, path: &Path) -> Result<ManifestMember, ComposeError> {
    let bad = || ComposeError::BadMember {
        path: path.to_path_buf(),
        index,
    };
    let kind = member_str(table, "kind").ok_or_else(bad)?;
    let id = member_str(table, "name").ok_or_else(bad)?;
    let body_lines = match table.get("line_count") {
        None => 0,
        Some(item) => item
            .as_integer()
            .and_then(|n| usize::try_from(n).ok())
            .ok_or_else(bad)?,
    };
    let source_dir = match table.get("source_dir") {
        None => None,
        Some(item) => Some(item.as_str().ok_or_else(bad)?.to_string()),
    };
    let headings = member_str_array(table, "headings", &bad)?;
    let directives = member_str_array(table, "directives", &bad)?;
    let satisfies = member_str_array(table, "satisfies", &bad)?;
    let fields = parse_member_fields(table);
    let sections = parse_member_sections(table, &bad)?;
    let fenced_blocks = parse_member_fenced(table, &bad)?;
    let genres = parse_member_genres(table, &bad)?;
    let published_requirements = parse_member_published(table, &bad)?;
    Ok(ManifestMember {
        kind,
        features: Features {
            id,
            fields,
            body_lines,
            headings,
            sections,
            source_dir,
            directives,
            fenced_blocks,
            genres,
            satisfies,
            published_requirements,
        },
    })
}

/// Parse one `source`-bearing `[[member]]` table into a typed [`InPlaceMember`] — the
/// inverse of [`inplace_member_to_table`]. Carries the `kind`, `name`, and `source`
/// (all required strings) plus the declared join edges (`satisfies`,
/// `[[member.published]]`); no feature facets, since the gate live-extracts those from
/// the landscape file. Any missing/mistyped facet folds into a single
/// [`ComposeError::BadMember`] naming its position.
fn parse_inplace_member(
    table: &Table,
    index: usize,
    path: &Path,
) -> Result<InPlaceMember, ComposeError> {
    let bad = || ComposeError::BadMember {
        path: path.to_path_buf(),
        index,
    };
    let kind = member_str(table, "kind").ok_or_else(bad)?;
    let name = member_str(table, "name").ok_or_else(bad)?;
    let source = member_str(table, "source").ok_or_else(bad)?;
    let satisfies = member_str_array(table, "satisfies", &bad)?;
    let published = parse_member_published(table, &bad)?;
    Ok(InPlaceMember {
        kind,
        name,
        source,
        satisfies,
        published,
    })
}

/// Read one required string key off a `[[member]]` table: present and a TOML string ⇒
/// `Some`, else `None` (which [`parse_member`] reports as a [`ComposeError::BadMember`]).
fn member_str(table: &Table, key: &str) -> Option<String> {
    Some(table.get(key)?.as_str()?.to_string())
}

/// Read an optional string-array facet off a `[[member]]` table (`headings`, `directives`,
/// `satisfies`): absent ⇒ an empty vec; present-but-not-an-array-of-strings ⇒ the folded
/// [`ComposeError::BadMember`].
fn member_str_array(
    table: &Table,
    key: &str,
    bad: &impl Fn() -> ComposeError,
) -> Result<Vec<String>, ComposeError> {
    match table.get(key) {
        None => Ok(Vec::new()),
        Some(item) => {
            let array = item.as_array().ok_or_else(bad)?;
            let mut out = Vec::with_capacity(array.len());
            for element in array.iter() {
                out.push(element.as_str().ok_or_else(bad)?.to_string());
            }
            Ok(out)
        }
    }
}

/// Rebuild a member's frontmatter `fields` from its `[member.field]` table: each value
/// travels back through the surface's `Item`→JSON→[`FeatureValue`] path
/// ([`document::item_to_json`] + [`extract::json_to_feature`]), so a scalar re-infers its
/// [`Kind`] exactly as extraction does. An absent or malformed `field` table yields no
/// fields (a member may legitimately carry none) — the round-trip's empty default.
fn parse_member_fields(table: &Table) -> BTreeMap<String, FeatureValue> {
    let mut fields = BTreeMap::new();
    if let Some(sub) = table.get("field").and_then(Item::as_table) {
        for (key, item) in sub.iter() {
            if let Some(json) = document::item_to_json(item) {
                fields.insert(key.to_string(), extract::json_to_feature(&json));
            }
        }
    }
    fields
}

/// Rebuild a member's body `sections` from its `[[member.section]]` tables — each a
/// `heading`/`body` string pair. Absent ⇒ empty; a malformed entry folds into
/// [`ComposeError::BadMember`].
fn parse_member_sections(
    table: &Table,
    bad: &impl Fn() -> ComposeError,
) -> Result<Vec<Section>, ComposeError> {
    let Some(item) = table.get("section") else {
        return Ok(Vec::new());
    };
    let array = item.as_array_of_tables().ok_or_else(bad)?;
    let mut sections = Vec::with_capacity(array.len());
    for entry in array.iter() {
        let heading = member_str(entry, "heading").ok_or_else(bad)?;
        let body = member_str(entry, "body").ok_or_else(bad)?;
        sections.push(Section { heading, body });
    }
    Ok(sections)
}

/// Rebuild a member's body `fenced_blocks` from its `[[member.fenced]]` tables — each
/// an `info`/`content` string pair, the inverse of the `fenced` extraction
/// (`specs/architecture/15-kinds.md`, "a fenced block — whose first consumer is the
/// genre fence"). Absent ⇒ empty; a malformed entry folds into
/// [`ComposeError::BadMember`], exactly as [`parse_member_sections`] handles its own.
fn parse_member_fenced(
    table: &Table,
    bad: &impl Fn() -> ComposeError,
) -> Result<Vec<FencedBlock>, ComposeError> {
    let Some(item) = table.get("fenced") else {
        return Ok(Vec::new());
    };
    let array = item.as_array_of_tables().ok_or_else(bad)?;
    let mut blocks = Vec::with_capacity(array.len());
    for entry in array.iter() {
        let info = member_str(entry, "info").ok_or_else(bad)?;
        let content = member_str(entry, "content").ok_or_else(bad)?;
        blocks.push(FencedBlock { info, content });
    }
    Ok(blocks)
}

/// Rebuild a member's `genres` from its `[[member.genre]]` tables — the inverse of
/// [`genre_to_table`], reconstructing the exact [`GenreValue`]s extraction folded
/// (`specs/architecture/20-surface.md`, "Genre values"): the `genre`/`key` identity, the
/// `[member.genre.leaves]` prose leaves, and the
/// `[member.genre.collections.<collection>.<entry>]` keyed sub-tables of string leaves.
/// Absent ⇒ empty; a malformed entry (missing identity, or a non-string leaf) folds into
/// [`ComposeError::BadMember`], exactly as [`parse_member_fenced`] handles its own.
fn parse_member_genres(
    table: &Table,
    bad: &impl Fn() -> ComposeError,
) -> Result<Vec<GenreValue>, ComposeError> {
    let Some(item) = table.get("genre") else {
        return Ok(Vec::new());
    };
    let array = item.as_array_of_tables().ok_or_else(bad)?;
    let mut genres = Vec::with_capacity(array.len());
    for entry in array.iter() {
        let genre = member_str(entry, "genre").ok_or_else(bad)?;
        let key = member_str(entry, "key").ok_or_else(bad)?;
        let leaves = parse_leaf_table(entry.get("leaves"), bad)?;
        let collections = parse_genre_collections(entry.get("collections"), bad)?;
        genres.push(GenreValue {
            genre,
            key,
            leaves,
            collections,
        });
    }
    Ok(genres)
}

/// Rebuild a genre value's sibling **collections** from a `[member.genre.collections]`
/// table — each key a collection name, its value a table of keyed entries, each entry a
/// table of string leaves. Absent ⇒ empty; a non-table at any level, or a non-string
/// leaf, folds into [`ComposeError::BadMember`].
fn parse_genre_collections(
    item: Option<&Item>,
    bad: &impl Fn() -> ComposeError,
) -> Result<GenreCollections, ComposeError> {
    let Some(item) = item else {
        return Ok(BTreeMap::new());
    };
    let table = item.as_table().ok_or_else(bad)?;
    let mut collections = BTreeMap::new();
    for (name, entries_item) in table.iter() {
        let entries_table = entries_item.as_table().ok_or_else(bad)?;
        let mut entries = BTreeMap::new();
        for (entry_key, leaves_item) in entries_table.iter() {
            entries.insert(
                entry_key.to_string(),
                parse_leaf_table(Some(leaves_item), bad)?,
            );
        }
        collections.insert(name.to_string(), entries);
    }
    Ok(collections)
}

/// Rebuild a table of prose **leaves** — field name → authored string — the inverse of
/// [`leaf_table`]. Absent ⇒ empty; a non-table, or a non-string leaf value, folds into
/// [`ComposeError::BadMember`] (leaves are authored strings).
fn parse_leaf_table(
    item: Option<&Item>,
    bad: &impl Fn() -> ComposeError,
) -> Result<BTreeMap<String, String>, ComposeError> {
    let Some(item) = item else {
        return Ok(BTreeMap::new());
    };
    let table = item.as_table().ok_or_else(bad)?;
    let mut leaves = BTreeMap::new();
    for (field, leaf) in table.iter() {
        leaves.insert(
            field.to_string(),
            leaf.as_str().ok_or_else(bad)?.to_string(),
        );
    }
    Ok(leaves)
}

/// Rebuild a member's `published_requirements` from its `[[member.published]]` tables —
/// the demand side of the fill edge, each carrying the same facets a header
/// `[requirement.<name>]` module does. Absent ⇒ empty; a malformed entry folds into
/// [`ComposeError::BadMember`].
fn parse_member_published(
    table: &Table,
    bad: &impl Fn() -> ComposeError,
) -> Result<Vec<PublishedRequirement>, ComposeError> {
    let Some(item) = table.get("published") else {
        return Ok(Vec::new());
    };
    let array = item.as_array_of_tables().ok_or_else(bad)?;
    let mut published = Vec::with_capacity(array.len());
    for entry in array.iter() {
        let name = member_str(entry, "name").ok_or_else(bad)?;
        let required = match entry.get("required") {
            None => false,
            Some(item) => item.as_bool().ok_or_else(bad)?,
        };
        published.push(PublishedRequirement {
            name,
            means: opt_member_str(entry, "means", bad)?,
            kind: opt_member_str(entry, "kind", bad)?,
            package: opt_member_str(entry, "package", bad)?,
            required,
        });
    }
    Ok(published)
}

/// Read an optional string key off a `[[member.published]]` table: absent ⇒ `None`,
/// present-but-not-a-string ⇒ the folded [`ComposeError::BadMember`].
fn opt_member_str(
    table: &Table,
    key: &str,
    bad: &impl Fn() -> ComposeError,
) -> Result<Option<String>, ComposeError> {
    match table.get(key) {
        None => Ok(None),
        Some(item) => Some(item.as_str().ok_or_else(bad).map(str::to_string)).transpose(),
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
        // (`specs/architecture/50-distribution.md`): an *override* (same identity) replaces the
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
        // (`specs/architecture/10-contracts.md`, "all facets optional except its name").
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
        // (`specs/architecture/10-contracts.md`, the typing facet). A leftover `[[requirement.*.clause]]`
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
        // retired: typing is `package` by name (`specs/architecture/10-contracts.md`, the typing
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
        // parse rather than silently dropped (`specs/architecture/10-contracts.md`, "Decision:
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
        // any other (`specs/architecture/40-composition.md`, "Decision: a custom kind is an authored
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

    #[test]
    fn no_reachability_declaration_is_none() {
        // The graph-scope opt-in is opt-in: a `temper.toml` declaring no `[reachability]`
        // leaves it `None`, so `graph::reachable` never runs (`specs/architecture/45-governance.md`).
        let layer = AuthorLayer::parse("[kind.skill]\n", Path::new("temper.toml")).unwrap();
        assert!(layer.reachability().is_none());
    }

    #[test]
    fn a_reachability_declaration_carries_its_severity_dial() {
        // The assembly opts in and declares the dial — `required` here; the parsed value
        // is the author's `required`/`advisory` severity, mapped to the gate weight at
        // dispatch (`specs/architecture/45-governance.md`).
        let required = AuthorLayer::parse(
            "[reachability]\nseverity = \"required\"\n",
            Path::new("temper.toml"),
        )
        .unwrap();
        assert_eq!(
            required.reachability(),
            Some(Reachability {
                severity: contract::Severity::Required
            })
        );

        let advisory = AuthorLayer::parse(
            "[reachability]\nseverity = \"advisory\"\n",
            Path::new("temper.toml"),
        )
        .unwrap();
        assert_eq!(
            advisory.reachability().map(|r| r.severity),
            Some(contract::Severity::Advisory)
        );
    }

    #[test]
    fn a_reachability_without_a_severity_is_a_load_error() {
        // The one required key is `severity` — its absence is not a silent default (that
        // would fabricate a dial the author never set), so it folds to `BadReachability`.
        let err = AuthorLayer::parse("[reachability]\n", Path::new("temper.toml")).unwrap_err();
        assert!(
            matches!(err, ComposeError::BadReachability { .. }),
            "an absent severity is a load error, got: {err:?}"
        );
    }

    #[test]
    fn a_reachability_with_an_out_of_vocab_severity_is_a_load_error() {
        // The dial is the closed `required`/`advisory` vocabulary; `blocking` is not in
        // it, rejected at load rather than coerced.
        let err = AuthorLayer::parse(
            "[reachability]\nseverity = \"blocking\"\n",
            Path::new("temper.toml"),
        )
        .unwrap_err();
        assert!(matches!(err, ComposeError::BadReachability { .. }));
    }

    #[test]
    fn a_reachability_with_a_stray_key_is_a_load_error() {
        // A stray key would silently disable or mis-scope the dial — rejected, not
        // ignored (`specs/architecture/10-contracts.md`, unknown keys rejected).
        let err = AuthorLayer::parse(
            "[reachability]\nseverity = \"required\"\nkind = \"skill\"\n",
            Path::new("temper.toml"),
        )
        .unwrap_err();
        assert!(matches!(err, ComposeError::BadReachability { .. }));
    }

    // ---- serialized member features (the emitted `[[member]]` root) --------

    /// A member carrying every facet — scalar/list/map/int/bool fields, body facts,
    /// sections, fenced blocks, satisfies, and a published requirement — the exhaustive
    /// round-trip subject.
    fn full_member() -> ManifestMember {
        let mut fields = BTreeMap::new();
        fields.insert(
            "name".to_string(),
            FeatureValue::scalar(Kind::String, "coordinate"),
        );
        fields.insert(
            "allowed-tools".to_string(),
            FeatureValue::List(vec!["Task".to_string(), "Read".to_string()]),
        );
        fields.insert(
            "priority".to_string(),
            FeatureValue::scalar(Kind::Integer, "3"),
        );
        fields.insert(
            "enabled".to_string(),
            FeatureValue::scalar(Kind::Boolean, "true"),
        );
        fields.insert("meta".to_string(), FeatureValue::Map);
        ManifestMember {
            kind: "skill".to_string(),
            features: Features {
                id: "coordinate".to_string(),
                fields,
                body_lines: 4,
                headings: vec!["Coordinate".to_string()],
                sections: vec![Section {
                    heading: "Coordinate".to_string(),
                    body: "Drive the team through the playbook.".to_string(),
                }],
                source_dir: Some("coordinate".to_string()),
                directives: vec!["./PLAYBOOK.md".to_string()],
                fenced_blocks: vec![FencedBlock {
                    info: "toml genre.manifest".to_string(),
                    content: "name = \"coordinate\"".to_string(),
                }],
                genres: vec![GenreValue {
                    genre: "decision".to_string(),
                    key: "surface-authority".to_string(),
                    leaves: BTreeMap::from([
                        ("chosen".to_string(), "the surface is canonical".to_string()),
                        (
                            "because".to_string(),
                            "law 7 needs an authored surface".to_string(),
                        ),
                    ]),
                    collections: BTreeMap::from([(
                        "rejected".to_string(),
                        BTreeMap::from([(
                            "baked-projection".to_string(),
                            BTreeMap::from([(
                                "because".to_string(),
                                "a stamping projector breaks law 5".to_string(),
                            )]),
                        )]),
                    )]),
                }],
                satisfies: vec!["dev-standards".to_string()],
                published_requirements: vec![PublishedRequirement {
                    name: "task-planning".to_string(),
                    means: Some("the harness plans tasks".to_string()),
                    kind: Some("skill".to_string()),
                    package: None,
                    required: true,
                }],
            },
        }
    }

    #[test]
    fn a_serialized_member_round_trips_back_to_the_same_features() {
        // The write shape and the read shape are one: serializing a member into the
        // `[[member]]` root and reparsing yields the exact `Features` a live extraction
        // does — the invariant MANIFEST-GATE-READ leans on.
        let member = full_member();
        let mut doc = DocumentMut::new();
        write_manifest_members(&mut doc, std::slice::from_ref(&member));

        let layer = AuthorLayer::parse(&doc.to_string(), Path::new("temper.toml")).unwrap();
        assert_eq!(layer.members(), std::slice::from_ref(&member));
    }

    #[test]
    fn a_minimal_member_round_trips_with_every_optional_facet_absent() {
        // A member with no fields, headings, sections, directives, satisfies, or
        // published requirements: the emitted table omits each empty facet, and the
        // reparse restores the same empty defaults — absence round-trips.
        let member = ManifestMember {
            kind: "spec".to_string(),
            features: Features {
                id: "00-intent".to_string(),
                fields: BTreeMap::new(),
                body_lines: 0,
                headings: Vec::new(),
                sections: Vec::new(),
                source_dir: None,
                directives: Vec::new(),
                fenced_blocks: Vec::new(),
                genres: Vec::new(),
                satisfies: Vec::new(),
                published_requirements: Vec::new(),
            },
        };
        let mut doc = DocumentMut::new();
        write_manifest_members(&mut doc, std::slice::from_ref(&member));
        // The terse table carries only the two required keys and the line count.
        let emitted = doc.to_string();
        assert!(!emitted.contains("headings"));
        assert!(!emitted.contains("[member.field]"));

        let layer = AuthorLayer::parse(&emitted, Path::new("temper.toml")).unwrap();
        assert_eq!(layer.members(), std::slice::from_ref(&member));
    }

    #[test]
    fn writing_no_members_drops_the_member_root_entirely() {
        // An empty member set removes the root — an empty array-of-tables would vanish on
        // the toml round-trip anyway, so the emit matches the reparse (no phantom root).
        let mut doc = "[kind.skill]\npackage = \"skill.anthropic\"\n"
            .parse::<DocumentMut>()
            .unwrap();
        write_manifest_members(&mut doc, &[]);
        assert!(!doc.to_string().contains("member"));
        // The hand-authored binding is untouched.
        assert!(doc.to_string().contains("[kind.skill]"));
    }

    #[test]
    fn the_member_root_re_emits_whole_while_the_authored_roots_are_preserved() {
        // Patch-preservation: writing members over a hand-authored manifest replaces the
        // member root wholesale but leaves the authored binding/requirement (and their
        // comments) verbatim.
        let mut doc = "# hand-authored — keep me\n\
[kind.skill]\n\
package = \"skill.anthropic\"\n\
\n\
[requirement.dev-standards]\n\
required = true\n\
\n\
[[member]]\n\
kind = \"skill\"\n\
name = \"stale\"\n\
line_count = 1\n"
            .parse::<DocumentMut>()
            .unwrap();
        write_manifest_members(&mut doc, std::slice::from_ref(&full_member()));

        let out = doc.to_string();
        assert!(out.contains("# hand-authored — keep me"));
        assert!(out.contains("[kind.skill]"));
        assert!(out.contains("[requirement.dev-standards]"));
        // The stale member is gone, replaced by the freshly-emitted one.
        assert!(!out.contains("stale"));
        assert!(out.contains("name = \"coordinate\""));

        // And the whole thing reparses (the gate load never chokes on the emitted root).
        let layer = AuthorLayer::parse(&out, Path::new("temper.toml")).unwrap();
        assert_eq!(layer.members(), std::slice::from_ref(&full_member()));
        assert!(layer.requirements().contains_key("dev-standards"));
    }

    #[test]
    fn a_member_root_that_is_not_an_array_is_a_load_error() {
        // The `member` root is its own array-of-tables namespace; a scalar in its place
        // is malformed, rejected as a non-array root.
        let err = AuthorLayer::parse("member = 7\n", Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::MemberRootNotArray { .. }));
    }

    #[test]
    fn a_member_missing_its_required_name_is_a_load_error() {
        // Each `[[member]]` carries a `kind` and a `name`; a member missing one folds into
        // a single `BadMember` naming its position.
        let err = AuthorLayer::parse("[[member]]\nkind = \"skill\"\n", Path::new("temper.toml"))
            .unwrap_err();
        assert!(matches!(err, ComposeError::BadMember { index: 0, .. }));
    }
}
