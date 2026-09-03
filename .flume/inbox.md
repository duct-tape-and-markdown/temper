<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at 0d5de600 (surfaced while hand-landing EMIT-BANNER-OWNERSHIP-MOVE,
  GH #37) — the read face (`Member::from_source_rooted`) returns placed
  metadata as part of `body`: the block-level banner on a frontmatterless
  markdown projection is the body's first block, so `extent`/`layout`
  contracts range over it. Not new — install's banner was read the same way
  and this repo's own CLAUDE.md carries one — but now every frontmatterless
  projection has it from first emit, so the asymmetry is universal: the
  frontmatter note is inside the YAML block (never body), the banner is in
  the body. Spec question (representation.md "member": what `body` is),
  not build's to settle: should the read face strip placement lines from
  `body` so contracts see authored prose only? Session position: yes,
  strip — metadata is not prose; the emit hash still covers the bytes.
  tests/memory_contract.rs's third assertion names the gap.
