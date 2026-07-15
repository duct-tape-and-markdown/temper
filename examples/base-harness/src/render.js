/**
 * The renderer: turn scanned items into the one-line summary. It works
 * from scan results alone and never re-reads the document
 * (docs/systems/renderer.md).
 */

/** Render scanned items as the summary line: `2/5 done`. */
export function render(items) {
  const done = items.filter((item) => item.done).length;
  return `${done}/${items.length} done`;
}
