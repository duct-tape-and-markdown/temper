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
//! The same harness carries a second property over the same targets: **placement
//! round-trips through discovery**. Emit splices a name through the very glob `check`
//! later walks, so every path emit wrote must be one that glob finds again — a path
//! outside it is written and locked yet ungoverned, the kind counted `(0)` with no
//! finding to name it.
//!
//! Driven on the pattern `tests/builtin_lock_frozen.rs` sets: a real `node` subprocess
//! running the built SDK through `drift::emit_program`, exactly as `tests/emit.rs`
//! drives the seam.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use temper::drift::{self, EmitOptions, EmitReport};

use crate::common;

/// A harness whose embedded `waypoint` format renders one link per edge field, each
/// spelled off the derived target facts alone (`value.targets`), and whose targets cover
/// every unit shape a member can project at: `skill` (a directory unit), `rule` and
/// `command` (a single-segment single-`*` flat glob), `agent` and `memory` (an any-depth
/// `**\/` prefix, which collapses to zero segments before the splice — `agent`'s
/// `.claude/agents/**\/*.md` takes the name in its leaf, `memory`'s `**\/CLAUDE.md`
/// carries no `*` and is wholly fixed), `note-doc` (a literal leading segment, fixed
/// placement the splice carries verbatim), `supporting-doc` (a nested file child, whose
/// path composes from its `guide` host's unit and that host's template pattern rather
/// than from a glob of its own), and `conventions` (a starred-segment lone file, keyed by
/// the directory segment its `*/conventions.md` glob stars and seated inside the skill's
/// own directory).
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

const noteDoc = kind<object>({
  name: "note-doc",
  locus: { kind: "at", root: ".claude", glob: "notes/*.md" },
  unitShape: "file",
  registration: [],
});

const conventions = kind<object>({
  name: "conventions",
  locus: { kind: "at", root: ".claude/skills", glob: "*/conventions.md" },
  unitShape: "starred-segment",
  registration: [],
});

const guide = kind<object>({
  name: "guide",
  locus: { kind: "at", root: ".claude/guides", glob: "*/GUIDE.md" },
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
      { field: "to_skill", to: ["skill"] },
      { field: "to_rule", to: ["rule"] },
      { field: "to_agent", to: ["agent"] },
      { field: "to_command", to: ["command"] },
      { field: "to_memory", to: ["memory"] },
      { field: "to_doc", to: ["supporting-doc"] },
      { field: "to_conventions", to: ["conventions"] },
      { field: "to_note", to: ["note-doc"] },
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
            to_conventions: "conventions:coordinate",
            to_note: "note-doc:cadence",
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
    conventions({ name: "coordinate", prose: text`# Conventions` }),
    noteDoc({ name: "cadence", prose: text`# Cadence` }),
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
    ("to_conventions", "conventions", "coordinate"),
    ("to_note", "note-doc", "cadence"),
];

/// Every member the fixture projects — the edge targets, the `skill` host that carries
/// them, and the `guide` that hosts the nested file child — paired with the locus its own
/// glob is rooted at and that glob, verbatim as the program above declares them.
///
/// For an `at` locus the root is the kind's `governs` root; for the nested file child,
/// whose kind governs no glob at all, it is the **host's unit** — the base its host kind's
/// template pattern is spelled against, and the base `import`'s own per-host scan walks
/// that pattern from.
const GOVERNED: &[(&str, &str, &str, &str)] = &[
    ("skill", "citing", ".claude/skills", "*/SKILL.md"),
    ("skill", "coordinate", ".claude/skills", "*/SKILL.md"),
    ("rule", "rust", ".claude/rules", "*.md"),
    ("agent", "explore", ".claude/agents", "**/*.md"),
    ("command", "review", ".claude/commands", "*.md"),
    ("memory", "CLAUDE", ".", "**/CLAUDE.md"),
    ("guide", "operate-the-gate", ".claude/guides", "*/GUIDE.md"),
    (
        "supporting-doc",
        "checklist",
        ".claude/guides/operate-the-gate",
        "*.md",
    ),
    (
        "conventions",
        "coordinate",
        ".claude/skills",
        "*/conventions.md",
    ),
    ("note-doc", "cadence", ".claude", "notes/*.md"),
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

#[test]
fn every_emitted_projection_is_one_its_own_kinds_glob_finds_again() {
    let (harness, into) = common::wire_sdk_harness("projection-round-trip", WAYPOINT_PROGRAM);

    let report = drift::emit_program(&into, EmitOptions::default()).expect(
        "gating the placement round trip requires a working node + the built @dtmd/temper \
         module — the lane fails loud here rather than silently skipping the comparison",
    );

    for (kind, name, root, glob) in GOVERNED {
        let wrote = projection_of(&report, kind, name);
        let relative = wrote.strip_prefix(&harness).unwrap_or_else(|_| {
            panic!(
                "emit reports `{kind}:{name}` at `{}`, which is not under the harness root `{}`",
                wrote.display(),
                harness.display(),
            )
        });
        // The glob is spelled against its locus, so the candidate is the path beneath it —
        // exactly the spelling `import`'s walk matches segment by segment from there. A
        // `.` root names the harness root itself and prefixes nothing.
        let candidate =
            if *root == "." {
                relative.to_path_buf()
            } else {
                relative.strip_prefix(root).unwrap_or_else(|_| {
                panic!(
                    "emit wrote `{kind}:{name}` to `{}`, which does not even sit under its own \
                     locus root `{root}`",
                    relative.display(),
                )
            }).to_path_buf()
            };

        // Built here rather than reached for through the engine's own matcher: the lane
        // exists to catch the engine agreeing with itself, so the glob semantics are
        // re-stated from `globset` directly, `literal_separator` on as every caller
        // compiles it (`*` inside one segment, `**` across them).
        let matcher = globset::GlobBuilder::new(glob)
            .literal_separator(true)
            .build()
            .unwrap_or_else(|err| panic!("the fixture's `{kind}` glob `{glob}` compiles: {err}"))
            .compile_matcher();

        assert!(
            matcher.is_match(&candidate),
            "emit wrote `{kind}:{name}` to `{}`, but kind `{kind}`'s own glob `{glob}` (rooted \
             at `{root}`) does not find `{}` — placement and discovery are one round trip, so \
             this projection would be committed and locked yet never discovered, leaving the \
             kind counted `(0)` with no finding to name it; fix whichever of the two moved",
            relative.display(),
            candidate.display(),
        );
    }
}
