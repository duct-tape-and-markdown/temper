# LAYOUT-ENTRIES-READ-OFF-THE-DOCUMENT-AT-CHECK: check half shipped; two adjacent gaps

compose::assemble_lock_family now resolves every committed layout kind and
replaces each discovered host's stored nested_member rows with the ones its
document lowers. The replacement is narrowed to the kinds the layout's
Collection regions name. A host may also nest `templates`-declared rows that
are no layout entry: graph.rs embedded_edge_targets hand-authors such rows on
a collection-less layout host, and replacing every row on the host broke all
five. A host the walk no longer finds keeps its lock rows.

Cost: a committed layout document is now read and parsed once more per check
run. assemble_lock_family resolves it, and kind_units_and_features resolves it
again. The check_cost parse pins were left alone because they cover local
kinds only. A pin over a committed layout kind would make the doubled pass
visible, and hoisting the units out of the family pass would remove it.

Same staleness, not fixed here: a committed host's `satisfies` rows and its
layout_prose rows still come off the lock. Editing a `satisfies` edge slot
or a verbatim prose region without a re-emit still goes unseen by check. The
entry scoped this to collection entries, and the fix has the same shape: the
same pass already lowers both into rows (rows.satisfies / rows.prose).
