<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `emit` and `check` disagree on a `json-document` kind with a `directory` unit shape. A kind at `{ kind: "at", root: "docs", glob: "*/area.json" }`, `format: "json-document"`, `unitShape: "directory"` (and a nested-file child of the same shape templated at `*/page.json`) emits every member to its derived path; `check` then refuses to load the harness: "kind `area` declares `json-document` with an identity shape it cannot serve" (`src/json_manifest.rs` `NoDeclaredIdentity`). Emit admitting what check refuses breaks invariant 6 whichever side is right. The corpus says which: decision 0038 makes starred-segment identity a declared fact available to every kind — "generalized to the constructor every author uses: ownership, not privilege" — and no spec section narrows identity by format; the restriction lives only in `NoDeclaredIdentity`'s doc comment ("never a directory or a glob segment"). So the JSON document face admits the directory/starred-segment identity a markdown kind already gets, and emit and check agree by construction. Observed at fca7b4f9.
- `root.fresh` names a drifted projection's member by its bare name — "committed projection `docs/beta/one/intents/home.json` (member `home`) does not match the lock's emit fingerprint" (`src/drift.rs:3236`) — where decision 0049 says every finding prints the member's address (`area:beta/page/one/intent/home`; at top level, `area:alpha`). With nesting at any depth, several members share a bare name; the path disambiguates today, but the grammar is the bar. Observed at fca7b4f9.
