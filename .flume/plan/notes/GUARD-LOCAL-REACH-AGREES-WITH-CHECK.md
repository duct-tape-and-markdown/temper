# Guard placement under a local host now mirrors check's Reach

Shipped in src/import.rs: `Discovery::local_scan_reaches` plus `steps_reach`, a
single-path mirror of `collect_glob` over `Reach::children`. It runs only for a
local host in `nested_file_placement`. A non-literal directory step out of a
directory the index descended needs the child descended too. Any other step stays
fence-only, which `discovers_path` already judges. For an existing directory the
index answers (the one owner). For a directory a pending write would create,
`admitted` decides, and it gained a `leaf_dir` flag so the dir-only pattern `build/`
matches a directory that does not exist yet.

Tests: the tests[] line and the pins[] line are both in tests/it/install.rs.
`local_host_child_harness` is now a wrapper over `local_host_harness(label,
gitignore, child)`. I checked the tests[] line: it is red on base and green with
the fix.

Debt seen: `steps_reach` and `collect_glob` now spell the same segment walk twice,
once enumerating and once deciding. A shared step rule, a fn deciding "is this
step listed from the index", would keep them from drifting. I left that for
another tick because it would be a refactor beyond this entry.
