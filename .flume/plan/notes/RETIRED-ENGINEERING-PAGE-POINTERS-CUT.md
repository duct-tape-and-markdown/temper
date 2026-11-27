# Retired engineering-page pointers cut

All 42 `engineering.md` pointers across 14 files are gone; acceptance rg returns
nothing. Where the cited section title was the only statement of the constraint,
it stays in prose without the path: graph.rs ("a shared concept is one type"),
member_address.rs ("one job, which has one home"), compose.rs and
tests/it/common/mod.rs ("a seam gate reads what the real writer wrote"),
session_start.rs ("a vacuous green"). Nothing was repointed at the playbook.

Observed: check_cost.rs doc lines that lost a wrapped parenthetical now run
past 100 columns (rustfmt does not wrap comments, so no gate flags it). I left
them unreflowed to keep the diff small.
