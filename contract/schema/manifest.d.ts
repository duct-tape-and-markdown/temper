// Generated Rust-first from the manifest IR (ts-rs). Do not edit by hand —
// regenerate with `cargo test contract` under `BLESS=1` (tests/contract_fixtures.rs).

export type Kind = "String" | "Integer" | "Number" | "Boolean" | "Null" | "List" | "Map";
export type FeatureValue = { "Scalar": { 
/**
 * The parsed source kind of the scalar.
 */
kind: Kind, 
/**
 * The scalar as text (the YAML/JSON scalar stringified).
 */
text: string, } } | { "List": Array<string> } | "Map";
export type Section = { 
/**
 * The heading text, with its `#` markers stripped exactly as
 * [`body_headings`] strips them.
 */
heading: string, 
/**
 * The body span beneath the heading — the intervening lines rejoined with
 * `\n`, the text a `section_contains` marker check searches.
 */
body: string, };
export type FencedBlock = { 
/**
 * The opening fence's info string, trimmed — `sh`, `toml`, or empty for a bare
 * fence. The declared kind the genre consumer keys on.
 */
info: string, 
/**
 * The block's interior content — the lines between the fences, rejoined with
 * `\n`, byte-faithful to the body span exactly as a [`Section`]'s body is.
 */
content: string, };
export type GenreValue = { 
/**
 * The genre this value instantiates — the fence info string's `genre.<genre>`
 * (`decision`), one of the kind's declared genres.
 */
genre: string, 
/**
 * The fence key naming this instance among its siblings in the same member —
 * the info string's second token (`surface-authority`). Part of a leaf's
 * address, so it is keyed, never positional.
 */
key: string, 
/**
 * The value's top-level **prose leaves** — field name → authored string, in
 * stable (sorted) key order so serialization is deterministic.
 */
leaves: { [key in string]: string }, 
/**
 * The value's **sibling collections** — collection name → (entry key → the
 * entry's own prose leaves), so a collection leaf addresses as
 * `<collection>.<entry>.<field>` (`rejected.baked-projection.because`). Keyed at
 * every level, never positional — an address that survives insertion and reorder
 * (`specs/architecture/20-surface.md`, "leaf addresses are structural and keyed").
 */
collections: { [key in string]: { [key in string]: { [key in string]: string } } }, };
export type PublishedRequirement = { 
/**
 * The requirement's name — the `[requirement.<name>]` module key.
 */
name: string, 
/**
 * The authored *intent*, the why. Carried verbatim and never interpreted
 * (`00-intent.md` law 3).
 */
means: string | null, 
/**
 * The artifact kind that may fill the requirement. Absent ⇒ kind-blind.
 */
kind: string | null, 
/**
 * The package the filling artifact must conform to, named by name. Absent ⇒ no
 * package constraint.
 */
package: string | null, 
/**
 * Whether an unfilled requirement is gate-blocking. Absent ⇒ `false`.
 */
required: boolean, };
export type Features = { 
/**
 * The artifact id used in diagnostics (for a skill, its `name`).
 */
id: string, 
/**
 * Frontmatter fields by name — the typed fields *and* the `extra` keys, so
 * a clause resolves `name`/`description`/`version` or any unknown key
 * (e.g. for `forbidden_keys`) through one generic lookup.
 */
fields: { [key in string]: FeatureValue }, 
/**
 * The artifact body's line count (for `max_lines`).
 */
body_lines: number, 
/**
 * The ATX headings (`#`..`######`) in the body, in document order, with the
 * `#` run and any closing `#` run trimmed (for `require_sections`). A `#`
 * inside a fenced code block is not a heading.
 */
headings: Array<string>, 
/**
 * The body's ATX [`Section`]s (heading + the body span beneath it), in
 * document order — the feature a `section_contains` clause decides over
 * (`specs/architecture/10-contracts.md`, the `section_contains` structural primitive). A
 * superset of [`headings`](Features::headings): where `headings` carries only
 * each heading's text, a section pairs it with its body span so a marker check
 * has prose to search.
 */
sections: Array<Section>, 
/**
 * The name of the directory the unit was imported from, off provenance
 * (for `name-matches-dir`). `None` when the source path has no parent.
 */
source_dir: string | null, 
/**
 * The body's format-executed directive occurrences, in document order — the
 * `at-import` `@path` targets a `directives` primitive extracts
 * (`specs/architecture/15-kinds.md`, "Directives — format-executed body syntax").
 * A body-derived feature like [`headings`](Features::headings)/[`sections`](Features::sections):
 * the raw occurrence strings only, resolution/classing a later slice. Empty
 * when the kind composes no `directives` primitive.
 */
directives: Array<string>, 
/**
 * The body's fenced code blocks, in document order — each block's info string
 * paired with its interior content, the feature a `fenced` primitive yields
 * (`specs/architecture/15-kinds.md`, "a fenced block — whose first consumer is
 * the genre fence"). A body-derived feature like
 * [`headings`](Features::headings)/[`sections`](Features::sections)/[`directives`](Features::directives):
 * the same fence boundaries the heading extractor tracks, surfaced whole. Empty
 * when the kind composes no `fenced` primitive.
 */
fenced_blocks: Array<FencedBlock>, 
/**
 * The body's **genre values**, in document order — each a genre fence
 * (`genre.<genre> <key>`) whose interior TOML parsed into a typed
 * [`GenreValue`] (`specs/architecture/20-surface.md`, "Genre values"). The typed layer
 * over [`fenced_blocks`](Features::fenced_blocks): a raw block whose info string
 * names a genre the kind declares is folded here beside its raw form, keyed by the
 * fence's genre+key; every other fenced block stays raw-only. Empty when the kind
 * declares no genres, or no block opts into one — genre adoption is per-block, and
 * no check quantifies over its completeness.
 */
genres: Array<GenreValue>, 
/**
 * The requirements this artifact opts into filling — the authored
 * `[representation].satisfies` bindings, surfaced for the coverage check
 * (`specs/architecture/20-surface.md`, "Each artifact directory is a representation, not
 * a copy"). This is a *representation* edge the coverage resolver reads, NOT
 * a contract-checkable frontmatter field — so it lives here, distinct from
 * `fields`, and never resolves through [`Features::field`]. The authored
 * `rationale` is deliberately absent: it is the human *why*, never a
 * decidable feature.
 */
satisfies: Array<string>, 
/**
 * The requirements this artifact **publishes** — the authored
 * `[requirement.<name>]` header modules (`specs/architecture/10-contracts.md`, "Decision: a
 * requirement's publisher is any authored surface document"). The demand side of
 * the fill edge, carried beside `satisfies` (the fill side) so the gate gathers
 * every member's published obligations across every kind and unions them with the
 * assembly roster into the one requirement namespace. Like `satisfies`, this is a
 * *representation* fact carried through, never a contract-checkable frontmatter
 * field. Empty when the member publishes none.
 */
published_requirements: Array<PublishedRequirement>, };
export type ManifestMember = { 
/**
 * The bare kind name the member is checked under (`skill`, `rule`, a custom kind's
 * name) — the key `check` groups members by (`assemble_by_kind`), so the manifest
 * carries it explicitly rather than nesting members under a per-kind table.
 */
kind: string, 
/**
 * The member's deterministically-extracted [`Features`] — the exact value the gate
 * consumes, so a serialized member round-trips to the same features a live
 * extraction yields.
 */
features: Features, };
