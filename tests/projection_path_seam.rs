//! The frozen-agreement lane for the member-projection-path seam.
//!
//! One locus is derived twice, in two languages: the SDK's `projectionPath`
//! (`sdk/src/emit.ts`) spells the path a rendered edge link points at, and the engine's
//! `member_projection_path` (`src/drift.rs`) picks the path emit actually writes the
//! member to. The duplication is forced — `render` is erased at the seam, so the engine
//! never sees the hook and cannot supply the path (`specs/model/representation.md`,
//! "kind") — which leaves agreement as a property to gate rather than a home to unify.
//! `sdk/src/emit.ts` states the invariant in prose ("the two must agree"); this lane is
//! that comment as a test.
//!
//! Agreement is compared through the property the two derivations must share, never
//! symbol to symbol: an embedded format renders a relative link off `value.targets`,
//! and the link is resolved from the host's own emitted projection and compared against
//! the path the engine wrote the target to (`drift::EmitEntry::source_path`). Neither
//! derivation is reached for directly — no `pub` widen on the engine's private fn, no
//! SDK-internal export — so the lane fails exactly when the two disagree, and passes
//! however either spells its own internals.
//!
//! Driven on the pattern `tests/builtin_lock_frozen.rs` sets: a real `node` subprocess
//! running the built SDK through `drift::emit_program`, exactly as `tests/emit.rs`
//! drives the seam.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use temper::drift::{self, EmitOptions, EmitReport};

mod common;

/// A harness whose embedded `waypoint` format renders one link per edge field, each
/// spelled off the derived target facts alone (`value.targets`), and whose targets cover
/// every unit shape a member can project at: `skill` (a directory unit), `rule` and
/// `command` (a single-segment single-`*` flat glob), `agent` and `memory` (an any-depth
/// `**` glob — `agent`'s locus is `.claude/agents/**\/*.md`, so it splices through the
/// any-depth branch, not the flat one), and `supporting-doc` (a nested file child, whose
/// path composes from its `guide` host's unit and that host's template pattern rather
/// than from a glob of its own).
///
/// The host is itself a `skill`, so its own projection lands two directories deep and
/// every rendered link must climb out of it — a host at the root would let a broken
/// relative derivation pass by accident.
const WAYPOINT_PROGRAM: &str = r#"
import { blocks, emit, embeddedMemberValue, harness, kind, text } from "@dtmd/temper";
import { agent, command, memory, rule, skill } from "@dtmd/temper/claude-code";

const supportingDoc = kind<object>({
  name: "supporting-doc",
  locus: { kind: "nested-file" },
  unitShape: "file",
  registration: [],
});

const guide = kind<object>({
  name: "guide",
  locus: { kind: "at", root: ".claude/guides", glob: "GUIDE.md" },
  unitShape: "directory",
  registration: [],
  templates: [{ kind: supportingDoc, path: "*.md" }],
});

const operating = guide({ name: "operate-the-gate", prose: text`# Operate the gate` });

const waypoint = kind<object>(
  {
    name: "waypoint",
    locus: { kind: "embedded" },
    unitShape: "file",
    registration: [],
    edgeFields: [
      { field: "to_skill", to: "skill" },
      { field: "to_rule", to: "rule" },
      { field: "to_agent", to: "agent" },
      { field: "to_command", to: "command" },
      { field: "to_memory", to: "memory" },
      { field: "to_doc", to: "supporting-doc" },
    ],
  },
  {
    render: (value) =>
      Object.entries(value.targets)
        .map(([field, target]) => `- ${field}: [${target.name}](${target.path})`)
        .join("\n"),
  },
);

const program = harness({
  members: [
    skill({
      name: "citing",
      description: "Use when a rendered reference must resolve where emit wrote it.",
      prose: blocks(
        embeddedMemberValue({
          kind: waypoint,
          key: "every-unit-shape",
          leaves: {
            to_skill: "skill:coordinate",
            to_rule: "rule:rust",
            to_agent: "agent:explore",
            to_command: "command:review",
            to_memory: "memory:CLAUDE",
            to_doc: "supporting-doc:checklist",
          },
        }),
      ),
    }),
    operating,
    supportingDoc({ name: "checklist", host: operating, prose: text`# Checklist` }),
    skill({
      name: "coordinate",
      description: "Use when driving a complex task across a team of agents.",
      prose: text`# Coordinate`,
    }),
    rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust conventions` }),
    agent({ name: "explore", description: "Use when a broad read-only sweep is the task.", prose: text`# Explore` }),
    command({ name: "review", description: "Use when reviewing the working diff.", prose: text`# Review` }),
    memory({ name: "CLAUDE", prose: text`# Memory` }),
  ],
  admit: [{ host: skill, admits: [waypoint] }],
});

process.stdout.write(emit(program).seam);
"#;

/// Each edge field the fixture renders, paired with the `kind:name` its leaf addresses —
/// the target whose emitted projection the field's rendered link must resolve to.
const EDGES: &[(&str, &str, &str)] = &[
    ("to_skill", "skill", "coordinate"),
    ("to_rule", "rule", "rust"),
    ("to_agent", "agent", "explore"),
    ("to_command", "command", "review"),
    ("to_memory", "memory", "CLAUDE"),
    ("to_doc", "supporting-doc", "checklist"),
];

/// The path `emit` wrote the `kind`/`name` member to, as the engine itself reported it —
/// the ground truth this lane compares the SDK's rendered links against.
fn projection_of(report: &EmitReport, kind: &str, name: &str) -> PathBuf {
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.kind == kind && entry.name == name)
        .unwrap_or_else(|| panic!("emit reports a projection for `{kind}:{name}`"));
    entry.source_path.clone()
}

/// The links the host's rendered body carries, keyed by edge field — the fixture's
/// `render` hook spells one `- <field>: [<name>](<path>)` line per edge, and the path is
/// the whole subject of this lane.
fn rendered_links(body: &str) -> BTreeMap<String, String> {
    let line = Regex::new(r"(?m)^- (\w+): \[[^\]]*\]\(([^)]*)\)$").unwrap();
    line.captures_iter(body)
        .map(|caps| (caps[1].to_string(), caps[2].to_string()))
        .collect()
}

/// `path` with every `.`/`..` segment resolved against real disk. Canonicalizing both
/// sides is what makes the comparison a comparison of *files* rather than of spellings —
/// and it is `std`'s job, not a second hand-rolled path derivation inside the gate that
/// exists to catch hand-rolled path derivations disagreeing.
fn resolve(path: &Path, context: &str) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|err| {
        panic!(
            "{context}: `{}` resolves to no file on disk ({err}) — the SDK's `projectionPath` \
             and the engine's `member_projection_path` have drifted, and the rendered link \
             points nowhere",
            path.display()
        )
    })
}

#[test]
fn every_rendered_edge_link_resolves_to_the_projection_emit_wrote_for_its_target() {
    let (_harness, into) = common::wire_sdk_harness("projection-path-seam", WAYPOINT_PROGRAM);

    let report = drift::emit_program(&into, EmitOptions::default()).expect(
        "gating the projection-path seam requires a working node + the built @dtmd/temper \
         module — the lane fails loud here rather than silently skipping the comparison",
    );

    let host = projection_of(&report, "skill", "citing");
    let host_dir = host
        .parent()
        .expect("a projected member's path names the directory its links resolve from");
    let links =
        rendered_links(&fs::read_to_string(&host).expect("the host's projection is on disk"));

    for (field, kind, name) in EDGES {
        let link = links.get(*field).unwrap_or_else(|| {
            panic!("the `waypoint` format renders a link for edge field `{field}`; body carried {links:?}")
        });

        let resolved = resolve(
            &host_dir.join(link),
            &format!("edge `{field}` rendered `{link}` from the `skill:citing` host"),
        );
        let wrote = resolve(
            &projection_of(&report, kind, name),
            &format!("emit's own projection for `{kind}:{name}`"),
        );

        assert_eq!(
            resolved,
            wrote,
            "edge `{field}`'s rendered link `{link}` resolves from the `skill:citing` host to \
             `{}`, but emit wrote `{kind}:{name}` to `{}` — the SDK's `projectionPath` \
             (sdk/src/emit.ts) and the engine's `member_projection_path` (src/drift.rs) derive \
             one locus in two languages and have drifted apart; fix whichever side moved, since \
             a rendered reference is only true while the two agree",
            resolved.display(),
            wrote.display(),
        );
    }

    assert_eq!(
        links.len(),
        EDGES.len(),
        "every declared edge field renders exactly one link — the lane covers each built-in \
         file kind's unit shape only while all {} are present",
        EDGES.len(),
    );
}
