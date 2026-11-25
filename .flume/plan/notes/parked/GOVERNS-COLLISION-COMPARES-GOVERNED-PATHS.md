# Parked: path-overlap collision fires memory against every custom *.md kind

I built it: a decidable joint walk over the two templates' segments plus the
union of both kinds' excludes. It tries literal, exclude-named and one fresh
value per segment, with a visited set over (i, j, exclude progress). The three
new tests went green. The full suite went to 20 red, all outside the fence, so
nothing landed.

1. **Design gap.** `memory`'s `**/CLAUDE.md` overlaps every custom markdown
   kind: `docs/*.md` shares `docs/CLAUDE.md`. A relocated `rule` at
   `custom-locus/rules/*.md` hits the same thing, because memory excludes
   only `.claude/rules`. The check is right by 0081: Claude Code does load a
   subdirectory `CLAUDE.md` as memory. But for a `docs/**/*.md` kind the
   adopter has no remedy. Excludes are starless paths and names narrow by
   clause, never by pattern. 0084 rejects most-specific-wins and does not
   say who owns `CLAUDE.md` inside a custom locus. That takes a ruling.
   Candidates: memory yields to custom loci, or a filename-level exclude.
2. **Fence.** Even after a ruling, these red tests carry custom `*.md` kinds
   without excludes. They sit in acceptance, coverage, coverage_note, emit,
   gate_fail_loud, gauntlet (snapshots), graph, layout_kind, local_locus,
   lock_declaration_rows, requirement_roster and session_start. Their
   fixtures, or the ruling's mechanism, need a wider fence.
3. **Dogfood.** `cli::self_host_check_is_clean_over_tempers_own_surface`
   goes red because the committed `.temper` lock's `memory` row carries no
   `exclude`, and its `rule` row is `*.md`. The lock predates 0084 and needs
   a human re-emit before this entry lands.

Acceptance as written ("the built-in set draws none") holds once the lock is
current. It is the custom-kind consequence nobody has decided.
