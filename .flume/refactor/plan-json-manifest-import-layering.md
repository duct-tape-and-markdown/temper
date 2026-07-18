## Surface

`src/json_manifest.rs`'s `Manifest::read_kind` (419-435) takes
`disc: &crate::import::Discovery` and calls `crate::import::discover_kind_files`
(426-431) itself — the formats subsystem reaching into the pipeline
subsystem's discovery mechanism (`import` is pipeline per
architecture.md's codemap).

This is an asymmetry against the sibling format: for frontmatter-shaped
kinds, `src/main.rs` does the discovery walk itself
(`import::discover_kind_files` at main.rs:1372, inside `resolve_kind_units`'s
file-governed branch) and then calls the *pure-parse*
`frontmatter::Member::from_source_rooted` (main.rs:1440) — `frontmatter.rs`
never imports `crate::import`. For manifest-shaped kinds, `main.rs`'s
`manifest_units` (1495-1508) just calls
`json_manifest::Manifest::read_kind(disc, kind)` (1501), and `read_kind`
does its own discovery internally — the only formats-subsystem function
that imports `crate::import`.

Same class of edge architecture.md's Invariants section already tracks
and has ruled three times over (0040: `drift → install`, `frontmatter →
builtin_kind` test-only, `extract`'s upward imports) and a fourth time
this cycle (`normalize_path`, `graph → address`) — each resolved by
moving code to its layer-correct home, never by amending the map to admit
the edge. This is a fifth, currently undocumented instance: `read_kind`'s
discovery half structurally belongs beside `discover_kind_files` in
`import.rs` (or `manifest_units` in main.rs should do its own discovery
call, mirroring the frontmatter path at main.rs:1372), leaving
`json_manifest.rs`'s `read_kind` a pure parse-per-file function.

Fifteen call sites reference `Manifest::read_kind` (grep-verified):
`src/main.rs:1501`, and fourteen more across
`tests/hook_kind.rs` (78, 120), `tests/manifest_adapter.rs` (142),
`tests/mcp_server_kind.rs` (68), `tests/installed_plugin_kind.rs` (68, 82,
112, 198), `tests/known_marketplace_kind.rs` (58, 89, 176) — a fix
threading a `Vec<PathBuf>`/discovered-files param through `read_kind`
instead of a `&Discovery` would touch every one.

## Observed at

9e197d6 (HEAD when observed — posture sweep, formats subsystem, job 4).

## Suggested consolidation

Two candidate shapes, a design call for whoever drains this:
(a) move the discovery call out of `json_manifest.rs` into `main.rs`'s
`manifest_units`, mirroring `resolve_kind_units`'s frontmatter branch
exactly, and narrow `read_kind` to take the already-discovered file list;
or (b) keep the discovery-orchestration convenience but home it in
`import.rs` itself (a `manifest_units`-shaped helper there) rather than in
the formats subsystem, matching how 0040 homed the `drift → install`
edge in a new shared module. Either way `json_manifest.rs` stops
importing `crate::import`.
