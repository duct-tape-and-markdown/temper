/**
 * Declaration rows — the composed program's erased declarations
 * (`specs/architecture/20-surface.md`, "The seam — one implementation"; "The lock
 * and drift"). Every type erases at the seam: kinds, clauses, requirements, and
 * assembly facts compile to plain rows the engine reads. The **row shape** matches
 * the Rust lock's `[declaration]` families (`src/drift.rs` `Declarations`) — the
 * byte-parity lockstep two writers keep until single-writer lands
 * (`SDK-RECUT-CORPUS-FACE`). The same rows ride the internal versioned JSON pipe
 * ({@link declarationsToJson}) — not a designed IR, versioned in lockstep.
 */

import type { Harness } from "./assembly.js";
import type { KindFacts, Registration } from "./kind.js";
import type { Requirement } from "./contract.js";

/** One kind's declaration row — its identity and declared runtime facts. */
export interface KindFactRow {
  readonly name: string;
  readonly provider?: string;
  readonly governs_root: string;
  readonly governs_glob: string;
  readonly format?: string;
  readonly unit_shape?: string;
  readonly activation?: string;
}

/** One clause of a kind's effective contract, reduced to the lock's columns. */
export interface ClauseRow {
  readonly kind: string;
  readonly predicate: string;
  readonly field?: string;
  readonly severity: string;
}

/** One named requirement's declaration row, reduced to the scalar facets the lock records. */
export interface RequirementRow {
  readonly name: string;
  readonly kind?: string;
  readonly required: boolean;
  readonly verified_by?: string;
}

/** One assembly-scope fact — authority, reachability, or an edge. */
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

/** The lock label for a kind's declared registration — the activation spelling. */
function activationLabel(registration: Registration): string {
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

/** One kind's fact row — the `at` locus supplies `governs_root`/`governs_glob`. */
function kindFactRow(facts: KindFacts): KindFactRow {
  if (facts.locus.kind !== "at") {
    // A genre inherits its world residue through its host; it carries no `at`
    // locus, so it takes no kind-fact row (`15-kinds.md`). Callers filter these
    // out before this point.
    throw new Error(`kind \`${facts.name}\` is a genre — it carries no locus-bearing kind fact.`);
  }
  return {
    name: facts.name,
    provider: facts.provider,
    governs_root: facts.locus.root,
    governs_glob: facts.locus.glob,
    format: facts.format,
    unit_shape: facts.unitShape,
    activation: activationLabel(facts.registration),
  };
}

/** The distinct locus-bearing kinds in play — member kinds ∪ expect kinds, name-sorted. */
function kindsInPlay(harness: Harness): KindFacts[] {
  const byName = new Map<string, KindFacts>();
  for (const member of harness.members) byName.set(member.facts.name, member.facts);
  for (const binding of harness.expect) byName.set(binding.kind.facts.name, binding.kind.facts);
  return [...byName.values()]
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
      // admissibility finding, never a shadowing rule (`10-contracts.md`).
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
      verified_by: requirement.verifiedBy,
    }));
}

/**
 * The assembly-scope facts, in a stable order: authority (always declared — the
 * `shared` default anchors every harness until a surface-authority posture is
 * authored), reachability when the dial is set, then one edge row per kind edge
 * field (`40-composition.md`; `45-governance.md`).
 */
function assemblyFactRows(harness: Harness, kinds: readonly KindFacts[]): AssemblyFactRow[] {
  const facts: AssemblyFactRow[] = [{ fact: "authority", value: "shared" }];
  if (harness.reachability !== undefined) {
    facts.push({ fact: "reachability", value: harness.reachability });
  }
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
  const kinds = kindsInPlay(harness);
  const clauses: ClauseRow[] = [];
  for (const binding of [...harness.expect].sort((a, b) => (a.kind.key < b.kind.key ? -1 : a.kind.key > b.kind.key ? 1 : 0))) {
    for (const clause of binding.clauses) {
      clauses.push({
        kind: binding.kind.key,
        predicate: clause.predicate.key,
        field: clause.predicate.field,
        severity: clause.severity,
      });
    }
  }
  return {
    kinds: kinds.map(kindFactRow),
    clauses,
    requirements: requirementRows(harness),
    assembly: assemblyFactRows(harness, kinds),
    satisfies: satisfiesRows(harness),
  };
}

/** The SDK's pinned engine/interchange version — the JSON pipe rides it in lockstep. */
export const SEAM_VERSION = 1;

/**
 * Serialize the declaration rows to the internal versioned JSON pipe
 * (`20-surface.md`, "The seam"). Not a designed IR — a stable public interchange
 * is admitted only when its consumer lands. Deterministic: insertion-ordered keys
 * and a trailing newline, so a re-emit is byte-identical (law 5).
 */
export function declarationsToJson(declarations: Declarations): string {
  return JSON.stringify({ version: SEAM_VERSION, ...declarations }, null, 2) + "\n";
}
