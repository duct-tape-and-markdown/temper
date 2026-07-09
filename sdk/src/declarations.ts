/**
 * Declaration rows — the composed program's erased declarations.
 * Every type erases at the seam: kinds, clauses, requirements, and
 * assembly facts compile to plain rows the engine reads. The **row shape** matches
 * the Rust lock's `[declaration]` families (`src/drift.rs` `Declarations`) — the
 * byte-parity lockstep two writers keep until single-writer lands
 * (`SDK-RECUT-CORPUS-FACE`). The same rows ride the internal versioned JSON pipe
 * ({@link declarationsToJson}) — not a designed IR, versioned in lockstep.
 */

import type { Harness } from "./assembly.js";
import type { EmbeddedMemberValue, KindFacts, Registration } from "./kind.js";
import type { Charset, Clause, Predicate, Requirement } from "./contract.js";
import { resolveLeaf } from "./prose.js";

/** One kind's declaration row — its identity and declared runtime facts. */
export interface KindFactRow {
  readonly name: string;
  readonly provider?: string;
  readonly governs_root: string;
  readonly governs_glob: string;
  readonly format?: string;
  readonly unit_shape?: string;
  /** The declared registration channel set's wire labels, in declaration order. */
  readonly registration?: readonly string[];
  /** The host's declared nesting templates — its embedded child kinds' names. */
  readonly templates?: readonly string[];
}

/**
 * One clause of a kind's effective contract, or one of a requirement's own
 * set-/edge-scope demands — the same row shape either way (`src/drift.rs`
 * `ClauseRow`). `kind` is absent when this row is nested inside a
 * `RequirementRow`'s own `clauses`: a requirement's demand names no kind of its
 * own.
 */
export interface ClauseRow {
  readonly kind?: string;
  readonly predicate: string;
  readonly field?: string;
  readonly severity: string;
 /** The just-in-time teaching channel the predicate cannot encode. */
  readonly guidance?: string;
 /** The external-fact source backing the clause — a doc URL plus retrieved date. */
  readonly cite?: string;
  /** The `count` predicate's satisfier-set-size bound. */
  readonly count?: { readonly min: number; readonly max: number };
  /** The `membership` predicate's target requirement name. */
  readonly target?: string;
  /** The `degree` predicate's in/out edge-count bound. */
  readonly degree?: {
    readonly incoming?: { readonly min?: number; readonly max?: number };
    readonly outgoing?: { readonly min?: number; readonly max?: number };
  };
  /** The `min_len`/`max_len`/`max_lines` predicate's scalar bound. */
  readonly bound?: { readonly min?: number; readonly max?: number };
  /** The `allowed_chars` predicate's declared character class. */
  readonly charset?: Charset;
  /** The `forbidden_keys` predicate's forbidden key list. */
  readonly keys?: readonly string[];
  /** The `deny` predicate's forbidden value list. */
  readonly values?: readonly string[];
}

/**
 * One named requirement's declaration row — the scalar facets plus its own
 * `count`/`unique`/`membership`/`degree` clause rows: the requirement's `clauses` array
 * is the whole of its set-/edge-scope demand, no facet columns beside it.
 */
export interface RequirementRow {
  readonly name: string;
  readonly kind?: string;
  readonly required: boolean;
  readonly clauses: readonly ClauseRow[];
  readonly verified_by?: string;
}

/**
 * Compile one `Clause` into its lock row: the shared `key`/`field`/`severity`/
 * `guidance`/`cite` columns — the clause's four channels surviving erasure
 * — plus, when the
 * predicate carries them, the `count`/`target`/`degree` argument columns a
 * requirement's own set-/edge-scope demand needs, and the
 * `bound`/`charset`/`keys`/`values` argument columns a kind's own node-scope
 * floor clause needs (`min_len`/`max_len`/`max_lines`'s bound,
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
    bound: nodeScopeBoundArgs(predicate),
    charset: predicate.key === "allowed_chars" ? predicate.charset : undefined,
    keys: predicate.key === "forbidden_keys" ? predicate.keys : undefined,
    values: predicate.key === "deny" ? predicate.values : undefined,
  };
}

/** `min_len`/`max_len`/`max_lines`'s scalar bound off their shared `min`/`max`
 * args keys — `undefined` for every other predicate, and for these three when
 * neither endpoint is present. */
function nodeScopeBoundArgs(predicate: Predicate): { readonly min?: number; readonly max?: number } | undefined {
  if (predicate.key !== "min_len" && predicate.key !== "max_len" && predicate.key !== "max_lines") {
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

/** One assembly-scope fact — the root member's declared enforcement mode, or an edge. */
export interface AssemblyFactRow {
  readonly fact: string;
  readonly value?: string;
  readonly from?: string;
  readonly field?: string;
  readonly to?: string;
}

/** One member→requirement fill edge — a resolved `satisfies` key. */
export interface SatisfiesRow {
  readonly member: string;
  readonly requirement: string;
}

/**
 * One authored `n` mention edge — the citing member's own `kind:name` address and
 * the address it names (another member's `kind:name`, or a bare requirement name).
 * Recorded regardless of resolution — a dangling mention is `emit`'s own refusal
 * (`emit.ts`), never this row's concern, mirroring how a `SatisfiesRow` carries a
 * claim before `refuseBrokenSource` judges it.
 */
export interface MentionRow {
  readonly member: string;
  readonly target: string;
}

/**
 * One host member's declared embedded-member value — the row an embedded member's
 * facts are carried as (0018, "the projection is not the database"): the same
 * composed value `blocks()` renders into its host's `member.<kind> <key>` TOML
 * fence, captured here as declared data rather than a second copy the engine
 * reads back off the rendered artifact. A `Text`-authored leaf is resolved (its
 * mentions display-rendered) before it lands here — the row, like the engine
 * reading it, carries only the leaf's final stored string, never the template
 * (`prose.ts`'s `resolveLeaf`).
 */
export interface NestedMemberRow {
  /** The host member's own `kind:name` address. */
  readonly host: string;
  /** The embedded child kind this value instantiates. */
  readonly kind: string;
  /** The value's key — the identity a leaf address carries. */
  readonly key: string;
  /** Prose leaves, keyed by field name — each already resolved to its final string. */
  readonly leaves: Readonly<Record<string, string>>;
  /** Sibling collections: collection name → its entries, in authored order, each entry's leaves resolved. */
  readonly collections: Readonly<Record<string, readonly ResolvedCollectionEntryRow[]>>;
}

/** One resolved sibling-collection entry: its own key plus its leaves, already resolved to strings. */
export interface ResolvedCollectionEntryRow {
  /** The entry's key among its collection's siblings. */
  readonly key: string;
  /** The entry's own leaf fields, already resolved to their final strings. */
  readonly leaves: Readonly<Record<string, string>>;
}

/** The seven declaration families — the whole erased program the lock and pipe carry. */
export interface Declarations {
  readonly kinds: readonly KindFactRow[];
  readonly clauses: readonly ClauseRow[];
  readonly requirements: readonly RequirementRow[];
  readonly assembly: readonly AssemblyFactRow[];
  readonly satisfies: readonly SatisfiesRow[];
  readonly mentions: readonly MentionRow[];
  readonly nested_members: readonly NestedMemberRow[];
}

/**
 * The lock label for a kind's declared unit shape: `file`/`directory` verbatim,
 * or `named-field(<identityField>)` for the third mode — the same `<name>(<field>)`
 * call syntax [`registrationLabel`] uses, so the id source round-trips through the
 * row rather than degrading to a bare, unreconstructable `"named-field"`.
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
 }
}

/** The lock labels for a kind's declared registration **set**, in declaration order —
 * `undefined` for an empty set, the same omit-the-column tolerance `templatesFor` takes. */
function registrationLabels(registration: readonly Registration[]): readonly string[] | undefined {
  return registration.length > 0 ? registration.map(registrationLabel) : undefined;
}

/** The stable-sort ordering every declaration row family shares. */
export function compareStrings(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0;
}

/**
 * A host kind's declared nesting templates — the embedded kinds among `allKinds`
 * whose `withinHosts` names it, name-sorted. `undefined` when the host nests nothing, so the row omits the column
 * rather than carrying an empty array.
 */
function templatesFor(hostName: string, allKinds: readonly KindFacts[]): readonly string[] | undefined {
  const names = allKinds
    .filter((facts) => facts.locus.kind === "embedded" && facts.locus.withinHosts.includes(hostName))
    .map((facts) => facts.name)
    .sort(compareStrings);
  return names.length > 0 ? names : undefined;
}

/**
 * One kind's fact row — the `at` locus supplies `governs_root`/`governs_glob`,
 * and `templates` names the embedded kinds (among `allKinds`) declared within it.
 */
function kindFactRow(facts: KindFacts, allKinds: readonly KindFacts[]): KindFactRow {
  if (facts.locus.kind !== "at") {
    // An embedded kind inherits its world residue through its host; it carries no
 // `at` locus, so it takes no kind-fact row. Callers filter these out before this point.
    throw new Error(`kind \`${facts.name}\` is embedded — it carries no locus-bearing kind fact.`);
 }
  return {
    name: facts.name,
    provider: facts.provider,
    governs_root: facts.locus.root,
    governs_glob: facts.locus.glob,
    format: facts.format,
    unit_shape: unitShapeLabel(facts),
    registration: registrationLabels(facts.registration),
    templates: templatesFor(facts.name, allKinds),
  };
}

/** Every kind in play, at any locus — member kinds ∪ expect kinds ∪ their embedded children. */
function kindsInPlay(harness: Harness): KindFacts[] {
  const byName = new Map<string, KindFacts>();
  for (const member of harness.members) byName.set(member.facts.name, member.facts);
  for (const binding of harness.expect) byName.set(binding.kind.facts.name, binding.kind.facts);
  return [...byName.values()];
}

/** The distinct locus-bearing (`at`) kinds in play, name-sorted. */
function atLocusKindsInPlay(allKinds: readonly KindFacts[]): KindFacts[] {
  return allKinds
    .filter((facts) => facts.locus.kind === "at")
    .sort((a, b) => compareStrings(a.name, b.name));
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
      kind: requirement.kind?.key,
      required: requirement.required ?? false,
      clauses: (requirement.clauses ?? []).map((clause) => clauseRow(clause)),
      verified_by: requirement.verifiedBy,
    }));
}

/**
 * The assembly-scope facts, in a stable order: the root member's declared
 * `mode` (always present"The root
 * member": harness-wide declarations are root-member fields), then one edge
 * row per kind edge field.
 */
function assemblyFactRows(harness: Harness, kinds: readonly KindFacts[]): AssemblyFactRow[] {
  const facts: AssemblyFactRow[] = [{ fact: "mode", value: harness.mode }];
  for (const kind of kinds) {
    for (const edge of kind.edgeFields ?? []) {
      facts.push({ fact: "edge", from: kind.name, field: edge.field, to: edge.to });
 }
 }
  return facts;
}

/** The `satisfies` rows — every member's fill claims, member-then-requirement sorted. */
function satisfiesRows(harness: Harness): SatisfiesRow[] {
  const rows: SatisfiesRow[] = [];
  for (const member of harness.members) {
    for (const requirement of member.satisfies) {
      rows.push({ member: member.name, requirement });
 }
 }
  return rows.sort((a, b) => compareStrings(a.member, b.member) || compareStrings(a.requirement, b.requirement));
}

/**
 * The `mention` rows — every member's authored `n` targets, member-then-target
 * sorted. `text`-kind prose contributes one row per mention, keyed to the
 * member's own `kind:name` address; `blocks()`-kind prose additionally
 * contributes one row per mention inside a `Text`-authored leaf, keyed to that
 * leaf's own `<member>/<kind>/<key>/<child-path>` address
 * ([`embeddedLeafMentionRows`]) — a `file()` body names none. Recorded off the
 * raw authored address, unconditionally — resolution is `emit`'s own refusal
 * (`emit.ts`), not this row's concern.
 */
function mentionRows(harness: Harness): MentionRow[] {
  const rows: MentionRow[] = [];
  for (const member of harness.members) {
    if (member.prose?.kind === "text") {
      const address = `${member.kind}:${member.name}`;
      for (const mention of member.prose.mentions) {
        rows.push({ member: address, target: mention.target.address });
 }
 }
    if (member.prose?.kind === "blocks") {
      for (const value of member.prose.values) {
        rows.push(...embeddedLeafMentionRows(member.name, value));
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
 * One host member's declared embedded-member value as its declaration row —
 * each `Text`-authored leaf resolved to its final stored string
 * ([`NestedMemberRow`]), mention-resolution-checked against `mentionable` the
 * identical way `emit.ts`'s `renderMemberToml` checks the same leaf on its way
 * into the rendered fence.
 */
function nestedMemberRow(host: string, value: EmbeddedMemberValue, mentionable: ReadonlySet<string>): NestedMemberRow {
  const context = (childPath: string): string => `member.${value.kind} ${value.key}: leaf \`${childPath}\``;
  const leaves: Record<string, string> = {};
  for (const [field, leaf] of Object.entries(value.leaves)) {
    leaves[field] = resolveLeaf(leaf, mentionable, context(field));
 }
  const collections: Record<string, ResolvedCollectionEntryRow[]> = {};
  for (const [collection, entries] of Object.entries(value.collections)) {
    collections[collection] = entries.map((entry) => {
      const entryLeaves: Record<string, string> = {};
      for (const [field, leaf] of Object.entries(entry.leaves)) {
        entryLeaves[field] = resolveLeaf(leaf, mentionable, context(`${collection}.${entry.key}.${field}`));
 }
      return { key: entry.key, leaves: entryLeaves };
 });
 }
  return { host, kind: value.kind, key: value.key, leaves, collections };
}

/**
 * The `nested_member` rows — every host member's `blocks()`-declared embedded-member
 * values, host-then-kind-then-key sorted. Only `blocks`-kind prose carries them (a
 * `file()`/`text` body names none); the fence rendering itself is unchanged
 * (`emit.ts`'s `resolveBody`) — this row is a second *read* of the same authored
 * value, never a second copy the engine reads back (0018).
 */
function nestedMemberRows(harness: Harness, mentionable: ReadonlySet<string>): NestedMemberRow[] {
  const rows: NestedMemberRow[] = [];
  for (const member of harness.members) {
    if (member.prose?.kind !== "blocks") continue;
    const host = `${member.kind}:${member.name}`;
    for (const value of member.prose.values) {
      rows.push(nestedMemberRow(host, value, mentionable));
 }
 }
  return rows.sort(
    (a, b) => compareStrings(a.host, b.host) || compareStrings(a.kind, b.kind) || compareStrings(a.key, b.key),
 );
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
 * `kind:name`. Shared by `emit.ts` (a member-level `Text` body's mentions) and
 * this module (an embedded member's `Text` leaves) — the one resolution-check
 * set, so a leaf mention and a member mention are held to the identical bar.
 */
export function declaredAddresses(harness: Harness): Set<string> {
  const set = declaredRequirements(harness);
  for (const member of harness.members) set.add(`${member.kind}:${member.name}`);
  return set;
}

/** Compile a harness into its seven declaration families — the erased program. */
export function compileDeclarations(harness: Harness): Declarations {
  const allKinds = kindsInPlay(harness);
  const kinds = atLocusKindsInPlay(allKinds);
  const clauses: ClauseRow[] = [];
  for (const binding of [...harness.expect].sort((a, b) => compareStrings(a.kind.key, b.kind.key))) {
    for (const clause of binding.clauses) {
      clauses.push(clauseRow(clause, binding.kind.key));
 }
 }
  return {
    kinds: kinds.map((facts) => kindFactRow(facts, allKinds)),
    clauses,
    requirements: requirementRows(harness),
    assembly: assemblyFactRows(harness, kinds),
    satisfies: satisfiesRows(harness),
    mentions: mentionRows(harness),
    nested_members: nestedMemberRows(harness, declaredAddresses(harness)),
  };
}

/** The SDK's pinned engine/interchange version — the JSON pipe rides it in lockstep. */
export const SEAM_VERSION = 2;

/**
 * Serialize a payload to the internal versioned JSON pipe. Deterministic:
 * insertion-ordered keys and a trailing newline, so a re-emit is byte-identical.
 */
export function encodeSeam(payload: object): string {
  return JSON.stringify({ version: SEAM_VERSION, ...payload }, null, 2) + "\n";
}

/**
 * Serialize the declaration rows to the internal versioned JSON pipe.
 * Not a designed IR — a stable public interchange
 * is admitted only when its consumer lands.
 */
export function declarationsToJson(declarations: Declarations): string {
  return encodeSeam(declarations);
}
