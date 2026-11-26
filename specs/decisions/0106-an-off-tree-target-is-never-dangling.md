# 0106 — an off-tree target is never dangling

- **Date:** 2026-11-26 · **Status:** accepted

## Context

The `out-of-corpus-edge-targets` question, from
OUTPUT-STYLE-IS-SELECTED-BY-SETTINGS. The runtime resolves some edge
targets the harness tree cannot show:

- built-in styles (`default`, `Proactive`, …);
- user-scope styles under `~/.claude/output-styles`;
- plugin-provided styles;
- user-scope marketplaces.

The code ships a `provided` edge fact (a listed name neither resolves nor
dangles), and `contract.md` "edge" did not name it. Every other off-tree
target reads as dangling at error. So a `settings.local.json` naming a
personal style is a false error. Ruled from classes 0101 and 0103, on
plan's lean, under John's standing direction.

## Decision

**A target the runtime answers from outside the corpus is never dangling**
(`contract.md`, "edge"):

- A name the provider documents as built in is a **provided** name, and it
  resolves (0101).
- Any other name the tree cannot show is undecidable in a check of the
  project. It collapses to one coverage note naming the scopes temper does
  not read (0103), never a per-edge error.

## Rejected

- **Keeping off-tree targets dangling.** The finding is false for its
  reader.
- **A per-edge severity for unresolved scopes.** It lets an author silence a
  false claim instead of the tool not making it.

## Consequences

The `provided` fact joins the corpus. A dangling finding fires only for a
name that could have resolved in the tree and did not.
