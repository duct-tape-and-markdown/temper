# Emit's placement carry now follows the current shape

Shipped. `emit_one` (src/drift.rs) filters the prior file's placement lines
to the form the current render holds: `#` comments (note, modeline) only
when the projection renders frontmatter, the banner only when it does not.
The filter is symmetric, so a kind that *gains* frontmatter also stops
carrying the banner into a YAML block, which was the same bug the other way
around. The schema modeline had the same cause: it is dropped when the
frontmatter goes, because nothing frontmatterless can hold it. Install
re-places it if the shape returns, so this tick made no install change.
