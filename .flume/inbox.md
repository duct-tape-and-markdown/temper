<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

<!-- The notes below are carried from PR #20 (closed unmerged; branch
`field/consumer-notes-0710` preserved). Every code claim was adversarially
re-verified against f67303c before refiling. The lock-migration cluster was
drained at 06c44b1 → fork `(lock-upgrade-migration-posture)` plus its three
instance entries (SATISFIES-LABEL-QUALIFY, LOCK-SPELLING-REAP,
EMIT-INTO-REROOT-REAP). The five remaining notes — the 0019-content layout
cluster (docs remainder, member-fence fork, layout-probe triple), the 0019
decision-record renumber, and the pack-kind field trial — drain in later
ticks. -->

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
