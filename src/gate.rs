//! The gate verb: linting a harness against the active contract.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::admissibility;
use crate::builtin_kind;
use crate::check::{self, Severity};
use crate::compose;
use crate::contract::Contract;
use crate::coverage;
use crate::coverage_note;
use crate::dial;
use crate::drift;
use crate::engine;
use crate::extract;
use crate::graph;
use crate::import;
use crate::install;
use crate::kind::CustomKind;
use crate::roster;

/// Dispatch a single kind through the shared "two-greens" contract validation:
/// admissibility (contract checks the definition) then conformance (artifacts
/// check the contract). Returns the (possibly dial-modified) contract and the
/// collected diagnostics.
fn two_greens_dispatch(
    mut contract: Contract,
    locus: &engine::Locus,
    features: &[extract::Features],
    dial: &dial::Dial,
    mode: compose::EnforcementMode,
    skip_dial: bool,
    dialed: &mut BTreeSet<String>,
) -> (Contract, Vec<check::Diagnostic>) {
    let mut diagnostics = Vec::new();

    if !skip_dial {
        dialed.extend(dial.apply(mode, &mut contract.clauses));
    }

    diagnostics.extend(engine::admissibility(&contract, locus));
    diagnostics.extend(engine::validate(&contract, features));

    (contract, diagnostics)
}

/// Produce the merged diagnostic set for a surface `workspace` against the active
/// by-kind contracts, with the [`check::Announcement`] of the inputs that judged it —
/// the shared gate behind both `check` and the session-start
/// reporter. `harness_root` is the
/// directory a member's source path and a script verifier's path resolve against (the
/// CWD for a two-step `check`, the harness path for the one-shot gate). `layers` names
/// the policy locks this invocation joins, top of the layer stack.
pub fn gate(
    workspace: &Path,
    harness_root: &Path,
    layers: &[std::path::PathBuf],
) -> miette::Result<(Vec<check::Diagnostic>, check::Announcement)> {
    // The assembly's own declared facts — requirements and edges — ride the lock's
    // declaration rows: `emit` is the sole
    // producer, this is the gate's one read of it.
    // Never gated on a lock's presence — an unadopted harness's lock declares
    // nothing, so this tier is a no-op over it rather than skipped (never a
    // half-adopted state).
    let committed = drift::read_declarations(workspace)?;
    // Parse the lock document once for reuse across source-dependency checks, hoisting
    // the read/parse operation per the cost doctrine (engineering.md, "Cost scale is hoisted").
    let lock_doc = drift::read_lock_document(workspace)?;
    // One ignore-honoring walk per flavor, shared across every kind and nested host this
    // gate discovers ([`import::Discovery`]) — the session-open `check` walks the
    // consumer's whole tree, so a per-kind re-walk is the tick's dominant cost. This one
    // cache is threaded through every discovery call below; a run walks each consulted
    // flavor exactly once, pinned at run granularity by [`import::walk_count`].
    let discovery = import::Discovery::new(harness_root);
    // The dial's softening is inert under `block`, so the mode is resolved before any
    // contract is, and off the same declarations the family assembles from — a run whose
    // gate and whose guard disagreed about the harness's own posture would be two gates.
    let mode = compose::mode_from_declarations(&committed)?;
    // Empty cache for early assembly; will build a proper cache after lock_family returns.
    let empty_cache: compose::ManifestCache = BTreeMap::new();
    let compose::LockFamily {
        declarations,
        joined_clauses,
        joined_locks,
        local_members,
        dial,
        overlaid_builtin_kinds,
    } = compose::assemble_lock_family(&discovery, &committed, layers, &empty_cache)?;
    // Every address the dial reached, accumulated across the contracts and selections
    // below: an entry that reached none is the one thing a dial can be wrong about that
    // its own schema cannot catch.
    let mut dialed: BTreeSet<String> = BTreeSet::new();
    let assembly_requirements: BTreeMap<String, compose::Requirement> = committed
        .requirements
        .iter()
        .map(|row| Ok((row.name.clone(), drift::requirement_from_row(row)?)))
        .collect::<Result<_, compose::ClauseRowError>>()?;
    let assembly_edges = drift::edges_from_declarations(&declarations)?;
    // The lifted reference edges the graph predicates and read verbs fold in alongside the
    // declared-field arcs: authored mentions (route-resolved at check — `route_mentions`
    // owns a deferred mention's dangling verdict) and layout prose imports (path-resolved
    // at emit), each lifted off the lock's own declaration family.
    let mut mention_edges = drift::mention_edges_from_declarations(&declarations);
    mention_edges.extend(drift::import_edges_from_doc(&lock_doc)?);

    // The generic two-greens over EVERY embedded built-in kind, keyed by its bare row
    // label: each kind's members — resolved by
    // [`kind_features`] straight off harness disk, shared with `explain`
    // (READ-EDGE-UNIFY) so a read cannot disagree with the gate about which members
    // exist — are dispatched to its default contract and validated, so a discovered `CLAUDE.md`
    // memory member fires its `memory` clauses exactly as a skill/rule does — no
    // longer silently skipped by a hardcoded skill/rule pair. The resolved features
    // feed straight into `builtin_features` (MEMORY-ENTERS-REQUIREMENT-CORPUS): every
    // built-in's satisfies edges reach the roster/graph/coverage tiers below, not only
    // skill/rule's.
    let mut diagnostics = Vec::new();
    // Per-kind checked-member counts, keyed by bare row label — carried out of
    // the dispatch loop for the advisory coverage note below (WEDGE-COVERAGE-NOTE),
    // so "checked N members" is stated rather than left as bare silence.
    let mut member_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut builtin_features: BTreeMap<String, Vec<extract::Features>> = BTreeMap::new();
    // Each kind's resolved contract, kept past its dispatch loop: the set clauses in it
    // bind to the kind's *whole* by-kind selection, which only exists once every
    // dispatcher has run and `by_kind` is assembled below.
    let mut contracts: BTreeMap<String, Contract> = BTreeMap::new();

    // Build a shared manifest cache: one read per manifest file, shared across all
    // manifest kinds that govern it (GATE-MANIFEST-SHARED-READ-HOIST).
    let manifest_cache =
        compose::build_manifest_cache(&discovery, &declarations, &overlaid_builtin_kinds)?;

    // Every clause's address is unique across the lock, decided before a single contract
    // is lifted: a clause no finding can name unambiguously cannot be judged usefully.
    diagnostics.extend(admissibility::clause_collision_diagnostics(
        &declarations,
        &joined_clauses,
    ));
    let builtin_defs = builtin_kind::definitions();
    let mut builtin_units_and_features: BTreeMap<String, compose::KindUnitsAndFeatures> =
        BTreeMap::new();
    for (kind_name, kind) in &overlaid_builtin_kinds {
        // The contract is the lock's declared `clauses` for the kind when it names any,
        // else the embedded default (`builtin_contract`). The invocation's joined clauses
        // are appended *after* that fallback decides, never folded into the rows it reads:
        // a layer's row is not this harness declaring one, so it must never be what tips a
        // built-in off its embedded default and onto a contract of the layer's alone.
        let contract = compose::with_joined_clauses(
            compose::builtin_contract(&declarations.clauses, kind_name)?,
            &joined_clauses,
            kind_name,
        )?;
        // Every kind's contract is dialable but the dial's own: its clauses are the
        // envelope the dial document is checked against, so a machine that could soften
        // them could spell its way out of the shape that bounds it. `dial::refusals`
        // reports the entry that tried rather than leaving it silently inert.
        let skip_dial = kind_name == dial::KIND;

        let uaf = compose::kind_units_and_features(
            kind,
            &discovery,
            &declarations,
            &manifest_cache,
            &overlaid_builtin_kinds,
        )?;
        let features = &uaf.features;

        let (contract, dispatch_diags) = two_greens_dispatch(
            contract,
            &engine::Locus::Document,
            features,
            &dial,
            mode,
            skip_dial,
            &mut dialed,
        );
        diagnostics.extend(dispatch_diags);
        member_counts.insert(kind_name.clone(), features.len());
        contracts.insert(kind_name.clone(), contract);
        builtin_features.insert(kind_name.clone(), features.clone());
        builtin_units_and_features.insert(kind_name.clone(), uaf);
    }

    // Every lock-declared kind that is not one of the embedded built-ins:
    // a
    // built-in's own row is only the overlay `compose::overlay_builtin_kind` already
    // consumes, never a second kind definition. A custom kind carries no embedded
    // default — its whole default contract is the committed lock's own clause rows
    // naming it ([`compose::default_contract_from_rows`]) — but is otherwise
    // dispatched through the identical two-greens the built-in loop above runs.
    let mut custom_kinds: Vec<compose::CustomKindEntry> = Vec::new();
    let mut custom_units_and_features: Vec<(CustomKind, compose::KindUnitsAndFeatures)> =
        Vec::new();
    let (custom_rows, collisions) = compose::partition_kind_rows(&declarations, &builtin_defs)?;
    // The one site among the three dispatchers that can surface a diagnostic.
    diagnostics.extend(
        collisions
            .iter()
            .map(|row| admissibility::kind_collision_diagnostic(row)),
    );
    // Two distinct kinds resolving to one `governs` locus would double-route every
    // matching document into both member sets — a document's kind is its position
    // alone, never its content — so a shared locus refuses loud here.
    diagnostics.extend(admissibility::governs_collision_diagnostics(
        &overlaid_builtin_kinds,
        &custom_rows,
        &declarations,
    )?);
    // A declared commitment class the locus cannot carry is decided here, beside the
    // locus's other coherence check, before any member is read under a kind whose own
    // declaration does not hold together.
    diagnostics.extend(admissibility::local_locus_admissibility(
        &overlaid_builtin_kinds,
        &custom_rows,
        &declarations,
    )?);
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row)?;
        let contract = compose::with_joined_clauses(
            compose::default_contract_from_rows(&declarations.clauses, &row.name)?,
            &joined_clauses,
            &row.name,
        )?;
        let uaf = compose::kind_units_and_features(
            &custom_kind,
            &discovery,
            &declarations,
            &manifest_cache,
            &overlaid_builtin_kinds,
        )?;
        let features = &uaf.features;

        let (contract, dispatch_diags) = two_greens_dispatch(
            contract,
            &engine::Locus::Document,
            features,
            &dial,
            mode,
            false,
            &mut dialed,
        );
        diagnostics.extend(dispatch_diags);
        member_counts.insert(row.name.clone(), features.len());
        contracts.insert(row.name.clone(), contract);
        custom_kinds.push((custom_kind.clone(), features.clone()));
        custom_units_and_features.push((custom_kind, uaf));
    }

    // The directive backing-set file-set: every file under the harness root, over-collected so an extra
    // file can only suppress a finding, never forge one. Computed once on the FLOOR
    // and read by the directive classing below.
    let repo_files = compose::repo_file_set(harness_root);

    // Directive-target classing on the FLOOR tier: an unbacked `@import` is a **pure fact** about the importing member —
    // the silent-context-loss failure class made author-time — so it surfaces with zero
    // config. Over the built-in kinds' members (empty custom slice), the unbacked findings
    // extend as a **non-gating advisory**: the fact is stated, the run never fails on it
    // alone. The graph-scope escalation stays assembly-gated (WEDGE ruling 2026-07-03: an
    // unbacked import is a pure fact, not a graph-scope opinion like reachability).
    diagnostics.extend(
        graph::classify_directives(
            &compose::directive_members_from_resolved(
                &builtin_units_and_features,
                &custom_units_and_features,
            ),
            &repo_files,
        )
        .findings
        .into_iter()
        .map(|mut finding| {
            finding.severity = Severity::Warn;
            finding
        }),
    );

    // The harness-contract tier: the set predicates over the parsed roster, each
    // quantified over a requirement's opt-in selection.
    // Runs unconditionally — the lock is the sole source of assembly facts now, so an
    // unadopted harness's empty declarations make this tier a no-op rather than a
    // skip.
    let edges = assembly_edges.clone();

    // Cross-family coherence: a `nested_member` row of a kind no host templates is an
    // orphan the by-kind corpus below would unmodel while the host-address read still
    // carries it. Reject it at admissibility, before the corpus is trusted.
    diagnostics.extend(admissibility::nested_member_admissibility(&declarations));

    // The by-kind corpus every set-scope and graph predicate ranges over,
    // assembled through the same helper the read arm uses.
    let embedded_features = compose::embedded_features_by_kind(&declarations);

    // The third dispatcher: an embedded kind's members through the identical two greens
    // the two at-locus loops above run, so a clause bound to an embedded kind is judged
    // rather than silently no-opped. Ordered here because the embedded corpus is what a
    // host's `templates` column yields, which is only assembled above. Like a custom
    // kind, an embedded kind carries no embedded default — its whole contract is the
    // committed lock's own clause rows naming it. Its member counts stay out of the
    // coverage note's summary: that map is keyed by kind-fact row label, which an
    // embedded kind has none of, and an embedded member's host file is already counted
    // under its own kind.
    for (kind, features) in &embedded_features {
        let contract = compose::with_joined_clauses(
            compose::default_contract_from_rows(&declarations.clauses, kind)?,
            &joined_clauses,
            kind,
        )?;

        let (contract, dispatch_diags) = two_greens_dispatch(
            contract,
            &engine::Locus::Embedded(kind.clone()),
            features,
            &dial,
            mode,
            false,
            &mut dialed,
        );
        diagnostics.extend(dispatch_diags);
        contracts.insert(kind.clone(), contract);
    }

    // Every kind a joined clause names that this corpus declares none of. Nothing here
    // selects such a clause, so it judges nothing — but a layer must fail closed whether
    // or not the host happens to give its clauses something to range over, so the rows
    // still face the admissibility their kind's own dispatcher would have run.
    diagnostics.extend(admissibility::joined_kind_admissibility(
        &joined_clauses,
        &contracts,
    )?);

    let by_kind = compose::assemble_by_kind(&builtin_features, &custom_kinds, &embedded_features);

    // A bare `satisfies` label an older engine wrote qualifies against this corpus, but a
    // name two kinds share is a malformed lock refused loud rather than cross-attributed.
    diagnostics.extend(admissibility::satisfies_label_admissibility(
        &declarations,
        &by_kind,
    ));

    // Every opt-in-capable member's features (every built-in kind *and* each custom
    // kind's members) — the stream coverage ranges over below.
    let all_features: Vec<extract::Features> = builtin_features
        .values()
        .flatten()
        .chain(custom_kinds.iter().flat_map(|(_, features)| features))
        .cloned()
        .collect();

    // The one requirement namespace: the assembly's declared `[requirement.*]`
    // roster. A custom-kind member has no channel of its own to publish a
    // requirement (the pre-0016 own-path surface that once carried one is retired);
    // the SDK already unions `harness.require` and every member's `requires` into
    // `declarations.requirements` at emit time, rejecting a cross-publisher name
    // collision there.
    let requirements = assembly_requirements.clone();

    // Each requirement's own definition is validated before the roster is
    // trusted to judge the harness.
    diagnostics.extend(roster::admissibility(&requirements, &by_kind, harness_root));

    // The declared selections, whole: every requirement's opt-in selection and every
    // kind's by-kind selection. Both lists are assembled before either is judged
    // because a `membership` clause draws its allowed set from a *second* selection,
    // and the judge resolves that target off this one list — the existential and the
    // universal binding are the same algebra, so neither can be judged in isolation.
    let mut selections = roster::selections(&requirements, &by_kind);
    selections.extend(kind_selections(&contracts, &by_kind));
    // The last of the dial's four sites, and the only one over selections rather than
    // contracts. A requirement's own clauses reach a judge only here,
    // as does the each-grain narrowing clause its `kind` facet sources — both are
    // synthesized past the contracts above, so dialing those alone would leave a
    // requirement's findings the one family an author could read an address off and not
    // dial. A kind selection re-dials clauses already dialed above; `apply` is
    // idempotent, and one site over the whole list beats a second rule about which half
    // of it is fresh — the dial's own kind selection excepted, since it carries a copy of
    // the very contract the loop above holds out of reach.
    for selection in &mut selections {
        if selection.selector == engine::Selector::Kind(dial::KIND.to_string()) {
            continue;
        }
        dialed.extend(dial.apply(mode, &mut selection.clauses));
    }
    diagnostics.extend(dial.refusals(&dialed));

    // Every dial site has run, so `dialed` is final and the three thirds are all in hand:
    // what judged this run beyond the committed harness, assembled here rather than
    // re-derived by whichever reporter renders it.
    let announcement = check::Announcement {
        local_members,
        dialed_clauses: dialed.into_iter().collect(),
        joined_locks,
    };

    // The selection grain: `count` / `unique` / `membership` whole, `kind` each.
    diagnostics.extend(engine::judge(&selections));

    // The edge scope: build the reference graph over the declared edges and check route
    // resolution — a declared reference must resolve to a real artifact of the
    // target kind. Admissibility before conformance:
    // an edge naming no reference field or targeting an unmodeled kind is
    // reported once and skipped by the route check.
    diagnostics.extend(graph::admissibility(&edges, &by_kind));
    diagnostics.extend(graph::check(&edges, &by_kind));

    // Mention route resolution: `emit` defers a mention naming a declared kind with no
    // composed member — its row rides the lock — so `check` owns that verdict here,
    // resolving each mention's target against the discovered corpus (members) and the
    // roster (bare requirement names). A dangler fires `graph.route` exactly as a declared
    // reference does.
    diagnostics.extend(graph::route_mentions(
        &mention_edges,
        &by_kind,
        &assembly_requirements,
    ));

    // Compute the resolved edges once, shared across acyclic, degree, and mention_reachable
    // to avoid recomputation of the whole-input edge-resolution walk.
    let resolved_edges_result = graph::resolved_edges(&edges, &by_kind);
    let resolved_edges = &resolved_edges_result.resolved;

    // `acyclic`: the resolved graph must contain no
    // cycle — a circular import loads nothing, so every finding is a true
    // positive. Always-on over the whole edge set, like route resolution above.
    diagnostics.extend(graph::acyclic(resolved_edges));

    // `degree`: the one set predicate whose judge needs the graph — a clause bounds
    // every selected member's in/out edge count, so it takes the same selections
    // `engine::judge` reads *and* the edges, reusing the arc resolution
    // `acyclic`/`check` assemble, plus the already-resolved mention edges —
    // obligation-free by default, counted only when a `degree` clause opts in.
    diagnostics.extend(graph::degree(&selections, resolved_edges, &mention_edges));

    // `mention-reachable`: the second selection predicate whose judge needs the graph —
    // each selected member's references must be able to fire where their target can be
    // invoked, which reads the *target* member's gate field. It ranges over the same
    // unified edge set `degree` does — the resolved field edges *and* the mention/import
    // family — so a rendering claim carried on a field edge is judged, never dropped.
    // Opt-in like `degree`: a selection declaring no such clause does no work.
    diagnostics.extend(graph::mention_reachable(
        &selections,
        resolved_edges,
        &mention_edges,
        &by_kind,
        &embedded_hosts_by_source(&declarations),
    ));

    // The requirement-coverage tier: every `required`
    // requirement must have a resolving home (≥1 artifact opting in via
    // `satisfies`) and every authored `satisfies` must resolve to a declared
    // requirement. Kind-blind: it ranges over every opt-in-capable artifact —
    // built-in kinds *and* each custom kind's members — so temper's own `spec`
    // corpus can opt in exactly as a skill does. The
    // requirement set is the *unioned* namespace, so a member-published obligation
    // is gated here exactly as an assembly-published one.
    diagnostics.extend(coverage::check(&requirements, &all_features));

    // The install self-verify: temper checking its
    // *own* gate is wired. Advisory (warn) only — a not-yet-installed gate nudges
    // without failing the run, and the session-start reporter ignores warn
    // severity.
    diagnostics.extend(install::gate_installed(harness_root));

    // The wedge's advisory coverage note: state which kinds checked how many members,
    // and name the known Claude Code surfaces present on disk that no kind — built-in
    // or locked custom — governs, so the gate's silence about an unmodeled surface never
    // reads as "checked". Warn-only — it leaves the run's exit code and the session-start
    // verdict unchanged. Threads the already-parsed `committed.kinds` to avoid a redundant
    // lock re-parse (COVERAGE-NOTE-LOCK-PARSE-HOIST).
    diagnostics.extend(coverage_note::check(
        harness_root,
        &builtin_kind::definitions(),
        &member_counts,
        &committed.kinds,
    )?);

    // The freshness fact: a committed projection
    // whose bytes no longer match the lock's emit fingerprint is `config.stale`. Read
    // off the surface `workspace`'s lock (where the members were imported and the
    // fingerprints recorded), advisory so a hand-edited or un-re-emitted projection is
    // surfaced without failing the run.
    diagnostics.extend(drift::config_stale_from_doc(&lock_doc, workspace));

    // The source-dependency freshness facts: a fingerprinted layout-import or
    // composed-prose include target whose bytes no longer match the lock — the target
    // moved and `emit` has not re-run. Advisory, the same `warn` posture `config.stale`
    // takes over a drifted projection. Use the pre-parsed lock document to avoid re-reading.
    let harness_root_for_staleness = drift::harness_root_of(workspace);
    diagnostics.extend(drift::layout_import_stale_from_doc(
        &lock_doc,
        &harness_root_for_staleness,
    )?);
    diagnostics.extend(drift::include_stale_from_doc(
        &lock_doc,
        &harness_root_for_staleness,
    )?);

    Ok((diagnostics, announcement))
}

/// Every kind's **by-kind selection**: its whole member population — the universal
/// binding — bound to its own contract's clauses.
///
/// The clause set is the contract entire, not the set clauses alone: `engine::judge`
/// reads the ones that range over the selection and `engine::validate` has already read
/// the member-grain ones off the identical contract, so the two judges split one
/// declaration rather than two filtered copies of it. A kind in `by_kind` with no
/// contract of its own declares no clause to judge and contributes no selection.
fn kind_selections<'a>(
    contracts: &BTreeMap<String, Contract>,
    by_kind: &BTreeMap<&'a str, &'a [extract::Features]>,
) -> Vec<engine::Selection<'a>> {
    by_kind
        .iter()
        .filter_map(|(kind, features)| {
            let contract = contracts.get(*kind)?;
            Some(engine::Selection {
                selector: engine::Selector::Kind((*kind).to_string()),
                clauses: contract.clauses.clone(),
                members: features.iter().map(|feature| (*kind, feature)).collect(),
            })
        })
        .collect()
}

/// Each embedded member's source node keyed to its **host**'s node: `(embedded-kind,
/// key) → (host-kind, host-id)`, read off the `nested_member` rows' `host` address. An
/// embedded-carried edge keys its source to the embedded member, never the host, so
/// `graph::mention_reachable` needs this map to judge a body-carried citation under its
/// host's scope — the source-side twin of the target-side `target_identity` seam. A row
/// whose `host` is not a `kind:name` address is skipped: it addresses no host
/// node, so the edge it would map stays keyed to the embedded member alone.
fn embedded_hosts_by_source(
    declarations: &drift::Declarations,
) -> BTreeMap<graph::Node, graph::Node> {
    let mut hosts = BTreeMap::new();
    for row in &declarations.nested_members {
        if let Some((kind, name)) = row.host.split_once(':') {
            hosts.insert(
                (row.kind.clone(), row.key.clone()),
                (kind.to_string(), name.to_string()),
            );
        }
    }
    hosts
}
