# GUARD-IGNORED-DIR-MEMORY-LOCAL-ISOLATED: locus-declared never binds memory-local

The entry's premise doesn't hold. A bound `locus-declared` clause does not make an unignored, undeclared `CLAUDE.local.md` draw a finding.

Probe: I added `root.locus-declared` (required) to the test's `GuardLock`, wrote `sub/CLAUDE.local.md` and ran both edges. The guard printed nothing. `check` on the same tree raised no `root.locus-declared` finding either; coverage counted the file as `memory-local (1)`. Guard and check agree, so this is no guard defect.

Why the spec entails silence:
- `specs/builtins.md` puts memory-local at the **local** commitment class: "never emitted".
- `specs/model/representation.md` (commitment class): a local member's "rows derive at read time, never in the lock".
- So "the lock declares no member" is the normal state for every memory-local, not drift.
- In the code, `drift::LocusMemberSite` / `undeclared_locus_members_from_doc` scope the file-locus half to *committed* members.

The entry's note says silence is correct only because no clause is bound. In fact silence is correct whatever is bound. No `locus-declared` contrast can exist for memory-local.

To isolate the ignored-dir silence, the contrast must come from a judge that does bind an undeclared local member. That means a memory-local requirement clause, such as a body-size or layout clause the guard judges at the pending write, which fires on `sub/CLAUDE.local.md` and not on `build/CLAUDE.local.md`. Which clause, and whether the guard binds local-kind requirement clauses at PreToolUse at all, is plan's to establish. Re-scope the entry; don't retry it as written.
