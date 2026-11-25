# Bundle embeds the harness reference

Shipped. `src/bundled_reference/` holds the SDK's `reference()` emitted over the
lock's memberless built-in program (`members: [reference()]`): `SKILL.md` plus 15
`kind-*.md` topics (`dial` and `settings-local` stay out: local loci). Embedded as
`bundle::BUNDLED_REFERENCE`, written under `skills/temper/`. The frozen test
re-derives the tree and compares the file set as well as bytes, so a kind added to
the std-lib fails it until the directory is re-copied. No domain topics: the
memberless program contracts no domain, so the bundled description names none.

Observed, for plan:

- The embedded bytes are emit's verbatim, so the bundled `SKILL.md` and every
  topic carry the managed-projection note ("edit the owning .temper/ module ...
  re-run temper emit"). In an installed plugin that note points at a program the
  adopter does not have. Byte equality is what the entry and 0093 ask for, so it
  ships as-is; if the plugin copy should drop the note, that needs a spec ruling
  on what "the same value" means for a bundle.
- Re-emitting the reference is manual (copy `.claude/skills/temper/` from the
  emit), the same as the built-in lock. Both would want one regenerate command.
