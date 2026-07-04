/**
 * Genre values — prose that declares its own anatomy (`specs/architecture/15-kinds.md`,
 * "A genre is a kind at the block locus"; ratified `specs/intent/00-intent.md`, the
 * genre Decision). A genre value's meaning-carrying fields are prose leaves —
 * authored strings, law-5 protected one by one — plus keyed sibling collections.
 * These constructors carry the **shape only**: any predicate over a genre value
 * (a decision names at least one rejected alternative) is a clause some module
 * ships, never here (`15-kinds.md`, the genre Decision).
 *
 * They are the posture-3 spelling — fully composed values passed to `blocks()`
 * (`20-surface.md`). The byte-identical posture-2 fence render awaits
 * `(genre-fence-format)`, deferred until its first consumer lands.
 */

/**
 * A genre value serialized whole: leaves are authored strings keyed by field
 * name; sibling collections are keyed at every level (`rejected."baked-projection"`),
 * never positional — leaf addresses are structural and keyed (`20-surface.md`,
 * the leaf-address Decision).
 */
export interface GenreValue {
  /** The genre name — `decision`, `law`, `bound`, or a project's own. */
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

/** A rejected alternative: keyed by option slug, its rationale a prose leaf. */
export interface Alternative {
  readonly because: string;
}

/**
 * The `decision` genre — the Chosen/Rejected convention, typed. Sibling
 * collections are keyed by option slug, never positional: positional addresses
 * die on insertion and reorder, which is exactly when impact must survive.
 */
export function decision(init: {
  key: string;
  chosen: string;
  rejected?: Readonly<Record<string, Alternative>>;
}): GenreValue {
  return {
    genre: "decision",
    key: init.key,
    leaves: { chosen: init.chosen },
    collections: {
      rejected: Object.fromEntries(
        Object.entries(init.rejected ?? {}).map(([slug, alt]) => [slug, { because: alt.because }]),
      ),
    },
  };
}

/** The `law` genre — a numbered law's statement with its named bounds. */
export function law(init: {
  key: string;
  statement: string;
  bounds?: Readonly<Record<string, { claim: string }>>;
}): GenreValue {
  return {
    genre: "law",
    key: init.key,
    leaves: { statement: init.statement },
    collections: {
      bounds: Object.fromEntries(
        Object.entries(init.bounds ?? {}).map(([slug, bound]) => [slug, { claim: bound.claim }]),
      ),
    },
  };
}

/** The `bound` genre — the honest bound: claim, deferral, unlock condition. */
export function bound(init: {
  key: string;
  claim: string;
  deferred: string;
  unlock: string;
}): GenreValue {
  return {
    genre: "bound",
    key: init.key,
    leaves: { claim: init.claim, deferred: init.deferred, unlock: init.unlock },
    collections: {},
  };
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
