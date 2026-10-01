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
  KindFacts,
  Member,
  ResolvedEmbeddedMemberCollectionEntry,
  ResolvedEmbeddedMemberValue,
} from "./kind.js";
import type { MentionScope, Text } from "./prose.js";
import { checkMentions, defersToGate, isTextSpan, renderText, resolveLeaf } from "./prose.js";
import { permissionUnion } from "./needs.js";
import type { Declarations, RenderedExtent } from "./declarations.js";
import {
  compareStrings,
  compileDeclarations,
  declaredAddresses,
  declaredAtLocusKinds,
  declaredRequirements,
  encodeSeam,
  mergedRegistrationRows,
  settingsRows,
  uniqueMap,
} from "./declarations.js";
import type { PayloadMember } from "./generated/index.js";
import {
  bareLookupKey,
  edgeLookupKey,
  hostAddress,
  isOneSegment,
  memberAddress,
  nestedAddress,
} from "./member-address.js";

// The projected-member shape is the generated `ts-rs` binding, re-exported so the
// public face keeps the name — a Rust-side member-column rename is a compile
// error here, never a silent shape drift.
export type { PayloadMember } from "./generated/index.js";

/**
 * One composed embedded value as an edge target, carried with the host member whose body
 * it lives in: an embedded member owns no file, so every path fact about it is derived
 * from its host's own projection.
 */
export interface EmbeddedTarget {
  /** The member whose composed body carries the value — the projection it lives in. */
  readonly host: Member;
  /** The composed value itself. */
  readonly value: EmbeddedMemberValue;
}

/**
 * What one address in the member table names: a top-level composed member, or the
 * embedded values one nested spelling reaches. The nested list carries one element for
 * a full `<host-address>/<kind>/<key>` address, and one per host for a bare `kind:key` —
 * a bare key names a nested member only when a single host carries it, and resolution
 * refuses the rest as ambiguous rather than picking one: uniqueness is the resolver's
 * bar, not the grammar's.
 */
export type EdgeTarget = Member | readonly EmbeddedTarget[];

/** What a mention may resolve against at emit. */
export interface ResolveOptions {
  /** The addresses a mention may name — resolution-checked; a mention cannot dangle. */
  readonly mentionable?: ReadonlySet<string>;
  /**
   * The discoverable (`at`-locus) kinds the program declares. A reference naming one of
   * these whose member is not composed defers to `check` rather than refusing at emit —
   * a mention and an embedded value's edge target alike.
   */
  readonly deferrableKinds?: ReadonlySet<string>;
  /**
   * The program's composed members by address — what an embedded value's edge field
   * resolves against to derive its target facts. Top-level members index at their
   * `kind:name` address, and each composed embedded value at both of its own spellings:
   * its full `<host-address>/<kind>/<key>` address and its bare `kind:key`.
   * Refusal reaches exactly as far as the program's own universe, the one rule a mention
   * and an edge target share: an address this table misses refuses, unless it names a kind
   * {@link deferrableKinds} carries — then the member may be discovered on disk, and the
   * field defers to `check` with no facts derived.
   */
  readonly members?: ReadonlyMap<string, EdgeTarget>;
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
 * `name` spliced through `pattern`'s lone `*` — the one name-through-a-glob rule, and the
 * only one: every flat `at` glob and every host template's path pattern places through it.
 *
 * A leading `**\/` collapses to zero segments, because an any-depth prefix names where the
 * glob *matches*, never where a projection *lands*. What remains is placement: literal
 * segments verbatim, the name into the final segment's lone `*` (`**\/CLAUDE.md` → the
 * fixed `CLAUDE.md`, `**\/sub/*.json` → `sub/<name>.json`), and a `*`-free pattern wholly
 * fixed. `starredSegment` is the one admission on top: a `*\/<file>` glob whose `*` stars a
 * whole leading directory segment, landing `<name>/<file>` — there the name identifies the
 * directory, not the leaf. Every other caller passes `false`.
 *
 * # Throws
 * If what remains after the collapse carries more than one `*`, or its one `*` sits above
 * the final segment and is not the admitted starred-segment case: the splice would leave a
 * stray literal `*` behind, or star a directory the member name does not identify.
 */
function spliceName(
  kindName: string,
  pattern: string,
  name: string,
  starredSegment: boolean,
): string {
  const placed = pattern.startsWith("**/") ? pattern.slice(3) : pattern;
  const stars = placed.split("*").length - 1;
  const starInLeaf = placed.lastIndexOf("*") > placed.lastIndexOf("/");
  const leadingSegment = starredSegment && stars === 1 && placed.startsWith("*/");
  if (stars > 0 && !leadingSegment && (stars > 1 || !starInLeaf)) {
    throw new Error(
      `kind \`${kindName}\`: glob \`${pattern}\` maps its member name onto no one path — a ` +
        `name splices through exactly one \`*\`, confined to the glob's final segment (a ` +
        `leading \`**/\` collapses to nothing, and literal leading segments are fixed ` +
        `placement); this glob carries more than one \`*\`, or a \`*\` above the leaf.`,
    );
  }
  return placed.replace("*", name);
}

/**
 * The placement a member of `facts` named `name` takes **inside the locus its pattern is
 * rooted at** — the half an `at` kind and a nested file child derive identically, since a
 * host template's path pattern stands to its host's unit exactly as an `at` kind's glob
 * stands to its `governs` root (`specs/model/representation.md`, "locus").
 *
 * A **directory** unit owns a directory and seats its entry file inside it, so the pattern
 * names that entry file and the member's own name is the directory: the leading segment
 * drops and `<name>/` takes its place, which is what makes `*\/SKILL.md` and a template's
 * `*\/PAGE.md` land the same shape. Every other shape places its name through the one
 * splice rule ({@link spliceName}), a starred-segment kind's `*\/<file>` included.
 *
 * # Throws
 * Whatever {@link spliceName} throws when the pattern maps the name onto no one path.
 */
function placementInUnit(facts: KindFacts, pattern: string, name: string): string {
  if (facts.unitShape === "directory") {
    const slash = pattern.indexOf("/");
    return joinSlash(name, slash < 0 ? pattern : pattern.slice(slash + 1));
  }
  return spliceName(facts.name, pattern, name, facts.unitShape === "starred-segment");
}

/**
 * The unit `host`'s file children compose their paths under — a directory unit's own
 * directory, since a template's path pattern is relative to the parent's unit.
 *
 * Composed from the host's **own projection**, never read off its locus columns: a
 * directory unit seats its entry file inside the directory it owns, so the directory is
 * that projection less its final segment. A host at an `at` locus supplies
 * `<root>/<name>`; a host that is itself a nested file child supplies an interior under
 * *its* own host's unit, so depth is unbounded and one rule composes every layer —
 * exactly as the engine's `nested_file_path` (`src/drift.rs`) composes it.
 *
 * # Throws
 * If the host owns no directory unit: a lone file has no interior for a child to sit in.
 * Propagates every refusal the host's own path derivation raises.
 */
function hostUnit(host: Member, context: string): string {
  if (host.facts.unitShape !== "directory") {
    throw new Error(
      `${context}: its host \`${memberAddress(host)}\` owns no directory unit — a template's ` +
        `path pattern is relative to the host's unit, and a lone file has no interior for a ` +
        `child to sit in (specs/model/representation.md, "locus").`,
    );
  }
  const projection = projectionPath(host);
  const slash = projection.lastIndexOf("/");
  if (slash < 0) {
    throw new Error(
      `${context}: its host \`${memberAddress(host)}\` projects to \`${projection}\`, which names ` +
        `no directory for a child to sit in (specs/model/representation.md, "locus").`,
    );
  }
  return projection.slice(0, slash);
}

/**
 * A nested file child's harness-relative locus: its host member's unit joined with the
 * host template's path pattern, its own name placed through the pattern
 * ({@link placementInUnit}). The pattern is the host kind's declared fact and the child
 * kind governs no glob, so one home owns the path and no child contends with its host's
 * own locus.
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
      `${context}: its host \`${memberAddress(host)}\` templates no file layer for kind ` +
        `\`${member.kind}\` — the path pattern is the host kind's declared fact, and there is ` +
        `none to compose against (specs/model/representation.md, "locus").`,
    );
  }
  return joinSlash(hostUnit(host, context), placementInUnit(member.facts, template.path, member.name));
}

/**
 * The harness-relative locus `member` projects onto: a file member places its name inside
 * the locus its own `governs` glob is rooted at ({@link placementInUnit}); a nested file
 * child places it inside its host's unit instead, through that same rule and its host
 * kind's declared template pattern ({@link nestedFilePath}). The engine
 * derives the same locus from the same facts (`src/drift.rs`'s `member_projection_path`);
 * the two must agree, since a hook's rendered link is written from this side and reaped
 * from that one.
 *
 * The property both derivations owe is a **round trip**: the path a member is placed at is
 * one the kind's own glob finds again when `check` walks the locus, so placement and
 * discovery are one agreement rather than two spellings that happen to coincide. A path
 * outside its glob is written and locked yet ungoverned — the kind counted `(0)` with no
 * finding to name it. The engine holds the round trip with the glob engine it already has
 * (`UngovernedProjection`); this side has no matcher, so what it holds is the half a
 * matcher is not needed for — a member name carrying the `/` a path is placed with
 * ({@link refuseSegmentedName}) — and `tests/projection_path_seam.rs` gates the property
 * itself, per unit shape, over what emit actually wrote.
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
  return joinSlash(root, placementInUnit(facts, glob, member.name));
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
 * own field schema, which fails in the author's program at compose time. A filled leaf
 * whose address {@link defersToGate} admits derives none either — the program's own
 * universe does not reach it, so the address rides the lock as authored and `check` owns
 * its route verdict, exactly as a dangling mention defers (`pipeline.md`, "Emit", the
 * "Refusing" bullet). Such a field is reported by name in {@link EdgeTargets.deferred}
 * rather than dropped, so reading it refuses by name ({@link deferringTargets}) instead
 * of surfacing as an absent key.
 *
 * # Throws
 * If a filled leaf names no composed member and no declared `at`-locus kind either, names
 * one that owns no projection to point at, or names a bare nested key several hosts carry
 * ({@link resolvedTargetFacts}).
 */
function edgeTargetFacts(
  host: Member,
  value: EmbeddedMemberValue,
  leaves: Readonly<Record<string, string>>,
  options: ResolveOptions,
): EdgeTargets {
  const facts: Record<string, EdgeTargetFacts> = {};
  const deferred = new Map<string, string>();
  const { deferrableKinds } = scopeOf(options);
  const context = `member \`${host.name}\`: embedded value \`${value.key}\` of kind \`${value.kind}\``;
  for (const edge of value.edgeFields ?? []) {
    const address = leaves[edge.field];
    if (address === undefined || address === "") continue;
    const lookup = edgeLookupKey(address, edge.to);
    const target = options.members?.get(lookup);
    const reference = `${context}: edge field \`${edge.field}\` names \`${address}\``;
    if (target === undefined) {
      // Judged on the address the author wrote, never `lookup`: the one-element-`to` lift
      // spells a bare name as a host address for the member table alone, and a bare name
      // names no discoverable member.
      if (defersToGate(address, deferrableKinds)) {
        deferred.set(edge.field, reference);
        continue;
      }
      throw new Error(
        `${reference}, which resolves to no composed member — an edge target's facts are ` +
          `derived, never fabricated (specs/model/pipeline.md, "Emit", the "Refusing" bullet).`,
      );
    }
    facts[edge.field] = resolvedTargetFacts(host, target, reference);
  }
  return { facts, deferred };
}

/**
 * What {@link edgeTargetFacts} derived for one embedded value: the resolved edge fields'
 * facts, and the fields it deliberately derived nothing for — each keyed to the reference
 * naming its host member, its value's kind and key, the edge field and the authored
 * address, so the refusal a read of one raises can say all of it.
 */
interface EdgeTargets {
  readonly facts: Record<string, EdgeTargetFacts>;
  readonly deferred: ReadonlyMap<string, string>;
}

/**
 * The `targets` view a `render` hook receives: the resolved facts, plus a trap that
 * refuses by name on a deferred edge field. Emit derives no facts for a target `check`
 * owns, and a format cannot spell a reference off facts that do not exist — so the read
 * is the failure, and it names the member, the value, the field and the address rather
 * than surfacing as a `TypeError` off an absent key in the author's own template
 * (invariant 6, "Loud or nothing").
 *
 * Only a string key in the deferred set is trapped: a symbol get — inspection, a `then`
 * probe — reads through untouched. `ownKeys` is deliberately untrapped, so the deferred
 * field stays invisible to {@link placedEdges}'s `Object.keys` and earns no
 * `format-places-edges` obligation.
 */
function deferringTargets(targets: EdgeTargets): Readonly<Record<string, EdgeTargetFacts>> {
  if (targets.deferred.size === 0) return targets.facts;
  return new Proxy(targets.facts, {
    get(record, property, receiver) {
      const reference = typeof property === "string" ? targets.deferred.get(property) : undefined;
      if (reference !== undefined) {
        throw new Error(
          `${reference}, whose target defers to \`check\` — emit derives no facts for it, so a ` +
            `format cannot spell a reference off it (specs/model/pipeline.md, "Emit", the ` +
            `"Refusing" bullet).`,
        );
      }
      return Reflect.get(record, property, receiver);
    },
  });
}

/** Whether an address named the nested index — the embedded values one spelling reaches. */
function isNested(target: EdgeTarget): target is readonly EmbeddedTarget[] {
  return Array.isArray(target);
}

/**
 * The four derived facts one resolved edge target contributes, read off the target
 * itself and never off the citing instance. A member answers with its own identity, its own
 * {@link memberAddress} — the canonical one, whichever of its spellings the leaf authored,
 * so a file child cited by its bare key still renders the host-qualified address it
 * resolves at — and its own projection; an embedded value answers with its own kind and
 * key, its canonical `<host-address>/<kind>/<key>` address, and its *host's* projection —
 * an embedded member owns no file, so the file its rendering lands in is the host's.
 *
 * # Throws
 * If a bare `kind:key` is carried by more than one host — an ambiguous address names
 * nothing, and the full spelling is what tells the carriers apart — or if the target,
 * or the host carrying it, owns no projection to point at.
 */
function resolvedTargetFacts(host: Member, target: EdgeTarget, reference: string): EdgeTargetFacts {
  const noProjection = (): Error =>
    new Error(`${reference}, which owns no projection to reference (specs/model/representation.md, "locus").`);
  if (!isNested(target)) {
    if (!isProjected(target)) throw noProjection();
    return {
      name: target.name,
      address: memberAddress(target),
      kind: target.kind,
      path: relativeProjection(projectionPath(host), projectionPath(target)),
      repoRootedPath: projectionPath(target),
    };
  }
  if (target.length > 1) {
    const hosts = target.map((carrier) => `\`${hostAddress(carrier.host.kind, carrier.host.name)}\``).join(", ");
    throw new Error(
      `${reference}, a bare key ${target.length} hosts carry (${hosts}) — a nested member's address ` +
        `composes through its host, so spell the whole \`<host-address>/<kind>/<key>\` ` +
        `(specs/model/representation.md, "member").`,
    );
  }
  const { host: carrier, value } = target[0]!;
  if (!isProjected(carrier)) throw noProjection();
  const carrierPath = projectionPath(carrier);
  return {
    name: value.key,
    address: nestedAddress(memberAddress(carrier), value.kind, value.key),
    kind: value.kind,
    path: relativeProjection(projectionPath(host), carrierPath),
    repoRootedPath: carrierPath,
  };
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
    targets: deferringTargets(edgeTargetFacts(host, value, leaves, options)),
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
 * The obligation ranges over the edges this value *fills and resolves*, never its kind's
 * whole declared set: an unfilled field is no edge, so a format cannot omit it, and a
 * field whose target defers to `check` carries no facts to place — the route verdict is
 * the gate's, not this clause's. `undefined` when the value leaves the set empty — there
 * is nothing to place, so the row records nothing rather than an empty column on every
 * ordinary value.
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
  // `targets` carries exactly the filled, resolved edge fields — an unfilled one and a
  // deferred one each derive no facts.
  const edgeFields = new Set(Object.keys(resolved.targets));
  if (edgeFields.size === 0) return undefined;
  if (value.render === undefined) return [...edgeFields].sort(compareStrings);
  const placed = new Set<string>();
  value.render(recordingView(resolved, edgeFields, placed));
  return [...placed].sort(compareStrings);
}

/**
 * Every composed embedded value's placed edge fields, keyed by the value's
 * {@link nestedAddress} — what `emit` hands {@link compileDeclarations} so each
 * `nested_member` row carries its own format's placement record. Iterates exactly the
 * values `nestedMemberRows` does, so every edge-bearing row it builds has an observation.
 */
function edgePlacements(harness: Harness, options: ResolveOptions): Map<string, string[]> {
  const entries: Array<[string, string[]]> = [];
  for (const member of harness.members) {
    if (member.prose?.kind !== "blocks") continue;
    for (const value of member.prose.values) {
      if (isTextSpan(value)) continue;
      const placed = placedEdges(member, value, options);
      if (placed !== undefined) {
        entries.push([nestedAddress(memberAddress(member), value.kind, value.key), placed]);
      }
    }
  }
  return uniqueMap(entries);
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
 * block `emit` projected for it — keyed by its {@link nestedAddress}, what `emit` hands
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
function renderedExtents(harness: Harness, options: ResolveOptions): Map<string, RenderedExtent> {
  const entries: Array<[string, RenderedExtent]> = [];
  for (const member of harness.members) {
    if (member.prose?.kind !== "blocks") continue;
    for (const value of member.prose.values) {
      if (isTextSpan(value)) continue;
      const block = renderMemberBlock(member, value, options);
      entries.push([
        nestedAddress(memberAddress(member), value.kind, value.key),
        {
          lines: renderedLineCount(block),
          // Unicode scalar values, matching Rust's `chars().count()` — iterating a string
          // yields code points, so a surrogate pair counts once, the way it does file-side.
          chars: [...block].length,
        },
      ]);
    }
  }
  return uniqueMap(entries);
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
  /** The collection key the entry writes under — a hook's event, a server's name. Not
   * the member's own name where the two differ: a hook's name joins its matcher. */
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
 * The rows are {@link mergedRegistrationRows}' — the fields-only registration members
 * with the memberless tap hooks a telemetry verifier synthesizes joined into them — the
 * one home `compileDeclarations` also reads for `declarations.registrations`, so a tap
 * that joined an authored group did so for both faces or for neither.
 *
 * # Throws
 * If a fields-only member declares no collection address — it surfaces in no manifest.
 */
function registrationFacts(harness: Harness): RegistrationFact[] {
  return mergedRegistrationRows(harness).map((row) => ({
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
 * The harness's composed members by address — the table an embedded value's edge field
 * resolves its target against. Every member keys by its own address ({@link memberAddress}),
 * the identical way {@link declaredAddresses} spells one, so an edge field and a mention
 * name a member the same way; a nested member keys under both of its own spellings — an
 * embedded value through {@link nestedTargets}, a file child through its own address here
 * and {@link bareFileChildTargets} — so a nested edge target resolves at emit as a
 * top-level one does.
 *
 * A projected member's address is its file, so two at one address are a collision and
 * refuse loud. A nested file child's address carries its host, so two hosts each carrying a
 * `home` are two addresses rather than one name twice — the collision a corpus-wide
 * `<kind>:<name>` keying raised for a corpus that is perfectly well-formed. A registration
 * member's address is its *group key* — a `hook` registers on its event, and Claude Code
 * admits any number of matcher groups per event — so several legitimately share one address
 * and the table keeps the last composed (the pre-0.0.16 reading; an edge targeting that
 * shared address resolves ambiguously, the open fork `(hook-member-identity)` in
 * `.flume/plan/open-questions.md`). Refusing them broke `emit` on every harness with two
 * hooks on one event (0.0.16).
 */
function memberTable(harness: Harness): Map<string, EdgeTarget> {
  const table = uniqueMap([
    ...harness.members
      .filter((member) => !isRegistration(member))
      .map((member) => [memberAddress(member), member] as [string, EdgeTarget]),
    ...bareFileChildTargets(harness),
    ...nestedTargets(harness),
  ]);
  for (const member of harness.members.filter(isRegistration)) {
    table.set(memberAddress(member), member);
  }
  return table;
}

/**
 * Every nested **file** child under the bare `<kind>:<name>` short form as well as its own
 * host-qualified address — the same second spelling {@link nestedTargets} gives an embedded
 * member, so the two nested grains resolve alike.
 *
 * A bare key names a nested member only while a single host carries it, so a name several
 * hosts spell enters under no bare key at all and an edge citing it refuses as an address
 * resolving to no composed member ({@link edgeTargetFacts}) — loud, with the whole
 * `<host-address>/<kind>/<key>` spelling always available to tell the carriers apart. The
 * ambiguity is not refused *here*, because uniqueness is the resolver's bar and not the
 * corpus's: one name two hosts carry and nothing cites still composes.
 */
function bareFileChildTargets(harness: Harness): Array<[string, EdgeTarget]> {
  const carriers = new Map<string, Member[]>();
  for (const member of harness.members) {
    if (member.host === undefined || isRegistration(member)) continue;
    const key = bareLookupKey(member.kind, member.name);
    const carried = carriers.get(key);
    if (carried === undefined) carriers.set(key, [member]);
    else carried.push(member);
  }
  return [...carriers]
    .filter(([, carried]) => carried.length === 1)
    .map(([key, carried]) => [key, carried[0]!] as [string, EdgeTarget]);
}

/**
 * Every composed embedded value as member-table entries, under both of its spellings: the
 * full `<host-address>/<kind>/<key>` address, whose one carrier is the host whose body
 * composed it — two of those coincident are a malformed lock, refused by the shared
 * {@link uniqueMap} the way two members at one address are — and the bare `kind:key`,
 * whose entry carries *every* host that spells it. A bare key names a nested member only
 * when a single host carries it; several is ambiguous, refused by name at resolution
 * ({@link resolvedTargetFacts}) rather than here, since uniqueness is the resolver's bar
 * and not the corpus's — one key two hosts carry and nothing cites still composes.
 */
function nestedTargets(harness: Harness): Array<[string, EdgeTarget]> {
  const qualified: Array<[string, EdgeTarget]> = [];
  const bare = new Map<string, EmbeddedTarget[]>();
  for (const member of harness.members) {
    if (member.prose?.kind !== "blocks") continue;
    for (const value of member.prose.values) {
      if (isTextSpan(value)) continue;
      const target: EmbeddedTarget = { host: member, value };
      qualified.push([nestedAddress(memberAddress(member), value.kind, value.key), [target]]);
      const key = bareLookupKey(value.kind, value.key);
      const carriers = bare.get(key);
      if (carriers === undefined) bare.set(key, [target]);
      else carriers.push(target);
    }
  }
  return [...qualified, ...bare];
}

/**
 * Refuse a projected member whose **name** carries the `/` an address is cut at
 * ({@link isOneSegment}) — the bar `kind.ts`'s `refuseSegmentedKey` already holds one grain
 * down, for an embedded member's key.
 *
 * A member's name is its own first address segment and, at every `at` locus, the identity a
 * projection path is spliced from: a file stem, or the one directory segment the kind's glob
 * stars. A `/` in it therefore does two things at once — it shifts every address beneath the
 * member by a segment, and it places the projection outside the glob the kind declares, so
 * the file is written where no discovery walk looks. The engine refuses the same name
 * (`src/drift.rs`'s `MemberNameSeparator`), and this half is the loud one: at compose time
 * the author can still rename.
 *
 * Total over the projected set rather than inside {@link spliceName}, which a directory-unit
 * member's path never reaches.
 *
 * # Throws
 * If `name` is empty or carries `/`.
 */
function refuseSegmentedName(kind: string, name: string): void {
  if (isOneSegment(name)) return;
  throw new Error(
    `member \`${name}\` of kind \`${kind}\`: a member's name is one address segment — ` +
      `non-empty and carrying no \`/\`, the separator its address is cut at and the one a ` +
      `projection path is placed with; a name carrying it addresses another member's leaf ` +
      `and places outside the kind's own glob`,
  );
}

/** The harness's projected members as payload members, deterministically kind-then-name ordered. */
function orderedMembers(harness: Harness, options: ResolveOptions): PayloadMember[] {
  return [...harness.members]
    .filter(isProjected)
    .sort((a, b) => compareStrings(a.kind, b.kind) || compareStrings(a.name, b.name))
    .map((member) => {
      refuseSegmentedName(member.kind, member.name);
      return {
        kind: member.kind,
        name: member.name,
        // The host's **own whole address**, never its `kind:name` alone: the engine reads
        // this column to find the host's host when it composes a child's path, so a
        // flattened spelling would strand every layer above the nearest one.
        host: member.host && memberAddress(member.host),
        // The generated row carries a mutable field list; the member's is read-only,
        // so copy each pair into a fresh tuple — the same values, a shape the row accepts.
        fields: member.fields.map(([name, value]): [string, unknown] => [name, value]),
        body: resolveBody(member, options),
        source_path: fileSourcePath(member),
      };
    });
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
