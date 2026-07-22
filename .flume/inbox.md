<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## Dogfood findings (filed 2026-07-22, interactive) — route these

1. **`emit` projects emit-owned targets without the managed-by placement
   `install` owns.** A member authored after the last `install` ships its
   projection with no guard marker until `install` reruns; `emit` never
   stamps the note (by design), only `install` places it, so the window is
   structural. `install.gate-installed` counts the gap but never *names* the
   files, and the count grows with each emit-authored member.
   *observed at 0861619 (testbed centercode: fresh projections — an
   engineering skill pair + the sweep-discipline rule — advisory went 2→3 as
   they emitted; now reads 4).*

2. **engine/SDK version skew surfaces as `payload_parse: invalid type: map,
   expected a string`, with no version hint.** The lock doesn't pin the
   sdk/engine version it was authored against, so a stale binary against a
   newer symlinked SDK reads as a corrupt payload rather than a version
   mismatch. Cost a real debugging detour (a Jul-10 binary vs HEAD SDK).
   *observed 2026-07-20 (no exact sha captured; baseline ~26c79cd).*

3. **Placed hooks invoke bare `temper`; with no binary on PATH the guard and
   the SessionStart reporter die command-not-found — silently, non-blocking.**
   The gate cannot report its own absence. Fresh-clone class. Fix candidates:
   `install` verifies resolvability, or the placed hook command fails loudly.
   *observed 2026-07-20 (no exact sha captured; baseline ~26c79cd).*

4. **ENGINE BUG — the SessionStart reporter omits the projection-fingerprint
   drift finding.** Repro (append a line to any emitted projection, then diff
   the two commands): plain `temper check .` reports the drift (`committed
   projection … does not match the lock's emit fingerprint`);
   `temper check . --reporter session-start` returns `additionalContext` that
   carries *other* advisories but **not** the drift finding — so every session
   (interactive and headless) opens blind to drift. Reproduced this repo at
   09e6a8a 2026-07-22: `additionalContext` held the `settings-local` advisory,
   drift absent — this refines the testbed observation (b82b087e 07-22), which
   saw a fully empty payload only because that tree carried no other advisory.
   Core bug identical: drift never reaches the session.
   *observed at 09e6a8a (reproduced 2026-07-22; testbed re-verify b82b087e
   07-22, a sha outside this repo).*
   - **Separate open question, split on route:** `temper guard` is advisory
     (exit 0), and PreToolUse exit-0 stdout is not surfaced to the model, so
     the guard nudge may be inert for agents — is a blocking mode (exit 2)
     wanted?
