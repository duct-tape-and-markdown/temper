# Plan state

- Spec derived through: a53eee4
- Audited through: c93eeed
- Residue swept through: e6d0311
- This tick: Spec delta (job 2). One commit past the prior cursor —
  a53eee4 (0018, "the projection is not the database"): an embedded
  member's facts move from re-parsed-fence to declared lock rows;
  `CustomKind::fold_members` retires with no fallback; the embedded-locus
  format becomes writer-only/unconstrained. Derived into two entries,
  sequenced independently-green per the decision's own consequence note
  ("Implementation... is build's to derive from this delta, sequenced
  independently-green"): NESTED-MEMBER-LOCK-ROW (open — adds the new
  `[[declaration.nested_member]]` row family, SDK + Rust + the embedded
  lock, additive, read side untouched) then RETIRE-FOLD-MEMBERS
  (blockedBy NESTED-MEMBER-LOCK-ROW — the one atomic swap of the read
  side, retiring `fold_members`/`overlay_templates`'s fence dependency).
  Did not file a separate entry for the format-admissibility split
  (representation.md "kind"): confirmed on disk (`src/kind.rs`) that no
  code enforces the admissibility bars today — `Format::YamlFrontmatter`
  is a bare label and `CustomKind.format` is documented inert until a
  future DECLARED-FRONTMATTER-ADAPTER — so the file-locus/embedded-locus
  split has nothing to relax yet; already vacuously true. Cursor advances
  to a53eee4 (fully routed). Verified no live inbox/refactor content, no
  new src/tests/sdk commits past either the audit or residue cursor
  (`git log c93eeed..HEAD` and `e6d0311..HEAD`, both `-- src tests sdk`,
  empty) — jobs 3 and 4 stay current, untouched this tick.
- Queue: NESTED-MEMBER-LOCK-ROW (open, pickable) → RETIRE-FOLD-MEMBERS
  (blockedBy) → PACKAGING-CHANNELS (parked, unchanged). Disjoint: the two
  new entries share `tests/lock_declaration_rows.rs` but never run `open`
  simultaneously (serialized by blockedBy); PACKAGING-CHANNELS shares no
  path with either.

Plan continues: no — inbox and refactor-captures are empty, the spec delta
is now fully routed, and no commit touched src/tests/sdk past the audit or
residue cursors. Hand off with one pickable entry (NESTED-MEMBER-LOCK-ROW)
for build.
