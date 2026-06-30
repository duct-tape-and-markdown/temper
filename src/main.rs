//! `temper` CLI entry point.
//!
//! Thin command dispatch over the [`temper`] library. The subcommands mirror the
//! surface in `specs/20-surface.md` ("CLI surface"): `import` scans a harness
//! into the typed config surface, `check` runs **both greens** of
//! `specs/10-contracts.md` — *admissibility* (each built-in contract is itself
//! valid against the definition) and *conformance* (each artifact satisfies its
//! contract) — and exits non-zero when either an inadmissible contract or a
//! `required`-severity conformance clause is violated. All logic lives in the
//! library — `main` only parses args, projects the workspace into the engine's
//! [`Features`] view, runs the generic contract engine (`specs/10-contracts.md`),
//! and maps the result to an exit code.
//!
//! [`Features`]: temper::extract::Features

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use temper::check::{self, Severity, Workspace};
use temper::compose;
use temper::contract::Contract;
use temper::engine;
use temper::extract;
use temper::import;
use temper::roster;

/// The surface workspace default for `--into` / the `check` argument: a `.temper`
/// directory under the current working directory (`specs/20-surface.md`).
const DEFAULT_WORKSPACE: &str = "./.temper";

/// The optional author-declared contract layer, discovered at the project root —
/// the invocation directory, beside the harness it governs (`specs/40-composition.md`,
/// "The author-declared contract — `temper.toml`"). Absent ⇒ the by-kind floor
/// runs unchanged.
const TEMPER_TOML: &str = "temper.toml";

/// The built-in Anthropic skill contract — the curated "std-lib" default
/// (`contracts/skill.anthropic.toml`), embedded at build time so `check` has a
/// contract to validate against without any on-disk configuration.
const BUILTIN_SKILL_CONTRACT: &str = include_str!("../contracts/skill.anthropic.toml");

/// The built-in rule contract — the curated default for the `rule` artifact kind
/// (`contracts/rule.toml`), embedded beside the skill one so `check` validates
/// each artifact against the contract for *its* kind without any on-disk config
/// (`specs/20-surface.md`, "contract selection is by artifact kind").
const BUILTIN_RULE_CONTRACT: &str = include_str!("../contracts/rule.toml");

/// A typed maintenance surface for the Claude Code harness.
#[derive(Parser)]
#[command(name = "temper", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scan the harness into the typed config surface (+ provenance lock).
    Import {
        /// The harness to scan: a `skills/*/SKILL.md` tree, or a bare skill dir.
        harness_path: PathBuf,
        /// Where to write the surface workspace (defaults to `./.temper`).
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
    },
    /// Lint the config surface against the active contract.
    Check {
        /// The surface workspace to lint (defaults to `./.temper`).
        workspace: Option<PathBuf>,
        /// Also fail the run on `advisory` (warn-severity) violations, not just
        /// `required` ones — the strict CI policy from `specs/10-contracts.md`.
        #[arg(long)]
        deny_advisories: bool,
    },
}

fn main() -> miette::Result<ExitCode> {
    match Cli::parse().command {
        Command::Import { harness_path, into } => {
            import::run(&harness_path, &into)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Check {
            workspace,
            deny_advisories,
        } => {
            let workspace = workspace.unwrap_or_else(|| PathBuf::from(DEFAULT_WORKSPACE));
            let ws = Workspace::load(&workspace)?;

            // The optional author-declared layer at the project root. Absent ⇒
            // `None` and the floor runs verbatim (every existing test's path);
            // present ⇒ it layers over the by-kind floor per kind below
            // (`specs/40-composition.md`, the `temper.toml` Decision).
            let layer = compose::AuthorLayer::load(Path::new(TEMPER_TOML))?;

            // Dispatch by artifact kind: each kind's features are validated
            // against the *effective* contract for its kind — the embedded floor
            // with the author layer applied — and the findings are merged into one
            // diagnostic set (`specs/20-surface.md`, "contract selection is by
            // artifact kind"). The generic engine holds no per-kind opinion — each
            // contract carries its own clauses, so a mixed harness (skills *and*
            // rules) is judged correctly in one run.
            let skill_features: Vec<extract::Features> =
                ws.skills.iter().map(extract::skill_features).collect();
            let skill_floor =
                Contract::parse(BUILTIN_SKILL_CONTRACT, Path::new("skill.anthropic.toml"))?;
            let skill_contract = compose::effective(layer.as_ref(), "skill", skill_floor)?;

            let rule_features: Vec<extract::Features> =
                ws.rules.iter().map(extract::rule_features).collect();
            let rule_floor = Contract::parse(BUILTIN_RULE_CONTRACT, Path::new("rule.toml"))?;
            let rule_contract = compose::effective(layer.as_ref(), "rule", rule_floor)?;

            // Two greens, not one (`specs/10-contracts.md`, both-greens finish
            // line). **Admissibility** first: each built-in contract is itself
            // validated against the definition before it is trusted to judge a
            // harness — an inadmissible contract fails the run exactly as a
            // `required` conformance violation does. **Conformance** second: each
            // artifact is checked against the contract for its kind. Both sets of
            // findings merge into one rendered diagnostic stream.
            let mut diagnostics = engine::admissibility(&skill_contract);
            diagnostics.extend(engine::admissibility(&rule_contract));
            diagnostics.extend(engine::validate(&skill_contract, &skill_features));
            diagnostics.extend(engine::validate(&rule_contract, &rule_features));

            // The harness-contract tier: run role match-selection over the parsed
            // roster, gating each `required` single-filler role on being filled by
            // exactly one artifact of its kind (`specs/10-contracts.md`, "Roles and
            // matching"). Absent `temper.toml` ⇒ no layer ⇒ this adds nothing, so
            // the floor-only path stays byte-for-byte unchanged.
            if let Some(layer) = layer.as_ref() {
                let by_kind: std::collections::BTreeMap<&str, &[extract::Features]> =
                    std::collections::BTreeMap::from([
                        ("skill", skill_features.as_slice()),
                        ("rule", rule_features.as_slice()),
                    ]);
                diagnostics.extend(roster::check(layer.roles(), &by_kind));

                // The `conforms-to` half of the same tier: each role's selected
                // filler(s) are validated against the role's resolved contract —
                // its inline clauses, or a template path taken relative to the
                // `temper.toml` directory — with findings retagged under
                // `role.conforms-to` (`specs/10-contracts.md`, the `role`
                // primitive). A non-resolving template is the roster-admissibility
                // entry's finding, skipped here rather than double-reported.
                let base_dir = Path::new(TEMPER_TOML)
                    .parent()
                    .unwrap_or_else(|| Path::new("."));
                diagnostics.extend(roster::conformance(layer.roles(), &by_kind, base_dir));
            }

            print!("{}", check::render(&diagnostics));

            // A `required` violation always fails the run; `--deny-advisories`
            // additionally promotes `advisory` (warn) violations to blocking.
            let advisory_blocks = deny_advisories
                && diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.severity == Severity::Warn);
            Ok(if check::any_error(&diagnostics) || advisory_blocks {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            })
        }
    }
}
