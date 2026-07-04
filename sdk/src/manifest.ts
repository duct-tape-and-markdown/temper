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
 * A requirement a member **publishes** — the demand side of a fill edge, a
 * `[[member.published]]` table. Mirrors the Rust `PublishedRequirement`
 * (`src/document.rs`): `name` always, the rest optional, `required` omitted
 * when false.
 */
export interface ManifestPublishedRequirement {
  readonly name: string;
  readonly means?: string;
  readonly kind?: string;
  readonly package?: string;
  readonly required?: boolean;
}

/**
 * One member's serialized features — a `[[member]]` table. Every carriage
 * (module, document, in-place) lands here identically; every consumer is
 * carriage-blind (`specs/architecture/20-surface.md`, the carriage Decision).
 * Key order mirrors the Rust `member_to_table` (`src/compose.rs`): `kind`,
 * `name`, `line_count`, `source_dir?`, `headings?`, `satisfies?`, the
 * `[member.field]` frontmatter table, `[[member.section]]`, `[[member.genre]]`,
 * `[[member.published]]` — the exact serialization the gate reparses.
 */
export interface ManifestMember {
  readonly kind: string;
  readonly name: string;
  readonly line_count: number;
  readonly source_dir?: string;
  readonly headings: readonly string[];
  readonly satisfies: readonly string[];
  /** Frontmatter fields (a rule's `paths`, a skill's `description`). */
  readonly fields: Readonly<Record<string, unknown>>;
  /**
   * The whole content-faithful body — the **projection source**. The Rust
   * importer carries the full body (`skill.body`/`rule.body`, `src/drift.rs`)
   * beside its extracted `sections`; the SDK mirrors that split because projection
   * (`projectMember`) needs the untouched body while `sections` below carry only
   * the per-heading extraction the manifest serializes. Not itself serialized into
   * the manifest — the body lives there in `[[member.section]]` form. Empty for a
   * bodiless member (a genre-only decision).
   */
  readonly body: string;
  /**
   * The body's ATX **sections**, one per heading with the heading line split out
   * of the span — the extraction the manifest serializes, converged on the Rust
   * importer's `body_sections` (`src/extract.rs`) so a member projected to disk and
   * re-imported re-extracts identical `[[member.section]]` tables.
   */
  readonly sections: readonly ManifestSection[];
  readonly genres: readonly ManifestGenreValue[];
  readonly published: readonly ManifestPublishedRequirement[];
}
