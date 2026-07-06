/**
 * The assembly — `harness()` takes the whole as one typed value
 * (`specs/architecture/40-composition.md`): `Harness = members · expect · require ·
 * settings`. Like every SDK type it erases at the seam — the
 * engine never sees the constructor, only the declaration rows it compiles to
 * (`20-surface.md`). There is no second authoring surface: no `temper.toml`, no
 * roster/bindings dialect (the Decision rejects both); composing partial
 * harnesses is ordinary code.
 */

import type { Member, KindDefinition } from "./kind.js";
import type { Clause, Requirement } from "./contract.js";

/**
 * One `expect` binding — universal: every member of `kind` owes these clauses
 * (`40-composition.md`). Keyed by the kind **value**, an import never a string;
 * binding is implicit — a floor is just a clause array spread into `clauses`.
 */
export interface ExpectBinding {
  readonly kind: KindDefinition<object>;
  readonly clauses: readonly Clause[];
}

/** The composed harness — the four fields, erased to rows at the seam. */
export interface Harness {
  /** The member roster — the assembly's imports (`40-composition.md`, "`members`"). */
  readonly members: readonly Member[];
  /** Universal clause bindings, keyed by kind value. */
  readonly expect: readonly ExpectBinding[];
  /** Existential obligations the harness must contain a fill for, keyed by name. */
  readonly require: Readonly<Record<string, Requirement>>;
  /** The residual harness-level settings with no member home (a shrinking list). */
  readonly settings: Readonly<Record<string, unknown>>;
}

/**
 * Compose the harness from its four fields — ordinary code, Turing-completeness
 * quarantined at authoring time (`00-intent.md`, the SDK Decision). Absent
 * fields default empty; the member list is the only required part.
 */
export function harness(init: {
  members: readonly Member[];
  expect?: readonly ExpectBinding[];
  require?: Readonly<Record<string, Requirement>>;
  settings?: Readonly<Record<string, unknown>>;
}): Harness {
  return {
    members: init.members,
    expect: init.expect ?? [],
    require: init.require ?? {},
    settings: init.settings ?? {},
  };
}
