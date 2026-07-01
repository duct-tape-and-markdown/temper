<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- COMPLEXITY-AUDIT remediation batch (all CONFIRMED). Verdict: the accretion pattern
  does NOT recur beyond role/requirement — these are ordinary entropy (dead code, dup
  helpers, drift), cheap and mostly independent. File the entries below.

- SEQUENCING (read first): CONSOLIDATE-REQUIREMENT is IN FLIGHT (atomic, editing
  compose.rs/roster.rs/coverage.rs/graph.rs/main.rs). Everything below lands AFTER it.
  Anything touching compose.rs MUST be `blockedBy CONSOLIDATE-REQUIREMENT`. The check.rs
  and extract.rs entries are disjoint from the consolidation and from each other →
  parallel-safe once it lands.

- CHECK-CLEANUP (D1+D3; `src/check.rs` only; MEDIUM value, low risk; the top item):
  delete the dead pre-engine lint architecture — `trait Rule` (`check.rs:227`), `run()`
  (`:237`), the test-only `OneErrorPerSkill` (`:335`) and its two `run` tests; and the
  never-used diagnostic-span channel — `Diagnostic::span` (`:154`), `with_span`
  (`:192-197`, zero callers), the `labels()` override (`:216-219`, always None), and the
  now-unused `SourceSpan`/`LabeledSpan` imports (`:26`). Rewrite the false module header
  (`:11-17`) that still sells `Rule::check`/`run` as the architecture — `engine.rs`
  superseded it (`specs/10-contracts.md` "kill the heuristic rule registry"). KEEP
  `Workspace`/`Diagnostic` struct/`Severity`/`render`/`any_error`. Zero production edits.

- FEATURES-COMPANIONS-DROP (D4; `src/extract.rs` + `engine.rs` comment; low risk;
  parallel-safe): remove the never-read `Features.companions` field (`extract.rs:175`),
  its population in `skill_features` (`:243-247`), and the ~6 `Vec::new()` placeholders.
  KEEP `Skill::companions` (needed for byte-faithful import copy, `import.rs:295`) —
  only the dead *feature* projection goes. The sole ex-consumer (`companion-refs`) was
  killed as unsound; a companion-reference clause is inadmissible (`specs/10-contracts.md`).

- BODY-HASH-DROP (D2; `src/import.rs` + `drift.rs`; low risk): remove the write-only
  `RollupEntry.body_hash` — computations (`import.rs:306/345/460`), emit (`:573`), re_add
  rewrites (`drift.rs:1037/1055`), and the "five-column row" doc. No production reader
  exists; law 5 provenance is `{source_path, import_hash}` only (`specs/20-surface.md`).
  Collapses to the four columns production uses.

- SHA256-HOIST (U2-sha256 only; hoist `sha256_hex` to one shared util, remove the 3
  duplicate defs at `skill.rs:350`/`rule.rs:358`/`import.rs:597`/`drift.rs:322`; generic
  `&[u8]`, no kind-typing lost). Touches import.rs/drift.rs → `blockedBy BODY-HASH-DROP`
  (shared files). NOTE: do ONLY the sha256 hoist. Do NOT hoist the frontmatter round-trip
  helpers out of skill.rs/rule.rs — that is accepted debt (see below).

- DEPENDENCY-EXISTS-FENCE (R2 code; `src/contract.rs` + `engine.rs`; low risk;
  parallel-safe): spec `2bc1013` fenced `dependency-exists` as held-back. Match the code:
  REJECT `Predicate::DependencyExists` in admissibility (like an unknown/held predicate)
  so a hand-authored clause fails loudly instead of `engine::decide` returning
  `Outcome::Indeterminate` (`engine.rs:333`) — the silent no-op law 1 forbids.

- KIND-ENTITIES-RECONCILE (R1 code; `src/compose.rs`; low risk): spec `2bc1013` dropped
  the phantom `[kind.<name>.entities]` table. Match the code: fix the false comment
  (`compose.rs:66`, "folded in elsewhere" — it is folded in nowhere; nodes derive from
  `features.id`), and handle `[kind.*.entities]` consistently — both parse paths should
  REJECT it uniformly (today `parse_kind_layer` rejects but `parse_custom_kind` silently
  drops, `:1132`/`:1171`). `blockedBy CONSOLIDATE-REQUIREMENT` (shares compose.rs).

- MATCHSELECTOR-ROLE-DROP (U1; `src/compose.rs` + `roster.rs`; the one true accretion
  residue): after the consolidation, drop the `role:` frontmatter opt-in — the
  `MatchSelector::Role` variant + both parse arms (`compose.rs:1637/1713`), the read
  (`roster.rs:412-416`), and the empty-marker admissibility guard (`roster.rs:258-269`,
  pure seam-policing). `satisfies` is the sole artifact-side opt-in; `match` = name/glob
  is the contract-side path (spec `specs/10-contracts.md` fill vocabulary). Spec-aligned
  deletion. `blockedBy CONSOLIDATE-REQUIREMENT` AND serialize after KIND-ENTITIES-RECONCILE
  (both edit compose.rs). Audit rated this risk-high only as a standalone pre-fold change;
  as a post-fold delete over the unified type it is a clean removal.

- ACCEPTED DEBT (do NOT file): the frontmatter round-trip helpers duplicated across
  skill.rs/rule.rs (split_frontmatter, pod_scalar_to_string, pod_hash_to_json,
  json_to_toml_value, toml_item_to_json, toml_value_to_json) stay duplicated — hoisting
  them contradicts the documented "one artifact kind per module" self-containment tradeoff
  (`.claude/rules/rust.md`; `rule.rs:24-26`). Honoring the documented decision, not
  overriding it.
