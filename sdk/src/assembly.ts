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
  readonly kind: KindDefinition<never>;
  readonly clauses: readonly Clause[];
}

/**
 * One `admit` declaration — the adopting corpus naming, for one `host` kind, the
 * embedded kinds its composed body admits. An embedded kind declares no host, so
 * admission is the corpus's call, not the child's: a shipped kind's composed body
 * admits corpus-declared types by this declaration alone. Keyed by kind **value**
 * the way {@link ExpectBinding} keys `expect`; absent, a host admits nothing.
 */
export interface Admission {
  readonly host: KindDefinition<never>;
  readonly admits: readonly KindDefinition<never>[];
}

/**
 * The declared enforcement-mode vocabulary — how firmly a guard binds an
 * intercepted action, split by where the finding goes.
 * `block`: denies the action.
 * `warn` (default): allows the action and surfaces the finding in-band, into the
 * live context. `note`: allows the action and records the finding out-of-band
 * only — the next report, never the session.
 */
export type EnforcementMode = "note" | "warn" | "block";

/** The composed harness — the root member's own fields, erased to rows at the seam. */
export interface Harness {
  /** The member roster — the assembly's imports. */
  readonly members: readonly Member[];
  /** Universal clause bindings, keyed by kind value. */
  readonly expect: readonly ExpectBinding[];
  /** The embedded kinds each host kind's composed body admits, keyed by kind value. */
  readonly admit: readonly Admission[];
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
 * Compose the harness from its six fields — ordinary code, Turing-completeness
 * quarantined at authoring time. Absent
 * fields default empty (`mode` defaults `warn`); the member list is the
 * only required part.
 */
export function harness(init: {
  members: readonly Member[];
  expect?: readonly ExpectBinding[];
  admit?: readonly Admission[];
  require?: Readonly<Record<string, Requirement>>;
  settings?: Readonly<Record<string, unknown>>;
  mode?: EnforcementMode;
}): Harness {
  return {
    members: init.members,
    expect: init.expect ?? [],
    admit: init.admit ?? [],
    require: init.require ?? {},
    settings: init.settings ?? {},
    mode: init.mode ?? "warn",
  };
}
