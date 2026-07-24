<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

#15 — `schema --kind`'s flag help still says "(`skill`, `rule`)"
(`src/main.rs`, the `Schema` command's `kind` field doc) after
SCHEMA-KIND-DOMAIN-WIDEN (141448f3) widened the domain to every
YAML-frontmatter kind; the unknown-kind error already enumerates the true
set (agent, command, rule, skill — verified against the built binary).
Two-word residue of #13, missed by the entry and by post-ship
reconciliation (f0ef148a read it clean). Ruled (interactive, 2026-07-24):
build-ready one-liner — say the domain generically or enumerate the same
set the error message derives; a hardcoded list here re-fossilizes.
observed at f0ef148a.
