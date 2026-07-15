# Plan state

- Spec derived through: a9f7b9e
- Audited through: 2aed996
- Residue swept through: 2aed996
- This tick: Inbox drain (3 notes). (1) PACKAGING-CHANNELS supersession
  (John, 07-11) — verified on disk: `.github/workflows/release.yml` exists
  (linux-x64 + win32-x64 matrix + standalone tag-asset step), root
  package.json still the private `temper-flume-harness`, launcher +
  exact-pinned optionalDependencies live on sdk (`@dtmd/temper@0.0.7`). Both
  the retired entry's file claims dead → retired PACKAGING-CHANNELS, filed
  PACKAGING-CHANNELS-REMAINDER (parked: darwin notarize + channel-3
  bundle/marketplace.json + v0.1 version lockstep). (2) Dogfood findings
  routed against HEAD (PR #19 recompose is docs/example only — src/sdk code
  paths unchanged, all reproduce): DRIFT-REAP-PATH-NORMALIZE (drift.rs:634
  harness_root = workspace_dir.parent(); `./.temper`→`.` vs `.temper`→``
  mis-spells owned_paths → live byte-faithful projection reaped+deleted at
  1314 — 2nd-cut#1, folds 1st-cut#2 install dry-run contradiction),
  EMBED-RENDER-FENCE-FREE (renderMemberFence:139 fences every blocks() value
  despite a render hook — 2nd-cut#2), EMBED-FILL-DEFER (checkFills:196 can't
  see layout satisfies, refuses; blockedBy EMBED-RENDER — shared emit.ts —
  1st-cut#1), MENTION-EMBEDDED-TARGETS (declaredAddresses:551 omits embedded
  members + no Member→Mentionable adapter — 2nd-cut#4+5). Two forks
  registered: (discovery-nested-root-fence) 1st-cut#3, (prose-interleaving)
  2nd-cut#3. Inbox emptied.
- Queue: DRIFT-REAP-PATH-NORMALIZE, EMBED-RENDER-FENCE-FREE,
  MENTION-EMBEDDED-TARGETS (open, disjoint files); EMBED-FILL-DEFER
  (blockedBy EMBED-RENDER-FENCE-FREE — shared sdk/src/emit.ts);
  PACKAGING-CHANNELS-REMAINDER (parked, human release actions).

Plan continues: yes — post-ship reconciliation over the sdk launcher window
(2aed996..HEAD touched sdk/bin/temper.js + sdk/package.json — release first
cut); Audited/Residue cursors unmoved this tick.
