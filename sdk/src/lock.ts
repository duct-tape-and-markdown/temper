/**
 * Lock stamping — the generated state-of-record the drift engine reads
 * (`specs/architecture/20-surface.md`, "The lock"; "Drift — one direction, two
 * freshness facts"). For every projected member the lock carries a `[[<kind>]]`
 * row of four columns — `name`, `source_path`, `source_hash`, `emit_hash` —
 * matching the Rust lock (`src/import.rs` `RollupEntry` / `rollup_tables`).
 *
 * The two fingerprints are SHA-256 hex, computed the same way as the Rust
 * `hash::sha256_hex` (lowercase hex over the raw UTF-8 bytes), so an SDK-emitted
 * lock and a Rust-emitted lock agree for the same face. A module-carried member's
 * `.claude/**` projection is its own state-of-record — the surface was just
 * derived from it — so a fresh emit stamps `source_hash == emit_hash ==
 * sha256(projection bytes)`, exactly the baseline a Rust `import` records before
 * any later `emit` advances the emit fingerprint.
 */

import { createHash } from "node:crypto";

import type { Projection } from "./project.js";
import { joinSections, keyValue, encodeString } from "./toml.js";

/** Lowercase hex SHA-256 of `text`'s UTF-8 bytes — the Rust `sha256_hex` port. */
export function sha256Hex(text: string): string {
  return createHash("sha256").update(text, "utf8").digest("hex");
}

/** One lock row: a member's identity and the two freshness fingerprints. */
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

/** The bare kind name (`claude-code.skill` → `skill`) the lock array keys on. */
function bareKind(kind: string): string {
  const parts = kind.split(".");
  return parts[parts.length - 1];
}

/**
 * The lock row a projection stamps: both fingerprints are `sha256(projection
 * bytes)`, the fresh-emit baseline (`source_hash == emit_hash`) a Rust import then
 * emit lands on for a byte-identical projection.
 */
export function lockRow(kind: string, projection: Projection): LockRow {
  const hash = sha256Hex(projection.bytes);
  return {
    kind: bareKind(kind),
    name: projectionName(projection),
    sourcePath: projection.path,
    sourceHash: hash,
    emitHash: hash,
  };
}

/** The member name a projection encodes — the path's identity segment. */
function projectionName(projection: Projection): string {
  // `.claude/rules/<name>.md` → `<name>`; `.claude/skills/<name>/SKILL.md` → `<name>`.
  const segments = projection.path.split("/");
  const last = segments[segments.length - 1];
  if (last === "SKILL.md") return segments[segments.length - 2];
  return last.replace(/\.md$/, "");
}

/**
 * Serialize the lock rows to `lock.toml` bytes — one `[[<kind>]]` array-of-tables
 * per kind, kinds name-sorted then rows name-sorted, byte-identical to the Rust
 * `write_rollup`'s `toml_edit` output. Each row emits the four columns in the
 * fixed `name`/`source_path`/`source_hash`/`emit_hash` order, and the whole
 * document carries exactly one blank line before every table header but the first.
 */
export function stampLock(rows: readonly LockRow[]): string {
  const byKind = new Map<string, LockRow[]>();
  for (const row of rows) {
    const bucket = byKind.get(row.kind);
    if (bucket) bucket.push(row);
    else byKind.set(row.kind, [row]);
  }

  const sections: string[] = [];
  for (const kind of [...byKind.keys()].sort()) {
    const kindRows = byKind.get(kind)!.slice().sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
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
  return joinSections(sections);
}
