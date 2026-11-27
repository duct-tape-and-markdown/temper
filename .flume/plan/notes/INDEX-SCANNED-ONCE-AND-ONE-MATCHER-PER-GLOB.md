# INDEX-SCANNED-ONCE-AND-ONE-MATCHER-PER-GLOB shipped

Measured first: a gate run over a one-skill fixture made 59 locus-scan
requests over 15 distinct (root, glob, flavor) loci. Call sites repeated
the same scans: the gate's per-kind loop, `declared_governed_paths`,
`host_units`, compose (three sites), admissibility, json_manifest.
`Discovery::scan` now memoizes each locus's result, so the index is scanned
once per distinct locus per run. `import::locus_scan_count` is the pin.

How "each index scanned once" was read: once per locus, not one traversal
for all loci together. Six distinct loci still lead with `**` from the
harness root (memory x2, memory-local, rules, skills, output-styles). Each
one walks the whole index once. Collapsing those into one pass needs the
loci known up front, either a GlobSet over every declared locus or a flat
file list. The local flavor's raw-disk reach complicates the flat-list
route. Neither was measured as the residual. If a field timing still
points at discovery, measure these six before cutting.

Matchers: `compile_glob` hands back `Rc<GlobMatcher>`. A clone carried a
cold regex cache that rebuilt on its first match. `collect_glob` resolves
each segment's matcher once per scan, not once per child.
`Reach::children` borrows the index's child list (`Cow`) and no longer
copies it per directory.

Memo caveat: a local-flavor scan's raw-disk reads are now memoized for the
whole life of a `Discovery`, as the index already was. A caller that writes
files and then rediscovers through the same `Discovery` would see stale
results. No test exercises that, and nothing in the tree does it today.
