<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- The address-parity question in the NESTED-LAYERS-COMPOSE-TO-ANY-DEPTH capture (`.flume/refactor/build-nested-layers-need-the-address-grammar.md`) is **not open**: `specs/model/representation.md` ("member", clarified this commit) and decision 0049 define the address recursively — any member address may host `/<kind>/<key>` — and every name, key and leaf is one segment (a leaf's child path joins with `.`, `src/extract.rs` `addressed_leaves`), so a member address has an odd segment count and a leaf address an even one by construction; `area:a/page/b/leaf/k` is a member, and resolution stays total. Do not file it as an open question. Re-scope the depth entry with both address readers in its fence — `src/member_address.rs` (`segment`'s `splitn(4,'/')`, `parse_nested_address`, `address_of`), `sdk/src/member-address.ts` (`memberAddress` flattening a nested host, and its "one level is the whole grammar" doc comment), and `src/read.rs` (`species`) — with `src/gate.rs`, `src/builtin_kind.rs`, `src/admissibility.rs`, `src/graph.rs` as `address_of`'s blast radius to verify. Observed at b038eec9.
- `frontmatter::fold_file_id` folds a template's **literal** leading directories into a file-shaped child's id (`notes/*.md` template, `notes/home.md` → `notes-home`), which `drift::splice_name` then places at `notes/notes-home.md` — the round trip `refuse_ungoverned` guards fails at one nesting layer, independent of depth. Decision 0038: identity is a declared fact — file stem or starred segment, never inferred — and a literal template segment is locus, not identity; placement folding exists only for directories a wildcard (`**`) spans, where depth genuinely distinguishes same-named files (its own doc comment: nearest-wins memory nesting). Fold strips the template's literal segments exactly as splice re-inserts them, making the two inverses; uniqueness is unaffected since every member under one template shares the literal prefix. Its own entry, not the depth entry's. Observed at b038eec9.
