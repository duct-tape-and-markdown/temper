<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->








- A `format: "json-document"` (or `toml-document`) file kind whose glob contains `**` projects `<root>/<name>.md`: `member_projection_path` (`src/drift.rs`, the `glob.contains("**")` arm) hardcodes `.md`, written for `**/CLAUDE.md`. The emitted file matches neither the kind's format nor its own glob, so `check` then counts the kind `(0)`. Reproduce: a json-document kind at `root: "docs", glob: "**/sub/*.json"`, members `alpha`/`beta` → `docs/alpha.md`, `docs/beta.md`. Expected: the format's extension, or a refusal when the glob gives no one path. Observed at 040e2d1c (unchanged since v0.0.21; consumer report on 0.0.21, reproduced from npm).
- Emit splices a member name containing `/` through a flat glob's `*` and writes outside the glob: `*.json` with name `a/b/sub/x` → `docs/a/b/sub/x.json`, which `*` cannot match, so `check` counts `(0)` and `explain 'note:a/b/sub/x'` parses the slash as nested-address syntax. `splice_name` refuses a glob that can't map a name through one `*` (`FlatGlobDepth`) but never checks the name; a name that cannot round-trip through its glob should refuse the same way. Observed at 040e2d1c.
- Nested-file children are keyed corpus-wide by `kind:name`, not host-scoped as decision 0049 requires (`<host-address>/<kind>/<key>`; 0049 rejects corpus-unique keys by name). Reproduce: a directory-unit host kind templating `{kind: note, path: "*.json"}`, hosts `alpha`/`beta`, one child `home` under each → emit fails `duplicate identity key 'note:home'` (`sdk/src/emit.ts` `memberTable`, `declarations.ts` `uniqueMap`). With distinct names it emits and `check` counts them, but `explain 'unit:alpha/note/home'` finds nothing while `explain 'note:home'` resolves — the address grammar is not honored for this locus. Observed at 040e2d1c.
- One splice rule, and placement round-trips through discovery (subsumes the two notes above as its reproductions). Every path emit derives for a member must be one its own kind's glob discovers — a projection its glob cannot find is written yet ungoverned, counted `(0)` with no finding (invariant 6; 0027 "position stays decidable"); refuse loud when the derived path fails the kind's glob. The derivation is `splice_name`, already "the one name-through-a-glob map" for flat globs and host templates: exactly one `*`, confined to the final path segment (0038 — the star names the member); literal leading segments are fixed placement; a leading `**/` collapses to zero segments (`**/CLAUDE.md` → `CLAUDE.md` as today, `**/*.json` → `<name>.json`, `**/sub/*.json` → `sub/<name>.json`). This retires `member_projection_path`'s `glob.contains("**")` → `.md` arm and the one-segment refusal in templates, where `representation.md` ("kind", the template bullet) declares "the path pattern (relative to the parent's unit)" with no segment bound and a template has no `root` to move a directory into — today `path: "notes/*.json"` refuses `FlatGlobDepth` while that very error's remedy is "use a nesting kind". A name containing `/` fails the round trip and should refuse naming why: identity is a file stem or one starred segment (0038) and `/` is the address separator (0049); depth is nesting. Observed at 022477c4.
- A nested-file member cannot host a further nested layer: `nested_file_path` (`src/drift.rs`) demands the host kind's `governs_root` and a directory unit, and a nested-file kind governs no root, so nesting stops at one level — against `representation.md` ("nesting": "A kind may template inner layers of members, to arbitrary depth") and 0049 (the address composes through the host the way a nested file composes its path). A unit two directories deep is two layers (`area` → `page` → children), address `area:<a>/page/<b>/<kind>/<key>`, path `<root>/<a>/<b>/<dir>/<key>.<ext>`; a multi-segment host name is not the answer (0038, 0049). Observed at 022477c4.
