//! The gauntlet corpus — one kitchen-sink SDK harness holding every legal
//! composition, driven `emit` then `check`, both faces `insta`-snapshotted so a
//! later feature's composition seam surfaces as a snapshot diff at ship time. This
//! is the base each feature entry that adds a composable surface extends.
//!
//! The one harness composes the four compositions the corpus names, each a shipped
//! feature:
//!
//! 1. a composed layout body over a templated host — a `spec` document graining
//!    into embedded `invariant` members, itself a nested-file child of a
//!    templated `guide` host, its entries judged as `invariant` members and from
//!    the host over `invariant[*]`;
//! 2. an embedded edge scoped on both endpoints — a `citation` edge inside a
//!    `paths`-scoped rule's body, pointing at a second `paths`-scoped rule;
//! 3. a declared manifest container — a `settings` member owning `settings.json`
//!    whole, its residue keys its own fields, with two declared `hook`s (one of them a
//!    non-command handler, so a `when`-guarded clause composes over a group-array
//!    entry), an `installed-plugin` and a `known-marketplace` registering at its three
//!    collection addresses;
//! 4. a local-locus member under ignore rules — a `local` kind whose gitignored
//!    document the reviewed `governs` discovers all the same (the dial precedent);
//! 5. a starred-segment lone file inside a directory-owning host — a `handbook`
//!    keyed by the directory segment its `*/conventions.md` glob stars, seated in a
//!    `guide`'s directory and carved out of that host's template discovery;
//! 6. a top-level layout document at a governed locus — a `charter` whose prose
//!    region imports a preamble outside the locus, so the lock carries a declaration
//!    row and an import fingerprint for it and no provenance row;
//! 7. a yaml-frontmatter layout document keyed by a frontmatter field — a `decision`
//!    whose layout structures its body alone: its frontmatter fields reach a clause and
//!    its `named-field` identity keys its rows, never the file stem.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use temper::drift::{self, EmitOptions};

use crate::common;
use crate::module_closure::{is_vendored_sdk, without_module_rows};

/// The one kitchen-sink harness program. Every composition the model claims legal
/// that ships today, in one `harness()` — the base the queued feature cells extend.
///
/// External facts cited at the point of claim: `skill`/`rule`/`hook`/
/// `installed-plugin` loci and field schemas are the Claude Code harness's, carried
/// by the built-in kinds themselves (`@dtmd/temper/claude-code`, each `cite`d in
/// `sdk/src/builtins.ts`); the six-noun composition surface (layout regions,
/// nested-file templates, embedded edges, admission, the local commitment class,
/// manifest residue) is temper's own model (`specs/model/`), not an external fact.
const GAUNTLET_PROGRAM: &str = r#"
import { blocks, clause, emit, embeddedMemberValue, harness, kind, required, text, type } from "@dtmd/temper";
import { hook, installedPlugin, knownMarketplace, rule, settings } from "@dtmd/temper/claude-code";

// Composition 1 — a composed layout body over a templated host.
//
// `spec` is a nested-file child of the templated `guide` host, and its body is a
// composed layout: a prose preamble, an `intent` field section, and a collection
// that grains each child heading into one embedded `invariant` member. The
// document is a source read at emit and check; emit writes nothing at its path.
const spec = kind<{ intent?: string }>({
  name: "spec",
  locus: { kind: "nested-file" },
  unitShape: "file",
  content: {
    regions: [
      { region: "prose" },
      { region: "field", slot: "intent" },
      { region: "collection", memberKind: "invariant" },
    ],
  },
});

// The collection's entry kind, declared so a clause can name it. An entry is a member of
// its kind in every respect: a clause on `invariant` judges each entry, and a host path
// over the entries (`invariant[*].test` on `spec`) judges each one from the host. Neither
// seated entry carries a `test` sub-heading, so both clauses fire on both entries.
const invariant = kind({
  name: "invariant",
  locus: { kind: "embedded" },
  unitShape: "file",
});

const guide = kind({
  name: "guide",
  locus: { kind: "at", root: "docs/guides", glob: "*/GUIDE.md" },
  unitShape: "directory",
  registration: [],
  templates: [{ kind: spec, path: "*.md" }],
});

const gateGuide = guide({ name: "operate-the-gate", prose: text`# Operate the gate` });
const representation = spec({ name: "representation", host: gateGuide });

// Composition 2 — an embedded edge scoped on both endpoints.
//
// `citation` is an embedded kind whose `source` leaf is an edge to a `rule`; its
// render hook spells the reference off the derived target facts alone. The edge
// sits inside a `paths`-scoped rule's body and points at a second `paths`-scoped
// rule, so both endpoints carry a `paths` gate.
const citation = kind(
  {
    name: "citation",
    locus: { kind: "embedded" },
    unitShape: "file",
    edgeFields: [{ field: "source", to: ["rule"] }],
  },
  {
    render: (value) => `See [${value.targets.source.name}](${value.targets.source.path}).`,
  },
);

const conventions = rule({
  name: "conventions",
  paths: ["src/**/*.rs"],
  prose: text`
    # Conventions

    The standing bar every module holds.
  `,
});

// `authoring` is the corpus's dead registration: its `paths` globs match no file here
// and nothing imports it, so the shipped root `reachable` clause warns on it. `conventions`
// carries the identical globs and stays silent — the embedded `citation` below is
// unconditionally live and carries liveness across its edge, which is the asymmetry the
// check-diagnostics snapshot pins.
const authoring = rule({
  name: "authoring",
  paths: ["src/**/*.rs"],
  prose: blocks(
    text`
      # Authoring

      How a change enters, and where the standard lives.
    `,
    embeddedMemberValue({ kind: citation, key: "the-standard", leaves: { source: "rule:conventions" } }),
  ),
});

// Composition 3 — a declared manifest container with a resolving plugin→marketplace edge.
//
// Two `hook`s, an `installed-plugin`, and a `known-marketplace` register at the three
// collection addresses inside `settings.json`, and the `settings` container owns the
// file whole — its `permissions`/`autoMemoryEnabled` residue is its own fields, so emit
// renders the declared segments in address order and the residue after them. The
// plugin's `<plugin>@<marketplace>` key names `acme-marketplace`, and the
// known-marketplace declares it, so the marketplace-half edge resolves on the
// reference graph (0039).
const sessionHook = hook({
  name: "SessionStart",
  hooks: [{ type: "command", command: "temper reporter" }],
});
// The second hook fires a *non-command* handler: an `http` one carrying its documented
// `url`. It is the corpus's one guarded clause over a manifest member — the per-handler-kind
// `when` guards live in `handler`'s default contract and fire on the handler child's own
// `type`, so this entry exercises a guard composing over the handler child of a group-array
// registration, beside the object-shape (`known-marketplace`) and scalar-shape
// (`installed-plugin`) ones. Legal, so the only verdict it moves is the member tally — which
// now counts the handler child too. `PostToolUseFailure` is deliberate: the three events
// temper's own gate places at (`SessionStart`, `PreToolUse`, `PostToolUse`) would
// additionally supersede a gate placement, and this cell is about the guard, not about that.
const auditHook = hook({
  name: "PostToolUseFailure",
  matcher: "Write",
  hooks: [{ type: "http", url: "https://hooks.example.com/audit" }],
});
const formatterPlugin = installedPlugin({ name: "formatter@acme-marketplace", enabled: true });
const acmeMarketplace = knownMarketplace({ name: "acme-marketplace", source: "./vendor/acme-marketplace" });
const projectSettings = settings({
  name: "settings",
  permissions: { allow: ["Bash(cargo build:*)"] },
  autoMemoryEnabled: false,
});

// Composition 4 — a local-locus member under ignore rules.
//
// `machine` is an `at`-locus kind with the `local` commitment class: the kind is
// declared and reviewed, its per-machine document is not. Its gitignored document
// is discovered all the same — the reviewed `governs` is the authorship claim that
// overrides the ignore-rule prune (the dial precedent).
const machine = kind({
  name: "machine",
  locus: { kind: "at", root: ".claude/local", glob: "*.md", commitment: "local" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [],
});

// Composition 5 — a starred-segment lone file inside a directory-owning host.
//
// `handbook` is a starred-segment kind: a lone `conventions.md` keyed by the
// directory segment its `*/conventions.md` glob stars, not the shared stem. It sits
// inside the `guide`'s own directory — the guide owns the directory and templates its
// `*.md` companions, and this file coexists there, borrowing the segment for identity.
// The declared locus carves it out of the guide's template discovery, so it is the
// path's sole home and no phantom `spec` twin forms.
const handbook = kind({
  name: "handbook",
  locus: { kind: "at", root: "docs/guides", glob: "*/conventions.md" },
  format: "yaml-frontmatter",
  unitShape: "starred-segment",
  registration: [],
});

const gateHandbook = handbook({ name: "operate-the-gate", prose: text`# Conventions` });

// Composition 6 — a top-level layout document at a governed locus, importing prose.
//
// `charter` is an `at`-locus layout kind: its document is a source emit reads — the
// lock records its declaration row and the import's fingerprint — and writes nothing at
// its path, so it carries no provenance row. Its opening prose region imports a shared
// preamble outside the locus, a fingerprinted content dependency of the member.
const charter = kind<{ intent?: string }>({
  name: "charter",
  locus: { kind: "at", root: "docs/charters", glob: "*.md" },
  unitShape: "file",
  registration: [],
  content: {
    regions: [
      { region: "prose", import: "../preamble.md" },
      { region: "field", slot: "intent" },
    ],
  },
});

const gateCharter = charter({ name: "gate" });

// Composition 7 — a yaml-frontmatter layout document keyed by a frontmatter field.
//
// `decision`'s layout structures its body alone: the frontmatter reads as on any file
// member, so `status` reaches the `required` clause below, and the `named-field`
// identity is the document's own `title`, not the `0001` stem its path splices from.
const decision = kind<{ title?: string; status?: string; ruling?: string }>({
  name: "decision",
  locus: { kind: "at", root: "docs/decisions", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "named-field",
  identityField: "title",
  registration: [],
  content: {
    regions: [
      { region: "field", slot: "ruling" },
      { region: "collection", memberKind: "invariant" },
    ],
  },
});

const gateDecision = decision({ name: "0001" });

process.stdout.write(
  emit(
    harness({
      members: [gateGuide, representation, conventions, authoring, sessionHook, auditHook, formatterPlugin, acmeMarketplace, projectSettings, gateHandbook, gateCharter, gateDecision],
      admit: [{ host: rule, admits: [citation] }],
      expect: [
        { kind: machine, clauses: [clause(type("mode", ["string"]), { severity: "advisory" })] },
        { kind: decision, clauses: [clause(required("status"), { severity: "required" })] },
        { kind: invariant, clauses: [clause(required("test"), { severity: "advisory" })] },
        { kind: spec, clauses: [clause(required("invariant[*].test"), { severity: "advisory" })] },
      ],
    }),
  ).seam,
);
"#;

/// The `spec` layout source — a prose preamble, an `# Intent` field section, and an
/// `# Invariants` collection whose child headings grain into `invariant` members.
/// A document is the layout host's source (`specs/model/pipeline.md`, "Emit"): emit
/// reads it, derives its members, and writes nothing at its path.
const SPEC_DOC: &str = "The representation model, authored in prose.\n\
\n\
# Intent\n\
temper types the documents that program agents.\n\
\n\
# Invariants\n\
\n\
## Loud or nothing\n\
A gate never fabricates absence.\n\
\n\
## The projection is not the database\n\
Facts are declared, never mined back.\n";

/// A per-machine `machine` document — valid frontmatter the local kind reads in
/// place. Gitignored, the way a real per-machine document always is.
const MACHINE_DOC: &str = "---\nmode: advisory\n---\n\nThis workstation's own knob.\n";

/// The `charter` layout source — one `# Intent` field section after the imported prose.
const CHARTER_DOC: &str = "# Intent\nThe gate is check at a narrower trigger.\n";

/// The `decision` layout source — frontmatter naming it and carrying the `status` its
/// clause requires, then a `# Ruling` field section and an `# Invariants` collection.
const DECISION_DOC: &str = "---\ntitle: check-at-session-open\nstatus: accepted\n---\n\
# Ruling\n\
check runs at session open.\n\
\n\
# Invariants\n\
\n\
## Loud or nothing\n\
A gate never fabricates absence.\n";

/// The preamble the `charter` kind's prose region imports.
const PREAMBLE_DOC: &str = "Shared preamble prose.\n";

/// Seat the source documents the harness reads but does not emit — the `spec` layout
/// source under its `guide` host's unit, the gitignored local `machine` document, and
/// the `charter` layout source with the preamble it imports — plus the `.gitignore` that
/// names the local locus. Written before
/// `emit_program`, since a layout host and a local kind each read a source off disk.
fn seat_sources(harness: &Path) {
    common::write_sibling(
        harness,
        "docs/guides/operate-the-gate/representation.md",
        SPEC_DOC,
    );
    common::write_sibling(harness, ".claude/local/machine.md", MACHINE_DOC);
    common::write_sibling(harness, ".gitignore", ".claude/local/\n");
    common::write_sibling(harness, "docs/charters/gate.md", CHARTER_DOC);
    common::write_sibling(harness, "docs/preamble.md", PREAMBLE_DOC);
    common::write_sibling(harness, "docs/decisions/0001.md", DECISION_DOC);
}

/// The emitted projection tree, faced as one deterministic listing: the emit
/// report's per-member outcome and harness-relative path, then every file the
/// compile left under the harness — the projections, the lock, and the sources it
/// read — dropping the workspace's own scaffolding (the vendored SDK and the
/// authored `harness.ts`), which is not a projection. `tree_bytes` sorts, and every
/// byte is the compile's, so the face is stable across runs and machines.
fn projection_tree(harness: &Path, report: &drift::EmitReport) -> String {
    let mut out = String::from("# emit report\n\n");
    let mut lines: BTreeMap<String, String> = BTreeMap::new();
    for entry in &report.entries {
        let rel = entry
            .source_path
            .strip_prefix(harness)
            .unwrap_or(&entry.source_path)
            .to_string_lossy()
            .replace('\\', "/");
        lines.insert(
            format!("{}:{}", entry.kind, entry.name),
            format!("{:?} @ {rel}", entry.outcome),
        );
    }
    for (member, line) in &lines {
        out.push_str(&format!("{member} -> {line}\n"));
    }

    out.push_str("\n# tree\n");
    for (rel, bytes) in common::tree_bytes(harness) {
        let rel = rel.to_string_lossy().replace('\\', "/");
        if rel.contains("node_modules") || rel.ends_with("harness.ts") {
            continue;
        }
        out.push_str(&format!("\n=== {rel} ===\n"));
        let text = String::from_utf8_lossy(&bytes);
        if rel.ends_with(".temper/lock.toml") {
            out.push_str(&redact_engine_stamp(&without_module_rows(
                &text,
                is_vendored_sdk,
            )));
        } else {
            out.push_str(&text);
        }
        if !out.ends_with('\n') {
            out.push('\n');
        }
    }
    out
}

/// The lock's engine stamp, rewritten to a fixed placeholder. The lock names the engine
/// version that wrote it, and this tree is derived fresh on every run with nothing
/// committed to regenerate — so the snapshot pins the key's presence, never the release,
/// which would otherwise churn this face on every version bump.
fn redact_engine_stamp(text: &str) -> String {
    text.split_inclusive('\n')
        .map(|line| {
            if line.starts_with("engine = \"") {
                "engine = \"<VERSION>\"\n"
            } else {
                line
            }
        })
        .collect()
}

/// The check diagnostics, faced as `ok` plus every finding line, the transient
/// harness temp path redacted to `<HARNESS>` so the face is machine-stable.
fn check_diagnostics(harness: &Path) -> String {
    let (findings, ok) = common::check_harness(harness);
    let needle = harness.to_string_lossy().replace('\\', "/");
    let mut out = format!("ok = {ok}\n");
    for finding in &findings {
        out.push_str(&finding.replace(&needle, "<HARNESS>"));
        out.push('\n');
    }
    out
}

#[test]
fn the_gauntlet_corpus_emits_and_checks_to_stable_snapshots() {
    // One kitchen-sink harness holding every legal composition, driven `emit` then
    // `check` over the one wired harness — a later feature's composition seam
    // surfaces as a diff in one of the two faces below.
    let (harness, into) = common::wire_sdk_harness("gauntlet", GAUNTLET_PROGRAM);
    seat_sources(&harness);

    let report = drift::emit_program(&into, EmitOptions::default())
        .expect("the gauntlet composes only legal compositions, so emit compiles the whole");

    insta::assert_snapshot!("projection_tree", projection_tree(&harness, &report));
    insta::assert_snapshot!("check_diagnostics", check_diagnostics(&harness));
}

#[test]
fn emitted_member_set_matches_independent_discovery() {
    // Emit's declared member set and independent discovery agree exactly over the
    // kitchen-sink corpus's locus-shape composition — every kind the lock declares,
    // every shape the gauntlet already composes (file, directory, nested-file,
    // embedded, local-locus, starred-segment).
    let (harness, into) = common::wire_sdk_harness("gauntlet-round-trip", GAUNTLET_PROGRAM);
    seat_sources(&harness);

    let report = drift::emit_program(&into, EmitOptions::default())
        .expect("the gauntlet composes only legal compositions, so emit compiles the whole");

    // Read the lock's provenance: for each kind, the set of source paths it emitted.
    let declarations =
        drift::read_declarations(&into).expect("the emitted lock exists and is valid");
    let mut emitted_members_by_kind: BTreeMap<
        String,
        std::collections::BTreeSet<std::path::PathBuf>,
    > = BTreeMap::new();
    for entry in &report.entries {
        emitted_members_by_kind
            .entry(entry.kind.clone())
            .or_default()
            .insert(entry.source_path.clone());
    }

    // Independently discover each kind's members and verify against the lock.
    let lock_doc = drift::read_lock_document(&into).expect("the emitted lock exists and is valid");

    let mut lock_members_by_kind: BTreeMap<String, std::collections::BTreeSet<std::path::PathBuf>> =
        BTreeMap::new();
    for (kind, item) in lock_doc.as_table().iter() {
        // Skip the [declaration] section and non-member arrays.
        if kind == "declaration" {
            continue;
        }
        let Some(table_rows) = item.as_array_of_tables() else {
            continue;
        };
        for row in table_rows.iter() {
            if let Some(source_path_str) = row.get("source_path").and_then(|item| item.as_str()) {
                lock_members_by_kind
                    .entry(kind.to_string())
                    .or_default()
                    .insert(std::path::PathBuf::from(source_path_str));
            }
        }
    }

    // For each kind in the declarations that has a governs locus, use discover_kind_files
    // to independently discover its members and verify they match the lock.
    let discovery = temper::import::Discovery::new(&harness);
    for kind_fact in &declarations.kinds {
        let kind_name = &kind_fact.name;
        let Some(ref governs_root) = kind_fact.governs_root else {
            // Nested-file kind: its members are discovered off the host, not directly.
            continue;
        };
        let Some(ref governs_glob) = kind_fact.governs_glob else {
            continue;
        };
        // Skip kinds with no provenance — registration members embedded in manifests.
        if !lock_members_by_kind.contains_key(kind_name) {
            continue;
        }

        // Construct a minimal CustomKind for discover_kind_files.
        let governs = temper::kind::Governs {
            root: governs_root.clone(),
            glob: governs_glob.clone(),
        };
        let kind_commitment = kind_fact
            .commitment
            .as_ref()
            .and_then(|c| match c.as_str() {
                "local" => Some(temper::kind::Commitment::Local),
                _ => None,
            });
        let custom_kind = temper::kind::CustomKind::new(
            kind_name.clone(),
            governs.clone(),
            temper::kind::Extraction::new(vec![]),
        );
        let custom_kind = temper::kind::CustomKind {
            commitment: kind_commitment,
            ..custom_kind
        };

        // Discover this kind's files independently.
        let discovered = temper::import::discover_kind_files(
            &discovery,
            &custom_kind,
            &governs,
            temper::import::LocalOverride::Honored,
        );

        // Normalize discovered paths to be harness-relative (matching the lock's format).
        let discovered_set: std::collections::BTreeSet<std::path::PathBuf> = discovered
            .into_iter()
            .map(|path| {
                path.strip_prefix(&harness)
                    .map(|p| p.to_path_buf())
                    .unwrap_or(path)
            })
            .collect();

        // Compare with lock's provenance for this kind.
        let lock_set = lock_members_by_kind
            .get(kind_name)
            .cloned()
            .unwrap_or_default();

        assert_eq!(
            lock_set, discovered_set,
            "kind '{kind_name}': lock provenance must match independent discovery"
        );
    }
}

/// The `(rule, message)` findings one shell-edge `temper guard` call surfaced in-band —
/// each `  <rule>: <message>` line under a section preamble — empty for a pass, which
/// prints nothing. Read at `warn`, the gauntlet's declared mode.
fn shell_edge_findings(harness: &Path) -> Vec<(String, String)> {
    let (code, output) = common::run_guard(harness, SHELL_CALL);
    assert_eq!(
        code,
        Some(0),
        "no mode exits non-zero at the post edge, got: {output}"
    );
    if output.is_empty() {
        return Vec::new();
    }
    common::guard_in_band(&output, "PostToolUse")
        .lines()
        .filter_map(|line| line.strip_prefix("  ")?.split_once(": "))
        .map(|(rule, message)| (rule.to_string(), message.to_string()))
        .collect()
}

/// Whether `check`'s finding lines carry `rule` over a message containing `needle`.
fn check_raises(check: &[String], rule: &str, needle: &str) -> bool {
    common::findings_for(check, rule)
        .iter()
        .any(|line| line.contains(needle))
}

/// The `PostToolUse` payload after a Bash call (code.claude.com/docs/en/hooks,
/// "PostToolUse input", retrieved 2026-09-25): no field names what the call wrote.
const SHELL_CALL: &str = "{\"session_id\":\"s\",\"hook_event_name\":\"PostToolUse\",\
     \"tool_name\":\"Bash\",\"tool_input\":{\"command\":\"true\"},\
     \"tool_response\":{\"stdout\":\"\",\"stderr\":\"\",\"interrupted\":false}}";

/// The guard is `check` at a narrower trigger (`specs/distribution.md`, "The placements
/// and their enforcement modes"): every finding it raises at either edge is one `check`
/// raises on the tree it judges. Over the kitchen-sink corpus — every composition the
/// guard's loci, member walk and freshness judges must classify the way `check` does —
/// at the post edge on the clean tree and on a tree a call disturbed (a projection
/// edited, an import target moved, a stray left at a governed locus), and at the pre edge
/// over a write to each kind of path, judged against `check` on the tree the write leaves.
#[test]
fn the_guard_s_findings_on_the_gauntlet_are_a_subset_of_check_s_at_both_edges() {
    let (harness, into) = common::wire_sdk_harness("gauntlet-guard-subset", GAUNTLET_PROGRAM);
    seat_sources(&harness);
    drift::emit_program(&into, EmitOptions::default())
        .expect("the gauntlet composes only legal compositions, so emit compiles the whole");

    // Post edge, clean tree: whatever the guard says, `check` says word for word.
    let clean = shell_edge_findings(&harness);
    let (check_clean, _) = common::check_harness(&harness);
    for (rule, message) in &clean {
        assert!(
            check_raises(&check_clean, rule, message),
            "the post edge raised `{rule}: {message}` on the clean gauntlet, which `check` \
             does not: {check_clean:#?}"
        );
    }

    // An undeclared `spec` child under its `guide` host's unit: its locus and its address
    // both compose from the host, and both edges name it as `check` does.
    common::write_sibling(&harness, "docs/guides/operate-the-gate/stray.md", SPEC_DOC);

    // Pre edge: a write to each kind of path the corpus holds — a projection, a declared
    // top-level layout document, a nested layout source, an undeclared one, a local
    // document, a vendored memory file, and a new stray at a governed locus. Each finding is recorded with the
    // path it bound, then every write lands and `check` judges the tree they left.
    let writes = [
        ".claude/rules/conventions.md",
        "docs/charters/gate.md",
        "docs/guides/operate-the-gate/representation.md",
        "docs/guides/operate-the-gate/stray.md",
        ".claude/local/machine.md",
        "vendor/dep/CLAUDE.md",
        ".claude/rules/stray.md",
        "docs/charters/stray.md",
    ];
    // The pending-write edge speaks one finding about the one path its payload names, so
    // a surfaced verdict at all is the binding recorded.
    let mut bound = Vec::new();
    for path in writes {
        let (code, output) = common::run_guard(&harness, &common::guard_write_payload(path));
        assert_eq!(code, Some(0), "warn allows every write, got: {output}");
        if !output.is_empty() {
            bound.push(path);
        }
    }
    assert!(
        bound.contains(&".claude/rules/conventions.md")
            && bound.contains(&".claude/rules/stray.md")
            && bound.contains(&"docs/charters/stray.md")
            && bound.contains(&"docs/guides/operate-the-gate/stray.md"),
        "non-vacuity: the pre edge binds the projection, the stray and both undeclared \
         layout documents, got: {bound:?}"
    );
    for path in writes {
        let target = harness.join(path);
        // A new layout document lands bytes that fit its kind's layout, so `check` reads it
        // as a member of the kind rather than refusing it as a load fault.
        let mut bytes = fs::read(&target).unwrap_or_else(|_| {
            if path.starts_with("docs/charters/") {
                CHARTER_DOC.into()
            } else {
                Vec::new()
            }
        });
        bytes.extend_from_slice(b"\nA line the pending write adds.\n");
        common::write_sibling(&harness, path, &String::from_utf8(bytes).unwrap());
    }
    let (check_written, _) = common::check_harness(&harness);
    for path in &bound {
        assert!(
            check_raises(&check_written, "root.fresh", path)
                || check_raises(&check_written, "root.locus-declared", path),
            "the pre edge bound a write to `{path}` that `check` raises nothing about once \
             it lands: {check_written:#?}"
        );
    }

    // Post edge, disturbed tree: the written tree above, plus the import target moved and
    // an undeclared `named-field` layout document, whose identity is its own frontmatter's.
    common::write_sibling(&harness, "docs/preamble.md", "Edited preamble prose.\n");
    common::write_sibling(
        &harness,
        "docs/decisions/0002.md",
        &DECISION_DOC.replace("check-at-session-open", "an-undeclared-ruling"),
    );
    let disturbed = shell_edge_findings(&harness);
    let (check_disturbed, _) = common::check_harness(&harness);
    let said = |needle: &str| {
        disturbed
            .iter()
            .any(|(_, message)| message.contains(needle))
    };
    assert!(
        said(".claude/rules/conventions.md")
            && said("docs/preamble.md")
            && said(".claude/rules/stray.md")
            && said("docs/charters/stray.md")
            && said("docs/guides/operate-the-gate/stray.md")
            && said("decision:an-undeclared-ruling"),
        "non-vacuity: the post edge names the edited projection, the moved import, the \
         stray and every undeclared layout document, got: {disturbed:#?}"
    );
    for (rule, message) in &disturbed {
        assert!(
            check_raises(&check_disturbed, rule, message),
            "the post edge raised `{rule}: {message}` on the disturbed gauntlet, which \
             `check` does not: {check_disturbed:#?}"
        );
    }
}
