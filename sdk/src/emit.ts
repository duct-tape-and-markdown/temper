/**
 * Emit — the compile from the six-noun face to the seam's JSON pipe.
 * The SDK implements **no semantics**: emit
 * produces plain data — the declaration rows the engine reads and, per projected
 * member, its ordered typed fields and resolved prose body. The engine is the
 * sole compiler of every projection and the whole lock; the SDK writes neither.
 * Emit is total (members are the only source), refuses before it produces a byte
 * on a broken source, and is byte-reproducible — double-emit verified at every
 * run.
 */

import { resolve as resolvePath } from "node:path";
import { readFileSync } from "node:fs";

import type { Harness } from "./assembly.js";
import type { Member } from "./kind.js";
import { renderText } from "./prose.js";
import { permissionUnion } from "./needs.js";
import type { Declarations } from "./declarations.js";
import { SEAM_VERSION, compileDeclarations } from "./declarations.js";

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
 * refused until the fence format lands. The words are never reworded.
 *
 * # Throws
 * If a `file()` asset does not resolve, a mention names no declared value, or a
 * `blocks()` body is projected before `(genre-fence-format)` lands.
 */
function resolveBody(member: Member, options: ResolveOptions): string {
  const prose = member.prose;
  if (prose === undefined) return "";
  if (prose.kind === "file") {
    const assetPath = fileSourcePath(member, options)!;
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
        `deferred until its first consumer lands ((genre-fence-format), specs/model/representation.md).`,
    );
  }
  const mentionable = options.mentionable ?? new Set<string>();
  for (const mention of prose.mentions) {
    if (!mentionable.has(mention.target.address)) {
      throw new Error(
        `member \`${member.name}\`: mention of \`${mention.target.address}\` resolves to no ` +
          `declared value — a mention cannot dangle (specs/model/contract.md).`,
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
 * The two declare-side refusals emit runs before it produces a byte:
 * a `satisfies` claim naming
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
            `(specs/model/pipeline.md, "Emit", the "Refusing" bullet).`,
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
          `(specs/model/pipeline.md, "Emit", the "Refusing" bullet).`,
      );
    }
  }
}

/** A member is projected iff its kind lives at a path locus (a genre member is not). */
function isProjected(member: Member): boolean {
  return member.facts.locus.kind === "at";
}

/**
 * The resolved absolute path of a `file()` prose asset, or `undefined` for
 * `text`/`blocks` prose (or no prose) — the lift's own-path detection
 * (drift: the lock is what names a path a
 * projection, so the engine needs each `file()` member's true source path to
 * tell a lifted member's own file apart from a generated one).
 */
function fileSourcePath(member: Member, options: ResolveOptions): string | undefined {
  const prose = member.prose;
  if (prose?.kind !== "file") return undefined;
  return resolvePath(options.baseDir ?? process.cwd(), prose.path);
}

/** One projected member's erased payload — the engine derives its locus from the kind's own declaration row. */
export interface PayloadMember {
  /** The kind's bare name — joins the payload's `declarations.kinds` family. */
  readonly kind: string;
  /** Identity within the kind. */
  readonly name: string;
  /** The kind's typed fields, flat and ordered — the projected frontmatter. */
  readonly fields: ReadonlyArray<readonly [string, unknown]>;
  /** The resolved prose body, byte-faithful. */
  readonly body: string;
  /** The resolved `file()` asset's absolute path; absent for `text`/`blocks` prose. */
  readonly source_path?: string;
}

/** The harness's projected members as payload members, deterministically kind-then-name ordered. */
function orderedMembers(harness: Harness, options: ResolveOptions): PayloadMember[] {
  return [...harness.members]
    .filter(isProjected)
    .sort(
      (a, b) =>
        (a.kind < b.kind ? -1 : a.kind > b.kind ? 1 : 0) ||
        (a.name < b.name ? -1 : a.name > b.name ? 1 : 0),
    )
    .map((member) => ({
      kind: member.kind,
      name: member.name,
      fields: member.fields,
      body: resolveBody(member, options),
      source_path: fileSourcePath(member, options),
    }));
}

/** Emit-time inputs beyond the harness — where a `file()` asset's module-relative path resolves against. */
export interface EmitOptions {
  /** Base dir a `file()` asset's module-relative path resolves against (default: cwd). */
  readonly baseDir?: string;
}

/**
 * A full emit's compiled outputs — the whole seam the engine reads.
 * A pure function of the harness, so [`emit`]
 * double-verifies it.
 */
export interface EmitResult {
  /** The declaration rows — the erased program the lock's six families carry. */
  readonly declarations: Declarations;
  /** The projected members — the engine's sole input for every projection. */
  readonly members: readonly PayloadMember[];
  /**
   * The internal versioned JSON pipe to the engine — not a designed IR. The
   * SDK's whole output surface: printed to stdout, never written to a file.
   */
  readonly seam: string;
  /**
   * The derived permission list — the union of every member's `needs`, deduped and
 * sorted.
   * Folds into the settings artifact once hook/MCP members land; carried here as
   * data until then.
   */
  readonly permissions: readonly string[];
}

/**
 * Compile the whole face in one deterministic pass: the declaration rows (its
 * rollup and its six families) and every projected member's erased payload.
 * Prose resolves once (`file()` assets read in, mentions resolution-checked
 * against the harness's declared values). Double-emit verified — nondeterministic
 * authoring is a loud failure, never a silent churn.
 */
export function emit(harness: Harness, options: EmitOptions = {}): EmitResult {
  refuseBrokenSource(harness);
  const resolve: ResolveOptions = {
    mentionable: declaredAddresses(harness),
    baseDir: options.baseDir,
  };
  const compile = (): EmitResult => {
    const members = orderedMembers(harness, resolve);
    const declarations = compileDeclarations(harness);
    return {
      declarations,
      members,
      seam: JSON.stringify({ version: SEAM_VERSION, declarations, members }, null, 2) + "\n",
      permissions: permissionUnion(harness.members.flatMap((member) => [...member.needs])),
    };
  };
  const first = compile();
  const second = compile();
  if (first.seam !== second.seam) {
    throw new Error(
      "double-emit divergence: two passes over the same harness produced different bytes — " +
        "authoring code is nondeterministic (a timestamp? an unordered map?).",
    );
  }
  return first;
}
