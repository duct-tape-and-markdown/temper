<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## CI Node 20 deprecation (filed 2026-07-24) — harness maintenance

11. **Both workflows pin actions that target Node 20, which GitHub is
    retiring; the runners already force them onto Node 24.** Every job in
    the v0.0.12 release run (30056541441) annotated it. Affected pins:
    `actions/checkout@v4`, `actions/setup-node@v4`,
    `actions/upload-artifact@v4`, `actions/download-artifact@v4` — in
    `.github/workflows/release.yml` (6 uses) and `.github/workflows/temper.yml`
    (2 uses) [github.blog changelog 2025-09-19, "deprecation of Node 20 on
    GitHub Actions runners"; surfaced by CI annotations 2026-07-24].
    Non-blocking today (forced onto Node 24), hard-fails once Node 20 is
    dropped. Fix: bump the pinned majors (`checkout@v5`, etc.).
    **Disposition: human chore, not a build entry** — `.github/workflows/` is
    release/CI infrastructure outside build's writablePaths (`release.md`
    governs release.yml; "a release is interactive, never a build tick").
    plan: surface as a human `chore(ci)` item, do not route to build.
    *observed at bf4b5cd (CI annotations on release run 30056541441).*

