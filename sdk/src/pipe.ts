/**
 * The one pipe — the engine's face of the SDK, never the author's. `temper emit` runs
 * the program's module through {@link pipe}: the module's default export is its harness,
 * {@link emit} compiles it, and the seam goes to stdout for the engine to read. A program
 * therefore carries no plumbing of its own and never calls an emit of its own; it ends
 * `export default harness({ ... })`.
 */

import { pathToFileURL } from "node:url";

import type { Harness } from "./assembly.js";
import { emit } from "./emit.js";

export type { EmitResult, RegistrationFact, SettingsResidue } from "./emit.js";
export { emit } from "./emit.js";

/**
 * Whether `value` is a composed {@link Harness} — the shape `harness()` returns. A
 * structural read: the seam's own refusals judge the rest.
 */
function isHarness(value: unknown): value is Harness {
  if (typeof value !== "object" || value === null) {
    return false;
  }
  const candidate = value as Record<string, unknown>;
  return Array.isArray(candidate.members) && Array.isArray(candidate.expect) && Array.isArray(candidate.admit);
}

/**
 * The program at `entry` (a path), compiled: its module's default export, refused loud
 * unless it is a composed harness.
 */
export async function programSeam(entry: string): Promise<string> {
  const module = (await import(pathToFileURL(entry).href)) as { default?: unknown };
  const program = module.default;
  if (!isHarness(program)) {
    const found = program === undefined ? "no default export" : `a default export of type ${typeof program}`;
    throw new Error(
      `${entry} exports no harness as its default (found ${found}): ` +
        "end the program's module with `export default harness({ ... })`.",
    );
  }
  return emit(program).seam;
}

/** Run the program at `entry` and print its seam to stdout — the whole of the engine's run. */
export async function pipe(entry: string): Promise<void> {
  process.stdout.write(await programSeam(entry));
}
