/**
 * Genre values — prose that declares its own anatomy
 * (`specs/architecture/20-surface.md`; ratified `specs/intent/00-intent.md`,
 * the genre Decision). A genre value's meaning-carrying fields are prose
 * leaves: authored strings, law-5 protected one by one. The constructors
 * here are the module-carried spelling; the document-carried spelling is the
 * TOML genre fence — one manifest shape either way, every consumer
 * carriage-blind.
 *
 * These constructors carry the **shape only**. Any predicate over a genre
 * value (a decision names at least one rejected alternative) belongs to the
 * bound package, never here — the same ownership line as everywhere
 * (`specs/architecture/15-kinds.md`, "genres (optional)").
 */

import type { ManifestGenreValue } from "./manifest.js";

/** A rejected alternative: keyed by option slug, its rationale a prose leaf. */
export interface Alternative {
  readonly because: string;
}

/**
 * The `decision` genre — the Chosen/Rejected convention, typed. Sibling
 * collections are keyed by option slug, never positional: positional
 * addresses die on insertion and reorder, which is exactly when impact must
 * survive (the leaf-address Decision).
 */
export function decision(init: {
  key: string;
  chosen: string;
  rejected?: Readonly<Record<string, Alternative>>;
}): ManifestGenreValue {
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
}): ManifestGenreValue {
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
}): ManifestGenreValue {
  return {
    genre: "bound",
    key: init.key,
    leaves: { claim: init.claim, deferred: init.deferred, unlock: init.unlock },
    collections: {},
  };
}

/** A project's own genre — the same machinery, an author-declared shape. */
export function genre(init: {
  genre: string;
  key: string;
  leaves: Readonly<Record<string, string>>;
  collections?: ManifestGenreValue["collections"];
}): ManifestGenreValue {
  return {
    genre: init.genre,
    key: init.key,
    leaves: init.leaves,
    collections: init.collections ?? {},
  };
}
