//! The `Contract` artifact — the decidable artifact-clause algebra.
//!
//! A [`Contract`] is a named set of [`Clause`]s over a **closed** vocabulary of
//! decidable predicates, each carrying an author-declared [`Severity`]. Every live
//! `Contract` is built from the lock's `ClauseRow` family ([`crate::builtin`],
//! [`crate::compose::default_contract_from_rows`]) — there is no hand-authored clause
//! grammar; the SDK's types are the only authoring spelling.
//!
//! There is no arbitrary-code clause: adding a predicate is a deliberate language
//! change, never a per-contract escape hatch.

use std::collections::BTreeSet;

use crate::drift::{CharsetRow, ClauseRow};
use crate::extract::ValueType;

/// A named set of clauses over the decidable predicate algebra — the type a
/// harness (or one artifact in it) is checked against.
///
/// Not `Eq`: the `range` predicate carries `f64` bounds,
/// and `f64` is only `PartialEq`. Equality is still derived (the tests compare
/// whole contracts), just not the reflexive marker.
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    /// Display label for diagnostics — an explicit top-level `name` if present,
    /// else the file stem. A contract's *identity* is its path/role, not this
    /// field, so `name` is never a required input.
    pub name: String,
    /// The clauses, in declaration order. An empty set is a valid (vacuous)
    /// contract — a named shape that asserts nothing.
    pub clauses: Vec<Clause>,
    /// Contract-level **guidance**: best-practice
    /// prose the clauses cannot encode. Like the per-clause
    /// [`guidance`](Clause::guidance) channel it *never gates* — the closed algebra
    /// has no path from prose to a predicate. `None` when the contract authors none.
    pub guidance: Option<String>,
}

/// One clause: a decidable [`Predicate`] plus the [`Severity`] its author
/// declared for it. Pairing the two here is the whole point — `temper` never
/// decides error-vs-warning; the contract does.
///
/// Not `Eq` — its [`Predicate`] may carry `f64` `range` bounds; see [`Contract`].
#[derive(Debug, Clone, PartialEq)]
pub struct Clause {
    /// The clause's **address** — the label emit wrote onto its lock row
    /// ([`clause_label`]), lifted back here verbatim and printed as the `rule` id of
    /// every finding this clause produces. Opaque to the engine: nothing here parses
    /// it, derives it, or disambiguates two clauses that wear the same one — a
    /// collision is a malformed lock, refused before the contract judges anything.
    pub label: String,
    /// Whether a violation of this clause blocks the gate or is merely reported.
    pub severity: Severity,
    /// The decidable predicate this clause asserts over the surface.
    pub predicate: Predicate,
    /// Optional per-clause **guidance** prose — advisory-only best-practice text
    /// kept *out of checks*: it plays no part
    /// in conformance or admissibility. It rides its JSON Schema property's
    /// `description` in the emitted schema, never a validation keyword — taste becomes documentation, never a
    /// squiggle. Absent ⇒ the clause documents nothing.
    pub guidance: Option<String>,
    /// Optional **source** citation — the clause's provenance of taste, a URL plus
    /// retrieval date.
    /// *Preserved metadata*, not a predicate: no gate reads its content, so admitting
    /// it neither adds nor relaxes any check. Absent ⇒ the clause is uncited (every
    /// clause on disk today).
    pub source: Option<String>,
}

/// A clause's compiled **address**, from the columns that identify its row: dot-joined
/// `<owner>.<predicate>[.<field>]`, where `owner` is the kind whose contract carries the
/// clause (a requirement's own clause passes [`requirement_owner`] instead) and `field`
/// is present exactly when the predicate names one.
///
/// Deterministic — a pure function of the row, so the same program emits the same
/// labels — and human-legible, because the author who reads a finding's `rule` id must
/// be able to spell it back into a dial entry. Uniqueness is *not* enforced here: two
/// rows reducing to one label are a malformed lock, refused by admissibility rather
/// than disambiguated with a counter that would renumber every sibling on an insert.
///
/// `owner` is `None` only for a row that names no kind and hangs off no requirement — a
/// shape no producer writes and no consumer reads; its label simply omits the segment.
#[must_use]
pub fn clause_label(owner: Option<&str>, predicate: &str, field: Option<&str>) -> String {
    owner
        .into_iter()
        .chain(std::iter::once(predicate))
        .chain(field)
        .collect::<Vec<_>>()
        .join(".")
}

/// The [`clause_label`] owner segment for a clause attached to a requirement rather
/// than to a kind: `requirement.<name>`, keeping a requirement's demands in the same
/// namespace its other findings already report under.
#[must_use]
pub fn requirement_owner(name: &str) -> String {
    format!("requirement.{name}")
}

/// The author-declared weight of a clause. Replaces the tool-baked error/warn
/// split: the default gate blocks on `Required` clauses only, and a strict CI
/// policy can promote `Advisory` to blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Gate-blocking: a violation fails the run.
    Required,
    /// Reported but non-blocking by default.
    Advisory,
}

/// A single decidable predicate from the closed vocabulary. Given the surface,
/// every variant is unambiguously true or false — so a violation is always a
/// true positive, which is what earns the hard gate.
///
/// ## The field a predicate names
///
/// A value predicate's `field` is an **addressing path** ([`crate::address`]): name
/// segments walk into an object (`owner.name`), and `[*]` grains over an array's
/// elements (`plugins[*].source`), so one clause decides once per element and indicts
/// each offender by its own address. That subset — names and `[*]`, nothing else — is
/// the whole surface, and [`crate::engine::admissibility`] refuses the rest rather than
/// evaluating it: an author names *where* a value lives, never a pattern that matches
/// it, and the RFC engine underneath stays mechanics.
///
/// [`Predicate::ForbiddenKeys`] and [`Predicate::MustDefine`] name a top-level **key**
/// instead; [`Predicate::ClosedKeys`] names the key *set*, reading it off its sibling
/// clauses; and the set predicates' `field` is read by their own judges over a selection.
///
/// Not `Eq`: [`Predicate::Range`] carries `f64` bounds (only `PartialEq`).
#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
    /// `required`: the named field must be present.
    ///
    /// Presence is asked of the path's trailing name segment, so a path ending in `[*]`
    /// names elements rather than a key and is inadmissible — there would be nothing to
    /// be absent.
    Required {
        /// The field that must be present.
        field: String,
    },
    /// `optional`: the named field may be present (always satisfied; it records
    /// that the key is part of the declared schema, e.g. for a closed surface).
    Optional {
        /// The field that is permitted.
        field: String,
    },
    /// `type`: the field's preserved source kind is one of the declared [`ValueType`]s
    /// over the closed scalar/container lattice (`string`/`integer`/`number`/
    /// `boolean`/`list`/`map`/`null`). Unlike `min_len`/`enum`/`pattern`, which
    /// refine *within* a scalar type, `type` only fixes the kind.
    ///
    /// The declaration is a **set**, not one kind: an external format that documents a
    /// field as `string|array` is gateable by the set `{string, list}`, where a
    /// single-kind clause could only reject one of the two documented forms — a false
    /// positive, which no clause may produce. A one-element set is the single-kind
    /// clause exactly, down to the diagnostic's wording; an empty set admits no value
    /// at all and is inadmissible ([`crate::engine`]), the rule an inverted `range`
    /// or `count` bound already sets.
    Type {
        /// The field constrained.
        field: String,
        /// The declared source kinds, any one of which the field may carry. Held as a
        /// set so the author's write order is not a distinction the lock, the
        /// diagnostic, or the emitted schema can carry.
        kinds: BTreeSet<ValueType>,
    },
    /// `min_len`: the field's value is at least `min` characters long.
    MinLen {
        /// The field measured.
        field: String,
        /// The inclusive lower bound, in characters.
        min: usize,
    },
    /// `max_len`: the field's value is at most `max` characters long.
    MaxLen {
        /// The field measured.
        field: String,
        /// The inclusive upper bound, in characters.
        max: usize,
    },
    /// `range`: the field's numeric value lies within the inclusive `[min, max]`
    /// bound over `integer`/`number` fields. Bounds are `f64` so a single predicate spans both integer and
    /// fractional fields; an inverted `min > max` bound is rejected as inadmissible
    /// (`crate::engine`).
    Range {
        /// The field measured.
        field: String,
        /// The inclusive lower bound.
        min: f64,
        /// The inclusive upper bound.
        max: f64,
    },
    /// `enum`: the field's value is one of `values`.
    Enum {
        /// The field constrained.
        field: String,
        /// The permitted values.
        values: Vec<String>,
    },
    /// `deny`: the field's value is none of `values` (forbidden values).
    Deny {
        /// The field constrained.
        field: String,
        /// The forbidden values.
        values: Vec<String>,
    },
    /// `forbidden_keys`: none of `keys` appear (e.g. the Cursor `globs` /
    /// `alwaysApply` keys Claude Code ignores).
    ForbiddenKeys {
        /// The keys that must be absent.
        keys: Vec<String>,
    },
    /// `closed-keys`: the kind's declared top-level key set is **exhaustive** — a member
    /// carrying a key no clause declares is a finding. The deny-list complement of
    /// [`Predicate::ForbiddenKeys`], which names a finite set over an open key space.
    ///
    /// It carries no argument of its own: the allow-list is [`declared_keys`] over the
    /// contract's *sibling* clauses — every `required`/`optional` row's top-level key —
    /// so a kind's key set is declared once and never authored a second time beside
    /// itself, where the two copies could disagree.
    ClosedKeys,
    /// `allowed_chars`: every character of the field's value is permitted by the
    /// declared [`Charset`] — the in-crate stand-in for the `[a-z0-9-]` case,
    /// short of the full `pattern` (regex) primitive.
    AllowedChars {
        /// The field constrained.
        field: String,
        /// The permitted character set.
        charset: Charset,
    },
    /// `shape`: the field's value holds the named [`Shape`] — one member of a closed set
    /// of engine-implemented forms, each backed by an external document that states it.
    ///
    /// The author names a shape and can spell nothing else. Its mechanics are a regex the
    /// engine owns ([`Shape::pattern`]), hidden exactly as [`Charset`] hides the
    /// character class it stands for: naming a shape is not authoring a pattern, and the
    /// closure is what keeps the two apart. Growing the set is a deliberate language
    /// change — a new member ships with its implementation and its cite, or it does not
    /// ship — so this door cannot widen into an open lint.
    Shape {
        /// The field constrained.
        field: String,
        /// The declared shape its value must hold.
        shape: Shape,
    },
    /// `extent`: the selected item's **rendered** extent is at most `max`, measured in
    /// the declared [`ExtentUnit`]. Node-scope (names no field, like `max_lines` did),
    /// and the one predicate that carries its own grain: each-grain (`whole` false)
    /// bounds every selected item's own rendered extent; whole-grain (`whole` true)
    /// bounds the selection's summed rendered extent, judged over the set by
    /// [`crate::engine::judge`].
    ///
    /// Measurement is render-side — the bytes the item contributes to its projection, off
    /// [`Features::rendered_extent`](crate::extract::Features::rendered_extent), never the
    /// `LineCount` primitive's source-side `body_lines`.
    Extent {
        /// The unit the bound is measured in — lines or characters.
        unit: ExtentUnit,
        /// The inclusive upper bound, in `unit`s.
        max: usize,
        /// Whether the bound ranges over the whole selection (summed extent) rather than
        /// each selected item on its own.
        whole: bool,
    },
    /// `require_sections`: each named heading is present in the body.
    RequireSections {
        /// The headings that must appear.
        sections: Vec<String>,
    },
    /// `must_define`: the named field/marker exists (e.g.
    /// `disable-model-invocation`).
    MustDefine {
        /// The marker that must be defined.
        marker: String,
    },
    /// `section_contains`: every body section whose heading *starts with* the
    /// declared `heading` carries the declared `marker` (a substring of the section
    /// body) — e.g. "every `## Decision` section carries a `Rejected` marker".
    /// Decidable over the extracted [`sections`](crate::extract::Features::sections).
    SectionContains {
        /// The heading-text prefix that selects the sections this clause governs.
        heading: String,
        /// The marker text every governed section's body must contain.
        marker: String,
    },
    /// `name-matches-dir`: the artifact's name equals its containing directory.
    NameMatchesDir,
    /// `unique-name`: names are unique within the artifact kind.
    UniqueName,
    /// `dependency-exists`: every declared dependency resolves. **Held back** — like
    /// the full `pattern` (regex) primitive: named by the vocabulary so it parses, but
    /// inadmissible until it declares a decidable reference syntax *and* an extractor.
    /// Without one the engine could only return *indeterminate* — a silent no-op
    /// — so a hand-authored clause fails admissibility
    /// ([`crate::engine::admissibility`]) rather than acting as a working clause.
    DependencyExists,
    /// `count`: the **whole** grain — the selection's size lies within the inclusive
    /// `[min, max]` bound. An inverted `min > max` bound admits nothing and is rejected
    /// at admissibility.
    Count {
        /// The inclusive lower bound on the selection's size.
        min: usize,
        /// The inclusive upper bound on the selection's size.
        max: usize,
    },
    /// `unique`: the **whole** grain — the named field's extracted value does not repeat
    /// across the selection.
    Unique {
        /// The field checked for uniqueness across the selection.
        field: String,
    },
    /// `membership`: the **whole** grain — every selected member's `field` value is
    /// drawn from a feature over the selection the named `target` requirement declares.
    /// Shaping that set is the target's own job, so this predicate names it, never
    /// re-derives it. Its arg key is `target`, not `source` — the clause's own
    /// [`Clause::source`] citation already owns that key.
    Membership {
        /// The field checked on every member of this clause's own selection.
        field: String,
        /// The name of the requirement whose selection supplies the allowed values.
        target: String,
    },
    /// `degree`: the **each** grain over the selection, whole-grain over each member's
    /// own by-incidence selection — the in/out edge-count bound every selected member
    /// must land in over the one relation graph. At least one direction must be bounded —
    /// an empty `degree` constrains nothing and is rejected at admissibility.
    Degree {
        /// The bound on a member's incoming edge count, when constrained.
        incoming: Option<EdgeBound>,
        /// The bound on a member's outgoing edge count, when constrained.
        outgoing: Option<EdgeBound>,
    },
    /// `kind`: the **each** grain — every member of the selection is of the declared
    /// artifact kind. This is how a selection narrows: a member of a different kind is a
    /// finding, never a silent exclusion from the set a `count`/`unique`/`membership`
    /// clause ranges over. An empty `kind` names nothing to match and is rejected at
    /// admissibility.
    Kind {
        /// The kind every member of the selection must be.
        kind: String,
    },
    /// `glob-valid`: every glob the named field carries parses under `globset`
    /// (brace-expansion aware — the one glob engine already inside `ignore`). An
    /// unparseable pattern (an unclosed `[`) is invalid and silently matches
    /// nothing, so the scope it declares is dead: the rule never loads there, the
    /// skill never registers, with no error surfaced. This turns that silent dead
    /// scope into a finding.
    GlobValid {
        /// The field whose every glob must parse.
        field: String,
    },
    /// `mention-reachable`: the **each** grain over the selection's members — every
    /// mention a selected member authors must be reachable where it fires. A target
    /// whose `gate_field` carries globs is *gated*: it is removed from every invocation
    /// channel until the agent reads a file the gate matches, so a mention of it fires
    /// only where the gate is open. Two diagnoses, one invariant: a **scoped** source
    /// whose `scope_field` globs are not contained in the target's gate can fire where
    /// the target cannot be invoked; an **unscoped** source mentioning a gated target is
    /// actionable only inside that gate.
    ///
    /// The predicate is generic over both ends and hard-codes no kind — the source's
    /// scope field and the target's gate field are arguments, the two-argument sibling
    /// of [`Predicate::GlobValid`]'s one field. The **trigger is the target's declared
    /// gate field carrying a non-empty value**, never its kind or its registration set:
    /// a gate is a field a kind documents, and a kind may gate without declaring a
    /// `paths-match` registration channel (a skill's `paths` is exactly that —
    /// `sdk/src/builtins.ts`).
    ///
    /// **Declared leniency:** containment is *literal* — every source glob must appear
    /// verbatim in the target's gate set — because true glob-set containment is
    /// undecidable. It therefore false-fires on a semantically contained narrower glob
    /// (`src/**/*.ts` inside `src/**`), which is why a clause naming it ships at
    /// advisory severity: a check that can be wrong must not block
    /// (`specs/decisions/0028-a-mention-must-be-reachable-where-it-fires.md`).
    ///
    /// Judged by [`crate::graph::mention_reachable`], not the per-member table: the
    /// verdict needs the mention graph and the *target* member's features, neither of
    /// which the source member carries.
    MentionReachable {
        /// The source member's field carrying the scope globs the mention fires under.
        scope_field: String,
        /// The **target** member's field carrying the gate globs that must contain the
        /// source's scope.
        gate_field: String,
    },
    /// `format-places-edges`: the edge scope, at the **each** grain — the selection is
    /// the edges incident on the member, and every one of them must be placed by the
    /// format that renders the member. A format that omits an edge its kind declares
    /// renders a contract the prose does not represent.
    ///
    /// Carries no argument: the selection is every edge the member's kind declares, and
    /// the grain is already `each`. Decidable over
    /// [`edge_placements`](crate::extract::Features::edge_placements) — `emit` observes
    /// which edges the format selected and lowers the observation into a declaration
    /// row, because the engine never sees a `render` hook and never reads a projection
    /// back.
    FormatPlacesEdges,
}

/// Lift one clause row's `charset` column into the typed [`Charset`] — `None`
/// when a range spec is not the `<lo>-<hi>` shape `emit` always writes.
fn charset_from_row(row: &CharsetRow) -> Option<Charset> {
    let mut ranges = Vec::with_capacity(row.ranges.len());
    for spec in &row.ranges {
        match spec.chars().collect::<Vec<char>>().as_slice() {
            [lo, '-', hi] => ranges.push((*lo, *hi)),
            _ => return None,
        }
    }
    let chars = row.chars.as_deref().unwrap_or_default().chars().collect();
    Some(Charset { ranges, chars })
}

/// Lift one lock `ClauseRow` — whichever family sourced it (a kind's own floor
/// row, `crate::builtin`; a requirement's nested set-/edge-scope row,
/// `crate::main`) — into its typed [`Predicate`], the full argument payload
/// (`bound`/`charset`/`keys`/`values`/`count`/`target`/`degree`) alongside
/// `field`. `None` for a predicate key or argument shape this projection
/// carries no column for. Decodes only the predicate: severity, guidance, and
/// cite are each call site's own assembly, not this function's job. `pub`
/// (not `pub(crate)`) so the `main` binary's lift of a requirement's own
/// clause rows reuses the identical decoder, never a second copy.
pub fn predicate_from_row(row: &ClauseRow) -> Option<Predicate> {
    Some(match row.predicate.as_str() {
        "required" => Predicate::Required {
            field: row.field.clone()?,
        },
        "optional" => Predicate::Optional {
            field: row.field.clone()?,
        },
        // The declared kinds cross the lock as their lattice names; an unknown name is
        // no predicate at all, so the row is rejected at load rather than decided
        // against a guessed type. An empty set decodes fine and fails admissibility
        // instead — a vacuous clause is the engine's refusal to make, not the
        // decoder's.
        "type" => Predicate::Type {
            field: row.field.clone()?,
            kinds: row
                .value_type
                .as_ref()?
                .iter()
                .map(|name| ValueType::from_name(name))
                .collect::<Option<BTreeSet<ValueType>>>()?,
        },
        "range" => {
            let bound = row.range?;
            Predicate::Range {
                field: row.field.clone()?,
                min: bound.min,
                max: bound.max,
            }
        }
        "enum" => Predicate::Enum {
            field: row.field.clone()?,
            values: row.values.clone()?,
        },
        "must_define" => Predicate::MustDefine {
            marker: row.field.clone()?,
        },
        "section_contains" => {
            let section = row.section.as_ref()?;
            Predicate::SectionContains {
                heading: section.heading.clone(),
                marker: section.marker.clone(),
            }
        }
        "min_len" => Predicate::MinLen {
            field: row.field.clone()?,
            min: row.bound?.min?,
        },
        "max_len" => Predicate::MaxLen {
            field: row.field.clone()?,
            max: row.bound?.max?,
        },
        // An unknown unit is no unit at all: `from_name` returns `None`, the row is
        // rejected at load rather than lifted into a clause measuring nothing. A lock
        // still carrying the retired `max_lines` predicate falls through to the closed
        // vocabulary's ordinary unknown-predicate reject.
        "extent" => Predicate::Extent {
            unit: ExtentUnit::from_name(row.unit.as_deref()?)?,
            max: row.bound?.max?,
            whole: false,
        },
        "allowed_chars" => Predicate::AllowedChars {
            field: row.field.clone()?,
            charset: charset_from_row(row.charset.as_ref()?)?,
        },
        // The same rule the declared kinds cross under: the shape crosses the lock as its
        // name, and a name outside the closed set is no predicate at all — the row is
        // rejected at load rather than skipped into a contract that silently checks less
        // than it says.
        "shape" => Predicate::Shape {
            field: row.field.clone()?,
            shape: Shape::from_name(row.shape.as_deref()?)?,
        },
        "forbidden_keys" => Predicate::ForbiddenKeys {
            keys: row.keys.clone()?,
        },
        // No argument column to decode: the allow-list is the kind's own sibling rows.
        "closed-keys" => Predicate::ClosedKeys,
        "deny" => Predicate::Deny {
            field: row.field.clone()?,
            values: row.values.clone()?,
        },
        "name-matches-dir" => Predicate::NameMatchesDir,
        "unique-name" => Predicate::UniqueName,
        "count" => {
            let bound = row.count?;
            Predicate::Count {
                min: bound.min,
                max: bound.max,
            }
        }
        "unique" => Predicate::Unique {
            field: row.field.clone()?,
        },
        "glob-valid" => Predicate::GlobValid {
            field: row.field.clone()?,
        },
        // The one two-argument predicate: the source's scope rides the shared `field`
        // column, the target's gate its own `gate` column — one `field` cannot carry
        // both ends.
        "mention-reachable" => Predicate::MentionReachable {
            scope_field: row.field.clone()?,
            gate_field: row.gate.clone()?,
        },
        "format-places-edges" => Predicate::FormatPlacesEdges,
        "membership" => Predicate::Membership {
            field: row.field.clone()?,
            target: row.target.clone()?,
        },
        "degree" => {
            let bound = row.degree.as_ref()?;
            Predicate::Degree {
                incoming: bound.incoming.map(|edge| EdgeBound {
                    min: edge.min,
                    max: edge.max,
                }),
                outgoing: bound.outgoing.map(|edge| EdgeBound {
                    min: edge.min,
                    max: edge.max,
                }),
            }
        }
        _ => return None,
    })
}

/// One direction's inclusive `[min, max]` edge-count bound for [`Predicate::Degree`],
/// each endpoint optional: absent `min` ⇒ no lower bound, absent `max` ⇒ unbounded
/// above (the routed "≥ 1" idiom is `min: Some(1), max: None`;
/// self-registering / routed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeBound {
    /// The inclusive lower bound. `None` ⇒ no lower bound.
    pub min: Option<usize>,
    /// The inclusive upper bound. `None` ⇒ unbounded above.
    pub max: Option<usize>,
}

impl EdgeBound {
    /// Whether `degree` lands inside this inclusive bound — `min <= degree <= max`
    /// with an absent endpoint imposing no limit on that side. The decidable core of
    /// the graph-scope `degree` check ([`crate::graph::degree`]).
    #[must_use]
    pub fn admits(self, degree: usize) -> bool {
        self.min.is_none_or(|min| degree >= min) && self.max.is_none_or(|max| degree <= max)
    }
}

impl Predicate {
    /// This predicate's clause key — the lock `ClauseRow`'s `predicate`
    /// discriminator, reused verbatim as the diagnostic `rule` id a finding reports
    /// under (`crate::engine`).
    #[must_use]
    pub fn key(&self) -> &'static str {
        match self {
            Predicate::Required { .. } => "required",
            Predicate::Optional { .. } => "optional",
            Predicate::Type { .. } => "type",
            Predicate::MinLen { .. } => "min_len",
            Predicate::MaxLen { .. } => "max_len",
            Predicate::Range { .. } => "range",
            Predicate::Enum { .. } => "enum",
            Predicate::Deny { .. } => "deny",
            Predicate::ForbiddenKeys { .. } => "forbidden_keys",
            Predicate::ClosedKeys => "closed-keys",
            Predicate::AllowedChars { .. } => "allowed_chars",
            Predicate::Shape { .. } => "shape",
            Predicate::Extent { .. } => "extent",
            Predicate::RequireSections { .. } => "require_sections",
            Predicate::MustDefine { .. } => "must_define",
            Predicate::SectionContains { .. } => "section_contains",
            Predicate::NameMatchesDir => "name-matches-dir",
            Predicate::UniqueName => "unique-name",
            Predicate::DependencyExists => "dependency-exists",
            Predicate::Count { .. } => "count",
            Predicate::Unique { .. } => "unique",
            Predicate::Membership { .. } => "membership",
            Predicate::Degree { .. } => "degree",
            Predicate::Kind { .. } => "kind",
            Predicate::GlobValid { .. } => "glob-valid",
            Predicate::MentionReachable { .. } => "mention-reachable",
            Predicate::FormatPlacesEdges => "format-places-edges",
        }
    }

    /// Whether this predicate ranges over the **selection** a clause binds to rather
    /// than one member's own features — `count`/`unique`/`membership` at the whole
    /// grain, `degree`/`kind`/`mention-reachable` at the each grain. Judged by
    /// [`crate::engine::judge`], [`crate::graph::degree`], and
    /// [`crate::graph::mention_reachable`] over the resolved
    /// selection; every other predicate is judged by [`crate::engine::validate`] over a
    /// member.
    ///
    /// The line is the *feature read*, not the grain: `mention-reachable` is each-grain
    /// over the members, but each verdict reads the mention graph and the target
    /// member's own gate field, so it belongs to the selection judges exactly as
    /// `degree` does. `extent` is the one predicate that ranges over the selection *or*
    /// the member depending on its own `whole` flag — a whole-grain budget sums the set,
    /// an each-grain budget reads one member.
    #[must_use]
    pub fn ranges_over_selection(&self) -> bool {
        matches!(
            self,
            Predicate::Count { .. }
                | Predicate::Unique { .. }
                | Predicate::Membership { .. }
                | Predicate::Degree { .. }
                | Predicate::Kind { .. }
                | Predicate::MentionReachable { .. }
                | Predicate::Extent { whole: true, .. }
        )
    }

    /// The field (or marker) this predicate constrains, or `None` for the
    /// artifact- and cross-artifact-level predicates that name no single field
    /// (`forbidden_keys`, `extent`, `require_sections`, `name-matches-dir`,
    /// `unique-name`, `dependency-exists`).
    #[must_use]
    pub fn target(&self) -> Option<&str> {
        match self {
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
            Predicate::MustDefine { marker } => Some(marker),
            // A `section_contains` constrains the content under one heading, so the
            // heading is the field it names.
            Predicate::SectionContains { heading, .. } => Some(heading),
            Predicate::Unique { field } | Predicate::Membership { field, .. } => Some(field),
            Predicate::ForbiddenKeys { .. }
            | Predicate::ClosedKeys
            | Predicate::Extent { .. }
            | Predicate::RequireSections { .. }
            | Predicate::NameMatchesDir
            | Predicate::UniqueName
            | Predicate::DependencyExists
            | Predicate::Count { .. }
            | Predicate::Degree { .. }
            | Predicate::Kind { .. }
            // Two field arguments, so no *one* field is "the" field it constrains —
            // the set predicates' silence here is the precedent.
            | Predicate::MentionReachable { .. }
            | Predicate::FormatPlacesEdges => None,
        }
    }

    /// The **frontmatter field** this predicate documents — the property a clause's
    /// [`guidance`](Clause::guidance) rides as a JSON Schema `description` in the
    /// emitted schema's docs channel. `Some` for the per-field frontmatter predicates whose property
    /// can carry hover docs; `None` for the body/structural and cross-artifact
    /// predicates that name no frontmatter property. Distinct from
    /// [`Predicate::target`] in one place: a `must_define` marker is a *body*
    /// marker, not a frontmatter field, so it documents no property here even though
    /// `target` names it.
    #[must_use]
    pub fn documented_field(&self) -> Option<&str> {
        match self {
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
            // The set predicates range over a selection, not a single member's
            // frontmatter — they document no schema property here even when
            // [`Predicate::target`] names a field.
            Predicate::MustDefine { .. }
            | Predicate::ForbiddenKeys { .. }
            | Predicate::ClosedKeys
            | Predicate::Extent { .. }
            | Predicate::RequireSections { .. }
            | Predicate::SectionContains { .. }
            | Predicate::NameMatchesDir
            | Predicate::UniqueName
            | Predicate::DependencyExists
            | Predicate::Count { .. }
            | Predicate::Unique { .. }
            | Predicate::Membership { .. }
            | Predicate::Degree { .. }
            | Predicate::Kind { .. }
            // Its scope field is a real frontmatter property, but the clause's verdict
            // is about the *target*'s gate, not this property's value — guidance about
            // the pair belongs to neither property's hover docs alone.
            | Predicate::MentionReachable { .. }
            | Predicate::FormatPlacesEdges => None,
        }
    }
}

/// The top-level keys `clauses` declare — the allow-list a [`Predicate::ClosedKeys`]
/// clause consumes, read off its own siblings rather than authored a second time.
///
/// A key is declared by a `required` or `optional` row, the two rows that say a key is
/// part of the kind's schema; every other predicate refines a value whose key one of them
/// already names. What a path declares is its **head segment**: `required("owner.name")`
/// says the member carries an `owner` key, never that it carries an `owner.name` one.
///
/// An out-of-subset path declares nothing here — [`crate::engine::admissibility`] refuses
/// the clause that spelled it, so no contract reaching evaluation carries one.
#[must_use]
pub fn declared_keys(clauses: &[Clause]) -> BTreeSet<String> {
    clauses
        .iter()
        .filter_map(|clause| match &clause.predicate {
            Predicate::Required { field } | Predicate::Optional { field } => Some(field),
            _ => None,
        })
        .filter_map(|field| crate::address::FieldPath::parse(field).ok())
        .filter_map(|path| path.head_name().map(str::to_string))
        .collect()
}

/// The in-crate character set for [`Predicate::AllowedChars`]. A character is
/// permitted iff it falls within one of `ranges` or appears in `chars`. This is
/// the deliberately weak, decidable substitute for a regex character class — it
/// expresses `[a-z0-9-]` (as `ranges = ["a-z", "0-9"]`, `chars = "-"`) without
/// admitting the full `pattern` primitive held behind the regex fork.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Charset {
    /// Inclusive character ranges, e.g. `('a', 'z')`.
    pub ranges: Vec<(char, char)>,
    /// Individually permitted characters, e.g. `-`.
    pub chars: BTreeSet<char>,
}

impl Charset {
    /// Whether `c` is permitted by this charset.
    #[must_use]
    pub fn allows(&self, c: char) -> bool {
        self.chars.contains(&c) || self.ranges.iter().any(|&(lo, hi)| (lo..=hi).contains(&c))
    }
}

/// The closed set of value forms a [`Predicate::Shape`] clause may name.
///
/// A member exists only where an external document states the rule and this engine
/// implements it, so the set is closed by construction and an author extends it with
/// nothing: the enum *is* the vocabulary, and [`from_name`](Shape::from_name) rejects
/// anything outside it at load. That closure is the whole safety property — it is what
/// separates a shape from an open lint, which no clause may become.
///
/// Each member's mechanics are one regex plus a polarity: [`pattern`](Shape::pattern) is
/// the expression, [`match_holds`](Shape::match_holds) says whether matching it means the
/// shape holds or is violated. One definition serves both the gate and the emitted
/// schema, so the two channels cannot decide the same shape differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ts_rs::TS)]
#[ts(rename_all = "kebab-case")]
pub enum Shape {
    /// `hyphen-placement`: a hyphen neither leads, trails, nor doubles — the value is
    /// non-empty segments joined by single hyphens (`pdf-processing`, never `-pdf`,
    /// `pdf-`, or `pdf--processing`). Says nothing about the alphabet: an
    /// [`AllowedChars`](Predicate::AllowedChars) clause is where a value's character set
    /// is declared, and the two compose on one field without either restating the other.
    ///
    /// The empty value holds it — there is no hyphen to misplace — because emptiness is
    /// [`MinLen`](Predicate::MinLen)'s to indict, and a shape that fired there would
    /// duplicate that clause's finding under a worse name.
    HyphenPlacement,
    /// `no-xml-tags`: the value carries no XML tag.
    ///
    /// **Declared leniency:** a *tag* is read as well-formed — an element name, then
    /// quoted attributes if any — so `<br/>`, `</note>`, and `<a href="x">` are tags
    /// while prose like `use when x < y and y > z` is not. A malformed near-tag
    /// therefore passes. That direction is deliberate: the reading is the narrow one, so
    /// every finding is a true positive and the clause can gate, where a loose reading
    /// would fire on ordinary prose that spells a comparison.
    NoXmlTags,
}

/// `hyphen-placement`'s expression: optional non-empty segments joined by single
/// hyphens. Anchored, so it decides the whole value.
const HYPHEN_PLACEMENT_PATTERN: &str = r"^([^-]+(-[^-]+)*)?$";

/// `no-xml-tags`' expression: one well-formed start, end, or empty-element tag —
/// `<name>`, `</name>`, `<name/>`, `<name attr="v">`. Unanchored: a tag anywhere in the
/// value is the violation.
const NO_XML_TAGS_PATTERN: &str = r#"</?[A-Za-z_:][-A-Za-z0-9._:]*(\s+[A-Za-z_:][-A-Za-z0-9._:]*\s*=\s*("[^"]*"|'[^']*'))*\s*/?>"#;

/// Every shape's compiled expression, built once. Compilation cannot fail — the patterns
/// are crate constants, covered by [`crate::contract::tests`] — so the shape's own judge
/// never reaches for a fallible path at check time.
static SHAPE_PATTERNS: std::sync::LazyLock<[(Shape, regex::Regex); 2]> =
    std::sync::LazyLock::new(|| {
        [
            (
                Shape::HyphenPlacement,
                regex::Regex::new(HYPHEN_PLACEMENT_PATTERN)
                    .expect("HYPHEN_PLACEMENT_PATTERN is a valid regex"),
            ),
            (
                Shape::NoXmlTags,
                regex::Regex::new(NO_XML_TAGS_PATTERN)
                    .expect("NO_XML_TAGS_PATTERN is a valid regex"),
            ),
        ]
    });

impl Shape {
    /// This shape's declared name — the spelling the lock's `shape` column carries and
    /// the author names in the SDK.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Shape::HyphenPlacement => "hyphen-placement",
            Shape::NoXmlTags => "no-xml-tags",
        }
    }

    /// Parse a declared shape name into its [`Shape`], or `None` when it names no member
    /// of the closed set. The single home of the shape name table.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Shape> {
        match name {
            "hyphen-placement" => Some(Shape::HyphenPlacement),
            "no-xml-tags" => Some(Shape::NoXmlTags),
            _ => None,
        }
    }

    /// The regex this shape is decided by, paired with [`match_holds`](Shape::match_holds).
    /// The engine judges by it and [`crate::schema`] projects it, so the gate and the
    /// keystroke check one expression rather than two that could drift.
    #[must_use]
    pub fn pattern(self) -> &'static str {
        match self {
            Shape::HyphenPlacement => HYPHEN_PLACEMENT_PATTERN,
            Shape::NoXmlTags => NO_XML_TAGS_PATTERN,
        }
    }

    /// Whether matching [`pattern`](Shape::pattern) means the shape **holds** (`true`) or
    /// is **violated** (`false`) — a shape stating what a value must be reads positively,
    /// one stating what it must not carry reads negatively, and the regex crate spells no
    /// lookaround to collapse the two.
    #[must_use]
    pub fn match_holds(self) -> bool {
        match self {
            Shape::HyphenPlacement => true,
            Shape::NoXmlTags => false,
        }
    }

    /// Whether `value` holds this shape.
    #[must_use]
    pub fn admits(self, value: &str) -> bool {
        let matched = SHAPE_PATTERNS
            .iter()
            .find(|(shape, _)| *shape == self)
            .is_some_and(|(_, pattern)| pattern.is_match(value));
        matched == self.match_holds()
    }

    /// What this shape demands, in the prose a finding quotes — the teaching a clause's
    /// own guidance then widens on.
    #[must_use]
    pub fn demand(self) -> &'static str {
        match self {
            Shape::HyphenPlacement => {
                "a hyphen may not lead, trail, or double — the value is segments joined \
                 by single hyphens"
            }
            Shape::NoXmlTags => "the value carries no XML tag",
        }
    }
}

/// The unit an [`Extent`](Predicate::Extent) bound is measured in — the closed set of
/// stable render-side size proxies. Token count is deliberately absent: it moves when a
/// tokenizer or model updates, and a verdict that changes with no diff is not a gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtentUnit {
    /// The rendered line count.
    Lines,
    /// The rendered character count.
    Characters,
}

impl ExtentUnit {
    /// This unit's declared name — the spelling the lock's `unit` column carries and the
    /// author names in the SDK. The single home of the unit name table.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            ExtentUnit::Lines => "lines",
            ExtentUnit::Characters => "characters",
        }
    }

    /// Parse a declared unit name into its [`ExtentUnit`], or `None` when it names no
    /// member of the closed set — the admissibility of an author's unit, refused at load.
    #[must_use]
    pub fn from_name(name: &str) -> Option<ExtentUnit> {
        match name {
            "lines" => Some(ExtentUnit::Lines),
            "characters" => Some(ExtentUnit::Characters),
            _ => None,
        }
    }
}
