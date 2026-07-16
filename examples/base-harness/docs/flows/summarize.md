<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

Summarize is the toy's one behavior: read a checklist, report how much of it is done (`node src/main.js TODO.md`).

Participants: scanner, renderer.

## Steps

### Scan the lines

Every line of the document is classified. Lines carrying a box become items; everything else drops out here and is never seen again downstream.

System: scanner.

### Render the summary

The surviving items tally into the summary line: done over total. The document itself is out of reach by design.

System: renderer.
