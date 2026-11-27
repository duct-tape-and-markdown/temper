# One fenced tree index per run

Shipped. Observations for plan:

- Local-locus scans (`import::Reach`) now read the shared fenced index instead of a
  second whole-tree walk with the rules waived. A local kind reaches ignored
  documents only where its own locus names them: literal segments are stat'ed, a
  directory the index never descended (ignored, or `.temper/`) is listed raw, and a
  wildcard leaf lists its directory raw. A wildcard (`**`) never descends an ignored
  directory, so `build/CLAUDE.local.md` inside an ignored `build/` is no longer a
  `memory-local` member. This follows from the rust rule; no test pinned the old
  reach. Divergence left: `import::discovers_path(.., local=true)` (the guard's
  pending-write placement) still admits that path, so guard and check can disagree
  on a local document inside an ignored directory. Worth an entry to align them.
- The counter moved into `import::ignore_walk` (`index_pass_count`), so every walk
  riding the shared ignore policy is counted. `walk_count` and
  `install::locus_member_site_walk_count` are retired. Raw `read_dir`s from local
  scans are bounded by the locus and are not counted. `compose::repo_file_set`
  (walkdir) stays out of scope, as the entry says.
- Two edge behavior changes from the `.claude/` stray scan now reading the fenced
  index: a nested governed root sitting directly under `.claude/` is dropped, not
  named a stray, and entries enumerate in name order rather than readdir order.
  The dogfood `check` output on this repo is byte-identical before and after.
- `shell_edge_findings`, `coverage_note::check` and
  `compose::governed_member_address` take `&Discovery`. `LayoutGuard::read` holds
  it too. `run_guard` owns one per call for both edges.
