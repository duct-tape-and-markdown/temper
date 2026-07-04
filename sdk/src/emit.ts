/**
 * Emit — the compile from the authoring face to the inert manifest
 * (`specs/architecture/20-surface.md`, "Emit is byte-reproducible, and
 * checked"; law 5). This scaffold serializes members into the manifest's
 * `[[member]]` / `[[member.section]]` / `[[member.genre]]` schema and
 * enforces the double-emit discipline in-process: emit twice, compare bytes,
 * fail loud on any divergence.
 *
 * SCAFFOLD BOUNDS — stated, not hidden:
 * - Byte-parity with the Rust emitter's `toml_edit` output is the altitude
 *   slice's acceptance bar, not this scaffold's; this serializer emits valid
 *   TOML of the same schema.
 * - Projection writing (members → `.claude/**`), lock stamping, `fromFile`
 *   resolution, and mention resolution-checking are deliberately absent —
 *   each is a named altitude entry, never silently faked here.
 */

import type { Harness } from "./assembly.js";
import type { Member } from "./members.js";
import type { ManifestGenreValue, ManifestMember } from "./manifest.js";

/** TOML basic-string escape. */
function str(value: string): string {
  return `"${value.replace(/\\/g, "\\\\").replace(/"/g, '\\"').replace(/\n/g, "\\n")}"`;
}

/** TOML multi-line literal for authored bodies — the Rust emitter's spelling. */
function multiline(value: string): string {
  return `"""\n${value.replace(/\\/g, "\\\\").replace(/"""/g, '\\"\\"\\"')}"""`;
}

function stringArray(values: readonly string[]): string {
  return `[${values.map(str).join(", ")}]`;
}

/** Render one member's authored body to its manifest text (scaffold: inline only). */
function bodyText(member: Member): string {
  if (member.body.kind === "fromFile") {
    throw new Error(
      `member \`${member.name}\`: fromFile resolution is not in the scaffold — ` +
        `inline the body or wait for the asset-resolution slice.`,
    );
  }
  if (member.body.mentions.length > 0) {
    throw new Error(
      `member \`${member.name}\`: mention resolution is not in the scaffold — ` +
        `mentions require the resolution-checked emit slice.`,
    );
  }
  return member.body.template;
}

/** Serialize one member to its manifest tables. */
function memberTables(member: Member): string {
  const body = bodyText(member);
  const lines: string[] = [];
  lines.push("[[member]]");
  lines.push(`kind = ${str(member.kind)}`);
  lines.push(`name = ${str(member.name)}`);
  lines.push(`line_count = ${body.split("\n").length}`);
  const headings = [...body.matchAll(/^#{1,6} +(.+)$/gm)].map((m) => m[1]);
  lines.push(`headings = ${stringArray(headings)}`);
  lines.push(`satisfies = ${stringArray(Object.keys(member.satisfies).sort())}`);
  lines.push("");
  lines.push("[[member.section]]");
  lines.push(`heading = ${str(headings[0] ?? member.name)}`);
  lines.push(`body = ${multiline(body)}`);
  lines.push("");
  for (const genre of member.genres) {
    lines.push(...genreTables(genre));
  }
  return lines.join("\n");
}

/** Serialize one genre value whole — leaves as a string table, collections keyed. */
function genreTables(value: ManifestGenreValue): string[] {
  const lines: string[] = [];
  lines.push("[[member.genre]]");
  lines.push(`genre = ${str(value.genre)}`);
  lines.push(`key = ${str(value.key)}`);
  lines.push("");
  lines.push("[member.genre.leaves]");
  for (const field of Object.keys(value.leaves).sort()) {
    lines.push(`${field} = ${str(value.leaves[field])}`);
  }
  lines.push("");
  for (const collection of Object.keys(value.collections).sort()) {
    const entries = value.collections[collection];
    for (const key of Object.keys(entries).sort()) {
      lines.push(`[member.genre.collections.${collection}.${key}]`);
      const fields = entries[key];
      for (const field of Object.keys(fields).sort()) {
        lines.push(`${field} = ${str(fields[field])}`);
      }
      lines.push("");
    }
  }
  return lines;
}

/** One emit pass: harness → manifest members text, deterministically ordered. */
function emitOnce(harness: Harness): string {
  const members = [...harness.members].sort(
    (a, b) => a.kind.localeCompare(b.kind) || a.name.localeCompare(b.name),
  );
  return members.map(memberTables).join("\n") + "\n";
}

/**
 * Compile the harness's members to manifest TOML — double-emit verified:
 * nondeterminism in authoring code is a loud failure, never silent churn
 * (law 5, the emit bullet).
 */
export function emitManifestMembers(harness: Harness): string {
  const first = emitOnce(harness);
  const second = emitOnce(harness);
  if (first !== second) {
    throw new Error(
      "double-emit divergence: two passes over the same harness produced different bytes — " +
        "authoring code is nondeterministic (a timestamp? an unordered map?).",
    );
  }
  return first;
}

/** The parsed-shape view of one member, for tests and tooling. */
export function toManifestMember(member: Member): ManifestMember {
  const body = bodyText(member);
  const headings = [...body.matchAll(/^#{1,6} +(.+)$/gm)].map((m) => m[1]);
  return {
    kind: member.kind,
    name: member.name,
    line_count: body.split("\n").length,
    headings,
    satisfies: Object.keys(member.satisfies).sort(),
    sections: [{ heading: headings[0] ?? member.name, body }],
    genres: member.genres,
  };
}
