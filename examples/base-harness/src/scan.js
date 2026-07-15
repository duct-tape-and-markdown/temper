/**
 * The scanner: classify checklist lines. A line is an item only when it
 * carries a box (`- [ ]` or `- [x]`); anything else is ignored, never
 * guessed at (docs/systems/scanner.md).
 */

/** Classify one line: `{ item: false }` or `{ item: true, done }`. */
export function scanLine(line) {
  const match = line.match(/^\s*[-*] \[([ x])\] /);
  if (match === null) return { item: false };
  return { item: true, done: match[1] === "x" };
}

/** Scan a document into its checklist items; non-item lines drop out. */
export function scan(text) {
  return text
    .split("\n")
    .map(scanLine)
    .filter((entry) => entry.item);
}
