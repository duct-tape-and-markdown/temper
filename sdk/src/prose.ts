/**
 * Prose — three constructors, one field type. A member's words are data the
 * member declares: `file()` for a document that keeps its medium, `` text`…` ``
 * for short inline prose, `blocks()` for a composed body that interleaves verbatim
 * prose spans with embedded-member values in authored order. Whatever
 * the constructor, the words land byte-identical to their authored text.
 * Interpolations in `` text`…` `` are references, two intents apart: a **mention**
 * (a {@link Mentionable}) is a declared one-way edge that moves no content, and an
 * **include** (an {@link Include}) pulls the target file's bytes into the host's emitted
 * projection. Both are authored per word and resolution-checked at emit, never mined.
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

/**
 * The resolution universe a mention is checked against at emit: the addresses that
 * resolve here and now ({@link mentionable}), and the kinds whose members the program
 * declares but does not compose ({@link deferrableKinds}) — a mention naming one of
 * those defers to `check` rather than refusing at emit.
 */
export interface MentionScope {
  /** Every address a mention may resolve against in the program's own universe. */
  readonly mentionable: ReadonlySet<string>;
  /** The discoverable (`at`-locus) kinds the program declares — the deferral signal. */
  readonly deferrableKinds: ReadonlySet<string>;
}

/**
 * Whether a mention's unresolved address **defers to the gate** rather than refusing at
 * emit: a top-level `kind:name` address whose kind is one the program declares at a
 * discovery locus (an `at`-locus kind) may name a member discovered on disk, so `check`
 * owns the verdict. An embedded leaf address (a `<host>/<kind>/<key>` form, carrying a
 * `/`), a bare requirement name (no `:`), or a kind the program does not declare has no
 * discovery locus and stays a dangling refusal.
 */
export function defersToGate(address: string, deferrableKinds: ReadonlySet<string>): boolean {
  if (address.includes("/")) return false;
  const colon = address.indexOf(":");
  return colon > 0 && deferrableKinds.has(address.slice(0, colon));
}

/**
 * Refuse a mention whose address neither resolves against the scope's `mentionable`
 * set nor defers to the gate ({@link defersToGate}) — the one dangling-mention refusal,
 * shared by a member-level `Text` body, a composed body's prose span, and an embedded
 * `Text` leaf. `context` prefixes the error so it names the host.
 *
 * # Throws
 * If a mention names no declared value and its address has no discovery locus.
 */
export function checkMentions(mentions: readonly Mention[], scope: MentionScope, context: string): void {
  for (const mention of mentions) {
    const { address } = mention.target;
    if (!scope.mentionable.has(address) && !defersToGate(address, scope.deferrableKinds)) {
      throw new Error(
        `${context}: mention of \`${address}\` resolves to no declared value — ` +
          `a mention cannot dangle (specs/model/contract.md).`,
      );
    }
  }
}

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
 * markdown file, full tooling, forever legal. Resolved and read in
 * byte-for-byte at emit, relative to the declaring module, never the
 * process cwd.
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
 * the text, never inside it.
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
 * `blocks(…)` — a composed body of ordered children: verbatim prose spans
 * ({@link Text}) and embedded-member values interleaved in authored order,
 * the write-side mirror of a layout's ordered regions. A {@link Text} span stays
 * prose — it carries its own mentions and includes and mints no wrapper member;
 * an embedded value renders to its `member.<kind> <key>` TOML fence, byte-identical
 * to the same value authored as a fence directly and read back by the
 * engine's fold (`src/extract.rs` `parse_embedded_info`/`parse_embedded_member`).
 */
export interface Blocks {
  readonly kind: "blocks";
  readonly values: readonly (Text | EmbeddedMemberValue)[];
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
 * Declare a document whose medium is preserved — read in whole at emit.
 * `moduleUrl` is the declaring module's own `import.meta.url`,
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
 * fully legal forever.
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

/**
 * Compose a member's body from ordered children: verbatim prose spans
 * ({@link text}) and embedded-member values interleaved in authored order. A prose
 * span rides as prose — no wrapper member is minted to carry a narrative.
 *
 * # Throws
 * If a child is a {@link File} — the parameter type excludes one, so only a JS or
 * cast caller arrives here, and the alternative is a raw crash downstream on the
 * `leaves` a `File` has no reason to carry.
 */
export function blocks(...values: (Text | EmbeddedMemberValue)[]): Blocks {
  values.forEach((value, index) => {
    if (isFileValue(value)) {
      throw new Error(
        `blocks(): block ${index} is a \`file()\` value — a composed body admits a ` +
          `\`text\` span or an embedded member value. A document is a member's whole ` +
          `\`prose\` body (\`prose: file(…)\`); to pull its bytes into a composed body, ` +
          `interpolate \`include()\` into a \`text\` span (specs/model/pipeline.md, "The SDK").`,
      );
    }
  });
  return { kind: "blocks", values };
}

/**
 * Whether a composed-body child is a {@link File}. A child kind may itself be named
 * `file`, so the kind tag alone is ambiguous — an embedded value always carries
 * `leaves`, and only a `File` anchors a `moduleUrl`.
 */
function isFileValue(value: Text | EmbeddedMemberValue): boolean {
  const candidate = value as Partial<File>;
  return candidate.kind === "file" && typeof candidate.moduleUrl === "string";
}

/**
 * Whether a composed-body child is a verbatim prose span rather than an embedded
 * member value — the discriminant emit and the row builders branch on to render a
 * span as prose and count its refs at host level, never as a nested member.
 */
export function isTextSpan(value: Text | EmbeddedMemberValue): value is Text {
  return (value as Text).kind === "text";
}

/**
 * Render an inline body to its final text — the display rule applied: each
 * mention slot becomes its target's display form, the surrounding words
 * untouched. The chunk count is `mentions.length + 1` by {@link text}'s
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
 * is unchanged; a `Text` leaf is mention-resolution-checked against `scope`
 * ({@link checkMentions}: loud on a dangling address, a discovery-locus one
 * deferred) and rendered by {@link renderText} — the same rule a member-level
 * `Text` body resolves by. `context` prefixes the dangling-mention error so it
 * names the leaf, not just the mention.
 *
 * # Throws
 * If a mention names no declared value and has no discovery locus.
 */
export function resolveLeaf(value: string | Text, scope: MentionScope, context: string): string {
  if (typeof value === "string") return value;
  if (value.includes.length > 0) {
    throw new Error(
      `${context}: an embedded-member leaf cannot carry an include — a content pull is a ` +
        `member-body intent, not a fenced-value one (specs/model/pipeline.md, "The SDK").`,
    );
  }
  checkMentions(value.mentions, scope, context);
  return renderText(value);
}
