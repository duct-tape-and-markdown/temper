/**
 * The lock — tool-written provenance, emit fingerprints, and the program's
 * declaration rows (`specs/architecture/20-surface.md`, "The lock and drift").
 * Two row families: a per-member `[[<kind>]]` rollup (`name`, `source_path`,
 * `source_hash`, `emit_hash`) and the `[declaration]` table's four sub-families
 * (`[[declaration.kind]]`, `[[declaration.clause]]`, `[[declaration.requirement]]`,
 * `[[declaration.assembly]]`). Both are byte-identical to the Rust lock
 * (`src/import.rs` `write_rollup`, `src/drift.rs` `Declarations::write_into`) — the
 * byte-parity lockstep two writers keep until single-writer lands.
 *
 * Fingerprints are SHA-256 hex over raw UTF-8 (`hash::sha256_hex`), so an
 * SDK-emitted lock and a Rust-emitted lock agree for the same harness.
 */

import { createHash } from "node:crypto";

import type { Projection } from "./project.js";
import type { Declarations } from "./declarations.js";
import { joinSections, keyValue, encodeString } from "./toml.js";

/** Lowercase hex SHA-256 of `text`'s UTF-8 bytes — the Rust `sha256_hex` port. */
export function sha256Hex(text: string): string {
  return createHash("sha256").update(text, "utf8").digest("hex");
}

/** One rollup row: a member's identity and the two freshness fingerprints. */
export interface LockRow {
  /** The bare kind name — the `[[<kind>]]` array key (`rule`, `skill`, `memory`). */
  readonly kind: string;
  /** The member id — its `[[<kind>]]` `name` column. */
  readonly name: string;
  /** The projection's harness path — the source-of-record the fingerprints anchor. */
  readonly sourcePath: string;
  /** SHA-256 of the authored source bytes (the projection, for a module-carried member). */
  readonly sourceHash: string;
  /** SHA-256 of the last emitted projection — the `config.stale` baseline. */
  readonly emitHash: string;
}

/** The member name a projection encodes — the path's identity segment. */
function projectionName(projection: Projection): string {
  const segments = projection.path.split("/");
  const last = segments[segments.length - 1];
  if (last === "SKILL.md") return segments[segments.length - 2];
  return last.replace(/\.md$/, "");
}

/**
 * The rollup row a projection stamps: both fingerprints are `sha256(projection
 * bytes)`, the fresh-emit baseline (`source_hash == emit_hash`) a Rust import then
 * emit lands on for a byte-identical projection.
 */
export function lockRow(kind: string, projection: Projection): LockRow {
  const hash = sha256Hex(projection.bytes);
  return {
    kind,
    name: projectionName(projection),
    sourcePath: projection.path,
    sourceHash: hash,
    emitHash: hash,
  };
}

/** The rollup sections — one `[[<kind>]]` table per member, kinds then rows name-sorted. */
function rollupSections(rows: readonly LockRow[]): string[] {
  const byKind = new Map<string, LockRow[]>();
  for (const row of rows) {
    const bucket = byKind.get(row.kind);
    if (bucket) bucket.push(row);
    else byKind.set(row.kind, [row]);
  }
  const sections: string[] = [];
  for (const kind of [...byKind.keys()].sort()) {
    const kindRows = byKind
      .get(kind)!
      .slice()
      .sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
    for (const row of kindRows) {
      sections.push(
        `[[${kind}]]\n` +
          keyValue("name", encodeString(row.name)) +
          keyValue("source_path", encodeString(row.sourcePath)) +
          keyValue("source_hash", encodeString(row.sourceHash)) +
          keyValue("emit_hash", encodeString(row.emitHash)),
      );
    }
  }
  return sections;
}

/** A `key = "value"\n` line for a present string column, or "" to omit an absent one. */
function optionalColumn(key: string, value: string | undefined): string {
  return value === undefined ? "" : keyValue(key, encodeString(value));
}

/**
 * The `[declaration]` table's sections, in the fixed family order kind · clause ·
 * requirement · assembly, each row a `[[declaration.<family>]]` table with its
 * columns in the Rust `to_table` order. An empty family writes nothing — an empty
 * array vanishes on the toml round-trip.
 */
function declarationSections(declarations: Declarations): string[] {
  const sections: string[] = [];
  for (const row of declarations.kinds) {
    sections.push(
      "[[declaration.kind]]\n" +
        keyValue("name", encodeString(row.name)) +
        optionalColumn("provider", row.provider) +
        keyValue("governs_root", encodeString(row.governs_root)) +
        keyValue("governs_glob", encodeString(row.governs_glob)) +
        optionalColumn("format", row.format) +
        optionalColumn("unit_shape", row.unit_shape) +
        optionalColumn("activation", row.activation),
    );
  }
  for (const row of declarations.clauses) {
    sections.push(
      "[[declaration.clause]]\n" +
        keyValue("kind", encodeString(row.kind)) +
        keyValue("predicate", encodeString(row.predicate)) +
        optionalColumn("field", row.field) +
        keyValue("severity", encodeString(row.severity)),
    );
  }
  for (const row of declarations.requirements) {
    sections.push(
      "[[declaration.requirement]]\n" +
        keyValue("name", encodeString(row.name)) +
        optionalColumn("kind", row.kind) +
        optionalColumn("package", row.package) +
        keyValue("required", row.required ? "true" : "false") +
        optionalColumn("verified_by", row.verified_by),
    );
  }
  for (const row of declarations.assembly) {
    sections.push(
      "[[declaration.assembly]]\n" +
        keyValue("fact", encodeString(row.fact)) +
        optionalColumn("value", row.value) +
        optionalColumn("from", row.from) +
        optionalColumn("field", row.field) +
        optionalColumn("to", row.to),
    );
  }
  return sections;
}

/**
 * Serialize the lock — the rollup rows then the declaration families, joined the
 * `toml_edit` way (exactly one blank line before every table header but the
 * document's first). An all-empty declaration set contributes no sections, so a
 * memberless lock is empty and a rollup-only lock carries no `[declaration]` rows.
 */
export function stampLock(rows: readonly LockRow[], declarations?: Declarations): string {
  const sections = [...rollupSections(rows)];
  if (declarations !== undefined) sections.push(...declarationSections(declarations));
  return joinSections(sections);
}
