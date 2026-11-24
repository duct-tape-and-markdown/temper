# Derive 0079: the guard is check at a narrower trigger (GH #66)

Spec commit a97f66b2 landed at the adoption cursor; derive `specs/distribution.md`, "Per tool call", final sentence.

The guard keeps its own member enumeration: `guarded_loci` (`src/main.rs` ~836) filters kinds itself and never excludes layout kinds, so a layout document is judged against provenance rows it never carries (#66: `root.locus-declared` on every Bash call for declared documents). `locus_member_sites` (`src/install.rs` ~1291) walks with plain walkdir, where check's discovery honours ignore rules. The post edge runs `config_stale_from_doc` only (`src/install.rs` ~1262) and skips the layout-import, include and input staleness check judges.

One entry: the guard's locus and freshness answers come from check's site classification (`src/gate.rs` `site_class`) and drift judges; the private kind filter retires. Pin: on the gauntlet, the guard's findings at both edges are a subset of check's.
