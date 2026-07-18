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
//!    templated `guide` host;
//! 2. an embedded edge scoped on both endpoints — a `citation` edge inside a
//!    `paths`-scoped rule's body, pointing at a second `paths`-scoped rule;
//! 3. a partially-declared manifest — a `settings.json` carrying a declared
//!    `hook` and `installed-plugin` beside opaque residue the harness models as no
//!    member;
//! 4. a local-locus member under ignore rules — a `local` kind whose gitignored
//!    document the reviewed `governs` discovers all the same (the dial precedent);
//! 5. a starred-segment lone file inside a directory-owning host — a `handbook`
//!    keyed by the directory segment its `*/conventions.md` glob stars, seated in a
//!    `guide`'s directory and carved out of that host's template discovery.

use std::collections::BTreeMap;
use std::path::Path;

use temper::drift::{self, EmitOptions};

mod common;

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
import { blocks, clause, emit, embeddedMemberValue, harness, kind, text, type } from "@dtmd/temper";
import { hook, installedPlugin, knownMarketplace, rule } from "@dtmd/temper/claude-code";

// Composition 1 — a composed layout body over a templated host.
//
// `spec` is a nested-file child of the templated `guide` host, and its body is a
// composed layout: a prose preamble, an `intent` field section, and a collection
// that grains each child heading into one embedded `invariant` member. The
// document is a source read at emit and check; emit writes nothing at its path.
const spec = kind({
  name: "spec",
  locus: { kind: "nested-file" },
  unitShape: "file",
  registration: [],
  content: {
    regions: [
      { region: "prose" },
      { region: "field", slot: "intent" },
      { region: "collection", memberKind: "invariant" },
    ],
  },
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
    registration: [],
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

// Composition 3 — a partially-declared manifest with a resolving plugin→marketplace edge.
//
// A `hook`, an `installed-plugin`, and a `known-marketplace` register inside
// `settings.json`, and the harness-level `settings` residue (`permissions`,
// `autoMemoryEnabled`) folds in beside them as opaque keys the harness models as no
// member — the file carries more than the program declares. The plugin's
// `<plugin>@<marketplace>` key names `acme-marketplace`, and the known-marketplace
// declares it, so the marketplace-half edge resolves on the reference graph (0039).
const sessionHook = hook({ name: "SessionStart", type: "command", command: "temper reporter" });
const formatterPlugin = installedPlugin({ name: "formatter@acme-marketplace", enabled: true });
const acmeMarketplace = knownMarketplace({ name: "acme-marketplace", source: "./vendor/acme-marketplace" });

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

process.stdout.write(
  emit(
    harness({
      members: [gateGuide, representation, conventions, authoring, sessionHook, formatterPlugin, acmeMarketplace, gateHandbook],
      admit: [{ host: rule, admits: [citation] }],
      expect: [
        { kind: machine, clauses: [clause(type("mode", ["string"]), { severity: "advisory" })] },
      ],
      settings: {
        permissions: { allow: ["Bash(cargo build:*)"] },
        autoMemoryEnabled: false,
      },
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

/// Seat the two source documents the harness reads but does not emit — the `spec`
/// layout source under its `guide` host's unit, and the gitignored local `machine`
/// document — plus the `.gitignore` that names the local locus. Written before
/// `emit_program`, since a layout host and a local kind each read a source off disk.
fn seat_sources(harness: &Path) {
    common::write_sibling(
        harness,
        "docs/guides/operate-the-gate/representation.md",
        SPEC_DOC,
    );
    common::write_sibling(harness, ".claude/local/machine.md", MACHINE_DOC);
    common::write_sibling(harness, ".gitignore", ".claude/local/\n");
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
        out.push_str(&String::from_utf8_lossy(&bytes));
        if !out.ends_with('\n') {
            out.push('\n');
        }
    }
    out
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
