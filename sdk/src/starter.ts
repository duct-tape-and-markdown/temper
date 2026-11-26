/**
 * The `@dtmd/temper/starter` subpath — temper's default harness as shapes the consumer
 * hydrates. Each export is a typed constructor: temper supplies the structure, wiring,
 * edges and rendering; the consumer supplies the content. A required parameter is where
 * the consumer's own content lives and has no default; an optional one is mechanics,
 * with a default.
 *
 * These are the **mechanics** shapes — what using temper well takes in any harness:
 *
 * - {@link gateHooks} — the three gate hooks: the session-start report and the guard's
 *   two edges.
 * - {@link typecheck} — the compiler `emit` runs over the program before running it.
 * - {@link reference} — the emitted harness reference, its description hydratable and its
 *   topics derived from what is declared.
 *
 * The SDK counsel rule is not a shape here: `emit` synthesizes it over every program,
 * starter or not, so there is nothing to compose. {@link starter} composes every shape
 * with its defaults into the parts `harness()` takes. It carries no taste — no memory
 * content, no conventions — and declares no telemetry: a tap is opt-in, the consumer's
 * own `require` entry with a `telemetry` verifier.
 */

import type { Typecheck } from "./assembly.js";
import { gateHook, tool as toolMember } from "./builtins.js";
import type { Input, Member } from "./kind.js";
import { reference } from "./reference.js";
import type { ReferenceInit } from "./reference.js";

export type { ReferenceInit } from "./reference.js";
export { reference } from "./reference.js";

/**
 * The tool-name matcher the guard's `PreToolUse` edge binds — Claude Code's own
 * file-writing tools (`code.claude.com/docs/en/hooks`, retrieved 2026-07-24), the matcher
 * `temper install` places (`src/install.rs`'s `GUARD_MATCHER`).
 */
const GUARD_MATCHER = "Write|Edit|MultiEdit";

/**
 * The tool-name matcher the guard's `PostToolUse` edge binds: a Bash write carries no file
 * path, so the guard judges the tree the call left (`src/install.rs`'s `BASH_MATCHER`).
 */
const SHELL_MATCHER = "Bash";

/** What {@link gateHooks} takes — mechanics alone, every field defaulted. */
export interface GateHooksInit {
  /**
   * The {@link tool} member every gate hook runs as temper. Absent, the temper package's
   * bin from the program's `node_modules/.bin` — the gate hook's own default.
   */
  readonly tool?: Member;
}

/**
 * The three gate hooks: the advisory report at `SessionStart`, the write-boundary guard
 * at `PreToolUse` over the file-writing tools, and the guard's shell edge at
 * `PostToolUse` over `Bash`. Each is a `gateHook(...)` member, so the engine renders its
 * command; the harness's `mode` decides how firmly the guard binds.
 *
 * # Throws
 * If `tool` is not a `tool(...)` member (`gateHook`).
 */
export function gateHooks(init: GateHooksInit = {}): readonly Member[] {
  const tool = init.tool === undefined ? {} : { tool: init.tool };
  return [
    gateHook({ name: "SessionStart", ...tool }),
    gateHook({ name: "PreToolUse", matcher: GUARD_MATCHER, ...tool }),
    gateHook({ name: "PostToolUse", matcher: SHELL_MATCHER, ...tool }),
  ];
}

/** What {@link typecheck} takes — the project file is the consumer's; the compiler is mechanics. */
export interface TypecheckInit {
  /** The project file the compiler checks — `input(import.meta.url, "./tsconfig.json")`. */
  readonly project: Input;
  /**
   * The {@link tool} member that runs the compiler. Absent, the program's own TypeScript
   * compiler: `tool({ package: "typescript", bin: "tsc" })`.
   */
  readonly tool?: Member;
}

/**
 * The program's typecheck — the compiler `emit` runs before the program, refusing on a
 * type error. Its `tool` is a member the harness composes in `members`, as {@link starter}
 * does.
 *
 * # Throws
 * If `tool` is not a `tool(...)` member.
 */
export function typecheck(init: TypecheckInit): Typecheck {
  if (init.tool !== undefined && init.tool.kind !== toolMember.key) {
    throw new Error(
      "typecheck: `tool` names the compiler the typecheck runs — pass a `tool(...)` member, " +
        "or omit it for the program's own `tsc`.",
    );
  }
  return {
    tool: init.tool ?? toolMember({ package: "typescript", bin: "tsc" }),
    project: init.project,
  };
}

/** What {@link starter} takes — one option per shape, each the shape's own init. */
export interface StarterOptions {
  /** The {@link typecheck} — required, since its project file is the consumer's path. */
  readonly typecheck: TypecheckInit;
  /** The {@link gateHooks}' mechanics; absent, their defaults. */
  readonly gates?: GateHooksInit;
  /** The {@link reference}'s description and domains; absent, its defaults. */
  readonly reference?: ReferenceInit;
}

/**
 * The composed starter — the parts `harness()` takes: `harness(starter(options))` is a
 * whole program, and a consumer adds its own members by spreading
 * (`harness({ ...base, members: [...base.members, ...mine] })`).
 */
export interface Starter {
  /** Every tool the shapes run, then the gate hooks, then the reference. */
  readonly members: readonly Member[];
  /** The program's typecheck, its tool among {@link members}. */
  readonly typecheck: Typecheck;
}

/**
 * Compose every mechanics shape with its defaults: the typecheck and its compiler, the
 * three gate hooks and any tool they run, and the harness reference. A tool two shapes
 * share is composed once.
 */
export function starter(options: StarterOptions): Starter {
  const checked = typecheck(options.typecheck);
  const tools = [...new Set([checked.tool, ...(options.gates?.tool === undefined ? [] : [options.gates.tool])])];
  return {
    members: [...tools, ...gateHooks(options.gates), reference(options.reference)],
    typecheck: checked,
  };
}
