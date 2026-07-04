<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-04 (session + John): dogfood-migration pilot ran — temper's two live
  rules authored through the SDK (`rule()` + `fromFile`, `emit`) and diffed
  against the committed dogfood. John's ruling: the dogfood migrates onto the
  SDK; **memory projection is the wanted next SDK slice** (8 of the dogfood's
  10 built-in-kind members are memories — the migration can't carry the
  harness without it; `sdk/README.md` already names it a follow-on bound).
- Pilot evidence, same date (empirical, scratch harness under the session
  scratchpad; commands reproducible from the emitted artifacts):
  - Projection parity HOLDS: `collaboration` byte-identical to the live
    `.claude/rules/collaboration.md`; `rust` identical except the two install
    placement lines (managed-by note + schema modeline) — SDK `projectMember`
    never rounds committed placements through, where Rust emit does. Wants a
    slice: placement round-through in SDK emit (or a ruling that install
    always re-runs after writeEmit).
  - Lock parity follows the bytes: `collaboration` hashes exact;
    `source_path` dialect differs (`.claude/...` vs `./.claude/...`).
  - Gate seam 1: `temper check` **silently skips** members spelled
    `kind = "claude-code.rule"` (checked 0, exit 0 — invisible, not loud);
    respelled bare `rule`, it checks them. Two halves: align the SDK's
    manifest kind spelling with the gate's, and decide whether the gate's
    silent skip of an unrecognized kind should be loud.
  - Gate seam 2: the gate reads no `roster.toml`/`bindings.toml` (no reader
    under `src/` at all) — an SDK-emitted members-only `temper.toml` yields
    `requirement.dangling` ×2; splicing the requirement tables into
    `temper.toml` turns the same surface green (exit 0, 2 members checked).
    Either the gate learns the assembly-fact artifacts or `writeEmit` writes
    the full assembly file.
  - Manifest dialect (gate-tolerated, drift-relevant): SDK emits one
    whole-body section with the H1 line in the body; the Rust importer
    sectionizes per heading with the heading line split out. SDK
    `line_count` is +1 (trailing-newline `split("\n")` artifact — a 47-line
    body counts 48). No `source_dir` on module-carried members (locus-less,
    by design). Needs a ruling on whether module carriage legitimately has
    its own manifest shape or converges on the importer's.
