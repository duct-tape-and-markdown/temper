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

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use miette::IntoDiagnostic;
use temper::bundle;
use temper::check::{self, Severity, Workspace};
use temper::compose;
use temper::contract::Contract;
use temper::coverage;
use temper::drift;
use temper::engine;
use temper::extract;
use temper::graph;
use temper::import;
use temper::install;
use temper::kind::{self, CustomKind, KindError, Unit};
use temper::reporter;
use temper::roster;
use temper::schema;

/// The surface workspace default for `--into` / the `check` argument: a `.temper`
/// directory under the current working directory (`specs/20-surface.md`).
const DEFAULT_WORKSPACE: &str = "./.temper";

/// The authored surface directory beside a harness's `temper.toml` — where a
/// project's own custom-kind definitions (`kinds/<name>/KIND.md`) and packages
/// (`packages/<name>/PACKAGE.md`) live (`specs/40-composition.md`, "Decision: a
/// custom kind is an authored `.temper/` artifact"). On the one-shot gate paths
/// (session-start, `check --harness`) it is the authored root handed to [`gate`],
/// distinct from the throwaway scratch surface the imported members land in.
const TEMPER_DIR: &str = ".temper";

/// The optional author-declared contract layer, discovered at the project root —
/// the invocation directory, beside the harness it governs (`specs/40-composition.md`,
/// "The author-declared contract — `temper.toml`"). Absent ⇒ the by-kind floor
/// runs unchanged.
const TEMPER_TOML: &str = "temper.toml";

/// The gitignored personal override layer discovered beside [`TEMPER_TOML`] — the
/// committed-plus-local split the spec names (`specs/40-composition.md`, "a
/// gitignored `temper-local.toml` layers over *it*"; the split Lefthook proves).
/// `temper.toml` is committed project policy; `temper-local.toml` is a developer's
/// personal clause/severity override that layers over it. Absent ⇒ the committed
/// layer (or bare floor) runs unchanged.
const TEMPER_LOCAL_TOML: &str = "temper-local.toml";

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
        /// The **one-shot mode** (`specs/20-surface.md`, "CLI surface"): lint a raw
        /// harness directly — import it internally into a throwaway surface, run the
        /// identical by-kind gate, and touch no workspace on disk. The zero-config
        /// wedge: `temper check --harness .` finds real problems in a project's own
        /// `.claude/` before any `import` ceremony. Conflicts with the positional
        /// `workspace`.
        #[arg(long, conflicts_with = "workspace")]
        harness: Option<PathBuf>,
        /// Also fail the run on `advisory` (warn-severity) violations, not just
        /// `required` ones — the strict CI policy from `specs/10-contracts.md`.
        #[arg(long)]
        deny_advisories: bool,
        /// The machine format for the diagnostic set (`specs/50-distribution.md`,
        /// "Outward seams — Reporters"). Reporters reshape presentation only — the
        /// exit-code verdict is identical whichever is chosen.
        #[arg(long, value_enum, default_value_t = Reporter::Terminal)]
        reporter: Reporter,
    },
    /// Emit the active per-kind contract as an editor JSON Schema (the keystroke
    /// gate — `specs/50-distribution.md`, "The gate at keystroke").
    Schema {
        /// Emit only this artifact kind's schema (`skill`, `rule`); omitted ⇒ a
        /// JSON object mapping each modeled kind to its schema.
        #[arg(long)]
        kind: Option<String>,
    },
    /// Report on-disk drift of a harness against the surface's import baseline.
    Diff {
        /// The harness to re-scan and compare against the import baseline.
        harness_path: PathBuf,
        /// The surface workspace holding the baseline (defaults to `./.temper`).
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
    },
    /// Project the surface back onto its harness sources, patching only changed
    /// fields over the three-state drift model (`specs/20-surface.md`, the hard
    /// core). Each artifact writes back to the source path `import` recorded.
    Apply {
        /// The surface workspace to project (defaults to `./.temper`). The lock
        /// under it carries the last-applied fingerprints the merge stands on.
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
        /// Compute and report every outcome without writing a single byte — not
        /// the patched sources, not the updated lock.
        #[arg(long)]
        dry_run: bool,
    },
    /// Reconcile direct on-disk harness edits back into the surface — the third
    /// drift direction (`specs/20-surface.md`, the hard core). Drifted and added
    /// sources are pulled into the surface tree and the lock is refreshed; an
    /// in-sync harness is a no-op. A reconcile, not a gate — it exits zero.
    ReAdd {
        /// The harness to re-scan for drifted / added on-disk sources.
        harness_path: PathBuf,
        /// The surface workspace to reconcile into (defaults to `./.temper`). Its
        /// lock is refreshed to the current source bytes.
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
    },
    /// The advisory session-start gate (`specs/50-distribution.md`, "Decision:
    /// the session-start gate is advisory, not blocking"): check a harness in one
    /// shot and emit the `claude-session-start` reporter payload on stdout for a
    /// Claude Code `SessionStart` hook. This is the one-shot import-internally
    /// path — not the two-step import-then-check of the author workflow — so it
    /// takes a *harness* path, not a surface workspace. Advisory: it always exits
    /// zero, never blocking the session; a failing contract routes through the
    /// human via the reporter's notify-and-approve verdict.
    SessionStart {
        /// The harness to check: the same tree `import` scans (a `skills/*` tree,
        /// a bare skill dir, `.claude/rules/*`, plus any `temper.toml` kinds).
        harness_path: PathBuf,
    },
    /// Project temper's own gate wiring into the harness (`specs/50-distribution.md`,
    /// "Decision: `install` projects the gate's wiring"): the `SessionStart` hook
    /// into `.claude/settings.json`, the CI job into `.github/`, and the schema
    /// modeline into each artifact's frontmatter — all as artifacts under the
    /// three-state drift engine, so re-running is idempotent and re-adds anything a
    /// human deleted. `check` then verifies its own gate stays installed.
    Install {
        /// The project root to wire the gate into (defaults to the current
        /// directory, beside the `.claude/` and `.github/` the placements land in).
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Compute and report every placement without writing a single byte.
        #[arg(long)]
        dry_run: bool,
    },
    /// Compose the imported surface into a publishable Claude Code plugin +
    /// `marketplace.json` (`specs/50-distribution.md`, "The plugin — the
    /// Claude-Code-native delivery"): the operate-the-gate skill, the `SessionStart`
    /// hook in its own `hooks.json`, and the shipped built-in packages embedded. The
    /// vendored, generated plugin is byte-faithful where it carries prose and
    /// deterministic, so re-running reproduces an identical tree. `temper bundle` over
    /// temper's own surface self-packages temper's plugin — the dogfood target.
    Bundle {
        /// The imported surface workspace to compose from (defaults to `./.temper`).
        #[arg(default_value = DEFAULT_WORKSPACE)]
        path: PathBuf,
        /// Where to write the plugin tree (defaults to `./plugin`).
        #[arg(long, default_value = "./plugin")]
        out: PathBuf,
    },
}

/// The machine format `check` renders its diagnostic set in — the reporter family
/// of `specs/50-distribution.md` ("Outward seams — Reporters"), one contract, many
/// placements. Every variant reshapes *presentation only*; none re-judges the
/// harness, so the exit-code verdict is identical across all three.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Reporter {
    /// The default: miette's graphical terminal render ([`check::render`]).
    Terminal,
    /// GitHub Actions `::error`/`::warning::` workflow-command annotations, inline
    /// on the PR ([`reporter::github`]).
    Github,
    /// A SARIF 2.1.0 log for code-scanning ingestion ([`reporter::sarif`]).
    Sarif,
}

fn main() -> miette::Result<ExitCode> {
    match Cli::parse().command {
        Command::Import { harness_path, into } => {
            import::run(&harness_path, &into)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Check {
            workspace,
            harness,
            deny_advisories,
            reporter,
        } => {
            // Two ways into the same gate (`specs/20-surface.md`, "CLI surface").
            // `--harness <path>` is the **one-shot** wedge: import the raw harness
            // into a throwaway scratch surface and gate against the harness's own
            // `temper.toml`, exactly as the session-start placement does, then tear
            // the scratch down — no workspace is written. Without it, the author's
            // two-step path gates an already-imported surface. Both produce the same
            // diagnostic set shape, so the render + exit-code below is shared.
            let diagnostics = match harness {
                Some(harness) => {
                    let scratch = scratch_surface()?;
                    import::run(&harness, &scratch)?;
                    // Members import into the scratch surface, but the authored kinds
                    // and packages live beside the harness's `temper.toml` — resolve
                    // them from the harness's own `.temper/`, not the scratch (mirrors
                    // the session-start one-shot).
                    let diagnostics = gate(
                        &scratch,
                        &harness.join(TEMPER_DIR),
                        &harness.join(TEMPER_TOML),
                    )?;
                    // A leftover scratch dir must never fail the run; swallow removal
                    // errors, mirroring the session-start one-shot.
                    let _ = fs::remove_dir_all(&scratch);
                    diagnostics
                }
                None => {
                    let workspace = workspace.unwrap_or_else(|| PathBuf::from(DEFAULT_WORKSPACE));
                    // Two-step: the surface *is* the authored `.temper/`, so members and
                    // the authored kinds/packages resolve from the one directory.
                    gate(&workspace, &workspace, Path::new(TEMPER_TOML))?
                }
            };

            // Reporters reshape presentation only — the same diagnostic set, a
            // different machine format (`specs/50-distribution.md`, "Outward seams
            // — Reporters"). The exit-code gate below is untouched by the choice.
            match reporter {
                Reporter::Terminal => print!("{}", check::render(&diagnostics)),
                Reporter::Github => print!("{}", reporter::github(&diagnostics)),
                Reporter::Sarif => println!("{}", reporter::sarif(&diagnostics)),
            }

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
        Command::Schema { kind } => {
            // The keystroke placement of the one gate (`specs/50-distribution.md`,
            // "The gate at keystroke"): emit the *active* contract per kind — the
            // same by-kind floor ⊕ optional `temper.toml` layer `check` gates
            // against — as an editor JSON Schema. Two channels: the decidable
            // clauses as validation keywords, and each field clause's advisory
            // `guidance` prose as the property's `description` (hover docs), kept
            // strictly disjoint (see `schema.rs`).
            let layer = load_layer(Path::new(TEMPER_TOML))?;

            // A bound project package resolves from the default surface's
            // `.temper/packages/` — the same resolution set `check` uses (built-in
            // floor ∪ `.temper/packages/`, `specs/20-surface.md`, "Decision: package
            // binding is by artifact kind"). The schema command takes no workspace, so
            // it reads the default `./.temper/packages/`.
            let packages_dir = Path::new(DEFAULT_WORKSPACE).join("packages");

            // The modeled by-kind floors, paired with the kind name the layer keys
            // on — the identical floors `check` composes above.
            let floors: Vec<(&str, Contract)> = vec![
                (
                    "skill",
                    Contract::parse(BUILTIN_SKILL_CONTRACT, Path::new("skill.anthropic.toml"))?,
                ),
                (
                    "rule",
                    Contract::parse(BUILTIN_RULE_CONTRACT, Path::new("rule.toml"))?,
                ),
            ];

            let json = match kind.as_deref() {
                // `--kind <k>`: emit just that kind's schema. An unknown kind is a
                // hard error, never a silent empty schema — the caller named a kind
                // `temper` does not model.
                Some(requested) => {
                    let floor = floors
                        .into_iter()
                        .find_map(|(name, floor)| (name == requested).then_some((name, floor)));
                    let (name, floor) = floor.ok_or_else(|| {
                        miette::miette!("unknown kind `{requested}` (temper models: skill, rule)")
                    })?;
                    let contract = compose::effective(layer.as_ref(), name, floor, &packages_dir)?;
                    schema::emit(&contract)
                }
                // No `--kind`: a JSON object mapping each modeled kind to its schema.
                None => {
                    let mut map = serde_json::Map::new();
                    for (name, floor) in floors {
                        let contract =
                            compose::effective(layer.as_ref(), name, floor, &packages_dir)?;
                        map.insert(name.to_string(), schema::emit(&contract));
                    }
                    serde_json::Value::Object(map)
                }
            };

            println!("{}", serde_json::to_string_pretty(&json).into_diagnostic()?);
            Ok(ExitCode::SUCCESS)
        }
        Command::Diff { harness_path, into } => {
            // Drift is a report, not a gate (`specs/20-surface.md`, CLI surface):
            // load the surface, compare it against the live harness, print the
            // four-state report, and exit zero regardless of what it finds. The
            // engine writes nothing — this is the read-only on-disk-vs-baseline
            // slice; `apply`/`re-add` own write-back.
            let ws = Workspace::load(&into)?;
            let report = drift::diff(&ws, &harness_path)?;
            print!("{}", drift::render(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::Apply { into, dry_run } => {
            // The write direction (`specs/20-surface.md`, "Drift / apply"): load the
            // surface + its lock, project it back onto the harness sources patching
            // only changed fields, and print the applied/unchanged/conflicted report.
            // `--dry-run` reports every outcome but writes nothing. Apply targets the
            // recorded provenance path per artifact — the destination is the source it
            // came from, so no harness root is re-supplied here (unlike `diff`, whose
            // harness arg drives its on-disk *rescan* for the "added" axis).
            let ws = Workspace::load(&into)?;
            let report = drift::apply(&ws, &into, drift::ApplyOptions { dry_run })?;
            if dry_run {
                println!("dry run — no files written");
            }
            print!("{}", drift::render_apply(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::ReAdd { harness_path, into } => {
            // The on-disk -> surface reconcile (`specs/20-surface.md`, "the surface
            // is the source of truth" — `re-add` keeps direct on-disk editing
            // first-class). Load the surface + its lock, pull every drifted / added
            // harness source back into the surface, refresh the lock, and print the
            // reconciled/added/unchanged report. A reconcile, not a gate: exit zero
            // regardless. Unlike `apply`, this re-scans the live harness (like
            // `diff`), so it takes the harness path as well as the surface.
            let ws = Workspace::load(&into)?;
            let report = drift::re_add(&ws, &into, &harness_path)?;
            print!("{}", drift::render_readd(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::SessionStart { harness_path } => {
            // The advisory session-start gate (`specs/50-distribution.md`,
            // "Decision: the session-start gate is advisory, not blocking"): a
            // one-shot check over a *harness* path. Import it internally into a
            // throwaway scratch surface, run the same by-kind gate `check` runs,
            // and emit the `claude-session-start` reporter payload on stdout for a
            // Claude Code `SessionStart` hook.
            //
            // Import-internally, not the author's two-step import-then-check: the
            // surface is scratch, and the author layer (the harness's `temper.toml`
            // custom kinds/requirements) is read from the harness itself, not the process
            // CWD — so the gate judges the harness under check.
            //
            // Advisory: the run *always* exits zero. `SessionStart` cannot block,
            // and a failing contract routes through the human via the reporter's
            // notify-and-approve verdict, never a hard denial.
            let scratch = scratch_surface()?;
            import::run(&harness_path, &scratch)?;
            // Members import into the scratch surface; the authored custom-kind
            // definitions and bound packages resolve from the harness's own `.temper/`
            // beside its `temper.toml`, never the scratch (which never carries them).
            let diagnostics = gate(
                &scratch,
                &harness_path.join(TEMPER_DIR),
                &harness_path.join(TEMPER_TOML),
            )?;
            // Best-effort cleanup of the scratch surface: a leftover temp dir must
            // never fail the advisory gate, so a removal error is swallowed.
            let _ = fs::remove_dir_all(&scratch);

            println!("{}", reporter::session_start(&diagnostics));
            Ok(ExitCode::SUCCESS)
        }
        Command::Install { path, dry_run } => {
            // Project the gate wiring into the harness under the three-state drift
            // engine (`specs/50-distribution.md`, "Decision: `install` projects the
            // gate's wiring"). Idempotent and re-add-able; `--dry-run` reports every
            // placement's outcome without writing a byte.
            let report = install::run(&path, dry_run)?;
            if dry_run {
                println!("dry run — no files written");
            }
            print!("{}", install::render(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::Bundle { path, out } => {
            // Compose the imported surface into a publishable plugin + marketplace
            // (`specs/50-distribution.md`, "The plugin"). Deterministic and
            // byte-faithful where it carries prose; the CLI is a thin wrapper over
            // the library composer.
            let report = bundle::run(&path, &out)?;
            print!("{}", bundle::render(&report));
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// Produce the merged diagnostic set for a surface `workspace` against the active
/// by-kind contracts — the shared gate logic behind both `check` and the
/// session-start reporter (`specs/10-contracts.md`, both greens; `specs/20-surface.md`,
/// "contract selection is by artifact kind"). Extracted verbatim from `check` so
/// the one-shot session-start path runs byte-identical checks; `check`'s own
/// behaviour is unchanged.
///
/// `temper_toml` is the author-declared layer's path: `check` passes the project
/// root's `temper.toml`, while the one-shot session-start gate passes the
/// harness's own, so the roster/graph/custom-kind tiers resolve relative to the
/// harness under check rather than the process CWD. Absent that file the layer is
/// `None` and the by-kind floor runs verbatim.
///
/// `authored` is the `.temper/` directory the author's own custom kinds and
/// packages are read from (`<authored>/kinds/<name>/KIND.md`,
/// `<authored>/packages/<name>/PACKAGE.md`) — kept distinct from `workspace`, the
/// surface the *imported members* are enumerated from. On the two-step `check`
/// path the two coincide (both the surface), so behaviour is unchanged. On the
/// one-shot path (session-start, `check --harness`) they diverge: members import
/// into a throwaway scratch surface (`workspace`) while the authored KIND.md and
/// bound package live beside the harness's `temper.toml` (`authored`), so a
/// registered custom kind's definition resolves from the harness, not the scratch.
/// Load the author-declared layer for a `temper_toml` path, folding a gitignored
/// `temper-local.toml` beside it over the committed layer when present
/// (`specs/40-composition.md`, "a gitignored `temper-local.toml` layers over *it*").
/// The committed `temper.toml` is project policy; the local file is a developer's
/// personal clause/severity override that layers on top — the committed-plus-local
/// split Lefthook proves. The local file is discovered in the *same* directory as
/// the committed one, so both `check` (project root) and the session-start gate
/// (harness root) resolve it beside the file they already consult.
///
/// Absent local ⇒ the committed layer (or bare floor) is returned verbatim — every
/// existing path is unchanged. Present local over a present committed layer folds
/// via [`compose::AuthorLayer::fold_local`]; present local with no committed layer
/// layers straight over the floor, so the personal file alone still customizes the
/// contract.
fn load_layer(temper_toml: &Path) -> miette::Result<Option<compose::AuthorLayer>> {
    let committed = compose::AuthorLayer::load(temper_toml)?;

    let local_path = temper_toml.parent().map_or_else(
        || PathBuf::from(TEMPER_LOCAL_TOML),
        |dir| dir.join(TEMPER_LOCAL_TOML),
    );
    let Some(local) = compose::AuthorLayer::load(&local_path)? else {
        return Ok(committed);
    };

    Ok(Some(match committed {
        Some(base) => base.fold_local(local),
        None => local,
    }))
}

fn gate(
    workspace: &Path,
    authored: &Path,
    temper_toml: &Path,
) -> miette::Result<Vec<check::Diagnostic>> {
    let ws = Workspace::load(workspace)?;

    // The optional author-declared layer beside the harness — the committed
    // `temper.toml` with a gitignored `temper-local.toml` folded over it when
    // present. Absent both ⇒ `None` and the floor runs verbatim (every existing
    // test's path); present ⇒ it layers over the by-kind floor per kind below
    // (`specs/40-composition.md`, the `temper.toml` Decision).
    let layer = load_layer(temper_toml)?;

    // Where a bound project package resolves from: the *authored* `.temper/packages/`
    // directory. A `[kind.<k>] package = "<name>"` binding resolves a name against the
    // built-in floor ∪ this directory (`specs/20-surface.md`, "Decision: package
    // binding is by artifact kind"): the built-in name (`skill.anthropic`, `rule`)
    // resolves from the embedded floor below, any other name from
    // `<packages_dir>/<name>/PACKAGE.md` via PACKAGE-DOCUMENT's loader. Absent binding ⇒
    // the floor, so this directory is never consulted on the floor-only path.
    //
    // `authored` is the `.temper/` that carries the author's own kinds/packages, which
    // is *not* the imported `workspace` on the one-shot path: session-start (and
    // `check --harness`) import members into a throwaway scratch surface, but the
    // authored KIND.md/PACKAGE.md live beside the harness's `temper.toml`, never in the
    // scratch. The two-step `check` passes the surface itself, so its resolution is
    // unchanged.
    let packages_dir = authored.join("packages");

    // Where a registered custom kind's authored definition resolves from: the authored
    // `.temper/kinds/<name>/KIND.md` (`specs/40-composition.md`, "Decision: a custom
    // kind is an authored `.temper/` artifact"). Consulted only when the assembly
    // registers a custom kind, so the floor-only path never reads it. Resolves from
    // `authored`, not the imported `workspace`, for the same reason `packages_dir` does.
    let kinds_dir = authored.join("kinds");

    // Dispatch by artifact kind: each kind's features are validated against the
    // *effective* contract for its kind — the package the kind binds (the embedded
    // floor by default) with the author layer's clauses folded over it — and the
    // findings are merged into one diagnostic set (`specs/20-surface.md`, "Decision:
    // package binding is by artifact kind"). The generic engine holds no per-kind
    // opinion — each contract carries its own clauses, so a mixed harness (skills
    // *and* rules) is judged correctly in one run.
    let skill_features: Vec<extract::Features> =
        ws.skills.iter().map(extract::skill_features).collect();
    let skill_floor = Contract::parse(BUILTIN_SKILL_CONTRACT, Path::new("skill.anthropic.toml"))?;

    let rule_features: Vec<extract::Features> =
        ws.rules.iter().map(extract::rule_features).collect();
    let rule_floor = Contract::parse(BUILTIN_RULE_CONTRACT, Path::new("rule.toml"))?;

    // The built-in package set, keyed by name (`skill.anthropic`, `rule`) — the embedded
    // floor a by-name `package` binding resolves against before `.temper/packages/`
    // (PACKAGE-BINDING's order). A requirement's `package` typing and a `membership`
    // `conforms_to` both resolve through this, so packages **compose**: a satisfier is
    // checked by its kind's bound package *and* any package a requirement names
    // (`specs/10-contracts.md`, the typing facet). Cloned before the floors are consumed
    // by `effective` below.
    let package_resolver = compose::PackageResolver::new(
        std::collections::BTreeMap::from([
            (skill_floor.name.clone(), skill_floor.clone()),
            (rule_floor.name.clone(), rule_floor.clone()),
        ]),
        packages_dir.clone(),
    );

    let skill_contract = compose::effective(layer.as_ref(), "skill", skill_floor, &packages_dir)?;
    let rule_contract = compose::effective(layer.as_ref(), "rule", rule_floor, &packages_dir)?;

    // Two greens, not one (`specs/10-contracts.md`, both-greens finish line).
    // **Admissibility** first: each built-in contract is itself validated against
    // the definition before it is trusted to judge a harness — an inadmissible
    // contract fails the run exactly as a `required` conformance violation does.
    // **Conformance** second: each artifact is checked against the contract for
    // its kind. Both sets of findings merge into one rendered diagnostic stream.
    let mut diagnostics = engine::admissibility(&skill_contract);
    diagnostics.extend(engine::admissibility(&rule_contract));
    diagnostics.extend(engine::validate(&skill_contract, &skill_features));
    diagnostics.extend(engine::validate(&rule_contract, &rule_features));

    // The harness-contract tier: run the set-scope predicates over the parsed roster,
    // each quantified over a requirement's satisfier set — the artifacts opting in via
    // `satisfies` (`specs/10-contracts.md`, "Requirements — the harness's named
    // obligations"). Absent `temper.toml` ⇒ no layer ⇒ this adds nothing, so the
    // floor-only path stays byte-for-byte unchanged.
    if let Some(layer) = layer.as_ref() {
        let base_dir = temper_toml.parent().unwrap_or_else(|| Path::new("."));

        // Load every registered custom kind and compute its features *before* the
        // graph tier, so a custom-kind member joins the corpus graph exactly as a
        // built-in kind's members do (`specs/15-kinds.md`, "The entity graph is a kind
        // capability"). Each kind's authored definition loads from
        // `.temper/kinds/<name>/KIND.md` and its imported units project through the
        // kind's own composed extractor. Owned here so the feature slices outlive both
        // the graph tier (which borrows them through `by_kind`) and the conformance
        // loop below.
        let mut custom_kinds: Vec<(&str, CustomKind, Vec<extract::Features>)> = Vec::new();
        for name in layer.registered_kinds() {
            if kind::BUILTIN_KINDS.contains(&name) {
                continue;
            }
            let custom = CustomKind::load(&kinds_dir, name)?;
            let units = custom_units(workspace, &custom)?;
            let features: Vec<extract::Features> = units
                .iter()
                .map(|unit| custom.extraction.extract(unit))
                .collect();
            custom_kinds.push((name, custom, features));
        }

        // The by-kind corpus every set-scope and graph predicate ranges over: the
        // built-in kinds plus each registered custom kind's features, so an edge to or
        // from a custom-kind member resolves through the same generic graph functions a
        // built-in kind's edge does.
        let mut by_kind: std::collections::BTreeMap<&str, &[extract::Features]> =
            std::collections::BTreeMap::from([
                ("skill", skill_features.as_slice()),
                ("rule", rule_features.as_slice()),
            ]);
        for (name, _custom, features) in &custom_kinds {
            by_kind.insert(*name, features.as_slice());
        }

        // The declared edge set the graph tier resolves: the built-in kinds'
        // `[[kind.<name>.relationships]]` (`layer.edges()`) plus each custom kind's own
        // `[[relationships]]` parsed onto `CustomKind` — a reference is a kind
        // capability regardless of the owning kind's category (`specs/15-kinds.md`).
        let mut edges: Vec<compose::Edge> = layer.edges().to_vec();
        for (_name, custom, _features) in &custom_kinds {
            edges.extend(custom.relationships.iter().cloned());
        }

        // Admissibility before conformance, here too: each requirement's own
        // definition is validated against the definition — a `required` typed
        // requirement's kind is satisfiable, its `package` names a real package and is
        // itself admissible, a `count` bound is well-ordered, a `membership`
        // `conforms_to` names a real package, and any `verified_by` resolves — before
        // the roster is trusted to judge the harness (`specs/10-contracts.md`,
        // "Decision: the contract is itself checked — admissibility").
        diagnostics.extend(roster::admissibility(
            layer.requirements(),
            &by_kind,
            &package_resolver,
            base_dir,
        ));

        // The set-scope predicates: each requirement's `count` / `unique` / `membership`
        // gate quantified over its satisfier set — the artifacts opting in via
        // `satisfies` (`specs/45-governance.md`, "The set scope").
        diagnostics.extend(roster::check(
            layer.requirements(),
            &by_kind,
            &package_resolver,
        ));

        // The `conforms-to` half of the same tier: each requirement's satisfiers are
        // validated against its bound `package`'s contract — resolved by name through
        // PACKAGE-BINDING's order — with findings retagged under `requirement.conforms-to`.
        // A non-resolving package is admissibility's finding above, skipped here rather
        // than double-reported.
        diagnostics.extend(roster::conformance(
            layer.requirements(),
            &by_kind,
            &package_resolver,
        ));

        // The graph scope: build the harness reference graph over the edges
        // declared as each kind's `[[kind.<name>.relationships]]` — a reference is a
        // kind capability, not a standalone construct (`specs/15-kinds.md`, "The
        // entity graph is a kind capability") — and check route resolution: a
        // declared reference (`routes_to: standards`) must resolve to a real
        // artifact of the target kind (`specs/45-governance.md`, "The harness is a
        // graph too"). Admissibility before conformance, here too: an edge that
        // names no reference field or targets an unmodeled kind is reported once and
        // skipped by the route check. Absent `temper.toml` ⇒ no layer ⇒ no
        // relationships ⇒ this adds nothing, so the floor-only path stays
        // byte-for-byte unchanged.
        diagnostics.extend(graph::admissibility(&edges, &by_kind));
        diagnostics.extend(graph::check(&edges, &by_kind));

        // The graph-scope `acyclic` predicate (`specs/45-governance.md`, "The graph
        // scope (the model)"): the resolved reference graph must contain no cycle —
        // a circular import loads nothing, so every finding is a true positive.
        // Intrinsic to the declared edges, so always-on over the whole edge set like
        // route resolution above; no `temper.toml` ⇒ no edges ⇒ this adds nothing,
        // so the floor-only path is unchanged.
        diagnostics.extend(graph::acyclic(&edges, &by_kind));

        // The graph-scope `degree` predicate (`specs/45-governance.md`, "The graph
        // scope (the model)"; the worked example "self-registering vs routed"): a
        // requirement declares an in/out edge-count bound and every artifact
        // satisfying it must have a degree inside it over the resolved reference
        // arcs. Declared at the set scope (on the requirement) but ranging over the
        // edge graph, so it takes the requirements *and* the edges, reusing the arc
        // resolution `acyclic`/`check` assemble. Opt-in, per-requirement: a roster
        // declaring no bound does no graph work here, so the floor-only path stays
        // byte-for-byte unchanged.
        diagnostics.extend(graph::degree(layer.requirements(), &edges, &by_kind));

        // The requirement-coverage tier (`specs/10-contracts.md`, "Requirements and
        // `satisfies` — the meaningful contract"): the referential shadow of the
        // meaningful contract. Every `required` requirement must have a resolving
        // home — ≥1 artifact whose representation opts in with a `satisfies` link
        // naming it — and every authored `satisfies` must resolve to a declared
        // requirement. `means` is never judged; coverage is the whole of the gate.
        // Ranges over every opt-in-capable artifact — the built-in kinds (skill ⊕
        // rule) *and* each registered custom kind's members, whose authored
        // `satisfies` is threaded off the member document through the kind's own
        // extractor (`src/kind.rs`). Coverage stays kind-blind by design: a
        // requirement may be filled by any artifact that opts in, so temper's own
        // `spec` corpus can opt into requirements exactly as a skill does
        // (`specs/15-kinds.md`, the worked example). Absent `temper.toml` ⇒ no layer ⇒
        // no requirements ⇒ this adds nothing, so the floor-only path is unchanged.
        let all_features: Vec<extract::Features> = skill_features
            .iter()
            .chain(rule_features.iter())
            .chain(custom_kinds.iter().flat_map(|(_, _, features)| features))
            .cloned()
            .collect();
        diagnostics.extend(coverage::check(layer.requirements(), &all_features));

        // The custom-kind conformance tier: each custom kind the assembly *registers*
        // (a `[kind.<name>]` whose name is not a built-in) is checked through its **own
        // authored extractor** (its features, computed above and shared with the graph
        // tier) and its **bound package** — the same two greens the built-in kinds run
        // above (`specs/15-kinds.md`, "A kind definition — one composed object"), but
        // the definition is loaded from the authored `.temper/kinds/<name>/KIND.md`
        // (`specs/40-composition.md`, "Decision: a custom kind is an authored `.temper/`
        // artifact") and the require-side is a bound package, never inline clauses. Its
        // bound package resolves by name (defaulting to the kind's own name) and runs
        // admissibility + conformance. Absent a registered custom kind ⇒ the loop is
        // empty, so the built-in-only path is byte-for-byte unchanged.
        for (name, _custom, features) in &custom_kinds {
            // The require-side is a bound package (uniform with a built-in kind),
            // resolved by name through the same order every binding uses — defaulting
            // to the kind's own name when the registration binds none explicitly.
            let package_name = layer.kind_package(name).unwrap_or(*name);
            match package_resolver.resolve(package_name)? {
                Some(contract) => {
                    diagnostics.extend(engine::admissibility(&contract));
                    diagnostics.extend(engine::validate(&contract, features));
                }
                None => diagnostics.push(check::Diagnostic::error(
                    format!("{name}.package"),
                    *name,
                    format!(
                        "custom kind `{name}` binds unknown package `{package_name}` (author `.temper/packages/{package_name}/PACKAGE.md`)"
                    ),
                )),
            }
        }
    }

    // The install self-verify: temper checking that its *own* gate is wired
    // (`specs/50-distribution.md`, "the harness checking that its self-check is
    // wired"). The gate wiring lives at the project root — beside the `temper.toml`
    // that governs the harness — so the placements are read relative to its parent
    // (the process CWD for `check`, the harness path for the session-start gate).
    // Advisory (warn) only: a not-yet-installed gate nudges without failing the run,
    // and the session-start reporter ignores warn severity, so the floor-only path's
    // exit verdict is unchanged.
    let root = temper_toml
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    diagnostics.extend(install::gate_installed(root));

    Ok(diagnostics)
}

/// Create a fresh throwaway surface directory for the one-shot session-start
/// import — a scratch workspace under the system temp dir, unique to this process,
/// that the caller removes once it has the diagnostics. Import-internally needs a
/// place to project the harness, but the session-start gate never persists a
/// surface (unlike the author's `import --into`), so it is torn down after use.
fn scratch_surface() -> miette::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("temper-session-start-{}", std::process::id()));
    // A stale directory from a crashed prior run must not poison this import.
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).into_diagnostic()?;
    Ok(dir)
}

/// Load a custom `kind`'s units from the surface, generically — every surface
/// directory under the workspace at the kind's declared `governs.root`, each
/// reloaded into a raw [`Unit`] via [`Unit::from_surface_dir`]. Keyed on the
/// declared locus, never the kind name: temper reads its own `specs/` because its
/// `temper.toml` declares a kind rooted there, not because anything is hardwired to
/// `spec` — and a custom kind rooted anywhere else (`docs/adr`, …) is read the same
/// way, not just `specs/` (`specs/40-composition.md`, "Declaring a custom kind").
///
/// A surface directory is one holding the kind's `<KIND>.md` member document,
/// mirroring the built-in [`Workspace::load`] enumeration, name-sorted so the
/// diagnostic set is stable across runs. A workspace with no directory at the kind's
/// root contributes no units — its contract's admissibility still runs, over zero
/// artifacts.
fn custom_units(workspace_dir: &Path, custom: &CustomKind) -> Result<Vec<Unit>, KindError> {
    let root = workspace_dir.join(&custom.governs.root);
    if !root.is_dir() {
        return Ok(Vec::new());
    }

    // The member document a custom unit is written under — the kind name upper-cased
    // (`spec` → `SPEC.md`), the same convention `import` writes (`src/import.rs`).
    let document = format!("{}.md", custom.name.to_uppercase());
    let listing = fs::read_dir(&root).map_err(|source| KindError::Io {
        path: root.clone(),
        source,
    })?;
    let mut dirs = Vec::new();
    for entry in listing {
        let entry = entry.map_err(|source| KindError::Io {
            path: root.clone(),
            source,
        })?;
        let path = entry.path();
        if path.is_dir() && path.join(&document).is_file() {
            dirs.push(path);
        }
    }
    dirs.sort();

    dirs.iter().map(|dir| Unit::from_surface_dir(dir)).collect()
}
