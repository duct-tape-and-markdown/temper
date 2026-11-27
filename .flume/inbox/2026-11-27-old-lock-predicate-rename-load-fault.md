# A lock from an older engine with a renamed predicate is a load fault, not stale

Found while drafting the 0.0.26 notes. A release-notes agent ran the current binary over the v0.0.25 example's committed lock. `check` stops with `gate.load-fault`, because that lock names the predicate `min_len`, and 0114 made predicate segments kebab-case. `specs/model/pipeline.md` ("The lock", 0117) says an engine reads the stamp before any row, so a lock it cannot read is reported as written by another engine, never as a malformed row. Today that reading covers only stale built-in rows.

Every represented adopter's SessionStart report shows a load fault until they re-emit, and the message names the wrong cause.

Acceptance:
- **new:** a lock stamped by another engine that names an unknown predicate draws the "written by another engine, re-emit" finding, never `gate.load-fault`;
- **kept:** a lock with this engine's stamp and an unknown predicate is still a load fault.
