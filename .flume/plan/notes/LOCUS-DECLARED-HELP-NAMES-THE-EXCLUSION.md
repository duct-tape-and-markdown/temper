# LOCUS-DECLARED-HELP-NAMES-THE-EXCLUSION

Shipped. One helper, `drift::locus_alternative_remedy(kind, custom)`, now speaks
the second answer for both `check`'s `root.locus-declared` finding and guard's
pending-write message: a built-in names `relocate(<kind>, { exclude: [...] })`,
a custom kind keeps the `local` remedy.

Side effect worth knowing: guard's pending-write message previously carried no
second answer at all, so a custom kind's guard refusal now also names `local` —
the same words `check` already spoke for it.

The shell-edge locus header (`GUARD_SHELL_EDGE_LOCUS_MESSAGE`) stays
kind-agnostic; the per-finding lines under it are `check`'s and carry the
remedy.
