# 0111 — the playbook is the starter's central shape

- **Date:** 2026-11-26 · **Status:** accepted

## Context

0110 shipped the starter's patterns as separate kinds: a standard, a spec
layout and a procedure. John's model was one level up: a single construct
for an area of guidance, a path-bound rule pointing to a skill whose
SKILL.md routes to topic files. A blind review, which read the current
Claude Code docs and none of the repo, found the same unit. It found six
axes of variation rather than separate constructors, ranked the uses, and
named the construct `playbook`. Its cost argument: every model-invocable
skill's description rides every request, and the listing is capped, so one
router per area beats N procedure skills (code.claude.com/docs/en/skills,
retrieved 2026-11-26). This supersedes 0110's separate pattern kinds.

## Decision

**The playbook is the starter's central shape** (`distribution.md`, "The
SDK"): one area emitted across the surfaces that load it, a path rule for
standing facts and a routing skill with topic files, all references typed.

- Its axes are trigger, invocation, execution, topics, preloading agents
  and enforcement attachments.
- It derives what hand authoring gets wrong. A user-only playbook's rule
  says to ask the user to run it, and a user-only playbook refuses
  preloading. The skill keeps a required description of its own, because a
  path rule never loads before a file is touched.
- A MUST is an attachment (a clause or a hook held by a verifier), never
  prose.
- The router's wording is a replaceable default renderer, so the moments
  and references are typed fields and taste stays the consumer's.

## Rejected

- **Separate pattern kinds** (0110's standard, spec layout and procedure).
  They are one construct along a few axes.
- **`guide`, `pack`, `bundle`** as names. They are close to
  `claude-code-guide`, plugins and bundling.

## Consequences

STARTER-SHIPS-THE-OPT-IN-PATTERNS retires, and derive files the playbook
from this diff. Three runtime facts stay UNVERIFIED and are cited before
build: whether a skill's own `paths` auto-loads it, whether Grep or Glob
hits load a path rule, and whether read topic files survive compaction.

## Amendment (2026-11-26)

Two inputs narrowed the first version. Jeff's survey of hand-built
harness patterns found that a guard on edits, a fence of `ask`
permissions, and an allowlisted task script each hold one area's intent,
so they are the playbook's attachment types, not separate shapes. Its
other patterns are a single mechanism each: the session-start report ships
as the starter's gate hooks, a subagent's declared model is a clause on
the agent kind, and imported doctrine belongs to layers. John ruled that
every axis stays visible. Execution and preloading are declared but not
yet offered, refusing at emit with where to request them, so the shape
shows what it could do. The refusal of a preloaded user-only playbook
waits with preloading.

Rejected: four further constructors (fence, report, role, doctrine
import). A pattern without an area and an intent is a kind or a clause.

## Amendment: the three runtime facts (2026-11-26)

Probed with headless Claude Code 2.1.332 in a trusted throwaway project,
with canary instructions and transcript inspection (the docs are silent):
- a skill's own `paths` gate its listing, not its body: the skill joins
  the listing once a matching file is read, and loads only when invoked;
- a Grep or Glob hit on a rule's paths loads nothing, only a read does;
- a supporting document read in a session returns verbatim after
  compaction, as a file attachment beside the summary.

So a path-triggered playbook keeps its skill listed by default, and
listing it only on touch is the trigger's option, trading discoverability
for listing budget. The facts live in `builtins.md` (rule, skill). The
probe needed a trusted directory: in an untrusted one no project memory
loads at all.
