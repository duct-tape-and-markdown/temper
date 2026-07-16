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
import type { EmbeddedMemberValue, Member, ResolvedEmbeddedMemberCollectionEntry, ResolvedEmbeddedMemberValue } from "./kind.js";
import type { MentionScope, Text } from "./prose.js";
import { checkMentions, isTextSpan, renderText, resolveLeaf } from "./prose.js";
import { permissionUnion } from "./needs.js";
import type { Declarations } from "./declarations.js";
import {
  compareStrings,
  compileDeclarations,
  declaredAddresses,
  declaredAtLocusKinds,
  declaredRequirements,
  encodeSeam,
  registrationRows,
  settingsRows,
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

/**
 * Resolve one embedded member's value's leaves — top-level and each
 * collection entry's — to their final stored strings: a `Text`-authored leaf
 * resolves the way `resolveBody` resolves a member-level `Text` body (mention
 * resolution-checked against `mentionable`, loud on a dangling address); a
 * bare-string leaf is unchanged. The one resolution point shared by the
 * default TOML view and a kind's own `render` hook, so refusing on a dangling
 * embedded-kind leaf mention never depends on whether the kind declares
 * `render` (`pipeline.md`, "Emit", the "Refusing" bullet).
 */
function resolveMemberLeaves(value: EmbeddedMemberValue, scope: MentionScope): ResolvedEmbeddedMemberValue {
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
  return { kind: value.kind, key: value.key, leaves, collections };
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
function renderMemberBlock(value: EmbeddedMemberValue, options: ResolveOptions): string {
  const resolved = resolveMemberLeaves(value, scopeOf(options));
  if (value.render !== undefined) return value.render(resolved);
  return `\`\`\`member.${value.kind} ${value.key}\n${renderMemberToml(resolved)}\n\`\`\``;
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
        .map((value) => (isTextSpan(value) ? renderTextBody(value, scope, context) : renderMemberBlock(value, options)))
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
 * A member is projected iff its kind lives at a path locus and is not a fields-only
 * registration member — an embedded member and a registration member each carry no
 * standalone projection.
 */
function isProjected(member: Member): boolean {
  return member.facts.locus.kind === "at" && !isRegistration(member);
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
 * The harness's fields-only registration members as the public {@link RegistrationFact}
 * view — the seam's own `registration` rows ({@link registrationRows}) mapped to the
 * nested `collectionAddress` shape the `EmitResult` sibling exposes, so the two cannot
 * disagree on what a manifest carries. Kind-then-key ordered so double emit is byte-stable.
 *
 * # Throws
 * If a fields-only member declares no collection address — it surfaces in no manifest.
 */
function registrationFacts(harness: Harness): RegistrationFact[] {
  return registrationRows(harness).map((row) => ({
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

/** The harness's projected members as payload members, deterministically kind-then-name ordered. */
function orderedMembers(harness: Harness, options: ResolveOptions): PayloadMember[] {
  return [...harness.members]
    .filter(isProjected)
    .sort((a, b) => compareStrings(a.kind, b.kind) || compareStrings(a.name, b.name))
    .map((member) => ({
      kind: member.kind,
      name: member.name,
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
  };
  const compile = (): EmitResult => {
    const members = orderedMembers(harness, resolve);
    const declarations = compileDeclarations(harness);
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
