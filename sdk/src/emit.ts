/**
 * Emit — the compile from the authoring face to the inert manifest
 * (`specs/architecture/20-surface.md`, "Content-faithful, deterministically
 * emitted (law 5)"). Members serialize into the manifest's `[[member]]` /
 * `[member.field]` / `[[member.section]]` / `[[member.genre]]` /
 * `[[member.published]]` schema, **byte-identical** to the Rust emitter's
 * `toml_edit` output: a member emitted here reparses on the Rust side with no
 * loss, and `temper emit` and this face agree to the byte.
 *
 * Byte-parity is not a coincidence — it is a port. The string/key encoder below
 * mirrors `toml_write` 0.1.2 (`TomlStringBuilder::as_default` and the
 * `write_toml_value` escaper) and the table layout mirrors `toml_edit` 0.22.27
 * (`DocumentMut`'s `visit_table`: one blank line before every table header but
 * the document's first, `DEFAULT_TABLE_DECOR = ("\n", "")`). The double-emit
 * discipline still runs in-process: emit twice, compare bytes, fail loud on any
 * divergence.
 *
 * SCAFFOLD BOUNDS — stated, not hidden:
 * - Projection writing (members → `.claude/**`) and lock stamping are
 *   deliberately absent — each is a named altitude entry, never silently faked
 *   here. `fromFile` asset resolution and mention resolution-checking are
 *   resolved at emit (below), the `20-surface.md` "Mentions" contract.
 */

import { readFileSync } from "node:fs";
import { resolve as resolvePath } from "node:path";

import type { Harness } from "./assembly.js";
import type { Member } from "./members.js";
import { renderInline } from "./prose.js";
import type {
  ManifestGenreValue,
  ManifestMember,
  ManifestPublishedRequirement,
} from "./manifest.js";

// ---------------------------------------------------------------------------
// String & key encoding — a faithful port of `toml_write` 0.1.2 (src/string.rs).
// The auto style-detection and the escaper are reproduced exactly so a value
// emitted here is byte-identical to `toml_edit`'s `value(String)` output.
// ---------------------------------------------------------------------------

type Encoding = "literal" | "basic" | "mlliteral" | "mlbasic";

interface StringMetrics {
  readonly maxSingle: number;
  readonly maxDouble: number;
  readonly escapeCodes: boolean;
  readonly escape: boolean;
  readonly newline: boolean;
}

/** `ValueMetrics::calculate` — the run-length and escape facts style choice reads. */
function stringMetrics(s: string): StringMetrics {
  let maxSingle = 0;
  let maxDouble = 0;
  let escapeCodes = false;
  let escape = false;
  let newline = false;
  let prevSingle = 0;
  let prevDouble = 0;
  for (const ch of s) {
    const cp = ch.codePointAt(0)!;
    if (cp === 0x27) {
      prevSingle += 1;
      maxSingle = Math.max(maxSingle, prevSingle);
    } else {
      prevSingle = 0;
    }
    if (cp === 0x22) {
      prevDouble += 1;
      maxDouble = Math.max(maxDouble, prevDouble);
    } else {
      prevDouble = 0;
    }
    // The arm order mirrors the Rust match: `\` then `\t` (allowed) then `\n`
    // then the general control range.
    if (cp === 0x5c) escape = true;
    else if (cp === 0x09) {
      /* horizontal tab is always allowed — neutral */
    } else if (cp === 0x0a) newline = true;
    else if (cp <= 0x1f || cp === 0x7f) escapeCodes = true;
  }
  return { maxSingle, maxDouble, escapeCodes, escape, newline };
}

/** `TomlStringBuilder::as_default` — the fall-through style preference. */
function chooseEncoding(m: StringMetrics): Encoding {
  // as_basic_pretty
  if (!(m.escapeCodes || m.escape || m.maxDouble > 0 || m.newline)) return "basic";
  // as_literal
  if (!(m.escapeCodes || m.maxSingle > 0 || m.newline)) return "literal";
  // as_ml_basic_pretty
  if (!(m.escapeCodes || m.escape || m.maxDouble > 2)) return "mlbasic";
  // as_ml_literal
  if (!(m.escapeCodes || m.maxSingle > 2)) return "mlliteral";
  // fallback: the escaped forms
  return m.newline ? "mlbasic" : "basic";
}

/** The basic/multiline-basic escaper from `write_toml_value` (the `escaped` branch). */
function escapeBasic(s: string, isMl: boolean): string {
  const maxSeqDouble = isMl ? 2 : 0;
  let out = "";
  let seqDouble = 0;
  for (const ch of s) {
    const cp = ch.codePointAt(0)!;
    if (cp === 0x22) {
      seqDouble += 1;
      if (seqDouble > maxSeqDouble) {
        out += '\\"';
        seqDouble = 0;
        continue;
      }
      out += '"';
      continue;
    }
    seqDouble = 0;
    switch (cp) {
      case 0x08:
        out += "\\b";
        break;
      case 0x09:
        out += "\\t";
        break;
      case 0x0a:
        // A literal newline survives inside a multiline string; a basic string
        // escapes it.
        out += isMl ? "\n" : "\\n";
        break;
      case 0x0c:
        out += "\\f";
        break;
      case 0x0d:
        out += "\\r";
        break;
      case 0x5c:
        out += "\\\\";
        break;
      default:
        if (cp <= 0x1f || cp === 0x7f) {
          out += "\\u" + cp.toString(16).toUpperCase().padStart(4, "0");
        } else {
          out += ch;
        }
    }
  }
  return out;
}

/** A TOML string *value* — the exact bytes `toml_edit`'s `value(String)` emits. */
function encodeString(s: string): string {
  const m = stringMetrics(s);
  const enc = chooseEncoding(m);
  const delimiter =
    enc === "literal" ? "'" : enc === "basic" ? '"' : enc === "mlliteral" ? "'''" : '"""';
  const isMl = enc === "mlliteral" || enc === "mlbasic";
  const escaped = enc === "basic" || enc === "mlbasic";
  let out = delimiter;
  if (m.newline && isMl) out += "\n";
  out += escaped ? escapeBasic(s, isMl) : s;
  out += delimiter;
  return out;
}

interface KeyMetrics {
  readonly unquoted: boolean;
  readonly single: boolean;
  readonly double: boolean;
  readonly escapeCodes: boolean;
  readonly escape: boolean;
}

/** `KeyMetrics::calculate` — whether a key may be bare, and its escape facts. */
function keyMetrics(s: string): KeyMetrics {
  let unquoted = s.length > 0;
  let single = false;
  let double = false;
  let escapeCodes = false;
  let escape = false;
  for (const ch of s) {
    const cp = ch.codePointAt(0)!;
    const wordByte =
      (cp >= 0x61 && cp <= 0x7a) ||
      (cp >= 0x41 && cp <= 0x5a) ||
      (cp >= 0x30 && cp <= 0x39) ||
      cp === 0x2d ||
      cp === 0x5f;
    if (!wordByte) unquoted = false;
    if (cp === 0x27) single = true;
    else if (cp === 0x22) double = true;
    else if (cp === 0x5c) escape = true;
    else if (cp === 0x09) {
      /* tab allowed */
    } else if (cp <= 0x1f || cp === 0x7f) escapeCodes = true;
  }
  return { unquoted, single, double, escapeCodes, escape };
}

/** A TOML *key* — bare where it can be, else `toml_edit`'s `TomlKeyBuilder::as_default`. */
function encodeKey(s: string): string {
  const m = keyMetrics(s);
  if (m.unquoted) return s;
  // as_basic_pretty
  if (!(m.escapeCodes || m.escape || m.double)) return '"' + escapeBasic(s, false) + '"';
  // as_literal
  if (!(m.escapeCodes || m.single)) return "'" + s + "'";
  // as_basic (fallback)
  return '"' + escapeBasic(s, false) + '"';
}

// ---------------------------------------------------------------------------
// Field values — the FeatureValue projection of `feature_to_value`
// (`src/compose.rs`): scalars in their kind, a list as a string array, a map
// as an empty inline table, a null omitted.
// ---------------------------------------------------------------------------

/** A `[member.field]` value, or `null` to omit the field (the Rust `null`→`None` drop). */
function encodeFieldValue(name: string, value: unknown): string | null {
  if (value === null || value === undefined) return null;
  if (typeof value === "string") return encodeString(value);
  if (typeof value === "boolean") return value ? "true" : "false";
  if (typeof value === "number") {
    if (Number.isInteger(value)) return String(value);
    throw new Error(
      `field \`${name}\`: non-integer number ${value} — float field emission is not in ` +
        `byte-parity scope; the Rust f64 spelling is unverified here.`,
    );
  }
  if (Array.isArray(value)) {
    // A list re-emits as a string array — `str_array` over the stringified
    // elements, exactly as the extractor stringifies a list field.
    return "[" + value.map((el) => encodeString(String(el))).join(", ") + "]";
  }
  // A map has no payload — an empty inline table, the `FeatureValue::Map` spelling.
  if (typeof value === "object") return "{}";
  throw new Error(
    `field \`${name}\`: value of type ${typeof value} has no manifest spelling — ` +
      `only strings, booleans, integers, string lists, and maps are byte-parity modelled.`,
  );
}

// ---------------------------------------------------------------------------
// Table layout — `toml_edit`'s `visit_table`. Each table is one "section"
// string (header line + its scalar `key = value` lines). The document joins
// every section across every member with exactly one blank line before each
// section but the first (`DEFAULT_TABLE_DECOR = ("\n", "")`, first table `("", …)`).
// ---------------------------------------------------------------------------

/** One `key = value\n` line, the key/value decor `toml_edit` renders (`key = value`). */
function keyValue(key: string, valueRepr: string): string {
  return `${encodeKey(key)} = ${valueRepr}\n`;
}

/** A TOML string array — `["a", "b"]`, no leading space, `, ` between elements. */
function stringArray(values: readonly string[]): string {
  return "[" + values.map(encodeString).join(", ") + "]";
}

/** Sorted keys — the stable order `toml_edit` gets for free from its `BTreeMap`s. */
function sortedKeys(record: Readonly<Record<string, unknown>>): string[] {
  return Object.keys(record).sort();
}

/** The `[[member.genre]]` value serialized whole — leaves flat, collections keyed. */
function genreSections(genre: ManifestGenreValue): string[] {
  const sections: string[] = [];
  sections.push(
    "[[member.genre]]\n" +
      keyValue("genre", encodeString(genre.genre)) +
      keyValue("key", encodeString(genre.key)),
  );
  const leafKeys = sortedKeys(genre.leaves);
  if (leafKeys.length > 0) {
    sections.push(
      "[member.genre.leaves]\n" +
        leafKeys.map((field) => keyValue(field, encodeString(genre.leaves[field]))).join(""),
    );
  }
  const collectionNames = sortedKeys(genre.collections);
  if (collectionNames.length > 0) {
    // The parent `[member.genre.collections]` table carries only sub-tables, but
    // `toml_edit` still emits its (childless) header — it is not implicit.
    sections.push("[member.genre.collections]\n");
    for (const name of collectionNames) {
      const path = `member.genre.collections.${encodeKey(name)}`;
      sections.push(`[${path}]\n`);
      const entries = genre.collections[name];
      for (const entryKey of sortedKeys(entries)) {
        const leaves = entries[entryKey];
        sections.push(
          `[${path}.${encodeKey(entryKey)}]\n` +
            sortedKeys(leaves)
              .map((field) => keyValue(field, encodeString(leaves[field])))
              .join(""),
        );
      }
    }
  }
  return sections;
}

/** A published requirement's `[[member.published]]` table — optional facets omitted. */
function publishedSection(requirement: ManifestPublishedRequirement): string {
  let block = "[[member.published]]\n";
  block += keyValue("name", encodeString(requirement.name));
  if (requirement.means !== undefined) block += keyValue("means", encodeString(requirement.means));
  if (requirement.kind !== undefined) block += keyValue("kind", encodeString(requirement.kind));
  if (requirement.package !== undefined) {
    block += keyValue("package", encodeString(requirement.package));
  }
  if (requirement.required) block += keyValue("required", "true");
  return block;
}

/** Every table section a member serializes into, in `member_to_table` key order. */
function memberSections(member: ManifestMember): string[] {
  const sections: string[] = [];

  let head = "[[member]]\n";
  head += keyValue("kind", encodeString(member.kind));
  head += keyValue("name", encodeString(member.name));
  head += keyValue("line_count", String(member.line_count));
  if (member.source_dir !== undefined) {
    head += keyValue("source_dir", encodeString(member.source_dir));
  }
  if (member.headings.length > 0) head += keyValue("headings", stringArray(member.headings));
  if (member.satisfies.length > 0) head += keyValue("satisfies", stringArray(member.satisfies));
  sections.push(head);

  // `[member.field]` — emitted whenever the frontmatter carries any key, even if
  // every value is a dropped null (the Rust `!fields.is_empty()` gate).
  const fieldKeys = sortedKeys(member.fields);
  if (fieldKeys.length > 0) {
    let body = "[member.field]\n";
    for (const key of fieldKeys) {
      const repr = encodeFieldValue(key, member.fields[key]);
      if (repr !== null) body += keyValue(key, repr);
    }
    sections.push(body);
  }

  for (const section of member.sections) {
    sections.push(
      "[[member.section]]\n" +
        keyValue("heading", encodeString(section.heading)) +
        keyValue("body", encodeString(section.body)),
    );
  }

  for (const genre of member.genres) sections.push(...genreSections(genre));

  for (const requirement of member.published) sections.push(publishedSection(requirement));

  return sections;
}

/** The manifest's `[[member]]` root — every section joined the `toml_edit` way. */
function emitDocument(members: readonly ManifestMember[]): string {
  const sections = members.flatMap(memberSections);
  return sections.map((section, i) => (i === 0 ? "" : "\n") + section).join("");
}

/**
 * Serialize one member's `[[member]]` tables — byte-identical to the Rust
 * emitter's output for the same member (`src/compose.rs` `write_member_table`).
 * The seam the byte-parity fixtures pin.
 */
export function serializeManifestMember(member: ManifestMember): string {
  return emitDocument([member]);
}

/**
 * How emit resolves the two body carriages a module-carried member can hold
 * (`specs/architecture/20-surface.md`, "Mentions"). Resolution happens **at
 * emit, not at authoring** — the address set is the whole harness's declared
 * values, unknown to any one member in isolation.
 */
export interface ResolveOptions {
  /**
   * The addresses a mention may name — every declared value in the harness
   * (member `kind:name`, requirement name, genre leaf). A mention outside it is
   * a loud emit error: a mention cannot dangle (`45-governance.md`). Absent, no
   * address resolves, so any mention fails loud — a standalone member cannot
   * carry a resolvable mention; that is emit's job.
   */
  readonly mentionable?: ReadonlySet<string>;
  /** Base dir a `fromFile` module-relative path resolves against (default: cwd). */
  readonly baseDir?: string;
}

/**
 * Resolve a member's authored body to its final manifest text: a `fromFile`
 * asset is read into the body; an inline body's mentions are resolution-checked
 * (loud on a dangling address) and rendered by the one display rule. The words
 * themselves are never reworded (law 5) — a mention only substitutes its target's
 * display form, and an asset is copied byte-for-byte.
 *
 * # Throws
 * If a `fromFile` asset does not exist, or a mention names an address no
 * declared value carries.
 */
function resolveBody(member: Member, options: ResolveOptions): string {
  if (member.body.kind === "fromFile") {
    const assetPath = resolvePath(options.baseDir ?? process.cwd(), member.body.path);
    try {
      return readFileSync(assetPath, "utf8");
    } catch (cause) {
      throw new Error(
        `member \`${member.name}\`: fromFile asset \`${member.body.path}\` did not resolve ` +
          `(looked at \`${assetPath}\`).`,
        { cause },
      );
    }
  }
  const mentionable = options.mentionable ?? new Set<string>();
  for (const mention of member.body.mentions) {
    if (!mentionable.has(mention.target.address)) {
      throw new Error(
        `member \`${member.name}\`: mention of \`${mention.target.address}\` resolves to no ` +
          `declared value — a mention cannot dangle (specs/architecture/45-governance.md).`,
      );
    }
  }
  return renderInline(member.body);
}

/** The parsed-shape view of one authored member, for tests and tooling. */
export function toManifestMember(member: Member, options: ResolveOptions = {}): ManifestMember {
  const body = resolveBody(member, options);
  const headings = [...body.matchAll(/^#{1,6} +(.+)$/gm)].map((m) => m[1]);
  const published: ManifestPublishedRequirement[] = Object.keys(member.requirements)
    .sort()
    .map((name) => {
      const requirement = member.requirements[name];
      return {
        name,
        means: requirement.means,
        kind: requirement.kind,
        ...(requirement.required ? { required: true } : {}),
      };
    });
  return {
    kind: member.kind,
    name: member.name,
    line_count: body.split("\n").length,
    headings,
    satisfies: Object.keys(member.satisfies).sort(),
    fields: member.fields,
    sections: [{ heading: headings[0] ?? member.name, body }],
    genres: member.genres,
    published,
  };
}

/**
 * Every address a mention may name — the harness's declared values, the one-way
 * edge's resolution set (`specs/architecture/45-governance.md`, "the mention is
 * the readmitted one-way annotation class"). A member is `kind:name`; a
 * requirement (assembly-declared or member-published) is its name; a genre leaf
 * is its member-qualified structural address (`member.genre-key.field`, sibling
 * collections keyed at every level — `20-surface.md`, the leaf-address Decision).
 * Section anchors are a later producer — no Mentionable helper mints one yet.
 */
function declaredAddresses(harness: Harness): Set<string> {
  const set = new Set<string>();
  for (const name of Object.keys(harness.requirements)) set.add(name);
  for (const member of harness.members) {
    set.add(`${member.kind}:${member.name}`);
    for (const name of Object.keys(member.requirements)) set.add(name);
    for (const genre of member.genres) {
      for (const field of Object.keys(genre.leaves)) {
        set.add(`${member.name}.${genre.key}.${field}`);
      }
      for (const [collection, entries] of Object.entries(genre.collections)) {
        for (const [entry, leaves] of Object.entries(entries)) {
          for (const field of Object.keys(leaves)) {
            set.add(`${member.name}.${genre.key}.${collection}.${entry}.${field}`);
          }
        }
      }
    }
  }
  return set;
}

/** The harness's members as manifest members, deterministically kind-then-name ordered. */
function orderedMembers(harness: Harness, options: ResolveOptions): ManifestMember[] {
  return [...harness.members]
    .sort(
      (a, b) =>
        (a.kind < b.kind ? -1 : a.kind > b.kind ? 1 : 0) ||
        (a.name < b.name ? -1 : a.name > b.name ? 1 : 0),
    )
    .map((member) => toManifestMember(member, options));
}

/** Emit-time inputs beyond the harness — where `fromFile` assets resolve from. */
export interface EmitOptions {
  /** Base dir a `fromFile` module-relative path resolves against (default: cwd). */
  readonly baseDir?: string;
}

/**
 * Compile the harness's members to manifest TOML — double-emit verified:
 * nondeterminism in authoring code is a loud failure, never silent churn
 * (law 5, the emit bullet). Bodies resolve here, not at authoring: `fromFile`
 * assets are read in and mentions are resolution-checked against the whole
 * harness's declared values.
 */
export function emitManifestMembers(harness: Harness, options: EmitOptions = {}): string {
  const resolve: ResolveOptions = {
    mentionable: declaredAddresses(harness),
    baseDir: options.baseDir,
  };
  const first = emitDocument(orderedMembers(harness, resolve));
  const second = emitDocument(orderedMembers(harness, resolve));
  if (first !== second) {
    throw new Error(
      "double-emit divergence: two passes over the same harness produced different bytes — " +
        "authoring code is nondeterministic (a timestamp? an unordered map?).",
    );
  }
  return first;
}
