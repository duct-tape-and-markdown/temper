/**
 * Contracts — clauses and requirements as typed values (`specs/architecture/10-contracts.md`).
 * A clause is `predicate · severity · guidance · cite`; a requirement is
 * `means · kind · required · count? · unique? · membership? · degree? · verifiedBy?`.
 * Both erase to compiled data at the seam (`20-surface.md`): the author composes
 * typed objects, the engine consumes their rows. The predicate vocabulary is the
 * closed algebra — a clause outside it is a squiggle, not a runtime rejection
 * (`10-contracts.md`, the two walls).
 */

import type { KindDefinition } from "./kind.js";

/** A clause's delivery posture: `required` gate-blocks, `advisory` reports. */
export type Severity = "required" | "advisory";

/**
 * A member of the closed predicate algebra (`10-contracts.md`). The compiled
 * clause row records only `key`, the targeted `field` (when it names one), and
 * severity — the reduced shape the lock and the JSON pipe both carry; a
 * predicate's scalar bounds (`args`) stay author-side until the fuller
 * interchange lands its consumer (the entry gate, `20-surface.md`).
 */
export interface Predicate {
  /** The predicate's clause key (`required`, `max_len`, `max_lines`, …). */
  readonly key: string;
  /** The field (or marker) the predicate constrains, when it names one. */
  readonly field?: string;
  /** The predicate's scalar bounds — author-side; not yet in the erased row. */
  readonly args?: Readonly<Record<string, number>>;
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
export const allowedChars = (field: string): Predicate => ({ key: "allowed_chars", field });
/** The member's body is at most `n` lines. */
export const maxLines = (n: number): Predicate => ({ key: "max_lines", args: { max: n } });
/** The forbidden keys (e.g. the Cursor `globs`/`alwaysApply` keys) are absent. */
export const forbiddenKeys = (): Predicate => ({ key: "forbidden_keys" });
/** The named headings are present. */
export const requireSections = (): Predicate => ({ key: "require_sections" });
/** The member's name matches its directory. */
export const nameMatchesDir = (): Predicate => ({ key: "name-matches-dir" });

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
 * `required` is the posture declaration; the set-scope facets measure the
 * satisfier set; `verifiedBy` wires the behavioral remainder.
 */
export interface Requirement {
  readonly means: string;
  readonly kind?: KindDefinition<object>;
  readonly required?: boolean;
  readonly count?: { readonly min?: number; readonly max?: number };
  readonly unique?: string;
  readonly membership?: string;
  readonly degree?: string;
  readonly verifiedBy?: string;
}

/** An identity helper — types a requirement literal at the keystroke (`40-composition.md`). */
export function requirement(init: Requirement): Requirement {
  return init;
}
