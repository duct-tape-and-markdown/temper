/**
 * Contracts — clauses and requirements as typed values.
 * A clause is `predicate · severity · guidance · cite`; a requirement is
 * `prose · kind · required · clauses? · verifiedBy?`. Both erase to compiled data
 * at the seam: the author composes typed objects, the engine
 * consumes their rows. The predicate vocabulary is the closed algebra — a clause
 * outside it is a squiggle, not a runtime rejection.
 */

import type { KindDefinition } from "./kind.js";

/** A clause's delivery posture: `required` gate-blocks, `advisory` reports. */
export type Severity = "required" | "advisory";

/**
 * A member of the closed predicate algebra. Both a kind's own `expect` clauses and a
 * requirement's `clauses` compile to the row's full `key`/`field`/`severity`/argument
 * shape (`declarations.ts` `clauseRow`) — one spelling, whichever selection the clause
 * binds to: `bound`/`charset`/`keys`/`values` ride the row alongside
 * `count`/`target`/`degree`, so the lock encodes the clause losslessly rather than
 * identity+severity alone.
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
   * checked field) since `membership` names both.
   */
  readonly target?: string;
  /** `allowed_chars`'s declared character class. */
  readonly charset?: Charset;
  /** `forbidden_keys`'s forbidden key list. */
  readonly keys?: readonly string[];
  /** `enum`/`deny`'s permitted or forbidden value list. */
  readonly values?: readonly string[];
  /** `range`'s inclusive numeric bound. */
  readonly range?: { readonly min: number; readonly max: number };
  /** `section_contains`'s heading-text prefix and the marker each governed section must carry. */
  readonly section?: { readonly heading: string; readonly marker: string };
}

/**
 * The character class `allowed_chars` admits — inclusive ranges plus individual
 * characters, e.g. `[a-z0-9-]`. Each range is a `"<lo>-<hi>"` two-character
 * span (`src/contract.rs` `parse_range`'s wire spelling).
 */
export interface Charset {
  readonly ranges?: readonly string[];
  readonly chars?: string;
}

// Node-scope predicates.
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
/** Names are unique within the artifact kind (a scope-wide identity collision). */
export const uniqueName = (): Predicate => ({ key: "unique-name" });
/**
 * The named field may be present — always satisfied, recording the key as part of a
 * declared (closed) schema. `dependency-exists` has no constructor: the engine holds
 * it back absent a decidable reference syntax, so a hand-authored clause would fail
 * admissibility.
 */
export const optional = (field: string): Predicate => ({ key: "optional", field });
/** The field's numeric value lies within the inclusive `[min, max]` bound. */
export const range = (field: string, min: number, max: number): Predicate => ({
  key: "range",
  field,
  range: { min, max },
});
/** The field's value is one of `values`. Spelled `enumOf` — `enum` is a reserved word. */
export const enumOf = (field: string, values: readonly string[]): Predicate => ({
  key: "enum",
  field,
  values,
});
/** The named body marker is defined (e.g. `disable-model-invocation`). */
export const mustDefine = (marker: string): Predicate => ({ key: "must_define", field: marker });
/** Every body section whose heading *starts with* `heading` carries `marker` in its body. */
export const sectionContains = (heading: string, marker: string): Predicate => ({
  key: "section_contains",
  section: { heading, marker },
});
/** Every glob the field carries parses under globset (brace-expansion aware). */
export const globValid = (field: string): Predicate => ({ key: "glob-valid", field });
/**
 * Every edge the member's kind declares is placed by the format that renders the member
 * — a format that omits one renders a contract the prose does not represent. Names no
 * field: the selection is the member's whole incident edge set, at the `each` grain.
 * Decided over the placement `emit` observes while rendering and lowers into the
 * member's `nested_member` row, since the engine never sees a format.
 */
export const formatPlacesEdges = (): Predicate => ({ key: "format-places-edges" });

// Set predicates — they range over the **selection** their clause binds to, not one
// member's own fields: a kind's `expect` binds them to that kind's whole population
// (the universal binding), a requirement's `clauses` to its opt-in satisfiers (the
// existential one). The quantifier is the clause's grain, so the same value says the
// same thing in either place, riding the same four-channel `clause()` shape as the
// member-scope predicates above.
/** The selection's size lies in the inclusive `[min, max]` bound. */
export const count = (bounds: { min?: number; max?: number }): Predicate => {
  const args: Record<string, number> = {};
  if (bounds.min !== undefined) args.min = bounds.min;
  if (bounds.max !== undefined) args.max = bounds.max;
  return { key: "count", args };
};
/** The field's extracted value does not repeat across the selection. */
export const unique = (field: string): Predicate => ({ key: "unique", field });
/** Every selected member's `field` value is drawn from a feature over the selection `target` declares. */
export const membership = (field: string, target: string): Predicate => ({
  key: "membership",
  field,
  target,
});
/** The in/out edge-count bound every selected member must land in. At least one direction must be given. */
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
 * a maintained floor auditable.
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
 * A requirement — a named obligation on the harness. `prose` is the authored
 * intent, carried never interpreted; `kind` constrains what may fill it —
 * either a bare kind-name string or the kind's `KindDefinition`, since the slot
 * carries only the kind's *identity* for coverage resolution, never its field
 * type: a kind whose fields carry required members (skill, hook) assigns here,
 * where `KindDefinition<never>` would have rejected it. `required` is the
 * posture declaration; `clauses` bind to the requirement's **opt-in selection** —
 * ordinary [`Clause`] values, the same four-channel clause as everywhere, judged
 * over that selection by the same algebra a kind's own clauses are judged over its
 * population; `verifiedBy` wires the behavioral remainder.
 */
export interface Requirement {
  readonly prose: string;
  readonly kind?: string | KindDefinition<any>;
  readonly required?: boolean;
  readonly clauses?: readonly Clause[];
  readonly verifiedBy?: string;
}

/** An identity helper — types a requirement literal at the keystroke. */
export function requirement(init: Requirement): Requirement {
  return init;
}
