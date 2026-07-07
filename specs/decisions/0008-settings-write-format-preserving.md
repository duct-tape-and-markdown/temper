# 0008 — the settings write is format-preserving

- **Date:** 2026-07-07 · **Status:** accepted

## Context

Install's hook merge into an existing `.claude/settings.json` was
semantically correct but re-serialized the whole file — keys reordered, EOL
churn (field report 2026-07-07). The round-trip discipline stated for TOML
and markdown had no JSON equivalent.

## Decision

The one settings write is format-preserving: existing keys, order, and
formatting survive the insertion. Install never re-serializes a file it
does not own — the JSON peer of the format-preserving round-trip keystone.

## Rejected

Whole-file re-serialization (a hand-authored file churned by a tool that
touched one key — exactly the diff noise the round-trip discipline exists
to prevent).

## Consequences

The insertion becomes surgical; a hand-authored `settings.json` diffs by
one hunk after install.
