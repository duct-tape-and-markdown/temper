# Parked: the guard never places a memory-local member, so the named test is green on base

The divergence is real, but the case the entry names cannot show it. Both guard loci sets
filter on `gate::site_class` (src/gate.rs:185), which returns `None` for any
`Commitment::Local` kind. That covers `guarded_loci` (`matches_governed_locus`) and
`guarded_layout_loci` (`pending_finding`'s governed branch). `hosted_member` takes only
nested-file kinds, and memory-local governs a glob. So a pending write to
`build/CLAUDE.local.md` gets no placement on base, the same as `check`. The tests[] line
already passes there and would revert.

The only guard caller of `discovers_path(local=true)` that can be reached is
`nested_file_placement` (src/import.rs:424), the hosted child under a local host. Example:
a local host `guide` at `docs/guides` `*/GUIDE.md`, child template `**/*.md`, unit
`docs/guides/operate` not ignored, `docs/guides/operate/build/` gitignored. A pending
write to `.../build/x.md`: check's `**` reads the index under the descended unit, never
enters `build/`, and finds no member. The guard checks only the fences, places a child,
and raises the layout finding.

Fix shape (not built): a single-path mirror of `Reach::children` over the locus root and
glob. A literal segment or a wildcard leaf passes the fences only. A wildcard or `**`
directory step from a dir the index descended (`discovers_path(dir, false)`) needs the
child to be index-admitted too. `admitted` needs an explicit directory flag for its leaf,
so a not-yet-created ignored dir is judged as a dir.

Re-file on that case: guard and check agree on a hosted child in an ignored subdirectory
of a local host unit. Files: src/import.rs, plus tests/it/install.rs next to
`local_host_child_harness`. If wanted, the memory-local case fits as a pins[] line.
