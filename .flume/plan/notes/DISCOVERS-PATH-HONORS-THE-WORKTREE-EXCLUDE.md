# Worktree common exclude: shipped, one boundary left open

Both tests[] lines now live in tests/it/install.rs and were checked red on
the pre-fix src: the walk through `install::discover`, the single-path
query through `temper guard`, which is discovers_path's public caller.
The shared helper `linked_worktree_excluding_a_rule` builds the fixture
with a real `git worktree add`.

import::git_dirs is the one resolver (git dir plus common dir). tap::log_path
calls it and keeps its old behavior: a pointer with no `commondir` (a
submodule) still falls back to the local log.

Open boundary: only the harness root's pointer is resolved. A `.git`
pointer file in a nested sub-checkout below the root is still not read, and
the walk and discovers_path both miss it the same way. No spec line asks
for more yet.

Two unit tests stay in src/import.rs as unjudged coverage (exclude
precedence below a .gitignore negation, and the resolver's three .git
shapes). They are named so that neither matches a tests[] line.
