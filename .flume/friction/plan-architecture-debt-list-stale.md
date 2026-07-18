## Symptom

`specs/process/architecture.md`'s Invariants section ("Three edges in
today's tree contradicted this map; all three are ruled to resolve toward
it... Until the entries ship, the edges stand here as declared debt")
still lists the `frontmatter → builtin_kind` (test-only) edge as live,
unshipped debt. It isn't: `frontmatter.rs`'s test module already builds
synthetic kinds via `test_support::{skill_kind, rule_kind}` (shipped at
5ad2e61, "build: frontmatter.rs test fixtures swap to synthetic kinds via
test_support"), with zero remaining reference to `builtin_kind`. The
pending entry that tracked it (`FRONTMATTER-TEST-SYNTHETIC-KINDS`) was
correctly drained by a prior post-ship reconciliation. The paragraph's
prose is just stale: it originated at 663e03f (0040's ratification) and
survived unedited through 53df138 (a later spec commit that amended the
same section to add a fourth debt edge, `normalize_path`) without anyone
noticing edge #2 had already shipped three hours earlier.

## Cost this tick

~10 minutes: sweeping the `provider` subsystem (`src/builtin.rs`,
`src/builtin_kind.rs`) led me to re-verify the Invariants section's claims
against disk, since `builtin_kind` is one of the two named endpoints.
Confirming the edge was already dissolved (checking `test_support.rs`,
`git log -S`, and the shipping commit's body) cost real turns that a
correct spec would have skipped. Low stakes this time, but the general
shape recurs: `plan` cannot write `specs/`, so a shipped fix that
discharges a spec-declared debt bullet has no mechanism to flag the now-
stale bullet back to the humans who can edit it — it just silently rots
until someone happens to re-derive it by hand, as I did here.

## Suggested fix

When post-ship reconciliation's audit motion drops a pending entry because
its work shipped, and that entry's `per` or `notes` traces back to a named
spec-declared debt item (a decision's "declared debt" list, an
Invariants-section bullet), route a friction note (this channel) instead
of silently discharging — so a human's next specs/ edit closes the loop.
The other two edges this same architecture.md paragraph names
(`drift → install` / PLACEMENT-MODULE-EXTRACTION, `extract`'s upward
imports / EXTRACT-FOUNDATION-BOUNDARY-RESTORE) are still genuinely open,
so this file's paragraph needs a targeted edit removing just the
`frontmatter → builtin_kind` bullet and its clause in the "Three edges"
list — not a wholesale rewrite.
