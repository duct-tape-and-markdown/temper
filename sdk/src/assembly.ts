/**
 * The assembly — `harness()` takes the whole as one typed value: `Harness =
 * members · expect · require · settings · mode`. Like every SDK type it
 * erases at the seam — the engine never sees the constructor, only the
 * declaration rows it compiles to. There is no second authoring surface: no
 * `temper.toml`, no roster/bindings dialect (the Decision rejects both);
 * composing partial harnesses is ordinary code.
 */

import type { Member, KindDefinition } from "./kind.js";
import { clause, fresh, reachable } from "./contract.js";
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
   * The **root member's own contract** — the clauses that bind to the whole governed
   * forest rather than to one kind's population. Lowered to top-level clause rows
   * carrying no `kind` column, the absence the engine reads as "the root's"
   * (`specs/model/representation.md`, "The root member": the contract attaches to the
   * harness the way it attaches to any member).
   *
   * Every clause here is judged at the selection grain, so the vocabulary is the
   * selection predicates plus {@link reachable}; a member-grain clause names a field in
   * a schema the root has no single kind to supply, and the gate refuses it at
   * admissibility rather than leaving it unjudged.
   *
   * Absent, {@link rootDefaultContract} applies — the shipped floor, composed by
   * {@link harness}.
   */
  readonly contract: readonly Clause[];
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
  contract?: readonly Clause[];
}): Harness {
  return {
    members: init.members,
    expect: init.expect ?? [],
    admit: init.admit ?? [],
    require: init.require ?? {},
    settings: init.settings ?? {},
    mode: init.mode ?? "warn",
    // The one place "a shipped root default applies when the author declares none" is
    // spelled. Composing the array is the whole override surface: an authored
    // `contract` replaces the default wholesale, never merges with it — the same
    // rows-or-default rule a kind's `expect` binding takes over its floor.
    contract: init.contract ?? rootDefaultContract,
  };
}

/**
 * The **shipped root default contract** — what the root member owes absent an authored
 * one, and what the embedded built-in lock therefore carries for the stranger gate (a
 * harness with no lock of its own gets the same root contract an emitted one does).
 *
 * Homed beside the root member's other fields, the precedent `dialDefaultContract` sets
 * in `dial.ts`: a default contract lives with the surface it governs.
 *
 * Two clauses, both predicates only the root can bind: their selection is the whole
 * forest and their judges read the reference graph and the committed lock, neither of
 * which any one kind's population carries. `reachable` is the one check that catches
 * authored configuration the harness never loads at all; `fresh` is the one that catches
 * a projection or a fingerprinted source dependency that has moved out from under its
 * lock row.
 *
 * Both advisory — today's posture for each, so no adopter turns red on the upgrade. A
 * dead registration is often deliberate work-in-progress, and a drifted projection is
 * usually a re-emit away; whether either gates is the adopting author's call, dialed or
 * re-declared rather than tool-decided.
 */
export const rootDefaultContract: readonly Clause[] = [
  clause(reachable(), {
    severity: "advisory",
    guidance:
      "This member is authored but unreachable: every registration channel its kind declares is provably dead, and no member that is reachable imports it. Claude never loads it, so it is context you maintain and never pay for — and a reader of the tree cannot tell it from live configuration. Three remedies: open a channel (give the `paths` globs a file they match, give the `description` trigger words), import it from a member that is reachable, or delete it. Advisory because a dead edge is a legitimate work-in-progress state; dial it or re-declare the clause `required` once your tree should hold the line.",
    cite:
      "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-15); https://code.claude.com/docs/en/skills (retrieved 2026-07-16)",
  }),
  clause(fresh(), {
    severity: "advisory",
    guidance:
      "A lock row this member owns no longer matches disk: either its committed projection was hand-edited, or a fingerprinted source dependency it imports or includes has moved. Nothing reverse-parses a projection back into the program, so the two sides stay apart until you reconcile them: edit the owning source and re-emit, and for a moved dependency re-verify the member's claims against the new bytes first. Advisory because a drifted checkout is usually one `emit` away and blocking every such run would be temper's escalation, not yours; dial this label to `required` — or re-declare the clause — once a drifted projection should fail CI.",
  }),
];
