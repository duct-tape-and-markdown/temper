# Swallow detection now spans every later region

Shipped: `Layout::read` holds the first heading-with-children a field or
collection binds until every later region is judged; any later
heading-bound region that starves refuses with that swallow.

Behavior change beyond the repro: a prose region between the swallow and
the starved region no longer clears the pending swallow (it did before).
Prose is not heading-bound, so it cannot vindicate the binding; 0048's
rule names "a later region stays unbound" with no prose carve-out.

Still out of scope (unchanged): a childless replacement heading shifts
silently (#41).
