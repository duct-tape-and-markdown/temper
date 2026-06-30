//! The `Contract` artifact — the decidable artifact-clause algebra.
//!
//! Models the primitive algebra of `specs/10-contracts.md` ("The primitive
//! algebra (decidable only)"): a [`Contract`] is a *named set of clauses* over a
//! fixed, **closed** vocabulary of decidable predicates, loaded from a TOML
//! contract file. There is no arbitrary-code clause — adding a predicate is a
//! deliberate language change, never a per-contract escape hatch (`00-intent.md`
//! law 3). Loading therefore **rejects an unknown predicate key** rather than
//! skipping it silently: a contract is closed-vocabulary data, not data with a
//! trapdoor.
//!
//! Severity is author-declared, not tool-baked: every [`Clause`] carries a
//! [`Severity`] marking it `required` (gate-blocking) or `advisory` (reported,
//! non-blocking). The engine that *checks* a surface against these clauses lives
//! elsewhere; this module is the type system it checks against.
//!
//! ## Scope (this entry)
//!
//! The artifact-level primitives buildable in-crate, without a new dependency:
//! field presence (`required`/`optional`), `min_len`/`max_len`, `enum`, `deny`,
//! `forbidden_keys`, an in-crate `allowed_chars` charset (the `[a-z0-9-]` case);
//! structural `max_lines`, `require_sections`, `must_define`; and cross-artifact
//! `name-matches-dir`, `unique-name`, `dependency-exists`. The full `pattern`
//! (regex) field primitive is held behind the regex fork; the harness-contract
//! primitives (`role`, `verified_by`) belong to the harness-contract layer, not
//! here.
//!
//! ## Why hand-walk `toml_edit` instead of `serde` derive
//!
//! Parsing walks the `toml_edit` document by hand, mirroring [`crate::skill`].
//! A closed vocabulary keyed on a discriminator field is exactly the internally
//! tagged-enum shape the TOML deserializer handles poorly, and the diagnostics
//! *are* the product (`00-intent.md`): a precise "clause 3 names unknown
//! predicate `word_count`" beats a generic serde decode error.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use toml_edit::{DocumentMut, Item, Table};

/// A named set of clauses over the decidable primitive algebra — the type a
/// harness (or one artifact in it) is checked against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    /// The contract's name (e.g. `skill`), the identity other layers reference.
    pub name: String,
    /// The clauses, in declaration order. An empty set is a valid (vacuous)
    /// contract — a named shape that asserts nothing.
    pub clauses: Vec<Clause>,
}

/// One clause: a decidable [`Predicate`] plus the [`Severity`] its author
/// declared for it. Pairing the two here is the whole point — `temper` never
/// decides error-vs-warning; the contract does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    /// Whether a violation of this clause blocks the gate or is merely reported.
    pub severity: Severity,
    /// The decidable predicate this clause asserts over the surface.
    pub predicate: Predicate,
}

/// The author-declared weight of a clause. Replaces the tool-baked error/warn
/// split: the default gate blocks on `Required` clauses only, and a strict CI
/// policy can promote `Advisory` to blocking (`specs/10-contracts.md`,
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// `name-matches-dir`: the artifact's name equals its containing directory.
    NameMatchesDir,
    /// `unique-name`: names are unique within the artifact kind.
    UniqueName,
    /// `dependency-exists`: every declared dependency resolves.
    DependencyExists,
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
    /// The contract file could not be read.
    #[error("failed to read contract {path}")]
    #[diagnostic(code(temper::contract::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

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

    /// The top-level `name` key is absent or not a string.
    #[error("{path}: contract is missing required field `name`")]
    #[diagnostic(code(temper::contract::missing_name))]
    MissingName {
        /// The contract whose header is incomplete.
        path: PathBuf,
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
    /// Load and parse a contract from a TOML file on disk.
    pub fn load(path: &Path) -> Result<Self, ContractError> {
        let src = fs::read_to_string(path).map_err(|source| ContractError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        Self::parse(&src, path)
    }

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

        let name = table
            .get("name")
            .and_then(Item::as_str)
            .map(str::to_string)
            .ok_or_else(|| ContractError::MissingName {
                path: path.to_path_buf(),
            })?;

        let clauses = parse_clauses(table, path)?;
        Ok(Self { name, clauses })
    }
}

/// Parse the `[[clause]]` array of tables, in declaration order. Absent ⇒ no
/// clauses; present-but-not-an-array-of-tables ⇒ [`ContractError::ClauseNotArray`].
fn parse_clauses(table: &Table, path: &Path) -> Result<Vec<Clause>, ContractError> {
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

/// Parse one clause table into its typed severity + predicate.
fn parse_clause(table: &Table, index: usize, path: &Path) -> Result<Clause, ContractError> {
    let severity = parse_severity(table, index, path)?;
    let predicate = parse_predicate(table, index, path)?;
    Ok(Clause {
        severity,
        predicate,
    })
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
        "min_len" => Predicate::MinLen {
            field: str_param(table, "field", index, path)?,
            min: usize_param(table, "min", index, path)?,
        },
        "max_len" => Predicate::MaxLen {
            field: str_param(table, "field", index, path)?,
            max: usize_param(table, "max", index, path)?,
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
        "name-matches-dir" => Predicate::NameMatchesDir,
        "unique-name" => Predicate::UniqueName,
        "dependency-exists" => Predicate::DependencyExists,
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
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-contract-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

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
"#;

    /// The typed model `REP` must deserialize into — every primitive in the
    /// algebra, each pinned to the severity its clause declared.
    fn rep_expected() -> Contract {
        Contract {
            name: "skill".to_string(),
            clauses: vec![
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::Required {
                        field: "name".to_string(),
                    },
                },
                Clause {
                    severity: Severity::Advisory,
                    predicate: Predicate::Optional {
                        field: "version".to_string(),
                    },
                },
                Clause {
                    severity: Severity::Advisory,
                    predicate: Predicate::MinLen {
                        field: "description".to_string(),
                        min: 1,
                    },
                },
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::MaxLen {
                        field: "name".to_string(),
                        max: 64,
                    },
                },
                Clause {
                    severity: Severity::Advisory,
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
                    severity: Severity::Required,
                    predicate: Predicate::Deny {
                        field: "name".to_string(),
                        values: vec!["anthropic".to_string(), "claude".to_string()],
                    },
                },
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::ForbiddenKeys {
                        keys: vec!["globs".to_string(), "alwaysApply".to_string()],
                    },
                },
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::AllowedChars {
                        field: "name".to_string(),
                        charset: Charset {
                            ranges: vec![('a', 'z'), ('0', '9')],
                            chars: BTreeSet::from(['-']),
                        },
                    },
                },
                Clause {
                    severity: Severity::Advisory,
                    predicate: Predicate::MaxLines { max: 500 },
                },
                Clause {
                    severity: Severity::Advisory,
                    predicate: Predicate::RequireSections {
                        sections: vec!["Usage".to_string(), "Examples".to_string()],
                    },
                },
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::MustDefine {
                        marker: "disable-model-invocation".to_string(),
                    },
                },
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::NameMatchesDir,
                },
                Clause {
                    severity: Severity::Required,
                    predicate: Predicate::UniqueName,
                },
                Clause {
                    severity: Severity::Advisory,
                    predicate: Predicate::DependencyExists,
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
    fn load_reads_a_contract_from_disk() {
        let dir = tmpdir("load");
        let path = dir.join("skill.contract.toml");
        fs::write(&path, REP).unwrap();

        let contract = Contract::load(&path).unwrap();
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
    fn missing_name_is_a_load_error() {
        let toml = r#"
[[clause]]
severity = "required"
predicate = "name-matches-dir"
"#;
        let err = Contract::parse(toml, Path::new("c.toml")).unwrap_err();
        assert!(matches!(err, ContractError::MissingName { .. }));
    }

    #[test]
    fn a_contract_with_no_clauses_is_vacuously_valid() {
        let contract = Contract::parse("name = \"empty\"\n", Path::new("c.toml")).unwrap();
        assert_eq!(contract.name, "empty");
        assert!(contract.clauses.is_empty());
    }
}
