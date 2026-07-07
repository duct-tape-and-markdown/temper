# 0009 — path references resolve module-relative

- **Date:** 2026-07-07 · **Status:** accepted

## Context

`file()` edge resolution disagreed between doc and engine: the SDK
documented module-relative, emit resolved workspace-relative; scaffold's
paths worked only by accident of directory depth (field report 2026-07-07).

## Decision

A path reference resolves relative to the module that states it, never the
workspace — the rule every import system already teaches. Scaffold output,
resolution, and docs move together.

## Rejected

Workspace-relative — it breaks module composability (a module's meaning
changes with the directory it is mounted from) and contradicts the
documented behavior authors were already writing against.

## Consequences

Emit's resolution recuts to the stating module's directory; scaffold paths
simplify; the SDK doc becomes true.
