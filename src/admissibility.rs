//! Admissibility judges for declared-program well-formedness.
//!
//! Eight judges cluster by one job: validating declared kinds, clauses, members,
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
use crate::member_address;

/// The embedded kinds the lock declares: every child kind a host names, whether through
/// its `templates` column — a *path-less* entry, the embedded layer; a `path` templates a
/// file child, which owns its own unit — or a layout member collection's `member_kind`. The set a
/// `nested_member` row's kind must belong to — a row of any other kind is an orphan no
/// host templates ([`nested_member_admissibility`]) — and the keys
/// [`embedded_features_by_kind`](crate::compose::embedded_features_by_kind) seeds its corpus with.
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
                     collection admits; declare `{}` in the `templates` of the kind that governs \
                     `{}` (or as a member collection's `member_kind`), or drop the member",
                    row.key, row.host, row.kind, row.kind, row.host
                ),
            )
        })
        .collect()
}

/// Reject a host declaring the **same `(kind, key)` nested member twice** — two rows
/// spelling one address. `representation.md` ("member") makes resolution total and
/// coincident addresses a malformed lock: within one host a nested member's address *is*
/// its `(kind, key)`, so the repeat names two members with one name and nothing
/// downstream — the by-kind corpus, the host-address read, an edge target — can tell them
/// apart. Refused here, the same malformed-lock class as an orphaned row
/// ([`nested_member_admissibility`]), under the same rule id.
///
/// Two **different** hosts sharing a `(kind, key)` are not coincident — their addresses
/// differ in the host segment (`spec:alpha/invariant/x` and `spec:beta/invariant/x` are
/// distinct) — and are admissible. What is ambiguous there is a *bare* reference to the
/// key, refused at resolution naming every carrier host (`crate::graph`), never at
/// declaration.
pub fn nested_member_coincidence(declarations: &drift::Declarations) -> Vec<check::Diagnostic> {
    let mut counts: BTreeMap<(&str, &str, &str), usize> = BTreeMap::new();
    for row in &declarations.nested_members {
        *counts
            .entry((row.host.as_str(), row.kind.as_str(), row.key.as_str()))
            .or_default() += 1;
    }
    counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|((host, kind, key), count)| {
            check::Diagnostic::error(
                NESTED_MEMBER_ADMISSIBILITY_RULE,
                host,
                format!(
                    "`{host}` declares the nested member `{}` {count} times — one address \
                     naming {count} members, a coincidence no reader can resolve; a host's \
                     `(kind, key)` names exactly one of its nested members",
                    member_address::nested_address(host, kind, key),
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
                let qualified = kinds
                    .iter()
                    .map(|kind| format!("`{kind}:{member}`"))
                    .collect::<Vec<_>>()
                    .join(" or ");
                check::Diagnostic::error(
                    SATISFIES_LABEL_ADMISSIBILITY_RULE,
                    member,
                    format!(
                        "satisfies row `{}` is a bare member label the `{}` kinds each carry — an \
                         ambiguous lock row no single member can own; qualify it as the member it \
                         means — {qualified}",
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
/// reuses a built-in's name — `row_relocates_builtin`) nor as a distinct custom kind
/// (the name is already claimed). Shares the roster's admissibility tag — a colliding
/// bare name is inadmissible, decided before anything judges the kind.
const KIND_COLLISION_RULE: &str = "kind.admissibility";

/// A [`KIND_COLLISION_RULE`] finding per colliding row from [`compose::partition_kind_rows`] —
/// refusing rather than the silent skip that dropped the row's members from every
/// corpus with no diagnostic.
///
/// # Errors
///
/// Returns a [`drift::LockRowError`] when a colliding row's own labels fall outside their
/// closed vocabulary.
pub fn kind_collision_diagnostics(
    collisions: &[&drift::KindFactRow],
    builtin_defs: &BTreeMap<String, CustomKind>,
) -> Result<Vec<check::Diagnostic>, drift::LockRowError> {
    collisions
        .iter()
        .map(|row| kind_collision_diagnostic(row, builtin_defs.get(&row.name)))
        .collect()
}

/// The finding itself, naming the shape facts that diverge from the built-in the row's
/// name claims — the row's two exits are to drop exactly those (and relocate the built-in)
/// or to change the name, and neither is choosable from the bare fact of a collision.
fn kind_collision_diagnostic(
    row: &drift::KindFactRow,
    builtin: Option<&CustomKind>,
) -> Result<check::Diagnostic, drift::LockRowError> {
    let declared = CustomKind::from_kind_fact_row(row)?;
    let mut diverging: Vec<String> = Vec::new();
    if let Some(builtin) = builtin {
        if let Some(label) = &row.format
            && declared.format != builtin.format
        {
            diverging.push(format!("`format` (`{label}`)"));
        }
        if let Some(label) = &row.unit_shape
            && declared.unit_shape != builtin.unit_shape
        {
            diverging.push(format!("`unit_shape` (`{label}`)"));
        }
        if !row.registration.is_empty() && declared.registration != builtin.registration {
            diverging.push(format!(
                "`registration` (`{}`)",
                row.registration.join("`, `")
            ));
        }
    }
    // A collision row names a built-in and diverges from it in at least one of the three
    // facts (`compose::partition_kind_rows`), so the fallback backstops an unreachable
    // empty list rather than printing a sentence with a hole in it.
    if diverging.is_empty() {
        diverging.push("`format`/`unit_shape`/`registration` shape".to_string());
    }
    Ok(check::Diagnostic::error(
        KIND_COLLISION_RULE,
        &row.name,
        format!(
            "kind `{}` collides with an embedded built-in of the same name: its declared {} \
             diverges from the built-in's, so the row is neither an admissible relocation of \
             the built-in's `governs` locus nor a distinct custom kind of its own — drop the \
             diverging facts to relocate the built-in, or rename `{}` to a name no built-in \
             claims",
            row.name,
            diverging.join(" and "),
            row.name
        ),
    ))
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
/// layer that produced it (`LAYER_QUALIFIER`) — so what fires here is a malformed layer
/// or a malformed corpus, never the join itself.
pub fn clause_collision_diagnostics(
    declarations: &drift::Declarations,
    joined: &[drift::ClauseRow],
) -> Vec<check::Diagnostic> {
    // The authoring site is the one fact a shared address cannot carry: the label already
    // exhausts owner, predicate, and field, so which declaration to edit is exactly what
    // the colliding rows differ in.
    let own = declarations.clauses.iter().map(|row| {
        let site = match row.kind.as_deref() {
            Some(kind) => format!("the `{kind}` kind's own clauses"),
            None => "a clause row naming no kind".to_string(),
        };
        (site, row)
    });
    let nested = declarations.requirements.iter().flat_map(|requirement| {
        requirement.clauses.iter().map(move |row| {
            (
                format!("requirement `{}`'s own clauses", requirement.name),
                row,
            )
        })
    });
    let layered = joined
        .iter()
        .map(|row| ("a joined layer's clauses".to_string(), row));
    let mut sites: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for (site, row) in own.chain(nested).chain(layered) {
        if let Some(label) = &row.label {
            sites
                .entry(label.as_str())
                .or_default()
                .push(format!("{site} at severity `{}`", row.severity));
        }
    }
    sites
        .into_iter()
        .filter(|(_, sites)| sites.len() > 1)
        .map(|(label, sites)| {
            check::Diagnostic::error(
                CLAUSE_COLLISION_RULE,
                label,
                format!(
                    "{} clause rows share the address `{label}` — {} — so none of them can be \
                     named: a clause's label is its identity, the id its findings print and \
                     the only name a dial reaches it by, and one address can address one \
                     clause. Keep the row you mean and drop the rest, or move one onto a \
                     field of its own",
                    sites.len(),
                    sites.join("; "),
                ),
            )
        })
        .collect()
}

/// Builds the effective kind set: every built-in kind, optionally overlaid with any
/// relocation, plus every custom kind from the lock.
fn effective_kinds(
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
    custom_rows: &[&drift::KindFactRow],
) -> Result<Vec<CustomKind>, drift::LockRowError> {
    let mut kinds = Vec::new();
    for kind in overlaid_builtin_kinds.values() {
        kinds.push(kind.clone());
    }
    for row in custom_rows {
        kinds.push(CustomKind::from_kind_fact_row(row)?);
    }
    Ok(kinds)
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
    let kinds = effective_kinds(overlaid_builtin_kinds, custom_rows)?;
    Ok(kinds
        .iter()
        .filter_map(|kind| {
            let fault = kind.local_locus_fault()?;
            Some(check::Diagnostic::error(
                LOCAL_LOCUS_RULE,
                &kind.name,
                format!(
                    "kind `{}` declares the `local` commitment class, but {fault} — declare a \
                     `governs` locus for `{}`, or drop its `local` commitment",
                    kind.name, kind.name
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
    let kinds = effective_kinds(overlaid_builtin_kinds, custom_rows)?;
    Ok(kinds
        .iter()
        .filter_map(|kind| {
            let fault = kind.registration_locus_fault()?;
            Some(check::Diagnostic::error(
                REGISTRATION_LOCUS_RULE,
                &kind.name,
                format!(
                    "kind `{}` declares a non-empty `registration`, but {fault} — drop the \
                     `registration` from `{}`, or give the kind a `governs` locus of its own",
                    kind.name, kind.name
                ),
            ))
        })
        .collect())
}

/// The diagnostic `rule` id an `at`-locus member rooted under the workspace directory reports
/// under. Sibling of [`LOCAL_LOCUS_RULE`] and [`REGISTRATION_LOCUS_RULE`]: all guard the
/// locus's own coherence before the corpus is trusted to model the harness.
const AT_LOCUS_UNDER_WORKSPACE_RULE: &str = "at-locus.under-workspace";

/// An [`AT_LOCUS_UNDER_WORKSPACE_RULE`] finding per `at`-locus member whose committed
/// `source_path` falls under `.temper/` — the same invariant emit refuses at projection
/// time, surfaced at check time over an already-committed lock.
pub fn at_locus_under_workspace_admissibility(
    kind_rows: &[&drift::KindFactRow],
    _declarations: &drift::Declarations,
) -> Vec<check::Diagnostic> {
    let mut diagnostics = Vec::new();
    for kind_row in kind_rows {
        // Only `at` loci (both governs_root and governs_glob present) can be under workspace.
        let Some(root) = kind_row.governs_root.as_deref() else {
            continue;
        };
        let Some(_glob) = kind_row.governs_glob.as_deref() else {
            continue;
        };
        let root_path = std::path::Path::new(root);
        let workspace = std::path::Path::new(crate::WORKSPACE_DIR);
        if root_path == workspace || root_path.starts_with(format!("{}/", crate::WORKSPACE_DIR)) {
            diagnostics.push(check::Diagnostic::error(
                AT_LOCUS_UNDER_WORKSPACE_RULE,
                &kind_row.name,
                format!(
                    "kind `{}` declares an `at` locus rooted at `{}`, which falls under the \
                     workspace directory `.temper/` — a member rooted there would emit and lock \
                     but never be discoverable by design; re-root `{}`'s `governs` outside \
                     `.temper/`",
                    kind_row.name, root, kind_row.name
                ),
            ));
        }
    }
    diagnostics
}

/// The diagnostic `rule` id a kind declaring more than one verbatim prose region reports
/// under. Sibling of other layout-coherence checks: the region is a declared template,
/// and a second verbatim prose region is a permanent no-op (Layout::read lands the preamble
/// in the first one only), so declaring it is a malformed definition.
const LAYOUT_DUPLICATE_PROSE_REGION_RULE: &str = "layout.duplicate-prose-region";

/// A [`LAYOUT_DUPLICATE_PROSE_REGION_RULE`] finding per kind declaring more than one
/// verbatim (non-import) prose region in its layout — the second and every subsequent
/// prose region without an import can never carry a byte, so declaring it is a permanent
/// no-op the admissible set excludes.
pub fn layout_duplicate_prose_region_admissibility(
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
    custom_rows: &[&drift::KindFactRow],
    _declarations: &drift::Declarations,
) -> Result<Vec<check::Diagnostic>, drift::LockRowError> {
    let mut diagnostics: Vec<check::Diagnostic> = overlaid_builtin_kinds
        .values()
        .filter_map(duplicate_prose_region_diagnostic)
        .collect();
    for row in custom_rows {
        diagnostics.extend(duplicate_prose_region_diagnostic(
            &CustomKind::from_kind_fact_row(row)?,
        ));
    }
    Ok(diagnostics)
}

/// The [`LAYOUT_DUPLICATE_PROSE_REGION_RULE`] finding for one kind, naming the regions
/// that can never carry a byte — every verbatim prose region past the first, which is the
/// exact set an author gives an `import` or removes.
fn duplicate_prose_region_diagnostic(kind: &CustomKind) -> Option<check::Diagnostic> {
    let crate::kind::Content::Layout(layout) = &kind.content else {
        return None;
    };
    let verbatim: Vec<usize> = layout
        .regions
        .iter()
        .enumerate()
        .filter_map(|(index, region)| {
            matches!(region, crate::layout::LayoutRegion::Prose { import: None }).then_some(index)
        })
        .collect();
    let (first, dead) = verbatim.split_first()?;
    if dead.is_empty() {
        return None;
    }
    let named = dead
        .iter()
        .map(|index| format!("region {index}"))
        .collect::<Vec<_>>()
        .join(", ");
    Some(check::Diagnostic::error(
        LAYOUT_DUPLICATE_PROSE_REGION_RULE,
        &kind.name,
        format!(
            "kind `{}` declares {} verbatim prose regions, but only region {first} can carry \
             text — `Layout::read` lands the preamble there — so every later one ({named}) is a \
             permanent no-op; give each of those an `import`, or remove them from the layout",
            kind.name,
            verbatim.len(),
        ),
    ))
}

/// The diagnostic `rule` id for two distinct kinds resolving to the same `governs`
/// (root+glob) locus. Sibling of [`KIND_COLLISION_RULE`], which guards the bare-name
/// namespace; this one guards the locus namespace — a document's kind is its position
/// alone, so two kinds selecting the same locus is a routing ambiguity, not two homes.
const GOVERNS_COLLISION_RULE: &str = "kind.governs-collision";

/// The diagnostic `rule` id for two distinct kinds declaring the same `collection_address`
/// (manifest + key_path). Sibling of [`GOVERNS_COLLISION_RULE`], which guards file-locus
/// collisions; this one guards manifest-registration collisions — two kinds at the same
/// address register members ambiguously, each silently union-selecting the other's members.
const COLLECTION_ADDRESS_COLLISION_RULE: &str = "kind.collection-address-collision";

/// Governs-glob-collision findings over the **effective** kind set: the built-in
/// definitions, each overlaid with any `row_relocates_builtin` row that moves its
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

/// Collection-address-collision findings: two distinct kinds declaring the same
/// manifest and key_path would silently union their selections and cross-apply each
/// other's contracts. Each shared address surfaces one error naming the kinds, the
/// manifest, and the key path. Like [`governs_collision_diagnostics`], this check runs
/// over the effective kind set: the overlaid built-in kinds plus the custom rows.
pub fn collection_address_collision_diagnostics(
    overlaid_builtin_kinds: &BTreeMap<String, CustomKind>,
    custom_rows: &[&drift::KindFactRow],
    _declarations: &drift::Declarations,
) -> Result<Vec<check::Diagnostic>, drift::LockRowError> {
    let mut by_address: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for kind in overlaid_builtin_kinds.values() {
        let Some(collection_address) = &kind.collection_address else {
            continue;
        };
        by_address
            .entry((
                collection_address.manifest.clone(),
                collection_address.key_path.wire_label().to_string(),
            ))
            .or_default()
            .push(kind.name.clone());
    }
    for row in custom_rows {
        let Some(collection_address) = &row.collection_address else {
            continue;
        };
        by_address
            .entry((
                collection_address.manifest.clone(),
                collection_address.key_path.clone(),
            ))
            .or_default()
            .push(row.name.clone());
    }
    Ok(by_address
        .into_iter()
        .filter(|(_, names)| names.len() > 1)
        .map(|((manifest, key_path), mut names)| {
            names.sort();
            collection_address_collision_diagnostic(&manifest, &key_path, &names)
        })
        .collect())
}

/// A [`COLLECTION_ADDRESS_COLLISION_RULE`] finding naming every kind that shares the
/// given manifest and key_path address.
fn collection_address_collision_diagnostic(
    manifest: &str,
    key_path: &str,
    names: &[String],
) -> check::Diagnostic {
    let named = names
        .iter()
        .map(|name| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(", ");
    check::Diagnostic::error(
        COLLECTION_ADDRESS_COLLISION_RULE,
        format!("{manifest}#{key_path}"),
        format!(
            "kinds {named} declare the same `collectionAddress` (`{manifest}#{key_path}`) — \
             two manifest kinds contending for one address would register members ambiguously, \
             each silently union-selecting the other's members; give each kind a distinct address",
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::builtin_kind;
    use crate::extract::Features;

    /// A [`drift::ClauseRow`] at `label`, carrying only the columns a collision reads.
    fn clause_row(kind: Option<&str>, label: &str, severity: &str) -> drift::ClauseRow {
        drift::ClauseRow {
            label: Some(label.to_string()),
            kind: kind.map(str::to_string),
            predicate: "required".to_string(),
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
            unit: None,
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

    /// A [`drift::KindFactRow`] naming `name` and declaring nothing else — the base each
    /// locus case struct-updates with the one fact it exercises.
    fn kind_row(name: &str) -> drift::KindFactRow {
        drift::KindFactRow {
            name: name.to_string(),
            provider: None,
            governs_root: None,
            governs_glob: None,
            commitment: None,
            format: None,
            unit_shape: None,
            registration: Vec::new(),
            templates: Vec::new(),
            content: None,
            shape: None,
            collection_address: None,
            guidance: None,
            cite: None,
        }
    }

    /// A minimal [`Features`] carrying just the id a bare `satisfies` label resolves by.
    fn member(id: &str) -> Features {
        Features {
            id: id.to_string(),
            fields: BTreeMap::new(),
            body_lines: 0,
            rendered_lines: Some(0),
            rendered_chars: Some(0),
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: None,
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            nested_members: Vec::new(),
            satisfies: Vec::new(),
            edge_placements: None,
        }
    }

    /// The one message of `findings`, or a panic naming what was found instead.
    fn only_message(findings: &[check::Diagnostic]) -> &str {
        assert_eq!(
            findings.len(),
            1,
            "expected one finding, got: {findings:#?}"
        );
        &findings[0].message
    }

    #[test]
    fn a_clause_collision_names_the_site_of_every_colliding_row() {
        let declarations = drift::Declarations {
            clauses: vec![clause_row(Some("skill"), "skill.required", "required")],
            requirements: vec![drift::RequirementRow {
                name: "approved-model".to_string(),
                kind: None,
                required: true,
                clauses: vec![clause_row(None, "skill.required", "advisory")],
                verifier: None,
                prose: None,
            }],
            ..drift::Declarations::default()
        };
        let message = only_message(&clause_collision_diagnostics(&declarations, &[])).to_string();
        // The shared label is the one thing both rows already agree on: what the author
        // needs is which two declarations to open.
        assert!(
            message.contains("the `skill` kind's own clauses at severity `required`"),
            "the host kind's row must be named: {message}"
        );
        assert!(
            message.contains("requirement `approved-model`'s own clauses at severity `advisory`"),
            "the requirement's nested row must be named: {message}"
        );
    }

    #[test]
    fn a_joined_layer_row_is_named_as_the_layer_it_arrived_on() {
        let declarations = drift::Declarations {
            clauses: vec![clause_row(Some("rule"), "rule.required", "required")],
            ..drift::Declarations::default()
        };
        let joined = vec![clause_row(Some("rule"), "rule.required", "required")];
        let message =
            only_message(&clause_collision_diagnostics(&declarations, &joined)).to_string();
        assert!(
            message.contains("a joined layer's clauses"),
            "the joined row must be named as the layer's: {message}"
        );
    }

    #[test]
    fn an_orphaned_nested_member_names_the_declaration_that_would_admit_it() {
        let declarations = drift::Declarations {
            nested_members: vec![drift::NestedMemberRow {
                host: "spec:intent".to_string(),
                kind: "invariant".to_string(),
                key: "loud-or-nothing".to_string(),
                leaves: BTreeMap::new(),
                collections: Vec::new(),
                placed_edges: None,
                rendered_lines: None,
                rendered_chars: None,
            }],
            ..drift::Declarations::default()
        };
        let message = only_message(&nested_member_admissibility(&declarations)).to_string();
        assert!(
            message.contains("declare `invariant` in the `templates` of the kind that governs"),
            "the admitting declaration must be named: {message}"
        );
    }

    #[test]
    fn an_ambiguous_satisfies_label_names_the_qualified_spellings_that_resolve_it() {
        let declarations = drift::Declarations {
            satisfies: vec![drift::SatisfiesRow {
                member: "review".to_string(),
                requirement: "gated".to_string(),
            }],
            ..drift::Declarations::default()
        };
        let skills = vec![member("review")];
        let rules = vec![member("review")];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("skill", skills.as_slice()), ("rule", rules.as_slice())]);
        let message =
            only_message(&satisfies_label_admissibility(&declarations, &by_kind)).to_string();
        assert!(
            message.contains("`rule:review`") && message.contains("`skill:review`"),
            "both qualified spellings must be offered: {message}"
        );
    }

    #[test]
    fn a_kind_collision_names_the_fact_that_diverges_from_the_builtin() {
        let row = drift::KindFactRow {
            unit_shape: Some("file".to_string()),
            ..kind_row("skill")
        };
        let findings = kind_collision_diagnostics(&[&row], &builtin_kind::definitions()).unwrap();
        let message = only_message(&findings).to_string();
        assert!(
            message.contains("`unit_shape` (`file`)"),
            "the diverging fact must be named: {message}"
        );
        assert!(
            !message.contains("`format`"),
            "a fact matching the built-in's is no part of the edit: {message}"
        );
    }

    #[test]
    fn a_local_locus_fault_names_the_two_columns_that_resolve_it() {
        let row = drift::KindFactRow {
            commitment: Some("local".to_string()),
            ..kind_row("scratch")
        };
        let findings =
            local_locus_admissibility(&BTreeMap::new(), &[&row], &drift::Declarations::default())
                .unwrap();
        let message = only_message(&findings).to_string();
        assert!(
            message.contains("declare a `governs` locus for `scratch`")
                && message.contains("drop its `local` commitment"),
            "both exits must be named: {message}"
        );
    }

    #[test]
    fn a_registration_locus_fault_names_the_two_columns_that_resolve_it() {
        let row = drift::KindFactRow {
            registration: vec!["always".to_string()],
            ..kind_row("nested-note")
        };
        let findings = registration_locus_admissibility(
            &BTreeMap::new(),
            &[&row],
            &drift::Declarations::default(),
        )
        .unwrap();
        let message = only_message(&findings).to_string();
        assert!(
            message.contains("drop the `registration` from `nested-note`")
                && message.contains("give the kind a `governs` locus of its own"),
            "both exits must be named: {message}"
        );
    }

    #[test]
    fn a_workspace_rooted_at_locus_names_the_re_rooting() {
        let row = drift::KindFactRow {
            governs_root: Some(".temper/notes".to_string()),
            governs_glob: Some("*.md".to_string()),
            ..kind_row("note")
        };
        let findings =
            at_locus_under_workspace_admissibility(&[&row], &drift::Declarations::default());
        let message = only_message(&findings).to_string();
        assert!(
            message.contains("re-root `note`'s `governs` outside `.temper/`"),
            "the re-rooting must be named: {message}"
        );
    }

    #[test]
    fn duplicate_prose_regions_name_only_the_regions_that_can_never_carry_a_byte() {
        let prose = |import: Option<&str>| drift::LayoutRegionRow {
            region: "prose".to_string(),
            import: import.map(str::to_string),
            slot: None,
            member_kind: None,
            key: None,
        };
        let row = drift::KindFactRow {
            governs_root: Some("specs".to_string()),
            governs_glob: Some("*.md".to_string()),
            content: Some(drift::LayoutRow {
                regions: vec![prose(None), prose(Some("head.md")), prose(None)],
            }),
            ..kind_row("spec")
        };
        let findings = layout_duplicate_prose_region_admissibility(
            &BTreeMap::new(),
            &[&row],
            &drift::Declarations::default(),
        )
        .unwrap();
        let message = only_message(&findings).to_string();
        // Region 1 imports, so it carries bytes; region 0 is the one that reads. Only
        // region 2 is the no-op, and only it is what the author edits.
        assert!(
            message.contains("only region 0 can carry text") && message.contains("(region 2)"),
            "only the dead region must be named: {message}"
        );
    }
}
