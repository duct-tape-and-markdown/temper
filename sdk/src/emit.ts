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
import type {
  EdgeTargetFacts,
  EmbeddedMemberValue,
  Member,
  ResolvedEmbeddedMemberCollectionEntry,
  ResolvedEmbeddedMemberValue,
} from "./kind.js";
import type { MentionScope, Text } from "./prose.js";
import { checkMentions, isTextSpan, renderText, resolveLeaf } from "./prose.js";
import { permissionUnion } from "./needs.js";
import type { Declarations, RenderedExtent } from "./declarations.js";
import {
  compareStrings,
  compileDeclarations,
  declaredAddresses,
  declaredAtLocusKinds,
  declaredRequirements,
  encodeSeam,
  placementKey,
  registrationRows,
  settingsRows,
  tapHookRows,
} from "./declarations.js";
import type { PayloadMember } from "./generated/index.js";

// The projected-member shape is the generated `ts-rs` binding, re-exported so the
// public face keeps the name — a Rust-side member-column rename is a compile
// error here, never a silent shape drift.
export type { PayloadMember } from "./generated/index.js";

/** What a mention may resolve against at emit. */
export interface ResolveOptions {
  /** The addresses a mention may name — resolution-checked; a mention cannot dangle. */
  readonly mentionable?: ReadonlySet<string>;
  /**
   * The discoverable (`at`-locus) kinds the program declares. A mention naming one of
   * these whose member is not composed defers to `check` rather than refusing at emit.
   */
  readonly deferrableKinds?: ReadonlySet<string>;
  /**
   * The program's composed members by `kind:name` address — what an embedded value's
   * edge field resolves against to derive its target facts. An edge target never defers
   * to the gate the way a bare mention may: the facts are rendered into the projection
   * now, so an unresolved one has nothing true to place.
   */
  readonly members?: ReadonlyMap<string, Member>;
}

/** The {@link MentionScope} a set of {@link ResolveOptions} names — its two sets, each defaulting to empty. */
function scopeOf(options: ResolveOptions): MentionScope {
  return {
    mentionable: options.mentionable ?? new Set<string>(),
    deferrableKinds: options.deferrableKinds ?? new Set<string>(),
  };
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

/** Join path parts with `/`, dropping the empties and the `.` root a root-locus kind carries. */
function joinSlash(...parts: string[]): string {
  return parts.filter((part) => part !== "" && part !== ".").join("/");
}

/**
 * `name` spliced through `pattern`'s single `*` — the one name-through-a-glob map, shared
 * by a flat `at` glob and a host template's path pattern. A `*`-free pattern is a fixed
 * path, left verbatim. `starredSegment` admits a single-`*` segment glob whose one `*` stars
 * a whole leading directory segment (a starred-segment kind's locus), landing `<name>/<file>`;
 * every other caller passes `false`, where a `/` beside the `*` is a stray directory the
 * splice cannot place.
 *
 * # Throws
 * If `pattern` carries a `*` yet is neither single-star nor single-segment (and not the
 * admitted leading-segment case): the splice would leave a stray literal `*` or directory
 * segment behind.
 */
function spliceName(
  kindName: string,
  pattern: string,
  name: string,
  starredSegment: boolean,
): string {
  const stars = pattern.split("*").length - 1;
  const leadingSegment = starredSegment && stars === 1 && pattern.startsWith("*/");
  if (stars > 0 && !leadingSegment && (stars > 1 || pattern.includes("/"))) {
    throw new Error(
      `kind \`${kindName}\`: glob \`${pattern}\` is neither a single-segment single-\`*\` ` +
        `pattern nor an any-depth \`**\` glob — a member name splices through neither.`,
    );
  }
  return pattern.replace("*", name);
}

/**
 * The unit `host`'s file children compose their paths under — a directory unit's own
 * directory, since a template's path pattern is relative to the parent's unit.
 *
 * # Throws
 * If the host owns no directory unit: a lone file has no interior for a child to sit in.
 */
function hostUnit(host: Member, context: string): string {
  if (host.facts.locus.kind === "at" && host.facts.unitShape === "directory") {
    return joinSlash(host.facts.locus.root, host.name);
  }
  throw new Error(
    `${context}: its host \`${host.kind}:${host.name}\` owns no directory unit — a template's ` +
      `path pattern is relative to the host's unit, and a lone file has no interior for a ` +
      `child to sit in (specs/model/representation.md, "locus").`,
  );
}

/**
 * A nested file child's harness-relative locus: its host member's unit joined with the
 * host template's path pattern, its own name spliced through the pattern. The pattern is
 * the host kind's declared fact and the child kind governs no glob, so one home owns the
 * path and no child contends with its host's own locus.
 *
 * # Throws
 * If the child names no host, or its host's kind templates no file layer for the child's
 * kind — there is no pattern to compose against.
 */
function nestedFilePath(member: Member): string {
  const context = `member \`${member.name}\` of kind \`${member.kind}\``;
  const host = member.host;
  if (host === undefined) {
    throw new Error(`${context}: a nested file child names the host its path composes under.`);
  }
  const template = (host.facts.templates ?? []).find(
    (layer) => layer.kind.key === member.kind && layer.path !== undefined,
  );
  if (template?.path === undefined) {
    throw new Error(
      `${context}: its host \`${host.kind}:${host.name}\` templates no file layer for kind ` +
        `\`${member.kind}\` — the path pattern is the host kind's declared fact, and there is ` +
        `none to compose against (specs/model/representation.md, "locus").`,
    );
  }
  return joinSlash(hostUnit(host, context), spliceName(member.kind, template.path, member.name, false));
}

/**
 * The harness-relative locus `member` projects onto: a directory unit lands its entry
 * file under `<root>/<name>/`; a lone file splices the name through the glob's single
 * `*` (an any-depth glob — a memory kind's `**\/CLAUDE.md` — lands the root `<name>.md`,
 * and a `*`-free glob is a fixed path left verbatim); a nested file child composes its
 * path under its host's unit ({@link nestedFilePath}). The engine derives the same locus
 * from the same facts (`src/drift.rs`'s `member_projection_path`); the two must agree,
 * since a hook's rendered link is written from this side and reaped from that one.
 *
 * # Throws
 * If the kind is embedded (no standalone projection), or the member's glob or host
 * template pattern maps its name to no one path ({@link spliceName},
 * {@link nestedFilePath}).
 */
function projectionPath(member: Member): string {
  const facts = member.facts;
  if (facts.locus.kind === "embedded") {
    throw new Error(
      `kind \`${facts.name}\` is embedded — its members live inside a host body and ` +
        `carry no standalone projection (specs/model/representation.md, "locus").`,
    );
  }
  if (facts.locus.kind === "nested-file") return nestedFilePath(member);
  const { root, glob } = facts.locus;
  if (facts.unitShape === "directory") {
    const slash = glob.indexOf("/");
    return joinSlash(root, member.name, slash < 0 ? glob : glob.slice(slash + 1));
  }
  if (glob.includes("**")) return joinSlash(root, `${member.name}.md`);
  const starredSegment = facts.unitShape === "starred-segment";
  return joinSlash(root, spliceName(facts.name, glob, member.name, starredSegment));
}

/**
 * `to`'s path as read from the document at `from` — the shared leading segments drop
 * and each of `from`'s remaining directory segments becomes a `..`, so a rendered link
 * resolves from wherever the host member's own projection lands.
 */
function relativeProjection(from: string, to: string): string {
  const fromDirs = from.split("/").slice(0, -1);
  const toParts = to.split("/");
  let shared = 0;
  while (shared < fromDirs.length && shared < toParts.length - 1 && fromDirs[shared] === toParts[shared]) {
    shared += 1;
  }
  return [...fromDirs.slice(shared).map(() => ".."), ...toParts.slice(shared)].join("/");
}

/**
 * The closed, engine-derived facts about one embedded value's edge-field targets,
 * keyed by edge field: the declaring kind names which leaves are addresses, and each
 * address resolves against the program's composed members — the same table a mention
 * resolves against. The facts are read off the resolved target, so a format that
 * selects them renders a reference true by construction; the four are the whole set.
 *
 * An unfilled leaf is no edge, so it contributes no entry: requiredness is the kind's
 * own field schema, which fails in the author's program at compose time.
 *
 * # Throws
 * If a filled leaf names no composed member, or names one that owns no projection to
 * point at. An edge target cannot defer to the gate the way a bare mention may: the
 * reference is written now, and there is nothing true to write.
 */
function edgeTargetFacts(
  host: Member,
  value: EmbeddedMemberValue,
  leaves: Readonly<Record<string, string>>,
  options: ResolveOptions,
): Record<string, EdgeTargetFacts> {
  const targets: Record<string, EdgeTargetFacts> = {};
  const context = `member \`${host.name}\`: embedded value \`${value.key}\` of kind \`${value.kind}\``;
  for (const edge of value.edgeFields ?? []) {
    const address = leaves[edge.field];
    if (address === undefined || address === "") continue;
    // A one-element `to` set resolves a bare address within its one kind; a
    // multi-element set reads the kind-qualified `kind:name` the author wrote
    // (`EdgeField.to`). An already-qualified address carries its own colon, so
    // only a bare leaf is lifted to `${edge.to[0]}:${address}` for the lookup.
    const lookup = edge.to.length === 1 && !address.includes(":") ? `${edge.to[0]}:${address}` : address;
    const target = options.members?.get(lookup);
    if (target === undefined) {
      throw new Error(
        `${context}: edge field \`${edge.field}\` names \`${address}\`, which resolves to no ` +
          `composed member — an edge target's facts are derived, never fabricated ` +
          `(specs/model/pipeline.md, "Emit", the "Refusing" bullet).`,
      );
    }
    if (!isProjected(target)) {
      throw new Error(
        `${context}: edge field \`${edge.field}\` names \`${address}\`, which owns no ` +
          `projection to reference (specs/model/representation.md, "locus").`,
      );
    }
    targets[edge.field] = {
      name: target.name,
      address: lookup,
      kind: target.kind,
      path: relativeProjection(projectionPath(host), projectionPath(target)),
    };
  }
  return targets;
}

/**
 * Resolve one embedded member's value's leaves — top-level and each
 * collection entry's — to their final stored strings, and derive its edge fields'
 * target facts off the resolved leaves: a `Text`-authored leaf
 * resolves the way `resolveBody` resolves a member-level `Text` body (mention
 * resolution-checked against `mentionable`, loud on a dangling address); a
 * bare-string leaf is unchanged. The one resolution point shared by the
 * default TOML view and a kind's own `render` hook, so refusing on a dangling
 * embedded-kind leaf mention never depends on whether the kind declares
 * `render` (`pipeline.md`, "Emit", the "Refusing" bullet).
 */
function resolveMemberLeaves(
  host: Member,
  value: EmbeddedMemberValue,
  options: ResolveOptions,
): ResolvedEmbeddedMemberValue {
  const scope = scopeOf(options);
  const context = (childPath: string): string => `member.${value.kind} ${value.key}: leaf \`${childPath}\``;
  const leaves: Record<string, string> = {};
  for (const [key, leaf] of Object.entries(value.leaves)) {
    leaves[key] = resolveLeaf(leaf, scope, context(key));
  }
  const collections: Record<string, ResolvedEmbeddedMemberCollectionEntry[]> = {};
  for (const [collection, entries] of Object.entries(value.collections)) {
    collections[collection] = entries.map((entry) => {
      const entryLeaves: Record<string, string> = {};
      for (const [leaf, text] of Object.entries(entry.leaves)) {
        entryLeaves[leaf] = resolveLeaf(text, scope, context(`${collection}.${entry.key}.${leaf}`));
      }
      return { key: entry.key, leaves: entryLeaves };
    });
  }
  return {
    kind: value.kind,
    key: value.key,
    leaves,
    collections,
    targets: edgeTargetFacts(host, value, leaves, options),
  };
}

/**
 * Render one resolved embedded member's interior TOML: its top-level leaves,
 * then each collection's entries, in authored order, each its own
 * `[collection.entry]` table — the default view a `blocks()` value renders
 * with, when its originating kind declares no `render` hook (`kind.ts`).
 */
function renderMemberToml(value: ResolvedEmbeddedMemberValue): string {
  const lines: string[] = [];
  for (const [key, leaf] of Object.entries(value.leaves)) {
    lines.push(`${key} = ${tomlString(leaf)}`);
  }
  for (const [collection, entries] of Object.entries(value.collections)) {
    for (const entry of entries) {
      if (lines.length > 0) lines.push("");
      lines.push(`[${collection}.${entry.key}]`);
      for (const [leaf, text] of Object.entries(entry.leaves)) {
        lines.push(`${leaf} = ${tomlString(text)}`);
      }
    }
  }
  return lines.join("\n");
}

/**
 * Render one embedded member's value to its projected block. A `render`-less
 * kind projects the default `[collection.entry]` TOML view wrapped in a
 * `member.<kind> <key>` fence, byte-unchanged. A kind that declares a `render`
 * hook projects the hook's output directly, with no fence: an embedded format
 * is writer-only and unconstrained when its host is composed (`representation.md`,
 * "kind") — the engine never reads the block back (nested-member facts ride the
 * lock, `pipeline.md`, "The lock"), so the fence is cosmetic and a hook that
 * already renders readable markdown should not be re-buried in a code fence.
 * Leaves resolve once (`resolveMemberLeaves`) before either path sees them, so a
 * hook receives plain strings, never a raw `Text` leaf.
 */
function renderMemberBlock(host: Member, value: EmbeddedMemberValue, options: ResolveOptions): string {
  const resolved = resolveMemberLeaves(host, value, options);
  if (value.render !== undefined) return value.render(resolved);
  return `\`\`\`member.${value.kind} ${value.key}\n${renderMemberToml(resolved)}\n\`\`\``;
}

/**
 * A recording view of a resolved value: every read of an edge field's key — off the
 * derived `targets` facts or off the `leaves` that authored its address — is collected
 * into `placed`. Those two are the whole surface an edge's data can reach a format
 * through, so a format that touches neither placed nothing.
 *
 * Placement is observed as *selection*, which bounds the check in one direction only: a
 * format that reads an edge and discards it reads as placed. That keeps the predicate
 * free of false positives, which is what earns it the gate — a format that never names
 * the edge, the case the check exists for, is caught exactly.
 */
function recordingView(
  resolved: ResolvedEmbeddedMemberValue,
  edgeFields: ReadonlySet<string>,
  placed: Set<string>,
): ResolvedEmbeddedMemberValue {
  const watch = <T extends object>(record: T): T =>
    new Proxy(record, {
      get(target, property, receiver) {
        if (typeof property === "string" && edgeFields.has(property)) placed.add(property);
        return Reflect.get(target, property, receiver);
      },
    });
  return { ...resolved, leaves: watch(resolved.leaves), targets: watch(resolved.targets) };
}

/**
 * The declared edge fields one embedded value's format placed, sorted — the fact a
 * `format-places-edges` clause decides over, since the engine never sees a format and
 * never reads a rendering back. A `render`-less kind takes the default TOML view, which
 * writes every leaf, so every edge the value fills is placed by construction; a kind that
 * declares one runs the hook against a {@link recordingView} and reports what it
 * selected.
 *
 * The obligation ranges over the edges this value *fills*, never its kind's whole
 * declared set: an unfilled field is no edge, so a format cannot omit it. `undefined`
 * when the value fills none — there is nothing to place, so the row records nothing
 * rather than an empty column on every ordinary value.
 *
 * This renders the value a second time, the way `nestedMemberRow` reads its leaves a
 * second time: a hook is pure (emit double-verifies its own bytes), so the observing
 * render and the projecting one cannot disagree.
 */
function placedEdges(
  host: Member,
  value: EmbeddedMemberValue,
  options: ResolveOptions,
): string[] | undefined {
  if ((value.edgeFields ?? []).length === 0) return undefined;
  const resolved = resolveMemberLeaves(host, value, options);
  // `targets` carries exactly the filled edge fields — an unfilled one derives no facts.
  const edgeFields = new Set(Object.keys(resolved.targets));
  if (edgeFields.size === 0) return undefined;
  if (value.render === undefined) return [...edgeFields].sort(compareStrings);
  const placed = new Set<string>();
  value.render(recordingView(resolved, edgeFields, placed));
  return [...placed].sort(compareStrings);
}

/**
 * Every composed embedded value's placed edge fields, keyed by the value's
 * {@link placementKey} — what `emit` hands {@link compileDeclarations} so each
 * `nested_member` row carries its own format's placement record. Iterates exactly the
 * values `nestedMemberRows` does, so every edge-bearing row it builds has an observation.
 */
export function edgePlacements(harness: Harness, options: ResolveOptions): Map<string, string[]> {
  const placements = new Map<string, string[]>();
  for (const member of harness.members) {
    if (member.prose?.kind !== "blocks") continue;
    for (const value of member.prose.values) {
      if (isTextSpan(value)) continue;
      const placed = placedEdges(member, value, options);
      if (placed !== undefined) {
        placements.set(placementKey(`${member.kind}:${member.name}`, value.kind, value.key), placed);
      }
    }
  }
  return placements;
}

/**
 * The line count of a rendered block, matching the engine's `str::lines()`: a single
 * trailing newline is absorbed (a block and the same block plus one `\n` span the same),
 * and an empty block spans none. Kept in step with `src/extract.rs`'s file-side count so a
 * budget reads one member the same whether it is a file or an embedded projection.
 */
function renderedLineCount(block: string): number {
  if (block.length === 0) return 0;
  const body = block.endsWith("\n") ? block.slice(0, -1) : block;
  return body.split("\n").length;
}

/**
 * Every composed embedded value's rendered extent — the line and character count of the
 * block `emit` projected for it — keyed by its {@link placementKey}, what `emit` hands
 * {@link compileDeclarations} so each `nested_member` row carries the span an `extent`
 * clause budgets. Iterates exactly the values {@link edgePlacements} does, rendering each
 * through the same {@link renderMemberBlock} the body projection uses (a hook is pure, so
 * the measured render and the projected one cannot disagree), never a second renderer.
 *
 * A value the SDK composes is always rendered here, so it always captures a span; a value
 * no format rendered — an embedded member read off a layout host's source — is lowered by
 * the engine, not this pass, and reaches its row with no span (the `placed_edges`
 * distinction between an observed empty and an unobserved absence).
 */
export function renderedExtents(harness: Harness, options: ResolveOptions): Map<string, RenderedExtent> {
  const extents = new Map<string, RenderedExtent>();
  for (const member of harness.members) {
    if (member.prose?.kind !== "blocks") continue;
    for (const value of member.prose.values) {
      if (isTextSpan(value)) continue;
      const block = renderMemberBlock(member, value, options);
      extents.set(placementKey(`${member.kind}:${member.name}`, value.kind, value.key), {
        lines: renderedLineCount(block),
        // Unicode scalar values, matching Rust's `chars().count()` — iterating a string
        // yields code points, so a surrogate pair counts once, the way it does file-side.
        chars: [...block].length,
      });
    }
  }
  return extents;
}

/**
 * Render a member-level `Text` body to its final bytes: its mentions are
 * resolution-checked against `scope` ({@link checkMentions}: loud on a dangling
 * address, a discovery-locus one deferred; `context` naming the host) and the
 * display rule applied, each include slot left standing for the engine to splice.
 * Shared by a `text` body and a composed body's prose spans, so a narrative span
 * resolves the identical way a member-level `text` body does.
 *
 * # Throws
 * If a mention names no declared value and has no discovery locus.
 */
function renderTextBody(prose: Text, scope: MentionScope, context: string): string {
  checkMentions(prose.mentions, scope, context);
  return renderText(prose);
}

/**
 * Resolve a member's prose to its final body bytes: a `file()` asset is read in
 * byte-for-byte; a `text` body's mentions are resolution-checked (loud on a
 * dangling address) and rendered by the one display rule; a `blocks()` composed
 * body renders each child in authored order — a prose span as its resolved words
 * (`renderTextBody`), an embedded member as a `member.<kind> <key>` TOML fence
 * (or, for a kind with a `render` hook, the hook's fence-free markdown). The words
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
  const scope = scopeOf(options);
  if (prose.kind === "blocks") {
    const context = `member \`${member.name}\``;
    return (
      prose.values
        .map((value) =>
          isTextSpan(value) ? renderTextBody(value, scope, context) : renderMemberBlock(member, value, options),
        )
        .join("\n\n") + "\n"
    );
  }
  return renderTextBody(prose, scope, `member \`${member.name}\``);
}

/**
 * The one declare-side refusal emit runs before it produces a byte: a
 * `satisfies` claim naming no declared requirement (a dangling join).
 *
 * Fill enforcement — every `required` requirement has ≥1 satisfier — is the
 * engine's, not the SDK's: it lands over the composed members' `satisfies`
 * *plus* the fill rows emit derives from a layout document's `satisfies` edge
 * slot, which the SDK never reads. A pre-flight over composed `satisfies`
 * alone would spuriously refuse a requirement a layout host fills, so the SDK
 * implements no semantics here and defers to the engine's requirement clause.
 *
 * # Throws
 * On a dangling `satisfies` join.
 */
function refuseBrokenSource(harness: Harness): void {
  const requirements = declaredRequirements(harness);
  for (const member of harness.members) {
    for (const name of member.satisfies) {
      if (!requirements.has(name)) {
        throw new Error(
          `member \`${member.name}\`: \`satisfies\` claims requirement \`${name}\`, which no ` +
            `harness-level or member-published requirement declares — a dangling join ` +
            `(specs/model/pipeline.md, "Emit", the "Refusing" bullet).`,
        );
      }
    }
  }
}

/**
 * A fields-only registration member (a hook, an MCP server) surfaces embedded in a
 * host manifest, so it owns no standalone artifact — its facts erase into a
 * {@link RegistrationFact} for the manifest write face, never a projected member.
 */
function isRegistration(member: Member): boolean {
  return member.facts.shape === "fields";
}

/**
 * A member is projected iff it owns a file — at its kind's governed glob or composed
 * under its host's unit — and is not a fields-only registration member. An embedded
 * member and a registration member each carry no standalone projection.
 */
function isProjected(member: Member): boolean {
  return member.facts.locus.kind !== "embedded" && !isRegistration(member);
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

/**
 * One fields-only registration member erased for the manifest write face: its key
 * (a hook's lifecycle event, an MCP server's name), the collection address it keys
 * at, and its folded typed fields — the same declaration-row shape the engine write
 * face reads back off a manifest (`json_manifest.rs`'s `RegistrationMember`). Carried
 * from the composing program, never mined from a projection.
 */
export interface RegistrationFact {
  /** The erased registration kind — `hook`, `mcp-server` — joining `declarations.kinds`. */
  readonly kind: string;
  /** The member's key among its collection's entries — a hook's event, a server's name. */
  readonly key: string;
  /** The manifest collection address the registration surfaces at. */
  readonly collectionAddress: { readonly manifest: string; readonly keyPath: string };
  /** The member's folded typed fields, in the author's declared order. */
  readonly fields: ReadonlyArray<readonly [string, unknown]>;
}

/**
 * The harness's registration write facts as the public {@link RegistrationFact} view —
 * the seam's own `registration` rows mapped to the nested `collectionAddress` shape the
 * `EmitResult` sibling exposes, so the two cannot disagree on what a manifest carries.
 * The fields-only registration members ({@link registrationRows}) and the memberless tap
 * hooks a telemetry verifier synthesizes ({@link tapHookRows}) fold in together, the same
 * union `compileDeclarations` writes into `declarations.registrations`.
 *
 * # Throws
 * If a fields-only member declares no collection address — it surfaces in no manifest.
 */
function registrationFacts(harness: Harness): RegistrationFact[] {
  return [...registrationRows(harness), ...tapHookRows(harness)].map((row) => ({
    kind: row.kind,
    key: row.key,
    collectionAddress: { manifest: row.manifest, keyPath: row.key_path },
    fields: row.fields,
  }));
}

/**
 * One harness-level settings-residue key erased for the manifest write face: the manifest
 * it surfaces in, its opaque key, and its value — the entry `emit` folds into the manifest's
 * residue beside its registration members' collection segments. Carried from the composing
 * program, never mined from a projection.
 */
export interface SettingsResidue {
  /** The host manifest the residue key surfaces in (`settings.json`). */
  readonly manifest: string;
  /** The opaque top-level manifest key with no member home. */
  readonly key: string;
  /** The key's opaque value, placed verbatim into the manifest's residue. */
  readonly value: unknown;
}

/**
 * The harness's residual settings keys as the public {@link SettingsResidue} view — the
 * seam's own `settings` rows ({@link settingsRows}) surfaced under the `EmitResult` sibling,
 * so the two cannot disagree on what a manifest's residue carries. Key-sorted, the same
 * byte-stable order the seam family takes.
 */
function settingsResidue(harness: Harness): SettingsResidue[] {
  return settingsRows(harness).map((row) => ({ manifest: row.manifest, key: row.key, value: row.value }));
}

/**
 * The harness's composed members by `kind:name` address — the table an embedded value's
 * edge field resolves its target against. Keyed the identical way {@link declaredAddresses}
 * spells a member address, so an edge field and a mention name a member the same way.
 */
function memberTable(harness: Harness): Map<string, Member> {
  return new Map(harness.members.map((member) => [`${member.kind}:${member.name}`, member]));
}

/** The harness's projected members as payload members, deterministically kind-then-name ordered. */
function orderedMembers(harness: Harness, options: ResolveOptions): PayloadMember[] {
  return [...harness.members]
    .filter(isProjected)
    .sort((a, b) => compareStrings(a.kind, b.kind) || compareStrings(a.name, b.name))
    .map((member) => ({
      kind: member.kind,
      name: member.name,
      host: member.host && `${member.host.kind}:${member.host.name}`,
      // The generated row carries a mutable field list; the member's is read-only,
      // so copy each pair into a fresh tuple — the same values, a shape the row accepts.
      fields: member.fields.map(([name, value]): [string, unknown] => [name, value]),
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
  /**
   * The fields-only registration members erased for the manifest write face — each
   * a name, its collection address, and its folded fields. Folds into the manifest
   * artifacts once the engine write face lands; carried here as data until then, the
   * way `permissions` is.
   */
  readonly registrations: readonly RegistrationFact[];
  /**
   * The harness-level settings residue erased for the manifest write face — each an opaque
   * settings.json key and its value. Folds into the settings.json manifest's residue at
   * emit, the way `registrations` builds its collection segments; carried here as data too.
   */
  readonly settings: readonly SettingsResidue[];
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
    deferrableKinds: declaredAtLocusKinds(harness),
    members: memberTable(harness),
  };
  const compile = (): EmitResult => {
    const members = orderedMembers(harness, resolve);
    const declarations = compileDeclarations(
      harness,
      edgePlacements(harness, resolve),
      renderedExtents(harness, resolve),
    );
    return {
      declarations,
      members,
      seam: encodeSeam({ declarations, members }),
      permissions: permissionUnion(harness.members.flatMap((member) => [...member.needs])),
      registrations: registrationFacts(harness),
      settings: settingsResidue(harness),
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
