<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

## The front-door demolition chain (John's go, 2026-07-06; cooling waived; CLEAN SLATE)

PR #6 is merged (0c11135): install is the front door, init absorbed,
placements follow the lock, the embedded default program is a built-in lock,
`temper.toml` retired as a filename. `(carriage-aware-placements)` is
RESOLVED in 20-surface. GATE-READ-LOCK-DEMOLITION un-parks: the decomposition
ceremony ran in-session with the blast map below; plan derives the entries.

**CLEAN SLATE (John's standing ruling, 2026-07-06): published pre-1.0
carries NO backward-compat burden.** No shims, no deprecation aliases, no
migration warnings, no protected old behavior. `init` may be removed the
moment it's convenient, even before the front door ships — a temporary
on-ramp gap on trunk is acceptable; green gates are the only bar.

The serialized chain (the order is the merge-conflict spine; plan may fuse
or split slices against disk evidence, and MUST re-verify the cited symbols):

- **S1 EMIT-PAYLOAD-SEAM** — engine `emit` executes the SDK program (node
  subprocess), receives the payload as JSON on the pipe, and compiles
  EVERYTHING generated: projections + the whole lock, all five declaration
  families (retires the SDK's missing-`satisfies` gap — sdk/src/lock.ts
  writes four families, drift.rs:933 reads five). The SDK stops writing TOML
  entirely (emit.ts:252 writeFileSync, lock.ts:154 stampLock, toml.ts):
  payload JSON out is its whole output surface. Interchange byte-parity
  goldens + schemars/ts-rs retire with it. The engine is the sole compiler.
- **S2 CHECK-READS-LOCK-GOVERNS** — the gate's and `explain`'s member corpus
  becomes: lock `KindFactRow.governs_root/glob` (drift.rs:724 — columns
  written by every producer, read by NOBODY today) → the discovery walk
  (import.rs discover_kind_units:697 / discover_builtin:569) → extraction.
  No lock ⇒ the built-in lock (embedded default program, same shape). Kills
  `check::surface_units` reads (main.rs:654,982,990,1130) and the in-place
  live-extract branch (skill_rule_corpus main.rs:962-990). Tests keep
  `import::run` as the transitional lock producer THIS slice — its
  write_rollup already writes a full lock — so fixture churn is deferred.
- **S3 FIXTURES-OFF-IMPORT** — port the ~11 behavior-test files' setup from
  `import::run` to golden-lock or SDK-emitted fixtures (coverage, graph,
  reachable_gate, requirement_roster, gate_fail_loud, acceptance,
  memory_contract, memory_gate, reporters, bundle, install, emit,
  lock_declaration_rows re-base). Delete the machinery-under-test files:
  adapter_fidelity (copy-tree byte-fixpoint), contract_fixtures (manifest
  goldens), temper_toml's manifest-emit block, genre_leaf's manifest
  round-trip, manifest_kind_spelling.
- **S4 SCRATCH-RETIRE** — delete `import::run`/`run_with_builtins`
  (import.rs:143,157), the copy-tree writers (write_member_surface:621,
  copy_companion:844, existing_surface_member:672), write_rollup:866 (its
  transitional-producer job ended in S3), emit_manifest:426,
  frontmatter::carry_representation:318, scratch_surface (main.rs:1076) and
  both call sites (main.rs:287,568). The discovery walk SURVIVES — it is
  the sole extractor.
- **S5 CODEC-RETIRE** — the `[[member]]` codec both directions (compose.rs
  writers 1487-1559, parse_member* 1738-1980), AuthorLayer
  members()/inplace_members(), and `import::init`/`lift` (their only
  remaining callers). Under clean slate the `init` verb is removed here
  (CLI + main.rs dispatch) without waiting for S6.
- **S6 INSTALL-FRONT-DOOR** — three sub-slices: (a) the scaffolder —
  discovery walk → generated TS member modules (`file()` pointers at the
  original paths, bytes untouched) + `harness.ts` skeleton; (b) the verb —
  discovery report first, one question (flags: `--yes`, `--no-represent`),
  no-path = consented SessionStart hook alone, yes-path = Node+`.temper`
  check (hard refusal with instructions), ensure `@dtmd/temper` dep,
  scaffold, run first emit, place per lock; (c) guard/note lock-grounding —
  emit-owned from lock rows, posture from the lock, GUARD_MESSAGE
  (install.rs:101) rewritten, modeline placed only when its schema artifact
  exists, guard placed only when emit-owned paths exist.
- **S7 TEMPER-TOML-ZERO** — eradicate the remaining readers (AuthorLayer
  load/authority/fold_local, TEMPER_TOML const, explain's load_layer if
  S2/S5 left residue): `rg -c 'temper\.toml' src/ tests/` = 0. cascade
  re-onboards via the new install when it ships; its manifest-era artifacts
  are old temper by definition.

Cascade's four field reports: (1) dissolves in S6c, (2)-(4) dissolve in
S2/S5 — the accepted-debt DATUM under `(inplace-lock-producer)` closes with
the chain. The genre-pilot item (below) routes independently — it rides
emit's fence render, so it sequences after S1 at the earliest; plan's call
whether it's the chain's sibling or successor.
- (cascade, 2026-07-06, John's ruling — CASCADE VOLUNTEERS AS THE GENRE-ADOPTION PILOT, the `(genre-fence-format)` first consumer.) What the pilot brings: the only external harness with live custom kinds (nine spec kinds, lock-compiled), a fully temper-produced spec corpus (74 members, byte-faithful projections, drift-gated), and a ratified Decision convention that IS the shipped `decision` genre shape ("every Decision earns rejected alternatives" — specs/00-style-guide.md). Pilot fixtures: 3-4 real Decisions from the corpus (candidates: training-plan felt-occupant, platform/conversation), picked in tomorrow's human+session workshop. What the pilot needs from temper: the fence render in emit (`blocks()` currently refuses), posture-2 fence parse, leaf addressing over genre values, `${address}` mentions — acceptance is the ratified byte-stable posture-2 ⇄ posture-3 round trip against cascade's real Decisions. Design happens with the consumer in the room: John + session author the fence format against live fixtures, temper's loop builds against it. Route to pending as the demolition's sibling or successor as the queue allows.
