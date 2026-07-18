## Surface
`read.rs`'s `narrate_citers` (778-810) is the shared leaf-grain
citer-narration helper (used by `impact_leaf` at 754 and `context_leaf` at
930), filtering citations on the full four-part leaf address
(`target.member`/`kind`/`key`/`child_path`). `context_member_one`
(983-1061) re-implements the same filter-then-narrate shape inline
(1017-1041) at member grain — filtering only on
`target.member == features.id`, and printing each citer's full leaf address
in its message (unlike `narrate_citers`' plain "cites it"). Same shape
(filter citations by a predicate, empty/non-empty branch, format a bullet
list), a second implementation rather than a shared one.

## Observed at
7ac498a

## Suggested consolidation
Generalize `narrate_citers` to take a predicate closure and a formatting
choice (whether to print the leaf address), or conclude the grains are
different enough (leaf address known vs. names-the-address) to warrant
staying separate and rename the member-grain block to make the
non-duplication explicit — a design call on whether the two narrations are
the same job at different grain or genuinely distinct jobs.
