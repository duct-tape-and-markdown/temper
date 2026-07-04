/**
 * Assembly artifacts — the two **locus-less** assembly facts emitted as small
 * committed temper-owned files (`specs/architecture/20-surface.md`, "The
 * surface: the assembly over its contents": "the bindings, the roster — are
 * emitted as small committed temper-owned artifacts"). Bindings and the roster
 * have no harness locus of their own, so emit lands them as members of temper's
 * own landscape — everything the offline engine reads is a committed artifact.
 *
 * Additive to the manifest/projection/lock: the engine still reads the existing
 * manifest until its evidence-gated sunset (D6), so nothing here demolishes a
 * read path. Both artifacts serialize through the shared `toml.ts` encoders —
 * deterministic, sorted-key TOML, whole-file bytes, double-emit stable.
 */

import type { Harness, KindBinding, Requirement } from "./assembly.js";
import { encodeKey, encodeString, joinSections, keyValue, sortedKeys } from "./toml.js";

/** The bindings artifact's committed path, beside `temper.toml`/`lock.toml`. */
export const BINDINGS_PATH = "bindings.toml";

/** The roster artifact's committed path, beside `temper.toml`/`lock.toml`. */
export const ROSTER_PATH = "roster.toml";

/** The two compiled assembly-fact artifacts — whole-file TOML bytes each. */
export interface AssemblyArtifacts {
  /** The kind→package bindings: one `[binding.<kind>]` table per declared kind. */
  readonly bindings: string;
  /** The requirement roster: one `[requirement.<name>]` table per declared requirement. */
  readonly roster: string;
}

/**
 * Serialize the kind bindings — one `[binding.<kind>]` table carrying the bound
 * `package`, kinds name-sorted so the bytes are stable across runs. The kind
 * name is a single quoted sub-key (`[binding."claude-code.rule"]`), so a dotted
 * kind never splits into nested tables.
 */
export function serializeBindings(kinds: Readonly<Record<string, KindBinding>>): string {
  const sections = sortedKeys(kinds).map(
    (kind) => `[binding.${encodeKey(kind)}]\n` + keyValue("package", encodeString(kinds[kind].package)),
  );
  return joinSections(sections);
}

/**
 * Serialize the requirement roster — one `[requirement.<name>]` table per
 * declared requirement carrying its `means`, `kind`, and (only when set) the
 * `required` flag, requirements name-sorted for byte-stability. `required` is
 * omitted when false, matching the manifest's published-requirement spelling.
 */
export function serializeRoster(requirements: Readonly<Record<string, Requirement>>): string {
  const sections = sortedKeys(requirements).map((name) => {
    const requirement = requirements[name];
    let block = `[requirement.${encodeKey(name)}]\n`;
    block += keyValue("means", encodeString(requirement.means));
    block += keyValue("kind", encodeString(requirement.kind));
    if (requirement.required) block += keyValue("required", "true");
    return block;
  });
  return joinSections(sections);
}

/** Compile both assembly-fact artifacts from a harness's bindings and roster. */
export function assemblyArtifacts(harness: Harness): AssemblyArtifacts {
  return {
    bindings: serializeBindings(harness.kinds),
    roster: serializeRoster(harness.requirements),
  };
}
