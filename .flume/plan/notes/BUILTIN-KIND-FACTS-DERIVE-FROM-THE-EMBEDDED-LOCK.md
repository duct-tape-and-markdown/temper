# Built-in kinds lift off the embedded lock; three facts stay row-less

`builtin_kind::definitions()`/`definition()` now lift each kind row through
`CustomKind::from_kind_fact_row`, relationships off the lock's `edge` assembly
facts. The seventeen hand-spelled constructors are gone.

Three Rust-only facts the row does not carry remain as named per-kind
exceptions in `src/builtin_kind.rs`:
- declared field keys (`declared_field_keys`: skill/command, agent, rule,
  plugin-manifest/marketplace/dial);
- memory's at-import directive (`declared_directives`);
- skill's bare-root `SKILL.md` (`declared_bare_root_file`).
The markdown-structure primitives are derived (kept only where the kind has
a markdown body: not `shape = "fields"`, not a JSON/TOML document).

Candidate follow-up: these three join the kind row (SDK `kindFactRow` +
`KindFactRow` columns + re-emit), which would also give custom kinds a
declared field order instead of the lift's fixed generic extraction. The
SDK already holds the field keys (`sdk/src/builtins.ts`), so today the key
lists are a second spelling of an SDK fact pinned only by the
`*_definition_matches_*` unit tests, not by the frozen-lock test.

Census: no other production site spells a built-in locus/glob/exclude/
content literal; the remaining hits in kind/import/coverage_note/graph/
admissibility are test fixtures. `the_embedded_lock_kind_facts_match_todays_
hand_written_kinds` was deleted (it compared the lock to itself once the
population derives from it).
