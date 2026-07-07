/**
 * The assembly — `harness()` takes the whole as one typed value: `Harness =
 * members · expect · require · settings · mode`. Like every SDK type it
 * erases at the seam — the engine never sees the constructor, only the
 * declaration rows it compiles to. There is no second authoring surface: no
 * `temper.toml`, no roster/bindings dialect (the Decision rejects both);
 * composing partial harnesses is ordinary code.
 */

import type { Member, KindDefinition } from "./kind.js";
import type { Clause, Requirement } from "./contract.js";

/**
 * One `expect` binding — universal: every member of `kind` owes these clauses.
 * Keyed by the kind **value**, an import never a string; binding is implicit
 * — a floor is just a clause array spread into `clauses`.
 */
export interface ExpectBinding {
  readonly kind: KindDefinition<object>;
  readonly clauses: readonly Clause[];
}

/**
 * The declared enforcement-mode vocabulary — how firmly the harness owns its
 * projections (`specs/decisions/0005-mode-on-root-member.md`). `shared`
 * (default): direct on-disk edits stay first-class, a hand edit surfaces as
 * drift, guards inform and route. `surface`: the author opts into
 * enforcement — the guard hook's write-boundary block.
 */
export type EnforcementMode = "shared" | "surface";

/** The composed harness — the root member's own fields, erased to rows at the seam. */
export interface Harness {
  /** The member roster — the assembly's imports. */
  readonly members: readonly Member[];
  /** Universal clause bindings, keyed by kind value. */
  readonly expect: readonly ExpectBinding[];
  /** Existential obligations the harness must contain a fill for, keyed by name. */
  readonly require: Readonly<Record<string, Requirement>>;
  /** The residual harness-level settings with no member home (a shrinking list). */
  readonly settings: Readonly<Record<string, unknown>>;
  /**
   * The root member's declared enforcement mode — harness-wide, overridable
   * per member (deferred, `specs/decisions/0005-mode-on-root-member.md`
   * Consequences). Defaults to `shared`: temper fabricates no enforcement the
   * author did not declare.
   */
  readonly mode: EnforcementMode;
}

/**
 * Compose the harness from its five fields — ordinary code, Turing-completeness
 * quarantined at authoring time (`specs/intent.md`, the SDK Decision). Absent
 * fields default empty (`mode` defaults `shared`); the member list is the
 * only required part.
 */
export function harness(init: {
  members: readonly Member[];
  expect?: readonly ExpectBinding[];
  require?: Readonly<Record<string, Requirement>>;
  settings?: Readonly<Record<string, unknown>>;
  mode?: EnforcementMode;
}): Harness {
  return {
    members: init.members,
    expect: init.expect ?? [],
    require: init.require ?? {},
    settings: init.settings ?? {},
    mode: init.mode ?? "shared",
  };
}
