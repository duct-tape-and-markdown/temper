/**
 * Prose — three constructors, one field type. A member's words are data the
 * member declares: `file()` for a document that keeps its medium, `` text`…` ``
 * for short inline prose, `blocks()` for fully composed embedded-member values. Whatever
 * the constructor, the words land byte-identical to their authored text (law 5).
 * Interpolations in `` text`…` `` are references, two intents apart: a **mention**
 * (a {@link Mentionable}) is a declared one-way edge that moves no content, and an
 * **include** (an {@link Include}) pulls the target file's bytes into the host's emitted
 * projection. Both are authored per word and resolution-checked at emit, never mined
 * (law 8).
 */

import type { EmbeddedMemberValue, Member } from "./kind.js";

/** A declared value a mention may name — the target of the one-way citation edge. */
export interface Mentionable {
  /** The mention's rendered form and graph edge target (`kind:name` or a leaf address). */
  readonly address: string;
  /** The display text the one corpus-wide rule renders in place. */
  readonly display: string;
}

/**
 * Spell a top-level member as the {@link Mentionable} a mention carries: its
 * `kind:name` address, its bare name the display text — the convention every
 * corpus repeats to cite a member from prose, captured once here.
 */
export function mentionOf(member: Member): Mentionable {
  return { address: `${member.kind}:${member.name}`, display: member.name };
}

/** One authored interpolation: position in the template plus its target. */
export interface Mention {
  readonly index: number;
  readonly target: Mentionable;
}

/**
 * An include the author interpolates into `` text`…` `` — it moves content, not a
 * citation: at emit the engine pulls the target file's bytes into the host's projection
 * at this slot and fingerprints the dependency. The path resolves against the stating
 * module ({@link moduleUrl}), never the workspace — the same anchor {@link file} takes.
 */
export interface Include {
  readonly kind: "include";
  /** Path to the include target, resolved against {@link moduleUrl}. */
  readonly path: string;
  /** The declaring module's own `import.meta.url` — what {@link path} resolves against. */
  readonly moduleUrl: string;
}

/** A `` text`…` `` interpolation — a mention (moves no content) or an include (moves content). */
export type Reference = Mentionable | Include;

/** Whether an interpolation target is an include rather than a mention. */
function isInclude(reference: Reference): reference is Include {
  return (reference as Include).kind === "include";
}

/**
 * The interpolation marker {@link text} plants in the template — `U+0000`, one
 * per mention in authored order. Authored markdown never carries a NUL, so
 * splitting the template on it recovers the literal chunks unambiguously.
 */
const MENTION_SLOT = "\u0000";

/**
 * The include marker {@link text} plants per include — `U+0001`, the content-moving
 * counterpart to {@link MENTION_SLOT}. A rendered body keeps it (mentions resolve to
 * display text, includes stay slots for the engine to splice); authored markdown never
 * carries one, so the split is unambiguous.
 */
const INCLUDE_SLOT = "\u0001";

/**
 * `file(moduleUrl, path)` — the document keeps its medium: markdown in a
 * markdown file, full tooling, forever legal (posture 1, `15-kinds.md`).
 * Resolved and read in byte-for-byte at emit, relative to the declaring
 * module, never the process cwd.
 */
export interface File {
  readonly kind: "file";
  /** Path to the authored document, resolved against {@link moduleUrl}. */
  readonly path: string;
  /** The declaring module's own `import.meta.url` — what {@link path} resolves against. */
  readonly moduleUrl: string;
}

/**
 * `` text`…` `` — short prose inline, dedented, byte-deterministic; the
 * three-line rule that would be silly as a sidecar file. Mentions ride beside
 * the text, never inside it (law 5).
 */
export interface Text {
  readonly kind: "text";
  /**
   * The dedented authored text with one {@link MENTION_SLOT} per mention and one
   * {@link INCLUDE_SLOT} per include, each in authored order.
   */
  readonly template: string;
  readonly mentions: readonly Mention[];
  /** The includes riding beside the text, in authored order — one per {@link INCLUDE_SLOT}. */
  readonly includes: readonly Include[];
}

/**
 * `blocks(…)` — fully composed embedded-member values (posture 3): typed
 * collections rendered to `member.<kind> <key>` TOML fences, byte-identical to
 * the same values authored as fences directly (posture 2) and read back by the
 * engine's fold (`src/extract.rs` `parse_embedded_info`/`parse_embedded_member`).
 */
export interface Blocks {
  readonly kind: "blocks";
  readonly values: readonly EmbeddedMemberValue[];
}

/** A member's prose — one of the three constructors, one field type. */
export type Prose = File | Text | Blocks;

/** Strip the common leading indentation a template literal picks up from its module. */
function dedent(text: string): string {
  const lines = text.split("\n");
  const indents = lines
    .filter((line) => line.trim().length > 0)
    .map((line) => line.match(/^[ \t]*/)![0].length);
  const cut = indents.length > 0 ? Math.min(...indents) : 0;
  return lines
    .map((line) => line.slice(cut))
    .join("\n")
    .replace(/^\n/, "")
    .replace(/\n[ \t]*$/, "\n");
}

/**
 * Declare a document whose medium is preserved — read in whole at emit
 * (posture 1). `moduleUrl` is the declaring module's own `import.meta.url`,
 * so `path` resolves relative to that module, never the process cwd:
 * `file(import.meta.url, "./long.md")`.
 */
export function file(moduleUrl: string, path: string): File {
  return { kind: "file", path, moduleUrl };
}

/**
 * The inline dedenting prose constructor. Interpolate {@link Reference} values — a
 * {@link Mentionable} is a mention (moves no content), an {@link Include} pulls the
 * target's bytes in; either is opt-in per word, and plain prose with zero references is
 * fully legal forever (the opt-in Decision, `20-surface.md`).
 *
 * # Throws
 * If an authored chunk carries {@link MENTION_SLOT} or {@link INCLUDE_SLOT} — the markers
 * are the tool's alone, so a stray one is a loud authoring error, not a silent mis-split.
 */
export function text(strings: TemplateStringsArray, ...targets: Reference[]): Text {
  let template = strings[0];
  const mentions: Mention[] = [];
  const includes: Include[] = [];
  strings.forEach((chunk, i) => {
    if (chunk.includes(MENTION_SLOT) || chunk.includes(INCLUDE_SLOT)) {
      throw new Error("authored prose contains a reserved reference marker (U+0000/U+0001); remove it.");
 }
    if (i === 0) return;
    const target = targets[i - 1];
    if (isInclude(target)) {
      template += INCLUDE_SLOT + chunk;
      includes.push(target);
    } else {
      mentions.push({ index: mentions.length, target });
      template += MENTION_SLOT + chunk;
    }
  });
  return { kind: "text", template: dedent(template), mentions, includes };
}

/**
 * Declare an include target — its bytes are pulled into the host's projection at emit,
 * a dependency the lock fingerprints. `moduleUrl` is the declaring module's own
 * `import.meta.url`, so `path` resolves relative to that module, never the process cwd:
 * `include(import.meta.url, "./fragment.md")`.
 */
export function include(moduleUrl: string, path: string): Include {
  return { kind: "include", path, moduleUrl };
}

/** Compose fully-typed embedded-member values into a member's body (posture 3). */
export function blocks(...values: EmbeddedMemberValue[]): Blocks {
  return { kind: "blocks", values };
}

/**
 * Render an inline body to its final text — the display rule applied: each
 * mention slot becomes its target's display form, the surrounding words
 * untouched (law 5). The chunk count is `mentions.length + 1` by {@link text}'s
 * construction, so the walk consumes every slot.
 */
export function renderText(prose: Text): string {
  const chunks = prose.template.split(MENTION_SLOT);
  let out = chunks[0];
  prose.mentions.forEach((mention, i) => {
    out += mention.target.display + chunks[i + 1];
  });
  return out;
}

/**
 * Resolve a leaf's authored value to its stored/rendered string: a bare string
 * is unchanged; a `Text` leaf is mention-resolution-checked against
 * `mentionable` (loud on a dangling address) and rendered by {@link renderText}
 * — the same rule a member-level `Text` body resolves by. `context` prefixes
 * the dangling-mention error so it names the leaf, not just the mention.
 *
 * # Throws
 * If a mention names no declared value.
 */
export function resolveLeaf(value: string | Text, mentionable: ReadonlySet<string>, context: string): string {
  if (typeof value === "string") return value;
  if (value.includes.length > 0) {
    throw new Error(
      `${context}: an embedded-member leaf cannot carry an include — a content pull is a ` +
        `member-body intent, not a fenced-value one (specs/model/pipeline.md, "The SDK").`,
    );
  }
  for (const mention of value.mentions) {
    if (!mentionable.has(mention.target.address)) {
      throw new Error(
        `${context}: mention of \`${mention.target.address}\` resolves to no declared value — ` +
          `a mention cannot dangle (specs/model/contract.md).`,
      );
    }
  }
  return renderText(value);
}
