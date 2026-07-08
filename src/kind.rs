//! The extraction algebra — a kind's read side, composed from data.
//!
//! Where `crate::contract` is the engine's predicate half
//! (what an artifact must satisfy), this is the extraction half (what it *is*,
//! and how it is read). Extraction is the soundness boundary: a clause is sound
//! only if its feature is deterministically extractable, so a kind carries no
//! code of its own — its extractor is composed from a closed algebra of
//! deterministic [`Primitive`]s, authored as plain Rust data (there is no kind
//! file format to parse it from — "Decision: field typing lives in the SDK").
//!
//! Every primitive delegates to the same surface extractor the built-in
//! projectors use (`crate::extract`), so the soundness boundary is one boundary,
//! not a forked implementation that can drift.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;

use crate::compose::Edge;
use crate::document::{Document, PublishedRequirement};
use crate::drift::KindFactRow;
use crate::extract::{self, Features};

/// The file locus a custom kind reads: the root
/// directory its units live under, and the filename glob that selects them.
/// `import` scans `root` for files matching `glob`. File placement is itself an
/// extraction primitive, so the locus is part of the authored definition, not
/// external config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Governs {
    /// The root directory the kind's units sit under (`specs`, `docs/adr`), a path
    /// relative to the harness the assembly governs.
    pub root: String,
    /// The filename glob that selects the kind's units under `root` (`*.md`,
    /// `[0-9][0-9]-*.md`), stored verbatim.
    pub glob: String,
}

/// A kind's declared definition — a constructor plus the five facts of runtime
/// residue: the [`governs`](CustomKind::governs) locus, the composed
/// [`Extraction`], and the declared [`relationships`](CustomKind::relationships).
/// Every kind is Rust data — a built-in is authored directly in
/// [`crate::builtin_kind`]; there is no `KIND.md` file format to parse it from.
///
/// A custom kind is purely declare-side — it carries no clauses. Predicates over
/// its members ride the assembly's `expect`/`require` clauses.
///
/// Not `Eq`: keeping the derive `PartialEq` leaves room for future `f64`-bearing
/// fields without churn, as it does for [`Clause`](crate::contract::Clause).
#[derive(Debug, Clone, PartialEq)]
pub struct CustomKind {
    /// The kind's bare name — the `[kind.<name>]` registration key, and the
    /// surface subdirectory/member-document convention key
    /// ([`member_document`](CustomKind::member_document)).
    pub name: String,
    /// The file locus the kind reads.
    pub governs: Governs,
    /// The composed extractor over the closed algebra,
    /// authored via [`Extraction::new`]. An empty primitive set is the vacuous
    /// extractor (only the intrinsic id).
    pub extraction: Extraction,
    /// The declared relationships — which of the kind's references are edges,
    /// each an [`Edge`] whose `from` is this kind.
    /// Absent ⇒ empty (the default [`CustomKind::new`] leaves it at).
    pub relationships: Vec<Edge>,
    /// The declared projection format — how a member's on-disk artifact is shaped.
    /// A closed vocabulary; absent ⇒ `None` (today's built-in kinds
    /// declare none). Inert until DECLARED-FRONTMATTER-ADAPTER: typed, consumed by
    /// nothing yet.
    pub format: Option<Format>,
    /// The declared unit shape — whether a member is a lone file (id from the stem),
    /// a directory with companions (id from the directory name), or a lone file whose
    /// id is read from a declared frontmatter field (an agent's `name`).
    /// A closed enum; absent ⇒ `None`. Inert alongside
    /// [`format`](CustomKind::format).
    pub unit_shape: Option<UnitShape>,
    /// The declared registration — the kind's world fact: the **set** of documented
    /// channels a member reaches the world over (user invocation and description
    /// trigger are channels, not rivals; `builtins.md`, "The shipped kinds"). A closed
    /// per-channel vocabulary; empty ⇒ no declared registration (today's built-in kinds
    /// each declare at least one). Stored inert — REACHABILITY reads it to decide a
    /// member's world edge is live iff any one channel is; nothing else consumes it yet.
    pub registration: Vec<Registration>,
    /// The kind's declared **templates** — one per inner layer of nested members it
    /// hosts at the embedded locus:
    /// the child kind plus its embedded addressing, per member fence. Extraction folds
    /// a member's embedded fences into typed [`EmbeddedMember`](crate::extract::EmbeddedMember)s
    /// against this set ([`CustomKind::extract`]); the shape is the kind's, any
    /// predicate over it rides the assembly's `expect`/`require` clauses.
    /// Absent ⇒ empty.
    pub templates: Vec<Template>,
}

/// A kind's declared **projection format** — the closed vocabulary naming how a
/// member's on-disk artifact is shaped. The engine implements each
/// format once, generically; the first and only harvested entry is
/// [`YamlFrontmatter`](Format::YamlFrontmatter).
/// Any other value is a load error, the same closed-vocabulary guard the extraction
/// primitives carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// `yaml-frontmatter` — YAML frontmatter over a markdown body, the Claude Code
    /// family's shape.
    YamlFrontmatter,
}

/// A kind's declared **unit shape** — the format fact that varies per kind:
/// whether a member's on-disk artifact is a lone file, its
/// identity the filename stem; a directory with companions, its identity the
/// directory name; or a lone file whose identity is read from a declared
/// frontmatter field rather than derived from the path. A closed enum; any other
/// value is a load error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitShape {
    /// `file` — a lone file; the member's id is its filename stem (a rule's
    /// `.claude/rules/rust.md`).
    File,
    /// `directory` — a directory with companions; the member's id is the directory
    /// name (a skill's `.claude/skills/<name>/SKILL.md`).
    Directory,
    /// `named-field` — a lone file whose id is read from a declared frontmatter
    /// field, not the filename (an agent's `name`; any containing subdirectory is
    /// purely organizational).
    NamedField {
        /// The frontmatter field the id is read from.
        field: String,
    },
}

/// A kind's declared registration — one **channel** among the inbound boundary edges
/// of the relation graph: one documented way the harness reaches a
/// member, per-kind mechanics over per-member data. A closed vocabulary harvested from the kinds
/// temper ships; any other value is a load error, the same closed-vocabulary guard
/// [`Format`] and [`UnitShape`] carry. The three field-carrying variants name the
/// declared frontmatter field they range over, never a value — the glob/description
/// *values* stay the member's ordinary clauses. A kind declares a **set** of these
/// ([`CustomKind::registration`]) — user invocation and description trigger are
/// channels, not rivals (`builtins.md`, "The shipped kinds"). Inert until REACHABILITY
/// reads the set to decide a member's world edge is live iff any one channel is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Registration {
    /// `always` — loaded at launch, unconditionally (a rule without `paths`;
    /// `CLAUDE.md` itself). Carries no field: the edge is unconditional.
    Always,
    /// `user-invoked` — the member is directly invocable by name (a skill's `/name`).
    /// Carries no field: no repo-decidable criterion names this channel dead, mirroring
    /// [`Always`](Registration::Always) — a member's `user-invocable` modulating field
    /// is an ordinary declared field, not part of this channel's identity.
    UserInvoked,
    /// `description-trigger(field)` — the named field is always in context, the body
    /// loading on invocation (a skill's `description`). The field names the declared
    /// frontmatter field the trigger ranges over.
    DescriptionTrigger {
        /// The declared frontmatter field always kept in context.
        field: String,
    },
    /// `paths-match(field)` — the member activates when the agent reads files matching
    /// the named glob field (a path-scoped rule's `paths`).
    PathsMatch {
        /// The declared frontmatter field carrying the registration glob.
        field: String,
    },
    /// `event(field)` — the member executes at a named lifecycle event (carried for the
    /// future `hook` kind). The field names the declared lifecycle-event field.
    Event {
        /// The declared frontmatter field naming the lifecycle event.
        field: String,
    },
}

/// A **template** a kind declares for one inner layer of nested members it hosts at
/// the embedded locus: the child kind a fence info string names, serialized whole
/// into the lock. Any *predicate* over a nested member's interior rides the
/// assembly's `expect`/`require` clauses, **out of the kind object** — the same
/// ownership line extraction and contract split on everywhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    /// The child kind — the `member.<kind>` a fence info string carries
    /// (`member.decision surface-authority` → `decision`), the token extraction matches
    /// a fence against to fold it into a typed
    /// [`EmbeddedMember`](crate::extract::EmbeddedMember).
    pub kind: String,
}

impl CustomKind {
    /// Construct a kind's declared definition directly — the constructor a built-in
    /// (`crate::builtin_kind`) or a future SDK-authored custom kind supplies its five
    /// facts through. There is no file format to load it from:
    /// every field here is plain Rust data, set by
    /// the caller rather than parsed.
    #[must_use]
    pub fn new(name: impl Into<String>, governs: Governs, extraction: Extraction) -> Self {
        Self {
            name: name.into(),
            governs,
            extraction,
            relationships: Vec::new(),
            format: None,
            unit_shape: None,
            registration: Vec::new(),
            templates: Vec::new(),
        }
    }

    /// Reconstruct a kind's declared definition from the committed lock's own
    /// [`KindFactRow`]: the row's five-fact
    /// residue lifts into `governs`/`format`/
    /// `unit_shape`/`registration` directly, each channel label this projection cannot
    /// parse dropped from the reconstructed set — the same tolerant read the rest of a
    /// hand-editable lock takes. A `KIND.md` file format never carried field-level
    /// extraction primitives either, so the
    /// reconstructed extractor stays the same generic markdown-structure set every
    /// built-in composes (`headings`/`sections`/`line_count`/`placement`); a floor
    /// clause's own `field` column, plus the permissive frontmatter fold every custom
    /// member's extraction already runs through (`crate::builtin_kind::features`), is
    /// what actually ranges over a custom member's declared fields — never a per-kind
    /// `Field` primitive list.
    ///
    /// The reconstructed extraction now includes `Fenced` alongside the generic
    /// markdown-structure set, so the raw fenced-block substrate a member fence needs
    /// is always available. The row's `templates` column lifts into one
    /// [`Template`] per declared child-kind name, so a lock-reconstructed kind folds
    /// the same embedded members its live SDK declaration does.
    #[must_use]
    pub fn from_kind_fact_row(row: &KindFactRow) -> Self {
        CustomKind {
            format: row.format.as_deref().and_then(format_from_label),
            unit_shape: row.unit_shape.as_deref().and_then(unit_shape_from_label),
            registration: row
                .registration
                .iter()
                .filter_map(|label| registration_from_label(label))
                .collect(),
            templates: row
                .templates
                .iter()
                .map(|kind| Template { kind: kind.clone() })
                .collect(),
            ..CustomKind::new(
                row.name.clone(),
                Governs {
                    root: row.governs_root.clone(),
                    glob: row.governs_glob.clone(),
                },
                Extraction::new(vec![
                    Primitive::LineCount,
                    Primitive::Headings,
                    Primitive::Sections,
                    Primitive::Placement,
                    Primitive::Fenced,
                ]),
            )
        }
    }

    /// Run the kind's composed extractor over `unit`, then fold its declared templates:
    /// each
    /// fenced block whose info string names a declared child kind (`member.<kind> <key>`)
    /// has its interior TOML parsed into a typed [`EmbeddedMember`](crate::extract::EmbeddedMember)
    /// and folded into `Features::nested_members`,
    /// beside its raw form in `fenced_blocks`. This composes the `Fenced` primitive with a
    /// TOML parse — the typed nested-member layer over the raw-block algebra.
    /// The single entry point every extract call site routes through, so member folding
    /// never forks from the primitive extraction. A kind declaring no templates (every
    /// built-in), or a body with no matching fence, folds nothing.
    #[must_use]
    pub fn extract(&self, unit: &Unit) -> Features {
        let mut features = self.extraction.extract(unit);
        self.fold_members(&mut features);
        features
    }

    /// Fold this kind's declared templates out of the already-extracted `fenced_blocks`.
    /// A block whose info string parses as
    /// `member.<kind> <key>` for a **declared** template and whose interior is well-formed
    /// TOML becomes an [`EmbeddedMember`](crate::extract::EmbeddedMember); a fence naming
    /// an undeclared child kind, or any non-member block, stays raw-only — adoption is
    /// opt-in per block. A pure function of `fenced_blocks` and the declared template
    /// set, so re-running is byte-identical, the property that keeps a nested member a
    /// sound gate input.
    fn fold_members(&self, features: &mut Features) {
        if self.templates.is_empty() {
            return;
        }
        let mut nested_members = Vec::new();
        for block in &features.fenced_blocks {
            let Some((kind, key)) = extract::parse_embedded_info(&block.info) else {
                continue;
            };
            if !self.templates.iter().any(|declared| declared.kind == kind) {
                continue;
            }
            if let Some(member) = extract::parse_embedded_member(&kind, &key, &block.content) {
                nested_members.push(member);
            }
        }
        features.nested_members = nested_members;
    }

    /// The kind's **identity** — its bare `name`. Kept as its own
    /// method rather than inlining `.name.clone()` at each call site.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        self.name.clone()
    }

    /// The kind's declared frontmatter fields, in declaration order — the `field`
    /// extraction primitives' keys. The generic frontmatter
    /// adapter (`crate::frontmatter`) lifts these
    /// into the leading `[clause.<field>]` tables, before the preserved unknown keys.
    #[must_use]
    pub fn declared_fields(&self) -> Vec<&str> {
        self.extraction
            .primitives()
            .iter()
            .filter_map(|primitive| match primitive {
                Primitive::Field { key } => Some(key.as_str()),
                _ => None,
            })
            .collect()
    }

    /// The surface member-document filename for this kind — the kind name upper-cased
    /// with a `.md` suffix (`skill` → `SKILL.md`, `rule` → `RULE.md`), the name both
    /// the emit face writes and the reload face reads (`src/frontmatter.rs`,
    /// `src/import.rs`).
    #[must_use]
    pub fn member_document(&self) -> String {
        format!("{}.md", self.name.to_uppercase())
    }

    /// The surface subdirectory a member of this kind lands under — the leaf of the
    /// `governs.root` locus (`.claude/skills` → `skills`, `.claude/rules` → `rules`).
    /// The read face's scan root and the emit face's write root share this leaf, so a
    /// built-in kind's surface tree is derived from its declaration, not hardwired.
    #[must_use]
    pub fn surface_subdir(&self) -> &str {
        self.governs
            .root
            .rsplit('/')
            .next()
            .unwrap_or(&self.governs.root)
    }

    /// Whether a surface member imported from `source_path` belongs to this kind — its
    /// source filename matches the kind's `governs` glob leaf. The discriminator for two
    /// kinds that **share a surface locus**. A kind at a unique locus
    /// (skill's `SKILL.md`, rule's `*.md`) matches its own members, so the filter is a
    /// no-op there. A member with no readable source name belongs to nothing rather than
    /// mis-dispatching.
    #[must_use]
    pub fn owns_source(&self, source_path: &Path) -> bool {
        source_path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| glob_matches(self.governs.glob_leaf(), name))
    }
}

impl Governs {
    /// The glob's file-matching **leaf** — its final `/`-separated segment
    /// (`*/SKILL.md` → `SKILL.md`, `CLAUDE.md` → `CLAUDE.md`). Earlier segments select
    /// subdirectories to descend; the leaf selects the member files under the locus, so it
    /// is the segment a per-member membership test ([`CustomKind::owns_source`]) matches
    /// against a source filename.
    #[must_use]
    pub fn glob_leaf(&self) -> &str {
        self.glob.rsplit('/').next().unwrap_or(&self.glob)
    }
}

/// Parse a [`KindFactRow::format`] label into its typed [`Format`] — `None` for any
/// label outside the closed vocabulary, the tolerant read the rest of a hand-editable
/// lock takes.
fn format_from_label(label: &str) -> Option<Format> {
    match label {
        "yaml-frontmatter" => Some(Format::YamlFrontmatter),
        _ => None,
    }
}

/// Parse a [`KindFactRow::unit_shape`] label into its typed [`UnitShape`] — `None`
/// outside the closed vocabulary. `named-field(<field>)` is the third mode's wire
/// form, the same `<name>(<field>)` call syntax [`registration_from_label`]'s
/// field-carrying variants use.
fn unit_shape_from_label(label: &str) -> Option<UnitShape> {
    match label {
        "file" => return Some(UnitShape::File),
        "directory" => return Some(UnitShape::Directory),
        _ => {}
    }
    let (name, field) = label.strip_suffix(')')?.split_once('(')?;
    (name == "named-field").then(|| UnitShape::NamedField {
        field: field.to_string(),
    })
}

/// Parse one [`KindFactRow::registration`] wire label into its typed [`Registration`]
/// channel — the closed vocabulary's compact wire form (`always`/`user-invoked`, or a
/// `<name>(<field>)` call for the three field-carrying variants). `None` for a bare
/// unrecognized name or a malformed `(field)` suffix. The row carries one label per
/// declared channel; the caller folds each label of the set through this.
fn registration_from_label(label: &str) -> Option<Registration> {
    match label {
        "always" => return Some(Registration::Always),
        "user-invoked" => return Some(Registration::UserInvoked),
        _ => {}
    }
    let (name, field) = label.strip_suffix(')')?.split_once('(')?;
    let field = field.to_string();
    match name {
        "description-trigger" => Some(Registration::DescriptionTrigger { field }),
        "paths-match" => Some(Registration::PathsMatch { field }),
        "event" => Some(Registration::Event { field }),
        _ => None,
    }
}

/// Whether `glob` matches `name`, treating `*` as "any run of characters (including
/// empty)" and every other character literally — the minimal in-crate wildcard a
/// `governs` glob segment needs (`*.md`), short of pulling in a glob crate for one
/// metacharacter. Lives beside [`Governs`], the glob's home, so both `import`'s discovery
/// scan and a kind's own [`CustomKind::owns_source`] membership test share one matcher
/// (`.claude/rules/rust.md`). A standard linear matcher with single-star backtracking: on
/// a mismatch it falls back to the most recent `*`, extending what that star consumed by
/// one character. Matches one glob *segment*, not a `/`-path — the caller splits a
/// multi-segment glob and matches each part.
pub(crate) fn glob_matches(glob: &str, name: &str) -> bool {
    let pattern: Vec<char> = glob.chars().collect();
    let text: Vec<char> = name.chars().collect();
    let mut pi = 0;
    let mut ti = 0;
    // The position of the last `*` in `pattern`, and how much of `text` it had
    // consumed when we matched it — the backtrack point.
    let mut star: Option<usize> = None;
    let mut star_ti = 0;
    while ti < text.len() {
        if pi < pattern.len() && pattern[pi] == text[ti] {
            pi += 1;
            ti += 1;
        } else if pi < pattern.len() && pattern[pi] == '*' {
            star = Some(pi);
            star_ti = ti;
            pi += 1;
        } else if let Some(star_pi) = star {
            // Mismatch under an open `*`: let the star swallow one more character.
            pi = star_pi + 1;
            star_ti += 1;
            ti = star_ti;
        } else {
            return false;
        }
    }
    // Trailing `*`s match the empty remainder.
    while pi < pattern.len() && pattern[pi] == '*' {
        pi += 1;
    }
    pi == pattern.len()
}

/// A custom kind's composed extractor: an ordered set of deterministic
/// [`Primitive`]s over the closed algebra. Run over a [`Unit`] with
/// [`Extraction::extract`] it yields the [`Features`] a contract validates —
/// re-running over the same unit is byte-identical, because every primitive is a
/// pure function of the surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extraction {
    /// The composed primitives, in declaration order. An empty set is a valid
    /// (vacuous) extractor — it yields only the intrinsic `id`, everything else
    /// at its default (no fields, zero lines, no headings, no placement).
    primitives: Vec<Primitive>,
}

/// A single extraction primitive from the closed vocabulary. Each names a locus
/// on the surface and the feature it yields — every one *deterministically
/// extractable*, so a clause over its feature is a true positive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    /// `field` — project the frontmatter value at the `key` **key-path** into the
    /// named field feature (kind-preserving). A dotted path (`a.b.c`) walks nested
    /// tables to the leaf (`extract::resolve_key_path`); a bare key is the flat
    /// lookup. Unresolved (a missing segment, or a scalar met before the leaf) ⇒ the
    /// feature is not yielded — absent, never errored — mirroring how a skill's
    /// optional `version` is omitted when unset.
    Field {
        /// The frontmatter key-path read, and the name the feature is keyed by (the
        /// whole dotted path, so a clause references the nested field as `a.b.c`).
        key: String,
    },
    /// `headings` — the body's ATX headings, in document order
    /// (`Features::headings`).
    Headings,
    /// `sections` — the body's ATX sections (each heading + the body span beneath
    /// it), in document order (`Features::sections`) — the `## Decision`-block
    /// feature a `section_contains` clause decides over.
    Sections,
    /// `line_count` — the body's line count (`Features::body_lines`), the
    /// `max_lines` feature.
    LineCount,
    /// `placement` — the name of the directory the unit sits under
    /// (`Features::source_dir`) — file placement.
    Placement,
    /// `directives` — the body's format-executed directive occurrences for the
    /// named [`syntax`](DirectiveSyntax), folded into `Features::directives` in
    /// document order. Unlike the mining the `references` retirement bans, a directive
    /// is grammar the format authority documents as *executed*, so its occurrences are
    /// observed structure, not typography.
    Directives {
        /// The directive syntax extracted — the closed per-syntax vocabulary, sole
        /// member `at-import`.
        syntax: DirectiveSyntax,
    },
    /// `fenced` — the body's fenced code blocks (`Features::fenced_blocks`), in
    /// document order, each block's info string paired with its interior content.
    /// Markdown structure, deterministically extractable like
    /// `headings`/`sections`: the same fence boundaries, surfaced whole. Its first
    /// consumer is the member fence — fenced extraction composed with a TOML parse;
    /// this primitive yields the raw blocks only.
    Fenced,
}

/// A directive's format-executed body syntax — the closed per-syntax vocabulary the
/// [`Directives`](Primitive::Directives) primitive ranges over.
/// The
/// sole harvested member is [`AtImport`](DirectiveSyntax::AtImport); any other value
/// is a load error, the closed-vocabulary guard the primitive discriminator carries
/// applied to the per-syntax face.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectiveSyntax {
    /// `at-import` — an `@path/to/file` occurrence imports the target file into
    /// context (documented for Claude Code memory files, resolved relative to the
    /// importing file, absolute allowed; code.claude.com/docs/en/memory, retrieved
    /// 2026-07-02).
    AtImport,
}

impl Primitive {
    /// This primitive's TOML `primitive` discriminator — the key it is parsed
    /// from, reused as the vocabulary name a diagnostic reports.
    #[must_use]
    pub fn key(&self) -> &'static str {
        match self {
            Primitive::Field { .. } => "field",
            Primitive::Headings => "headings",
            Primitive::Sections => "sections",
            Primitive::LineCount => "line_count",
            Primitive::Placement => "placement",
            Primitive::Directives { .. } => "directives",
            Primitive::Fenced => "fenced",
        }
    }

    /// Apply this primitive to `unit`, folding its one feature into `features`.
    /// Deterministic and side-effect-free over the surface, so the composed
    /// extractor is too.
    fn apply(&self, unit: &Unit, features: &mut Features) {
        match self {
            Primitive::Field { key } => {
                // Walk the dotted key-path to its leaf (a flat lookup for a single
                // segment); absent — not errored — when the path doesn't resolve.
                if let Some(value) = extract::resolve_key_path(&unit.frontmatter, key) {
                    features
                        .fields
                        .insert(key.clone(), extract::json_to_feature(value));
                }
            }
            Primitive::Headings => features.headings = extract::body_headings(&unit.body),
            Primitive::Sections => features.sections = extract::body_sections(&unit.body),
            Primitive::LineCount => features.body_lines = extract::body_line_count(&unit.body),
            Primitive::Placement => {
                features.source_dir = extract::source_dir_name(&unit.source_path)
            }
            Primitive::Directives { syntax } => match syntax {
                DirectiveSyntax::AtImport => {
                    features.directives = extract::body_at_imports(&unit.body)
                }
            },
            Primitive::Fenced => features.fenced_blocks = extract::body_fenced_blocks(&unit.body),
        }
    }
}

/// A raw markdown unit the composed extractor reads: the intrinsic identity plus
/// the three surface loci the primitives range over (parsed frontmatter, the
/// byte-faithful body, the source placement). Frontmatter is *already parsed* —
/// splitting it is the surface tier's job and varies per harness format
/// (`crate::frontmatter` vs a frontmatter-less spec), so this composer takes the
/// values rather than re-parse. A spec supplies an empty `frontmatter`.
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    /// The artifact id used in diagnostics and as `Features::id` (a file stem, a
    /// skill's `name`). Intrinsic to the unit, never a composed primitive.
    pub id: String,
    /// The parsed frontmatter values by key — the `field` primitive's locus.
    /// Empty for a frontmatter-less kind.
    pub frontmatter: BTreeMap<String, JsonValue>,
    /// The byte-faithful markdown body (frontmatter stripped) — the locus for
    /// `headings`, `sections`, and `line_count`.
    pub body: String,
    /// The source path the unit was read from — the `placement` locus.
    pub source_path: PathBuf,
    /// The requirements this unit opts into filling — the authored
    /// `[satisfies.<requirement>]` header modules. A
    /// representation edge the coverage check resolves, not a composed feature:
    /// intrinsic to the surface, threaded through unchanged so a custom-kind member
    /// joins coverage exactly as a skill/rule does. Empty
    /// when the member authors none.
    pub satisfies: Vec<String>,
    /// The same `[satisfies.<requirement>]` opt-ins **with their authored rationale**
    /// — the whole [`Satisfies`] clause,
    /// not just the name coverage reads. The read family (`why`/`requirements`) narrates
    /// the *why* a custom member fills a requirement (READ-CUSTOM-SATISFIERS), so it
    /// needs the rationale the decidable [`satisfies`](Unit::satisfies) name-vec drops.
    /// Populated from the same header parse (`crate::document::satisfies`); empty when
    /// the member authors none.
    pub satisfies_clauses: Vec<crate::document::Satisfies>,
    /// The requirements this unit **publishes** — the authored `[requirement.<name>]`
    /// header modules. The demand side of the fill edge, threaded
    /// through unchanged so a custom-kind member (an intent `spec`) publishes into the
    /// one requirement namespace exactly as the assembly does. Empty when the member
    /// publishes none.
    pub published_requirements: Vec<PublishedRequirement>,
}

impl Unit {
    /// Reload a written custom-unit surface `<root>/<name>/` into a raw [`Unit`]:
    /// the id is the surface directory name, and its lone `.md` sibling is the
    /// member document — a `+++`-fenced `[provenance]`
    /// header over the byte-faithful body, whose `source_path` `import` wrote
    /// (`src/import.rs`, `import_custom_unit`).
    ///
    /// The generic inverse of that projection: keyed on the surface shape every
    /// custom kind shares (a lone member document found by extension), not on any
    /// one kind's IR, so it is the sole reader `check`'s custom-kind path uses and a
    /// kind rooted at any `governs.root` — not just `specs/` — is read.
    /// The `[clause.<field>]` header values are lifted
    /// into `frontmatter`, so the `field` primitive ranges over a custom member's
    /// declared fields exactly as it does a built-in's parsed frontmatter;
    /// a member carrying no clause tables reloads with empty
    /// frontmatter, its whole source file preserved in the body. An unreadable or
    /// malformed surface is a [`KindError`], never a silent skip.
    pub fn from_surface_dir(dir: &Path) -> Result<Self, KindError> {
        let doc_path = lone_body_file(dir)?;
        Self::from_member_document(dir, &doc_path)
    }

    /// Reload a surface member from an explicit member document `doc_path` under the
    /// surface directory `dir`, sharing the whole parse [`from_surface_dir`] runs.
    ///
    /// [`from_surface_dir`](Unit::from_surface_dir) finds the member document by the
    /// lone-`.md` convention every custom kind's surface shares; a **built-in** kind
    /// whose surface may carry markdown companions (a skill's `PLAYBOOK.md`) names its
    /// own member document instead — `SKILL.md`, `RULE.md` — so the companion never
    /// confuses the read. Both faces then read the surface through this one path:
    /// the `[clause.*]` header lifts into `frontmatter`,
    /// `[satisfies.*]`/`[requirement.*]` into the edge sets,
    /// the body byte-faithful. The id is the surface directory name — the member's
    /// home, never a field it sets.
    ///
    /// # Errors
    ///
    /// Returns a [`KindError`] when the document is unreadable, is not a well-formed
    /// `+++`-fenced document, or carries no `[provenance]` — the same hard failures
    /// [`from_surface_dir`](Unit::from_surface_dir) raises, never a silent skip.
    pub fn from_member_document(dir: &Path, doc_path: &Path) -> Result<Self, KindError> {
        let id = dir
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_string)
            .ok_or_else(|| KindError::SurfaceMissingField {
                path: dir.to_path_buf(),
                field: "name",
            })?;

        let raw = std::fs::read_to_string(doc_path).map_err(|source| KindError::Io {
            path: doc_path.to_path_buf(),
            source,
        })?;
        let document = Document::parse(&raw).map_err(|source| KindError::Document {
            path: doc_path.to_path_buf(),
            source,
        })?;
        let (source_path, _source_hash) = crate::document::provenance(document.header())
            .ok_or_else(|| KindError::SurfaceMissingField {
                path: doc_path.to_path_buf(),
                field: "provenance",
            })?;

        // The rationale-carrying clauses are read whole: coverage feeds off the
        // requirement name alone (the per-clause `rationale` is the human *why*, never
        // a decidable feature), while the read family narrates the rationale too
        // (READ-CUSTOM-SATISFIERS). One parse, both consumers.
        let satisfies_clauses = crate::document::satisfies(document.header());
        let satisfies = satisfies_clauses
            .iter()
            .map(|s| s.requirement.clone())
            .collect();

        // The demand side: `[requirement.*]` modules the member publishes, carried
        // through unchanged into the one namespace the gate unions.
        let published_requirements =
            crate::document::requirements(document.header()).map_err(|source| {
                KindError::Document {
                    path: doc_path.to_path_buf(),
                    source,
                }
            })?;

        // The `[clause.<field>]` header values are the member's typed fields — lift
        // each into `frontmatter` so the `field` primitive ranges over a custom member
        // exactly as it does a built-in's parsed frontmatter. A
        // clause whose `value` is JSON-null-unrepresentable is dropped, never invented.
        let frontmatter = crate::document::clauses(document.header())
            .into_iter()
            .filter_map(|(field, value)| {
                crate::document::item_to_json(value).map(|json| (field, json))
            })
            .collect();

        Ok(Self {
            id,
            frontmatter,
            body: document.body().to_string(),
            source_path: PathBuf::from(source_path),
            satisfies,
            satisfies_clauses,
            published_requirements,
        })
    }
}

/// The lone `.md` member document in a custom-unit surface directory — the
/// `+++`-fenced document `import` writes (`<KIND>.md`; `src/import.rs`). Selected by
/// extension rather than by the kind's own upper-cased name, so the reader stays
/// generic over every custom kind. Exactly one is required: zero (no document) or
/// more than one (an ambiguous surface) is a [`KindError::SurfaceBody`].
fn lone_body_file(dir: &Path) -> Result<PathBuf, KindError> {
    let listing = std::fs::read_dir(dir).map_err(|source| KindError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    let mut bodies = Vec::new();
    for entry in listing {
        let entry = entry.map_err(|source| KindError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            bodies.push(path);
        }
    }
    match bodies.len() {
        1 => Ok(bodies.remove(0)),
        found => Err(KindError::SurfaceBody {
            dir: dir.to_path_buf(),
            found,
        }),
    }
}

/// Errors raised while reloading a written surface unit ([`Unit::from_surface_dir`],
/// [`Unit::from_member_document`]). Hard failures — distinct from a lint finding,
/// which the check engine collects rather than throws.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum KindError {
    /// The surface document could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::kind::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A written custom-unit surface's member document is not a well-formed
    /// `+++`-fenced document (missing or unterminated fence, or a malformed TOML
    /// header). Reloading parses the document `import` wrote, so a malformed one is a
    /// hard error, never a silent skip.
    #[error("{path}: {source}")]
    #[diagnostic(code(temper::kind::bad_document))]
    Document {
        /// The surface document that failed to parse.
        path: PathBuf,
        /// The underlying fenced-document parse error.
        #[source]
        source: crate::document::DocumentError,
    },

    /// A written custom-unit surface is missing a required part — its directory
    /// name, its `[provenance]` table, or the `source_path` inside that table.
    /// Reloading is the inverse of the projection `import` writes, so a surface
    /// missing what `import` always writes is malformed, never a silent skip.
    #[error("{path}: custom-unit surface is missing required field `{field}`")]
    #[diagnostic(code(temper::kind::surface_missing_field))]
    SurfaceMissingField {
        /// The surface (its directory, or its `meta.toml`) whose part is absent.
        path: PathBuf,
        /// The required field that was absent.
        field: &'static str,
    },

    /// A written custom-unit surface does not carry exactly one `.md` member document
    /// — the `+++`-fenced document the extractor reads (`src/import.rs`,
    /// `import_custom_unit`). Zero (no document) or more than one (an ambiguous
    /// surface) is malformed.
    #[error(
        "{dir}: custom-unit surface must carry exactly one `.md` member document (found {found})"
    )]
    #[diagnostic(code(temper::kind::surface_body))]
    SurfaceBody {
        /// The surface directory whose body is missing or ambiguous.
        dir: PathBuf,
        /// How many `.md` bodies were found (never exactly one).
        found: usize,
    },
}

impl Extraction {
    /// Compose an extractor directly from its ordered [`Primitive`]s — the
    /// constructor a kind's declared definition supplies (`crate::builtin_kind`, a
    /// future SDK-authored custom kind). There is no `[[extraction]]` file grammar
    /// to parse it from; an empty vec is the valid
    /// vacuous extractor.
    #[must_use]
    pub fn new(primitives: Vec<Primitive>) -> Self {
        Self { primitives }
    }

    /// The composed primitives, in declaration order.
    #[must_use]
    pub fn primitives(&self) -> &[Primitive] {
        &self.primitives
    }

    /// Run the composed extractor over a raw markdown `unit`, folding each
    /// primitive's one feature into a [`Features`]. The intrinsic `id` is always
    /// set; every other feature stays at its default until a primitive yields it.
    /// A pure function of the surface — re-running over the same unit is
    /// byte-identical, which is what makes the feature a sound gate input.
    #[must_use]
    pub fn extract(&self, unit: &Unit) -> Features {
        let mut features = Features {
            id: unit.id.clone(),
            fields: BTreeMap::new(),
            body_lines: 0,
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: None,
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            // Nested members are folded by [`CustomKind::extract`] after the primitives
            // run — a typed layer over `fenced_blocks`, needing the kind's declared
            // template set the primitive-only `Extraction` does not hold. Empty here on
            // purpose.
            nested_members: Vec::new(),
            // `satisfies` is a surface edge threaded through unchanged, not a
            // composed primitive, so a custom-kind member joins coverage exactly as
            // a built-in kind's does.
            satisfies: unit.satisfies.clone(),
            // The demand side rides through the same way — a published `[requirement.*]`
            // is authored surface state, never a composed feature.
            published_requirements: unit.published_requirements.clone(),
        };
        for primitive in &self.primitives {
            primitive.apply(unit, &mut features);
        }
        features
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::{FeatureValue, ValueType};

    /// The composed `spec`-shaped extractor the worked example needs:
    /// line count, ATX headings, and file placement —
    /// markdown structure only, no body-mined references (the `references`
    /// primitive is retired; the corpus's edges are declared in member headers).
    fn spec_extraction() -> Extraction {
        Extraction::new(vec![
            Primitive::LineCount,
            Primitive::Headings,
            Primitive::Placement,
        ])
    }

    /// A raw spec-shaped unit: no frontmatter, a body carrying two headings and a
    /// filename inside a fenced block (which heading/line-count extraction skips).
    fn spec_unit() -> Unit {
        let body = "# Kinds\n\
\n\
## The extraction algebra\n\
\n\
Composed like `15-kinds.md` over `10-contracts.md`.\n\
\n\
```text\n\
`inside-a-fence.md` is illustration, not a heading\n\
```\n";
        Unit {
            id: "15-kinds".to_string(),
            frontmatter: BTreeMap::new(),
            body: body.to_string(),
            source_path: PathBuf::from("specs/architecture/15-kinds.md"),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
            published_requirements: Vec::new(),
        }
    }

    #[test]
    fn composes_and_extracts_a_raw_unit_into_features() {
        let extraction = spec_extraction();
        let features = extraction.extract(&spec_unit());

        // The intrinsic id is always the unit id.
        assert_eq!(features.id, "15-kinds");

        // `line_count` — the whole body, counted the same way a spec projector does.
        assert_eq!(features.body_lines, 9);

        // `headings` — ATX headings in order (the fenced content is not a heading).
        assert_eq!(
            features.headings,
            vec!["Kinds".to_string(), "The extraction algebra".to_string()]
        );

        // `placement` — the folder the unit sits under (the class directory).
        assert_eq!(features.source_dir.as_deref(), Some("architecture"));

        // A frontmatter-less kind composes no `field`, and body-mined references are
        // retired — nothing lands in `fields`.
        assert!(features.fields.is_empty());
    }

    #[test]
    fn re_running_the_extractor_is_byte_identical() {
        let extraction = spec_extraction();
        let unit = spec_unit();

        // Extraction is a pure function of the surface — the soundness boundary:
        // the same unit yields the same features every run.
        let first = extraction.extract(&unit);
        let second = extraction.extract(&unit);
        assert_eq!(first, second);
    }

    #[test]
    fn a_field_primitive_projects_frontmatter_kind_preserving() {
        let extraction = Extraction::new(vec![
            Primitive::Field {
                key: "name".to_string(),
            },
            Primitive::Field {
                key: "priority".to_string(),
            },
        ]);

        let mut frontmatter = BTreeMap::new();
        frontmatter.insert("name".to_string(), JsonValue::String("demo".to_string()));
        frontmatter.insert("priority".to_string(), JsonValue::from(7));
        let unit = Unit {
            id: "demo".to_string(),
            frontmatter,
            body: "# Demo\n".to_string(),
            source_path: PathBuf::from("skills/demo/SKILL.md"),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
            published_requirements: Vec::new(),
        };

        let features = extraction.extract(&unit);

        // The frontmatter value is projected through the shared kind-preserving
        // projector: a string stays `string`, an integer keeps `integer`.
        assert_eq!(
            features.field("name"),
            Some(&FeatureValue::scalar(ValueType::String, "demo"))
        );
        assert_eq!(
            features.field("priority").map(FeatureValue::kind),
            Some(ValueType::Integer)
        );
        // The body loci are untouched — this extractor composes only `field`.
        assert_eq!(features.body_lines, 0);
        assert!(features.headings.is_empty());
    }

    #[test]
    fn a_fenced_primitive_parses_and_folds_block_interiors_into_features() {
        // `fenced` is a closed-vocab, parameterless primitive — it parses into
        // `Primitive::Fenced` and folds the body's fenced blocks into `fenced_blocks`,
        // each interior paired with its info string, surrounding prose skipped.
        let extraction = Extraction::new(vec![Primitive::Fenced]);
        assert_eq!(extraction.primitives(), &[Primitive::Fenced]);

        let body = "# Doc\n\nprose\n\n```toml member.manifest\nname = \"x\"\n```\n";
        let unit = Unit {
            id: "doc".to_string(),
            frontmatter: BTreeMap::new(),
            body: body.to_string(),
            source_path: PathBuf::from("specs/architecture/15-kinds.md"),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
            published_requirements: Vec::new(),
        };
        let features = extraction.extract(&unit);
        assert_eq!(features.fenced_blocks.len(), 1);
        assert_eq!(features.fenced_blocks[0].info, "toml member.manifest");
        assert_eq!(features.fenced_blocks[0].content, "name = \"x\"");
        // This extractor composes only `fenced` — every other locus stays at its
        // default (no headings extracted, no fields), the vacuous-composition floor.
        assert!(features.headings.is_empty());
        assert!(features.fields.is_empty());
    }

    #[test]
    fn a_field_absent_from_the_unit_is_not_yielded() {
        let extraction = Extraction::new(vec![Primitive::Field {
            key: "license".to_string(),
        }]);
        let unit = Unit {
            id: "demo".to_string(),
            frontmatter: BTreeMap::new(),
            body: String::new(),
            source_path: PathBuf::from("skills/demo/SKILL.md"),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
            published_requirements: Vec::new(),
        };
        // A key the unit does not carry yields no feature — never a phantom entry.
        assert!(extraction.extract(&unit).field("license").is_none());
    }

    #[test]
    fn an_empty_declaration_is_a_vacuous_extractor() {
        let extraction = Extraction::new(Vec::new());
        assert!(extraction.primitives().is_empty());

        let unit = spec_unit();
        let features = extraction.extract(&unit);
        // Only the intrinsic id; every composed feature stays at its default.
        assert_eq!(features.id, "15-kinds");
        assert_eq!(features.body_lines, 0);
        assert!(features.headings.is_empty());
        assert!(features.source_dir.is_none());
        assert!(features.fields.is_empty());
    }

    /// A fresh, empty temp directory unique to this call.
    fn surface_tmpdir(label: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-kind-surface-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Write a `<root>/<name>/<BODY>.md` surface exactly as `import` projects a
    /// custom-kind unit: ONE member document — a provenance-only `+++` header over
    /// the whole body. Returns the surface directory.
    fn write_surface(
        root: &Path,
        name: &str,
        source_path: &str,
        body_name: &str,
        body: &str,
    ) -> PathBuf {
        let dir = root.join(name);
        std::fs::create_dir_all(&dir).unwrap();
        let document = format!(
            "+++\n[provenance]\nsource_path = \"{source_path}\"\nsource_hash = \"deadbeef\"\n+++\n{body}"
        );
        std::fs::write(dir.join(body_name), document).unwrap();
        dir
    }

    #[test]
    fn from_surface_dir_reloads_a_written_unit_for_any_root() {
        // The root is `docs/adr`, not `specs` — the reader keys on the surface
        // shape, never a hardwired `specs` special case, so a kind rooted anywhere
        // reloads the same way.
        let root = surface_tmpdir("adr-root").join("docs").join("adr");
        let body = "# ADR 0001\n\nContext refers to `15-kinds.md`.\n";
        let dir = write_surface(
            &root,
            "0001-use-kinds",
            "docs/adr/0001-use-kinds.md",
            "ADR.md",
            body,
        );

        let unit = Unit::from_surface_dir(&dir).unwrap();

        // id is the surface directory name.
        assert_eq!(unit.id, "0001-use-kinds");
        // body is the lone `.md` sibling, byte-faithful.
        assert_eq!(unit.body, body);
        // source_path is read back from the `[provenance]` table.
        assert_eq!(
            unit.source_path,
            PathBuf::from("docs/adr/0001-use-kinds.md")
        );
        // A generic surface reload carries no frontmatter — the whole file is body.
        assert!(unit.frontmatter.is_empty());
    }

    #[test]
    fn from_surface_dir_feeds_the_composed_extractor() {
        // The reloaded unit is exactly what a kind's composed extractor reads: the
        // spec-shaped extractor over it yields the same features it would over a
        // freshly-parsed unit — the tie between the generic loader and the check path.
        let root = surface_tmpdir("feed-root").join("specs");
        let body = "# Kinds\n\nComposed over the predicate half.\n";
        let dir = write_surface(
            &root,
            "15-kinds",
            "specs/architecture/15-kinds.md",
            "SPEC.md",
            body,
        );

        let unit = Unit::from_surface_dir(&dir).unwrap();
        let features = spec_extraction().extract(&unit);

        assert_eq!(features.id, "15-kinds");
        assert_eq!(features.body_lines, 3);
        assert_eq!(features.headings, vec!["Kinds".to_string()]);
        assert_eq!(features.source_dir.as_deref(), Some("architecture"));
        // The composed `spec` extractor mines no references — `fields` stays empty.
        assert!(features.fields.is_empty());
    }

    #[test]
    fn from_surface_dir_lifts_clause_fields_into_frontmatter() {
        // A custom member carrying `[clause.<field>]` header tables reloads with those
        // fields in `frontmatter` — the generic reader that closes the built-in/custom
        // asymmetry: a custom member's declared fields are the
        // `field` primitive's locus, like a built-in's parsed frontmatter.
        let root = surface_tmpdir("clause-fields").join("specs");
        let dir = root.join("15-kinds");
        std::fs::create_dir_all(&dir).unwrap();
        let document = "+++\n\
[clause.name]\n\
value = \"15-kinds\"\n\
[clause.priority]\n\
value = 7\n\
[provenance]\n\
source_path = \"specs/architecture/15-kinds.md\"\n\
source_hash = \"deadbeef\"\n\
+++\n\
# Kinds\n\nBody.\n";
        std::fs::write(dir.join("SPEC.md"), document).unwrap();

        let unit = Unit::from_surface_dir(&dir).unwrap();

        // The clause values land in `frontmatter`, JSON-kind-faithful: a string stays
        // a string, a bare integer stays an integer.
        assert_eq!(
            unit.frontmatter.get("name"),
            Some(&JsonValue::String("15-kinds".to_string()))
        );
        assert_eq!(unit.frontmatter.get("priority"), Some(&JsonValue::from(7)));

        // And they resolve through the composed `field` primitive exactly as a
        // built-in's parsed frontmatter does — the asymmetry closed.
        let extraction = Extraction::new(vec![
            Primitive::Field {
                key: "name".to_string(),
            },
            Primitive::Field {
                key: "priority".to_string(),
            },
        ]);
        let features = extraction.extract(&unit);
        assert_eq!(
            features.field("name"),
            Some(&FeatureValue::scalar(ValueType::String, "15-kinds"))
        );
        assert_eq!(
            features.field("priority").map(FeatureValue::kind),
            Some(ValueType::Integer)
        );
    }

    #[test]
    fn from_surface_dir_with_no_clause_tables_yields_empty_frontmatter() {
        // A member document with no `[clause.<field>]` tables (only provenance) reloads
        // with empty frontmatter — the built-in floor's default, unchanged from before
        // this reader existed.
        let root = surface_tmpdir("no-clauses").join("specs");
        let dir = write_surface(
            &root,
            "00-intent",
            "specs/intent.md",
            "SPEC.md",
            "# Intent\n\nBody.\n",
        );

        let unit = Unit::from_surface_dir(&dir).unwrap();
        assert!(unit.frontmatter.is_empty());
    }

    #[test]
    fn a_surface_missing_its_provenance_is_a_load_error() {
        let root = surface_tmpdir("no-prov");
        let dir = root.join("00-intent");
        std::fs::create_dir_all(&dir).unwrap();
        // A member document whose header carries no `[provenance]` module.
        std::fs::write(dir.join("SPEC.md"), "+++\n# no provenance\n+++\n# Intent\n").unwrap();

        let err = Unit::from_surface_dir(&dir).unwrap_err();
        assert!(matches!(
            err,
            KindError::SurfaceMissingField {
                field: "provenance",
                ..
            }
        ));
    }

    #[test]
    fn a_surface_with_a_malformed_document_is_a_load_error() {
        let root = surface_tmpdir("bad-doc");
        let dir = root.join("00-intent");
        std::fs::create_dir_all(&dir).unwrap();
        // The lone `.md` is not a `+++`-fenced document — a hard error, never a skip.
        std::fs::write(dir.join("SPEC.md"), "# no fence here\nbody\n").unwrap();

        let err = Unit::from_surface_dir(&dir).unwrap_err();
        assert!(matches!(err, KindError::Document { .. }));
    }

    #[test]
    fn a_surface_without_a_body_file_is_a_load_error() {
        let root = surface_tmpdir("no-body");
        let dir = root.join("00-intent");
        std::fs::create_dir_all(&dir).unwrap();
        // No `.md` member document at all — only a stray non-markdown sibling.
        std::fs::write(dir.join("notes.txt"), "not a document\n").unwrap();

        let err = Unit::from_surface_dir(&dir).unwrap_err();
        assert!(matches!(err, KindError::SurfaceBody { found: 0, .. }));
    }

    #[test]
    fn a_surface_with_two_body_files_is_ambiguous() {
        let root = surface_tmpdir("two-body");
        let dir = root.join("00-intent");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("SPEC.md"), "+++\n+++\n# One\n").unwrap();
        std::fs::write(dir.join("EXTRA.md"), "+++\n+++\n# Two\n").unwrap();

        let err = Unit::from_surface_dir(&dir).unwrap_err();
        assert!(matches!(err, KindError::SurfaceBody { found: 2, .. }));
    }

    /// A bare `spec` kind — the shape a built-in or SDK-authored custom kind
    /// constructs ([`CustomKind::new`]), no file format involved.
    fn spec_kind() -> CustomKind {
        CustomKind::new(
            "spec",
            Governs {
                root: "specs".to_string(),
                glob: "*.md".to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }

    #[test]
    fn new_constructs_a_kind_with_every_optional_fact_absent() {
        // The constructor's defaults — nothing invented until the caller sets it.
        let kind = spec_kind();
        assert_eq!(kind.format, None);
        assert_eq!(kind.unit_shape, None);
        assert!(kind.registration.is_empty());
        assert!(kind.relationships.is_empty());
        assert!(kind.templates.is_empty());
    }

    #[test]
    fn qualified_name_is_the_bare_name() {
        // Identity travels by import, never by string — a kind's qualified identity
        // is always its own bare name.
        assert_eq!(spec_kind().qualified_name(), "spec");
    }

    #[test]
    fn from_kind_fact_row_lifts_every_declared_fact() {
        let row = KindFactRow {
            name: "spec".to_string(),
            provider: None,
            governs_root: "specs".to_string(),
            governs_glob: "*.md".to_string(),
            format: Some("yaml-frontmatter".to_string()),
            unit_shape: Some("directory".to_string()),
            registration: vec!["description-trigger(description)".to_string()],
            templates: Vec::new(),
        };
        let kind = CustomKind::from_kind_fact_row(&row);

        assert_eq!(kind.name, "spec");
        assert_eq!(
            kind.governs,
            Governs {
                root: "specs".to_string(),
                glob: "*.md".to_string(),
            }
        );
        assert_eq!(kind.format, Some(Format::YamlFrontmatter));
        assert_eq!(kind.unit_shape, Some(UnitShape::Directory));
        assert_eq!(
            kind.registration,
            vec![Registration::DescriptionTrigger {
                field: "description".to_string()
            }]
        );
        // The generic markdown-structure set every built-in composes, plus `Fenced` —
        // never a per-kind `Field` primitive, since the row carries no field-level
        // facts.
        assert_eq!(
            kind.extraction.primitives(),
            &[
                Primitive::LineCount,
                Primitive::Headings,
                Primitive::Sections,
                Primitive::Placement,
                Primitive::Fenced,
            ]
        );
    }

    #[test]
    fn from_kind_fact_row_degrades_unrecognized_labels_to_absent() {
        // A hand-editable lock's out-of-vocabulary label degrades to absent rather
        // than erroring — the same tolerance the rest of the lock's readers take.
        let row = KindFactRow {
            name: "spec".to_string(),
            provider: None,
            governs_root: "specs".to_string(),
            governs_glob: "*.md".to_string(),
            format: Some("xml".to_string()),
            unit_shape: Some("directory".to_string()),
            registration: vec!["bogus".to_string()],
            templates: Vec::new(),
        };
        let kind = CustomKind::from_kind_fact_row(&row);
        assert_eq!(kind.format, None);
        assert!(kind.registration.is_empty());
    }

    #[test]
    fn from_kind_fact_row_drops_only_the_unrecognized_channel_from_a_mixed_set() {
        // A set carrying one recognized and one bogus label lifts the recognized
        // channel and silently drops the other — per-channel tolerance, not a
        // whole-set failure.
        let row = KindFactRow {
            name: "spec".to_string(),
            provider: None,
            governs_root: "specs".to_string(),
            governs_glob: "*.md".to_string(),
            format: None,
            unit_shape: None,
            registration: vec!["user-invoked".to_string(), "bogus".to_string()],
            templates: Vec::new(),
        };
        let kind = CustomKind::from_kind_fact_row(&row);
        assert_eq!(kind.registration, vec![Registration::UserInvoked]);
    }

    #[test]
    fn from_kind_fact_row_lifts_a_multi_channel_registration_set_in_order() {
        // `skill`'s own two-channel set — both labels lift, order preserved.
        let row = KindFactRow {
            name: "skill".to_string(),
            provider: None,
            governs_root: ".claude/skills".to_string(),
            governs_glob: "*/SKILL.md".to_string(),
            format: None,
            unit_shape: None,
            registration: vec![
                "user-invoked".to_string(),
                "description-trigger(description)".to_string(),
            ],
            templates: Vec::new(),
        };
        let kind = CustomKind::from_kind_fact_row(&row);
        assert_eq!(
            kind.registration,
            vec![
                Registration::UserInvoked,
                Registration::DescriptionTrigger {
                    field: "description".to_string()
                },
            ]
        );
    }

    #[test]
    fn from_kind_fact_row_with_no_optional_facts_yields_the_generic_defaults() {
        let row = KindFactRow {
            name: "adr".to_string(),
            provider: None,
            governs_root: "adr".to_string(),
            governs_glob: "*.md".to_string(),
            format: None,
            unit_shape: None,
            registration: Vec::new(),
            templates: Vec::new(),
        };
        let kind = CustomKind::from_kind_fact_row(&row);
        assert_eq!(kind.format, None);
        assert_eq!(kind.unit_shape, None);
        assert!(kind.registration.is_empty());
    }

    #[test]
    fn from_kind_fact_row_lifts_declared_templates_by_child_kind() {
        // Each recorded child-kind name lifts into a `Template` — `fold_members`
        // keys only on `Template.kind`.
        let row = KindFactRow {
            name: "spec".to_string(),
            provider: None,
            governs_root: "specs".to_string(),
            governs_glob: "*.md".to_string(),
            format: None,
            unit_shape: None,
            registration: Vec::new(),
            templates: vec!["decision".to_string(), "law".to_string()],
        };
        let kind = CustomKind::from_kind_fact_row(&row);
        assert_eq!(
            kind.templates,
            vec![
                Template {
                    kind: "decision".to_string(),
                },
                Template {
                    kind: "law".to_string(),
                },
            ]
        );
    }
}
