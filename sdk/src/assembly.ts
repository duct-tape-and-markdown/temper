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
 * The declared enforcement-mode vocabulary — how firmly the `PreToolUse` guard
 * binds a tool call, split by where the finding goes.
 * `block`: denies the call.
 * `warn` (default): allows the call and surfaces the finding in-band, into the
 * live context. `note`: allows the call and records the finding out-of-band
 * only — the next report, never the session.
 */
export type EnforcementMode = "note" | "warn" | "block";

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
 * per member. Defaults to `warn`: temper fabricates no enforcement the
   * author did not declare.
 */
  readonly mode: EnforcementMode;
}

/**
 * Compose the harness from its five fields — ordinary code, Turing-completeness
 * quarantined at authoring time. Absent
 * fields default empty (`mode` defaults `warn`); the member list is the
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
    mode: init.mode ?? "warn",
  };
}
