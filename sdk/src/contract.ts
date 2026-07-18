/**
 * Contracts — clauses and requirements as typed values.
 * A clause is `predicate · severity · guidance · cite`; a requirement is
 * `prose · kind · required · clauses? · verifier?`. Both erase to compiled data
 * at the seam: the author composes typed objects, the engine
 * consumes their rows. The predicate vocabulary is the closed algebra — a clause
 * outside it is a squiggle, not a runtime rejection.
 */

import type { Shape, ValueType } from "./generated/index.js";
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
  /** The predicate's clause key (`required`, `max_len`, `extent`, …). */
  readonly key: string;
  /**
   * The field (or marker) the predicate constrains, when it names one.
   *
   * A value predicate's `field` is an **addressing path**: name segments walk into an
   * object (`owner.name`), and `[*]` is the each-grain over an array's elements, so
   * `plugins[*].source` decides once per entry and indicts each offending one by its own
   * address. Nothing else is spellable — an index, a slice, a filter, and a recursive
   * descent are all refused when the contract is checked, not silently evaluated. The
   * subset is the whole surface on purpose: a clause names *where* a value lives, never
   * a pattern that matches it.
   *
   * `forbidden_keys` and `must_define` name a top-level **key**, not a path, and
   * `closed-keys` names neither — it reads the key set its sibling clauses declare.
   */
  readonly field?: string;
  /** The predicate's scalar bounds, keyed per predicate (`min`/`max`,
   * `incoming_min`/`incoming_max`/`outgoing_min`/`outgoing_max`). */
  readonly args?: Readonly<Record<string, number>>;
  /**
   * `membership`'s target requirement name — a separate slot from `field` (the
   * checked field) since `membership` names both.
   */
  readonly target?: string;
  /**
   * `mention-reachable`'s **target-side gate field** — a separate slot from `field`
   * (which carries the source-side scope field) since it is the one predicate naming a
   * field on *both* ends. Spelled to match the lock's own `gate` column.
   */
  readonly gate?: string;
  /**
   * `type`'s declared source kinds over the closed scalar/container lattice — the set
   * the field may carry any one of. Spelled to match the lock's own `value_type`
   * column.
   */
  readonly value_type?: readonly ValueType[];
  /**
   * `shape`'s declared shape — one member of the closed set, spelled to match the lock's
   * own `shape` column.
   */
  readonly shape?: Shape;
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
  /** `require_sections`'s required heading list. */
  readonly sections?: readonly string[];
  /** `extent`'s declared unit — the render-side size proxy the bound is measured in. */
  readonly unit?: ExtentUnit;
}

/** The unit an `extent` bound is measured in — the closed set of stable render-side size
 * proxies. Token count is deliberately absent: a verdict that moves when a tokenizer
 * updates is a gate that changes its mind with no diff. */
export type ExtentUnit = "lines" | "characters";

/**
 * The character class `allowed_chars` admits — inclusive ranges plus individual
 * characters, e.g. `[a-z0-9-]`. Each range is a `"<lo>-<hi>"` two-character
 * span (`src/contract.rs` `parse_range`'s wire spelling).
 */
export interface Charset {
  readonly ranges?: readonly string[];
  readonly chars?: string;
}

// Node-scope predicates. A value predicate takes an addressing path as its `field` — see
// `Predicate.field` for the subset an author may spell, and for the key-naming and
// fieldless exceptions that sit here too.
/**
 * A field or marker is present. Presence is asked of the path's trailing name segment,
 * so `required("plugins[*].source")` fires once per entry that omits it — and a path
 * ending in `[*]` names elements rather than a key, which is refused.
 */
export const required = (field: string): Predicate => ({ key: "required", field });
/**
 * The field's parsed source kind is one of the declared ones. `kinds` is a set: a
 * format that documents a field as `string|array` is gated by `["string", "list"]`,
 * and a one-element `["list"]` is the plain single-kind check.
 */
export const type = (field: string, kinds: readonly ValueType[]): Predicate => ({
  key: "type",
  field,
  value_type: kinds,
});
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
/**
 * The field's value holds one named `Shape` from the closed set the engine generates
 * across the seam — the union *is* the vocabulary, so an author picks a member and can
 * spell nothing else. There is no escape to a hand-written pattern, which is the point.
 *
 * - `hyphen-placement` — a hyphen neither leads, trails, nor doubles (`pdf-processing`,
 *   never `-pdf`, `pdf-`, or `pdf--processing`). Says nothing about the alphabet; pair it
 *   with `allowedChars` where the character set matters.
 * - `no-xml-tags` — the value carries no XML tag (`<br/>`, `</note>`, `<a href="x">`).
 *   Prose spelling a comparison (`use when x < y`) is not a tag and does not fire.
 */
export const shape = (field: string, shape: Shape): Predicate => ({ key: "shape", field, shape });
/**
 * The selected item's **rendered** extent is at most `bound`, in the declared `unit`
 * (`lines` or `characters`). Node-scope and render-side: the measure is the bytes the
 * item contributes to its projection, not the source body a count reads before a
 * reference resolves or a render hook runs.
 */
export const extent = (unit: ExtentUnit, bound: number): Predicate => ({
  key: "extent",
  unit,
  args: { max: bound },
});
/** The forbidden keys (e.g. the Cursor `globs`/`alwaysApply` keys) are absent. */
export const forbiddenKeys = (keys: readonly string[]): Predicate => ({ key: "forbidden_keys", keys });
/**
 * The kind's declared key set is exhaustive — a member carrying any other top-level key
 * is a finding. `forbiddenKeys`' complement: a deny-list names a finite set over an open
 * key space, this closes the space.
 *
 * It takes no arguments, and that is the point: the allow-list is the contract's own
 * `required`/`optional` clauses, so the key set is declared once. Adding `optional("x")`
 * admits `x` with no second edit here — and a contract declaring no key at all fails
 * admissibility rather than indicting every key of every member.
 */
export const closedKeys = (): Predicate => ({ key: "closed-keys" });
/** The field's value is none of `values` (forbidden values). */
export const deny = (field: string, values: readonly string[]): Predicate => ({
  key: "deny",
  field,
  values,
});
/** The named headings are present. */
export const requireSections = (sections: readonly string[]): Predicate => ({ key: "require_sections", sections });
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
 * Every mention a selected member authors can fire where its target can be invoked. A
 * target whose `gateField` carries globs is gated — removed from every invocation
 * channel until the agent reads a matching file — so a mention of it is actionable only
 * inside that gate. Fires on a scoped source whose `scopeField` globs are not contained
 * in the target's gate, and on an unscoped source mentioning a gated target.
 *
 * Generic over both ends, hard-coding no kind: the trigger is the target's gate field
 * carrying a value, never its registration set. Containment is *literal* — every source
 * glob must appear verbatim in the gate — since true glob-set containment is
 * undecidable, so it false-fires on a semantically contained narrower glob
 * (`src/**\/*.ts` inside `src/**`). Declare it at advisory severity: a check that can be
 * wrong must not block.
 */
export const mentionReachable = (scopeField: string, gateField: string): Predicate => ({
  key: "mention-reachable",
  field: scopeField,
  gate: gateField,
});
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
 * A guarded clause — a predicate (restricted to `type` or `enum`) that acts as a
 * guard, with a body of ordinary clauses that fire only where the guard holds.
 * The guard and body share the guard's address binding: the body's field paths
 * evaluate at the concrete element address the guard locates.
 *
 * The guard must be a `type` or `enumOf` predicate, named for the decidable
 * field it addresses. Neither is enforced here; emit validates admissibility.
 */
export function when(guard: Predicate, body: readonly Clause[]): Clause {
  return {
    predicate: { key: "when" },
    severity: "required",
    when_guard: guard,
    when_body: body,
  };
}

/**
 * A clause — a predicate the author marks with a severity, the just-in-time
 * guidance the predicate cannot encode, and the external-fact `cite` that makes
 * a maintained floor auditable. A `when` clause carries an additional guard
 * predicate and a body of nested clauses that fire only where the guard holds.
 */
export interface Clause {
  readonly predicate: Predicate;
  readonly severity: Severity;
  readonly guidance?: string;
  readonly cite?: string;
  /** The guard predicate for a `when` clause; absent for all others. */
  readonly when_guard?: Predicate;
  /** The nested body clauses for a `when` clause; absent for all others. */
  readonly when_body?: readonly Clause[];
}

/** Compose a clause value — a predicate under a declared severity, with optional guidance/cite. */
export function clause(
  predicate: Predicate,
  opts: { severity: Severity; guidance?: string; cite?: string },
): Clause {
  return { predicate, severity: opts.severity, guidance: opts.guidance, cite: opts.cite };
}

/**
 * A requirement's **typed verifier** — the declared delegate that judges the
 * behavioral remainder, a species-tagged union the gate resolves at admissibility
 * and never runs. Two species this slice: a `script` (path-resolved) and a
 * `telemetry` declaration (named documented harness events). A probe stays a
 * documented pattern until a consumer types it — no `probe()` constructor is minted.
 */
export type Verifier =
  | { readonly species: "script"; readonly path: string }
  | { readonly species: "telemetry"; readonly events: readonly string[] };

/**
 * A script verifier — a path-resolved reference to the test or CI job that executes
 * the behavioral judgment. The gate checks the `path` resolves, never runs it.
 */
export const script = (path: string): Verifier => ({ species: "script", path });

/**
 * A telemetry verifier — the named documented harness events the emitted tap records
 * to a local-locus log, judged by reading the field record. Each name must be a
 * documented harness lifecycle event (`InstructionsLoaded`, `Skill`,
 * `UserPromptExpansion`, `PostToolUse`; code.claude.com/docs/en/hooks, retrieved
 * 2026-07-17) — the gate checks each resolves, never records into it.
 */
export const telemetry = (events: readonly string[]): Verifier => ({ species: "telemetry", events });

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
 * population; `verifier` wires the behavioral remainder as a typed species union.
 */
export interface Requirement {
  readonly prose: string;
  readonly kind?: string | KindDefinition<any>;
  readonly required?: boolean;
  readonly clauses?: readonly Clause[];
  readonly verifier?: Verifier;
}

/** An identity helper — types a requirement literal at the keystroke. */
export function requirement(init: Requirement): Requirement {
  return init;
}
