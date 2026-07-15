---
implemented-by: ["render"]
---
The renderer turns scan results into the one-line summary (`2/3 done`): items done over items total.

## Invariants

### The summary derives from scan results, never the document

The renderer takes the scanned items alone. If a count looks wrong, the scan is wrong; the renderer holds no second opinion about the document.

### One line out

The summary is a single line with no trailing detail. Anything richer is a new declared behavior, not a bigger string.
