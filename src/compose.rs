//! The harness assembly's domain types — [`Requirement`], [`Edge`], [`EnforcementMode`]
//! — and the lock-row lift [`default_contract_from_rows`], which builds a kind's whole
//! default contract from the clause rows naming it. A requirement's
//! set-/edge-scope demands ride ordinary [`contract::Clause`] values;
//! their predicate payloads ([`contract::EdgeBound`] and
//! friends) live in [`crate::contract`], not here.
//!
//! Member composition functions that resolve kinds and units live here too:
//! [`resolve_kind_units`], [`manifest_units`], [`kind_features`], [`assemble_lock_family`],
//! and [`assemble_by_kind`], along with their supporting functions. These are the corpus-assembly
//! functions that discover and resolve a harness's members off disk.
//!
//! There is no reader in this module: every value here is populated from the lock's
//! declaration rows (`crate::drift::Declarations`), the sole producer since `emit`
//! compiles the SDK program. These are the shared shapes the gate lifts lock rows
//! into and [`crate::roster`]/[`crate::graph`]/[`crate::coverage`] range over.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::builtin_kind;
use crate::contract::{self, Contract};
use crate::dial;
use crate::document;
use crate::drift::{self, ClauseRow};
use crate::extract;
use crate::frontmatter;
use crate::graph;
use crate::import;
use crate::json_manifest;
use crate::kind::{self, CollectionAddress, CustomKind, Unit};
use crate::layout::Layout;
use crate::toml_document;
use walkdir;

thread_local! {
    static RESOLVE_KIND_UNITS_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static OVERLAY_BUILTIN_KIND_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// This thread's cumulative count of `resolve_kind_units` invocations.
#[must_use]
pub fn resolve_kind_units_count() -> usize {
    RESOLVE_KIND_UNITS_COUNT.with(std::cell::Cell::get)
}

/// This thread's cumulative count of `overlay_builtin_kind` invocations.
#[must_use]
pub fn overlay_builtin_kind_count() -> usize {
    OVERLAY_BUILTIN_KIND_COUNT.with(std::cell::Cell::get)
}

/// A cache of manifest files read during one gate()/explain() invocation: one read per
/// manifest file path, shared across all kinds that govern it. Keys are manifest paths
/// (e.g., ".claude/settings.json"), values are the parsed manifest and its opaque fields.
pub type ManifestCache =
    BTreeMap<PathBuf, (json_manifest::Manifest, BTreeMap<String, serde_json::Value>)>;

/// A registered custom kind as the corpus construction carries it: its loaded
/// [`CustomKind`] definition (identity travels on `.name` — no separate borrowed name
/// column) and its computed member [`extract::Features`]. Named so the
/// shared corpus helpers keep a legible signature (`clippy::type_complexity`).
pub type CustomKindEntry = (CustomKind, Vec<extract::Features>);

/// The harness's declared **enforcement mode** — how firmly the guard binds a tool
/// call, split by where the finding goes: a closed vocabulary the author declares on
/// the root member, never a stance temper bakes in.
/// Defaults to [`Warn`](EnforcementMode::Warn).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnforcementMode {
    /// Allows the call and records the finding out-of-band only — the next report,
    /// never the live session. The newly expressible tier; unreachable until an
    /// author declares it.
    Note,
    /// Allows the call and surfaces the finding in-band, into the live context. The
    /// default: enforcement mode is author-declared per placement, never assumed.
    #[default]
    Warn,
    /// Denies the call.
    Block,
}

/// A declared **edge relationship** — a kind capability declared on the owning kind's
/// members. The owning kind is the edge *source*
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
    /// The non-empty set of artifact kinds the reference may resolve into — the edge
    /// *targets*. A one-element set resolves a bare address within its one kind; a
    /// multi-element set resolves only the kind-qualified `kind:name` address, whose
    /// kind names which member of the set. Every declared kind must be one `temper`
    /// models, else that element's routes can never resolve (a graph-admissibility
    /// concern, [`crate::graph`]).
    pub to: Vec<String>,
}

/// A requirement's **typed verifier** — the declared delegate that judges the
/// behavioral remainder, resolved at admissibility and never run. Two species this
/// slice; a probe stays a documented pattern until a consumer types its transcript
/// surface. One shared shape: [`crate::drift::RequirementRow`] carries it on the
/// wire and [`crate::roster`] resolves over it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, ts_rs::TS)]
#[serde(tag = "species", rename_all = "snake_case")]
pub enum Verifier {
    /// A path-resolved reference to the test or CI job that executes the judgment —
    /// resolved by whether its `path` exists under the harness root.
    Script {
        /// The test/CI path, relative to the harness root.
        path: String,
    },
    /// Named documented harness events the emitted tap records to a local-locus log —
    /// resolved by whether each name is a documented harness event.
    Telemetry {
        /// The harness lifecycle event names this verifier reads the tap for.
        events: Vec<String>,
    },
}

/// A named **requirement** — the harness's named obligation, declared in the
/// assembly's `[requirement.<name>]`. **Every facet is optional
/// except the name.** Fill is by the artifact's opt-in `satisfies` alone — there is
/// no name-`match` selector.
///
/// `temper` **carries `prose` verbatim, never interprets it** — it is authored
/// intent the surface carries, never a thing the engine judges. The decidable
/// shadow is what `check` gates: [`crate::coverage`] over the `satisfies` edges,
/// [`crate::roster`]/[`crate::graph`] over the **satisfier set** (the artifacts of
/// its `kind` that opt in via `satisfies`).
#[derive(Debug, Clone, PartialEq)]
pub struct Requirement {
    /// The requirement's name.
    pub name: String,
    /// The authored *intent*, stated in meaning not predicates. Carried verbatim and
    /// **never interpreted**.
    pub prose: Option<String>,
    /// The requirement's declared satisfier kind. Unlike the old name-`match`
    /// selector, this never narrows *which* opt-in artifacts are candidates —
    /// [`crate::roster`]/[`crate::graph`] draw the satisfier set kind-blind from
    /// every modeled kind, the opt-in `satisfies` join the sole filter.
    /// Present, it instead
    /// *sources* the shipped each-grain "every satisfier is kind K" clause
    /// [`crate::engine::judge`] evaluates over that kind-blind set — a satisfier of
    /// a different kind is a finding, never a silent exclusion. Absent ⇒
    /// **kind-blind**: any artifact that opts in fills it, and no narrowing clause
    /// attaches at all.
    pub kind: Option<String>,
    /// Whether an unfilled requirement is a gate-blocking violation. Absent ⇒ `false`
    /// (`temper` never fabricates a gate the author did not declare
    /// "Declared, never mined"). Never cardinality — posture and the set-scope `count` clause in
    /// [`clauses`](Requirement::clauses) are different kinds of thing.
    pub required: bool,
    /// The requirement's set-/edge-scope demands — ordinary [`contract::Clause`]
    /// values whose predicates range over the satisfier set and its graph
    /// neighborhood. Each carries its own severity/guidance/cite; empty ⇒ no set-scope
    /// demand at all. `count`/`unique`/`membership` are checked in
    /// [`crate::roster`]; `degree` ranges over the *edge* graph, so it is checked in
    /// [`crate::graph`] instead.
    pub clauses: Vec<contract::Clause>,
    /// The typed verifier for the behavioral remainder, when declared. Stored as its
    /// declared species; whether it *resolves* — a script's path, a telemetry event's
    /// documented name — is an admissibility check ([`crate::roster`]).
    pub verifier: Option<Verifier>,
}

/// A kind's whole default [`Contract`], built directly from the clause rows naming it
/// in the committed lock — the one lift both a custom kind (which carries no embedded
/// default: its committed rows **are** its contract) and a built-in kind whose lock
/// declares rows run through. A built-in kind the lock names no row for falls back to
/// its embedded default instead ([`crate::builtin::contract`]); either way rows-or-
/// default, never a severity-flip layer over the embedded default. This is the same
/// lift [`crate::builtin::contract`] runs over the *embedded* lock's own rows, run here
/// over the committed lock's.
///
/// # Errors
///
/// A row naming a predicate outside the closed vocabulary, or missing an argument its
/// predicate requires, is a [`ClauseRowError`] — the lock is tool-written, never
/// hand-patched (`specs/model/pipeline.md`), so a row the closed vocabulary cannot
/// admit is a corrupt lock rejected loud, never a clause silently dropped.
pub fn default_contract_from_rows(
    clauses: &[ClauseRow],
    kind: &str,
) -> Result<Contract, ClauseRowError> {
    let clauses = clauses
        .iter()
        .filter(|row| row.kind.as_deref() == Some(kind))
        .map(clause_from_row)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Contract {
        name: kind.to_string(),
        clauses,
        guidance: None,
    })
}

/// A contract with the clause rows of the invocation's joined locks naming `kind`
/// appended to it — the host's own contract, hardened.
///
/// The joined rows are lifted by the same [`clause_from_row`] the host's are: whatever
/// lock carried a row, it composes into the host corpus's selection for the kind it
/// names, and nothing downstream can tell the two apart. Appending is the whole
/// operation, and it is what bounds a layer to hardening: a joined row never replaces,
/// reorders, or deletes a host clause, so the host's reviewed contract still judges
/// every member it judged before. A joined row that would weaken a host clause simply
/// reports beside the one it cannot displace — visible, and inert on the verdict.
///
/// # Errors
///
/// As [`default_contract_from_rows`]: a joined row the closed vocabulary cannot admit
/// is a corrupt lock, refused rather than dropped.
pub fn with_joined_clauses(
    mut contract: Contract,
    joined: &[ClauseRow],
    kind: &str,
) -> Result<Contract, ClauseRowError> {
    for row in joined
        .iter()
        .filter(|row| row.kind.as_deref() == Some(kind))
    {
        contract.clauses.push(clause_from_row(row)?);
    }
    Ok(contract)
}

/// A lock clause row the closed predicate vocabulary cannot admit — an unknown
/// predicate or one missing its required argument, or an out-of-vocabulary severity.
/// Surfaced as a load error rather than a silent skip: the lock is tool-written, so a
/// row the SDK could not have emitted is corruption, not a tolerable hand-edit
/// (`specs/model/contract.md`, "clause": an unknown predicate is rejected at load).
#[derive(Debug, Clone, thiserror::Error, miette::Diagnostic)]
pub enum ClauseRowError {
    /// The row names a predicate outside the closed vocabulary, or omits an argument
    /// its predicate requires — either way no clause can be built.
    #[error(
        "lock clause row names predicate `{predicate}`, which is not in the closed \
         vocabulary or is missing a required argument"
    )]
    Predicate {
        /// The offending row's predicate key.
        predicate: String,
    },
    /// The row's severity label is outside the closed `required`/`advisory` vocabulary.
    #[error(
        "lock clause row for predicate `{predicate}` declares severity `{severity}`, \
         outside the closed `required`/`advisory` vocabulary"
    )]
    Severity {
        /// The offending row's predicate key.
        predicate: String,
        /// The unrecognized severity label.
        severity: String,
    },
    /// The row carries no address. Emit stamps one onto every row it writes, so a row
    /// without one never came from an emit.
    #[error(
        "lock clause row for predicate `{predicate}` carries no `label` — every emitted \
         clause row is stamped with its address, so a row without one is not a row emit wrote"
    )]
    Label {
        /// The offending row's predicate key.
        predicate: String,
    },
}

/// Lift one clause row into its typed [`contract::Clause`] — its address, predicate,
/// severity, guidance, and cite.
/// `pub` (not `pub(crate)`): the `main` binary is a separate crate from this
/// library, so its requirement-nested lift needs this visible across the crate
/// boundary to wrap it, as `crate::builtin`'s embedded-lock lift also does.
///
/// # Errors
///
/// A row naming a predicate outside the closed vocabulary, missing a required
/// argument, carrying no address, or declaring an out-of-vocabulary severity is a
/// [`ClauseRowError`] — rejected loud, never a silently dropped clause (see
/// [`default_contract_from_rows`]).
pub fn clause_from_row(row: &ClauseRow) -> Result<contract::Clause, ClauseRowError> {
    let severity = severity_from_label(&row.severity).ok_or_else(|| ClauseRowError::Severity {
        predicate: row.predicate.clone(),
        severity: row.severity.clone(),
    })?;
    let predicate = contract::predicate_from_row(row).ok_or_else(|| ClauseRowError::Predicate {
        predicate: row.predicate.clone(),
    })?;
    // Lifted verbatim, never re-derived: the label the lock committed is the label
    // every finding prints, so the two cannot drift apart on a grammar change.
    let label = row.label.clone().ok_or_else(|| ClauseRowError::Label {
        predicate: row.predicate.clone(),
    })?;
    Ok(contract::Clause {
        label,
        severity,
        predicate,
        guidance: row.guidance.clone(),
        source: row.cite.clone(),
    })
}

/// Parse a severity label into the typed [`contract::Severity`] — the closed
/// `required`/`advisory` vocabulary a bare contract's own clauses declare. An
/// out-of-vocabulary label is `None`. `pub(crate)` so [`crate::dial`]'s
/// read-time severity parse reuses the identical vocabulary parse, never a
/// second copy.
pub(crate) fn severity_from_label(label: &str) -> Option<contract::Severity> {
    match label {
        "required" => Some(contract::Severity::Required),
        "advisory" => Some(contract::Severity::Advisory),
        _ => None,
    }
}

/// The shipped each-grain clause a typed requirement's `kind` facet sources —
/// "every satisfier is kind K" at `required` severity. The mechanism — the predicate shape and its `required`
/// severity — ships fixed with the requirement facet; only `kind` is
/// per-requirement author data, so [`crate::roster::selections`] calls this to
/// synthesize the clause fresh from [`Requirement::kind`] every run
/// rather than storing it on the requirement.
///
/// Synthesized rather than lifted, so its address is derived here from the same grammar
/// emit stamps a written row with — `requirement` is the owner, since the clause is the
/// requirement's demand and not the narrowed kind's.
#[must_use]
pub fn kind_narrowing_clause(requirement: &str, kind: &str) -> contract::Clause {
    let predicate = contract::Predicate::Kind {
        kind: kind.to_string(),
    };
    contract::Clause {
        label: contract::clause_label(
            Some(&contract::requirement_owner(requirement)),
            predicate.key(),
            None,
        ),
        severity: contract::Severity::Required,
        predicate,
        guidance: None,
        source: None,
    }
}

/// A kind's resolved units and their extracted features — the corpus of members every
/// validator and reader consumes. Paired together to hoist the single `resolve_kind_units`
/// call per kind: both gate/explain and collect_directive_members use this data, and
/// computing it once per kind per run avoids a second disk read and parse pass.
pub struct KindUnitsAndFeatures {
    /// The kind's members resolved live off disk.
    pub units: Vec<Unit>,
    /// Each unit's extracted features in parallel order.
    pub features: Vec<extract::Features>,
}

/// The run's whole declaration family, assembled once for every consumer below.
pub struct LockFamily {
    /// The committed lock's rows, joined with every local-locus kind's read-time derived
    /// ones — the corpus's own declarations, the set that decides which kinds and members
    /// exist here.
    pub declarations: drift::Declarations,
    /// The clause rows of the locks this invocation joins, each already addressed under
    /// the layer that carried it ([`qualify_layer_label`]). Kept beside the corpus's own
    /// rows rather than merged into them: these declare nothing about what this harness
    /// *is* — they only add checks over what it already declares — and a consumer that
    /// reads them as the corpus's own would let a layer redefine the corpus.
    pub joined_clauses: Vec<drift::ClauseRow>,
    /// Every lock this invocation joined, as `--layer` spelled it — the locks the rows
    /// above came off, kept beside them because a lock that carried no clause joined the
    /// run just the same and the announcement names the lock, never its contents.
    pub joined_locks: Vec<String>,
    /// Every local member the assembly read, by `<kind>:<id>` address, retained here
    /// beside the rows its read produced: the documents are uncommitted, so no consumer
    /// below can find them again short of re-walking the kind's glob — a second read that
    /// could disagree with this one about which members exist.
    pub local_members: Vec<String>,
    /// This machine's own dial: the severities it re-reads the clauses above at. Read
    /// with the rest of the family for the same one-read reason, and kept apart from both
    /// row sets for the joined clauses': a dial declares nothing about what this harness
    /// is, and adds no check to it either — it only re-weighs the checks the rows above
    /// already carry.
    pub dial: dial::Dial,
    /// Every embedded built-in kind with any lock-declared overlay applied, keyed by bare
    /// kind name. Computed once per invocation and shared with every consumer —
    /// [`kind_units_and_features`], [`admissibility::local_locus_admissibility`],
    /// [`admissibility::governs_collision_diagnostics`] — to hoist the computation cost
    /// per the cost doctrine (engineering.md, "Cost scale is hoisted").
    pub overlaid_builtin_kinds: BTreeMap<String, CustomKind>,
}

/// This kind's effective declaration: the committed lock's own kind-fact row when the
/// lock declares one that qualifies for overlay — matched by bare name, the kind's
/// whole identity — overlaid onto `kind`'s embedded declaration, or `kind` unchanged
/// when it doesn't: the **built-in lock**, the same declaration shape the engine
/// carries compiled-in for an unadopted harness. Three facts overlay from the one
/// matched row: the `governs` locus always (a relocation may declare no other diverging
/// fact at all), `templates` only when the row declares at least one, and `content` only
/// when the row declares a layout — an empty row column defers to `kind`'s own (always
/// empty templates and a `File` body for a built-in), never blanking a nonexistent
/// override.
///
/// # Errors
///
/// Propagates errors from reading the kind-fact row.
pub fn overlay_builtin_kind(
    kind: &CustomKind,
    declarations: &drift::Declarations,
) -> Result<CustomKind, drift::LockRowError> {
    OVERLAY_BUILTIN_KIND_COUNT.with(|c| c.set(c.get() + 1));
    let mut matched = None;
    for row in &declarations.kinds {
        if row.name == kind.name && row_relocates_builtin(row, kind)? {
            matched = Some(row);
            break;
        }
    }
    let Some(row) = matched else {
        return Ok(kind.clone());
    };
    let mut overlaid = kind.clone();
    if let Some((root, glob)) = row.governs_root.clone().zip(row.governs_glob.clone()) {
        overlaid.governs = Some(kind::Governs { root, glob });
    }
    if !row.templates.is_empty() {
        overlaid = overlaid.overlay_templates(&row.templates);
    }
    overlaid = overlaid.overlay_content(row.content.as_ref())?;
    Ok(overlaid)
}

/// Read one layout-content document at `file` into a [`Unit`], off the kind's declared
/// `layout`: the whole file is the body's heading tree, the field sections fill the
/// unit's fields (each slot's verbatim span, so a clause ranges over it as a field). A
/// declared-relationship edge slot is the exception: its entries are addresses, folded
/// onto the unit as a list field the reference graph resolves live off the host's
/// features — like a file member's frontmatter reference list — while `satisfies` reaches
/// the unit off the lock's own family, keyed by member id, not off the document here. The
/// id folds the file's placement under `base` the same way a file-content member's does.
/// A document that does not fit the layout — a section missing, structure
/// no primitive admits — refuses loud through [`crate::layout::LayoutError`], naming the file and
/// heading.
///
/// # Errors
///
/// Returns an error if the document is unreadable or does not fit its declared layout.
fn layout_unit(
    layout: &Layout,
    file: &Path,
    base: &Path,
    edge_fields: &BTreeSet<String>,
) -> miette::Result<Unit> {
    let raw = std::fs::read_to_string(file)
        .map_err(|e| miette::miette!("failed to read layout document {}: {e}", file.display()))?;
    let reading = layout.read(&raw, file, edge_fields)?;
    let id = frontmatter::fold_file_id(base, file)?;
    let mut frontmatter: BTreeMap<String, serde_json::Value> = reading
        .fields
        .into_iter()
        .map(|(slot, span)| (slot, serde_json::Value::String(span)))
        .collect();
    for (slot, entries) in reading.edges {
        if slot == kind::SATISFIES_EDGE_FIELD {
            continue;
        }
        frontmatter.insert(
            slot,
            serde_json::Value::Array(entries.into_iter().map(serde_json::Value::String).collect()),
        );
    }
    Ok(Unit {
        id,
        frontmatter,
        body: raw,
        source_path: file.to_path_buf(),
        satisfies: Vec::new(),
        satisfies_clauses: Vec::new(),
    })
}

/// One discovered source file as a raw [`Unit`], its id folded against `base` — the read
/// both file loci share, so a nested file child and a `governs`-scanned member differ only
/// in the base each composes under. **The one adapter dispatch**: a layout kind's document
/// is read under its declared layout — its field sections fill the unit's fields, a
/// non-fitting document refusing loud; a kind declaring the `json-document` or
/// `toml-document` format reads its whole artifact as one structured document through that
/// grammar's adapter; every other file kind reads through the generic frontmatter adapter.
///
/// # Errors
///
/// Returns an error if the file is unreadable, malformed, or does not fit its declared
/// layout or format.
fn read_file_unit(
    kind: &CustomKind,
    file: &Path,
    base: &Path,
    edge_fields: &BTreeSet<String>,
) -> miette::Result<Unit> {
    match (&kind.content, &kind.format) {
        (kind::Content::Layout(layout), _) => layout_unit(layout, file, base, edge_fields),
        (kind::Content::File | kind::Content::Fields, Some(kind::Format::JsonDocument)) => {
            Ok(json_manifest::DocumentMember::read(kind, file)?.to_unit())
        }
        (kind::Content::File | kind::Content::Fields, Some(kind::Format::TomlDocument)) => {
            Ok(toml_document::read(kind, file)?.to_unit())
        }
        (
            kind::Content::File | kind::Content::Fields,
            Some(kind::Format::YamlFrontmatter) | None,
        ) => {
            let source = frontmatter::Member::from_source_rooted(kind, file, base)?;
            Ok(Unit {
                id: source.id.clone(),
                frontmatter: source.fields.iter().cloned().collect(),
                body: source.body.clone(),
                source_path: source.provenance.source_path.clone(),
                satisfies: Vec::new(),
                satisfies_clauses: Vec::new(),
            })
        }
    }
}

/// Every kind this harness declares, keyed by bare name, using pre-overlaid builtin kinds
/// to avoid re-deriving the overlay. The `overlaid_builtin_kinds` should come from
/// [`LockFamily::overlaid_builtin_kinds`].
///
/// # Errors
///
/// Returns an error if the embedded kind set fails to load or a lock row falls outside a
/// closed vocabulary.
fn declared_kinds_with_overlaid(
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
    declarations: &drift::Declarations,
) -> miette::Result<BTreeMap<String, CustomKind>> {
    let builtin_defs = builtin_kind::definitions();
    let mut kinds = overlaid_builtin_kinds.clone();
    let (custom_rows, _collisions) = partition_kind_rows(declarations, &builtin_defs)?;
    for row in custom_rows {
        kinds.insert(row.name.clone(), CustomKind::from_kind_fact_row(row)?);
    }
    Ok(kinds)
}

/// A manifest `kind`'s registration members as raw [`Unit`]s — every `hooks.<Event>` (or
/// `mcpServers.*`) entry the host manifest carries at the kind's declared collection
/// `address`, read through the JSON manifest adapter ([`json_manifest::Manifest::read_kind`]).
/// A member's id is its collection key, and that key surfaces under the address's key
/// field when it names one (`hooks.<Event>` → `event`), so a clause can range over the
/// lifecycle event a hook keys at. `satisfies` is left empty here — the caller folds it in
/// off the lock, exactly as for a file member.
///
/// # Errors
///
/// Returns an error if the manifest cannot be discovered or read.
fn manifest_units(
    disc: &import::Discovery,
    kind: &CustomKind,
    address: &CollectionAddress,
    cache: &ManifestCache,
) -> miette::Result<Vec<Unit>> {
    let (Some(governs),) = (&kind.governs,) else {
        return Ok(Vec::new());
    };
    let files = import::discover_kind_files(disc, kind, governs, import::LocalOverride::Honored);
    let mut units = Vec::new();
    let collection = address.key_path.collection_key();
    for file in files {
        if let Some((manifest, _)) = cache.get(&file) {
            let source_path = manifest.provenance.source_path.clone();
            for member in &manifest.members {
                if member.collection == collection {
                    units.push(member.to_unit(address, &source_path));
                }
            }
        }
    }
    Ok(units)
}

/// A kind's members, resolved live off disk — the one corpus both `gate` and `explain`
/// range over. Every member is discovered by walking this kind's [`overlay_builtin_kind`]-overlaid
/// `governs` locus, read straight off harness disk so the corpus can never drift from a
/// stale copy; its `satisfies` fill edges come from the run's assembled
/// [`drift::SatisfiesRow`] family, keyed by member id — a committed
/// member's row off the lock, a local member's derived at
/// [`assemble_lock_family`], so this read never re-decides which source it has. Its
/// rationale-carrying `satisfies_clauses` mirrors it: a lock-declared row narrates as a
/// rationale-less [`document::Satisfies`] — the lock row carries
/// no rationale text — so `explain` can never disagree with the gate about which
/// requirements a member fills.
///
/// # Errors
///
/// Returns an error if a source file is unreadable or malformed, or a governed
/// directory cannot be enumerated.
pub fn resolve_kind_units(
    kind: &CustomKind,
    disc: &import::Discovery,
    declarations: &drift::Declarations,
    cache: &ManifestCache,
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
) -> miette::Result<Vec<Unit>> {
    RESOLVE_KIND_UNITS_COUNT.with(|c| c.set(c.get() + 1));
    let overlaid = kind.clone();
    let governs = overlaid.governs.clone();
    let mut edge_fields = kind.edge_field_slots();
    edge_fields.extend(drift::layout_edge_fields(
        &declarations.assembly,
        &kind.name,
    )?);

    let mut units = match (&overlaid.content, &overlaid.collection_address, &governs) {
        (kind::Content::Fields, Some(address), _) => {
            manifest_units(disc, &overlaid, address, cache)?
        }
        (_, _, None) => {
            let kinds = declared_kinds_with_overlaid(overlaid_builtin_kinds, declarations)?;
            let mut child_units = Vec::new();
            for found in import::discover_nested_file(
                disc,
                &overlaid,
                &kinds,
                import::LocalOverride::Honored,
            ) {
                child_units.push(read_file_unit(
                    &overlaid,
                    &found.file,
                    &found.host_unit,
                    &edge_fields,
                )?);
            }
            child_units
        }
        (_, _, Some(governs)) => {
            let base = disc.harness().join(&governs.root);
            let mut file_units = Vec::new();
            for file in
                import::discover_kind_files(disc, kind, governs, import::LocalOverride::Honored)
            {
                file_units.push(read_file_unit(&overlaid, &file, &base, &edge_fields)?);
            }
            file_units
        }
    };

    for unit in &mut units {
        let address = extract::host_address(&kind.name, &unit.id);
        for row in &declarations.satisfies {
            if row.member != address && row.member != unit.id {
                continue;
            }
            if !unit.satisfies.contains(&row.requirement) {
                unit.satisfies.push(row.requirement.clone());
            }
            if !unit
                .satisfies_clauses
                .iter()
                .any(|clause| clause.requirement == row.requirement)
            {
                unit.satisfies_clauses
                    .push(document::Satisfies::new(row.requirement.clone()));
            }
        }
    }

    units.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(units)
}

/// A kind's members' extracted [`extract::Features`] — [`resolve_kind_units`]
/// run through the [`overlay_builtin_kind`]-overlaid kind's own composed extraction,
/// each member's nested-member facts resolved off the run's assembled `nested_members`
/// rows by address ([`builtin_kind::features`]), never by re-parsing its rendered body.
/// Both units and features are returned together to avoid a second resolution pass.
/// The `kind` parameter should be the pre-overlaid form from [`LockFamily::overlaid_builtin_kinds`].
///
/// # Errors
///
/// As [`resolve_kind_units`].
pub fn kind_units_and_features(
    kind: &CustomKind,
    disc: &import::Discovery,
    declarations: &drift::Declarations,
    cache: &ManifestCache,
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
) -> miette::Result<KindUnitsAndFeatures> {
    let units = resolve_kind_units(kind, disc, declarations, cache, overlaid_builtin_kinds)?;
    let features = units
        .iter()
        .map(|unit| builtin_kind::features(kind, unit, &declarations.nested_members))
        .collect();
    Ok(KindUnitsAndFeatures { units, features })
}

/// Resolve every embedded built-in kind's discovered units and features off `harness_root`,
/// keyed by bare kind name — the one loop that both `gate` and `explain` range over.
/// Used by `explain` to collect directive members without a second resolution pass.
/// The `overlaid_builtin_kinds` should come from [`LockFamily::overlaid_builtin_kinds`].
///
/// # Errors
///
/// As [`kind_units_and_features`].
pub fn builtin_units_and_features_by_kind(
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
    disc: &import::Discovery,
    declarations: &drift::Declarations,
    cache: &ManifestCache,
) -> miette::Result<BTreeMap<String, KindUnitsAndFeatures>> {
    let mut by_kind = BTreeMap::new();
    for kind in overlaid_builtin_kinds.values() {
        by_kind.insert(
            kind.name.clone(),
            kind_units_and_features(kind, disc, declarations, cache, overlaid_builtin_kinds)?,
        );
    }
    Ok(by_kind)
}

/// A kind's members' extracted [`extract::Features`] — [`resolve_kind_units`]
/// run through the [`overlay_builtin_kind`]-overlaid kind's own composed extraction,
/// each member's nested-member facts resolved off the run's assembled `nested_members`
/// rows by address ([`builtin_kind::features`]), never by re-parsing its rendered body.
/// The `overlaid_builtin_kinds` should come from [`LockFamily::overlaid_builtin_kinds`].
///
/// # Errors
///
/// As [`resolve_kind_units`].
pub fn kind_features(
    kind: &CustomKind,
    disc: &import::Discovery,
    declarations: &drift::Declarations,
    cache: &ManifestCache,
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
) -> miette::Result<Vec<extract::Features>> {
    kind_units_and_features(kind, disc, declarations, cache, overlaid_builtin_kinds)
        .map(|uaf| uaf.features)
}

/// A local-locus kind's members' declaration rows, derived off their own documents —
/// what the lock would carry for a committed kind, and never does for this one.
///
/// The rows go through the same reader `emit` lowers a committed layout host's source
/// with ([`drift::read_layout_document`]), so a local member's rows are the rows its
/// document declares, not a second interpretation of it.
///
/// # Errors
///
/// Returns an error when a member's document cannot be read or does not fit the kind's
/// declared layout.
fn local_document_rows(
    kind: &CustomKind,
    units: &[Unit],
    declarations: &drift::Declarations,
) -> miette::Result<drift::LayoutDocumentRows> {
    let layout = match (&kind.content, &kind.format) {
        (kind::Content::Layout(layout), _) => layout,
        (
            kind::Content::File | kind::Content::Fields,
            Some(
                kind::Format::YamlFrontmatter
                | kind::Format::JsonDocument
                | kind::Format::TomlDocument,
            )
            | None,
        ) => return Ok(drift::LayoutDocumentRows::default()),
    };
    let mut edge_fields = kind.edge_field_slots();
    edge_fields.extend(drift::layout_edge_fields(
        &declarations.assembly,
        &kind.name,
    )?);

    let mut rows = drift::LayoutDocumentRows::default();
    for unit in units {
        let document = drift::read_layout_document(
            layout,
            &kind.name,
            &unit.id,
            &unit.source_path,
            &edge_fields,
        )?;
        rows.nested.extend(document.nested);
        rows.satisfies.extend(document.satisfies);
    }
    Ok(rows)
}

/// The lock file a `--layer` argument names: the path itself, or the lock inside it when
/// the argument names a directory.
fn layer_lock_path(layer: &Path) -> PathBuf {
    if layer.is_dir() {
        layer.join(crate::LOCK_FILENAME)
    } else {
        layer.to_path_buf()
    }
}

/// The separator between a joined clause's own compiled address and the layer that
/// carried it: `<label>@<layer>`.
///
/// A compiled address is dot-joined ([`contract::clause_label`]), so `@` appears in no
/// label emit can write — which is what makes a joined address unable to collide with a
/// host's, whatever the two locks happen to declare.
const LAYER_QUALIFIER: char = '@';

/// The locks an invocation joined, and the clause rows they carried.
struct JoinedLayers {
    /// Each joined lock, as the invocation spelled it — the same spelling every clause
    /// below is addressed under, and deduped on the same identity, so a lock named twice
    /// is one layer here too.
    locks: Vec<String>,
    /// Every joined clause row, addressed under the layer that carried it.
    clauses: Vec<drift::ClauseRow>,
}

/// The clause rows of every lock `layers` names, each addressed under the layer that
/// carried it, with the locks themselves.
///
/// Only the clause family joins. A layer hardens the gate over *this* corpus; the rows
/// that say what this corpus is — its kinds, its members' fills, its requirements — stay
/// the committed lock's alone, because joining those would let a layer relocate a kind's
/// locus or forge a fill and so *soften* the very gate it claims to tighten. Clauses are
/// the family a join can only add to.
///
/// # Errors
///
/// A named layer that cannot be read, or whose lock is malformed, is an error, not an
/// empty set: an absent lock the invocation named is a layer that did not gate, and a
/// layer silently gating nothing is the one outcome fail-closed forbids. (The *host*
/// lock's absence is legitimate — an unadopted harness has none — which is why that read
/// tolerates it and this one cannot.)
fn read_layer_clauses(layers: &[PathBuf]) -> miette::Result<JoinedLayers> {
    let mut joined = JoinedLayers {
        locks: Vec::new(),
        clauses: Vec::new(),
    };
    let mut seen: BTreeSet<PathBuf> = BTreeSet::new();
    for layer in layers {
        let path = layer_lock_path(layer);
        let identity = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        if !seen.insert(identity) {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|source| {
            miette::miette!(
                "failed to read joined layer lock {}: {source}",
                path.display()
            )
        })?;
        let spelling = layer.display().to_string();
        for row in drift::parse_declarations(&path, &text)?.clauses {
            joined.clauses.push(qualify_layer_label(row, &spelling));
        }
        joined.locks.push(spelling);
    }
    Ok(joined)
}

/// One joined clause row, re-addressed under the layer that carried it.
///
/// A row carrying no address is left as it is: every emitted row is stamped with one, so
/// a row without one is a lock emit did not write, and the contract lift is the one home
/// that refuses it ([`clause_from_row`]) — re-deciding that here would be a
/// second verdict on the same fact.
fn qualify_layer_label(mut row: drift::ClauseRow, layer: &str) -> drift::ClauseRow {
    if let Some(label) = row.label.take() {
        row.label = Some(format!("{label}{LAYER_QUALIFIER}{layer}"));
    }
    row
}

/// This machine's [`dial::Dial`], read off the shipped `dial` kind's own members.
///
/// The kind is embedded rather than lock-declared, so it is the definition that is
/// reached for here rather than the loop above's declared set — a harness gets its dial
/// from adopting temper at all, never from declaring one. The read is the same
/// [`kind_features`] the gate's own dispatcher runs over the kind, which is what keeps
/// the entries this returns and the document the contract judges from ever being two
/// different reads of one file.
///
/// # Errors
///
/// As [`kind_features`] — a malformed dial document fails the run rather than reading as
/// an empty dial, since a dial silently applying nothing is the fail-open case.
fn read_dial(
    disc: &import::Discovery,
    declarations: &drift::Declarations,
    cache: &ManifestCache,
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
) -> miette::Result<dial::Dial> {
    let Some(kind) = builtin_kind::definition(dial::KIND) else {
        return Ok(dial::Dial::default());
    };
    Ok(dial::Dial::from_features(&kind_features(
        &kind,
        disc,
        declarations,
        cache,
        overlaid_builtin_kinds,
    )?))
}

/// The run's whole declaration family: the committed lock, every local-locus kind's
/// read-time derived rows ([`local_document_rows`]), and the clause rows of the locks
/// `layers` names.
///
/// A local kind is committed but its members' documents are not, so the lock carries no
/// row of theirs. Deriving them *here* — once, before any consumer reads — is what lets
/// every consumer below read one family: a clause bound to an embedded kind selects a
/// local host's members exactly as it selects a committed host's, and a local member's
/// fills reach the roster on the same read. A consumer re-deciding which of two sources it
/// reads is the shape this replaces; the derivation runs against the committed family,
/// which is what decides the kinds and loci the members are discovered under.
///
/// A joined lock and this machine's dial are read here for the same reason: one read,
/// before any consumer, so no call site below re-opens a layered input and re-decides
/// what it says.
///
/// # Errors
///
/// As [`resolve_kind_units`], [`local_document_rows`] and [`read_layer_clauses`].
pub fn assemble_lock_family(
    disc: &import::Discovery,
    committed: &drift::Declarations,
    layers: &[PathBuf],
    cache: &ManifestCache,
) -> miette::Result<LockFamily> {
    let mut assembled = committed.clone();
    let mut local_members: BTreeSet<String> = BTreeSet::new();

    let builtin_defs = builtin_kind::definitions();
    let mut overlaid_builtin_kinds = BTreeMap::new();
    for kind in builtin_defs.values() {
        overlaid_builtin_kinds.insert(kind.name.clone(), overlay_builtin_kind(kind, committed)?);
    }

    let all_declared = declared_kinds_with_overlaid(&overlaid_builtin_kinds, committed)?;
    for kind in all_declared.values() {
        if kind.commitment != Some(kind::Commitment::Local) {
            continue;
        }
        let units = resolve_kind_units(kind, disc, committed, cache, &overlaid_builtin_kinds)?;
        let rows = local_document_rows(kind, &units, committed)?;
        local_members.extend(
            units
                .iter()
                .map(|unit| extract::host_address(&kind.name, &unit.id)),
        );
        assembled.nested_members.extend(rows.nested);
        assembled.satisfies.extend(rows.satisfies);
    }
    let joined = read_layer_clauses(layers)?;
    Ok(LockFamily {
        dial: read_dial(disc, committed, cache, &overlaid_builtin_kinds)?,
        declarations: assembled,
        joined_clauses: joined.clauses,
        joined_locks: joined.locks,
        local_members: local_members.into_iter().collect(),
        overlaid_builtin_kinds,
    })
}

/// Assemble the by-kind [`extract::Features`] corpus every set-scope and
/// graph predicate ranges over: every built-in kind's resolved features
/// plus each lock-declared custom kind's features, keyed by kind name. Borrows every
/// slice, so the caller holds the owned feature vecs for the map's lifetime.
pub fn assemble_by_kind<'a>(
    builtin_features: &'a BTreeMap<String, Vec<extract::Features>>,
    custom_kinds: &'a [CustomKindEntry],
    embedded_features: &'a BTreeMap<String, Vec<extract::Features>>,
) -> BTreeMap<&'a str, &'a [extract::Features]> {
    let mut by_kind: BTreeMap<&str, &[extract::Features]> = builtin_features
        .iter()
        .map(|(name, features)| (name.as_str(), features.as_slice()))
        .collect();
    for (kind, features) in custom_kinds {
        by_kind.insert(kind.name.as_str(), features.as_slice());
    }
    for (kind, features) in embedded_features {
        by_kind.insert(kind.as_str(), features.as_slice());
    }
    by_kind
}

/// Determine whether a kind-fact row qualifies to overlay a built-in kind's definition.
fn row_relocates_builtin(
    row: &drift::KindFactRow,
    builtin: &CustomKind,
) -> Result<bool, drift::LockRowError> {
    let declared = CustomKind::from_kind_fact_row(row)?;
    Ok(
        (declared.format.is_none() || declared.format == builtin.format)
            && (declared.unit_shape.is_none() || declared.unit_shape == builtin.unit_shape)
            && (declared.registration.is_empty() || declared.registration == builtin.registration),
    )
}

/// Partition kind-fact rows into custom kinds (not relocating built-ins) and
/// collision kinds (relocating built-ins but mismatched).
pub fn partition_kind_rows<'a>(
    declarations: &'a drift::Declarations,
    builtin_defs: &BTreeMap<String, CustomKind>,
) -> Result<(Vec<&'a drift::KindFactRow>, Vec<&'a drift::KindFactRow>), drift::LockRowError> {
    let mut custom = Vec::new();
    let mut collisions = Vec::new();
    for row in &declarations.kinds {
        match builtin_defs.get(&row.name) {
            None => custom.push(row),
            Some(builtin) if !row_relocates_builtin(row, builtin)? => collisions.push(row),
            Some(_) => {}
        }
    }
    Ok((custom, collisions))
}

/// The embedded-kind corpus: every kind declared at the embedded locus keyed to its
/// members' [`Features`](extract::Features), so [`assemble_by_kind`] can fold it into the
/// one `by_kind` map every graph predicate ranges over. An embedded kind is named where a
/// host declares it — a `templates` column entry, or a layout member collection's
/// `member_kind` — and carries no kind-fact row, so this is the sole seam it enters the
/// corpus through. Its members are the run's assembled `nested_member` rows of that kind
/// — a committed host's off the lock, a local host's derived at [`assemble_lock_family`],
/// so a clause over it selects a local host's members and a committed host's alike — each
/// lifted to a member whose id is the row's key and whose fields are its leaves, so an
/// edge resolves against it by identity ([`embedded_member_features`]). A declared kind
/// with no rows keys to an empty slice — modeled, so an edge targeting it is admissible
/// and a dangling entry is a route finding, not an admissibility one; a kind no host
/// declares is absent, so an edge targeting it stays an admissibility finding. Depth is
/// one layer: a `nested_member` row's own sibling collections are the leaf grain the read
/// family addresses, not a second embedded kind's member set.
pub fn embedded_features_by_kind(
    declarations: &drift::Declarations,
) -> BTreeMap<String, Vec<extract::Features>> {
    let mut by_kind: BTreeMap<String, Vec<extract::Features>> =
        crate::admissibility::declared_embedded_kinds(declarations)
            .into_iter()
            .map(|kind| (kind, Vec::new()))
            .collect();
    // Each declared embedded kind's members are its `nested_member` rows. A row whose
    // kind no host declares is an orphan rejected at admissibility
    // ([`nested_member_admissibility`]), so this `get_mut` now backstops that already-loud
    // unreachable state rather than swallowing a live one.
    let edge_fields = edge_fields_by_kind(declarations);
    let no_edges = BTreeSet::new();
    for row in &declarations.nested_members {
        if let Some(features) = by_kind.get_mut(&row.kind) {
            features.push(embedded_member_features(
                row,
                edge_fields.get(&row.kind).unwrap_or(&no_edges),
            ));
        }
    }
    by_kind
}

/// The edge fields each kind declares, off the lock's `assembly` `edge` facts — the
/// declared set a `format-places-edges` clause measures a value's own
/// [`placed_edges`](drift::NestedMemberRow::placed_edges) against
/// ([`embedded_member_features`]). A malformed edge fact is
/// [`edges_from_declarations`]'s own load error, raised before any check runs, so this
/// fold reads the well-formed rows rather than raise the identical fault twice.
pub fn edge_fields_by_kind(
    declarations: &drift::Declarations,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut by_kind: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for fact in &declarations.assembly {
        if fact.fact != "edge" {
            continue;
        }
        if let (Some(from), Some(field)) = (fact.from.clone(), fact.field.clone()) {
            by_kind.entry(from).or_default().insert(field);
        }
    }
    by_kind
}

/// Lift one [`NestedMemberRow`](drift::NestedMemberRow) into the
/// [`Features`](extract::Features) an edge resolves against: the row's key is the member
/// id an edge matches by identity, and its leaves surface as string fields so a clause
/// (or a deeper edge) can range over them exactly as a file member's frontmatter. The
/// body-derived features are empty — an embedded member has no document of its own; it is
/// read off its host's declared surface.
///
/// `edge_fields` is what the member's kind declares ([`edge_fields_by_kind`]); pairing the
/// ones this row actually fills with its own `placed_edges` is what makes a
/// `format-places-edges` clause decidable without the engine ever seeing the format that
/// rendered the value. An unfilled field is no edge, so it is no obligation: ranging over
/// the kind's whole declared set would read an absent edge as one the format dropped.
pub fn embedded_member_features(
    row: &drift::NestedMemberRow,
    edge_fields: &BTreeSet<String>,
) -> extract::Features {
    let fields = row
        .leaves
        .iter()
        .map(|(name, text)| (name.clone(), serde_json::Value::String(text.clone())))
        .collect();
    extract::Features {
        id: row.key.clone(),
        fields,
        body_lines: 0,
        // The rendered span `emit` captured off the value's own projection, lifted from
        // the row so an `extent` clause bound to the embedded kind budgets real data. A
        // `None` span is a value no format rendered (a layout host read off source): it has
        // no projection to measure, so its `extent` stays undecidable rather than reading a
        // zero as a pass.
        rendered_lines: row.rendered_lines,
        rendered_chars: row.rendered_chars,
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: None,
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: Vec::new(),
        // `None` ⇒ no format rendered the value (a layout host's document is source, not
        // projection), which is not a format to indict. `Some` over an empty map ⇒ a
        // format ran and the value carries no edge to place. The engine cannot tell the
        // two apart once they collapse into one empty map, so they are kept apart here.
        edge_placements: row.placed_edges.as_ref().map(|placed| {
            edge_fields
                .iter()
                .filter(|field| row.leaves.get(*field).is_some_and(|text| !text.is_empty()))
                .map(|field| (field.clone(), placed.contains(field)))
                .collect()
        }),
    }
}

/// Construct directive members from pre-computed resolved units and features, avoiding
/// a second `resolve_kind_units` pass. Called by [`gate`] and [`explain`] to avoid
/// re-reading every member off disk after the units and features have already been
/// resolved for validation.
pub fn directive_members_from_resolved(
    builtin_units_and_features: &BTreeMap<String, KindUnitsAndFeatures>,
    custom_units_and_features: &[(CustomKind, KindUnitsAndFeatures)],
) -> Vec<graph::DirectiveMember> {
    let mut members = Vec::new();
    for (kind_name, uaf) in builtin_units_and_features {
        for (unit, features) in uaf.units.iter().zip(&uaf.features) {
            members.push(graph::DirectiveMember {
                kind: kind_name.clone(),
                id: features.id.clone(),
                source_path: unit.source_path.clone(),
                directives: features.directives.clone(),
            });
        }
    }
    for (custom_kind, uaf) in custom_units_and_features {
        for (unit, features) in uaf.units.iter().zip(&uaf.features) {
            members.push(graph::DirectiveMember {
                kind: custom_kind.name.clone(),
                id: features.id.clone(),
                source_path: unit.source_path.clone(),
                directives: features.directives.clone(),
            });
        }
    }
    members
}

/// Every represented manifest file on disk as a raw [`Vec<String>`] of paths,
/// walked once per run to avoid re-reading per-kind.
pub fn repo_file_set(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(root).min_depth(1).sort_by_file_name() {
        let Ok(entry) = entry else { continue };
        if entry.file_type().is_file()
            && let Ok(rel) = entry.path().strip_prefix(root)
        {
            files.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
    files
}

/// Build a shared manifest cache for a single gate/explain invocation, grouping manifest
/// kinds by their manifest file path and reading each file once with all governing kinds'
/// addresses. This hoisting ensures manifest files are read exactly once per run, never
/// once per governing kind (GATE-MANIFEST-SHARED-READ-HOIST).
pub fn build_manifest_cache(
    disc: &import::Discovery,
    declarations: &drift::Declarations,
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
) -> miette::Result<ManifestCache> {
    let mut cache: ManifestCache = BTreeMap::new();
    let kinds = declared_kinds_with_overlaid(overlaid_builtin_kinds, declarations)?;

    // Group manifest kinds by their manifest file path.
    let mut by_manifest: BTreeMap<PathBuf, Vec<&CollectionAddress>> = BTreeMap::new();
    for kind in kinds.values() {
        if let Some(address) = &kind.collection_address
            && let Some(governs) = &kind.governs
        {
            // Discover the actual files this kind governs.
            let files =
                import::discover_kind_files(disc, kind, governs, import::LocalOverride::Honored);
            for file in files {
                by_manifest.entry(file).or_default().push(address);
            }
        }
    }

    // Read each manifest file once with all addresses for that file.
    for (file, addresses) in by_manifest {
        let manifest = json_manifest::Manifest::read(&file, &addresses)?;
        cache.insert(file, (manifest, BTreeMap::new()));
    }

    Ok(cache)
}

/// A built-in `kind`'s effective [`Contract`]: its lock-declared clause rows are its
/// whole contract when the lock names any, lifted through the same reject-loud path a
/// custom kind's rows take ([`default_contract_from_rows`]); with no rows the
/// kind falls back to the embedded default (from [`crate::builtin::contract`]).
/// Rows-or-default — never a severity-flip layer over the embedded default: a spread's
/// appended clause gates, an array-surgery removal holds, and an out-of-vocabulary row
/// rejects loud rather than sitting inert.
///
/// # Errors
///
/// Propagates the [`ClauseRowError`] the row lift raises for a row the closed
/// vocabulary cannot admit, or the missing-embedded-contract error if a rowless kind
/// ships none.
pub fn builtin_contract(clauses: &[ClauseRow], kind: &str) -> miette::Result<Contract> {
    if clauses.iter().any(|row| row.kind.as_deref() == Some(kind)) {
        Ok(default_contract_from_rows(clauses, kind)?)
    } else {
        crate::builtin::contract(kind).ok_or_else(|| {
            miette::miette!("built-in kind `{kind}` ships no embedded default contract")
        })
    }
}

/// The enforcement mode `declarations` declare, for a caller that has already read them —
/// a harness needing the mode to decide whether a dialed softening binds and must
/// never reach a second verdict on the posture from a second read.
///
/// # Errors
///
/// Returns a [`drift::LockRowError::Vocabulary`] when the `mode` fact carries an
/// unrecognized value outside the closed `{note, warn, block}` vocabulary.
pub fn mode_from_declarations(
    declarations: &drift::Declarations,
) -> miette::Result<EnforcementMode> {
    let Some(value) = declarations
        .assembly
        .iter()
        .find(|row| row.fact == "mode")
        .and_then(|row| row.value.as_deref())
    else {
        return Ok(EnforcementMode::default());
    };
    match value {
        "note" => Ok(EnforcementMode::Note),
        "warn" => Ok(EnforcementMode::Warn),
        "block" => Ok(EnforcementMode::Block),
        other => Err(drift::LockRowError::Vocabulary {
            family: "assembly".to_string(),
            column: "mode".to_string(),
            value: other.to_string(),
        }
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::contract::{Clause, Predicate, Severity};
    use std::collections::BTreeMap;

    /// A [`ClauseRow`] at `severity`, every other column defaulted — the base the
    /// reject-loud cases struct-update, overriding only `kind`/`predicate` and any
    /// argument column the case exercises.
    fn clause_row(severity: &str) -> ClauseRow {
        ClauseRow {
            unit: None,
            label: Some("fixture.clause".to_string()),
            kind: None,
            predicate: String::new(),
            field: None,
            severity: severity.to_string(),
            guidance: None,
            cite: None,
            count: None,
            target: None,
            degree: None,
            gate: None,
            value_type: None,
            shape: None,
            bound: None,
            charset: None,
            keys: None,
            values: None,
            range: None,
            section: None,
            sections: None,
            guard_predicate: None,
            body: None,
        }
    }

    #[test]
    fn default_contract_from_rows_builds_a_custom_kinds_whole_default_contract() {
        // A custom kind has no built-in default to override — its committed rows are its
        // whole default contract, so a matching row contributes a brand new clause rather
        // than only flipping an existing one's severity.
        let rows = vec![
            ClauseRow {
                unit: Some("lines".to_string()),
                label: Some("spec.extent".to_string()),
                kind: Some("spec".to_string()),
                predicate: "extent".to_string(),
                field: None,
                severity: "advisory".to_string(),
                guidance: None,
                cite: None,
                count: None,
                target: None,
                degree: None,
                gate: None,
                value_type: None,
                shape: None,
                bound: Some(crate::drift::BoundRow {
                    min: None,
                    max: Some(150),
                }),
                charset: None,
                keys: None,
                values: None,
                range: None,
                section: None,
                sections: None,
                guard_predicate: None,
                body: None,
            },
            ClauseRow {
                unit: Some("lines".to_string()),
                label: Some("rule.extent".to_string()),
                kind: Some("rule".to_string()),
                predicate: "extent".to_string(),
                field: None,
                severity: "required".to_string(),
                guidance: None,
                cite: None,
                count: None,
                target: None,
                degree: None,
                gate: None,
                value_type: None,
                shape: None,
                bound: Some(crate::drift::BoundRow {
                    min: None,
                    max: Some(10),
                }),
                charset: None,
                keys: None,
                values: None,
                range: None,
                section: None,
                sections: None,
                guard_predicate: None,
                body: None,
            },
        ];

        let contract = default_contract_from_rows(&rows, "spec").unwrap();
        assert_eq!(contract.name, "spec");
        assert_eq!(
            contract.clauses,
            vec![Clause {
                label: "spec.extent".to_string(),
                severity: Severity::Advisory,
                predicate: Predicate::Extent {
                    unit: contract::ExtentUnit::Lines,
                    max: 150,
                    whole: false,
                },
                guidance: None,
                source: None,
            }]
        );
    }

    #[test]
    fn default_contract_from_rows_rejects_a_row_it_cannot_lift() {
        // The lock is tool-written, never hand-patched: a row the closed vocabulary
        // cannot admit is corruption rejected loud, never a clause silently dropped.
        // An unknown predicate names nothing in the vocabulary.
        let unknown = vec![ClauseRow {
            unit: None,
            label: None,
            kind: Some("spec".to_string()),
            predicate: "not_a_predicate".to_string(),
            ..clause_row("advisory")
        }];
        assert!(matches!(
            default_contract_from_rows(&unknown, "spec"),
            Err(ClauseRowError::Predicate { predicate }) if predicate == "not_a_predicate"
        ));

        // A known predicate missing its required argument (`section_contains` with no
        // `section` column) cannot be built either — the same loud rejection.
        let missing_arg = vec![ClauseRow {
            unit: None,
            label: None,
            kind: Some("spec".to_string()),
            predicate: "section_contains".to_string(),
            ..clause_row("advisory")
        }];
        assert!(matches!(
            default_contract_from_rows(&missing_arg, "spec"),
            Err(ClauseRowError::Predicate { .. })
        ));

        // A severity outside the closed `required`/`advisory` vocabulary is rejected
        // on the severity channel.
        let bad_severity = vec![ClauseRow {
            unit: Some("lines".to_string()),
            label: None,
            kind: Some("spec".to_string()),
            predicate: "extent".to_string(),
            bound: Some(crate::drift::BoundRow {
                min: None,
                max: Some(150),
            }),
            ..clause_row("blocking")
        }];
        assert!(matches!(
            default_contract_from_rows(&bad_severity, "spec"),
            Err(ClauseRowError::Severity { severity, .. }) if severity == "blocking"
        ));
    }

    /// The directive-backing set reads **raw disk**, never ignore-filtered: whether an
    /// `@import` target is backed is a fact about the filesystem the harness loads
    /// regardless of `.gitignore`, and the safe direction fixes it — an extra backing file only *suppresses* a
    /// finding, while pruning one could *forge* an unbacked finding on a target that
    /// exists. This is the counterpart to discovery, which *does* prune — two sets,
    /// two rules, never merged.
    #[test]
    fn repo_file_set_stays_raw_disk_including_gitignored_targets() {
        use crate::test_support::tmpdir;
        use std::fs;

        let root = tmpdir("repo-file-set");
        let dep = root.join("node_modules").join("dep");
        fs::create_dir_all(&dep).unwrap();
        fs::write(root.join(".gitignore"), "node_modules/\n").unwrap();
        fs::write(root.join("CLAUDE.md"), "# root\n").unwrap();
        // An `@import` target the harness loads even though `.gitignore` excludes it.
        fs::write(dep.join("SHARED.md"), "shared\n").unwrap();

        let files = repo_file_set(&root);
        assert!(
            files.iter().any(|f| f == "node_modules/dep/SHARED.md"),
            "the gitignored backing target must still be seen (raw disk): {files:?}"
        );
        assert!(files.iter().any(|f| f == "CLAUDE.md"));
    }

    /// One `nested_member` row of a `citation` kind declaring the edges `edges` names,
    /// filling the leaves `leaves` names, whose format placed `placed` (`None` ⇒ no
    /// format rendered the value).
    fn citation_row(
        edges: &[&str],
        leaves: &[&str],
        placed: Option<Vec<String>>,
    ) -> drift::Declarations {
        drift::Declarations {
            assembly: edges
                .iter()
                .map(|field| drift::AssemblyFactRow {
                    fact: "edge".to_string(),
                    from: Some("citation".to_string()),
                    field: Some((*field).to_string()),
                    to: Some(vec!["rule".to_string()]),
                    value: None,
                })
                .collect(),
            nested_members: vec![drift::NestedMemberRow {
                host: "memory:CLAUDE".to_string(),
                kind: "citation".to_string(),
                key: "the-standard".to_string(),
                leaves: leaves
                    .iter()
                    .map(|leaf| ((*leaf).to_string(), "rule:rust".to_string()))
                    .collect(),
                collections: Vec::new(),
                placed_edges: placed,
                rendered_lines: None,
                rendered_chars: None,
            }],
            ..drift::Declarations::default()
        }
    }

    /// The placement feature of the row `declarations` carries, against its kind's
    /// declared edges — the join the lift performs.
    fn placement_feature(declarations: &drift::Declarations) -> Option<BTreeMap<String, bool>> {
        let edges = edge_fields_by_kind(declarations);
        embedded_member_features(
            &declarations.nested_members[0],
            edges.get("citation").unwrap(),
        )
        .edge_placements
    }

    /// A member's placement feature is the join of two lock families: the edges the
    /// `assembly` family says its kind declares, against the `placed_edges` its own row
    /// says the format rendered. Neither alone decides a `format-places-edges` clause.
    #[test]
    fn an_embedded_members_placement_feature_joins_declared_edges_against_the_placed_set() {
        let declarations = citation_row(
            &["source", "supersedes"],
            &["source", "supersedes"],
            Some(vec!["source".to_string()]),
        );
        assert_eq!(
            placement_feature(&declarations),
            Some(BTreeMap::from([
                ("source".to_string(), true),
                ("supersedes".to_string(), false),
            ])),
            "the edge the format never selected must read as unplaced",
        );
    }

    /// An edge field the value never filled is no edge, so it is no placement obligation:
    /// ranging over the kind's whole declared set would read the absent field as one the
    /// format dropped. An empty leaf is unfilled the same way an absent one is.
    #[test]
    fn an_unfilled_edge_field_carries_no_placement_obligation() {
        let unfilled = citation_row(
            &["source", "supersedes"],
            &["source"],
            Some(vec!["source".to_string()]),
        );
        assert_eq!(
            placement_feature(&unfilled),
            Some(BTreeMap::from([("source".to_string(), true)])),
            "the unfilled `supersedes` is no edge, so the format omitted nothing",
        );

        let mut empty_leaf = citation_row(&["source"], &["source"], Some(Vec::new()));
        empty_leaf.nested_members[0]
            .leaves
            .insert("source".to_string(), String::new());
        assert_eq!(placement_feature(&empty_leaf), Some(BTreeMap::new()));
    }

    /// The two ways a member offers nothing to indict stay apart: no format rendered the
    /// value at all (a layout host's document is source), versus a format that ran over a
    /// value carrying no edge. Both hold at the gate, but only this lift can tell them
    /// apart — an empty map standing for both is what left the clause undecidable.
    #[test]
    fn a_value_no_format_rendered_is_distinct_from_a_format_with_nothing_to_place() {
        assert_eq!(
            placement_feature(&citation_row(&["source"], &["source"], None)),
            None
        );
        assert_eq!(
            placement_feature(&citation_row(&["source"], &[], Some(Vec::new()))),
            Some(BTreeMap::new()),
        );

        // The row whose format ran over a filled edge and placed nothing does carry the
        // fact — an unplaced edge, which is the finding the clause exists to make.
        assert_eq!(
            placement_feature(&citation_row(&["source"], &["source"], Some(Vec::new()))),
            Some(BTreeMap::from([("source".to_string(), false)])),
        );
    }
}
