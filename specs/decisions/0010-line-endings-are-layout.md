# 0010 — line endings are layout, never content

- **Date:** 2026-07-07 · **Status:** accepted

## Context

On Windows, emitted projections mixed LF (projection formatting) with CRLF
(verbatim bodies from CRLF sources), breaking byte-reproducibility and
tripping git's EOL warnings (field report 2026-07-07). The fork: is EOL
meaning-carrying, protected by the verbatim invariant?

## Decision

No. Line endings are layout: projections are written LF uniformly, whatever
the source's convention. The verbatim invariant guards words, not encoding
accidents — the same ruling that already made leaf whitespace layout in the
format model, one byte wider.

## Rejected

Preserve source EOL (byte-reproducibility becomes platform-dependent — the
same program emits different bytes on different machines, which invariant 3
exists to forbid). Per-file policy (a knob nobody asked for, guarding a
distinction without meaning).

## Consequences

Emit normalizes EOL to LF on every projection; double-emit comparison holds
across platforms; CRLF sources emit clean.
