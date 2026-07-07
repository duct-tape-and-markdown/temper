/**
 * Contracts — clauses and requirements as typed values (`specs/architecture/10-contracts.md`).
 * A clause is `predicate · severity · guidance · cite`; a requirement is
 * `means · kind · required · clauses? · verifiedBy?`. Both erase to compiled data
 * at the seam (`20-surface.md`): the author composes typed objects, the engine
 * consumes their rows. The predicate vocabulary is the closed algebra — a clause
 * outside it is a squiggle, not a runtime rejection (`10-contracts.md`, the two
 * walls).
 */

import type { KindDefinition } from "./kind.js";

/** A clause's delivery posture: `required` gate-blocks, `advisory` reports. */
export type Severity = "required" | "advisory";

/**
 * A member of the closed predicate algebra (`10-contracts.md`). A kind's own
 * `expect` clauses compile to a reduced row of `key`/`field`/`severity` only —
 * the node-scope engine reads a floor's per-clause severity overrides, never a
 * predicate's own bounds. A requirement's `clauses` compile fuller: `args`/
 * `target` ride the row too, since the roster/graph checks decide `count`/
 * `unique`/`membership`/`degree` from them directly (`declarations.ts`
 * `clauseRow`).
 */
export interface Predicate {
  /** The predicate's clause key (`required`, `max_len`, `max_lines`, …). */
  readonly key: string;
  /** The field (or marker) the predicate constrains, when it names one. */
  readonly field?: string;
  /** The predicate's scalar bounds, keyed per predicate (`min`/`max`,
   * `incoming_min`/`incoming_max`/`outgoing_min`/`outgoing_max`). */
  readonly args?: Readonly<Record<string, number>>;
  /**
   * `membership`'s target requirement name — a separate slot from `field` (the
   * checked field) since `membership` names both (`10-contracts.md`, "Judged
   * at the node-set scope").
   */
  readonly target?: string;
  /** `allowed_chars`'s declared character class (`10-contracts.md`, "allowed_chars"). */
  readonly charset?: Charset;
  /** `forbidden_keys`'s forbidden key list. */
  readonly keys?: readonly string[];
  /** `enum`/`deny`'s permitted or forbidden value list. */
  readonly values?: readonly string[];
}

/**
 * The character class `allowed_chars` admits — inclusive ranges plus individual
 * characters, e.g. `[a-z0-9-]` (`10-contracts.md`, "Decision: `allowed_chars`,
 * not a general `pattern` clause"). Each range is a `"<lo>-<hi>"` two-character
 * span (`src/contract.rs` `parse_range`'s wire spelling).
 */
export interface Charset {
  readonly ranges?: readonly string[];
  readonly chars?: string;
}

// Node-scope predicates (`10-contracts.md`, "The predicate algebra").
/** A field or marker is present. */
export const required = (field: string): Predicate => ({ key: "required", field });
/** The field's parsed scalar type is as declared. */
export const type = (field: string): Predicate => ({ key: "type", field });
/** The field's value is at least `n` characters. */
export const minLen = (field: string, n: number): Predicate => ({ key: "min_len", field, args: { min: n } });
/** The field's value is at most `n` characters. */
export const maxLen = (field: string, n: number): Predicate => ({ key: "max_len", field, args: { max: n } });
/** The field's characters are drawn from a declared class (`allowed_chars`). */
export const allowedChars = (field: string, charset: Charset): Predicate => ({
  key: "allowed_chars",
  field,
  charset,
});
/** The member's body is at most `n` lines. */
export const maxLines = (n: number): Predicate => ({ key: "max_lines", args: { max: n } });
/** The forbidden keys (e.g. the Cursor `globs`/`alwaysApply` keys) are absent. */
export const forbiddenKeys = (keys: readonly string[]): Predicate => ({ key: "forbidden_keys", keys });
/** The field's value is none of `values` (forbidden values). */
export const deny = (field: string, values: readonly string[]): Predicate => ({
  key: "deny",
  field,
  values,
});
/** The named headings are present. */
export const requireSections = (): Predicate => ({ key: "require_sections" });
/** The member's name matches its directory. */
export const nameMatchesDir = (): Predicate => ({ key: "name-matches-dir" });

// Node-set/edge-scope predicates (`10-contracts.md`, "Judged at the node-set
// scope" / "Judged at the edge scope") — a requirement's set-scope demands ride
// these as ordinary clause values, the same four-channel `clause()` shape as
// the node-scope predicates above.
/** The satisfier set's size lies in the inclusive `[min, max]` bound. */
export const count = (bounds: { min?: number; max?: number }): Predicate => {
  const args: Record<string, number> = {};
  if (bounds.min !== undefined) args.min = bounds.min;
  if (bounds.max !== undefined) args.max = bounds.max;
  return { key: "count", args };
};
/** The field's extracted value does not repeat across the satisfier set. */
export const unique = (field: string): Predicate => ({ key: "unique", field });
/** Every satisfier's `field` value is drawn from a feature over `target`'s own satisfier set. */
export const membership = (field: string, target: string): Predicate => ({
  key: "membership",
  field,
  target,
});
/** The in/out edge-count bound every satisfier must land in. At least one direction must be given. */
export const degree = (bounds: {
  incoming?: { min?: number; max?: number };
  outgoing?: { min?: number; max?: number };
}): Predicate => {
  const args: Record<string, number> = {};
  if (bounds.incoming?.min !== undefined) args.incoming_min = bounds.incoming.min;
  if (bounds.incoming?.max !== undefined) args.incoming_max = bounds.incoming.max;
  if (bounds.outgoing?.min !== undefined) args.outgoing_min = bounds.outgoing.min;
  if (bounds.outgoing?.max !== undefined) args.outgoing_max = bounds.outgoing.max;
  return { key: "degree", args };
};

/**
 * A clause — a predicate the author marks with a severity, the just-in-time
 * guidance the predicate cannot encode, and the external-fact `cite` that makes
 * a maintained floor auditable (`10-contracts.md`, "The clause").
 */
export interface Clause {
  readonly predicate: Predicate;
  readonly severity: Severity;
  readonly guidance?: string;
  readonly cite?: string;
}

/** Compose a clause value — a predicate under a declared severity, with optional guidance/cite. */
export function clause(
  predicate: Predicate,
  opts: { severity: Severity; guidance?: string; cite?: string },
): Clause {
  return { predicate, severity: opts.severity, guidance: opts.guidance, cite: opts.cite };
}

/**
 * A requirement — a named obligation on the harness (`10-contracts.md`,
 * "Requirements"). `means` is the authored intent, carried never interpreted;
 * `kind` constrains what may fill it **by import** (a value, never a string);
 * `required` is the posture declaration; `clauses` are the requirement's own
 * set-/edge-scope demands — ordinary [`Clause`] values whose predicates range
 * over the satisfier set (`count`/`unique`/`membership`) or its graph
 * neighborhood (`degree`), the same four-channel clause as everywhere
 * (`10-contracts.md`, "Decision: set-scope demands are clauses"); `verifiedBy`
 * wires the behavioral remainder.
 */
export interface Requirement {
  readonly means: string;
  readonly kind?: KindDefinition<object>;
  readonly required?: boolean;
  readonly clauses?: readonly Clause[];
  readonly verifiedBy?: string;
}

/** An identity helper — types a requirement literal at the keystroke (`40-composition.md`). */
export function requirement(init: Requirement): Requirement {
  return init;
}
