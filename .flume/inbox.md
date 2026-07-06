<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- (John's ruling, 2026-07-06 — UNSHIP THE PRESCRIBED GENRES.) The SDK exports
  `decision`, `law`, `bound` (+ the `Alternative` type) at the package root
  (sdk/src/index.ts:21-22, sdk/src/genres.ts). This is code drift from the
  ratified corpus: 15-kinds "Decision: a genre is a full kind, and genre
  checks are data, never engine" rejects a built-in ontology of argument as
  the tool's taste ("a corpus that argues differently declares its own genres
  with the same machinery"), and the corpus's genre target names
  decision/law/bound as TEMPER'S OWN dogfood conventions, declared on the
  spec kinds when the corpus SDK-migrates — never product exports. Verified:
  the three constructors have zero call sites in sdk/src + sdk/test outside
  genres.ts/index.ts. Delete them (clean slate: no deprecation); KEEP the
  mechanism — `genre<T>()`, `GenreValue`, `genreValue` — which is the tool
  the genre-adoption pilot consumes (cascade declares its own decision
  genre). Sequence after S1 (EMIT-PAYLOAD-SEAM is in sdk/ now); small,
  disjoint from S2+. Publish note: the deletion should ride the next
  @dtmd/temper publish so the registry never carries the taste long-term.
