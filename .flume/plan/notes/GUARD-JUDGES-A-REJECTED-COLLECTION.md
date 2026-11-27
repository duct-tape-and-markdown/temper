# Guard judges a rejected collection

Shipped. `manifest_member_findings` (src/install.rs) now raises
`manifest.rejected-collection` from `Manifest::rejected_collections`, under the
manifest kind's mode, through `compose::rejected_collection_finding` (made
`pub(crate)`) — one constructor, so the guard's wording is check's.

Deduped per (manifest path, collection key): the settings `hooks` addresses
each parse the same file and would otherwise report `hooks` once per event.
Check's own compose side filters per kind address; if two hook-event kinds
there both see a rejected `hooks`, check may report it more than once per file
— not verified this tick, worth a look if the counts ever need to match.
