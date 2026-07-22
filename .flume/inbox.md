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

## External-yield probe (filed 2026-07-22) — temper on 9 real external harnesses

Probe: `temper check --harness` against 9 public Claude Code harnesses
(libtorrent, algolia/instantsearch, liftosaur, freenet-core,
basedosdados/pipelines, rails_ai_agents, claude-copilot, go-crypto-wallet,
claude-code-scheduler). Yield confirmed real — freenet's `rule.forbidden_keys`
caught a Cursor `.mdc` `description` key CC silently ignores (the headline
value prop, on a real repo). But four robustness/correctness bugs the
self-dogfood can't surface (temper's own harness is pristine):

5. **`check` hard-errors (aborts the whole run) on a missing `name` or
   malformed frontmatter, instead of reporting a finding.** 4 of 9 harnesses
   crashed with `Error: temper::frontmatter::{no_named_field_id,malformed}`
   and reported nothing else — violates rust.md ("a Diagnostic is a value
   collected, not a thrown `Err`"). Fix one file, rerun, crash on the next;
   makes temper unusable on real foreign input. Highest severity.
   *observed at b6835e8 (claude-copilot, rails_ai_agents, liftosaur,
   claude-code-scheduler).*

6. **`command` kind marks `name`/`description` REQUIRED (exit 1), but Claude
   Code makes all command frontmatter optional** — the invocation name comes
   from the filename [code.claude.com/docs/en/slash-commands, retrieved
   2026-07-22: "All fields are optional. Only `description` is recommended"].
   14 false-positive hard-failures across instantsearch + go-crypto-wallet.
   Downgrade to advisory (portability nudge) or drop.
   *observed at b6835e8.*

7. **`command.required.*` renders *skill* guidance on a command artifact**
   ("Every skill declares a `name`…") — wrong kind label in the help text.
   *observed at b6835e8.*

8. **`install.gate-installed` fires on every foreign harness** — pure noise
   when checking a repo that has not adopted temper, not a drift signal.
   Suppress when the target carries no `.temper/`.
   *observed at b6835e8.*

## Triage dispositions (interactive, 2026-07-22) — for plan's routing

Build-ready (spec cites the intent, no fork):
- **4** reporter-omits-drift → the session-start reporter must deliver the
  gate's findings it is a reporter of (`distribution.md`, "Session start").
- **5** crash→fault → a nameless/malformed member is a well-formedness fault
  reported (keyed by path when it has no id), never a thrown `Err` that aborts
  the run (`intent.md`: "checks are well-formedness"; registration-locus
  precedent).
- **6 + 7** command → **ruled 07-22: require neither** — command requires no
  frontmatter (`builtins.md` command bullet, updated 07-22); drop the required
  name/description clauses and fix the "skill" guidance mislabel.
- **2** version-skew → pin the sdk/engine version in the lock; turn the
  `payload_parse` map error into a version-mismatch diagnostic.

Ruled, spec already covers it — NOT a build entry (no src half):
- **guard block** → **ruled 07-22: block**, but the src work is already done:
  `block` mode denies with exit 2 today (`src/main.rs:126`) and is tested
  (`tests/cli.rs:448-494` — a `block` lock denies a `.claude/` projection
  write, states the managed-by message, allows non-projection writes). The
  only remaining step is dialing the *dogfood's own* guard `warn`→`block` in
  `.temper/` (harness source) — which is **outside build's writablePaths**: a
  human `chore(harness):` change, never a pending entry. **plan: do not route
  any work that writes `.temper/`, `.claude/`, or the emitted `settings.json`
  — build's fence rejects the whole tick (this tick's revert of
  `GUARD-DOGFOOD-DIAL-BLOCK`).** The interactive session owns this dial.

My-call defaults (interactive; override if wrong):
- **8** gate-installed noise → suppress when the target carries no `.temper/`.
- **3** bare-`temper`-on-PATH → the placed hook fails loudly — fix in
  `install`'s command generation (src/), not the emitted `settings.json`
  (a projection, outside the fence).
- **1** emit/install window → `gate-installed` names the un-noted files.

## Centercode dogfood (filed 2026-07-23) — edge-vocabulary gap

9. **The edge grammar cannot express two relation classes the centercode
   harness needs, so they live as hand-authored prose — the exact drift
   class temper exists to kill.** Surfaced by the satisfier-agnostic
   review (prose must not name what an edge already declares):
   (a) **no plugin-citing edge** — `consult`/`reference` target only
   skill/rule/supporting-doc, so the "ask runner / ask cartograph"
   routing idiom in 5 member bodies hard-codes plugin names with no
   derivable rendering; (b) **no reverse-reference/derivation** — a
   supporting doc cannot render "which rule consults me," so ~8
   standards modules open by hand-naming their consuming rule ("The
   `cls` rule holds the invariant; this reference holds the detail").
   Both are candidates for one edge-vocabulary extension; the interim
   ruling on (b) is name-agnostic phrasing (the engineering module's
   form), which loses the concrete pointer a derived rendering would
   keep.
   *observed at 68a15282 (findings in testbed centercode
   landing/phase1-true-up @ 21b7304590, a sha outside this repo).*
