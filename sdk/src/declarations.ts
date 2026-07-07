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
import type { KindFacts, Registration } from "./kind.js";
import type { Charset, Clause, Predicate, Requirement } from "./contract.js";

/** One kind's declaration row — its identity and declared runtime facts. */
export interface KindFactRow {
  readonly name: string;
  readonly provider?: string;
  readonly governs_root: string;
  readonly governs_glob: string;
  readonly format?: string;
  readonly unit_shape?: string;
  readonly registration?: string;
  /** The host's declared nesting templates — its embedded genre kinds' names. */
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

/** The five declaration families — the whole erased program the lock and pipe carry. */
export interface Declarations {
  readonly kinds: readonly KindFactRow[];
  readonly clauses: readonly ClauseRow[];
  readonly requirements: readonly RequirementRow[];
  readonly assembly: readonly AssemblyFactRow[];
  readonly satisfies: readonly SatisfiesRow[];
}

/** The lock label for a kind's declared registration. */
function registrationLabel(registration: Registration): string {
  switch (registration.via) {
    case "always":
      return "always";
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

/**
 * A host kind's declared nesting templates — the genre kinds among `allKinds`
 * whose `withinHosts` names it, name-sorted. `undefined` when the host nests nothing, so the row omits the column
 * rather than carrying an empty array.
 */
function templatesFor(hostName: string, allKinds: readonly KindFacts[]): readonly string[] | undefined {
  const names = allKinds
    .filter((facts) => facts.locus.kind === "genre" && facts.locus.withinHosts.includes(hostName))
    .map((facts) => facts.name)
    .sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
  return names.length > 0 ? names : undefined;
}

/**
 * One kind's fact row — the `at` locus supplies `governs_root`/`governs_glob`,
 * and `templates` names the genre kinds (among `allKinds`) declared within it.
 */
function kindFactRow(facts: KindFacts, allKinds: readonly KindFacts[]): KindFactRow {
  if (facts.locus.kind !== "at") {
    // A genre inherits its world residue through its host; it carries no `at`
 // locus, so it takes no kind-fact row. Callers filter these out before this point.
    throw new Error(`kind \`${facts.name}\` is a genre — it carries no locus-bearing kind fact.`);
 }
  return {
    name: facts.name,
    provider: facts.provider,
    governs_root: facts.locus.root,
    governs_glob: facts.locus.glob,
    format: facts.format,
    unit_shape: facts.unitShape,
    registration: registrationLabel(facts.registration),
    templates: templatesFor(facts.name, allKinds),
  };
}

/** Every kind in play, at any locus — member kinds ∪ expect kinds ∪ their genres. */
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
    .sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
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
    .sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0))
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
  return rows.sort(
    (a, b) =>
      (a.member < b.member ? -1 : a.member > b.member ? 1 : 0) ||
      (a.requirement < b.requirement ? -1 : a.requirement > b.requirement ? 1 : 0),
 );
}

/** Compile a harness into its five declaration families — the erased program. */
export function compileDeclarations(harness: Harness): Declarations {
  const allKinds = kindsInPlay(harness);
  const kinds = atLocusKindsInPlay(allKinds);
  const clauses: ClauseRow[] = [];
  for (const binding of [...harness.expect].sort((a, b) => (a.kind.key < b.kind.key ? -1 : a.kind.key > b.kind.key ? 1 : 0))) {
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
  };
}

/** The SDK's pinned engine/interchange version — the JSON pipe rides it in lockstep. */
export const SEAM_VERSION = 2;

/**
 * Serialize the declaration rows to the internal versioned JSON pipe.
 * Not a designed IR — a stable public interchange
 * is admitted only when its consumer lands. Deterministic: insertion-ordered keys
 * and a trailing newline, so a re-emit is byte-identical.
 */
export function declarationsToJson(declarations: Declarations): string {
  return JSON.stringify({ version: SEAM_VERSION, ...declarations }, null, 2) + "\n";
}
