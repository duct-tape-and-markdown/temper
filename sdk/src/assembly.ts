/**
 * The assembly — `harness()` takes the whole as one typed value: `Harness =
 * members · expect · admit · require · settings · mode · contract · typecheck`. Like every SDK type it
 * erases at the seam — the engine never sees the constructor, only the
 * declaration rows it compiles to. There is no second authoring surface: no
 * `temper.toml`, no roster/bindings dialect (the Decision rejects both);
 * composing partial harnesses is ordinary code.
 */

import type { Input, Member, KindDefinition } from "./kind.js";
import { clause, engineMatches, fresh, importBacked, listing, locusDeclared, reachable, resolves } from "./contract.js";
import type { Clause, Requirement, RequirementValue } from "./contract.js";
import { command, gateHookTool, isRequirement, requirementOf, skill } from "./builtins.js";
import { memberAddress } from "./member-address.js";

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

/**
 * The program's **typecheck** — the compiler `emit` runs over the program before running
 * it, refusing on a type error with the compiler's own diagnostics. It is the consumer's
 * own toolchain, declared and never guessed: `tool` is the composed {@link tool} member
 * whose command runs the compiler (`tool({ package: "typescript", bin: "tsc" })` runs the
 * program's own `node_modules/.bin/tsc`), and `project` is the project file it checks,
 * stated as an {@link input} so a change to it marks the lock stale. Emit passes the
 * compiler `--noEmit --project <project>`, and refuses a program that declares none.
 */
export interface Typecheck {
  /** The {@link tool} member that runs the compiler — composed in `members` like any tool. */
  readonly tool: Member;
  /** The project file the compiler checks — `input(import.meta.url, "./tsconfig.json")`. */
  readonly project: Input;
}

/** The composed harness — the root member's own fields, erased to rows at the seam. */
export interface Harness {
  /** The member roster — the assembly's imports. */
  readonly members: readonly Member[];
  /** Universal clause bindings, keyed by kind value. */
  readonly expect: readonly ExpectBinding[];
  /** The embedded kinds each host kind's composed body admits, keyed by kind value. */
  readonly admit: readonly Admission[];
  /**
   * Existential obligations the harness must contain a fill for, keyed by name — every
   * requirement the assembly declares, whether through `require` or as a requirement
   * member composed in `members` (which {@link harness} draws out of the roster).
   */
  readonly require: Readonly<Record<string, Requirement>>;
  /** The residual harness-level settings with no member home (a shrinking list). */
  readonly settings: Readonly<Record<string, unknown>>;
  /** The program's declared {@link Typecheck}; absent where the program declares none, which `emit` refuses. */
  readonly typecheck?: Typecheck;
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
   * Whether the author declared {@link contract} at all — `true` even for an authored
   * empty array, `false` only where the field was absent and {@link rootDefaultContract}
   * filled it.
   *
   * The distinction {@link contract} itself cannot hold: an empty array is a declared
   * contract with no clauses, and the default applies only where nothing is declared
   * (decision 0072, `specs/builtins.md`, "Default contracts"). Row presence alone cannot
   * say it — an authored `[]` writes no clause row, exactly like silence — so
   * `assemblyFactRows` lowers this flag to a kind-less `contract` assembly fact and
   * `compose::root_contract` reads it instead of guessing from the rows.
   */
  readonly contractDeclared: boolean;
 /**
   * The root member's declared enforcement mode — harness-wide, overridable
 * per member. Defaults to `warn`: temper fabricates no enforcement the
   * author did not declare.
 */
  readonly mode: EnforcementMode;
}

/**
 * Compose the harness from its fields — ordinary code, Turing-completeness
 * quarantined at authoring time. Absent
 * fields default empty (`mode` defaults `warn`); the member list is the
 * only required part.
 */
export function harness(init: {
  members: readonly Member[];
  expect?: readonly ExpectBinding[];
  admit?: readonly Admission[];
  require?: Readonly<Record<string, RequirementValue>>;
  settings?: Readonly<Record<string, unknown>>;
  mode?: EnforcementMode;
  contract?: readonly Clause[];
  typecheck?: Typecheck;
}): Harness {
  const { members, require } = assemblyRequirements(withoutRepeatedGateHooks(init.members), init.require ?? {});
  return {
    members,
    ...(init.typecheck === undefined ? {} : { typecheck: init.typecheck }),
    expect: init.expect ?? [],
    admit: init.admit ?? [],
    require,
    settings: init.settings ?? {},
    mode: init.mode ?? "warn",
    // The one place "a shipped root default applies when the author declares none" is
    // spelled. Composing the array is the whole override surface: an authored
    // `contract` replaces the default wholesale, never merges with it — the same
    // rows-or-default rule a kind's `expect` binding takes over its floor.
    contract: init.contract ?? rootDefaultContract,
    // …and the last place the distinction the line above erases is still visible: an
    // authored `[]` is a declared contract with no clauses, so declaredness is recorded
    // rather than re-derived from a row count downstream (decision 0072).
    contractDeclared: init.contract !== undefined,
  };
}

/**
 * `roster` with each repeated gate hook dropped: a gate hook at the address of an earlier
 * one that runs the same tool and fills the same requirements is the same hook, so the
 * edge runs one command. A playbook's guard attachment and the harness's own gate hooks
 * both place the `PreToolUse` guard; any difference between the two is left to the
 * duplicate-address refusal.
 */
function withoutRepeatedGateHooks(roster: readonly Member[]): readonly Member[] {
  const seen = new Map<string, Member>();
  return roster.filter((member) => {
    const tool = gateHookTool(member);
    if (tool === undefined) return true;
    const address = memberAddress(member);
    const held = seen.get(address);
    if (held === undefined) {
      seen.set(address, member);
      return true;
    }
    const same =
      gateHookTool(held) === tool &&
      held.satisfies.length === member.satisfies.length &&
      held.satisfies.every((name, at) => member.satisfies[at] === name);
    return !same;
  });
}

/**
 * The assembly's requirements, one namespace: each `require` entry under its key, then
 * each requirement member the roster composes (a `requirement(…)` member, or a member of
 * a kind extending it) under its own name — drawn out of the roster, since a requirement
 * is what satisfies edges target, not a member the projection or the member rows carry.
 *
 * # Throws
 * If a name is declared twice, or a roster requirement member states no name.
 */
function assemblyRequirements(
  roster: readonly Member[],
  declared: Readonly<Record<string, RequirementValue>>,
): { readonly members: readonly Member[]; readonly require: Readonly<Record<string, Requirement>> } {
  const require: Record<string, Requirement> = {};
  for (const [name, value] of Object.entries(declared)) require[name] = requirementOf(name, value);
  const members: Member[] = [];
  for (const member of roster) {
    if (!isRequirement(member)) {
      members.push(member);
      continue;
    }
    if (member.name === "") {
      throw new Error(
        "harness: a requirement member in `members` names itself — pass `requirement({ name, … })`, " +
          "or declare it under its name in `require`.",
      );
    }
    if (Object.hasOwn(require, member.name)) {
      throw new Error(`harness: requirement \`${member.name}\` is declared twice — a requirement has one identity.`);
    }
    require[member.name] = requirementOf(member.name, member);
  }
  return { members, require };
}

/**
 * The skill listing's character budget, one budget over skills and commands alike: "The
 * budget scales dynamically at 1% of the context window, with a fallback of 8,000 characters" (`SLASH_COMMAND_TOOL_CHAR_BUDGET`, env-vars
 * docs). The fallback is the one character figure the docs give, and judging it keeps a
 * passing listing whole wherever the budget is not raised.
 */
const SKILL_LISTING_CHARACTER_BUDGET = 8_000;

/**
 * The per-entry cap: "the combined `description` and `when_to_use` text is truncated at
 * 1,536 characters in the skill listing" — `skillListingMaxDescChars`, default `1536`.
 */
const SKILL_LISTING_ENTRY_CAP = 1_536;

/** The skills and env-vars docs' statements of the listing budget and its per-entry cap. */
const SKILL_LISTING_BUDGET_CITE =
  "https://code.claude.com/docs/en/skills#skill-descriptions-are-cut-short; https://code.claude.com/docs/en/env-vars (SLASH_COMMAND_TOOL_CHAR_BUDGET); https://code.claude.com/docs/en/settings-reference#skilllistingmaxdescchars (retrieved 2026-11-26)";

/**
 * The **shipped root default contract** — what the root member owes absent an authored
 * one, and what the embedded built-in lock therefore carries for the stranger gate (a
 * harness with no lock of its own gets the same root contract an emitted one does).
 *
 * Homed beside the root member's other fields, the precedent `dialDefaultContract` sets
 * in `dial.ts`: a default contract lives with the surface it governs.
 *
 * Seven clauses, all predicates only the root can bind: their selection is the whole
 * forest and their judges read the reference graph, the committed lock and the discovery
 * walk, none of which any one kind's population carries. `reachable` is the one check
 * that catches authored configuration the harness never loads at all; `fresh` is the one
 * that catches a projection or a fingerprinted source dependency that has moved out from
 * under its lock row; `locus-declared` is the one that catches a document sitting at a
 * governed locus that the program never declared; `engine-matches` is the one that
 * catches a lock compiled by an engine other than the one now reading it.
 *
 * `resolves` and `import-backed`: a reference naming nothing, and an `@path` import
 * loading nothing. Both judge the reference graph, so they bind at the root too.
 *
 * `listing` closes the set: Claude Code's one skill listing carries every model-invocable
 * skill and command under one budget, so no single kind's population holds the sum —
 * two kinds each under budget can overrun it together.
 *
 * `resolves` ships at `required`: a reference naming nothing is a broken program, which
 * `emit` already refuses, and the gate holds the same line on a tree it did not write.
 * The rest are advisory — today's posture for each, so no adopter turns red on the upgrade. A
 * dead registration is often deliberate work-in-progress, a drifted projection is usually
 * a re-emit away, an undeclared document is often a surface mid-adoption, and a stale
 * engine stamp is the ordinary state of every checkout between an upgrade and the next
 * `emit`; whether any of them gates is the adopting author's call, dialed or re-declared
 * rather than tool-decided.
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
      "A lock row this member owns no longer matches disk: either its committed projection was hand-edited, or a fingerprinted source dependency has moved: a layout import, a prose include, a declared input, or — on the root member — a module the program itself loaded. Nothing reverse-parses a projection back into the program, so the two sides stay apart until you reconcile them: edit the owning source and re-emit, and for a moved declared input re-verify the member's claims against the new bytes first. Advisory because a drifted checkout is usually one `emit` away and blocking every such run would be temper's escalation, not yours; dial this label to `required` — or re-declare the clause — once a drifted projection should fail CI.",
    cite: "https://github.com/duct-tape-and-markdown/temper/blob/main/specs/model/pipeline.md#drift (retrieved 2026-11-26)",
  }),
  clause(locusDeclared(), {
    severity: "advisory",
    guidance:
      "A document sits at a governed locus that your program declares no member for. temper maintains nothing about it: `emit` never reaps it, and where your registrations write into it, `emit` renders their entries and refuses to delete any key it does not own; `guard` never bound it, and every address under it resolves to nothing — so it reads as governed configuration while being governed by no one. Where the locus is one the harness itself loads — a built-in kind's — it is live context on top of that; where the kind is your own, who reads it is yours to know and not temper's to claim. Declare the member in the program and re-emit; for a directory whose documents are authored in place rather than projected, declare its kind `local` so `check` derives the rows at read time instead. Advisory because a tree mid-adoption legitimately carries documents the program has not reached yet; dial this label to `required` — or re-declare the clause — once every document at a governed locus should be one you declared.",
    cite: "https://github.com/duct-tape-and-markdown/temper/blob/main/specs/decisions/0054-the-root-member-carries-a-contract.md (retrieved 2026-11-26)",
  }),
  clause(engineMatches(), {
    severity: "advisory",
    guidance:
      "Your committed lock records a different temper version than the one running this gate. The rows being read were compiled by a compiler this binary is not, so every verdict below rests on a translation: an older spelling normalizes at read time, but a genuine change in what a row means reads the same from the inside. Re-run `emit` — it rewrites the lock whole in this engine's canonical form, and the stamp goes back to matching. Advisory because a lagging stamp is the ordinary state of every checkout between an upgrade and the next emit, and blocking there would make an upgrade a hard stop; dial this label to `required` — or re-declare the clause — once your CI should refuse a lock it did not compile.",
    cite: "https://github.com/duct-tape-and-markdown/temper/blob/main/specs/model/pipeline.md#the-lock (retrieved 2026-11-26)",
  }),
  clause(resolves(), {
    severity: "required",
    guidance:
      "A reference this member declares — a field naming another member, or a mention in its prose — names a target the tree does not hold, or a bare key several hosts carry. The runtime resolves such a field by name: a subagent's `skills` list, for one, preloads each skill it names, so a name that matches no skill delivers nothing. Fix the name, author the missing member, or spell the whole `<host-address>/<kind>/<key>` address where hosts share a key. Required because the program's own `emit` refuses the same reference; dial this label to `advisory` while a tree mid-adoption still carries one.",
    cite: "https://code.claude.com/docs/en/sub-agents#preload-skills-into-subagents (retrieved 2026-11-26)",
  }),
  clause(importBacked(), {
    severity: "advisory",
    guidance:
      "This member's `@path` import names no member and no repository file. Claude Code expands an import into context beside the file that carries it, resolving a relative path against that file rather than the working directory, so a path that reaches nothing loads nothing, silently. Fix the path relative to the importing file — a trailing `.` reads as part of it — or set the text off in backticks if it was never meant as an import. Advisory because the importing file still loads; dial this label to `required` — or re-declare the clause — once a dead import should fail CI.",
    cite: "https://code.claude.com/docs/en/memory#import-additional-files (retrieved 2026-11-26)",
  }),
  clause(
    listing(["description", "when_to_use"], {
      entry: SKILL_LISTING_ENTRY_CAP,
      budget: SKILL_LISTING_CHARACTER_BUDGET,
      withheldBy: "disable-model-invocation",
      kinds: [skill, command],
    }),
    {
      severity: "advisory",
      guidance:
        "Every model-invocable skill's and command's `description` and `when_to_use` ride every request in one listing, under a character budget that scales at 1% of the context window, with a fallback of 8,000 characters — the figure this clause judges. Each entry counts up to the 1,536-character cap the listing cuts it at. Over the budget, Claude Code keeps every name but drops descriptions starting with the ones you invoke least, so those skills and commands lose the keywords Claude matches a request against. One with `disable-model-invocation: true` is not listed and is not counted. Trim descriptions to the key use case first, set rarely-used ones to `disable-model-invocation: true` or to `name-only` in `skillOverrides`, or raise the budget with `skillListingBudgetFraction` (or `SLASH_COMMAND_TOOL_CHAR_BUDGET`) and retune this clause's bound to match. Counted over this harness's own skills and commands only: plugin, bundled and personal skills share the same listing, so the real headroom is smaller.",
      cite: SKILL_LISTING_BUDGET_CITE,
    },
  ),
];
