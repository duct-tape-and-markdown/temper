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
- Design, not a plan derivation — for the session: a consumer wants one kind whose members project beside their unit (`<root>/<unit path>/<dir>/<name>.json`) with addressable identity. Nesting is the model's answer (a slashed name must stay refused — `/` is address syntax), but at 0.0.21 a host template path cannot carry a fixed leading directory (`notes/*.json` refuses `FlatGlobDepth`), and a directory host unit is one segment deep. Both widenings keep "the name maps through exactly one `*`"; neither is ruled. Observed at 040e2d1c.
