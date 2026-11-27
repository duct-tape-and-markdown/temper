# Architecture — the choices the tree can't teach

This page holds what a competent engineer could not re-derive from
the profession: deliberate selections where respectable alternatives
exist, and the product thesis where it surfaces as a dependency rule.
General engineering — layering hygiene, codec/logic separation, thin
entry points — is the reader's own trade, judged by the posture
sweep, and is not restated here. Adjudicated boundary calls live in
`specs/decisions/`; a module's job lives in its own doc header; the
module list is the tree's to answer.

## The tree stays flat

`src/` is a flat module list and stays one. The evidence-backed modern
practice for this size class is flat-over-nested — "even comparatively
large lists are easier to understand at a glance than even small
trees," and tree structure deteriorates where flat lists cannot
(matklad, "Large Rust Workspaces", retrieved 2026-07-17). A split
lands as a new flat module; directories are not adopted ahead of a
genuine multi-crate need, which no part of the tree has.

## The provider face is data

The product thesis as a dependency rule: the engine is generic, and
Claude Code is data it loads (`builtin`, `builtin_kind` — the shipped
kinds and their cited format facts). The model (`kind`, `contract`)
compiles without knowing Claude Code exists, and a documented provider
fact — a kind name, a locus, a payload schema — lives in the provider
face, never as a literal elsewhere: that is the knowledge-form of the
same dependency, invisible to the import graph. A second provider is a
new data set, never a new engine.

The SDK mirrors the seam: `builtins` + `claude-code` are the provider
face of the authoring layer, and `generated/` is the machine-written
ts-rs boundary between the two languages.
