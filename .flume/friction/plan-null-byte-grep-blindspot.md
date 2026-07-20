## Symptom

`sdk/src/declarations.ts` carried a literal NUL byte (0x00) at the exact spot
a separator space belonged in `tapHookRows`' dedup key, apparently since the
function's own authoring (7b26b4e). Plain `grep`/`rg` treat a file
containing a NUL byte as binary and silently skip it (no match, no warning)
— `rg SETTINGS_MANIFEST sdk/src sdk/test` at commit 7653fa7 found zero hits
in declarations.ts even though the file plainly imported and used it three
lines down from the corrupted one. The posture-sweep tick trusted that
result and filed BUILTINS-SETTINGS-MANIFEST-EXPORT-PRUNE believing the
export had no consumer outside its own module.

## Cost this tick

Not this tick's own — inherited and reconciled this tick, but the actual
price (read from `.flume/metrics.jsonl`) was 25 separate `build` attempts on
BUILTINS-SETTINGS-MANIFEST-EXPORT-PRUNE (next-highest entry in the whole
file: 6) before it finally landed *empty* at c4e0998 — several individual
attempts alone logged over 1M cache-read tokens, an order of magnitude past
the pending-entry rule's 200k-token comfortable band. An out-of-band `fix:`
commit (749f06f, no pending entry, non-conforming prefix) eventually deleted
the NUL byte, restoring `rg`'s visibility into the file, but left the dedup
key with no separator at all (routed to pending as
TAP-HOOK-DEDUP-KEY-SEPARATOR-RESTORE this tick).

## Suggested fix

Build's grep/rg-based "does X have a consumer" checks are a real blind spot
against a binary-flagged file (NUL byte, in this repo's case, but any binary
sniff heuristic applies) — a rule or CLAUDE.md line naming `rg -a`/`grep -a`
(or an explicit binary-file check before trusting a zero-hit result) for
consumer-search would have caught this in one attempt instead of 25. Possibly
also worth a lightweight `file`/null-byte sanity check somewhere in the gate
chain for `sdk/src/**`, `src/**` — a NUL byte in a TypeScript/Rust source
file is never intentional and is cheap to detect.
