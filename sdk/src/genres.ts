/**
 * Genre values — prose that declares its own anatomy. A genre value's meaning-carrying fields are prose leaves —
 * authored strings, law-5 protected one by one — plus keyed sibling collections.
 * This constructor carries the **shape only**: any predicate over a genre value
 * is a clause some module ships, never here (`15-kinds.md`, the genre Decision).
 * There is no prescribed genre ontology — a corpus that argues differently
 * declares its own genres with the same machinery (`15-kinds.md`, "a genre is
 * a full kind, and genre checks are data, never engine").
 *
 * `genreValue()` is the posture-3 spelling — a fully composed value passed to
 * `blocks()` (`20-surface.md`). The byte-identical posture-2 fence render awaits
 * `(genre-fence-format)`, deferred until its first consumer lands.
 */

/**
 * A genre value serialized whole: leaves are authored strings keyed by field
 * name; sibling collections are keyed at every level (`rejected."baked-projection"`),
 * never positional — leaf addresses are structural and keyed (`20-surface.md`,
 * the leaf-address Decision).
 */
export interface GenreValue {
  /** The genre name — a project's own, never a built-in prescribed ontology. */
  readonly genre: string;
  /** The value's key — the identity a leaf address carries (`surface-authority`). */
  readonly key: string;
  /** Prose leaves: authored strings, law-5 protected one by one. */
  readonly leaves: Readonly<Record<string, string>>;
  /** Keyed sibling collections: collection → entry key → field → authored string. */
  readonly collections: Readonly<
    Record<string, Readonly<Record<string, Readonly<Record<string, string>>>>>
  >;
}

/** A project's own genre — the same machinery, an author-declared shape. */
export function genreValue(init: {
  genre: string;
  key: string;
  leaves: Readonly<Record<string, string>>;
  collections?: GenreValue["collections"];
}): GenreValue {
  return {
    genre: init.genre,
    key: init.key,
    leaves: init.leaves,
    collections: init.collections ?? {},
  };
}
