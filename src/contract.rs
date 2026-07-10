//! The `Contract` artifact — the decidable artifact-clause algebra.
//!
//! A [`Contract`] is a named set of [`Clause`]s over a **closed** vocabulary of
//! decidable predicates, each carrying an author-declared [`Severity`]. Every live
//! `Contract` is built from the lock's `ClauseRow` family ([`crate::builtin`],
//! `crate::compose::effective`) — there is no hand-authored clause grammar; the
//! SDK's types are the only authoring spelling.
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
/// Not `Eq`: [`Predicate::Range`] carries `f64` bounds (only `PartialEq`).
#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
    /// `required`: the named field must be present.
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
    /// `type`: the field's preserved source kind is the declared [`ValueType`] over
    /// the closed scalar/container lattice (`string`/`integer`/`number`/
    /// `boolean`/`list`/`map`/`null`). Unlike `min_len`/`enum`/`pattern`, which
    /// refine *within* a scalar type, `type` only fixes the kind.
    Type {
        /// The field constrained.
        field: String,
        /// The declared source kind the field must carry.
        kind: ValueType,
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
    /// `allowed_chars`: every character of the field's value is permitted by the
    /// declared [`Charset`] — the in-crate stand-in for the `[a-z0-9-]` case,
    /// short of the full `pattern` (regex) primitive.
    AllowedChars {
        /// The field constrained.
        field: String,
        /// The permitted character set.
        charset: Charset,
    },
    /// `max_lines`: the artifact body is at most `max` lines.
    MaxLines {
        /// The inclusive upper bound, in lines.
        max: usize,
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
    /// `count`: the node-set scope — the satisfier set's size lies within the
    /// inclusive `[min, max]` bound. An inverted `min > max` bound admits nothing and is
    /// rejected at admissibility.
    Count {
        /// The inclusive lower bound on the set's size.
        min: usize,
        /// The inclusive upper bound on the set's size.
        max: usize,
    },
    /// `unique`: the node-set scope — the named field's extracted value does not
    /// repeat across the set.
    Unique {
        /// The field checked for uniqueness across the set.
        field: String,
    },
    /// `membership`: the node-set scope — every satisfier's `field` value is drawn
    /// from a feature over the named `target` requirement's own satisfier set.
    /// Shaping that set is the target requirement's own job,
    /// so this
    /// predicate names it, never re-derives it. Its arg key is `target`, not `source`
    /// — the clause's own [`Clause::source`] citation already owns that key.
    Membership {
        /// The field checked on every satisfier of this clause's own set.
        field: String,
        /// The name of the requirement whose satisfier set supplies the allowed values.
        target: String,
    },
    /// `degree`: the edge scope — the in/out edge-count bound every satisfier must
    /// land in over the one relation graph. At least one direction must be bounded — an empty `degree`
    /// constrains nothing and is rejected at admissibility.
    Degree {
        /// The bound on a satisfier's incoming edge count, when constrained.
        incoming: Option<EdgeBound>,
        /// The bound on a satisfier's outgoing edge count, when constrained.
        outgoing: Option<EdgeBound>,
    },
    /// `kind`: the node-set scope, at the **each** grain — every satisfier in the
    /// selection is of the declared artifact kind. A satisfier of a different kind is a finding, never a
    /// silent exclusion from the set a `count`/`unique`/`membership` clause ranges
    /// over. An empty `kind` names nothing to match and is rejected at
    /// admissibility.
    Kind {
        /// The kind every satisfier in the selection must be.
        kind: String,
    },
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
        "max_lines" => Predicate::MaxLines {
            max: row.bound?.max?,
        },
        "allowed_chars" => Predicate::AllowedChars {
            field: row.field.clone()?,
            charset: charset_from_row(row.charset.as_ref()?)?,
        },
        "forbidden_keys" => Predicate::ForbiddenKeys {
            keys: row.keys.clone()?,
        },
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
    /// under (`crate::engine`). It is also half a clause's *layering identity*
    /// (`crate::compose`): the key plus [`Predicate::target`].
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
            Predicate::AllowedChars { .. } => "allowed_chars",
            Predicate::MaxLines { .. } => "max_lines",
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
        }
    }

    /// The field (or marker) this predicate constrains, or `None` for the
    /// artifact- and cross-artifact-level predicates that name no single field
    /// (`forbidden_keys`, `max_lines`, `require_sections`, `name-matches-dir`,
    /// `unique-name`, `dependency-exists`). With [`Predicate::key`] it identifies
    /// a clause for layering (`crate::compose`): a layered clause sharing both
    /// *overrides* the floor clause (a severity flip or parameter change), while a
    /// new (key, target) pair *extends* the floor with a fresh clause.
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
            | Predicate::AllowedChars { field, .. } => Some(field),
            Predicate::MustDefine { marker } => Some(marker),
            // The section heading is the layering identity: a layered
            // `section_contains` on the same heading overrides the floor's (a
            // severity flip or a changed marker), while a fresh heading extends it.
            Predicate::SectionContains { heading, .. } => Some(heading),
            // `unique`/`membership` both check one field per satisfier, so the
            // checked field is their layering identity too.
            Predicate::Unique { field } | Predicate::Membership { field, .. } => Some(field),
            Predicate::ForbiddenKeys { .. }
            | Predicate::MaxLines { .. }
            | Predicate::RequireSections { .. }
            | Predicate::NameMatchesDir
            | Predicate::UniqueName
            | Predicate::DependencyExists
            | Predicate::Count { .. }
            | Predicate::Degree { .. }
            | Predicate::Kind { .. } => None,
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
            | Predicate::AllowedChars { field, .. } => Some(field),
            // The node-set/edge-scope predicates range over a satisfier set, not a
            // single kind's frontmatter — they document no schema property here even
            // when `target` (above) names a field for layering purposes.
            Predicate::MustDefine { .. }
            | Predicate::ForbiddenKeys { .. }
            | Predicate::MaxLines { .. }
            | Predicate::RequireSections { .. }
            | Predicate::SectionContains { .. }
            | Predicate::NameMatchesDir
            | Predicate::UniqueName
            | Predicate::DependencyExists
            | Predicate::Count { .. }
            | Predicate::Unique { .. }
            | Predicate::Membership { .. }
            | Predicate::Degree { .. }
            | Predicate::Kind { .. } => None,
        }
    }
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
