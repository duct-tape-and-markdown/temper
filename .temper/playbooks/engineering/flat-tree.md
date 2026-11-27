# The tree stays flat

`src/` is a flat module list and stays one. The evidence-backed modern
practice for this size class is flat-over-nested — "even comparatively
large lists are easier to understand at a glance than even small
trees," and tree structure deteriorates where flat lists cannot
(matklad, "Large Rust Workspaces", retrieved 2026-07-17). A split
lands as a new flat module; directories are not adopted ahead of a
genuine multi-crate need, which no part of the tree has.
