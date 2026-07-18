/**
 * Declaration rows — the composed program's erased declarations.
 * Every type erases at the seam: kinds, clauses, requirements, and
 * assembly facts compile to plain rows the engine reads. The **row shapes are the
 * generated `ts-rs` bindings** (`./generated/`, derived from `src/drift.rs`), so a
 * Rust-side row rename is a compile error here, never a silent shape drift. This
 * module authors the builders that fill them; the same rows ride the internal
 * versioned JSON pipe ({@link encodeSeam}) — not a designed IR, versioned
 * in lockstep.
 */

import { fileURLToPath } from "node:url";

import type { Harness } from "./assembly.js";
import type { EmbeddedMemberValue, KindFacts, Layout, Registration } from "./kind.js";
import type { Clause, Predicate, Requirement, Verifier } from "./contract.js";
import type { Include, MentionScope } from "./prose.js";
import { isTextSpan, resolveLeaf } from "./prose.js";

import type {
  AssemblyFactRow,
  ClauseRow,
  CollectionAddressRow,
  CollectionEntryWire,
  Declarations,
  IncludeRow,
  KindFactRow,
  LayoutRegionRow,
  LayoutRow,
  MentionRow,
  NestedMemberRow,
  Payload,
  RegistrationRow,
  RequirementRow,
  SatisfiesRow,
  SettingsRow,
  TemplateRow,
  Verifier as VerifierRow,
} from "./generated/index.js";

// The row shapes the authoring API surfaces re-export from here, so `index.ts`'s
// public face is unchanged by the move to generated bindings.
export type {
  AssemblyFactRow,
  ClauseRow,
  Declarations,
  KindFactRow,
  RequirementRow,
  SatisfiesRow,
} from "./generated/index.js";

/**
 * Compile one `Clause` into its lock row: the shared `key`/`field`/`severity`/
 * `guidance`/`cite` columns — the clause's four channels surviving erasure
 * — plus, when the
 * predicate carries them, the `count`/`target`/`degree` argument columns a
 * requirement's own set-/edge-scope demand needs, and the
 * `bound`/`charset`/`keys`/`values` argument columns a kind's own node-scope
 * floor clause needs (`min_len`/`max_len`/`extent`'s bound, `extent`'s unit,
 * `allowed_chars`'s charset, `forbidden_keys`'s keys, `deny`'s values) — so
 * the lock encodes the floor losslessly, not identity+severity alone. `kind`
 * is supplied only for a kind's own `expect` clause; a requirement's nested
 * clause carries none.
 */
function clauseRow(clause: Clause, kind?: string): ClauseRow {
  const { predicate } = clause;
  return {
    kind,
    predicate: predicate.key,
    field: predicate.field,
    severity: clause.severity,
    guidance: clause.guidance,
    cite: clause.cite,
    count:
      predicate.key === "count"
        ? { min: predicate.args?.min ?? 0, max: predicate.args?.max ?? Number.MAX_SAFE_INTEGER }
        : undefined,
    target: predicate.key === "membership" ? predicate.target : undefined,
    degree:
      predicate.key === "degree"
        ? {
            incoming: edgeBoundArgs(predicate.args, "incoming"),
            outgoing: edgeBoundArgs(predicate.args, "outgoing"),
 }
        : undefined,
    gate: predicate.key === "mention-reachable" ? predicate.gate : undefined,
    value_type:
      predicate.key === "type" && predicate.value_type ? [...predicate.value_type] : undefined,
    shape: predicate.key === "shape" ? predicate.shape : undefined,
    bound: nodeScopeBoundArgs(predicate),
    unit: predicate.key === "extent" ? predicate.unit : undefined,
    // The generated rows carry mutable columns; the predicate's `value_type`/
    // `charset`/`keys`/`values` are read-only, so copy each into a fresh
    // array/object — the same bytes, a shape the row will accept.
    charset:
      predicate.key === "allowed_chars" && predicate.charset !== undefined
        ? {
            ranges: predicate.charset.ranges ? [...predicate.charset.ranges] : undefined,
            chars: predicate.charset.chars,
          }
        : undefined,
    keys: predicate.key === "forbidden_keys" && predicate.keys ? [...predicate.keys] : undefined,
    values:
      (predicate.key === "deny" || predicate.key === "enum") && predicate.values
        ? [...predicate.values]
        : undefined,
    range:
      predicate.key === "range" && predicate.range !== undefined
        ? { min: predicate.range.min, max: predicate.range.max }
        : undefined,
    section:
      predicate.key === "section_contains" && predicate.section !== undefined
        ? { heading: predicate.section.heading, marker: predicate.section.marker }
        : undefined,
  };
}

/** `min_len`/`max_len`/`extent`'s scalar bound off their shared `min`/`max`
 * args keys — `undefined` for every other predicate, and for these three when
 * neither endpoint is present. */
function nodeScopeBoundArgs(predicate: Predicate): { readonly min?: number; readonly max?: number } | undefined {
  if (predicate.key !== "min_len" && predicate.key !== "max_len" && predicate.key !== "extent") {
    return undefined;
 }
  const min = predicate.args?.min;
  const max = predicate.args?.max;
  return min === undefined && max === undefined ? undefined : { min, max };
}

/** One `degree` direction's `{min, max}` off its flat `<dir>_min`/`<dir>_max` args
 * keys — `undefined` when neither is present (that direction is unconstrained). */
function edgeBoundArgs(
  args: Readonly<Record<string, number>> | undefined,
  direction: "incoming" | "outgoing",
): { readonly min?: number; readonly max?: number } | undefined {
  const min = args?.[`${direction}_min`];
  const max = args?.[`${direction}_max`];
  return min === undefined && max === undefined ? undefined : { min, max };
}

/**
 * The lock label for a kind's declared unit shape: `file`/`directory`/`starred-segment`
 * verbatim, or `named-field(<identityField>)` for the field-sourced mode — the same
 * `<name>(<field>)` call syntax [`registrationLabel`] uses, so the id source round-trips
 * through the row rather than degrading to a bare, unreconstructable `"named-field"`.
 */
function unitShapeLabel(facts: KindFacts): string | undefined {
  if (facts.unitShape !== "named-field") return facts.unitShape;
  return `named-field(${facts.identityField})`;
}

/** The lock label for one declared registration channel. */
function registrationLabel(registration: Registration): string {
  switch (registration.via) {
    case "always":
      return "always";
    case "user-invoked":
      return "user-invoked";
    case "description-trigger":
      return `description-trigger(${registration.field})`;
    case "paths-match":
      return `paths-match(${registration.field})`;
    case "event":
      return `event(${registration.field})`;
    case "connection":
      return "connection";
    case "enablement":
      return "enablement";
 }
}

/** The lock labels for a kind's declared registration **set**, in declaration order —
 * `undefined` for an empty set, the same omit-the-column tolerance `templatesFor` takes. */
function registrationLabels(registration: readonly Registration[]): string[] | undefined {
  return registration.length > 0 ? registration.map(registrationLabel) : undefined;
}

/** The stable-sort ordering every declaration row family shares. */
export function compareStrings(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0;
}

/**
 * A host kind's nesting templates, from its two declaration loci. The kind's own
 * declared templates carry each layer — child kind, plus a file layer's path pattern.
 * An adopting corpus that admits its own embedded kinds over the host overrides the
 * *embedded* grain only: an admission names a host and a child kind but no path, so it
 * can only speak for the pathless (embedded) layer. The path-carrying file layers the
 * host declares are the host's own facts and stand, joined with the admitted rows —
 * else composing a body over a host wipes its declared file layer and the engine, finding
 * no file template, refuses the host's nested file children.
 *
 * `undefined` when nothing is declared or admitted, so the row omits the column rather
 * than carrying an empty array.
 */
function templatesFor(facts: KindFacts, admissions: AdmissionsByHost): TemplateRow[] | undefined {
  const declared = (facts.templates ?? []).map((template): TemplateRow => ({
    kind: template.kind.key,
    path: template.path,
  }));
  const admitted = [...(admissions.get(facts.name) ?? [])].sort(compareStrings);
  if (admitted.length === 0) return declared.length > 0 ? declared : undefined;
  const fileLayers = declared.filter((template) => template.path !== undefined);
  return [...fileLayers, ...admitted.map((kind): TemplateRow => ({ kind }))];
}

/**
 * Lower a kind's declared {@link Layout} into its `content` row — one flat
 * discriminator-plus-columns [`LayoutRegionRow`] per region, `memberKind` spelled as the
 * wire's snake_case `member_kind`. `undefined` for a `file`-content kind (no declared
 * content), so its row omits the column and stays byte-identical.
 */
function contentRow(content: Layout | undefined): LayoutRow | undefined {
  if (content === undefined) return undefined;
  const regions = content.regions.map((region): LayoutRegionRow => {
    switch (region.region) {
      case "prose":
        return { region: "prose", import: region.import };
      case "field":
        return { region: "field", slot: region.slot };
      case "collection":
        return { region: "collection", member_kind: region.memberKind, key: region.key };
    }
  });
  return { regions };
}

/**
 * Lower a kind's declared {@link CollectionAddress} into its `collection_address` row —
 * `keyPath` spelled as the wire's snake_case `key_path`. `undefined` for a file-locus
 * kind, so its row omits the column and stays byte-identical.
 */
function collectionAddressRow(facts: KindFacts): CollectionAddressRow | undefined {
  if (facts.collectionAddress === undefined) return undefined;
  return { manifest: facts.collectionAddress.manifest, key_path: facts.collectionAddress.keyPath };
}

/**
 * One kind's fact row — an `at` locus supplies `governs_root`/`governs_glob` and a
 * nested-file kind neither (its path composes from its host's unit and the host
 * template's pattern, so it governs no glob). A file locus's `commitment` class rides
 * the same spelling, absent for the committed default. `templates` names the embedded kinds the
 * corpus admits over it, and `content` lowers a declared layout (absent for a
 * `file`-content kind). A registration kind extends the row with its `shape` marker and
 * `collection_address`.
 */
function kindFactRow(facts: KindFacts, admissions: AdmissionsByHost): KindFactRow {
  if (facts.locus.kind === "embedded") {
    // An embedded kind inherits its world residue through its host; it owns no unit at
 // all, so it takes no kind-fact row. Callers filter these out before this point.
    throw new Error(`kind \`${facts.name}\` is embedded — it carries no locus-bearing kind fact.`);
 }
  const governs = facts.locus.kind === "at" ? facts.locus : undefined;
  return {
    name: facts.name,
    provider: facts.provider,
    governs_root: governs?.root,
    governs_glob: governs?.glob,
    commitment: governs?.commitment,
    format: facts.format,
    unit_shape: unitShapeLabel(facts),
    registration: registrationLabels(facts.registration),
    templates: templatesFor(facts, admissions),
    content: contentRow(facts.content),
    shape: facts.shape,
    collection_address: collectionAddressRow(facts),
  };
}

/**
 * Every kind in play, at any locus — member kinds ∪ expect kinds ∪ their embedded
 * children — name-sorted, so every family derived from it inherits one stable order.
 *
 * The embedded children close transitively over both channels a host names one through:
 * `admit` (the adopting corpus's declaration) and a path-less `templates` entry (the
 * embedded layer). An embedded kind reaches the lock through its host's `templates`
 * column rather than a row of its own, so those channels are the only way one is in play
 * at all — and its declared edge fields are still owed their assembly facts.
 *
 * Only *embedded* children are drawn in. A path-carrying template is the nested-file
 * layer, whose child owns a unit and reaches the lock through `expect` like any other
 * unit kind; pulling one in here would forge it a kind-fact row it never declared.
 */
function kindsInPlay(harness: Harness): KindFacts[] {
  const byName = new Map<string, KindFacts>();
  const pending: KindFacts[] = [];
  const admit = (facts: KindFacts): void => {
    if (byName.has(facts.name)) return;
    byName.set(facts.name, facts);
    pending.push(facts);
  };
  for (const member of harness.members) admit(member.facts);
  for (const binding of harness.expect) admit(binding.kind.facts);
  for (const { admits } of harness.admit) for (const child of admits) admit(child.facts);
  for (let facts = pending.pop(); facts !== undefined; facts = pending.pop()) {
    for (const template of facts.templates ?? []) {
      if (template.kind.facts.locus.kind === "embedded") admit(template.kind.facts);
    }
  }
  return [...byName.values()].sort((a, b) => compareStrings(a.name, b.name));
}

/** The distinct discoverable (`at`) kinds in play. */
function atLocusKindsInPlay(allKinds: readonly KindFacts[]): KindFacts[] {
  return allKinds.filter((facts) => facts.locus.kind === "at");
}

/**
 * The distinct unit-owning kinds in play — every locus but `embedded`. The kinds that
 * take a fact row: a nested-file kind owns a file the engine must place, and places it
 * off its row, though it governs no glob to be discovered at.
 */
function unitKindsInPlay(allKinds: readonly KindFacts[]): KindFacts[] {
  return allKinds.filter((facts) => facts.locus.kind !== "embedded");
}

/** The requirement rows — assembly `require` and every member's `requires`, one namespace. */
function requirementRows(harness: Harness): RequirementRow[] {
  const merged = new Map<string, Requirement>();
  const publish = (name: string, requirement: Requirement, source: string): void => {
    const existing = merged.get(name);
    if (existing !== undefined && existing !== requirement) {
      // One namespace, one fill mechanism; a cross-publisher name collision is an
 // admissibility finding, never a shadowing rule.
      throw new Error(
        `requirement \`${name}\` is published twice (${source} collides with an earlier ` +
          `publisher) — a name collision across publishers is an admissibility finding.`,
 );
 }
    merged.set(name, requirement);
  };
  for (const [name, requirement] of Object.entries(harness.require)) publish(name, requirement, "the assembly");
  for (const member of harness.members) {
    for (const [name, requirement] of Object.entries(member.requires)) {
      publish(name, requirement, `member \`${member.name}\``);
 }
 }
  return [...merged.entries()]
    .sort(([a], [b]) => compareStrings(a, b))
    .map(([name, requirement]) => ({
      name,
      kind: typeof requirement.kind === "string" ? requirement.kind : requirement.kind?.key,
      required: requirement.required ?? false,
      clauses: (requirement.clauses ?? []).map((clause) => clauseRow(clause)),
      verifier: verifierRow(requirement.verifier),
      prose: requirement.prose,
    }));
}

/**
 * Lower a typed verifier to its species-tagged wire row — `species` plus the
 * variant's own payload. The generated row carries a mutable `events` column, so
 * the telemetry species copies its read-only names into a fresh array (the same
 * read-only→mutable copy `charset`/`keys`/`values` make above).
 */
function verifierRow(verifier: Verifier | undefined): VerifierRow | undefined {
  if (verifier === undefined) return undefined;
  return verifier.species === "script"
    ? { species: "script", path: verifier.path }
    : { species: "telemetry", events: [...verifier.events] };
}

/**
 * The assembly-scope facts, in a stable order: the root member's declared `mode`, then
 * one edge row per kind edge field.
 *
 * `kinds` is every kind in play at *any* locus, not just the unit-owning ones that take
 * a kind-fact row: an edge is a declared relationship at any grain, so an embedded
 * kind's edge fields are owed their rows even though the kind itself reaches the lock
 * through its host's `templates` column alone.
 */
function assemblyFactRows(harness: Harness, kinds: readonly KindFacts[]): AssemblyFactRow[] {
  const facts: AssemblyFactRow[] = [{ fact: "mode", value: harness.mode }];
  for (const kind of kinds) {
    for (const edge of kind.edgeFields ?? []) {
      facts.push({ fact: "edge", from: kind.name, field: edge.field, to: [...edge.to] });
 }
 }
  return facts;
}

/**
 * The `satisfies` rows — every member's fill claims, member-then-requirement sorted.
 * The `member` is the filler's own `kind:name` address, the same identity
 * `mentionRows` writes, so the read side joins on a kind-qualified label a same-named
 * member of another kind can never collide with.
 */
function satisfiesRows(harness: Harness): SatisfiesRow[] {
  const rows: SatisfiesRow[] = [];
  for (const member of harness.members) {
    const address = `${member.kind}:${member.name}`;
    for (const requirement of member.satisfies) {
      rows.push({ member: address, requirement });
 }
 }
  return rows.sort((a, b) => compareStrings(a.member, b.member) || compareStrings(a.requirement, b.requirement));
}

/**
 * The `mention` rows — every member's authored `n` targets, member-then-target
 * sorted. `text`-kind prose contributes one row per mention, keyed to the
 * member's own `kind:name` address. A `blocks()` composed body keys each child
 * to what it is: a prose span's mentions are host-level, keyed to the member's
 * own `kind:name` address like a `text` body; an embedded value's `Text`-leaf
 * mentions are keyed to that leaf's own `<member>/<kind>/<key>/<child-path>`
 * address ([`embeddedLeafMentionRows`]). A `file()` body names none. Recorded off
 * the raw authored address, unconditionally — resolution is `emit`'s own refusal
 * (`emit.ts`), not this row's concern.
 */
function mentionRows(harness: Harness): MentionRow[] {
  const rows: MentionRow[] = [];
  for (const member of harness.members) {
    const address = `${member.kind}:${member.name}`;
    if (member.prose?.kind === "text") {
      for (const mention of member.prose.mentions) {
        rows.push({ member: address, target: mention.target.address });
 }
 }
    if (member.prose?.kind === "blocks") {
      for (const value of member.prose.values) {
        if (isTextSpan(value)) {
          for (const mention of value.mentions) {
            rows.push({ member: address, target: mention.target.address });
 }
        } else {
          rows.push(...embeddedLeafMentionRows(member.name, value));
 }
 }
 }
 }
  return rows.sort((a, b) => compareStrings(a.member, b.member) || compareStrings(a.target, b.target));
}

/**
 * The mention rows one `blocks()`-declared embedded-member value's `Text` leaves
 * contribute — top-level leaves addressed by their bare field name, a
 * collection entry's leaves addressed `<collection>.<entry>.<field>` (one layer
 * deep, matching the row's own shape) — each row keyed to the leaf's own
 * structural address, the `<member>/<kind>/<key>/<child-path>` grammar
 * `src/read.rs`'s `parse_leaf_address` resolves. A bare-string leaf names no
 * mention.
 */
function embeddedLeafMentionRows(hostName: string, value: EmbeddedMemberValue): MentionRow[] {
  const rows: MentionRow[] = [];
  const addressed = (childPath: string): string => `${hostName}/${value.kind}/${value.key}/${childPath}`;
  for (const [field, leaf] of Object.entries(value.leaves)) {
    if (typeof leaf === "string") continue;
    for (const mention of leaf.mentions) {
      rows.push({ member: addressed(field), target: mention.target.address });
 }
 }
  for (const [collection, entries] of Object.entries(value.collections)) {
    for (const entry of entries) {
      for (const [field, leaf] of Object.entries(entry.leaves)) {
        if (typeof leaf === "string") continue;
        for (const mention of leaf.mentions) {
          rows.push({ member: addressed(`${collection}.${entry.key}.${field}`), target: mention.target.address });
 }
 }
 }
 }
  return rows;
}

/**
 * The `include` rows — every member's `text`-body includes, in member-then-authored
 * order. Each carries the host member's `kind:name` address and the include target's
 * path resolved against the stating module ({@link fileURLToPath} over the include's own
 * `moduleUrl`), never the workspace — the engine reads, splices, and fingerprints it.
 * Member order stays authored (never target-sorted): the body's include slots ride the
 * same order, so the engine pairs the k-th slot with the k-th row. Includes ride a `text`
 * body and a composed body's prose spans alike, in authored order across the interleave; an
 * embedded leaf carries none (refused at {@link resolveLeaf}) and a `file()` body names none.
 */
function includeRows(harness: Harness): IncludeRow[] {
  const rows: IncludeRow[] = [];
  const push = (address: string, include: Include): void => {
    rows.push({ member: address, source_path: fileURLToPath(new URL(include.path, include.moduleUrl)) });
  };
  for (const member of harness.members) {
    const address = `${member.kind}:${member.name}`;
    if (member.prose?.kind === "text") {
      for (const include of member.prose.includes) push(address, include);
    } else if (member.prose?.kind === "blocks") {
      for (const value of member.prose.values) {
        if (!isTextSpan(value)) continue;
        for (const include of value.includes) push(address, include);
      }
    }
  }
  return rows;
}

/**
 * One composed embedded value's key in an {@link EdgePlacements} table — its host's
 * `kind:name` address plus the value's own kind and key, the same triple the
 * `nested_member` row it feeds is identified by.
 */
export function placementKey(host: string, kind: string, key: string): string {
  return `${host}${kind}${key}`;
}

/**
 * Each composed embedded value's placed edge fields, by {@link placementKey} — `emit`'s
 * record of which declared edges each value's format actually rendered (`emit.ts`'s
 * `edgePlacements`). A value with no entry had no format observe it, which is distinct
 * from a format that placed nothing: the row omits the column entirely and the
 * `format-places-edges` clause stays undecided rather than indict a format that never ran.
 */
export type EdgePlacements = ReadonlyMap<string, readonly string[]>;

/** One composed embedded value's rendered extent — the line and character count of the
 * block `emit` projected for it ({@link RenderedExtents}), the span an `extent` clause
 * budgets. */
export interface RenderedExtent {
  readonly lines: number;
  readonly chars: number;
}

/**
 * Each composed embedded value's rendered extent, by {@link placementKey} — `emit`'s
 * record of the span it projected for each value (`emit.ts`'s `renderedExtents`). A value
 * with no entry had no format rendered it (a member read off a layout host's source),
 * which the row spells by omitting both span columns rather than a captured zero: an
 * unmeasured member's `extent` stays undecidable, never a zero read as a pass.
 */
export type RenderedExtents = ReadonlyMap<string, RenderedExtent>;

/**
 * One host member's declared embedded-member value as its declaration row —
 * each `Text`-authored leaf resolved to its final stored string
 * ([`NestedMemberRow`]), mention-resolution-checked against `scope` the identical
 * way `emit.ts`'s `renderMemberToml` checks the same leaf on its way into the
 * rendered fence — plus the `placed_edges` record `emit` observed while rendering the
 * same value, which is the only way an edge's placement reaches the engine.
 */
function nestedMemberRow(
  host: string,
  value: EmbeddedMemberValue,
  scope: MentionScope,
  placements: EdgePlacements | undefined,
  extents: RenderedExtents | undefined,
): NestedMemberRow {
  const context = (childPath: string): string => `member.${value.kind} ${value.key}: leaf \`${childPath}\``;
  const leaves: Record<string, string> = {};
  for (const [field, leaf] of Object.entries(value.leaves)) {
    leaves[field] = resolveLeaf(leaf, scope, context(field));
 }
  const collections: Record<string, CollectionEntryWire[]> = {};
  for (const [collection, entries] of Object.entries(value.collections)) {
    collections[collection] = entries.map((entry) => {
      const entryLeaves: Record<string, string> = {};
      for (const [field, leaf] of Object.entries(entry.leaves)) {
        entryLeaves[field] = resolveLeaf(leaf, scope, context(`${collection}.${entry.key}.${field}`));
 }
      return { key: entry.key, leaves: entryLeaves };
 });
 }
  const key = placementKey(host, value.kind, value.key);
  const placed = placements?.get(key);
  const extent = extents?.get(key);
  return {
    host,
    kind: value.kind,
    key: value.key,
    leaves,
    collections,
    // Omitted, never `undefined`-valued: an absent column is the wire's own spelling of
    // "no format placement was observed here", so a value with no edge to place keeps
    // the row it has always written. The generated row carries a mutable column and the
    // placement record is read-only, so the copy is a fresh array.
    ...(placed === undefined ? {} : { placed_edges: [...placed] }),
    // The rendered span, on the same omitted-not-null discipline: a value no render
    // observed (compiled without `emit`'s measurement) keeps the row it has always
    // written, and its `extent` reads as undecidable rather than a captured zero.
    ...(extent === undefined ? {} : { rendered_lines: extent.lines, rendered_chars: extent.chars }),
  };
}

/** Each host kind's admitted embedded kinds, by host kind name — the corpus's `admit`, indexed. */
type AdmissionsByHost = ReadonlyMap<string, ReadonlySet<string>>;

/**
 * The corpus's `admit` declarations, indexed by host kind name — the one source both
 * {@link templatesFor} and {@link nestedMemberRows} read. Admission carries kind values,
 * so an admitted kind is imported and thereby in play; repeated hosts union.
 *
 * # Throws
 * If an admission names a non-embedded kind: a body composes embedded members, and a
 * file-locus kind owns a file instead — admitting one declares a nesting no locus backs.
 */
function admissionsByHost(harness: Harness): AdmissionsByHost {
  const map = new Map<string, Set<string>>();
  for (const { host, admits } of harness.admit) {
    const admitted = map.get(host.key) ?? new Set<string>();
    for (const child of admits) {
      if (child.facts.locus.kind !== "embedded") {
        throw new Error(
          `host kind \`${host.key}\` admits \`${child.key}\`, which is not an embedded kind — ` +
            `a composed body admits embedded members only ` +
            `(specs/model/representation.md, "nesting").`,
        );
      }
      admitted.add(child.key);
    }
    map.set(host.key, admitted);
  }
  return map;
}

/**
 * The `nested_member` rows — every host member's `blocks()`-declared embedded-member
 * values, host-then-kind-then-key sorted. Only a composed body's embedded values carry
 * them (a `file()`/`text` body — and a composed body's prose spans — name none); the
 * fence rendering itself is unchanged
 * (`emit.ts`'s `resolveBody`) — this row is a second *read* of the same authored
 * value, never a second copy the engine reads back (0018).
 *
 * Refuses an unadmitted nesting before a byte is written: the corpus must admit the
 * value's kind over the hosting member's kind. `templates` derives from that same
 * admission ({@link templatesFor}), so an unadmitted value would reach the lock as a
 * `nested_member` row no `templates` column admits, to be unmodeled without a word —
 * admission is the adopting corpus's own declaration, so an unadmitted nested member is
 * an unresolved input, not output to write over.
 */
function nestedMemberRows(
  harness: Harness,
  admissions: AdmissionsByHost,
  scope: MentionScope,
  placements: EdgePlacements | undefined,
  extents: RenderedExtents | undefined,
): NestedMemberRow[] {
  const rows: NestedMemberRow[] = [];
  for (const member of harness.members) {
    if (member.prose?.kind !== "blocks") continue;
    const host = `${member.kind}:${member.name}`;
    for (const value of member.prose.values) {
      if (isTextSpan(value)) continue;
      if (!admissions.get(member.kind)?.has(value.kind)) {
        throw new Error(
          `member \`${member.name}\`: embedded value \`${value.key}\` is of kind ` +
            `\`${value.kind}\`, which does not nest within host kind \`${member.kind}\` — a ` +
            `\`blocks()\` value's kind must be one the harness \`admit\`s over the host kind ` +
            `(specs/model/representation.md, "nesting").`,
        );
      }
      rows.push(nestedMemberRow(host, value, scope, placements, extents));
 }
 }
  return rows.sort(
    (a, b) => compareStrings(a.host, b.host) || compareStrings(a.kind, b.kind) || compareStrings(a.key, b.key),
 );
}

/**
 * The `registration` rows — every fields-only registration member (a hook, an MCP server)
 * erased for the manifest write face, kind-then-key sorted so double emit is byte-stable.
 * Each carries its identity (`kind`/`key`), its collection address (`manifest`/`keyPath`,
 * the wire's snake_case `key_path`), and its folded typed fields — the entry value the
 * engine's write face places under `key`. The one source `emit.ts`'s public
 * {@link RegistrationFact} view also maps from, so the seam and the `EmitResult` sibling
 * cannot disagree on what a manifest carries.
 *
 * # Throws
 * If a fields-only member declares no collection address — it surfaces in no host manifest.
 */
export function registrationRows(harness: Harness): RegistrationRow[] {
  return harness.members
    .filter((member) => member.facts.shape === "fields")
    .map((member): RegistrationRow => {
      const address = member.facts.collectionAddress;
      if (address === undefined) {
        throw new Error(
          `member \`${member.name}\`: a fields-only registration kind declares no ` +
            `collection address — it surfaces in no host manifest (specs/model/pipeline.md, "The SDK").`,
        );
      }
      return {
        kind: member.kind,
        key: member.name,
        manifest: address.manifest,
        key_path: address.keyPath,
        // The generated row carries a mutable field list; the member's is read-only, so
        // copy each pair into a fresh tuple — the same values, a shape the row accepts.
        fields: member.fields.map(([name, value]): [string, unknown] => [name, value]),
      };
    })
    .sort((a, b) => compareStrings(a.kind, b.kind) || compareStrings(a.key, b.key));
}

/**
 * The manifest Claude Code's harness-level settings reside in — the file the assembly's
 * residual settings keys fold into as opaque residue, the same manifest the `hook` kind's
 * registrations surface inside (code.claude.com/docs/en/settings, retrieved 2026-07-10).
 */
const SETTINGS_MANIFEST = "settings.json";

/**
 * The `settings` rows — the assembly's harness-level residual settings keys, each folded
 * into the settings.json manifest's opaque residue at emit. Key-sorted so double emit is
 * byte-stable. Seam-inbound: the value lives in the projected manifest, never the lock.
 */
export function settingsRows(harness: Harness): SettingsRow[] {
  return Object.entries(harness.settings)
    .map(([key, value]): SettingsRow => ({ manifest: SETTINGS_MANIFEST, key, value }))
    .sort((a, b) => compareStrings(a.key, b.key));
}

/** Every requirement name a `satisfies` claim may fill — assembly `require` ∪ member `requires`. */
export function declaredRequirements(harness: Harness): Set<string> {
  const set = new Set<string>();
  for (const name of Object.keys(harness.require)) set.add(name);
  for (const member of harness.members) {
    for (const name of Object.keys(member.requires)) set.add(name);
 }
  return set;
}

/**
 * Every address a mention may name — declared requirement names ∪ each member's
 * `kind:name` ∪ each `blocks()`-declared embedded member's host-scoped
 * `<host-kind>:<host-name>/<kind>/<key>` address. Shared by `emit.ts` (a
 * member-level `Text` body's mentions) and this module (an embedded member's
 * `Text` leaves) — the one resolution-check set, so a leaf mention and a member
 * mention are held to the identical bar. The embedded address is host-scoped,
 * never a flat `<kind>:<key>` — flat would force corpus-wide key uniqueness on
 * embedded kinds.
 */
export function declaredAddresses(harness: Harness): Set<string> {
  const set = declaredRequirements(harness);
  for (const member of harness.members) {
    set.add(`${member.kind}:${member.name}`);
    if (member.prose?.kind !== "blocks") continue;
    for (const value of member.prose.values) {
      if (isTextSpan(value)) continue;
      set.add(`${member.kind}:${member.name}/${value.kind}/${value.key}`);
    }
  }
  return set;
}

/**
 * Every discoverable (`at`-locus) kind the program declares — the deferral signal a
 * dangling mention is measured against (`prose.ts`'s `defersToGate`): a mention naming
 * one of these whose member is not a composed value defers to `check`, while a mention
 * naming no declared kind refuses at emit. Member kinds ∪ `expect` kinds; an embedded
 * kind is excluded — its members are composed within a host, never discovered, so a
 * flat `kind:name` mention of one has no discovery locus to defer to.
 */
export function declaredAtLocusKinds(harness: Harness): Set<string> {
  return new Set(atLocusKindsInPlay(kindsInPlay(harness)).map((facts) => facts.name));
}

/** The full {@link MentionScope} the program resolves a mention against — its addresses and its deferral kinds. */
export function mentionScope(harness: Harness): MentionScope {
  return { mentionable: declaredAddresses(harness), deferrableKinds: declaredAtLocusKinds(harness) };
}

/**
 * Compile a harness into its seven declaration families — the erased program.
 *
 * `placements` and `extents` are `emit`'s record of what each embedded value's format
 * rendered (`emit.ts`'s `edgePlacements`) and the span it spanned (`renderedExtents`); this
 * pass compiles declarations and never renders, so it observes neither itself. Omitted,
 * every `nested_member` row omits its `placed_edges`/`rendered_lines`/`rendered_chars`
 * columns — honest (nothing observed a render) but undecidable for a `format-places-edges`
 * or `extent` clause, so a whole compile goes through `emit`, never this alone.
 */
export function compileDeclarations(
  harness: Harness,
  placements?: EdgePlacements,
  extents?: RenderedExtents,
): Declarations {
  const allKinds = kindsInPlay(harness);
  const admissions = admissionsByHost(harness);
  const clauses: ClauseRow[] = [];
  for (const binding of [...harness.expect].sort((a, b) => compareStrings(a.kind.key, b.kind.key))) {
    for (const clause of binding.clauses) {
      clauses.push(clauseRow(clause, binding.kind.key));
 }
 }
  return {
    kinds: unitKindsInPlay(allKinds).map((facts) => kindFactRow(facts, admissions)),
    clauses,
    requirements: requirementRows(harness),
    assembly: assemblyFactRows(harness, allKinds),
    satisfies: satisfiesRows(harness),
    mentions: mentionRows(harness),
    includes: includeRows(harness),
    nested_members: nestedMemberRows(harness, admissions, mentionScope(harness), placements, extents),
    registrations: registrationRows(harness),
    settings: settingsRows(harness),
  };
}

/** The SDK's pinned engine/interchange version — the JSON pipe rides it in lockstep. */
export const SEAM_VERSION = 2;

/**
 * Serialize the seam payload to the internal versioned JSON pipe — `encodeSeam`
 * stamps `version`, so the caller supplies the rest of the {@link Payload}.
 * Deterministic: insertion-ordered keys and a trailing newline, so a re-emit is
 * byte-identical.
 */
export function encodeSeam(payload: Omit<Payload, "version">): string {
  return JSON.stringify({ version: SEAM_VERSION, ...payload }, null, 2) + "\n";
}
