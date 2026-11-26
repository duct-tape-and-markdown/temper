# Narration-drift folds landed; two adjacent residues seen

All seven named docs cut to what their code does (comment-only diff).

Adjacent, not taken (out of the entry's named set):
- `src/frontmatter.rs` header still speaks of `import` idempotence and an
  "import-from-source" face; `import` is not one of the seven shipped verbs.
  Same family (a claim the model retired) — worth a look next sweep.
- `src/dial.rs` `Dial::is_empty` is `#[allow(dead_code)]` with only test
  callers; the doc now says only what it does, but the method itself is
  dead weight a later tick could retire with its two tests.
