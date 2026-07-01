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
//! - **Adopt** — name the kind's shipped template explicitly (`adopt = "..."`),
//!   the default made visible. Today the sole shipped template per kind *is* the
//!   embedded floor, so the only admissible name is the floor's own; adopting any
//!   other is a load error — a template the tool does not ship cannot be selected.
//!   Omitting `adopt` takes the floor implicitly.
//! - **Extend / override / flip** — an inline `[[kind.<k>.clause]]` array of the
//!   *same* closed-vocabulary clauses a bare contract carries. Each layered clause
//!   either **overrides** the floor clause with the same identity (its predicate
//!   [`key`](crate::contract::Predicate::key) and the field it
//!   [`target`](crate::contract::Predicate::target)s) — which is how a severity
//!   flip (`required` ⟷ `advisory`) and a parameter change are both expressed — or,
//!   when no floor clause shares that identity, **extends** the floor with it.
//!
//! ## The role roster (parse-only)
//!
//! A `temper.toml` may also carry top-level `[role.<name>]` tables — the
//! harness-contract tier (`specs/10-contracts.md`, "Roles and matching"): an
//! abstract role bound to whichever concrete artifact fills it. Each parses into
//! a typed [`Role`] — its artifact kind, the contract the filler must conform to
//! (a template path or inline `[[clause]]`s, the latter through the same
//! [`contract::parse_clauses`]), a decidable [`MatchSelector`] (a name glob or a
//! `role` marker, stored *verbatim* — never matched here), an optional `required`
//! flag (absent ⇒ false; `temper` never fabricates a gate the author did not
//! declare, `00-intent.md` law 4), and an optional `verified_by` verifier.
//!
//! This tier is **parse only**: the roster loads into typed values and a
//! malformed role is a load error, but no selection, single-filler conformance,
//! or admissibility (does `match` resolve, does `verified_by` resolve) runs yet —
//! those are separate follow-on entries.
//!
//! ## The custom-kind declaration (parse-only)
//!
//! The `[kind.<name>]` key serves two authorings (`specs/40-composition.md`,
//! "Declaring a custom kind"): a **built-in contract layer** (adopt a shipped kind
//! and layer clauses over it, above) and a **full custom-kind declaration** — a
//! project's own artifact kind (its specs, ADRs, playbooks) authored in full. The
//! two are disambiguated by structure: a declaration that carries a `governs`
//! locus or an `[[kind.<name>.extraction]]` array — neither of which a built-in
//! layer needs — is a [`CustomKind`], parsed into its file locus, its composed
//! [`Extraction`] (via the shared [`Extraction::from_table`], so an
//! out-of-vocabulary primitive is rejected exactly as in a standalone declaration),
//! and its `[[kind.<name>.clause]]` contract (via the same [`contract::parse_clauses`]).
//! Everything else stays a built-in layer. A declaration triggered as custom but
//! missing its `governs` locus — or carrying a malformed one — is a load error.
//!
//! This tier is **parse only**: custom kinds load into typed values off
//! [`AuthorLayer::custom_kinds`], but discovering units at each kind's `governs`
//! locus and running its extractor over them is a follow-on entry. The
//! `[kind.<name>.entities]` capability is folded in elsewhere, not here; the
//! `.relationships` capability *is* parsed here (below).
//!
//! ## Relationships — a kind capability, not a standalone construct
//!
//! A reference is a **kind capability**, not a free-standing table: a kind declares
//! which of its references are edges under its own `[[kind.<name>.relationships]]`
//! array (`specs/15-kinds.md`, "The entity graph is a kind capability";
//! `specs/40-composition.md`, the `.relationships` surface). The owning kind
//! `<name>` is each edge's *source* (the implicit `from`); each relationship names
//! its reference `field` and its target `to` kind. Relationships are orthogonal to
//! the custom-vs-layer split — a built-in kind layer and a full custom kind declare
//! them the same way — so they are gathered off *every* `[kind.<name>]` table,
//! whichever home the rest of the declaration lands in. They parse into the same
//! [`Edge`] shape [`crate::graph`] consumes; assembling the graph and checking
//! route resolution live there.
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
use crate::kind::{Extraction, KindError};

/// The author-declared layer parsed from a project-root `temper.toml`: a per-kind
/// set of adoptions and clause overrides to apply over the embedded floor.
#[derive(Debug, Clone)]
pub struct AuthorLayer {
    /// The source path, retained so a layering error (an unknown adopted
    /// template) can name the file it came from.
    path: PathBuf,
    /// The per-kind layers, keyed by artifact kind (`skill`, `rule`, …). A kind
    /// the author did not name falls through to the floor unchanged.
    kinds: BTreeMap<String, KindLayer>,
    /// The harness-contract roles parsed from `[role.<name>]` tables, keyed by
    /// role name. Empty when the `temper.toml` declares none. Parse-only in this
    /// tier — selection, required-filling, and admissibility are follow-on
    /// entries.
    roles: BTreeMap<String, Role>,
    /// The named requirements parsed from top-level `[requirement.<name>]` tables,
    /// keyed by requirement name — the meaningful-contract namespace
    /// (`specs/10-contracts.md`, "Requirements and `satisfies`"). Its own namespace,
    /// distinct from the `kind`/`role` maps. Empty when the `temper.toml` declares
    /// none. Parse-only in this tier — coverage over these requirements
    /// (REQUIREMENT-COVERAGE) is a follow-on entry.
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
    /// The fully-declared custom kinds parsed from `[kind.<name>]` tables that
    /// carry a `governs` locus or an `[[kind.<name>.extraction]]` array — a
    /// project's own artifact kinds (its specs, ADRs, playbooks), each composed
    /// from the closed algebras (`specs/40-composition.md`, "Declaring a custom
    /// kind"). Keyed by kind name, **disjoint** from the built-in-layer `kinds`
    /// map: a `[kind.<name>]` is either a built-in contract layer (adopt/clause-
    /// only) or a full custom-kind declaration, never both. Parse-only in this tier
    /// —
    /// discovering units at the `governs` locus and running the extractor are
    /// follow-on entries.
    custom_kinds: BTreeMap<String, CustomKind>,
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

/// A fully-declared **custom kind** parsed from a `[kind.<name>]` table
/// (`specs/40-composition.md`, "Declaring a custom kind"): the one home for a
/// project's own artifact kind (its specs, ADRs, playbooks), composed from the
/// closed algebras (`specs/15-kinds.md`). Where a **built-in** kind is *adopted* —
/// its extraction is temper's and only its contract is layered ([`KindLayer`]) — a
/// **custom** kind is *authored in full*: it declares the file locus it reads
/// ([`governs`](CustomKind::governs)), the composed [`Extraction`] that projects a
/// unit into features, and the [`Clause`]s its contract gates over those features.
/// The `governs`/`extraction` presence is exactly what disambiguates the two homes
/// (see [`is_custom_kind_declaration`]).
///
/// Not `Eq` — its [`clauses`](CustomKind::clauses) may carry `f64` `range` bounds
/// (see [`crate::contract::Contract`]); equality stays derived as `PartialEq`, as
/// it is for [`Role`].
#[derive(Debug, Clone, PartialEq)]
pub struct CustomKind {
    /// The kind's name — the `[kind.<name>]` table key.
    pub name: String,
    /// The file locus the kind reads: a root directory and a filename glob. File
    /// placement is itself an extraction primitive (`specs/40-composition.md`), so
    /// the locus is part of the declaration, not external config.
    pub governs: Governs,
    /// The composed extractor over the closed algebra (`specs/15-kinds.md`), parsed
    /// from the `[[kind.<name>.extraction]]` array by the shared
    /// [`Extraction::from_table`] — so an out-of-vocabulary primitive is rejected at
    /// load exactly as it is for a standalone extraction declaration, no per-kind
    /// escape hatch. Absent ⇒ the vacuous extractor (only the intrinsic id).
    pub extraction: Extraction,
    /// The kind's contract over the extracted features — the `[[kind.<name>.clause]]`
    /// array parsed by the shared [`contract::parse_clauses`], so a clause naming an
    /// unknown predicate is rejected at load just as in a bare contract. Absent ⇒
    /// empty (a kind with no gate; `temper` never fabricates one).
    pub clauses: Vec<Clause>,
}

/// The **file locus** a custom kind reads (`specs/40-composition.md`, "Declaring a
/// custom kind"): the root directory its units live under, and the filename glob
/// that selects them. `import` discovers a custom kind's units by scanning `root`
/// for files matching `glob`. Stored verbatim in this tier — the glob is not
/// compiled or matched here; discovering units at the locus is a follow-on entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Governs {
    /// The root directory the kind's units sit under (`specs`, `docs/adr`), a path
    /// relative to the `temper.toml`.
    pub root: String,
    /// The filename glob that selects the kind's units under `root` (`*.md`,
    /// `[0-9][0-9]-*.md`), stored verbatim.
    pub glob: String,
}

/// A named **requirement** — a semantic intent the harness must fill, declared in
/// a top-level `[requirement.<name>]` table (`specs/10-contracts.md`, "Requirements
/// and `satisfies` — the meaningful contract"). Where a [`Role`] fills a slot by a
/// decidable `match` selector, a requirement is filled by an artifact's opt-in,
/// resolving `satisfies` link and carries the *why* in [`means`](Requirement::means).
///
/// `temper` **never interprets `means`** — it is authored intent the surface carries
/// and organizes, never a thing the engine judges (no proxy; `00-intent.md` law 3).
/// What `check` gates is the decidable shadow — coverage over the parsed
/// requirements — but that is a follow-on entry (REQUIREMENT-COVERAGE); this tier is
/// parse-only. `requirement.` is its own namespace, distinct from the `rule`
/// artifact kind (the Decision's closing note).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requirement {
    /// The semantic intent the requirement states, in *meaning*, not predicates —
    /// "the harness has a skill that maintains development standards". Carried
    /// verbatim and **never interpreted**: `temper` organizes it, never judges it
    /// (`00-intent.md` law 3).
    pub means: String,
    /// Whether an unfilled requirement is a coverage violation. Absent in source ⇒
    /// `false`: `temper` never fabricates a gate the author did not declare
    /// (`00-intent.md` law 4). Gating coverage over this flag is a follow-on entry.
    pub required: bool,
}

/// A harness-contract **role**: an abstract slot bound to whichever concrete
/// artifact fills it (`specs/10-contracts.md`, "Roles and matching"). The engine
/// checks a filler is `present`, `conforms-to` the role's contract, and is picked
/// by its `match` selector — but this tier only *parses* the declaration into
/// typed values; none of those checks run here.
///
/// Not `Eq`: its [`RoleContract`] may carry inline clauses with `f64` `range`
/// bounds (see [`crate::contract::Contract`]); equality stays derived as
/// `PartialEq`.
#[derive(Debug, Clone, PartialEq)]
pub struct Role {
    /// The role's name — the `[role.<name>]` table key.
    pub name: String,
    /// The artifact kind expected to fill the role (`skill`, `command`, …),
    /// stored verbatim. Not validated against a closed kind set in this tier.
    pub artifact: String,
    /// The contract the filling artifact must conform to: an adopted template
    /// named by path, or inline clauses over the closed vocabulary.
    pub contract: RoleContract,
    /// The decidable selector that picks the filling artifact. Stored verbatim —
    /// the glob/marker is *not* evaluated against any surface in this tier.
    pub selector: MatchSelector,
    /// Whether an absent filler is a conformance violation. Absent in source ⇒
    /// `false`: `temper` never fabricates a gate the author did not declare
    /// (`00-intent.md` law 4). Mutually exclusive with [`Role::count`]: `required`
    /// is the single-filler shorthand, `count` the general cardinality form.
    pub required: bool,
    /// An optional bound on the matched-set cardinality — the set-scope `count`
    /// predicate (`specs/45-governance.md`, "The set scope (the roster)"): the
    /// number of artifacts matching the selector must land in `[min, max]`. Absent
    /// ⇒ `None` (no cardinality gate beyond `required`'s single-filler one). The
    /// general form of `required`; the two are mutually exclusive.
    pub count: Option<CountBound>,
    /// The declared field names held unique across the role's matched set — the
    /// set-scope `unique` predicate (`specs/45-governance.md`, "The set scope (the
    /// roster)"): each named field's extracted scalar must not repeat across the
    /// role's matched fillers. Absent ⇒ empty (no uniqueness gate). Generalizes the
    /// kind-wide `unique-name` engine predicate from name-only over a whole kind to
    /// an arbitrary field over a role's matched subset. Checked in [`crate::roster`].
    pub unique: Vec<String>,
    /// An optional set-scope `membership` predicate (`specs/45-governance.md`, "The
    /// set scope (the roster)"): a declared field `F` of every artifact matching the
    /// role's own selector (S₁) must lie in the feature-set drawn from a *second*
    /// matched set (S₂) — "every agent's `model` is one of the approved set." Unlike
    /// the static field `enum`, the allowed set is corpus-*derived*. Absent ⇒ `None`
    /// (no membership gate). Orthogonal to `count`/`unique`/`required`; checked in
    /// [`crate::roster`].
    pub membership: Option<Membership>,
    /// An optional graph-scope `degree` bound (`specs/45-governance.md`, "The graph
    /// scope (the model)"): the in/out edge count of every artifact the role's
    /// selector matches must land in the declared bound over the harness reference
    /// graph. Declared on the role (a set-scope home) but ranging over the *edge*
    /// graph, so it is checked in [`crate::graph`] — reusing the resolved arcs route
    /// resolution and `acyclic` assemble — not the set-scope [`crate::roster`].
    /// Absent ⇒ `None` (no degree gate). Orthogonal to `count`/`unique`/`membership`.
    pub degree: Option<DegreeBound>,
    /// An optional external verifier for the behavioral remainder (`verified_by`).
    /// Stored verbatim; whether it *resolves* is an admissibility check left to a
    /// follow-on entry.
    pub verified_by: Option<String>,
}

/// An inclusive bound on the cardinality of a role's matched set — the set-scope
/// `count` predicate (`specs/45-governance.md`, "The set scope (the roster)"). The
/// number of artifacts the role's selector matches must land in `[min, max]`;
/// "at most N agents" is `{ min = 0, max = N }`, "exactly one planner" is
/// `{ min = 1, max = 1 }`. An inverted `min > max` bound admits no cardinality and
/// is rejected as inadmissible (`crate::roster`), mirroring `range`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountBound {
    /// The inclusive lower bound on the matched-set size.
    pub min: usize,
    /// The inclusive upper bound on the matched-set size.
    pub max: usize,
}

/// The graph-scope `degree` predicate declared on a role — an optional inclusive
/// bound on the **incoming** and/or **outgoing** edge count of every artifact the
/// role's `match` selects over the harness reference graph
/// (`specs/45-governance.md`, "The graph scope (the model)"). Declared on the role
/// (a set-scope home) but ranging over the *edge* graph: "self-registering
/// artifact: zero incoming" is `degree = { incoming = { max = 0 } }`; "routed
/// artifact: at least one incoming" is `degree = { incoming = { min = 1 } }`. At
/// least one direction is present (an empty `degree` constrains nothing — rejected
/// at parse). Deciding a matched node's degree against the resolved arcs lives in
/// [`crate::graph`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DegreeBound {
    /// The bound on a matched node's incoming edge count (how many nodes point at
    /// it). Absent ⇒ `None` (incoming degree is unconstrained).
    pub incoming: Option<EdgeBound>,
    /// The bound on a matched node's outgoing edge count (how many nodes it points
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

/// A set-scope `membership` predicate over a role's matched set (S₁) — the
/// constraint that a declared field `F` of every artifact matching the role's own
/// selector must lie in a *corpus-derived* set, not a static `enum`
/// (`specs/45-governance.md`, "The set scope (the roster)"). The allowed set is the
/// `source_feature` (G) extracted over the artifacts of `source_kind` matched by
/// `source_selector` (S₂) — "every agent's `model` is one of the approved set;" "a
/// hook's binary is one the manifest declares." S₂ may name a different artifact
/// kind than the role's own, so the check ranges over the whole by-kind map. The
/// field-name and selector are stored verbatim; deciding membership lives in
/// [`crate::roster`].
///
/// Not `Eq` — its optional [`source_contract`](Membership::source_contract) may
/// carry inline clauses with `f64` `range` bounds (see [`RoleContract`]); equality
/// stays derived as `PartialEq`, as it is for [`Role`].
#[derive(Debug, Clone, PartialEq)]
pub struct Membership {
    /// The field `F` on each S₁ filler whose extracted scalar must be a member of
    /// the source set. A filler missing `F` carries no value to check.
    pub field: String,
    /// The artifact kind S₂ ranges over (`skill`, `manifest`, …). May differ from
    /// the role's own `artifact`, so the allowed set can be drawn from another kind.
    pub source_kind: String,
    /// The decidable selector picking S₂'s artifacts, stored verbatim — evaluated
    /// against the `source_kind` candidates in [`crate::roster`].
    pub source_selector: MatchSelector,
    /// The feature `G` whose extracted scalars over the S₂ matches form the allowed
    /// set. A source artifact missing `G` contributes nothing to the set.
    pub source_feature: String,
    /// An optional **typed reference** constraint (`conforms_to`,
    /// `specs/45-governance.md`, "The set scope (the roster)"): when set, S₂ is
    /// narrowed to the source artifacts that *also* conform to this contract, so the
    /// reference resolves to the right *kind* of thing — "a reference to an agent of
    /// kind K conforming to contract C." The same [`RoleContract`] a role's `contract`
    /// takes (a template path or inline clauses). Absent ⇒ `None`: plain membership
    /// over every matching source, unchanged. Deciding conformance lives in
    /// [`crate::roster`], reusing the resolve + validate machinery `conformance` runs.
    pub source_contract: Option<RoleContract>,
}

/// A role's contract reference: the filler's contract is either an adopted
/// template named by path, or inline clauses declared under the role
/// (`[[role.<name>.clause]]`) over the same closed vocabulary a bare contract
/// carries (`specs/10-contracts.md`, "Roles and matching").
///
/// Not `Eq` — its inline [`Clause`]s may carry `f64` `range` bounds; equality
/// stays derived as `PartialEq`.
#[derive(Debug, Clone, PartialEq)]
pub enum RoleContract {
    /// A template adopted by path (`contract = "contracts/skill.anthropic.toml"`).
    /// Stored verbatim; whether the path resolves is a follow-on admissibility
    /// check.
    Template(String),
    /// Inline clauses declared under the role, parsed by the shared
    /// [`contract::parse_clauses`] so an unknown predicate is rejected exactly as
    /// in a bare contract.
    Inline(Vec<Clause>),
}

impl RoleContract {
    /// Resolve this reference into the concrete [`Contract`] the engine validates
    /// a role's filler against (`specs/10-contracts.md`, "Roles and matching":
    /// the `role` primitive's `conforms-to` half). `Inline` wraps its already-
    /// parsed clauses directly, labelled `label` (the role name) for diagnostics;
    /// `Template` loads its path **relative to `base_dir`** — the `temper.toml`
    /// directory — and parses it through [`Contract::load`].
    ///
    /// A non-resolving or malformed template path is an *admissibility* concern
    /// (the template-resolve clause of the roster-admissibility follow-on entry),
    /// bubbled here as the [`ContractError`] so the caller can skip the
    /// conformance check rather than double-report what admissibility owns.
    pub fn resolve(&self, base_dir: &Path, label: &str) -> Result<Contract, ContractError> {
        match self {
            RoleContract::Inline(clauses) => Ok(Contract {
                name: label.to_string(),
                clauses: clauses.clone(),
            }),
            RoleContract::Template(rel) => Contract::load(&base_dir.join(rel)),
        }
    }
}

/// The decidable `match` selector picking a role's filler — a closed set
/// (`specs/10-contracts.md`, "Roles and matching"). The pattern is stored
/// *verbatim* and never matched here; resolving it against artifacts is a
/// follow-on entry, so no glob crate enters at this tier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSelector {
    /// By artifact name glob (`match = { name = "plan*" }`).
    Name {
        /// The glob, stored verbatim — not compiled or matched in this tier.
        glob: String,
    },
    /// By an explicit role marker the artifact declares / opts into
    /// (`match = { role = "task-planning" }`) — the "artifact opts in" option.
    Role {
        /// The marker the filling artifact must declare, stored verbatim.
        marker: String,
    },
}

/// One kind's customization: an optional adopted template and the clauses to layer
/// over that kind's floor.
#[derive(Debug, Clone)]
struct KindLayer {
    /// The explicitly adopted template, if the author named one. Validated against
    /// the kind's floor at layering time (`AuthorLayer::layer_over`).
    adopt: Option<String>,
    /// The override / extend clauses, in declaration order.
    clauses: Vec<Clause>,
}

/// Errors raised while loading or applying a `temper.toml`. Hard failures (an
/// unreadable or malformed file, a layer that adopts a template the tool does not
/// ship, a clause outside the closed vocabulary) — distinct from a lint finding,
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

    /// A `[kind.<k>]` layer's `adopt` value is not a string.
    #[error("{path}: `[kind.{kind}]` `adopt` must be a string")]
    #[diagnostic(code(temper::compose::adopt_not_string))]
    AdoptNotString {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The artifact kind whose `adopt` is mistyped.
        kind: String,
    },

    /// A `[kind.<k>]` layer adopts a template the tool does not ship. With only
    /// the embedded floor shipped per kind, the sole admissible name is the floor's
    /// own — selecting anything else is rejected, never silently ignored.
    #[error(
        "{path}: `[kind.{kind}]` adopts unknown template `{adopt}` (the only shipped `{kind}` template is `{expected}`)"
    )]
    #[diagnostic(
        code(temper::compose::unknown_template),
        help("adopt the kind's shipped template by its name, or omit `adopt` to take the floor")
    )]
    UnknownTemplate {
        /// The `temper.toml` that named the template.
        path: PathBuf,
        /// The artifact kind whose layer adopts it.
        kind: String,
        /// The unrecognized adopted template name.
        adopt: String,
        /// The kind's actual shipped (floor) template name.
        expected: String,
    },

    /// A built-in `[kind.<k>]` contract layer carries a key outside its closed set
    /// (`adopt`, `clause`, `relationships`). A misspelled `adpot` is rejected at
    /// parse rather than silently ignored — a typo that quietly disables an
    /// adoption or a clause is the silent gap temper exists to catch, applied to
    /// temper's own parser (`specs/10-contracts.md`, "Decision: unknown keys are
    /// rejected, not ignored"). Mirrors the unknown-predicate reject one rung out to
    /// keys.
    #[error("{path}: `[kind.{kind}]` has unknown key `{key}`")]
    #[diagnostic(
        code(temper::compose::kind_unknown_key),
        help(
            "a built-in kind layer carries only `adopt`, `clause`, and `relationships` — a stray key is a typo, not an escape hatch"
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

    /// The top-level `role` key is present but is not a table of role definitions.
    #[error("{path}: `role` must be a table of harness-contract roles")]
    #[diagnostic(code(temper::compose::role_root_not_table))]
    RoleRootNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
    },

    /// A `[role.<name>]` entry is present but is not a table.
    #[error("{path}: `[role.{role}]` must be a table")]
    #[diagnostic(code(temper::compose::role_not_table))]
    RoleNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role whose definition is malformed.
        role: String,
    },

    /// A `[role.<name>]` is missing its required `artifact` kind.
    #[error("{path}: `[role.{role}]` is missing required key `artifact`")]
    #[diagnostic(code(temper::compose::role_missing_artifact))]
    RoleMissingArtifact {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role missing its artifact kind.
        role: String,
    },

    /// A `[role.<name>]` is missing its required `match` selector.
    #[error("{path}: `[role.{role}]` is missing required key `match`")]
    #[diagnostic(code(temper::compose::role_missing_match))]
    RoleMissingMatch {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role missing its selector.
        role: String,
    },

    /// A `[role.<name>]` key has the wrong TOML type.
    #[error("{path}: `[role.{role}]` key `{key}` must be {expected}")]
    #[diagnostic(code(temper::compose::role_wrong_type))]
    RoleWrongType {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role whose key is mistyped.
        role: String,
        /// The mistyped key.
        key: &'static str,
        /// The type that was expected, for the message.
        expected: &'static str,
    },

    /// A `[role.<name>]` declares neither a `contract` template path nor inline
    /// `[[clause]]`s — a role with no contract names no shape for its filler to
    /// conform to.
    #[error(
        "{path}: `[role.{role}]` must declare a contract — a `contract` template path or inline `[[clause]]`s"
    )]
    #[diagnostic(code(temper::compose::role_no_contract))]
    RoleNoContract {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role missing a contract.
        role: String,
    },

    /// A `[role.<name>]` declares *both* a `contract` template path and inline
    /// `[[clause]]`s — the reference is ambiguous; exactly one is admissible.
    #[error(
        "{path}: `[role.{role}]` declares both a `contract` template path and inline `[[clause]]`s; choose one"
    )]
    #[diagnostic(code(temper::compose::role_ambiguous_contract))]
    RoleAmbiguousContract {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with a doubly-declared contract.
        role: String,
    },

    /// A `[role.<name>]`'s `match` selector is not exactly one of the closed set
    /// (a `name` glob or a `role` marker). Zero, many, or an unknown key all land
    /// here — matching is a decidable selector, never an open guess.
    #[error(
        "{path}: `[role.{role}]` `match` must name exactly one decidable selector (`name` glob or `role` marker)"
    )]
    #[diagnostic(code(temper::compose::role_bad_match))]
    RoleBadMatch {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with the malformed selector.
        role: String,
    },

    /// A `[role.<name>]`'s `count` bound is malformed — not an inline table, or its
    /// `min`/`max` are missing, non-integer, or negative. The matched-set
    /// cardinality bound is a pair of `usize` counts, never an open guess; zero,
    /// missing, mistyped, or negative bounds all land here.
    #[error(
        "{path}: `[role.{role}]` `count` must be an inline table with non-negative integer `min` and `max` bounds"
    )]
    #[diagnostic(code(temper::compose::role_bad_count))]
    RoleBadCount {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with the malformed count bound.
        role: String,
    },

    /// A `[role.<name>]`'s `unique` declaration is malformed — not an array, or an
    /// element that is not a string. The set-scope `unique` predicate names a list
    /// of declared field names, never an open guess; a non-array or a non-string
    /// element lands here.
    #[error("{path}: `[role.{role}]` `unique` must be an array of declared field-name strings")]
    #[diagnostic(code(temper::compose::role_bad_unique))]
    RoleBadUnique {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with the malformed `unique` declaration.
        role: String,
    },

    /// A `[role.<name>]`'s `membership` declaration is malformed — not an inline
    /// table, or missing/mistyped one of its `field`, `kind`, `feature` strings or
    /// its `match` selector. The set-scope `membership` predicate names a field, a
    /// source kind, a source feature, and a decidable second selector; any miss
    /// collapses here, the way [`parse_count`] folds its malformations into one error.
    #[error(
        "{path}: `[role.{role}]` `membership` must be an inline table with `field`, `kind`, `feature` strings and a decidable `match` selector"
    )]
    #[diagnostic(code(temper::compose::role_bad_membership))]
    RoleBadMembership {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with the malformed `membership` declaration.
        role: String,
    },

    /// A `[role.<name>]`'s `degree` declaration is malformed — not an inline table,
    /// naming neither direction, or a direction whose bound is not a `{ min?, max? }`
    /// table with non-negative integer, well-ordered endpoints. The graph-scope
    /// `degree` predicate names an optional incoming/outgoing edge-count bound; an
    /// empty declaration (constraining nothing) or an inverted `min > max` (satisfied
    /// by no node) is as malformed as a mistyped bound. Any miss collapses here, the
    /// way [`parse_count`] folds its malformations into one error.
    #[error(
        "{path}: `[role.{role}]` `degree` must be an inline table naming an `incoming` and/or `outgoing` bound, each a `{{ min?, max? }}` table of non-negative, well-ordered integers"
    )]
    #[diagnostic(code(temper::compose::role_bad_degree))]
    RoleBadDegree {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role with the malformed `degree` declaration.
        role: String,
    },

    /// A `[role.<name>]` declares *both* `required` and a `count` bound. The two
    /// are mutually exclusive: `required` is the single-filler shorthand, `count`
    /// the general cardinality form, so declaring both is ambiguous.
    #[error(
        "{path}: `[role.{role}]` declares both `required` and `count`; they are mutually exclusive (`count` is the general form of `required`'s single-filler bound)"
    )]
    #[diagnostic(code(temper::compose::role_count_and_required))]
    RoleCountAndRequired {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role declaring both.
        role: String,
    },

    /// A `[role.<name>]` carries a key outside its closed set (`artifact`,
    /// `contract`, `clause`, `match`, `required`, `count`, `unique`, `membership`,
    /// `degree`, `verified_by`). A misspelled `requird` is rejected at parse rather
    /// than silently ignored — a typo that quietly disables the gate it was meant to
    /// arm is the silent gap temper exists to catch (`specs/10-contracts.md`,
    /// "Decision: unknown keys are rejected, not ignored"). Mirrors the reject
    /// posture `match` already has for an unknown selector key, one rung out to the
    /// role's own keys.
    #[error("{path}: `[role.{role}]` has unknown key `{key}`")]
    #[diagnostic(
        code(temper::compose::role_unknown_key),
        help(
            "a role carries only `artifact`, `contract`, `clause`, `match`, `required`, `count`, `unique`, `membership`, `degree`, and `verified_by` — a stray key is a typo, not an escape hatch"
        )
    )]
    RoleUnknownKey {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The role carrying the stray key.
        role: String,
        /// The unrecognized key.
        key: String,
    },

    /// The top-level `requirement` key is present but is not a table of requirement
    /// definitions — its own namespace, distinct from `kind`/`role`.
    #[error("{path}: `requirement` must be a table of named requirements")]
    #[diagnostic(code(temper::compose::requirement_root_not_table))]
    RequirementRootNotTable {
        /// The malformed `temper.toml`.
        path: PathBuf,
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

    /// A `[requirement.<name>]` is missing its required `means` — a requirement with
    /// no meaning states no intent for an artifact to fill.
    #[error("{path}: `[requirement.{name}]` is missing required key `means`")]
    #[diagnostic(
        code(temper::compose::requirement_missing_means),
        help(
            "a requirement declares its semantic intent in `means` — the meaning the harness must fill"
        )
    )]
    RequirementMissingMeans {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The requirement missing its meaning.
        name: String,
    },

    /// A `[requirement.<name>]` key has the wrong TOML type — `means` not a string
    /// or `required` not a boolean.
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

    /// A `[requirement.<name>]` carries a key outside its closed set (`means`,
    /// `required`). A misspelled `mean` is rejected at parse rather than silently
    /// ignored — a typo that drops the requirement's whole meaning, or quietly
    /// disables its coverage gate, is the silent gap temper exists to catch
    /// (`specs/10-contracts.md`, "Decision: unknown keys are rejected, not
    /// ignored").
    #[error("{path}: `[requirement.{name}]` has unknown key `{key}`")]
    #[diagnostic(
        code(temper::compose::requirement_unknown_key),
        help(
            "a requirement carries only `means` and `required` — a stray key is a typo, not an escape hatch"
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

    /// A `[kind.<name>]` is a custom-kind declaration (it carries an `extraction`
    /// array) but names no `governs` locus — a custom kind that reads no files is
    /// meaningless, so the locus is required (`specs/40-composition.md`, "Declaring
    /// a custom kind"). A built-in contract layer carries neither `governs` nor
    /// `extraction` and needs neither, so this fires only on the custom path.
    #[error("{path}: custom kind `[kind.{kind}]` is missing required key `governs`")]
    #[diagnostic(
        code(temper::compose::custom_kind_missing_governs),
        help("a custom kind must declare the file locus it reads — a `governs` root and glob")
    )]
    CustomKindMissingGoverns {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The custom kind missing its locus.
        kind: String,
    },

    /// A `[kind.<name>]`'s `governs` locus is malformed — not a table, or
    /// missing/mistyped one of its `root` and `glob` strings. The locus is a root
    /// directory plus a filename glob; any miss collapses here, the way
    /// [`parse_count`] folds its malformations into one error.
    #[error("{path}: `[kind.{kind}]` `governs` must be a table with `root` and `glob` string keys")]
    #[diagnostic(code(temper::compose::bad_governs))]
    BadGoverns {
        /// The malformed `temper.toml`.
        path: PathBuf,
        /// The custom kind with the malformed locus.
        kind: String,
    },

    /// A custom kind's `[[kind.<name>.extraction]]` array is malformed — an
    /// out-of-vocabulary primitive or a mistyped parameter. Bubbled verbatim from
    /// [`crate::kind`] so a custom kind's extraction is held to the exact same
    /// closed algebra a standalone extraction declaration is — no per-kind escape
    /// hatch (`specs/15-kinds.md`).
    #[error(transparent)]
    #[diagnostic(transparent)]
    Extraction(#[from] KindError),

    /// A layered clause is outside the closed vocabulary (or otherwise malformed).
    /// Bubbled verbatim from [`crate::contract`] so the author layer's clauses are
    /// held to the exact same closed-vocabulary contract as a bare one's. Covers a
    /// role's inline `[[clause]]`s too, since they reuse [`contract::parse_clauses`].
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

        let mut kinds = BTreeMap::new();
        let mut custom_kinds = BTreeMap::new();
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
                // Relationships are a kind capability orthogonal to the custom-vs-
                // layer split (`specs/15-kinds.md`): gather them off *every* kind
                // table, the owning `name` each edge's source, before the rest of the
                // declaration lands in whichever home fits.
                edges.extend(parse_relationships(table, name, path)?);
                // A `governs` locus or an `extraction` array marks a full custom-kind
                // declaration; anything else is a built-in contract layer. The two
                // share the `[kind.<name>]` key but land in disjoint homes.
                if is_custom_kind_declaration(table) {
                    custom_kinds.insert(name.to_string(), parse_custom_kind(table, name, path)?);
                } else {
                    kinds.insert(name.to_string(), parse_kind_layer(table, name, path)?);
                }
            }
        }

        let mut roles = BTreeMap::new();
        if let Some(item) = doc.as_table().get("role") {
            let role_table = item
                .as_table()
                .ok_or_else(|| ComposeError::RoleRootNotTable {
                    path: path.to_path_buf(),
                })?;
            for (name, item) in role_table.iter() {
                let table = item.as_table().ok_or_else(|| ComposeError::RoleNotTable {
                    path: path.to_path_buf(),
                    role: name.to_string(),
                })?;
                roles.insert(name.to_string(), parse_role(table, name, path)?);
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
            roles,
            requirements,
            edges,
            custom_kinds,
        })
    }

    /// The parsed role roster, keyed by role name. Empty when the `temper.toml`
    /// declares no `[role.<name>]` tables — a kind-only (or empty) layer carries
    /// an empty roster. Parse-only in this tier.
    #[must_use]
    pub fn roles(&self) -> &BTreeMap<String, Role> {
        &self.roles
    }

    /// The parsed requirements, keyed by requirement name — the meaningful-contract
    /// namespace (`specs/10-contracts.md`, "Requirements and `satisfies`"). Empty
    /// when the `temper.toml` declares no `[requirement.<name>]` tables. Parse-only
    /// in this tier: coverage over these requirements is a follow-on entry
    /// (REQUIREMENT-COVERAGE).
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

    /// The parsed custom kinds, keyed by name. Empty when the `temper.toml`
    /// declares no full `[kind.<name>]` custom kind — a built-in contract layer, if
    /// any, lives off [`layer_over`](Self::layer_over) instead, not here. Parse-only
    /// in this tier: `import` discovering units at each kind's `governs` locus and
    /// running its extractor over them are follow-on entries.
    #[must_use]
    pub fn custom_kinds(&self) -> &BTreeMap<String, CustomKind> {
        &self.custom_kinds
    }

    /// The effective contract for `kind`: this layer's clauses for that kind
    /// applied over `floor`. A kind the author did not name returns `floor`
    /// unchanged, so a `temper.toml` that customizes only some kinds leaves the
    /// rest exactly at the floor.
    ///
    /// Each layered clause **overrides** the floor clause sharing its identity (the
    /// predicate key plus targeted field) — the severity flip and the parameter
    /// change both land here — or, with a new identity, **extends** the floor by
    /// appending. An `adopt` that names a template other than the kind's floor is a
    /// load error: the floor is the only shipped template this tier can select.
    pub fn layer_over(&self, kind: &str, floor: Contract) -> Result<Contract, ComposeError> {
        let Some(layer) = self.kinds.get(kind) else {
            return Ok(floor);
        };

        if let Some(adopt) = &layer.adopt
            && adopt != &floor.name
        {
            return Err(ComposeError::UnknownTemplate {
                path: self.path.clone(),
                kind: kind.to_string(),
                adopt: adopt.clone(),
                expected: floor.name.clone(),
            });
        }

        let mut clauses = floor.clauses;
        for clause in &layer.clauses {
            match clauses
                .iter()
                .position(|existing| same_identity(existing, clause))
            {
                Some(index) => clauses[index] = clause.clone(),
                None => clauses.push(clause.clone()),
            }
        }
        Ok(Contract {
            name: floor.name,
            clauses,
        })
    }
}

/// The effective contract for `kind` given an *optional* author layer: `floor`
/// unchanged when there is no layer, else [`AuthorLayer::layer_over`] applied. The
/// `Option` seam keeps the absent-`temper.toml` path — every existing test's path —
/// a verbatim pass-through of the floor.
pub fn effective(
    layer: Option<&AuthorLayer>,
    kind: &str,
    floor: Contract,
) -> Result<Contract, ComposeError> {
    match layer {
        Some(layer) => layer.layer_over(kind, floor),
        None => Ok(floor),
    }
}

/// Parse one `[kind.<k>]` table into its [`KindLayer`] — the optional `adopt`
/// template name and the inline `[[clause]]` array, the latter through the shared
/// closed-vocabulary parser ([`contract::parse_clauses`]).
fn parse_kind_layer(table: &Table, kind: &str, path: &Path) -> Result<KindLayer, ComposeError> {
    // A built-in layer carries only `adopt` and `clause`; `relationships` is a kind
    // capability gathered off every kind table before this point (`parse_relationships`),
    // so it is admissible here too. Anything else is a typo, rejected — not dropped.
    for (key, _) in table.iter() {
        if !matches!(key, "adopt" | "clause" | "relationships") {
            return Err(ComposeError::KindUnknownKey {
                path: path.to_path_buf(),
                kind: kind.to_string(),
                key: key.to_string(),
            });
        }
    }
    let adopt = match table.get("adopt") {
        None => None,
        Some(item) => Some(
            item.as_str()
                .ok_or_else(|| ComposeError::AdoptNotString {
                    path: path.to_path_buf(),
                    kind: kind.to_string(),
                })?
                .to_string(),
        ),
    };
    let clauses = contract::parse_clauses(table, path)?;
    Ok(KindLayer { adopt, clauses })
}

/// Whether a `[kind.<name>]` table is a **full custom-kind declaration** rather
/// than a built-in contract layer. The two share the `[kind.<name>]` key
/// (`specs/40-composition.md`, "Declaring a custom kind"): a custom kind authors a
/// file locus and an extraction, while a built-in layer only adopts a shipped kind
/// and layers clauses over it. The presence of a `governs` locus or an `extraction`
/// array — neither of which a built-in layer carries — disambiguates the two.
fn is_custom_kind_declaration(table: &Table) -> bool {
    table.contains_key("governs") || table.contains_key("extraction")
}

/// Parse one `[kind.<name>]` custom-kind declaration into a typed [`CustomKind`]:
/// its required `governs` locus, the composed `[[kind.<name>.extraction]]`
/// extractor (via the shared [`Extraction::from_table`]), and the
/// `[[kind.<name>.clause]]` contract (via the shared [`contract::parse_clauses`]).
/// Each malformed part is a load error, mirroring `[kind.<k>]` layer and
/// `[role.<name>]` parsing — a custom kind earns no escape hatch the floor lacks.
fn parse_custom_kind(table: &Table, name: &str, path: &Path) -> Result<CustomKind, ComposeError> {
    let governs = parse_governs(table, name, path)?;
    let extraction = Extraction::from_table(table, path)?;
    let clauses = contract::parse_clauses(table, path)?;
    Ok(CustomKind {
        name: name.to_string(),
        governs,
        extraction,
        clauses,
    })
}

/// The custom kind's required `governs` locus: a `governs = { root, glob }` table
/// whose `root` and `glob` are strings. Absent ⇒
/// [`ComposeError::CustomKindMissingGoverns`]; not a table, or a missing/mistyped
/// key ⇒ [`ComposeError::BadGoverns`], the way [`parse_count`] folds its
/// malformations into one error. The two names are stored verbatim; compiling the
/// glob and discovering units at the locus are follow-on entries.
fn parse_governs(table: &Table, kind: &str, path: &Path) -> Result<Governs, ComposeError> {
    let item = table
        .get("governs")
        .ok_or_else(|| ComposeError::CustomKindMissingGoverns {
            path: path.to_path_buf(),
            kind: kind.to_string(),
        })?;
    let bad = || ComposeError::BadGoverns {
        path: path.to_path_buf(),
        kind: kind.to_string(),
    };
    let governs = item.as_table_like().ok_or_else(bad)?;
    let root = governs_str(governs, "root").ok_or_else(bad)?;
    let glob = governs_str(governs, "glob").ok_or_else(bad)?;
    Ok(Governs { root, glob })
}

/// Read one required string key off a `governs` locus table: present and a TOML
/// string ⇒ `Some`, else `None` (which [`parse_governs`] reports as a single
/// [`ComposeError::BadGoverns`]).
fn governs_str(table: &dyn toml_edit::TableLike, key: &str) -> Option<String> {
    Some(table.get(key)?.as_str()?.to_string())
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

/// Parse one `[requirement.<name>]` table into a typed [`Requirement`]: the required
/// `means` string (carried verbatim, never interpreted — `00-intent.md` law 3) and
/// the optional `required` flag (absent ⇒ `false`; `temper` never fabricates a gate
/// the author did not declare — law 4). A missing or non-string `means`, or a
/// non-boolean `required`, is a load error, mirroring the `[role.<name>]` parse path.
fn parse_requirement(table: &Table, name: &str, path: &Path) -> Result<Requirement, ComposeError> {
    // A requirement carries only `means` and `required` — a stray key (a misspelled
    // `mean`, a `requird`) is a typo that would silently drop the meaning or disable
    // the coverage gate, so it is rejected at parse, never ignored.
    for (key, _) in table.iter() {
        if !matches!(key, "means" | "required") {
            return Err(ComposeError::RequirementUnknownKey {
                path: path.to_path_buf(),
                name: name.to_string(),
                key: key.to_string(),
            });
        }
    }
    let means = match table.get("means") {
        None => {
            return Err(ComposeError::RequirementMissingMeans {
                path: path.to_path_buf(),
                name: name.to_string(),
            });
        }
        Some(item) => item
            .as_str()
            .ok_or_else(|| ComposeError::RequirementWrongType {
                path: path.to_path_buf(),
                name: name.to_string(),
                key: "means",
                expected: "a string",
            })?
            .to_string(),
    };
    let required = match table.get("required") {
        None => false,
        Some(item) => item
            .as_bool()
            .ok_or_else(|| ComposeError::RequirementWrongType {
                path: path.to_path_buf(),
                name: name.to_string(),
                key: "required",
                expected: "a boolean",
            })?,
    };
    Ok(Requirement { means, required })
}

/// Parse one `[role.<name>]` table into a typed [`Role`]: the required `artifact`
/// kind and `match` selector, the contract reference (a `contract` path string or
/// an inline `[[clause]]` array — exactly one), the optional `required` flag
/// (absent ⇒ `false`), and the optional `verified_by` verifier. Each malformed
/// field is a load error, mirroring `[kind.<k>]` parsing.
fn parse_role(table: &Table, role: &str, path: &Path) -> Result<Role, ComposeError> {
    // A role carries only the closed set below; a stray key (a misspelled `requird`,
    // a `matches`) is a typo that would silently disable the gate it was meant to
    // arm, so it is rejected at parse, never dropped — the reject posture `match`
    // holds for an unknown selector key, one rung out to the role's own keys.
    for (key, _) in table.iter() {
        if !matches!(
            key,
            "artifact"
                | "contract"
                | "clause"
                | "match"
                | "required"
                | "count"
                | "unique"
                | "membership"
                | "degree"
                | "verified_by"
        ) {
            return Err(ComposeError::RoleUnknownKey {
                path: path.to_path_buf(),
                role: role.to_string(),
                key: key.to_string(),
            });
        }
    }
    let artifact = role_str(table, "artifact", role, path)?.ok_or_else(|| {
        ComposeError::RoleMissingArtifact {
            path: path.to_path_buf(),
            role: role.to_string(),
        }
    })?;
    let contract = parse_role_contract(table, role, path)?;
    let selector = parse_match(table, role, path)?;
    // `required` and `count` are two ways to express the same dimension (matched-set
    // cardinality), so declaring both is ambiguous — reject it before parsing either.
    if table.contains_key("required") && table.contains_key("count") {
        return Err(ComposeError::RoleCountAndRequired {
            path: path.to_path_buf(),
            role: role.to_string(),
        });
    }
    let required = parse_role_required(table, role, path)?;
    let count = parse_count(table, role, path)?;
    let unique = parse_unique(table, role, path)?;
    let membership = parse_membership(table, role, path)?;
    let degree = parse_degree(table, role, path)?;
    let verified_by = role_str(table, "verified_by", role, path)?;

    Ok(Role {
        name: role.to_string(),
        artifact,
        contract,
        selector,
        required,
        count,
        unique,
        membership,
        degree,
        verified_by,
    })
}

/// The role's optional `count` bound: an inline `count = { min, max }` table whose
/// `min` and `max` are non-negative integers (`usize`). Absent ⇒ `None`. Any
/// malformation — not a table, a missing/mistyped/negative bound — collapses to
/// [`ComposeError::RoleBadCount`], the way [`parse_match`] folds its malformations
/// into one error. The bound is stored verbatim; whether `min > max` (an
/// unsatisfiable bound) is an *admissibility* concern, checked in [`crate::roster`].
fn parse_count(table: &Table, role: &str, path: &Path) -> Result<Option<CountBound>, ComposeError> {
    let Some(item) = table.get("count") else {
        return Ok(None);
    };
    let bad_count = || ComposeError::RoleBadCount {
        path: path.to_path_buf(),
        role: role.to_string(),
    };
    let count_table = item.as_table_like().ok_or_else(bad_count)?;
    let min = count_bound(count_table, "min").ok_or_else(bad_count)?;
    let max = count_bound(count_table, "max").ok_or_else(bad_count)?;
    Ok(Some(CountBound { min, max }))
}

/// Read one `count` bound (`min`/`max`) off the inline table as a `usize`: present,
/// a TOML integer, and non-negative. Any miss — absent, a non-integer, or a
/// negative value (`usize` cannot hold one) — is `None`, which [`parse_count`]
/// reports as a single [`ComposeError::RoleBadCount`].
fn count_bound(table: &dyn toml_edit::TableLike, key: &str) -> Option<usize> {
    table
        .get(key)?
        .as_integer()
        .and_then(|n| usize::try_from(n).ok())
}

/// The role's optional `degree` bound: an inline `degree = { incoming = { min?,
/// max? }, outgoing = { min?, max? } }` table naming a graph-scope in/out edge-count
/// bound (`specs/45-governance.md`, "The graph scope (the model)"). Absent ⇒ `None`.
/// At least one direction must be named, and each present direction is a `{ min?,
/// max? }` table with at least one non-negative, well-ordered integer endpoint —
/// "self-registering" is `{ incoming = { max = 0 } }`, "routed" is `{ incoming =
/// { min = 1 } }`. Any malformation — not a table, naming no direction, a mistyped or
/// negative endpoint, an endpoint-less or inverted bound — collapses to
/// [`ComposeError::RoleBadDegree`], the way [`parse_count`] folds its malformations
/// into one error. The bound is stored verbatim; deciding a matched node's degree
/// against the resolved arcs lives in [`crate::graph`].
fn parse_degree(
    table: &Table,
    role: &str,
    path: &Path,
) -> Result<Option<DegreeBound>, ComposeError> {
    let Some(item) = table.get("degree") else {
        return Ok(None);
    };
    let bad = || ComposeError::RoleBadDegree {
        path: path.to_path_buf(),
        role: role.to_string(),
    };
    let degree = item.as_table_like().ok_or_else(bad)?;
    let incoming = parse_edge_bound(degree, "incoming", role, path)?;
    let outgoing = parse_edge_bound(degree, "outgoing", role, path)?;
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
/// [`ComposeError::RoleBadDegree`] — mirroring [`parse_count`]'s single folded error.
fn parse_edge_bound(
    table: &dyn toml_edit::TableLike,
    direction: &str,
    role: &str,
    path: &Path,
) -> Result<Option<EdgeBound>, ComposeError> {
    let Some(item) = table.get(direction) else {
        return Ok(None);
    };
    let bad = || ComposeError::RoleBadDegree {
        path: path.to_path_buf(),
        role: role.to_string(),
    };
    let bound = item.as_table_like().ok_or_else(bad)?;
    let min = edge_endpoint(bound, "min", role, path)?;
    let max = edge_endpoint(bound, "max", role, path)?;
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
/// [`ComposeError::RoleBadDegree`]. Unlike [`count_bound`], absence is distinguished
/// from malformation so an omitted endpoint means "unbounded on that side," not an
/// error.
fn edge_endpoint(
    table: &dyn toml_edit::TableLike,
    key: &str,
    role: &str,
    path: &Path,
) -> Result<Option<usize>, ComposeError> {
    match table.get(key) {
        None => Ok(None),
        Some(item) => {
            let value = item
                .as_integer()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or_else(|| ComposeError::RoleBadDegree {
                    path: path.to_path_buf(),
                    role: role.to_string(),
                })?;
            Ok(Some(value))
        }
    }
}

/// The role's optional `unique` field list: a `unique = ["field", …]` array of
/// declared field names, each held unique across the role's matched set by the
/// roster check (`specs/45-governance.md`, "The set scope (the roster)"). Absent ⇒
/// an empty vec (no uniqueness gate). Any malformation — not an array, or a
/// non-string element — collapses to [`ComposeError::RoleBadUnique`], the way
/// [`parse_count`] folds its malformations into one error. The names are stored
/// verbatim; grouping the matched fillers by each is left to [`crate::roster`].
fn parse_unique(table: &Table, role: &str, path: &Path) -> Result<Vec<String>, ComposeError> {
    let Some(item) = table.get("unique") else {
        return Ok(Vec::new());
    };
    let bad_unique = || ComposeError::RoleBadUnique {
        path: path.to_path_buf(),
        role: role.to_string(),
    };
    let array = item.as_array().ok_or_else(bad_unique)?;
    let mut fields = Vec::new();
    for value in array.iter() {
        fields.push(value.as_str().ok_or_else(bad_unique)?.to_string());
    }
    Ok(fields)
}

/// The role's optional `membership` predicate: an inline `membership = { field,
/// kind, match, feature }` table naming the constrained field `F` (on the role's
/// own matched set), the source artifact kind and second selector `S₂`, and the
/// source feature `G` whose values form the allowed set (`specs/45-governance.md`,
/// "The set scope (the roster)"). Absent ⇒ `None`. Any malformation — not a table,
/// a missing/mistyped string, or a malformed `match` selector — collapses to
/// [`ComposeError::RoleBadMembership`], the way [`parse_count`] folds its
/// malformations into one error. The field names and selector are stored verbatim;
/// deciding membership against the corpus is left to [`crate::roster`].
fn parse_membership(
    table: &Table,
    role: &str,
    path: &Path,
) -> Result<Option<Membership>, ComposeError> {
    let Some(item) = table.get("membership") else {
        return Ok(None);
    };
    let bad = || ComposeError::RoleBadMembership {
        path: path.to_path_buf(),
        role: role.to_string(),
    };
    let membership = item.as_table_like().ok_or_else(bad)?;
    let field = membership_str(membership, "field").ok_or_else(bad)?;
    let source_kind = membership_str(membership, "kind").ok_or_else(bad)?;
    let source_feature = membership_str(membership, "feature").ok_or_else(bad)?;
    let source_selector = membership
        .get("match")
        .and_then(selector_from)
        .ok_or_else(bad)?;
    let source_contract = parse_conforms_to(membership, role, path)?;
    Ok(Some(Membership {
        field,
        source_kind,
        source_selector,
        source_feature,
        source_contract,
    }))
}

/// The optional `conforms_to` constraint on a `membership`'s source set — the
/// **typed reference** (`specs/45-governance.md`, "The set scope (the roster)"):
/// the same [`RoleContract`] a role's `contract` takes, either a template path
/// string (`conforms_to = "contracts/…​.toml"`) or an inline `[[…​.conforms_to.clause]]`
/// array parsed by the shared [`contract::parse_clauses`] — so an out-of-vocabulary
/// predicate is rejected exactly as in a role's own inline contract, no escape
/// hatch. Absent ⇒ `None` (plain membership). A structurally malformed value —
/// neither a string nor a clause-bearing sub-table — folds into
/// [`ComposeError::RoleBadMembership`], the way every other membership miss does.
fn parse_conforms_to(
    table: &dyn toml_edit::TableLike,
    role: &str,
    path: &Path,
) -> Result<Option<RoleContract>, ComposeError> {
    let Some(item) = table.get("conforms_to") else {
        return Ok(None);
    };
    if let Some(template) = item.as_str() {
        return Ok(Some(RoleContract::Template(template.to_string())));
    }
    // A clause-bearing sub-table (`[role.<name>.membership.conforms_to]` with its
    // own `[[…​.clause]]` array) reuses the shared closed-vocabulary parser, so a
    // vocabulary error bubbles as the `ContractError` exactly as a role's inline
    // clauses do. Anything else — a number, a bare inline table with no clauses —
    // is a malformed `membership`.
    let sub = item
        .as_table()
        .ok_or_else(|| ComposeError::RoleBadMembership {
            path: path.to_path_buf(),
            role: role.to_string(),
        })?;
    Ok(Some(RoleContract::Inline(contract::parse_clauses(
        sub, path,
    )?)))
}

/// Read one required string key off an inline table-like (a `membership` field):
/// present and a TOML string ⇒ `Some`, else `None` (which [`parse_membership`]
/// reports as a single [`ComposeError::RoleBadMembership`]).
fn membership_str(table: &dyn toml_edit::TableLike, key: &str) -> Option<String> {
    Some(table.get(key)?.as_str()?.to_string())
}

/// Parse a [`MatchSelector`] out of a `match` item that is itself an inline table
/// naming exactly one decidable selector (a `name` glob or a `role` marker) with a
/// string value. Returns `None` on any malformation — not a table, zero/many keys,
/// an unknown key, or a non-string value — so a caller can collapse it into its own
/// error (the membership `match` folds into [`ComposeError::RoleBadMembership`]).
/// Mirrors the closed selector set [`parse_match`] enforces for a role's own `match`.
fn selector_from(item: &toml_edit::Item) -> Option<MatchSelector> {
    let table = item.as_table_like()?;
    let mut selector = None;
    for (key, value) in table.iter() {
        if selector.is_some() {
            // A second selector key — `match` must name exactly one.
            return None;
        }
        let pattern = value.as_str()?;
        selector = Some(match key {
            "name" => MatchSelector::Name {
                glob: pattern.to_string(),
            },
            "role" => MatchSelector::Role {
                marker: pattern.to_string(),
            },
            _ => return None,
        });
    }
    selector
}

/// The role's contract reference — exactly one of a `contract` template path or
/// an inline `[[role.<name>.clause]]` array. Declaring neither names no shape;
/// declaring both is ambiguous; both are load errors. Inline clauses go through
/// the shared [`contract::parse_clauses`], so an unknown predicate is rejected
/// just as in a bare contract.
fn parse_role_contract(
    table: &Table,
    role: &str,
    path: &Path,
) -> Result<RoleContract, ComposeError> {
    let template = role_str(table, "contract", role, path)?;
    let has_clauses = table.contains_key("clause");
    match (template, has_clauses) {
        (Some(_), true) => Err(ComposeError::RoleAmbiguousContract {
            path: path.to_path_buf(),
            role: role.to_string(),
        }),
        (Some(template), false) => Ok(RoleContract::Template(template)),
        (None, true) => Ok(RoleContract::Inline(contract::parse_clauses(table, path)?)),
        (None, false) => Err(ComposeError::RoleNoContract {
            path: path.to_path_buf(),
            role: role.to_string(),
        }),
    }
}

/// The role's `match` selector: the inline `match` table must name exactly one of
/// the closed set — a `name` glob or a `role` marker — whose value is a string.
/// Absent ⇒ [`ComposeError::RoleMissingMatch`]; zero/many/unknown keys ⇒
/// [`ComposeError::RoleBadMatch`]. The pattern is stored verbatim, never matched.
fn parse_match(table: &Table, role: &str, path: &Path) -> Result<MatchSelector, ComposeError> {
    let item = table
        .get("match")
        .ok_or_else(|| ComposeError::RoleMissingMatch {
            path: path.to_path_buf(),
            role: role.to_string(),
        })?;
    let selector_table = item
        .as_table_like()
        .ok_or_else(|| ComposeError::RoleWrongType {
            path: path.to_path_buf(),
            role: role.to_string(),
            key: "match",
            expected: "an inline table",
        })?;

    let bad_match = || ComposeError::RoleBadMatch {
        path: path.to_path_buf(),
        role: role.to_string(),
    };

    let mut selector = None;
    for (key, value) in selector_table.iter() {
        if selector.is_some() {
            // A second selector key — `match` must name exactly one.
            return Err(bad_match());
        }
        let pattern = value.as_str().ok_or_else(|| ComposeError::RoleWrongType {
            path: path.to_path_buf(),
            role: role.to_string(),
            key: "match",
            expected: "a string selector value",
        })?;
        selector = Some(match key {
            "name" => MatchSelector::Name {
                glob: pattern.to_string(),
            },
            "role" => MatchSelector::Role {
                marker: pattern.to_string(),
            },
            _ => return Err(bad_match()),
        });
    }
    selector.ok_or_else(bad_match)
}

/// The role's optional `required` flag: absent ⇒ `false` (`temper` never
/// fabricates a gate the author did not declare); present-but-not-a-boolean ⇒ a
/// load error.
fn parse_role_required(table: &Table, role: &str, path: &Path) -> Result<bool, ComposeError> {
    match table.get("required") {
        None => Ok(false),
        Some(item) => item.as_bool().ok_or_else(|| ComposeError::RoleWrongType {
            path: path.to_path_buf(),
            role: role.to_string(),
            key: "required",
            expected: "a boolean",
        }),
    }
}

/// Read an optional string key off a `[role.<name>]` table: absent ⇒ `None`,
/// present-but-not-a-string ⇒ [`ComposeError::RoleWrongType`].
fn role_str(
    table: &Table,
    key: &'static str,
    role: &str,
    path: &Path,
) -> Result<Option<String>, ComposeError> {
    match table.get(key) {
        None => Ok(None),
        Some(item) => {
            Some(
                item.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| ComposeError::RoleWrongType {
                        path: path.to_path_buf(),
                        role: role.to_string(),
                        key,
                        expected: "a string",
                    }),
            )
            .transpose()
        }
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

    /// A small skill-shaped floor: a required `max_len` on `name`, a required
    /// `forbidden_keys`, and an advisory `max_lines`. Enough distinct identities to
    /// exercise override-vs-extend.
    fn floor() -> Contract {
        Contract {
            name: "skill.anthropic".to_string(),
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
        assert_eq!(layer.layer_over("skill", floor()).unwrap(), floor());
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
        let effective = layer.layer_over("skill", floor()).unwrap();

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
        let effective = layer.layer_over("skill", floor()).unwrap();

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
        let effective = layer.layer_over("skill", floor()).unwrap();

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
        let effective = layer.layer_over("skill", floor).unwrap();

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
    fn adopting_the_floor_template_by_name_is_the_default_made_explicit() {
        // Naming the kind's own floor template is admissible and changes nothing.
        let toml = r#"
[kind.skill]
adopt = "skill.anthropic"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        assert_eq!(layer.layer_over("skill", floor()).unwrap(), floor());
    }

    #[test]
    fn adopting_a_template_the_tool_does_not_ship_is_a_load_error() {
        let toml = r#"
[kind.skill]
adopt = "skill.cursor"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let err = layer.layer_over("skill", floor()).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::UnknownTemplate { ref adopt, ref expected, .. }
                if adopt == "skill.cursor" && expected == "skill.anthropic"
        ));
    }

    #[test]
    fn an_empty_temper_toml_and_an_absent_one_both_yield_the_floor() {
        // Present-but-declares-nothing parses to a layer with no kinds, so every
        // kind falls through to the floor — the same result as `effective(None,..)`.
        let layer = AuthorLayer::parse("# nothing here\n", Path::new("temper.toml")).unwrap();
        assert_eq!(layer.layer_over("skill", floor()).unwrap(), floor());
        assert_eq!(effective(None, "skill", floor()).unwrap(), floor());
        assert_eq!(effective(Some(&layer), "skill", floor()).unwrap(), floor());
    }

    #[test]
    fn load_returns_none_for_an_absent_file_and_some_for_a_present_one() {
        let dir = tmpdir("load");
        let path = dir.join("temper.toml");
        // Absent ⇒ None (the floor-only path).
        assert!(AuthorLayer::load(&path).unwrap().is_none());

        // Present ⇒ Some, parsed from disk.
        fs::write(&path, "[kind.skill]\nadopt = \"skill.anthropic\"\n").unwrap();
        let layer = AuthorLayer::load(&path)
            .unwrap()
            .expect("a present file loads");
        assert_eq!(layer.layer_over("skill", floor()).unwrap(), floor());
    }

    #[test]
    fn a_non_table_kind_entry_is_a_load_error() {
        let err = AuthorLayer::parse("kind = 7\n", Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::KindRootNotTable { .. }));
    }

    // ---- role roster (parse-only) -----------------------------------------

    #[test]
    fn a_full_role_table_parses_into_a_typed_role() {
        // Every field present: artifact kind, a path-string contract, a name-glob
        // selector, an explicit `required`, and a `verified_by` verifier.
        let toml = r#"
[role.task-planning]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "plan*" }
required = true
verified_by = "tests/plan.rs"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer
            .roles()
            .get("task-planning")
            .expect("the role parses into the roster");
        assert_eq!(
            role,
            &Role {
                name: "task-planning".to_string(),
                artifact: "skill".to_string(),
                contract: RoleContract::Template("contracts/skill.anthropic.toml".to_string()),
                selector: MatchSelector::Name {
                    glob: "plan*".to_string(),
                },
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
    fn an_inline_clause_contract_parses_via_the_shared_parser() {
        // No `contract` path: the role's contract is inline `[[clause]]`s, parsed
        // by the same closed-vocabulary parser a bare contract uses. The selector
        // is the opt-in `role` marker form.
        let toml = r#"
[role.release-tool]
artifact = "command"
match = { role = "release" }
[[role.release-tool.clause]]
severity = "required"
predicate = "required"
field = "description"
[[role.release-tool.clause]]
severity = "required"
predicate = "must_define"
marker = "executable"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("release-tool").expect("the role parses");
        assert_eq!(
            role.selector,
            MatchSelector::Role {
                marker: "release".to_string(),
            }
        );
        assert_eq!(
            role.contract,
            RoleContract::Inline(vec![
                Clause {
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::Required {
                        field: "description".to_string(),
                    },
                },
                Clause {
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::MustDefine {
                        marker: "executable".to_string(),
                    },
                },
            ])
        );
    }

    #[test]
    fn an_absent_required_flag_defaults_to_false() {
        // `temper` never fabricates a gate the author did not declare: an absent
        // `required` is `false`, not `true`.
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("linter").expect("the role parses");
        assert!(!role.required);
        assert_eq!(role.verified_by, None);
    }

    #[test]
    fn an_unknown_predicate_in_an_inline_role_contract_is_a_load_error() {
        // The shared parser rejects an out-of-vocabulary predicate in a role's
        // inline clauses exactly as it does in a bare contract — no escape hatch.
        let toml = r#"
[role.linter]
artifact = "skill"
match = { name = "lint*" }
[[role.linter.clause]]
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
    fn a_role_missing_its_artifact_kind_is_a_load_error() {
        let toml = r#"
[role.linter]
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleMissingArtifact { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_role_with_neither_a_contract_nor_inline_clauses_is_a_load_error() {
        let toml = r#"
[role.linter]
artifact = "skill"
match = { name = "lint*" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleNoContract { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_role_with_both_a_contract_and_inline_clauses_is_a_load_error() {
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*" }
[[role.linter.clause]]
severity = "required"
predicate = "required"
field = "name"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleAmbiguousContract { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_role_missing_its_match_selector_is_a_load_error() {
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleMissingMatch { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_match_with_an_unknown_selector_key_is_a_load_error() {
        // `path` is not in the closed selector set {name, role}.
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { path = "skills/lint" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleBadMatch { ref role, .. } if role == "linter"
        ));
    }

    #[test]
    fn a_match_naming_two_selectors_is_a_load_error() {
        // Exactly one selector — `name` and `role` together is ambiguous.
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*", role = "lint" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadMatch { .. }));
    }

    #[test]
    fn a_non_boolean_required_flag_is_a_load_error() {
        let toml = r#"
[role.linter]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "lint*" }
required = "yes"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleWrongType {
                key: "required",
                ..
            }
        ));
    }

    #[test]
    fn a_non_table_role_root_is_a_load_error() {
        let err = AuthorLayer::parse("role = 7\n", Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleRootNotTable { .. }));
    }

    #[test]
    fn a_count_bound_parses_into_a_typed_role() {
        // The set-scope `count` predicate: an inline `{ min, max }` table parses
        // into a `CountBound`, and (being the general form of `required`) no
        // `required` flag rides alongside it.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = { min = 0, max = 3 }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert_eq!(role.count, Some(CountBound { min: 0, max: 3 }));
        assert!(!role.required);
    }

    #[test]
    fn a_non_table_count_is_a_load_error() {
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = 3
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleBadCount { ref role, .. } if role == "agents"
        ));
    }

    #[test]
    fn a_count_with_a_non_integer_bound_is_a_load_error() {
        // A `max` that is not a non-negative integer collapses to `RoleBadCount`,
        // the way a malformed `match` collapses to `RoleBadMatch`.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = { min = 0, max = "three" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadCount { .. }));
    }

    #[test]
    fn a_count_missing_a_bound_is_a_load_error() {
        // Both `min` and `max` are required — the bound is a closed pair, never a
        // half-open guess.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = { max = 3 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadCount { .. }));
    }

    #[test]
    fn a_negative_count_bound_is_a_load_error() {
        // A negative `min` cannot be a `usize` cardinality — rejected, not floored.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
count = { min = -1, max = 3 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadCount { .. }));
    }

    #[test]
    fn declaring_both_required_and_count_is_a_load_error() {
        // The two express the same dimension (matched-set cardinality); declaring
        // both is ambiguous, so it is rejected before either is read.
        let toml = r#"
[role.agents]
artifact = "agent"
contract = "contracts/agent.toml"
match = { name = "agent-*" }
required = true
count = { min = 0, max = 3 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleCountAndRequired { ref role, .. } if role == "agents"
        ));
    }

    #[test]
    fn a_unique_field_list_parses_into_a_typed_role() {
        // The set-scope `unique` predicate: a `unique = ["model"]` array parses into
        // `Role.unique`, the declared fields the roster holds unique across the set.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
unique = ["model"]
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert_eq!(role.unique, vec!["model".to_string()]);
    }

    #[test]
    fn an_absent_unique_defaults_to_an_empty_vec() {
        // `temper` never fabricates a gate the author did not declare: an absent
        // `unique` is no uniqueness gate, an empty vec.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert!(role.unique.is_empty());
    }

    #[test]
    fn a_non_array_unique_is_a_load_error() {
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
unique = "model"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleBadUnique { ref role, .. } if role == "agents"
        ));
    }

    #[test]
    fn a_unique_with_a_non_string_element_is_a_load_error() {
        // A non-string element collapses to `RoleBadUnique`, the way a malformed
        // `count` bound collapses to `RoleBadCount`.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
unique = ["model", 7]
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadUnique { .. }));
    }

    #[test]
    fn a_membership_clause_parses_into_a_typed_role() {
        // The set-scope `membership` predicate: an inline `{ field, kind, match,
        // feature }` table parses into a `Membership`, naming the constrained field
        // F, the source kind and second selector S₂, and the source feature G.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
membership = { field = "model", kind = "manifest", match = { name = "approved-models" }, feature = "model" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert_eq!(
            role.membership,
            Some(Membership {
                field: "model".to_string(),
                source_kind: "manifest".to_string(),
                source_selector: MatchSelector::Name {
                    glob: "approved-models".to_string(),
                },
                source_feature: "model".to_string(),
                source_contract: None,
            })
        );
    }

    #[test]
    fn a_membership_with_a_conforms_to_template_path_parses() {
        // The typed-reference form: `conforms_to` names a template path, so S₂ is
        // narrowed to sources conforming to that contract. It parses into
        // `source_contract: Some(Template(..))`, the same `RoleContract` a role's
        // own `contract` takes.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
membership = { field = "model", kind = "manifest", match = { name = "approved-*" }, feature = "model", conforms_to = "contracts/approved.toml" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert_eq!(
            role.membership.as_ref().unwrap().source_contract,
            Some(RoleContract::Template(
                "contracts/approved.toml".to_string()
            ))
        );
    }

    #[test]
    fn a_membership_with_inline_conforms_to_clauses_parses() {
        // The typed-reference form can also carry inline clauses, declared under a
        // `[role.<name>.membership.conforms_to]` sub-table with its own `[[clause]]`
        // array — parsed by the shared closed-vocabulary parser into
        // `source_contract: Some(Inline(..))`.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }

[role.agents.membership]
field = "model"
kind = "manifest"
feature = "model"
match = { name = "approved-*" }

[[role.agents.membership.conforms_to.clause]]
severity = "required"
predicate = "required"
field = "model"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert_eq!(
            role.membership.as_ref().unwrap().source_contract,
            Some(RoleContract::Inline(vec![Clause {
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::Required {
                    field: "model".to_string(),
                },
            }]))
        );
    }

    #[test]
    fn an_unknown_predicate_in_a_conforms_to_clause_is_a_load_error() {
        // The `conforms_to` clauses go through the same closed-vocabulary parser a
        // role's inline contract does — an out-of-vocabulary predicate is rejected at
        // load, no escape hatch.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }

[role.agents.membership]
field = "model"
kind = "manifest"
feature = "model"
match = { name = "approved-*" }

[[role.agents.membership.conforms_to.clause]]
severity = "required"
predicate = "word_count"
field = "model"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::Contract(ContractError::UnknownPredicate { ref predicate, .. })
                if predicate == "word_count"
        ));
    }

    #[test]
    fn a_membership_with_a_malformed_conforms_to_is_a_load_error() {
        // `conforms_to` must be a template-path string or a clause-bearing sub-table;
        // a bare number is neither, so it folds into `RoleBadMembership`.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
membership = { field = "model", kind = "manifest", match = { name = "approved-*" }, feature = "model", conforms_to = 7 }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleBadMembership { ref role, .. } if role == "agents"
        ));
    }

    #[test]
    fn a_membership_with_a_role_marker_source_selector_parses() {
        // S₂ may select by the opt-in `role` marker just as a role's own `match`
        // can — the selector is the same closed set.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
membership = { field = "model", kind = "skill", match = { role = "approved" }, feature = "model" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert_eq!(
            role.membership.as_ref().unwrap().source_selector,
            MatchSelector::Role {
                marker: "approved".to_string(),
            }
        );
    }

    #[test]
    fn an_absent_membership_defaults_to_none() {
        // `temper` never fabricates a gate the author did not declare: an absent
        // `membership` is no gate at all.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let role = layer.roles().get("agents").expect("the role parses");
        assert!(role.membership.is_none());
    }

    #[test]
    fn a_non_table_membership_is_a_load_error() {
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
membership = "model"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::RoleBadMembership { ref role, .. } if role == "agents"
        ));
    }

    #[test]
    fn a_membership_missing_a_required_key_is_a_load_error() {
        // `feature` (the source feature G) is required — its absence collapses to
        // `RoleBadMembership`, the way a missing `count` bound collapses to
        // `RoleBadCount`.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
membership = { field = "model", kind = "manifest", match = { name = "approved" } }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadMembership { .. }));
    }

    #[test]
    fn a_membership_with_a_malformed_source_selector_is_a_load_error() {
        // The source `match` must name exactly one decidable selector; `path` is not
        // in the closed set {name, role}, so the whole clause is malformed.
        let toml = r#"
[role.agents]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "agent-*" }
membership = { field = "model", kind = "manifest", match = { path = "x" }, feature = "model" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::RoleBadMembership { .. }));
    }

    #[test]
    fn a_kind_only_temper_toml_carries_an_empty_roster() {
        // Customizing only `[kind.*]` leaves the role roster empty — and the kind
        // layer still works exactly as before.
        let toml = r#"
[kind.skill]
[[kind.skill.clause]]
severity = "advisory"
predicate = "max_lines"
max = 100
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        assert!(layer.roles().is_empty());
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
    fn relationships_parse_alongside_a_built_in_layer_and_a_custom_kind() {
        // Relationships are orthogonal to the custom-vs-layer split: they parse off a
        // built-in kind layer (`[kind.rule]`, adopt/clause) and a full custom kind
        // (`[kind.spec]`, with a `governs` locus) the same way, gathered off both.
        let toml = r#"
[kind.rule]
adopt = "rule"
[[kind.rule.relationships]]
field = "routes_to"
to = "skill"

[kind.spec]
governs = { root = "specs", glob = "*.md" }
[[kind.spec.extraction]]
primitive = "references"
feature = "references"
[[kind.spec.relationships]]
field = "references"
to = "spec"
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        // The custom kind still lands in the custom-kind map; the built-in layer does
        // not — the relationships change neither classification.
        assert!(layer.custom_kinds().contains_key("spec"));
        assert!(!layer.custom_kinds().contains_key("rule"));
        // Both relationships are gathered as edges, each `from` its owning kind.
        let edges: Vec<(&str, &str, &str)> = layer
            .edges()
            .iter()
            .map(|e| (e.from.as_str(), e.field.as_str(), e.to.as_str()))
            .collect();
        assert!(edges.contains(&("rule", "routes_to", "skill")));
        assert!(edges.contains(&("spec", "references", "spec")));
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
[role.planner]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"
match = { name = "plan*" }
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
        // `RoleBadCount`.
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

    // ---- custom-kind declarations (parse-only) ----------------------------

    #[test]
    fn a_full_custom_kind_declaration_parses_into_a_typed_kind() {
        // `governs` + `[[kind.spec.extraction]]` + `[[kind.spec.clause]]` — the full
        // custom-kind declaration composes the closed algebras into a `CustomKind`.
        let toml = r#"
[kind.spec]
governs = { root = "specs", glob = "*.md" }

[[kind.spec.extraction]]
primitive = "line_count"

[[kind.spec.extraction]]
primitive = "references"
feature = "references"

[[kind.spec.clause]]
severity = "advisory"
predicate = "max_lines"
max = 400
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let spec = layer
            .custom_kinds()
            .get("spec")
            .expect("the custom kind parses");
        assert_eq!(spec.name, "spec");
        assert_eq!(
            spec.governs,
            Governs {
                root: "specs".to_string(),
                glob: "*.md".to_string(),
            }
        );
        // The extraction is composed via the shared closed-algebra parser.
        assert_eq!(
            spec.extraction.primitives(),
            &[
                crate::kind::Primitive::LineCount,
                crate::kind::Primitive::References {
                    feature: "references".to_string(),
                },
            ]
        );
        // The contract is parsed via the same closed-vocabulary clause parser.
        assert_eq!(
            spec.clauses,
            vec![Clause {
                severity: Severity::Advisory,
                guidance: None,
                predicate: Predicate::MaxLines { max: 400 },
            }]
        );
    }

    #[test]
    fn a_built_in_layer_and_a_custom_kind_land_in_disjoint_homes() {
        // `[kind.skill]` adopts + layers over a shipped kind (a built-in contract
        // layer); `[kind.spec]` declares `governs` (a custom kind). The
        // `governs`/`extraction` presence routes each to its own home.
        let toml = r#"
[kind.skill]
adopt = "skill.anthropic"
[[kind.skill.clause]]
severity = "advisory"
predicate = "max_lines"
max = 300

[kind.spec]
governs = { root = "specs", glob = "*.md" }
[[kind.spec.clause]]
severity = "advisory"
predicate = "max_lines"
max = 400
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();

        // `spec` is a custom kind; `skill` is not — it stays a built-in layer.
        assert!(layer.custom_kinds().contains_key("spec"));
        assert!(!layer.custom_kinds().contains_key("skill"));

        // The `skill` layer still applies over the floor exactly as before: its
        // advisory `max_lines` override lands in the effective contract.
        let effective = layer.layer_over("skill", floor()).unwrap();
        assert!(
            effective
                .clauses
                .iter()
                .any(|c| matches!(c.predicate, Predicate::MaxLines { max: 300 }))
        );
    }

    #[test]
    fn a_custom_kind_may_omit_extraction_and_clauses() {
        // `governs` alone still marks a custom kind; an absent extraction is the
        // vacuous extractor and an absent clause array an empty (no-gate) contract —
        // `temper` never fabricates either.
        let toml = r#"
[kind.spec]
governs = { root = "specs", glob = "*.md" }
"#;
        let layer = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap();
        let spec = layer.custom_kinds().get("spec").expect("parses");
        assert!(spec.extraction.primitives().is_empty());
        assert!(spec.clauses.is_empty());
    }

    #[test]
    fn a_custom_kind_missing_its_governs_locus_is_a_load_error() {
        // An `[[kind.spec.extraction]]` array marks this custom, but it names no
        // `governs` locus — a custom kind that reads no files is malformed.
        let toml = r#"
[kind.spec]
[[kind.spec.extraction]]
primitive = "line_count"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::CustomKindMissingGoverns { ref kind, .. } if kind == "spec"
        ));
    }

    #[test]
    fn a_custom_kind_with_a_malformed_governs_is_a_load_error() {
        // `governs` must be a table with `root` and `glob` strings; a bare string is
        // neither, so it folds into `BadGoverns`.
        let toml = r#"
[kind.spec]
governs = "specs"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::BadGoverns { ref kind, .. } if kind == "spec"
        ));
    }

    #[test]
    fn a_custom_kind_governs_missing_a_key_is_a_load_error() {
        // Both `root` and `glob` are required — a half-declared locus collapses to
        // `BadGoverns`, the way a missing `count` bound collapses to `RoleBadCount`.
        let toml = r#"
[kind.spec]
governs = { root = "specs" }
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(err, ComposeError::BadGoverns { .. }));
    }

    #[test]
    fn an_unknown_extraction_primitive_in_a_custom_kind_is_a_load_error() {
        // The extraction array goes through the same closed-algebra parser a
        // standalone declaration does — an out-of-vocabulary primitive is rejected
        // at load, bubbled as the `KindError`.
        let toml = r#"
[kind.spec]
governs = { root = "specs", glob = "*.md" }
[[kind.spec.extraction]]
primitive = "paragraph_meaning"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::Extraction(KindError::UnknownPrimitive { ref primitive, .. })
                if primitive == "paragraph_meaning"
        ));
    }

    #[test]
    fn an_unknown_predicate_in_a_custom_kind_clause_is_a_load_error() {
        // The custom kind's clauses reuse the shared closed-vocabulary parser, so an
        // unknown predicate is rejected exactly as in a bare contract.
        let toml = r#"
[kind.spec]
governs = { root = "specs", glob = "*.md" }
[[kind.spec.clause]]
severity = "required"
predicate = "word_count"
field = "body"
"#;
        let err = AuthorLayer::parse(toml, Path::new("temper.toml")).unwrap_err();
        assert!(matches!(
            err,
            ComposeError::Contract(ContractError::UnknownPredicate { ref predicate, .. })
                if predicate == "word_count"
        ));
    }
}
