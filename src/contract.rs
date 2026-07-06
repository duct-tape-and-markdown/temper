//! The `Contract` artifact — the decidable artifact-clause algebra
//! (`specs/architecture/10-contracts.md`, "The primitive algebra (decidable only)").
//!
//! A [`Contract`] is a named set of [`Clause`]s over a **closed** vocabulary of
//! decidable predicates, each carrying an author-declared [`Severity`]. Its authored
//! home is the embedded built-in floor ([`crate::builtin`]), with per-clause severity
//! overrides riding the lock's `ClauseRow` family (`crate::compose::effective`); both
//! parse through the same clause parser [`Contract::parse`] runs over bare TOML.
//!
//! There is no arbitrary-code clause: adding a predicate is a deliberate language
//! change, never a per-contract escape hatch (`00-intent.md` law 3), so loading
//! **rejects** an unknown predicate rather than skipping it silently.
//!
//! Parsing hand-walks `toml_edit` (mirroring [`crate::frontmatter`]) instead of deriving
//! `serde`: the diagnostics *are* the product, and a precise "clause 3 names unknown
//! predicate `word_count`" beats the generic decode error a tagged-enum deserializer
//! would give.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use toml_edit::{DocumentMut, Item, Table};

use crate::extract::Kind;

/// A named set of clauses over the decidable primitive algebra — the type a
/// harness (or one artifact in it) is checked against.
///
/// Not `Eq`: the `range` predicate carries `f64` bounds (`specs/architecture/45-governance.md`),
/// and `f64` is only `PartialEq`. Equality is still derived (the tests compare
/// whole contracts), just not the reflexive marker.
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    /// Display label for diagnostics — an explicit top-level `name` if present,
    /// else the file stem. A contract's *identity* is its path/role, not this
    /// field (specs/architecture/10-contracts.md), so `name` is never a required input.
    pub name: String,
    /// The clauses, in declaration order. An empty set is a valid (vacuous)
    /// contract — a named shape that asserts nothing.
    pub clauses: Vec<Clause>,
    /// Package-level **guidance** (`specs/architecture/10-contracts.md`, "Packages"):
    /// best-practice prose the clauses cannot encode. Like the per-clause
    /// [`guidance`](Clause::guidance) channel it *never gates* — the closed algebra
    /// has no path from prose to a predicate. `None` for a bare TOML contract, or a
    /// package that authors none.
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
    /// (`specs/architecture/10-contracts.md`, "Templates") kept *out of checks*: it plays no part
    /// in conformance or admissibility. It rides its JSON Schema property's
    /// `description` in the emitted schema (`specs/architecture/50-distribution.md`, "The gate at
    /// keystroke"), never a validation keyword — taste becomes documentation, never a
    /// squiggle. Absent ⇒ the clause documents nothing.
    pub guidance: Option<String>,
    /// Optional **source** citation — the clause's provenance of taste, a URL plus
    /// retrieval date (`specs/architecture/10-contracts.md`, "Decision: a built-in package is
    /// named for its source, and cited to it"). *Preserved metadata*, not a
    /// predicate: no gate reads its content, so admitting it neither adds nor relaxes
    /// any check. Absent ⇒ the clause is uncited (every clause on disk today).
    pub source: Option<String>,
}

/// The author-declared weight of a clause. Replaces the tool-baked error/warn
/// split: the default gate blocks on `Required` clauses only, and a strict CI
/// policy can promote `Advisory` to blocking (`specs/architecture/10-contracts.md`,
/// "Severity is declared, not baked").
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
    /// `type`: the field's preserved source kind is the declared [`Kind`] over
    /// the closed scalar/container lattice (`string`/`integer`/`number`/
    /// `boolean`/`list`/`map`/`null`). Unlike `min_len`/`enum`/`pattern`, which
    /// refine *within* a scalar type, `type` only fixes the kind.
    Type {
        /// The field constrained.
        field: String,
        /// The declared source kind the field must carry.
        kind: Kind,
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
    /// bound over `integer`/`number` fields (`specs/architecture/45-governance.md`, "Also in
    /// scope"). Bounds are `f64` so a single predicate spans both integer and
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
    /// body) — e.g. "every `## Decision` section carries a `Rejected` marker"
    /// (`specs/architecture/10-contracts.md`, "The primitive algebra"; `specs/architecture/15-kinds.md`).
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
    /// Without one the engine could only return *indeterminate* — a silent no-op law 1
    /// forbids — so a hand-authored clause fails admissibility
    /// ([`crate::engine::admissibility`]) rather than acting as a working clause.
    DependencyExists,
    /// `count`: the node-set scope — the satisfier set's size lies within the
    /// inclusive `[min, max]` bound (`specs/architecture/10-contracts.md`, "Judged at the
    /// node-set scope"). An inverted `min > max` bound admits nothing and is
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
    /// Shaping that set is the target requirement's own job
    /// (`specs/architecture/10-contracts.md`, "Judged at the node-set scope"), so this
    /// predicate names it, never re-derives it. Its arg key is `target`, not `source`
    /// — the clause's own [`Clause::source`] citation already owns that key.
    Membership {
        /// The field checked on every satisfier of this clause's own set.
        field: String,
        /// The name of the requirement whose satisfier set supplies the allowed values.
        target: String,
    },
    /// `degree`: the edge scope — the in/out edge-count bound every satisfier must
    /// land in over the one relation graph (`specs/architecture/10-contracts.md`, "Judged at
    /// the edge scope"). At least one direction must be bounded — an empty `degree`
    /// constrains nothing and is rejected at admissibility.
    Degree {
        /// The bound on a satisfier's incoming edge count, when constrained.
        incoming: Option<EdgeBound>,
        /// The bound on a satisfier's outgoing edge count, when constrained.
        outgoing: Option<EdgeBound>,
    },
}

/// One direction's inclusive `[min, max]` edge-count bound for [`Predicate::Degree`],
/// each endpoint optional: absent `min` ⇒ no lower bound, absent `max` ⇒ unbounded
/// above (the routed "≥ 1" idiom is `min: Some(1), max: None`;
/// `specs/architecture/10-contracts.md`, "self-registering" / "routed").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeBound {
    /// The inclusive lower bound. `None` ⇒ no lower bound.
    pub min: Option<usize>,
    /// The inclusive upper bound. `None` ⇒ unbounded above.
    pub max: Option<usize>,
}

impl Predicate {
    /// This predicate's clause key — the TOML `predicate` discriminator it is
    /// parsed from, reused verbatim as the diagnostic `rule` id a finding reports
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
        }
    }

    /// The arg keys this predicate reads off its clause table — the closed set of
    /// parameter keys admissible alongside the shared `severity`/`predicate`/
    /// `guidance`/`source` (`reject_unknown_clause_keys`). A key outside the union
    /// is rejected at parse, never silently dropped (`specs/architecture/10-contracts.md`,
    /// "Decision: unknown keys are rejected, not ignored"). The `allowed_chars`
    /// charset keys (`ranges`, `chars`) are both optional but both admissible; the
    /// cross-artifact predicates name none.
    #[must_use]
    pub fn arg_keys(&self) -> &'static [&'static str] {
        match self {
            Predicate::Required { .. } | Predicate::Optional { .. } => &["field"],
            Predicate::Type { .. } => &["field", "type"],
            Predicate::MinLen { .. } => &["field", "min"],
            Predicate::MaxLen { .. } => &["field", "max"],
            Predicate::Range { .. } => &["field", "min", "max"],
            Predicate::Enum { .. } | Predicate::Deny { .. } => &["field", "values"],
            Predicate::ForbiddenKeys { .. } => &["keys"],
            Predicate::AllowedChars { .. } => &["field", "ranges", "chars"],
            Predicate::MaxLines { .. } => &["max"],
            Predicate::RequireSections { .. } => &["sections"],
            Predicate::MustDefine { .. } => &["marker"],
            Predicate::SectionContains { .. } => &["heading", "marker"],
            Predicate::NameMatchesDir | Predicate::UniqueName | Predicate::DependencyExists => &[],
            Predicate::Count { .. } => &["min", "max"],
            Predicate::Unique { .. } => &["field"],
            Predicate::Membership { .. } => &["field", "target"],
            Predicate::Degree { .. } => &[
                "incoming_min",
                "incoming_max",
                "outgoing_min",
                "outgoing_max",
            ],
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
            | Predicate::Degree { .. } => None,
        }
    }

    /// The **frontmatter field** this predicate documents — the property a clause's
    /// [`guidance`](Clause::guidance) rides as a JSON Schema `description` in the
    /// emitted schema's docs channel (`specs/architecture/50-distribution.md`, "The gate at
    /// keystroke"). `Some` for the per-field frontmatter predicates whose property
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
            | Predicate::Degree { .. } => None,
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

/// Errors raised while loading a [`Contract`]. Hard failures (unreadable file,
/// malformed TOML, a clause outside the closed vocabulary) — distinct from a
/// lint finding, which is a value the check engine collects, not an error.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ContractError {
    /// The contract file is not valid TOML.
    #[error("failed to parse {path} as TOML")]
    #[diagnostic(code(temper::contract::toml))]
    Toml {
        /// The contract that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
    },

    /// `clause` is present but is not an array of tables (`[[clause]]`).
    #[error("{path}: `clause` must be an array of tables (`[[clause]]`)")]
    #[diagnostic(code(temper::contract::clause_not_array))]
    ClauseNotArray {
        /// The malformed contract.
        path: PathBuf,
    },

    /// A clause is missing a key its predicate requires.
    #[error("{path}: clause {index} is missing required key `{param}`")]
    #[diagnostic(code(temper::contract::missing_param))]
    MissingParam {
        /// The contract the clause lives in.
        path: PathBuf,
        /// The zero-based clause index.
        index: usize,
        /// The absent key.
        param: &'static str,
    },

    /// A clause key has the wrong TOML type.
    #[error("{path}: clause {index} key `{param}` must be {expected}")]
    #[diagnostic(code(temper::contract::wrong_type))]
    WrongType {
        /// The contract the clause lives in.
        path: PathBuf,
        /// The zero-based clause index.
        index: usize,
        /// The mistyped key.
        param: &'static str,
        /// The type that was expected, for the message.
        expected: &'static str,
    },

    /// A clause's `severity` is neither `required` nor `advisory`.
    #[error(
        "{path}: clause {index} has unknown severity `{value}` (expected `required` or `advisory`)"
    )]
    #[diagnostic(code(temper::contract::unknown_severity))]
    UnknownSeverity {
        /// The contract the clause lives in.
        path: PathBuf,
        /// The zero-based clause index.
        index: usize,
        /// The unrecognized severity.
        value: String,
    },

    /// A clause names a predicate outside the closed vocabulary. This is the
    /// trapdoor the closed algebra exists to keep shut — rejected, never skipped.
    #[error("{path}: clause {index} names unknown predicate `{predicate}`")]
    #[diagnostic(
        code(temper::contract::unknown_predicate),
        help(
            "a contract is closed-vocabulary data, not an escape hatch — extend the algebra deliberately, never per-contract"
        )
    )]
    UnknownPredicate {
        /// The contract the clause lives in.
        path: PathBuf,
        /// The zero-based clause index.
        index: usize,
        /// The unrecognized predicate key.
        predicate: String,
    },

    /// A clause carries a key outside its closed set — the shared
    /// `severity`/`predicate`/`guidance`/`source` plus the arg keys its predicate
    /// names. A misspelled `feild` that quietly disables a clause is exactly the
    /// silent gap temper exists to catch, so it is rejected at parse, not dropped
    /// (`specs/architecture/10-contracts.md`, "Decision: unknown keys are rejected, not ignored").
    #[error("{path}: clause {index} has unknown key `{key}`")]
    #[diagnostic(
        code(temper::contract::unknown_key),
        help(
            "a clause carries only `severity`, `predicate`, `guidance`, and its predicate's own parameters — a stray key is a typo, not an escape hatch"
        )
    )]
    UnknownKey {
        /// The contract the clause lives in.
        path: PathBuf,
        /// The zero-based clause index.
        index: usize,
        /// The unrecognized clause key.
        key: String,
    },

    /// A `type` clause declares a type outside the closed scalar/container
    /// lattice. Mirrors [`ContractError::UnknownPredicate`]: an out-of-vocabulary
    /// type is rejected at load, never silently coerced.
    #[error(
        "{path}: clause {index} declares unknown type `{declared}` (expected one of string, integer, number, boolean, list, map, null)"
    )]
    #[diagnostic(code(temper::contract::unknown_type))]
    UnknownType {
        /// The contract the clause lives in.
        path: PathBuf,
        /// The zero-based clause index.
        index: usize,
        /// The unrecognized declared type name.
        declared: String,
    },

    /// An `allowed_chars` range is not a `<lo>-<hi>` pair with `lo <= hi`.
    #[error("{path}: clause {index} has an invalid charset range `{value}` (expected `<lo>-<hi>`)")]
    #[diagnostic(code(temper::contract::invalid_range))]
    InvalidRange {
        /// The contract the clause lives in.
        path: PathBuf,
        /// The zero-based clause index.
        index: usize,
        /// The malformed range spec.
        value: String,
    },
}

impl Contract {
    /// Parse a contract from TOML source. `path` is used only to label
    /// diagnostics, so this is the seam tests drive without touching disk.
    pub fn parse(src: &str, path: &Path) -> Result<Self, ContractError> {
        let doc = src
            .parse::<DocumentMut>()
            .map_err(|source| ContractError::Toml {
                path: path.to_path_buf(),
                source,
            })?;
        let table = doc.as_table();

        // Identity is the contract's path/role, not a required internal name
        // (specs/architecture/10-contracts.md), so `name` falls back to the file stem.
        let name = table
            .get("name")
            .and_then(Item::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("contract")
                    .to_string()
            });

        let clauses = parse_clauses(table, path)?;
        // A bare `.toml` contract file has no document body, so it carries no
        // package-level guidance; the per-clause channel is parsed above.
        Ok(Self {
            name,
            clauses,
            guidance: None,
        })
    }
}

/// Parse a `[[clause]]` array of tables off `table`, in declaration order. Absent
/// ⇒ no clauses; present-but-not-an-array-of-tables ⇒
/// [`ContractError::ClauseNotArray`].
pub fn parse_clauses(table: &Table, path: &Path) -> Result<Vec<Clause>, ContractError> {
    let array = match table.get("clause") {
        None => return Ok(Vec::new()),
        Some(Item::ArrayOfTables(array)) => array,
        Some(_) => {
            return Err(ContractError::ClauseNotArray {
                path: path.to_path_buf(),
            });
        }
    };

    let mut clauses = Vec::with_capacity(array.len());
    for (index, clause) in array.iter().enumerate() {
        clauses.push(parse_clause(clause, index, path)?);
    }
    Ok(clauses)
}

/// Parse one clause table into its typed severity + predicate, plus its optional
/// advisory [`guidance`](Clause::guidance) prose.
fn parse_clause(table: &Table, index: usize, path: &Path) -> Result<Clause, ContractError> {
    let severity = parse_severity(table, index, path)?;
    let predicate = parse_predicate(table, index, path)?;
    let guidance = parse_guidance(table, index, path)?;
    let source = parse_source(table, index, path)?;
    reject_unknown_clause_keys(table, &predicate, index, path)?;
    Ok(Clause {
        severity,
        predicate,
        guidance,
        source,
    })
}

/// Reject any clause key outside the closed set — the shared `severity`,
/// `predicate`, `guidance`, `source`, plus the parsed predicate's own
/// [`arg keys`](Predicate::arg_keys) — so a stray key fails admissibility rather
/// than degrading silently (`specs/architecture/10-contracts.md`, "Decision: unknown keys are
/// rejected, not ignored").
fn reject_unknown_clause_keys(
    table: &Table,
    predicate: &Predicate,
    index: usize,
    path: &Path,
) -> Result<(), ContractError> {
    for (key, _) in table.iter() {
        let admissible = matches!(key, "severity" | "predicate" | "guidance" | "source")
            || predicate.arg_keys().contains(&key);
        if !admissible {
            return Err(ContractError::UnknownKey {
                path: path.to_path_buf(),
                index,
                key: key.to_string(),
            });
        }
    }
    Ok(())
}

/// Read the optional `guidance` key — the advisory docs-channel prose a clause may
/// carry ([`Clause::guidance`]). Absent ⇒ `None`; present-but-not-a-string ⇒
/// [`ContractError::WrongType`].
fn parse_guidance(
    table: &Table,
    index: usize,
    path: &Path,
) -> Result<Option<String>, ContractError> {
    match table.get("guidance") {
        None => Ok(None),
        Some(item) => match item.as_str() {
            Some(text) => Ok(Some(text.to_string())),
            None => Err(ContractError::WrongType {
                path: path.to_path_buf(),
                index,
                param: "guidance",
                expected: "a string",
            }),
        },
    }
}

/// Read the optional `source` key — the clause's provenance citation
/// ([`Clause::source`]), mirroring [`parse_guidance`]. Absent ⇒ `None`;
/// present-but-not-a-string ⇒ [`ContractError::WrongType`].
fn parse_source(table: &Table, index: usize, path: &Path) -> Result<Option<String>, ContractError> {
    match table.get("source") {
        None => Ok(None),
        Some(item) => match item.as_str() {
            Some(text) => Ok(Some(text.to_string())),
            None => Err(ContractError::WrongType {
                path: path.to_path_buf(),
                index,
                param: "source",
                expected: "a string",
            }),
        },
    }
}

/// Read the required `severity` key as a [`Severity`].
fn parse_severity(table: &Table, index: usize, path: &Path) -> Result<Severity, ContractError> {
    match str_param(table, "severity", index, path)?.as_str() {
        "required" => Ok(Severity::Required),
        "advisory" => Ok(Severity::Advisory),
        other => Err(ContractError::UnknownSeverity {
            path: path.to_path_buf(),
            index,
            value: other.to_string(),
        }),
    }
}

/// Read the required `predicate` discriminator and build the matching
/// [`Predicate`], pulling each predicate's own parameters. A discriminator
/// outside the closed vocabulary is rejected, never skipped.
fn parse_predicate(table: &Table, index: usize, path: &Path) -> Result<Predicate, ContractError> {
    let kind = str_param(table, "predicate", index, path)?;
    let predicate = match kind.as_str() {
        "required" => Predicate::Required {
            field: str_param(table, "field", index, path)?,
        },
        "optional" => Predicate::Optional {
            field: str_param(table, "field", index, path)?,
        },
        "type" => {
            let field = str_param(table, "field", index, path)?;
            let declared = str_param(table, "type", index, path)?;
            // The lattice's name table lives in `extract.rs`; an out-of-vocabulary
            // type is a load error, mirroring how an unknown predicate key is.
            let kind = Kind::from_name(&declared).ok_or(ContractError::UnknownType {
                path: path.to_path_buf(),
                index,
                declared,
            })?;
            Predicate::Type { field, kind }
        }
        "min_len" => Predicate::MinLen {
            field: str_param(table, "field", index, path)?,
            min: usize_param(table, "min", index, path)?,
        },
        "max_len" => Predicate::MaxLen {
            field: str_param(table, "field", index, path)?,
            max: usize_param(table, "max", index, path)?,
        },
        "range" => Predicate::Range {
            field: str_param(table, "field", index, path)?,
            min: f64_param(table, "min", index, path)?,
            max: f64_param(table, "max", index, path)?,
        },
        "enum" => Predicate::Enum {
            field: str_param(table, "field", index, path)?,
            values: str_list(table, "values", index, path)?,
        },
        "deny" => Predicate::Deny {
            field: str_param(table, "field", index, path)?,
            values: str_list(table, "values", index, path)?,
        },
        "forbidden_keys" => Predicate::ForbiddenKeys {
            keys: str_list(table, "keys", index, path)?,
        },
        "allowed_chars" => Predicate::AllowedChars {
            field: str_param(table, "field", index, path)?,
            charset: parse_charset(table, index, path)?,
        },
        "max_lines" => Predicate::MaxLines {
            max: usize_param(table, "max", index, path)?,
        },
        "require_sections" => Predicate::RequireSections {
            sections: str_list(table, "sections", index, path)?,
        },
        "must_define" => Predicate::MustDefine {
            marker: str_param(table, "marker", index, path)?,
        },
        "section_contains" => Predicate::SectionContains {
            heading: str_param(table, "heading", index, path)?,
            marker: str_param(table, "marker", index, path)?,
        },
        "name-matches-dir" => Predicate::NameMatchesDir,
        "unique-name" => Predicate::UniqueName,
        "dependency-exists" => Predicate::DependencyExists,
        "count" => Predicate::Count {
            min: usize_param(table, "min", index, path)?,
            max: usize_param(table, "max", index, path)?,
        },
        "unique" => Predicate::Unique {
            field: str_param(table, "field", index, path)?,
        },
        "membership" => Predicate::Membership {
            field: str_param(table, "field", index, path)?,
            target: str_param(table, "target", index, path)?,
        },
        "degree" => Predicate::Degree {
            incoming: parse_edge_bound(table, "incoming_min", "incoming_max", index, path)?,
            outgoing: parse_edge_bound(table, "outgoing_min", "outgoing_max", index, path)?,
        },
        other => {
            return Err(ContractError::UnknownPredicate {
                path: path.to_path_buf(),
                index,
                predicate: other.to_string(),
            });
        }
    };
    Ok(predicate)
}

/// Build a [`Charset`] from a clause's optional `ranges` (an array of `<lo>-<hi>`
/// specs) and optional `chars` (a literal string of permitted characters).
fn parse_charset(table: &Table, index: usize, path: &Path) -> Result<Charset, ContractError> {
    let ranges = match table.get("ranges") {
        None => Vec::new(),
        Some(_) => {
            let specs = str_list(table, "ranges", index, path)?;
            let mut ranges = Vec::with_capacity(specs.len());
            for spec in specs {
                ranges.push(parse_range(&spec, index, path)?);
            }
            ranges
        }
    };
    let chars = match table.get("chars") {
        None => BTreeSet::new(),
        Some(_) => str_param(table, "chars", index, path)?.chars().collect(),
    };
    Ok(Charset { ranges, chars })
}

/// Parse a single `<lo>-<hi>` inclusive range spec (exactly three characters, a
/// literal `-` in the middle, `lo <= hi`).
fn parse_range(spec: &str, index: usize, path: &Path) -> Result<(char, char), ContractError> {
    let chars: Vec<char> = spec.chars().collect();
    match chars.as_slice() {
        [lo, '-', hi] if lo <= hi => Ok((*lo, *hi)),
        _ => Err(ContractError::InvalidRange {
            path: path.to_path_buf(),
            index,
            value: spec.to_string(),
        }),
    }
}

/// Parse one [`EdgeBound`] direction of a `degree` clause off its `min`/`max` key
/// pair (e.g. `incoming_min`/`incoming_max`). Both absent ⇒ `None` — that
/// direction is unconstrained, distinct from a bound with an endpoint of `0`.
fn parse_edge_bound(
    table: &Table,
    min_key: &'static str,
    max_key: &'static str,
    index: usize,
    path: &Path,
) -> Result<Option<EdgeBound>, ContractError> {
    let min = usize_param_opt(table, min_key, index, path)?;
    let max = usize_param_opt(table, max_key, index, path)?;
    if min.is_none() && max.is_none() {
        Ok(None)
    } else {
        Ok(Some(EdgeBound { min, max }))
    }
}

/// Read an optional non-negative integer clause key as a `usize`. Absent ⇒
/// `None`; present-but-wrong-type ⇒ [`ContractError::WrongType`].
fn usize_param_opt(
    table: &Table,
    key: &'static str,
    index: usize,
    path: &Path,
) -> Result<Option<usize>, ContractError> {
    match table.get(key) {
        None => Ok(None),
        Some(item) => {
            let raw = item.as_integer().ok_or(ContractError::WrongType {
                path: path.to_path_buf(),
                index,
                param: key,
                expected: "an integer",
            })?;
            let value = usize::try_from(raw).map_err(|_| ContractError::WrongType {
                path: path.to_path_buf(),
                index,
                param: key,
                expected: "a non-negative integer",
            })?;
            Ok(Some(value))
        }
    }
}

/// Read a required string clause key.
fn str_param(
    table: &Table,
    key: &'static str,
    index: usize,
    path: &Path,
) -> Result<String, ContractError> {
    match table.get(key) {
        None => Err(ContractError::MissingParam {
            path: path.to_path_buf(),
            index,
            param: key,
        }),
        Some(item) => item
            .as_str()
            .map(str::to_string)
            .ok_or(ContractError::WrongType {
                path: path.to_path_buf(),
                index,
                param: key,
                expected: "a string",
            }),
    }
}

/// Read a required non-negative integer clause key as a `usize`.
fn usize_param(
    table: &Table,
    key: &'static str,
    index: usize,
    path: &Path,
) -> Result<usize, ContractError> {
    let item = table.get(key).ok_or(ContractError::MissingParam {
        path: path.to_path_buf(),
        index,
        param: key,
    })?;
    let raw = item.as_integer().ok_or(ContractError::WrongType {
        path: path.to_path_buf(),
        index,
        param: key,
        expected: "an integer",
    })?;
    usize::try_from(raw).map_err(|_| ContractError::WrongType {
        path: path.to_path_buf(),
        index,
        param: key,
        expected: "a non-negative integer",
    })
}

/// Read a required numeric clause key (a TOML integer or float) as an `f64` —
/// the bound the `range` predicate ranges over `integer`/`number` fields with.
/// An integer literal (`min = 0`) is accepted alongside a float (`min = 0.5`) so
/// a whole-number bound need not be written with a decimal point.
fn f64_param(
    table: &Table,
    key: &'static str,
    index: usize,
    path: &Path,
) -> Result<f64, ContractError> {
    let item = table.get(key).ok_or(ContractError::MissingParam {
        path: path.to_path_buf(),
        index,
        param: key,
    })?;
    if let Some(float) = item.as_float() {
        Ok(float)
    } else if let Some(int) = item.as_integer() {
        Ok(int as f64)
    } else {
        Err(ContractError::WrongType {
            path: path.to_path_buf(),
            index,
            param: key,
            expected: "a number",
        })
    }
}

/// Read a required array-of-strings clause key.
fn str_list(
    table: &Table,
    key: &'static str,
    index: usize,
    path: &Path,
) -> Result<Vec<String>, ContractError> {
    let item = table.get(key).ok_or(ContractError::MissingParam {
        path: path.to_path_buf(),
        index,
        param: key,
    })?;
    let array = item.as_array().ok_or(ContractError::WrongType {
        path: path.to_path_buf(),
        index,
        param: key,
        expected: "an array of strings",
    })?;

    let mut out = Vec::with_capacity(array.len());
    for value in array.iter() {
        let string = value.as_str().ok_or(ContractError::WrongType {
            path: path.to_path_buf(),
            index,
            param: key,
            expected: "an array of strings",
        })?;
        out.push(string.to_string());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A representative contract exercising every predicate in the algebra, with
    /// a mix of `required` and `advisory` severities.
    const REP: &str = r#"
name = "skill"

[[clause]]
severity = "required"
predicate = "required"
field = "name"

[[clause]]
severity = "advisory"
predicate = "optional"
field = "version"

[[clause]]
severity = "advisory"
predicate = "min_len"
field = "description"
min = 1

[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 64

[[clause]]
severity = "advisory"
predicate = "range"
field = "priority"
min = 0
max = 9

[[clause]]
severity = "advisory"
predicate = "enum"
field = "status"
values = ["draft", "active", "deprecated"]

[[clause]]
severity = "required"
predicate = "deny"
field = "name"
values = ["anthropic", "claude"]

[[clause]]
severity = "required"
predicate = "forbidden_keys"
keys = ["globs", "alwaysApply"]

[[clause]]
severity = "required"
predicate = "allowed_chars"
field = "name"
ranges = ["a-z", "0-9"]
chars = "-"

[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 500

[[clause]]
severity = "advisory"
predicate = "require_sections"
sections = ["Usage", "Examples"]

[[clause]]
severity = "required"
predicate = "must_define"
marker = "disable-model-invocation"

[[clause]]
severity = "required"
predicate = "name-matches-dir"

[[clause]]
severity = "required"
predicate = "unique-name"

[[clause]]
severity = "advisory"
predicate = "dependency-exists"

[[clause]]
severity = "required"
predicate = "type"
field = "name"
type = "string"
"#;

    /// The typed model `REP` must deserialize into — every primitive in the
    /// algebra, each pinned to the severity its clause declared.
    fn rep_expected() -> Contract {
        Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses: vec![
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::Required {
                        field: "name".to_string(),
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::Optional {
                        field: "version".to_string(),
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::MinLen {
                        field: "description".to_string(),
                        min: 1,
                    },
                },
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
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::Range {
                        field: "priority".to_string(),
                        min: 0.0,
                        max: 9.0,
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::Enum {
                        field: "status".to_string(),
                        values: vec![
                            "draft".to_string(),
                            "active".to_string(),
                            "deprecated".to_string(),
                        ],
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::Deny {
                        field: "name".to_string(),
                        values: vec!["anthropic".to_string(), "claude".to_string()],
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::ForbiddenKeys {
                        keys: vec!["globs".to_string(), "alwaysApply".to_string()],
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::AllowedChars {
                        field: "name".to_string(),
                        charset: Charset {
                            ranges: vec![('a', 'z'), ('0', '9')],
                            chars: BTreeSet::from(['-']),
                        },
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::MaxLines { max: 500 },
                },
                Clause {
                    source: None,
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::RequireSections {
                        sections: vec!["Usage".to_string(), "Examples".to_string()],
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::MustDefine {
                        marker: "disable-model-invocation".to_string(),
                    },
                },
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::NameMatchesDir,
                },
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::UniqueName,
                },
                Clause {
                    source: None,
                    severity: Severity::Advisory,
                    guidance: None,
                    predicate: Predicate::DependencyExists,
                },
                Clause {
                    source: None,
                    severity: Severity::Required,
                    guidance: None,
                    predicate: Predicate::Type {
                        field: "name".to_string(),
                        kind: Kind::String,
                    },
                },
            ],
        }
    }

    #[test]
    fn parses_a_multi_clause_contract_into_the_typed_algebra() {
        let contract = Contract::parse(REP, Path::new("skill.contract.toml")).unwrap();
        // Every primitive round-trips into its typed clause, with the per-clause
        // severity preserved exactly as the author declared it.
        assert_eq!(contract, rep_expected());
    }

    #[test]
    fn allowed_chars_charset_admits_the_declared_set_only() {
        let contract = Contract::parse(REP, Path::new("c.toml")).unwrap();
        let charset = contract
            .clauses
            .iter()
            .find_map(|clause| match &clause.predicate {
                Predicate::AllowedChars { charset, .. } => Some(charset),
                _ => None,
            })
            .expect("the representative contract carries an allowed_chars clause");

        assert!(charset.allows('a'));
        assert!(charset.allows('z'));
        assert!(charset.allows('0'));
        assert!(charset.allows('-'));
        assert!(!charset.allows('A'));
        assert!(!charset.allows('_'));
    }

    #[test]
    fn unknown_predicate_is_a_load_error_not_a_silent_skip() {
        let toml = r#"
name = "skill"

[[clause]]
severity = "required"
predicate = "word_count"
field = "description"
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(
            err,
            ContractError::UnknownPredicate { ref predicate, index: 0, .. } if predicate == "word_count"
        ));
    }

    #[test]
    fn a_stray_clause_key_is_a_load_error_not_a_silent_drop() {
        // A misspelled `feild` is not one of the clause's admissible keys
        // (`severity`/`predicate`/`guidance` + the predicate's own args), so it is
        // rejected at parse — a typo must fail loudly, never degrade the clause.
        let toml = r#"
[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 64
feild = "nmae"
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(
            err,
            ContractError::UnknownKey { ref key, index: 0, .. } if key == "feild"
        ));
    }

    #[test]
    fn an_arg_key_from_the_wrong_predicate_is_a_load_error() {
        // `max` belongs to `max_len`/`range`/`max_lines`, not `required` — carrying
        // it on a `required` clause is a stray key, rejected the same way a wholly
        // misspelled key is. The closed set is *per predicate*, not a global union.
        let toml = r#"
[[clause]]
severity = "required"
predicate = "required"
field = "name"
max = 64
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(
            err,
            ContractError::UnknownKey { ref key, index: 0, .. } if key == "max"
        ));
    }

    #[test]
    fn a_clause_carrying_guidance_and_its_own_args_parses_clean() {
        // The admissible set is `severity`/`predicate`/`guidance` plus the
        // predicate's args — a clean clause using all of them trips nothing.
        let toml = r#"
[[clause]]
severity = "advisory"
predicate = "allowed_chars"
field = "name"
ranges = ["a-z", "0-9"]
chars = "-"
guidance = "lowercase, digits, and hyphen only"
"#;
        let contract = Contract::parse(toml, Path::new("c.toml")).unwrap();
        assert_eq!(contract.clauses.len(), 1);
        assert_eq!(
            contract.clauses[0].guidance.as_deref(),
            Some("lowercase, digits, and hyphen only")
        );
    }

    #[test]
    fn a_type_clause_parses_into_the_closed_lattice() {
        let toml = r#"
[[clause]]
severity = "required"
predicate = "type"
field = "count"
type = "integer"
"#;
        let contract = Contract::parse(toml, Path::new("c.toml")).unwrap();
        assert_eq!(
            contract.clauses,
            vec![Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::Type {
                    field: "count".to_string(),
                    kind: Kind::Integer,
                },
            }]
        );
    }

    #[test]
    fn an_unknown_declared_type_is_a_load_error() {
        // A declared type outside the lattice is rejected at load, exactly as an
        // out-of-vocabulary predicate key is — no silent coercion.
        let toml = r#"
[[clause]]
severity = "required"
predicate = "type"
field = "count"
type = "int"
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(
            err,
            ContractError::UnknownType { ref declared, index: 0, .. } if declared == "int"
        ));
    }

    #[test]
    fn a_range_clause_accepts_integer_and_float_bounds() {
        // The `range` bound spans `integer`/`number` fields, so a whole-number
        // bound may be written without a decimal point and a fractional one with.
        let toml = r#"
[[clause]]
severity = "required"
predicate = "range"
field = "score"
min = 0
max = 1.5
"#;
        let contract = Contract::parse(toml, Path::new("c.toml")).unwrap();
        assert_eq!(
            contract.clauses,
            vec![Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::Range {
                    field: "score".to_string(),
                    min: 0.0,
                    max: 1.5,
                },
            }]
        );
    }

    #[test]
    fn a_non_numeric_range_bound_is_a_load_error() {
        let toml = r#"
[[clause]]
severity = "required"
predicate = "range"
field = "score"
min = 0
max = "ten"
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(err, ContractError::WrongType { param: "max", .. }));
    }

    #[test]
    fn unknown_severity_is_a_load_error() {
        let toml = r#"
name = "skill"

[[clause]]
severity = "blocker"
predicate = "name-matches-dir"
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(
            err,
            ContractError::UnknownSeverity { ref value, .. } if value == "blocker"
        ));
    }

    #[test]
    fn a_predicate_missing_its_parameter_is_a_load_error() {
        let toml = r#"
name = "skill"

[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(
            err,
            ContractError::MissingParam { param: "max", .. }
        ));
    }

    #[test]
    fn a_mistyped_parameter_is_a_load_error() {
        let toml = r#"
name = "skill"

[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = "sixty-four"
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(err, ContractError::WrongType { param: "max", .. }));
    }

    #[test]
    fn an_invalid_charset_range_is_a_load_error() {
        let toml = r#"
name = "skill"

[[clause]]
severity = "required"
predicate = "allowed_chars"
field = "name"
ranges = ["a..z"]
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(
            err,
            ContractError::InvalidRange { ref value, .. } if value == "a..z"
        ));
    }

    #[test]
    fn name_absent_derives_from_file_stem() {
        let toml = r#"
[[clause]]
severity = "required"
predicate = "name-matches-dir"
"#;
        // No top-level `name`: identity is the path, label derives from the stem.
        let contract = Contract::parse(toml, Path::new("skill.anthropic.toml")).unwrap();
        assert_eq!(contract.name, "skill.anthropic");
        assert_eq!(contract.clauses.len(), 1);
    }

    #[test]
    fn a_contract_with_no_clauses_is_vacuously_valid() {
        let contract = Contract::parse("name = \"empty\"\n", Path::new("c.toml")).unwrap();
        assert_eq!(contract.name, "empty");
        assert!(contract.clauses.is_empty());
    }

    // ---- node-set / edge-scope predicates (REQUIREMENT-CLAUSES-ALGEBRA) --------

    #[test]
    fn a_count_clause_parses_into_the_closed_algebra() {
        let toml = r#"
[[clause]]
severity = "required"
predicate = "count"
min = 1
max = 3
"#;
        let contract = Contract::parse(toml, Path::new("c.toml")).unwrap();
        assert_eq!(
            contract.clauses,
            vec![Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::Count { min: 1, max: 3 },
            }]
        );
    }

    #[test]
    fn a_unique_clause_parses_into_the_closed_algebra() {
        let toml = r#"
[[clause]]
severity = "advisory"
predicate = "unique"
field = "name"
"#;
        let contract = Contract::parse(toml, Path::new("c.toml")).unwrap();
        assert_eq!(
            contract.clauses,
            vec![Clause {
                source: None,
                severity: Severity::Advisory,
                guidance: None,
                predicate: Predicate::Unique {
                    field: "name".to_string(),
                },
            }]
        );
    }

    #[test]
    fn a_membership_clause_parses_into_the_closed_algebra() {
        let toml = r#"
[[clause]]
severity = "required"
predicate = "membership"
field = "model"
target = "approved-models"
"#;
        let contract = Contract::parse(toml, Path::new("c.toml")).unwrap();
        assert_eq!(
            contract.clauses,
            vec![Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::Membership {
                    field: "model".to_string(),
                    target: "approved-models".to_string(),
                },
            }]
        );
    }

    #[test]
    fn a_membership_clause_may_still_carry_its_own_citation() {
        // `target` (the requirement `membership` draws its set from) and `source`
        // (the clause's own provenance citation) are distinct keys — a membership
        // clause can carry both without collision.
        let toml = r#"
[[clause]]
severity = "required"
predicate = "membership"
field = "model"
target = "approved-models"
source = "https://example.com/models, retrieved 2026-07-06"
"#;
        let contract = Contract::parse(toml, Path::new("c.toml")).unwrap();
        assert_eq!(
            contract.clauses[0].predicate,
            Predicate::Membership {
                field: "model".to_string(),
                target: "approved-models".to_string(),
            }
        );
        assert_eq!(
            contract.clauses[0].source.as_deref(),
            Some("https://example.com/models, retrieved 2026-07-06")
        );
    }

    #[test]
    fn a_degree_clause_parses_both_directions_from_flat_bound_keys() {
        let toml = r#"
[[clause]]
severity = "required"
predicate = "degree"
incoming_min = 1
outgoing_max = 3
"#;
        let contract = Contract::parse(toml, Path::new("c.toml")).unwrap();
        assert_eq!(
            contract.clauses,
            vec![Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::Degree {
                    incoming: Some(EdgeBound {
                        min: Some(1),
                        max: None,
                    }),
                    outgoing: Some(EdgeBound {
                        min: None,
                        max: Some(3),
                    }),
                },
            }]
        );
    }

    #[test]
    fn a_degree_clause_with_neither_direction_still_parses() {
        // Parsing accepts a directionless `degree` — admissibility (`crate::engine`)
        // is where the vacuous clause is rejected, mirroring how an inverted
        // `range` bound parses clean and fails only at admissibility.
        let toml = r#"
[[clause]]
severity = "advisory"
predicate = "degree"
"#;
        let contract = Contract::parse(toml, Path::new("c.toml")).unwrap();
        assert_eq!(
            contract.clauses,
            vec![Clause {
                source: None,
                severity: Severity::Advisory,
                guidance: None,
                predicate: Predicate::Degree {
                    incoming: None,
                    outgoing: None,
                },
            }]
        );
    }

    #[test]
    fn a_count_clause_missing_max_is_a_load_error() {
        let toml = r#"
[[clause]]
severity = "required"
predicate = "count"
min = 1
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(
            err,
            ContractError::MissingParam { param: "max", .. }
        ));
    }

    #[test]
    fn a_membership_stray_key_is_a_load_error() {
        // `min` belongs to `count`/`min_len`/`range`, not `membership` — a stray key
        // from another predicate's vocabulary is rejected, not silently dropped.
        let toml = r#"
[[clause]]
severity = "required"
predicate = "membership"
field = "model"
target = "approved-models"
min = 1
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(
            err,
            ContractError::UnknownKey { ref key, index: 0, .. } if key == "min"
        ));
    }
}
