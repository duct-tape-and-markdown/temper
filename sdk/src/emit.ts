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

import { fileURLToPath } from "node:url";
import { readFileSync } from "node:fs";

import type { Harness } from "./assembly.js";
import type { EmbeddedMemberValue, Member } from "./kind.js";
import { renderText } from "./prose.js";
import { permissionUnion } from "./needs.js";
import type { Declarations } from "./declarations.js";
import { compareStrings, compileDeclarations, encodeSeam } from "./declarations.js";

/** What a mention may resolve against at emit. */
export interface ResolveOptions {
  /** The addresses a mention may name — resolution-checked; a mention cannot dangle. */
  readonly mentionable?: ReadonlySet<string>;
}

/**
 * TOML-quote a leaf's authored text into a basic-string literal — the escapes
 * `toml_edit`'s parser reads back (backslash, quote, and the C0 control set),
 * so a leaf survives the write↔read round trip byte-identically.
 */
function tomlString(text: string): string {
  let out = '"';
  for (const char of text) {
    switch (char) {
      case "\\":
        out += "\\\\";
        break;
      case '"':
        out += '\\"';
        break;
      case "\b":
        out += "\\b";
        break;
      case "\t":
        out += "\\t";
        break;
      case "\n":
        out += "\\n";
        break;
      case "\f":
        out += "\\f";
        break;
      case "\r":
        out += "\\r";
        break;
      default:
        out += char.charCodeAt(0) < 0x20 ? `\\u${char.charCodeAt(0).toString(16).padStart(4, "0")}` : char;
    }
  }
  return out + '"';
}

/**
 * Render one embedded member's interior TOML: its top-level leaves, then each
 * keyed collection entry as its own `[collection.entry]` table — the exact shape
 * `parse_embedded_member` (`src/extract.rs`) folds back into leaves/members.
 */
function renderMemberToml(value: EmbeddedMemberValue): string {
  const lines: string[] = [];
  for (const [key, text] of Object.entries(value.leaves)) {
    lines.push(`${key} = ${tomlString(text)}`);
  }
  for (const [collection, entries] of Object.entries(value.collections)) {
    for (const [entryKey, leaves] of Object.entries(entries)) {
      if (lines.length > 0) lines.push("");
      lines.push(`[${collection}.${entryKey}]`);
      for (const [leaf, text] of Object.entries(leaves)) {
        lines.push(`${leaf} = ${tomlString(text)}`);
      }
    }
  }
  return lines.join("\n");
}

/**
 * Render one embedded member's value to its `member.<kind> <key>` fenced block —
 * the write face `parse_embedded_info`/`parse_embedded_member` (`src/extract.rs`)
 * fold back into an identical `EmbeddedMember`.
 */
function renderMemberFence(value: EmbeddedMemberValue): string {
  return `\`\`\`member.${value.kind} ${value.key}\n${renderMemberToml(value)}\n\`\`\``;
}

/**
 * Resolve a member's prose to its final body bytes: a `file()` asset is read in
 * byte-for-byte; a `text` body's mentions are resolution-checked (loud on a
 * dangling address) and rendered by the one display rule; a `blocks()` body
 * renders each embedded member as a `member.<kind> <key>` TOML fence. The words
 * are never reworded.
 *
 * # Throws
 * If a `file()` asset does not resolve, or a mention names no declared value.
 */
function resolveBody(member: Member, options: ResolveOptions): string {
  const prose = member.prose;
  if (prose === undefined) return "";
  if (prose.kind === "file") {
    const assetPath = fileSourcePath(member)!;
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
    return prose.values.map(renderMemberFence).join("\n\n") + "\n";
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

/** A member is projected iff its kind lives at a path locus (an embedded member is not). */
function isProjected(member: Member): boolean {
  return member.facts.locus.kind === "at";
}

/**
 * The resolved absolute path of a `file()` prose asset, or `undefined` for
 * `text`/`blocks` prose (or no prose) — the lift's own-path detection
 * (drift: the lock is what names a path a
 * projection, so the engine needs each `file()` member's true source path to
 * tell a lifted member's own file apart from a generated one). Resolves
 * against the declaring module's own `import.meta.url` (`prose.moduleUrl`),
 * never the process cwd — the path is the stating module's, not the
 * workspace's.
 */
function fileSourcePath(member: Member): string | undefined {
  const prose = member.prose;
  if (prose?.kind !== "file") return undefined;
  return fileURLToPath(new URL(prose.path, prose.moduleUrl));
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
    .sort((a, b) => compareStrings(a.kind, b.kind) || compareStrings(a.name, b.name))
    .map((member) => ({
      kind: member.kind,
      name: member.name,
      fields: member.fields,
      body: resolveBody(member, options),
      source_path: fileSourcePath(member),
    }));
}

/**
 * A full emit's compiled outputs — the whole seam the engine reads.
 * A pure function of the harness, so [`emit`]
 * double-verifies it.
 */
export interface EmitResult {
  /** The declaration rows — the erased program the lock's seven families carry. */
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
 * rollup and its seven families) and every projected member's erased payload.
 * Prose resolves once (`file()` assets read in, mentions resolution-checked
 * against the harness's declared values). Double-emit verified — nondeterministic
 * authoring is a loud failure, never a silent churn.
 */
export function emit(harness: Harness): EmitResult {
  refuseBrokenSource(harness);
  const resolve: ResolveOptions = {
    mentionable: declaredAddresses(harness),
  };
  const compile = (): EmitResult => {
    const members = orderedMembers(harness, resolve);
    const declarations = compileDeclarations(harness);
    return {
      declarations,
      members,
      seam: encodeSeam({ declarations, members }),
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
