# 0089 — an ownerless registration is fingerprinted

- **Date:** 2026-11-25 · **Status:** accepted

## Context

The `ownerless-manifest-segment-fingerprint` question. In a manifest no
member owns, emit refuses a hand-added key or entry but cannot see a hand
edit to an entry it rendered: registration rows carry no bytes, so a
changed hook group is overwritten in silence. `pipeline.md` promises
"drift by hash" for every projection and that "nothing authored is lost".
0050 fingerprints only a manifest with a container member. Ruled by the
session under John's standing direction.

## Decision

**Each registration entry emit renders into a manifest no member owns
carries the byte hash of what it rendered** (`pipeline.md`, "The lock",
provenance). A mismatch is drift, routed to the owning hook member. Emit
refuses before overwriting it.

## Rejected

- **Fail closed on any unmatched entry.** Every rename of a hook member
  would refuse emit.
- **Accept the gap.** Declaring `settings` closes it (0050), but an
  adopter who hasn't learns of it only by losing an edit.
- **Rows carrying the rendered command.** 0050 rejected this because it
  makes the projection the database (0018). A hash names bytes without
  copying them. It is the provenance family the lock already has, at the
  grain emit writes.

## Consequences

The lock gains a provenance hash per ownerless registration row, and old
locks read the missing hash as unknown on first emit, not as drift. A
manifest with a container member keeps its whole-file fingerprint.
