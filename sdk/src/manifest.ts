/**
 * The manifest schema — the one shape every carriage serializes into and the
 * only thing the gate reads (`specs/architecture/20-surface.md`, "Member
 * carriage"; "The IR"). These types mirror the Rust side's `[[member]]` /
 * `[[member.section]]` / `[[member.genre]]` tables byte-for-shape: the SDK is
 * versioned against this schema (`specs/architecture/50-distribution.md`,
 * acquisition Decision), and `emit`'s output must reparse on the Rust side
 * with no loss.
 */

/** One extracted (or module-declared) section: a heading and its authored body. */
export interface ManifestSection {
  readonly heading: string;
  readonly body: string;
}

/**
 * A genre value serialized whole (`specs/architecture/20-surface.md`, "Genre
 * values — prose that declares its own anatomy"): leaves are top-level
 * authored strings keyed by field name; sibling collections are keyed at
 * every level (`rejected.baked-projection.because`), never positional — leaf
 * addresses are structural and keyed (same file, the leaf-address Decision).
 */
export interface ManifestGenreValue {
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

/**
 * One member's serialized features — a `[[member]]` table. Every carriage
 * (module, document, in-place) lands here identically; every consumer is
 * carriage-blind (`specs/architecture/20-surface.md`, the carriage Decision).
 */
export interface ManifestMember {
  readonly kind: string;
  readonly name: string;
  readonly line_count: number;
  readonly source_dir?: string;
  readonly headings: readonly string[];
  readonly satisfies: readonly string[];
  readonly sections: readonly ManifestSection[];
  readonly genres: readonly ManifestGenreValue[];
}
