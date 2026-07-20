//! Admissibility judges for declared-program well-formedness.
//!
//! Seven judges cluster by one job: validating declared kinds, clauses, members,
//! and collisions before the corpus is trusted to model the harness. Each judge
//! answers one narrow question about the lock's coherence, running in the assembly
//! tier before any member is read.

use std::collections::{BTreeMap, BTreeSet};

use crate::check;
use crate::compose;
use crate::contract::Contract;
use crate::drift;
use crate::engine;
use crate::extract;
use crate::kind::CustomKind;

/// The embedded kinds the lock declares: every child kind a host names, whether through
/// its `templates` column — a *path-less* entry, the embedded layer; a `path` templates a
/// file child, which owns its own unit — or a layout member collection's `member_kind`. The set a
/// `nested_member` row's kind must belong to — a row of any other kind is an orphan no
/// host templates ([`nested_member_admissibility`]) — and the keys
/// [`embedded_features_by_kind`] seeds its corpus with.
pub fn declared_embedded_kinds(declarations: &drift::Declarations) -> BTreeSet<String> {
    let mut kinds = BTreeSet::new();
    for row in &declarations.kinds {
        for template in &row.templates {
            // A template carrying a `path` templates a *file* child — the child owns a
            // unit at that pattern rather than a fence in the host's body — so its child
            // kind is no part of the embedded set.
            if template.path.is_none() {
                kinds.insert(template.kind.clone());
            }
        }
        if let Some(content) = &row.content {
            for region in &content.regions {
                if let Some(member_kind) = &region.member_kind {
                    kinds.insert(member_kind.clone());
                }
            }
        }
    }
    kinds
}

/// The diagnostic `rule` id an orphaned `nested_member` row reports under — a committed
/// lock's cross-family incoherence, decided before the by-kind corpus is trusted to model
/// the harness.
const NESTED_MEMBER_ADMISSIBILITY_RULE: &str = "nested-member.admissibility";

/// The diagnostic `rule` id a bare `satisfies` label two kinds both claim reports under.
const SATISFIES_LABEL_ADMISSIBILITY_RULE: &str = "satisfies.admissibility";

/// Validate the lock's `nested_member` rows against the declared nesting: every row's kind
/// must be an embedded kind some host declares — a `templates` column entry or a layout
/// member collection's `member_kind`. A row of a kind no host templates is an orphan the
/// by-kind corpus would unmodel while the host-address read still carries it — the two
/// disagreeing over one committed lock. Reject it here, naming the kind and its host, the
/// same malformed-lock class as two rows wearing one label.
pub fn nested_member_admissibility(declarations: &drift::Declarations) -> Vec<check::Diagnostic> {
    let declared = declared_embedded_kinds(declarations);
    declarations
        .nested_members
        .iter()
        .filter(|row| !declared.contains(&row.kind))
        .map(|row| {
            check::Diagnostic::error(
                NESTED_MEMBER_ADMISSIBILITY_RULE,
                &row.host,
                format!(
                    "nested member `{}` under `{}` is of kind `{}`, which no host declares as a \
                     nested template — an orphaned lock row no kind's `templates` or member \
                     collection admits",
                    row.key, row.host, row.kind
                ),
            )
        })
        .collect()
}

/// Reject a bare `satisfies` label a same-named member of two kinds both carry. A
/// canonical row addresses its filler by `kind:name`, so a bare label is one an older
/// engine wrote; it qualifies against the live corpus where exactly one kind bears the
/// name, but a name two kinds share is the ambiguous lock the closed identity forbids —
/// refused loud here rather than cross-attributed to both members' fill sets. A
/// qualified label already names its kind and can never be ambiguous.
pub fn satisfies_label_admissibility(
    declarations: &drift::Declarations,
    by_kind: &BTreeMap<&str, &[extract::Features]>,
) -> Vec<check::Diagnostic> {
    let bare: BTreeSet<&str> = declarations
        .satisfies
        .iter()
        .map(|row| row.member.as_str())
        .filter(|member| !member.contains(':'))
        .collect();
    bare.into_iter()
        .filter_map(|member| {
            let kinds: Vec<&str> = by_kind
                .iter()
                .filter(|(_, members)| members.iter().any(|features| features.id == member))
                .map(|(kind, _)| *kind)
                .collect();
            (kinds.len() > 1).then(|| {
                check::Diagnostic::error(
                    SATISFIES_LABEL_ADMISSIBILITY_RULE,
                    member,
                    format!(
                        "satisfies row `{}` is a bare member label the `{}` kinds each carry — an \
                         ambiguous lock row no single member can own",
                        member,
                        kinds.join("`, `"),
                    ),
                )
            })
        })
        .collect()
}

/// The diagnostic `rule` id a lock-declared kind row reports under when its bare name
/// matches an embedded built-in's but its declared shape does not: it can be admitted
/// neither as that built-in's relocated `governs` (the one legitimate reason a row
/// reuses a built-in's name — [`row_relocates_builtin`]) nor as a distinct custom kind
/// (the name is already claimed). Shares the roster's admissibility tag — a colliding
/// bare name is inadmissible, decided before anything judges the kind.
const KIND_COLLISION_RULE: &str = "kind.admissibility";

/// A [`KIND_COLLISION_RULE`] finding for a colliding row from [`compose::partition_kind_rows`] —
/// refusing rather than the silent skip that dropped the row's members from every
/// corpus with no diagnostic (KIND-NAME-COLLISION-ADMISSIBILITY).
pub fn kind_collision_diagnostic(row: &drift::KindFactRow) -> check::Diagnostic {
    check::Diagnostic::error(
        KIND_COLLISION_RULE,
        &row.name,
        format!(
            "kind `{}` collides with an embedded built-in of the same name: its declared \
             shape (`format`/`unit_shape`/`registration`) does not match the built-in's, so \
             it is neither an admissible relocation of the built-in's `governs` locus nor a \
             distinct custom kind of its own — rename it, or drop the diverging facts to \
             relocate the built-in instead",
            row.name
        ),
    )
}

/// The diagnostic `rule` id two clause rows sharing one address report under. Sibling of
/// [`KIND_COLLISION_RULE`] and [`GOVERNS_COLLISION_RULE`], one namespace down: those guard
/// the kind's bare name and its locus, this one guards the clause's own address.
const CLAUSE_COLLISION_RULE: &str = "clause.label-collision";

/// A [`CLAUSE_COLLISION_RULE`] finding per address worn by more than one clause row —
/// the whole declaration set at once, kinds' own clauses and requirements' nested ones
/// alike, since a requirement's address shares the namespace.
///
/// A clause's label is its identity: the name every one of its findings prints and the
/// only name a dial can reach it by. Two rows under one label leave both unaddressable —
/// a dial entry would silently hit whichever the reader resolved first, and a finding
/// would name a clause the author cannot tell apart from its twin. That is a malformed
/// lock, refused before it judges anything, never a collision resolved with a counter
/// that would renumber a clause's siblings every time one is inserted above it.
///
/// `joined` faces the same rule: a layer's rows share the address space they are judged
/// in, so two of them under one label leave both unaddressable exactly as the host's own
/// twins would. A joined row can never collide with a *host* row — its address carries the
/// layer that produced it ([`LAYER_QUALIFIER`]) — so what fires here is a malformed layer
/// or a malformed corpus, never the join itself.
pub fn clause_collision_diagnostics(
    declarations: &drift::Declarations,
    joined: &[drift::ClauseRow],
) -> Vec<check::Diagnostic> {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    let rows = declarations
        .clauses
        .iter()
        .chain(declarations.requirements.iter().flat_map(|r| &r.clauses))
        .chain(joined);
    for row in rows {
        if let Some(label) = &row.label {
            *counts.entry(label.as_str()).or_default() += 1;
        }
    }
    counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(label, count)| {
            check::Diagnostic::error(
                CLAUSE_COLLISION_RULE,
                label,
                format!(
                    "{count} clause rows share the address `{label}`, so neither can be \
                     named: a clause's label is its identity — the id its findings print \
                     and the only name a dial reaches it by — and one address can address \
                     one clause"
                ),
            )
        })
        .collect()
}

/// The diagnostic `rule` id a kind declaring an inadmissible commitment class reports
/// under. Sibling of [`GOVERNS_COLLISION_RULE`]: both guard the locus's own coherence
/// before the corpus is trusted to model the harness — that one across kinds, this one
/// within a kind.
const LOCAL_LOCUS_RULE: &str = "kind.local-locus";

/// A [`LOCAL_LOCUS_RULE`] finding per kind whose declared `local` commitment class is
/// inadmissible — the locus fence ([`CustomKind::local_locus_fault`]), raised over every
/// kind in play before their members are read.
pub fn local_locus_admissibility(
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
    custom_rows: &[&drift::KindFactRow],
    _declarations: &drift::Declarations,
) -> Result<Vec<check::Diagnostic>, drift::LockRowError> {
    let mut kinds = Vec::new();
    for kind in overlaid_builtin_kinds.values() {
        kinds.push(kind.clone());
    }
    for row in custom_rows {
        kinds.push(CustomKind::from_kind_fact_row(row)?);
    }
    Ok(kinds
        .iter()
        .filter_map(|kind| {
            let fault = kind.local_locus_fault()?;
            Some(check::Diagnostic::error(
                LOCAL_LOCUS_RULE,
                &kind.name,
                format!(
                    "kind `{}` declares the `local` commitment class, but {fault}",
                    kind.name
                ),
            ))
        })
        .collect())
}

/// The diagnostic `rule` id a kind declaring a non-empty `registration` with no file locus
/// reports under. Sibling of [`LOCAL_LOCUS_RULE`]: both guard the locus's own coherence
/// before the corpus is trusted to model the harness — that one within a kind's file
/// declaration, this one within a kind's locus declaration.
const REGISTRATION_LOCUS_RULE: &str = "kind.registration-locus";

/// A [`REGISTRATION_LOCUS_RULE`] finding per kind whose declared `registration` is
/// inadmissible under its locus — the registration fence ([`CustomKind::registration_locus_fault`]),
/// raised over every kind in play before their members are read.
pub fn registration_locus_admissibility(
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
    custom_rows: &[&drift::KindFactRow],
    _declarations: &drift::Declarations,
) -> Result<Vec<check::Diagnostic>, drift::LockRowError> {
    let mut kinds = Vec::new();
    for kind in overlaid_builtin_kinds.values() {
        kinds.push(kind.clone());
    }
    for row in custom_rows {
        kinds.push(CustomKind::from_kind_fact_row(row)?);
    }
    Ok(kinds
        .iter()
        .filter_map(|kind| {
            let fault = kind.registration_locus_fault()?;
            Some(check::Diagnostic::error(
                REGISTRATION_LOCUS_RULE,
                &kind.name,
                format!(
                    "kind `{}` declares a non-empty `registration`, but {fault}",
                    kind.name
                ),
            ))
        })
        .collect())
}

/// The diagnostic `rule` id for two distinct kinds resolving to the same `governs`
/// (root+glob) locus. Sibling of [`KIND_COLLISION_RULE`], which guards the bare-name
/// namespace; this one guards the locus namespace — a document's kind is its position
/// alone, so two kinds selecting the same locus is a routing ambiguity, not two homes.
const GOVERNS_COLLISION_RULE: &str = "kind.governs-collision";

/// Governs-glob-collision findings over the **effective** kind set: the built-in
/// definitions, each overlaid with any [`row_relocates_builtin`] row that moves its
/// locus, plus the genuinely-custom rows. Two distinct kinds resolving to the same
/// `governs` (root+glob) would silently double-route every matching document into both
/// member sets — a document's kind is its position alone (`representation.md`) — so each
/// shared locus surfaces one error naming the kinds and the glob. A relocation is
/// resolved before comparison, so moving a built-in's locus to a fresh path is never a
/// self-collision; moving it *onto* a custom kind's locus is.
///
/// Manifest kinds (a `collection_address`) are excluded: they register members at an
/// address *inside* a host manifest, never claiming the whole document, so a manifest
/// kind legitimately shares its host file with the file-locus kind that owns it whole (a
/// `hook` and a `settings.json`-owning kind coexist). The mining the spec forbids is two
/// *document* kinds contending for one file; two manifest kinds contending for one
/// collection address is a distinct, out-of-scope question.
///
/// A nested file kind falls out for a stronger reason: it governs no glob at all — its
/// members' paths compose from their host's unit and the host template's pattern — so it
/// enters no bucket and can contend with nobody. That is the whole point of the locus.
pub fn governs_collision_diagnostics(
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
    custom_rows: &[&drift::KindFactRow],
    _declarations: &drift::Declarations,
) -> Result<Vec<check::Diagnostic>, drift::LockRowError> {
    let mut by_governs: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for kind in overlaid_builtin_kinds.values() {
        let Some(governs) = kind.governs.clone() else {
            continue;
        };
        if kind.collection_address.is_some() {
            continue;
        }
        by_governs
            .entry((governs.root, governs.glob))
            .or_default()
            .push(kind.name.clone());
    }
    for row in custom_rows {
        let Some(governs) = row.governs_root.clone().zip(row.governs_glob.clone()) else {
            continue;
        };
        if row.collection_address.is_some() {
            continue;
        }
        by_governs
            .entry(governs)
            .or_default()
            .push(row.name.clone());
    }
    Ok(by_governs
        .into_iter()
        .filter(|(_, names)| names.len() > 1)
        .map(|((root, glob), mut names)| {
            names.sort();
            governs_collision_diagnostic(&root, &glob, &names)
        })
        .collect())
}

/// A [`GOVERNS_COLLISION_RULE`] finding naming every kind that shares the
/// `root`/`glob` locus and the glob itself.
fn governs_collision_diagnostic(root: &str, glob: &str, names: &[String]) -> check::Diagnostic {
    let named = names
        .iter()
        .map(|name| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(", ");
    check::Diagnostic::error(
        GOVERNS_COLLISION_RULE,
        format!("{root}/{glob}"),
        format!(
            "kinds {named} share the `governs` glob `{root}/{glob}` — a document's kind is \
             its position alone, so two kinds selecting the same locus would route every \
             matching document into both member sets; give each kind a distinct `governs`",
        ),
    )
}

/// Every kind a joined clause names that this corpus declares none of. Nothing here
/// selects such a clause, so it judges nothing — but a layer must fail closed whether
/// or not the host happens to give its clauses something to range over, so the rows
/// still face the admissibility their kind's own dispatcher would have run.
pub fn joined_kind_admissibility(
    joined: &[drift::ClauseRow],
    contracts: &BTreeMap<String, Contract>,
) -> Result<Vec<check::Diagnostic>, compose::ClauseRowError> {
    let undeclared: BTreeSet<&str> = joined
        .iter()
        .filter_map(|row| row.kind.as_deref())
        .filter(|kind| !contracts.contains_key(*kind))
        .collect();
    let mut diagnostics = Vec::new();
    for kind in undeclared {
        let contract = compose::default_contract_from_rows(joined, &[], kind)?;
        diagnostics.extend(engine::admissibility(&contract, &engine::Locus::Document));
    }
    Ok(diagnostics)
}
