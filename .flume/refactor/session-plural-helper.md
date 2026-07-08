## Surface

Byte-identical `fn plural(n: usize) -> &'static str` in two homes:
src/bundle.rs:324 and src/coverage_note.rs:221.

## Observed at

0ccba8d

## Suggested consolidation

One shared helper in the rendering-adjacent home; both call sites import it.
