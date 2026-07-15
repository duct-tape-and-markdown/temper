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
EMIT-INTO-REROOT-REAP). The 0019-content layout cluster drained at c535331 →
three open forks (custom-kind-consumer-docs, member-fence-dead-text,
layout-kind-heterogeneous-corpus) + LAYOUT-OVERLAY-CHECK-GAP (probe finding 2;
finding 1 verified working, no entry). The two remaining notes — the 0019
decision-record renumber, and the pack-kind field trial — drain in later
ticks. -->

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
