<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# Verify the summary

The implementation under `src/` is a checklist summarizer and `TODO.md`
is its input; the entrypoint under test is `src/main.js`.

1. Run `node src/main.js TODO.md`.
2. Against the committed `TODO.md` it prints `2/3 done` — a
   `done/total` count over the checklist items. If `TODO.md` changed,
   count its `[x]` items yourself and compare.
3. Finish with `temper check .`: the structural gate must be green
   before the change is done.

On a mismatch, the corpus is authoritative: read
[the scanner](../../../docs/systems/scanner.md) and
[the renderer](../../../docs/systems/renderer.md) and decide
whether the code or the document has the bug — never adjust the
expected output to match broken behavior.
