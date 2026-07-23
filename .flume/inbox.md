<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## Centercode dogfood (filed 2026-07-23) — edge-vocabulary gap
## CORRECTED same day: model misalignment, not a temper capability gap

9. **Withdrawn as filed; narrowed remainder below.** Original filing
   proposed (a) a plugin-citing edge so member bodies could cite
   runner/cartograph derivably. **Withdrawn — chef ruling:** the
   plugin registration is the single source of truth and routing; a
   registered plugin must not be referenced within underlying
   artifacts at all (typed or prose). The fix is deletion of the
   references, applied in the centercode harness — no grammar change
   wanted; a plugin-citing edge would re-duplicate the registration.
   **Remainder (low, optional):** (b) a supporting doc cannot render
   "which rule consults me," so ~8 centercode standards modules
   hand-name their consuming rule. Interim model fix is
   satisfier-agnostic phrasing; under the same single-source lens the
   agnostic form may simply be the end state. Stands only as a
   candidate if a derived reverse-pointer is ever judged valuable —
   do not build without a fresh ask.
   *observed at 68a15282; corrected at 68cd61e7 (findings in testbed
   centercode landing/phase1-true-up @ 21b7304590, a sha outside this
   repo).*

## CRLF checkout drift (filed 2026-07-23) — fresh-clone class

10. **Drift comparison is raw-byte, so a default Windows clone reads
    every projection as drifted before the user touches anything.**
    Git for Windows defaults `core.autocrlf=true`; checkout rewrites
    working-tree files to CRLF, while the lock's emit fingerprints
    were recorded from LF bytes. On first clone, `check` reports
    committed-projection drift for every emitted file, `guard` warns
    on every touch, and emit attempts bannerless rewrites of files
    that aren't semantically changed — the gate cries wolf at first
    contact, on the exact machines (teammates') adoption depends on.
    Repro: clone any temper-represented repo on default Windows git,
    run `temper check .` — mass drift findings, zero real drift.
    Sibling of item 3 (fresh-clone class): the gate misreports its
    own environment.
    Fix candidates: normalize line endings before fingerprint
    comparison (compare LF-canonical bytes; emit stays byte-exact on
    write), or record an EOL policy in the lock and honor it in both
    check and guard. Consumer-side mitigation exists (.gitattributes
    `eol=lf` on harness paths, being added to the centercode PR;
    workaround `core.autocrlf=false` repo-local) but temper should
    not require repos to know this — the comparator owning EOL
    canonicalization protects every future adopter.
    Design note (reporter): canonicalize on compare, not on write —
    emit's byte-determinism (double-emit identical) is load-bearing,
    so the comparator treats LF/CRLF as equivalent while emit keeps
    writing exact LF. An EOL-policy-in-lock instead needs check,
    guard, and install's banner hashes to all honor it.
    *observed at 83479a55 (mechanism hit during centercode Phase 1
    true-up on landing/phase1-true-up, evidence sha a3820277aa
    outside this repo; spurious rewrites reproduced with
    autocrlf=true before the repo-local false override).*

    **Disposition (interactive, 2026-07-23) — build-ready, carries a
    spec/Decision.** Verified against src at HEAD: the raw-byte
    compare is real and lives at ≥4 sites (`drift.rs` committed-drift
    2620, emit-reap 2226, import-hash 2824, guard baseline 2539) over
    `hash::sha256_hex` — no EOL handling anywhere. Not plan-invents:
    the gate misreporting its own environment is against intent.
    Ruling to encode with the entry:
    - **canonicalize-on-compare** (reporter's pick) — every drift/
      fingerprint *comparison* strips CRLF→LF; emit and the stored
      fingerprint stay LF-exact (byte-determinism untouched). The
      boundary is one ruling: all compare sites canonicalize together,
      or a split comparator (check forgives, guard/banner don't) is
      worse than the bug.
    - **reject EOL-policy-in-lock** unless a consumer wants CRLF
      projections (none does; Claude Code reads LF): new lock field +
      three honor sites + migration, buying unrequested config.
    - **ratify the visible edge:** under canonicalize-on-compare a
      genuinely CRLF-committed projection reads `check`-clean yet
      `emit` still rewrites it to LF. Coherent (compare semantic,
      write canonical) and desirable, but state it as intended.
    No grill needed — answer is clear; encode precisely, then build.
