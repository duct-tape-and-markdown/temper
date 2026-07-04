/**
 * Emit — the compile from the six-noun face to the committed seam
 * (`specs/architecture/20-surface.md`, "Emit — total, byte-reproducible, refusing";
 * "The seam — one implementation"). The SDK implements **no semantics**: emit
 * produces plain data — the declaration rows the engine reads (the internal
 * versioned JSON pipe and the lock's `[declaration]` families), a byte-faithful
 * `.claude/**` projection, and the lock. Emit is total (members are the only
 * source), refuses before it writes a byte on a broken source, and is
 * byte-reproducible — double-emit verified at every run (law 5).
 */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve as resolvePath } from "node:path";

import type { Harness } from "./assembly.js";
import type { Member } from "./kind.js";
import { renderText } from "./prose.js";
import { permissionUnion } from "./needs.js";
import type { Projection, ProjectionInput } from "./project.js";
import { projectMember } from "./project.js";
import type { LockRow } from "./lock.js";
import { lockRow, stampLock } from "./lock.js";
import type { Declarations } from "./declarations.js";
import { compileDeclarations, declarationsToJson } from "./declarations.js";

/** How a `file()` asset's module-relative path resolves at emit. */
export interface ResolveOptions {
  /** Base dir a `file()` module-relative path resolves against (default: cwd). */
  readonly baseDir?: string;
  /** The addresses a mention may name — resolution-checked; a mention cannot dangle. */
  readonly mentionable?: ReadonlySet<string>;
}

/**
 * Resolve a member's prose to its final body bytes: a `file()` asset is read in
 * byte-for-byte; a `text` body's mentions are resolution-checked (loud on a
 * dangling address) and rendered by the one display rule; a `blocks()` body is
 * refused until the fence format lands. The words are never reworded (law 5).
 *
 * # Throws
 * If a `file()` asset does not resolve, a mention names no declared value, or a
 * `blocks()` body is projected before `(genre-fence-format)` lands.
 */
function resolveBody(member: Member, options: ResolveOptions): string {
  const prose = member.prose;
  if (prose === undefined) return "";
  if (prose.kind === "file") {
    const assetPath = resolvePath(options.baseDir ?? process.cwd(), prose.path);
    try {
      return readFileSync(assetPath, "utf8");
    } catch (cause) {
      throw new Error(
        `member \`${member.name}\`: file() asset \`${prose.path}\` did not resolve ` +
          `(looked at \`${assetPath}\`).`,
        { cause },
      );
    }
  }
  if (prose.kind === "blocks") {
    throw new Error(
      `member \`${member.name}\`: a blocks() body renders through the genre fence format, ` +
        `deferred until its first consumer lands ((genre-fence-format), specs/architecture/15-kinds.md).`,
    );
  }
  const mentionable = options.mentionable ?? new Set<string>();
  for (const mention of prose.mentions) {
    if (!mentionable.has(mention.target.address)) {
      throw new Error(
        `member \`${member.name}\`: mention of \`${mention.target.address}\` resolves to no ` +
          `declared value — a mention cannot dangle (specs/architecture/45-governance.md).`,
      );
    }
  }
  return renderText(prose);
}

/** Every requirement name a `satisfies` claim may fill — assembly `require` ∪ member `requires`. */
function declaredRequirements(harness: Harness): Set<string> {
  const set = new Set<string>();
  for (const name of Object.keys(harness.require)) set.add(name);
  for (const member of harness.members) {
    for (const name of Object.keys(member.requires)) set.add(name);
  }
  return set;
}

/** Every address a mention may name — declared requirement names ∪ each member's `kind:name`. */
function declaredAddresses(harness: Harness): Set<string> {
  const set = declaredRequirements(harness);
  for (const member of harness.members) set.add(`${member.kind}:${member.name}`);
  return set;
}

/**
 * The two declare-side refusals emit runs before it compiles a byte
 * (`20-surface.md`, "Emit refuses before it writes"): a `satisfies` claim naming
 * no declared requirement (a dangling join), and a `required` requirement no
 * member fills (an unfilled required requirement).
 *
 * # Throws
 * On a dangling `satisfies` join or an unfilled `required` requirement.
 */
function refuseBrokenSource(harness: Harness): void {
  const requirements = declaredRequirements(harness);
  const filled = new Set<string>();
  for (const member of harness.members) {
    for (const name of member.satisfies) {
      if (!requirements.has(name)) {
        throw new Error(
          `member \`${member.name}\`: \`satisfies\` claims requirement \`${name}\`, which no ` +
            `harness-level or member-published requirement declares — a dangling join ` +
            `(specs/architecture/20-surface.md, "Emit refuses before it writes").`,
        );
      }
      filled.add(name);
    }
  }

  const requiredSources: [string, string][] = [];
  for (const [name, requirement] of Object.entries(harness.require)) {
    if (requirement.required) requiredSources.push([name, "the assembly"]);
  }
  for (const member of harness.members) {
    for (const [name, requirement] of Object.entries(member.requires)) {
      if (requirement.required) requiredSources.push([name, `member \`${member.name}\``]);
    }
  }
  for (const [name, source] of requiredSources) {
    if (!filled.has(name)) {
      throw new Error(
        `required requirement \`${name}\` (declared by ${source}) is filled by no member's ` +
          `\`satisfies\` — an unfilled required requirement ` +
          `(specs/architecture/20-surface.md, "Emit refuses before it writes").`,
      );
    }
  }
}

/** A member is projected iff its kind lives at a path locus (a genre member is not). */
function isProjected(member: Member): boolean {
  return member.facts.locus.kind === "at";
}

/** The harness's projected members as projection inputs, deterministically kind-then-name ordered. */
function orderedProjectionInputs(harness: Harness, options: ResolveOptions): ProjectionInput[] {
  return [...harness.members]
    .filter(isProjected)
    .sort(
      (a, b) =>
        (a.kind < b.kind ? -1 : a.kind > b.kind ? 1 : 0) ||
        (a.name < b.name ? -1 : a.name > b.name ? 1 : 0),
    )
    .map((member) => ({
      facts: member.facts,
      name: member.name,
      fields: member.fields,
      body: resolveBody(member, options),
    }));
}

/** Emit-time inputs beyond the harness — asset base dir and the committed-projection read root. */
export interface EmitOptions {
  /** Base dir a `file()` asset's module-relative path resolves against (default: cwd). */
  readonly baseDir?: string;
  /**
   * The harness root the committed projection is read from so a re-emit carries
   * install's placement lines through the whole-file re-emit (`20-surface.md`, the
   * two-projectors seam). Absent reads no committed projection; [`writeEmit`]
   * passes its `targetDir` here so a re-emit preserves them.
   */
  readonly projectionDir?: string;
}

/**
 * A full emit's compiled outputs — the seam the engine reads plus the on-disk
 * projection and lock (`20-surface.md`, "The seam"). All are a pure function of
 * the harness, so [`emit`] double-verifies them and [`writeEmit`] lands them.
 */
export interface EmitResult {
  /** The projection files, one per projected (rule/skill/memory) member. */
  readonly projections: readonly Projection[];
  /** The `lock.toml` bytes — rollup rows plus the `[declaration]` families. */
  readonly lock: string;
  /** The declaration rows — the erased program the lock and JSON pipe both carry. */
  readonly declarations: Declarations;
  /** The internal versioned JSON pipe to the engine — not a designed IR. */
  readonly seam: string;
  /**
   * The derived permission list — the union of every member's `needs`, deduped and
   * sorted (`20-surface.md`, "The permission list is derived, never authored").
   * Folds into the settings artifact once hook/MCP members land; carried here as
   * data until then.
   */
  readonly permissions: readonly string[];
}

/**
 * Compile the whole face in one deterministic pass: the projection, the lock (its
 * rollup and its declaration rows), the JSON pipe, and the derived permission
 * union. Prose resolves once (`file()` assets read in, mentions resolution-checked
 * against the harness's declared values). Double-emit verified — nondeterministic
 * authoring is a loud failure, never a silent churn (law 5).
 */
export function emit(harness: Harness, options: EmitOptions = {}): EmitResult {
  refuseBrokenSource(harness);
  const resolve: ResolveOptions = {
    mentionable: declaredAddresses(harness),
    baseDir: options.baseDir,
  };
  const compile = (): EmitResult => {
    const inputs = orderedProjectionInputs(harness, resolve);
    const projections = inputs.map((input) => projectMember(input, { projectionDir: options.projectionDir }));
    const rows: LockRow[] = inputs.map((input, i) => lockRow(input.facts.name, projections[i]));
    const declarations = compileDeclarations(harness);
    return {
      projections,
      lock: stampLock(rows, declarations),
      declarations,
      seam: declarationsToJson(declarations),
      permissions: permissionUnion(harness.members.flatMap((member) => [...member.needs])),
    };
  };
  const first = compile();
  const second = compile();
  if (
    first.lock !== second.lock ||
    first.seam !== second.seam ||
    !sameProjections(first.projections, second.projections)
  ) {
    throw new Error(
      "double-emit divergence: two passes over the same harness produced different bytes — " +
        "authoring code is nondeterministic (a timestamp? an unordered map?).",
    );
  }
  return first;
}

/** Whether two projection lists are byte-identical, path and bytes both. */
function sameProjections(a: readonly Projection[], b: readonly Projection[]): boolean {
  return a.length === b.length && a.every((p, i) => p.path === b[i].path && p.bytes === b[i].bytes);
}

/**
 * Run a full [`emit`] and write its committed artifacts under `targetDir`: the
 * lock to `lock.toml` and each projection to its `.claude/**` path (parent
 * directories created). Whole-file writes — a projection is regenerated, never
 * patched. The JSON pipe is in-flight, not a committed artifact, so it is not
 * written (`20-surface.md`, "the committed seam" is artifacts plus lock).
 */
export function writeEmit(harness: Harness, targetDir: string, options: EmitOptions = {}): EmitResult {
  const result = emit(harness, { ...options, projectionDir: options.projectionDir ?? targetDir });
  writeFileSync(join(targetDir, "lock.toml"), result.lock);
  for (const projection of result.projections) {
    const path = join(targetDir, projection.path);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, projection.bytes);
  }
  return result;
}
