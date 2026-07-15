<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `temper check <harness-root>` is a **silent half-gate**: with workspace
  `.` at a harness root, `drift::read_declarations(workspace)` finds no
  lock, so requirements, satisfies fills, custom kinds, and declared edges
  all vanish while built-in kinds still resolve off disk — exit 0, looks
  green. Verified in the field (examples/base-harness): `check .` stayed
  green with a `required: true` requirement unfilled; `check .temper` exits
  1 with `requirement.unfilled`. Bare `temper check` (no arg) resolves
  correctly, so the trap is only the explicit spelling — and
  `src/install.rs:88` **hardcodes** `temper check . --reporter
  session-start` into every adopted harness's SessionStart hook, so every
  install ships the half-gate reporter. The empty-assembly tripwire
  (check.rs) only fires at resolved_members == 0. Both this repo's and the
  example's hooks hand-fixed to `.temper` (aba7e47, 549969f); the product
  fix is install's wiring + the argument semantics (resolve `<arg>/.temper`
  like the default does, or fail loud). Verify `temper guard .`'s rooting
  while there — same argument shape. observed at aba7e47
- A composed **mention cannot target a discovered member**: authoring
  `text\`… ${{address: "source:main", display: "src/main.js"}} …\`` in the
  example's skill fails emit with "mention of `source:main` resolves to no
  declared value — a mention cannot dangle" (sdk/src/emit.ts,
  renderTextBody). But `source:main` is a real corpus member — the engine
  resolves `implemented-by` edges against it and `graph.route` fires on it.
  The SDK enforces corpus law against a program-scoped universe (declared
  program values), the same shape the fill-check deferral resolved: SDK
  keeps failing fast on definitely-dangling program addresses, and defers
  discovery-locus kinds' addresses to the engine's check-time mention
  edges. Blocks the primer's skill→script edge demo. observed at aba7e47
- `temper emit --into <dir>` pointed **inside an adopted harness** reaped 7
  live projections at the repo root (CLAUDE.md, five rules, a skill) —
  `--into .temper` re-rooted the projection tree, the real projections
  became ownerless, and the byte-faithful safety line let them be deleted.
  Restored via git; plain `emit` then reported 0/8/0 clean. The reap fix
  (e7b859a) covered path spelling, not re-rooting: emit should refuse (or
  dry-run-report) a reap wave caused by an `--into` that re-roots an
  already-adopted harness. observed at aba7e47

- Decision 0022 (`specs/decisions/0022-glob-validity-joins-the-vocabulary.md`,
  human-ruled 07-15) resolves the `(builtins-coverage-predicates)` fork:
  admit a **glob-validity predicate family** (globs parse under `globset`,
  brace-aware), first consumers the `rule` and `skill` default contracts
  over `paths`; `tools-must-resolve` rejected permanently on invariant 2
  (recorded in 0022 — do not re-file). Work: `Predicate` enum variant +
  schema surface in `src/contract.rs`, the two default-contract clauses in
  `sdk/src/builtins.ts` with fresh raw cites, frozen-lock re-derive +
  tests. The two deferred skill shape predicates are NOT in scope (0022 is
  explicit they need their own design). observed at dc43554

<!-- The seven notes below are carried from PR #20 (closed unmerged; branch
`field/consumer-notes-0710` preserved). Every code claim was adversarially
re-verified against f67303c before refiling; three of the PR's proposals
collided with standing rulings and are reframed here — the member-fence
refusal demoted to a fork, the satisfies-join migration respelled to the
lock's ruled posture, the full-reap refusal split off to a fork. -->

- Ruled and encoded (0019-content): a prose-interleaved host is a layout
  source document. Unrouted remainder, docs only: the consumer guidance —
  "if your host mixes prose and members, declare a layout and author the
  document" — has no written home; no custom-kind consumer docs path
  exists yet, so the entry names the home too. observed at 0aa9e62,
  re-verified at f67303c

- Fork to register, not a fix: a `member.<kind> <key>` fence in a
  `text()`/`file()` body is dead text with no finding on any path. Live
  repro: the centercode harness was green pre-0018 with ~57 embedded
  members resolving at leaf grain; it re-emits and re-checks green with
  that whole layer silently gone (`explain`: "carries no nested member").
  PR #20 proposed a "cheap refusal" (fence info string parses as
  `member.` + a declared kind, no `nested_member` row matches the host →
  finding); the standing objection, on the record: the check scans prose
  for temper's own retired syntax — invariant 1's "matching is mining",
  and the `@import` precedent doesn't cover it (Claude Code executes
  `@import`; nothing executes a member fence) — 0019-content's rejected
  alternatives retired per-entity fences by name, and any document
  *quoting* a fence (docs about temper, this corpus) false-positives. If
  it enters at all it is a decision-gated advisory clause, with the
  migration-loss repro as its context — never a shipped refusal. observed
  at 8c00159, re-verified at f67303c

- Layout probe results (centercode testbed): (1) emit honors a relocated
  built-in's declared `content` — `temper::layout::unadmitted` refused
  loud and located before writing a byte; the 0019-loud posture works
  end-to-end on that path. (2) Untested divergence to close: the gate
  resolves a built-in kind via `overlay_builtin_kind`
  (`src/main.rs:896-919`), which lifts exactly `governs` + `templates`,
  never `content` — a document that *fits* its layout would emit rows
  fine while `check`'s reconstruction may silently fall back to the plain
  frontmatter read; same class as T18 (templates overlay), one fact
  later; unverified past emit. (3) Fork to register: a layout binds the
  whole kind, but a real corpus is heterogeneous, and 0019-content's own
  answer ("two kinds, or it is prose") collides with governance — what
  two kinds sharing a governs glob means is unspecified
  (`representation.md`'s "per-kind precedence" is the runtime artifact
  levels, not this). First question any consumer hits adopting layout for
  a built-in kind. observed at 8c00159, re-verified at f67303c

- Corpus hygiene: two decision records wear 0019 —
  `0019-loud-or-nothing.md` and `0019-content-is-a-declared-kind-fact.md`
  (0020 cites "0019" bare). Decisions are addressed by number, so the
  shared number is a live addressing ambiguity; 0021/0022 have since
  taken the next slots, boxing the pair in — one record renumbers to the
  next free number and its cross-references move with it. Plan-routed,
  not a hand-fix. observed at 8c00159, still live at f67303c

- Fork to register (the generator for the two notes below, plus the
  standing `--into` reap note above): what does an upgraded engine owe a
  committed lock an older engine wrote? The corpus says only
  "tool-written whole, never patched" (`pipeline.md`, "The lock"). Three
  live incidents now need a per-row answer (spelling fork → mass reap;
  bare satisfies labels → wrong findings; `--into` re-rooting → reap
  wave); the ruling should be one posture the instances hang off, never
  per-incident ad-hoc migrations. observed at f67303c

- Field incident, high-severity class (centercode, 07-15): the first
  emit under a post-e7b859a engine against a pre-e7b859a lock mass-reaps
  every live projection, silently green. Repro: lock rows spelled
  `./CLAUDE.md`-style; one emit reported `0 emitted, 21 unchanged, 21
  reaped, 0 orphan-drift`, exit 0 — deleted all 21 projections
  (CLAUDE.md, every rule, every SKILL.md) while writing a lock claiming
  them live and unchanged. Mechanism (confirmed at f67303c): e7b859a
  normalizes the workspace path before deriving `harness_root`, so the
  fingerprint pass keys files at the new spelling while the orphan sweep
  (`src/drift.rs:865-872`) joins the old rows' raw `source_path` strings
  against the new owned-paths set, finds none, and reaps their files —
  `to_lock_path` normalizes backslashes, never `./`. Damage is transient
  (the next emit re-emits all 21) but the window is real — a session
  launched between the two emits finds no harness — and the run lied.
  Fix: normalize spellings when *reading* lock rows too — the join key,
  not just the derivation — pure canonicalization, with the backslash
  handling as precedent; posture per the lock-upgrade fork above. PR
  #20's companion proposal (refuse a reap sweep about to delete every
  row while emitting nothing) is deliberately NOT filed as a fix: it
  trips a legitimate full teardown and has no spelled escape — it
  belongs to the fork. observed at 0aa9e62, mechanism re-verified at
  f67303c

- The gate's satisfies/graph joins key on bare member name, not the
  compiled `kind:name` label — a cross-kind name collision produces
  *wrong findings*, not silence (worse than a 0019-loud gap: this path
  lies specifically). Live repro (centercode, 07-15): a `dev-pack`
  member named `csharp` beside the `rule` named `csharp` cross-attributed
  each member's satisfies claim and mention edges to the wrong same-named
  member — false `requirement.kind` refusals both ways, false
  zero-degree findings, collateral on an uninvolved clause; renaming the
  four colliding members cleared every finding (clean bisect), and a
  second pass with the pack layer contract-free showed mere *existence*
  still poisons — no contract-side dodge exists. Spec anchor (the
  entry's `per`): `pipeline.md`, "The lock" — identity is a compiled
  label, the engine never resolves a collision, and two rows wearing one
  label is a malformed lock rejected at admissibility. The wire defect is
  `SatisfiesRow.member` carrying a bare id (`src/drift.rs:2311`) where
  `MentionRow` already carries `kind:name`. Writers:
  `sdk/src/declarations.ts` `satisfiesRows`; `src/drift.rs`
  `derive_layout_rows` (has `host_address()` in hand). Fold:
  `src/main.rs` `resolve_kind_units`, the `row.member == unit.id` loop —
  it runs per kind, which is the leak. Migration, spec-faithful (contra
  PR #20's "old behavior, collision and all"): a pre-fix lock's bare
  rows stay accepted where the bare label is unambiguous; bare rows that
  collide cross-kind are the malformed lock the spec already names —
  refused loud, one re-emit heals. Test surface that moves:
  `tests/common/mod.rs` `author_satisfies`,
  `tests/requirement_roster.rs` (4 rows),
  `tests/lock_declaration_rows.rs` (2), `tests/emit.rs` (1); the
  regression case is two kinds sharing a member name with a qualified
  row binding only its own kind's member. The centercode testbed is
  deliberately left red on these five findings as the live repro; the
  fix flips it green with no testbed change. observed at 0aa9e62, wire
  shape re-verified at f67303c

- Pack-kind field trial (centercode, 07-15) — a convention layer typed
  as three frontmatterless custom kinds works end-to-end: ten members'
  `file()` prose adopted byte-faithful from the pre-existing on-disk
  packs (first emit: 21 unchanged), every pointer site lifted from
  inline-code text to a `packMention` with identical display bytes, and
  the layer's duality invariant machine-checked — every pack reached
  (per-slice `degree incoming ≥ 1` advisory clauses), every pointer
  resolves (emit's dangling-mention refusal); `registration: []` (a kind
  reachable only via edges) was accepted. Two model gaps: (a) a single
  kind can't govern a directory-sliced corpus —
  `member_projection_path` (`src/drift.rs:509`) substitutes the glob's
  first `*` only, so glob `*/*.md` derives a broken path;
  kind-per-directory works but proliferates kinds — a consumer wants
  nested-glob derivation or a declared path template; (b)
  frontmatterless projections carry no managed-by note (`install`'s
  note is frontmatter-borne) — a hand-edit is hash-caught as drift but
  the file itself never warns; the banner wants a frontmatterless form
  (HTML comment) or the gap documented. observed at 0aa9e62,
  re-verified at f67303c
