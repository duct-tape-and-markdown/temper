# `supporting-doc.degree` claims what temper cannot decide; it should claim the untyped reference

Replaces the note filed earlier today under this name. Verified on main with a fixture of two unrepresented skills, one linking its doc with `[the guide](guide.md)` and the other naming it in a code span: both docs draw `supporting-doc.degree` with incoming 0. temper rightly counts neither, because edges are declared, never mined (intent invariant 1). On one real harness it fired 168 times.

The advisory asserts "nothing points at this doc", and on an unrepresented harness temper cannot decide that. Its guidance tells authors to do what they already did. What temper can decide on any harness is that **no typed reference reaches the doc**, and that is the real risk: a prose reference breaks silently on a rename or move (John, 2026-11-25: a prose reference to a file or symbol is "a maintenance burden waiting to happen"; a typed one is known when broken).

Reword the clause's claim and guidance to that decidable fact. Its remedy is the typed reference: `mentionOf`, or the path mention (2bdb60c3). On a represented harness it stays per-doc. On an unrepresented one it collapses to one coverage line ("N supporting docs reached by no typed reference"), never a per-doc finding whose remedy cannot work there (`contract.md`, "well-formedness": no unfillable selection).
