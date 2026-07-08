<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- Field report (dogfood adoption, 2026-07-07): the scaffold↔published-SDK
  version skew bit for the second time. `install --yes` scaffolds HEAD's
  two-arg `file(import.meta.url, "…")` (0009 module-relative) but ensures
  `^0.0.4`, whose published `file(path)` is one-arg — JS swallowed the extra
  arg and emit died ENOENT on a cwd-joined `file:` URL (`src/install.rs`
  `SDK_VERSION_RANGE`'s own comment records the identical ^0.0.2→^0.0.4
  incident). The gap is systemic: nothing gates the scaffold's SDK-surface
  assumptions against the version range install writes — cargo tests ride
  the workspace sdk, never the registry tarball. Candidate shapes for plan:
  pin exact + a release-ritual bump, a scaffold-vs-published contract test,
  or install preferring a workspace sdk when one is present (what the
  dogfood hand-wired: `file:../sdk`). Also noisy-but-survivable: the emit
  failure surfaces as a raw Node stack through the miette report.