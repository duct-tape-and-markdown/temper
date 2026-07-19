## Invariant served

`plan-state.md`'s marker mechanic (the harness regex reads all three
values) — specifically its `after-build` carve-out: "iff the **only**
remaining live job is the posture sweep AND pickable entries exist —
ready work ships first, the sweep resumes when the wave hands back."
Cost observed at f01ba54 (this tick, SETTINGS-MANIFEST-PROVIDER-FACT-LEAK):
closing this tick with the posture sweep mid-rotation (frontier still
holds `tests/builtin_lock_frozen.rs`) and two pickable `open` entries in
the queue is exactly the overlap `after-build` names, yet
`posture-sweep.md`'s own closing-marker bullet reads unconditionally —
"While the frontier is non-empty, the tick's closing marker is
`Plan continues: yes`" — with no carve-out for pickable entries. Read
together, the two rules disagree on this tick's own marker; only cross-
referencing both (rather than either alone) surfaces it. This is the
restatement-drift class the open-questions.md "`.flume/` is ungoverned by
temper" record already re-armed over (prompts/rules/READMEs as
hand-synchronized restatements).

## Diff

```diff
--- a/.claude/rules/posture-sweep.md
+++ b/.claude/rules/posture-sweep.md
@@ -28,10 +28,12 @@
   never re-sweeps or re-draws it, even where fresh judgment would cut
   the boundary differently — the cursor decides coverage, never
   re-derivation.
 - **The rotation closes when the frontier empties.** Untouched modules
   never enter the frontier, so a quiet tree closes in one tick, never
   one tick per skip. **Quiet-on-clean is the normal verdict**,
   recorded by advancing the cursor alone.
-- **An open rotation is live input.** While the frontier is non-empty,
-  the tick's closing marker is `Plan continues: yes`; the rotation
-  drives itself to close and is never left waiting on a forced wake.
-  Hibernation is the empty frontier's verdict alone.
+- **An open rotation is live input.** While the frontier is non-empty,
+  the tick's closing marker follows `plan-state.md`'s mechanic —
+  `after-build` when pickable entries exist (ready work ships first;
+  the sweep resumes when the wave hands back), `yes` otherwise — so the
+  rotation always drives itself to close and is never left waiting on a
+  forced wake. Hibernation is the empty frontier's verdict alone.
```

## Expected settling

`posture-sweep.md` and `plan-state.md` state one marker mechanic instead
of two independently-worded ones that happen to collide exactly when a
posture-sweep tick files a pickable entry — the common case, not an
edge case. A future tick reading either page alone reaches the same
marker.
