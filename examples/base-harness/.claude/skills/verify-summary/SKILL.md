---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
name: "verify-summary"
description: "Verifies the checklist summarizer end to end: runs it against TODO.md and compares the printed count with the file's items. Use after changing anything under src/ or TODO.md."
paths: ["src/**","TODO.md"]
---
# Verify the summary

The implementation under `src/` is a checklist summarizer and `TODO.md`
is its input. To confirm a change actually works:

1. Run `node src/main.js TODO.md`.
2. Against the committed `TODO.md` it prints `2/3 done` — a
   `done/total` count over the checklist items. If `TODO.md` changed,
   count its `[x]` items yourself and compare.
3. On a mismatch, the corpus is authoritative: read
   `docs/systems/scanner.md` and `docs/systems/renderer.md` and decide
   whether the code or the document has the bug — never adjust the expected
   output to match broken behavior.
4. Finish with `temper check .`: the structural gate must be green
   before the change is done.


The entrypoint under test is src/main.js.
