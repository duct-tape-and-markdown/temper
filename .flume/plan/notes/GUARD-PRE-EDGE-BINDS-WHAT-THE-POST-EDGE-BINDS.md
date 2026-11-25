# GUARD-PRE-EDGE-BINDS-WHAT-THE-POST-EDGE-BINDS: shipped; two readings for plan

Both `.`-root skips are gone (`matches_governed_locus`, `LayoutGuard::pending_finding`);
`import::discovers_path` already prunes ignored and fenced paths per path. A named-field
layout identity is read off the pending bytes the payload carries, through
`Member::from_text_rooted` (frontmatter.rs, the on-disk reader now delegates to it) and
`SiteIdentity::Pending(Option<&str>)` (compose.rs, replacing `Placement`).

- Widened on purpose: `.mcp.json` in a lock with no registration rows now binds at the
  `mcp` container's locus (the post edge already did). The neighbours pin is restated.
  `reference/CLAUDE.md` in three projection pins is now a locus finding, not a pass; the
  pins assert it is never the projection finding.
- The pending bytes ride `install::pending_manifest`/`PendingManifest`, which now serves
  any pending document, not only manifests. A rename (`pending_text`) is a cheap follow-up;
  not taken here to keep the diff on the entry.
- A `Write` with no `content` (only the test fixtures send one) or an unreconstructable
  `Edit` falls back to the placement: a named-field kind binds nothing there, as before.
