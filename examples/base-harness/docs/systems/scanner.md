---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
implemented-by: ["scan"]
---
The scanner classifies the lines of a checklist document. A line is an item only when it carries a box: `- [ ]` open, `- [x]` done. The scan yields the items in document order and nothing else.

## Invariants

### Unrecognized lines are ignored, never guessed

A line without a box is not an item: the scanner drops it and does not repair near-misses. Loosening the pattern is a corpus amendment first, code second.

### A done box is exactly `[x]`

Lowercase `x`, nothing else. `[X]` and `[~]` are unrecognized lines under the invariant above.
