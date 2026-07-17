/**
 * Kinds — the engine room. A kind is a plain typed surface — an interface
 * `T` and a constructor `kind<T>()` — plus seven facts of runtime residue: label,
 * locus, projection, registration, edge fields, content, and template. A registration kind (a hook,
 * an MCP server) extends the content fact with a fields-only `shape` and a
 * `collectionAddress` naming the host manifest it surfaces in. `tsc` is the keystroke
 * wall; every type erases at the seam, and what a kind leaves behind rides the lock as
 * rows. Identity travels by import, never by string — a `kind` reference is the imported
 * value.
 */

import type { Prose, Text } from "./prose.js";
import type { Capability } from "./needs.js";
import type { Requirement } from "./contract.js";

/** The shape of the on-disk artifact a member projects to (fact 3, projection). */
export type Format = "yaml-frontmatter";

/**
 * Whether a member is a lone file (identity from the stem), a directory with an
 * entry file (identity from the directory name), or a lone file whose identity is
 * read from a declared frontmatter field (`identityField`) instead of derived from
 * the path (an agent's `name`).
 */
export type UnitShape = "file" | "directory" | "named-field";

/**
 * One **channel** a kind's registration declares — a documented way a member
 * reaches the world (fact 4, `builtins.md`, "The shipped kinds": "user
 * invocation and description trigger are channels, not rivals"). Reachability
 * is graph reachability from the world node, OR'd across a member's declared
 * channel set — live on any one channel is live.
 */
export type Registration =
  | { readonly via: "always" }
  | { readonly via: "user-invoked" }
  | { readonly via: "description-trigger"; readonly field: string }
  | { readonly via: "paths-match"; readonly field: string }
  | { readonly via: "event"; readonly field: string }
  | { readonly via: "connection" }
  | { readonly via: "enablement" };

/** One of a kind's fields that is a reference to another member — a graph edge (fact 5). */
export interface EdgeField {
  readonly field: string;
  /**
   * The far end of the edge: the non-empty **set** of kinds the field may target.
   * A one-element set resolves a bare address within its one kind; a multi-element
   * set demands the kind-qualified `kind:name` address always, since resolution
   * reads the written text and never infers from the member population. The
   * non-emptiness is the type's to hold — an edge declaring no target kind can
   * never resolve.
   */
  readonly to: readonly [string, ...string[]];
}

/**
 * A kind's **locus** (fact 2), in three spellings: members live at path globs (`at`),
 * as typed fenced blocks inside host documents (`embedded`), or as `nested-file`
 * children owning a file whose path composes from their host member's unit and the host
 * kind's template pattern. An `at` locus is split root + glob so the kind fact row
 * carries `governs_root`/`governs_glob` directly; a `nested-file` kind declares neither
 * — the pattern is the host {@link Template}'s one home, so a child can never collide
 * with its host's own governs glob. An `embedded` locus names no host: which types may
 * compose a kind's body is the adopting corpus's `admit` declaration over the host kind
 * (`assembly.ts`), so one embedded type means the same thing in every body that admits
 * it. A `nested-file` child names its host per member ({@link MemberInit}'s `host`) —
 * the unit its path composes under.
 */
export type Locus =
  | { readonly kind: "at"; readonly root: string; readonly glob: string }
  | { readonly kind: "embedded" }
  | { readonly kind: "nested-file" };

/**
 * One region of a kind's **layout** — one of the three corpus primitives over the
 * body's heading tree. `prose` is a verbatim span, or an `import` reference resolving to
 * a file's contents; `field` is a heading whose span fills a named field `slot`;
 * `collection` is a heading whose child headings are each one member of `memberKind`,
 * identity the slugged child heading unless an explicit `key` overrides it.
 */
export type LayoutRegion =
  | { readonly region: "prose"; readonly import?: string }
  | { readonly region: "field"; readonly slot: string }
  | { readonly region: "collection"; readonly memberKind: string; readonly key?: string };

/**
 * A declared **layout** — the ordered regions a `layout`-content kind's body is read as.
 * Declaring one on a kind's `content` makes the kind `layout`-content; leaving `content`
 * absent leaves it `file`-content (one verbatim prose body, the default).
 */
export interface Layout {
  readonly regions: readonly LayoutRegion[];
}

/**
 * A kind's **body shape** marker — `"fields"` for a fields-only kind: no body slot at
 * all, the member its typed fields and edges and nothing more (a hook, an MCP server).
 * Absent leaves the kind body-bearing, its body `file` (the default) or a declared
 * {@link Layout}.
 */
export type Shape = "fields";

/**
 * A registration member's **collection address** — where inside a host manifest its
 * registration surfaces: which `manifest` (`settings.json`, `.mcp.json`) and which
 * `keyPath` it keys at — one of the three addresses the shipped kinds surface at.
 * Carried by a fields-only registration kind; absent for a kind that owns its own file
 * locus.
 */
export interface CollectionAddress {
  readonly manifest: string;
  readonly keyPath: "hooks.<Event>" | "mcpServers.*" | "enabledPlugins.*";
}

/**
 * A kind's **template** for one inner layer of nested members it hosts (fact 7): the
 * child `kind`, plus the `path` pattern its children sit at — relative to the parent's
 * own unit — when they are files (a skill's bundled reference documents at `*.md`).
 * Omit `path` for an embedded layer, whose children live in the host's body and own no
 * unit of their own.
 *
 * The declaration is the kind's own nesting fact, and a declared fact only: nothing
 * discovers a file child off the pattern, exactly as a host's embedded members resolve
 * off `nested_members` by address rather than off its templates. An adopting corpus may
 * override the child kind by admitting its own over the host (`declarations.ts`'s
 * `templatesFor`).
 */
export interface Template {
  /** The child kind this layer templates — a kind value, since identity travels by import. */
  readonly kind: KindDefinition<any>;
  /** Where a file child's unit sits, relative to the parent's unit; absent for an embedded layer. */
  readonly path?: string;
}

/** The seven facts of a kind's runtime residue. */
export interface KindFacts {
  /** Fact 1, label — the compiled debug label findings speak; the kind's name. */
  readonly name: string;
  /** The declared provider authority, when the kind qualifies by one. */
  readonly provider?: string;
  /** Fact 2, locus — where members live. */
  readonly locus: Locus;
  /** Fact 3a, projection — the artifact format; omitted for a frontmatterless kind. */
  readonly format?: Format;
  /** Fact 3b, projection — the on-disk unit shape. */
  readonly unitShape: UnitShape;
  /** Fact 4, registration — the declared channel set naming every documented way
   * the world reaches a member (never rivals — a member is live if any one is). */
  readonly registration: readonly Registration[];
 /**
   * The frontmatter key the member's name writes under. For `unitShape:
   * "named-field"` this is the id **source** — the declared field a member's
   * identity is read from (an agent's `name`), never the filename or directory.
   * For `"directory"` it is a projection-order detail only (a skill's `name`
   * still writes into frontmatter, but identity is the directory name); absent
   * when identity is the file stem and no field carries it (a rule).
 */
  readonly identityField?: string;
  /** Fact 5, edge fields — the kind's fields that are references to other members. */
  readonly edgeFields?: readonly EdgeField[];
  /** Fact 6, content — a declared {@link Layout} over the body's heading tree; absent
   * leaves the kind `file`-content (one verbatim prose body, the default). */
  readonly content?: Layout;
  /** Fact 6b, content — the fields-only body shape (`"fields"`, no body slot); absent
   * leaves the kind body-bearing (`file` or a {@link Layout}). */
  readonly shape?: Shape;
  /** The registration member's {@link CollectionAddress} — which manifest and key path
   * its registration surfaces at; absent for a kind that owns its own file locus. */
  readonly collectionAddress?: CollectionAddress;
  /** Fact 7, template — one {@link Template} per inner layer of nested members the kind
   * hosts; absent for a kind that nests nothing. */
  readonly templates?: readonly Template[];
}

/**
 * One authored member — a typed value in the library. Kind identity travels by
 * import (`facts`), never by string; the
 * typed fields are flat at the top level, carried as an ordered pair list so the
 * projected frontmatter key order is the author's.
 */
export interface Member {
  /** The kind's name — its declaration-row and lock identity. */
  readonly kind: string;
  /** The kind's seven facts — carried for projection and the declaration rows. */
  readonly facts: KindFacts;
  /** Identity within the kind. */
  readonly name: string;
  /** The host member a nested-file child's path composes under; absent at every other locus. */
  readonly host?: Member;
  /** The member's words. */
  readonly prose?: Prose;
  /** The kind's typed fields, flat and ordered — the projected frontmatter. */
  readonly fields: ReadonlyArray<readonly [string, unknown]>;
  /** String keys naming the requirements this member fills. */
  readonly satisfies: readonly string[];
  /** Requirements the member itself publishes, by name. */
  readonly requires: Readonly<Record<string, Requirement>>;
  /** The capabilities the member's behavior uses — the permission union's source. */
  readonly needs: readonly Capability[];
}

/** The framework keys of a member init — everything else is a typed field (flat). */
const FRAMEWORK_KEYS = new Set(["name", "host", "prose", "satisfies", "requires", "needs"]);

/** The init a kind constructor takes — the framework keys plus the kind's typed fields `T`. */
export type MemberInit<T> = {
  readonly name: string;
  /** The host member this member's unit composes under — a nested-file child's, and only its. */
  readonly host?: Member;
  readonly prose?: Prose;
  readonly satisfies?: readonly string[];
  readonly requires?: Readonly<Record<string, Requirement>>;
  readonly needs?: readonly Capability[];
} & T;

/**
 * A kind — a callable constructor carrying its seven facts. Calling it builds a
 * member; `key` (its name) keys `expect` and a `kind` reference in a requirement.
 * The value *is* the identity — it travels by import, never by string.
 */
export interface KindDefinition<T> {
  (init: MemberInit<T>): Member;
  readonly facts: KindFacts;
  readonly key: string;
  /**
   * An embedded kind's own composed view of one of its values (`representation.md`,
   * "kind": an embedded-locus format is writer-only, so the hook is unconstrained).
   * Every leaf the hook receives is already resolved to its final stored string
   * (`emit.ts`'s `resolveMemberLeaves`) — a hook author never handles a raw `Text`
   * template. Erased at the emit seam — the engine only ever sees the resulting
   * string, never the function. Absent, `blocks()` renders the kind's values with
   * the default `[collection.entry]` TOML view.
   */
  readonly render?: (value: ResolvedEmbeddedMemberValue) => string;
}

/**
 * Build the ordered projected fields for a member: nothing for a frontmatterless
 * body-bearing kind (memory declares no `format` and is not fields-only), else the
 * identity field (when the kind writes its name into frontmatter) followed by the
 * typed fields in the author's declared order. A fields-only registration kind (a
 * hook, an MCP server) carries its typed fields though it declares no `format` —
 * the fields are the whole member, folded into a manifest entry, never a header.
 */
function orderedFields(facts: KindFacts, init: MemberInit<object>): Array<readonly [string, unknown]> {
  if (facts.format === undefined && facts.shape !== "fields") return [];
  const typed: Array<readonly [string, unknown]> = [];
  for (const [key, value] of Object.entries(init)) {
    if (!FRAMEWORK_KEYS.has(key)) typed.push([key, value]);
 }
  const head: Array<readonly [string, unknown]> =
    facts.identityField !== undefined ? [[facts.identityField, init.name]] : [];
  return [...head, ...typed];
}

/**
 * The host a member init names, checked against its kind's locus: a nested-file child's
 * path composes from its host's unit, so it names one and every other locus names none.
 *
 * # Throws
 * If a nested-file member declares no host, or a member at any other locus declares one.
 */
function hostOf(facts: KindFacts, init: MemberInit<object>): Member | undefined {
  const nested = facts.locus.kind === "nested-file";
  if (nested && init.host === undefined) {
    throw new Error(
      `member \`${init.name}\` of kind \`${facts.name}\`: a nested file child owns a file whose ` +
        `path composes from its host's unit, so it names the \`host\` member it sits under ` +
        `(specs/model/representation.md, "locus").`,
    );
  }
  if (!nested && init.host !== undefined) {
    throw new Error(
      `member \`${init.name}\` of kind \`${facts.name}\`: \`host\` names the member a nested ` +
        `file child composes its path under, and this kind's locus is \`${facts.locus.kind}\` — ` +
        `its path composes from nobody.`,
    );
  }
  return init.host;
}

/** The options `kind()` takes beyond its seven facts — today, only the embedded `render` hook. */
export interface KindOptions {
  readonly render?: (value: ResolvedEmbeddedMemberValue) => string;
}

/**
 * Define a kind. Returns a constructor over the kind's typed
 * fields `T`; every type erases at the seam, so what the returned member carries
 * into emit is the seven facts plus flat field data. `options.render`, when given,
 * rides alongside `facts`/`key` on the returned constructor — never on the member
 * it builds, since it is erased before a member reaches emit.
 */
export function kind<T extends object>(facts: KindFacts, options: KindOptions = {}): KindDefinition<T> {
  const construct = (init: MemberInit<T>): Member => ({
    kind: facts.name,
    facts,
    name: init.name,
    host: hostOf(facts, init),
    prose: init.prose,
    fields: orderedFields(facts, init),
    satisfies: init.satisfies ?? [],
    requires: init.requires ?? {},
    needs: init.needs ?? [],
  });
  return Object.assign(construct, { facts, key: facts.name, render: options.render });
}

/**
 * One entry in a sibling collection: its own key plus its leaf fields
 * (`rejected."baked-projection"`) — an ordered list element, never a positional
 * index; the entry's `key` is what a leaf address carries.
 */
export interface EmbeddedMemberCollectionEntry {
  /** The entry's key among its collection's siblings. */
  readonly key: string;
  /**
   * The entry's own leaf fields: field name → authored string, or a `Text`
   * template whose mentions resolve the way a member-level `Text` body does.
   */
  readonly leaves: Readonly<Record<string, string | Text>>;
}

/**
 * An **embedded member's** composed value — one child of a composed body, passed to `blocks()`:
 * leaves are authored strings keyed by field name; sibling collections are keyed
 * by collection name, each an authored-order list of entries — leaf addresses
 * are structural and keyed. Its
 * facts are declaration rows, captured the same emit pass that renders it —
 * never mined back from the `member.<kind> <key>` fence `blocks()` renders
 * (`pipeline.md`, "Emit"). There is no prescribed child-kind ontology — a
 * corpus that wants one declares its own child kind with the same machinery.
 */
export interface EmbeddedMemberValue {
  /** The child kind this value instantiates — the fence info string's `member.<kind>`. */
  readonly kind: string;
  /** The value's key — the identity a leaf address carries (`surface-authority`). */
  readonly key: string;
  /**
   * Prose leaves: authored strings, law-5 protected one by one, or a `Text`
   * template carrying its own mentions — a leaf mention lifts into the host's
   * mention rows and resolves the way a member-level `Text` body does.
   */
  readonly leaves: Readonly<Record<string, string | Text>>;
  /** Sibling collections: collection name → its entries, in authored order. */
  readonly collections: Readonly<Record<string, readonly EmbeddedMemberCollectionEntry[]>>;
  /** The originating kind's `render` hook, when declared — resolved once at construction. */
  readonly render?: (value: ResolvedEmbeddedMemberValue) => string;
  /**
   * The originating kind's declared edge fields — which of the value's leaves are
   * addresses, and the target facts emit derives from them. Resolved once at
   * construction, the way `render` is.
   */
  readonly edgeFields?: readonly EdgeField[];
}

/**
 * One resolved sibling-collection entry: its own key plus its leaf fields,
 * already resolved to plain strings — no `Text` template remains.
 */
export interface ResolvedEmbeddedMemberCollectionEntry {
  /** The entry's key among its collection's siblings. */
  readonly key: string;
  /** The entry's own leaf fields, already resolved to their final strings. */
  readonly leaves: Readonly<Record<string, string>>;
}

/**
 * The closed set of facts an embedded format may place about one edge field's
 * target — derived at emit off the resolved target member, never authored at the
 * instance and never fabricated, so a rendered reference is true by construction.
 * The set is exactly these four; a fifth fact is a spec question, not a
 * convenience.
 */
export interface EdgeTargetFacts {
  /** The target member's identity within its kind. */
  readonly name: string;
  /** The target's `kind:name` address — what the edge field's leaf authored. */
  readonly address: string;
  /** The target member's kind. */
  readonly kind: string;
  /** The target's projection, relative to the host member's own projection. */
  readonly path: string;
}

/**
 * An {@link EmbeddedMemberValue} after every leaf (top-level and each
 * collection entry's) resolves to its final stored string
 * (`emit.ts`'s `resolveMemberLeaves`) — the shape a kind's own `render` hook
 * receives, so a hook author never handles a raw `Text` leaf.
 */
export interface ResolvedEmbeddedMemberValue {
  /** The child kind this value instantiates. */
  readonly kind: string;
  /** The value's key. */
  readonly key: string;
  /** Prose leaves, already resolved to their final strings. */
  readonly leaves: Readonly<Record<string, string>>;
  /** Sibling collections, each entry's leaves already resolved. */
  readonly collections: Readonly<Record<string, readonly ResolvedEmbeddedMemberCollectionEntry[]>>;
  /**
   * The target facts of each edge field this value *fills*, keyed by the edge field's
   * own name — the data a `render` hook selects to spell a reference. An unfilled
   * field is no edge and carries no entry; a kind declaring no edge fields (or a value
   * composed off a bare kind name, which carries none) has an empty map.
   */
  readonly targets: Readonly<Record<string, EdgeTargetFacts>>;
}

/**
 * Compose an embedded member's value for `blocks()` — the shape any project's own
 * child kind uses. `kind` names the child kind: a bare string, or the child kind's
 * own `KindDefinition` — passing the definition carries its `render` hook and its
 * declared edge fields (when declared) through to emit, with no other change to the
 * composed value's shape. A bare string names a kind whose facts are out of reach, so
 * such a value renders with no target facts.
 */
export function embeddedMemberValue(init: {
  kind: string | KindDefinition<any>;
  key: string;
  leaves: Readonly<Record<string, string | Text>>;
  collections?: EmbeddedMemberValue["collections"];
}): EmbeddedMemberValue {
  const definition = typeof init.kind === "string" ? undefined : init.kind;
  const render = definition?.render;
  const edgeFields = definition?.facts.edgeFields;
  return {
    kind: definition?.key ?? (init.kind as string),
    key: init.key,
    leaves: init.leaves,
    collections: init.collections ?? {},
    ...(render !== undefined ? { render } : {}),
    ...(edgeFields !== undefined ? { edgeFields } : {}),
  };
}
