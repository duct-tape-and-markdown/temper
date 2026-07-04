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

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve as resolvePath } from "node:path";

import type { Harness } from "./assembly.js";
import type { Member } from "./members.js";
import { renderInline } from "./prose.js";
import type {
  ManifestGenreValue,
  ManifestMember,
  ManifestPublishedRequirement,
} from "./manifest.js";
import { encodeKey, encodeString, joinSections, keyValue, sortedKeys, stringArray } from "./toml.js";
import type { Projection } from "./project.js";
import { isProjectedKind, projectMember } from "./project.js";
import type { LockRow } from "./lock.js";
import { lockRow, stampLock } from "./lock.js";
import { BINDINGS_PATH, ROSTER_PATH, assemblyArtifacts } from "./assembly_artifacts.js";

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
  return joinSections(members.flatMap(memberSections));
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
 * Every requirement name a `satisfies` claim may fill — the roster the
 * assembly declares plus every demand a member publishes
 * (`specs/architecture/20-surface.md`: "`satisfies` and published
 * `requirement`s remain the graph's whole source"). A `satisfies` naming none
 * of these is a dangling join; a `required` name absent from every member's
 * `satisfies` is an unfilled required requirement. Both refusals read this set.
 */
function declaredRequirements(harness: Harness): Set<string> {
  const set = new Set<string>();
  for (const name of Object.keys(harness.requirements)) set.add(name);
  for (const member of harness.members) {
    for (const name of Object.keys(member.requirements)) set.add(name);
  }
  return set;
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
  const set = declaredRequirements(harness);
  for (const member of harness.members) {
    set.add(`${member.kind}:${member.name}`);
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

/**
 * The two **declare-side** refusals emit runs before it compiles a single byte
 * (`specs/architecture/20-surface.md`, "Emit refuses before it writes"): a
 * broken source yields no output, never silent bytes.
 *
 * 1. **Dangling join** — every member `satisfies` claim must name a declared
 *    requirement (harness-level or member-published). A `satisfies` resolving
 *    to nothing is a join with no far end.
 * 2. **Unfilled required requirement** — a requirement marked `required`, whether
 *    the assembly declares it or a member publishes it, must be filled by some
 *    member's `satisfies`. A required demand nobody meets is a loud error.
 *
 * Mentions already refuse in {@link resolveBody}; the genre-value-violating-its-
 * bound-package refusal the spec also lists stays out of scope — the SDK carries
 * no package model yet.
 *
 * # Throws
 * On a dangling `satisfies` join or an unfilled `required` requirement.
 */
function refuseBrokenSource(harness: Harness): void {
  const requirements = declaredRequirements(harness);
  const filled = new Set<string>();
  for (const member of harness.members) {
    for (const name of Object.keys(member.satisfies)) {
      if (!requirements.has(name)) {
        throw new Error(
          `member \`${member.name}\`: \`satisfies\` claims requirement \`${name}\`, which no ` +
            `harness-level or member-published requirement declares — a dangling join ` +
            `(specs/architecture/20-surface.md, "Emit refuses before it writes").`,
        );
      }
      filled.add(name);
    }
  }

  // A `required` demand — from the assembly roster or a member's own publication —
  // that no member fills. `member.name` labels the source so the diagnostic points
  // at the unmet publisher, not just the bare requirement name.
  const requiredSources: [string, string][] = [];
  for (const [name, requirement] of Object.entries(harness.requirements)) {
    if (requirement.required) requiredSources.push([name, "the assembly"]);
  }
  for (const member of harness.members) {
    for (const [name, requirement] of Object.entries(member.requirements)) {
      if (requirement.required) requiredSources.push([name, `member \`${member.name}\``]);
    }
  }
  for (const [name, source] of requiredSources) {
    if (!filled.has(name)) {
      throw new Error(
        `required requirement \`${name}\` (declared by ${source}) is filled by no member's ` +
          `\`satisfies\` — an unfilled required requirement ` +
          `(specs/architecture/20-surface.md, "Emit refuses before it writes").`,
      );
    }
  }
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
  refuseBrokenSource(harness);
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

/**
 * A full emit's compiled artifacts — the three provenance classes emit produces
 * (`specs/architecture/20-surface.md`, "Topology"): the generated-canonical
 * manifest, the generated `.claude/**` projection, and the generated lock. The
 * whole set is a pure function of the harness, so [`emit`] double-verifies it and
 * [`writeEmit`] lands it on disk.
 */
export interface EmitResult {
  /** The manifest's `[[member]]` section — every member, kind-then-name ordered. */
  readonly manifest: string;
  /** The projection files, one per projected (rule/skill/memory) member. */
  readonly projections: readonly Projection[];
  /** The `lock.toml` bytes — a freshness row per projected member. */
  readonly lock: string;
  /**
   * The two locus-less assembly-fact artifacts — the kind bindings and the
   * requirement roster (`specs/architecture/20-surface.md`, "the bindings, the
   * roster — are emitted as small committed temper-owned artifacts"). Additive:
   * the manifest/projection/lock above are unchanged, so their byte-parity holds.
   */
  readonly bindings: string;
  readonly roster: string;
}

/**
 * Compile the whole face in one deterministic pass: the manifest, the `.claude/**`
 * projection, and the lock whose fingerprints the drift engine reads. Bodies
 * resolve once (`fromFile` assets read in, mentions resolution-checked against the
 * harness's declared values) and feed all three outputs, so a projection and its
 * lock fingerprint agree by construction. The whole result is double-emit verified
 * — nondeterministic authoring is a loud failure, never a silent churn (law 5).
 *
 * The built-in projected kinds (`rule`, `skill`, `memory`) each carry a
 * projection and a lock row — a rule/skill under `.claude/**`, a memory a
 * frontmatterless root `CLAUDE.md`/`AGENTS.md`; a custom member lands in the
 * manifest but projects nowhere ([`isProjectedKind`]).
 */
export function emit(harness: Harness, options: EmitOptions = {}): EmitResult {
  refuseBrokenSource(harness);
  const resolve: ResolveOptions = {
    mentionable: declaredAddresses(harness),
    baseDir: options.baseDir,
  };
  const compile = (): EmitResult => {
    const members = orderedMembers(harness, resolve);
    const projected = members.filter((member) => isProjectedKind(member.kind));
    const projections = projected.map(projectMember);
    const rows: LockRow[] = projected.map((member, i) => lockRow(member.kind, projections[i]));
    const { bindings, roster } = assemblyArtifacts(harness);
    return {
      manifest: emitDocument(members),
      projections,
      lock: stampLock(rows),
      bindings,
      roster,
    };
  };
  const first = compile();
  const second = compile();
  if (
    first.manifest !== second.manifest ||
    first.lock !== second.lock ||
    first.bindings !== second.bindings ||
    first.roster !== second.roster ||
    !sameProjections(first.projections, second.projections)
  ) {
    throw new Error(
      "double-emit divergence: two passes over the same harness produced different bytes — " +
        "authoring code is nondeterministic (a timestamp? an unordered map?).",
    );
  }
  return first;
}

/** Whether two projection lists are byte-identical, path and bytes both. */
function sameProjections(a: readonly Projection[], b: readonly Projection[]): boolean {
  return (
    a.length === b.length &&
    a.every((p, i) => p.path === b[i].path && p.bytes === b[i].bytes)
  );
}

/**
 * Run a full [`emit`] and write its artifacts under `targetDir`: the manifest to
 * `temper.toml`, the lock to `lock.toml`, the two assembly-fact artifacts
 * (`bindings.toml`, `roster.toml`) beside them, and each projection to its
 * `.claude/**` path (parent directories created). Whole-file writes — a
 * projection is regenerated, never patched, so a hand-edited projection is
 * overwritten (that edit is drift routed to the source,
 * `specs/architecture/20-surface.md`).
 */
export function writeEmit(harness: Harness, targetDir: string, options: EmitOptions = {}): EmitResult {
  const result = emit(harness, options);
  writeFileSync(join(targetDir, "temper.toml"), result.manifest);
  writeFileSync(join(targetDir, "lock.toml"), result.lock);
  writeFileSync(join(targetDir, BINDINGS_PATH), result.bindings);
  writeFileSync(join(targetDir, ROSTER_PATH), result.roster);
  for (const projection of result.projections) {
    const path = join(targetDir, projection.path);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, projection.bytes);
  }
  return result;
}
