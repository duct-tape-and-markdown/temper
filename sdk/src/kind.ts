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

/**
 * The shape of the on-disk artifact a member projects to (fact 3, projection) — a closed
 * vocabulary the engine implements once per entry, and the fact deciding which adapter
 * reads a file kind's artifact. `"json-document"` is a whole artifact that is one JSON
 * object: its top-level keys the member's fields, its identity a declared key among them.
 * `"toml-document"` is that read over TOML, and a **read face only** — nothing projects a
 * member declaring it, so it serves a document temper reads in place and never writes.
 */
export type Format = "yaml-frontmatter" | "json-document" | "toml-document";

/**
 * Whether a member is a lone file (identity from the stem), a directory with an
 * entry file (identity from the directory name), a lone file whose identity is
 * read from a declared field (`identityField`) instead of derived from the path (an
 * agent's `name`) — on whichever surface the kind's `format` carries its fields — or a
 * lone file keyed by the directory segment its single-`*` segment glob stars
 * (`"starred-segment"`), coexisting inside another kind's directory rather than owning
 * one. The identity source is this spelled fact, never inferred from the glob.
 */
export type UnitShape = "file" | "directory" | "named-field" | "starred-segment";

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
  | { readonly via: "enablement"; readonly field: string }
  | { readonly via: "registry" };

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
 *
 * An `at` locus may declare a `commitment` class of `local`: per-machine and
 * uncommitted — the kind is declared and reviewed, its members' documents are not. The
 * class is a *file* locus's own fact, which is why only `at` spells it. A local locus is
 * read-side only (the document is the governed source, read at check in place under
 * whatever format the kind declares, and never an `emit` input or target), and its
 * members' rows never enter the lock —
 * they derive at read time under the kind the lock declares. Absent is the other class
 * and the default: committed, where the kind and its members' documents are reviewed
 * together and `emit` owns the bytes.
 */
export type Locus =
  | {
      readonly kind: "at";
      readonly root: string;
      readonly glob: string;
      readonly commitment?: "local";
    }
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
 * A kind's **leaf-set witness** — the leaf names a member of the kind carries, handed
 * over by the kind's own typed surface `T` (decision 0053). The type is the declaration:
 * nothing here is a second schema to keep in step with `T`, because the record's keys
 * *are* `keyof T`. It exists at all because TypeScript erases `T` at the seam, so the
 * set the compiler knows has to reach emit as a runtime value.
 *
 * The record is **exhaustive** — `-?` strips optionality, so omitting one key of `T` is a
 * compile error and a key that is not `T`'s own is one too. That binding is what keeps
 * the witness from degrading into the free-hand leaf schema the decision rejected: a
 * partial set is unwritable rather than merely discouraged.
 *
 * Key order is the order the lowered row carries (`declarations.ts`), so a kind's leaves
 * read in the order its author declared them.
 */
export type LeafSet<T extends object> = { readonly [K in keyof T]-?: true };

/**
 * A registration member's **collection address** — where inside a host manifest its
 * registration surfaces: which `manifest` (`settings.json`, `.mcp.json`) and which
 * `keyPath` it keys at — one of the four addresses the shipped kinds surface at.
 * Carried by a fields-only registration kind; absent for a kind that owns its own file
 * locus.
 */
export interface CollectionAddress {
  readonly manifest: string;
  readonly keyPath:
    | "hooks.<Event>"
    | "mcpServers.*"
    | "enabledPlugins.*"
    | "extraKnownMarketplaces.*";
  readonly entryShape: string;
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

/**
 * The seven facts of a kind's runtime residue, plus the derived {@link LeafSet} witness
 * its typed surface hands over. `T` is that surface — the interface the kind's
 * constructor is generic over. It defaults to an erased `Record<string, unknown>`, so
 * every signature that only ever *reads* a facts value (`declarations.ts`'s lowering,
 * {@link Member}, {@link KindDefinition}) names `KindFacts` bare and no call site
 * re-spells a type argument.
 */
export type KindFacts<T extends object = Record<string, unknown>> =
  | {
      /** Fact 1, label — the compiled debug label findings speak; the kind's name. */
      readonly name: string;
      /** The declared provider authority, when the kind qualifies by one. */
      readonly provider?: string;
      /**
       * The built-in kind this facts value **relocates** — set only by {@link relocate},
       * from the base kind's own name, never authored. It is the authoring layer's
       * *provenance* fact: the value was derived from the imported built-in, so a
       * second same-named kind in play is a sanctioned relocation rather than a name
       * collision. It never reaches a kind-fact row — the lock reader holds no base
       * value to compare against and re-decides *structurally* instead
       * (`src/compose.rs`'s `row_relocates_builtin`).
       */
      readonly relocates?: string;
      /** Fact 2, locus — where members live, and for a file locus whether their documents
       * are committed: a `local` commitment class declares the kind reviewed and its
       * members' documents not. */
      readonly locus: { readonly kind: "at"; readonly root: string; readonly glob: string; readonly commitment?: "local" };
      /** Fact 3a, projection — the artifact format; omitted for a kind that declares none. */
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
       * when identity is the file stem and no field carries it (a rule), or the
       * starred directory segment (`"starred-segment"`) — both path-derived, never a
       * field.
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
      /** The kind's **leaf set**, witnessed exhaustively over its own typed surface and
       * lowered to the row's `leaves` column at emit ({@link LeafSet}, decision 0053) —
       * what a read verb renders where the surface holds no member of the kind yet.
       * Absent for a kind whose constructor declares no witness. */
      readonly leaves?: LeafSet<T>;
      /** Advisory authoring counsel for the kind as a whole — teaching at authoring time via
       * `schema` hover or `explain`, carrying no predicate or severity (decision 0045). */
      readonly guidance?: string;
      /** External-fact source backing the guidance — a doc URL plus retrieved date,
       * carried as data. */
      readonly cite?: string;
    }
  | {
      /** Fact 1, label — the compiled debug label findings speak; the kind's name. */
      readonly name: string;
      /** The declared provider authority, when the kind qualifies by one. */
      readonly provider?: string;
      /**
       * The built-in kind this facts value **relocates** — set only by {@link relocate},
       * from the base kind's own name, never authored. It is the authoring layer's
       * *provenance* fact: the value was derived from the imported built-in, so a
       * second same-named kind in play is a sanctioned relocation rather than a name
       * collision. It never reaches a kind-fact row — the lock reader holds no base
       * value to compare against and re-decides *structurally* instead
       * (`src/compose.rs`'s `row_relocates_builtin`).
       */
      readonly relocates?: string;
      /** Fact 2, locus — where members live, and for a file locus whether their documents
       * are committed: a `local` commitment class declares the kind reviewed and its
       * members' documents not. */
      readonly locus: { readonly kind: "embedded" };
      /** Fact 3a, projection — the artifact format; omitted for a kind that declares none. */
      readonly format?: Format;
      /** Fact 3b, projection — the on-disk unit shape. */
      readonly unitShape: UnitShape;
      /** Fact 4, registration — embedded members register nothing. */
      readonly registration: readonly [];
      /**
       * The frontmatter key the member's name writes under. For `unitShape:
       * "named-field"` this is the id **source** — the declared field a member's
       * identity is read from (an agent's `name`), never the filename or directory.
       * For `"directory"` it is a projection-order detail only (a skill's `name`
       * still writes into frontmatter, but identity is the directory name); absent
       * when identity is the file stem and no field carries it (a rule), or the
       * starred directory segment (`"starred-segment"`) — both path-derived, never a
       * field.
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
      /** The kind's **leaf set**, witnessed exhaustively over its own typed surface and
       * lowered to the row's `leaves` column at emit ({@link LeafSet}, decision 0053) —
       * what a read verb renders where the surface holds no member of the kind yet.
       * Absent for a kind whose constructor declares no witness. */
      readonly leaves?: LeafSet<T>;
      /** Advisory authoring counsel for the kind as a whole — teaching at authoring time via
       * `schema` hover or `explain`, carrying no predicate or severity (decision 0045). */
      readonly guidance?: string;
      /** External-fact source backing the guidance — a doc URL plus retrieved date,
       * carried as data. */
      readonly cite?: string;
    }
  | {
      /** Fact 1, label — the compiled debug label findings speak; the kind's name. */
      readonly name: string;
      /** The declared provider authority, when the kind qualifies by one. */
      readonly provider?: string;
      /**
       * The built-in kind this facts value **relocates** — set only by {@link relocate},
       * from the base kind's own name, never authored. It is the authoring layer's
       * *provenance* fact: the value was derived from the imported built-in, so a
       * second same-named kind in play is a sanctioned relocation rather than a name
       * collision. It never reaches a kind-fact row — the lock reader holds no base
       * value to compare against and re-decides *structurally* instead
       * (`src/compose.rs`'s `row_relocates_builtin`).
       */
      readonly relocates?: string;
      /** Fact 2, locus — where members live, and for a file locus whether their documents
       * are committed: a `local` commitment class declares the kind reviewed and its
       * members' documents not. */
      readonly locus: { readonly kind: "nested-file" };
      /** Fact 3a, projection — the artifact format; omitted for a kind that declares none. */
      readonly format?: Format;
      /** Fact 3b, projection — the on-disk unit shape. */
      readonly unitShape: UnitShape;
      /** Fact 4, registration — nested-file members register nothing. */
      readonly registration: readonly [];
      /**
       * The frontmatter key the member's name writes under. For `unitShape:
       * "named-field"` this is the id **source** — the declared field a member's
       * identity is read from (an agent's `name`), never the filename or directory.
       * For `"directory"` it is a projection-order detail only (a skill's `name`
       * still writes into frontmatter, but identity is the directory name); absent
       * when identity is the file stem and no field carries it (a rule), or the
       * starred directory segment (`"starred-segment"`) — both path-derived, never a
       * field.
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
      /** The kind's **leaf set**, witnessed exhaustively over its own typed surface and
       * lowered to the row's `leaves` column at emit ({@link LeafSet}, decision 0053) —
       * what a read verb renders where the surface holds no member of the kind yet.
       * Absent for a kind whose constructor declares no witness. */
      readonly leaves?: LeafSet<T>;
      /** Advisory authoring counsel for the kind as a whole — teaching at authoring time via
       * `schema` hover or `explain`, carrying no predicate or severity (decision 0045). */
      readonly guidance?: string;
      /** External-fact source backing the guidance — a doc URL plus retrieved date,
       * carried as data. */
      readonly cite?: string;
    };

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
const FRAMEWORK_KEYS = new Set(["name", "host", "prose", "satisfies", "requires", "needs", "residue"]);

/**
 * The **reserved leaf key** a nested member's own span lands under (0051) —
 * `<host-address>/<kind>/<key>/prose`. On the read half a layout collection member's
 * own paragraph, the text under its heading before the first child heading, lands here
 * (`src/layout.rs`'s `OWN_SPAN_LEAF`); leaves stay one family, so leaf predicates, leaf
 * addresses and `explain`'s narration range over one source.
 *
 * `prose` is already this word at member grain ({@link MemberInit.prose}), so it is a
 * framework key for an init and a reserved name for a leaf — the same thing spelled once.
 * The composed half authors no own span (its leaves are all author-named), so the
 * reservation binds here as a refusal: a composed leaf of this name would address
 * identically to a read member's own span and mean something else.
 */
const RESERVED_LEAF = "prose";

/**
 * The **residue key** — the channel a partially-governed external format's undocumented
 * keys ride under. A kind whose schema is large and version-evolving types what it
 * governs and leaves the remainder opaque and *named* rather than indicted; `residue` is
 * that name, spliced flat into the projected fields by {@link orderedFields}.
 *
 * Like {@link RESERVED_LEAF}, this is a name the framework owns — a kind whose external
 * format documents a top-level key of it cannot spell that key as an ordinary field. The
 * reservation is kindless, but the channel is opt-in by **type**: only a surface `T`
 * declaring `residue?` can spell the bag, so a closed-frontmatter kind (a skill, a rule,
 * an agent) keeps refusing it by excess-property check like any other unknown key.
 */
const RESIDUE_KEY = "residue";

/**
 * The init a kind constructor takes — the framework keys plus the kind's typed fields `T`.
 * {@link RESIDUE_KEY} is a framework key too, deliberately not spelled below: a kind opts
 * into that channel through its own surface `T`.
 */
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
 *
 * A {@link RESIDUE_KEY} bag splices in last: what the program types, it orders; what it
 * merely carries, it sorts.
 */
function orderedFields(facts: KindFacts, init: MemberInit<object>): Array<readonly [string, unknown]> {
  if (facts.format === undefined && facts.shape !== "fields") return [];
  const typed: Array<readonly [string, unknown]> = [];
  for (const [key, value] of Object.entries(init)) {
    if (!FRAMEWORK_KEYS.has(key)) typed.push([key, value]);
 }
  const head: Array<readonly [string, unknown]> =
    facts.identityField !== undefined ? [[facts.identityField, init.name]] : [];
  return [...head, ...typed, ...residueFields(init)];
}

/**
 * The init's residue bag as projected fields, key-sorted. The bag is a record, so it
 * carries no authored order to preserve — sorting is what makes the projection a
 * function of the keys alone, the same stability the harness-level residue rows already
 * take (`declarations.ts`'s `settingsRows`).
 */
function residueFields(init: MemberInit<object>): Array<readonly [string, unknown]> {
  const residue = (init as { readonly [RESIDUE_KEY]?: Readonly<Record<string, unknown>> })[RESIDUE_KEY];
  if (residue === undefined) return [];
  // Default `sort()` is UTF-16 code-unit order — the same total order `compareStrings`
  // gives every declaration family, reached without importing `declarations.ts` (which
  // imports `builtins.ts`, which imports this module).
  return Object.keys(residue)
    .sort()
    .map((key): readonly [string, unknown] => [key, residue[key]]);
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
 *
 * The facts are typed over the same `T` the constructor is, which is what binds a
 * declared {@link LeafSet} witness to `keyof T`: the leaf set is the kind's own surface,
 * checked here at the keystroke, and its lowering to the row is the one place it is
 * spelled again (`declarations.ts`).
 */
export function kind<T extends object>(facts: KindFacts<T>, options: KindOptions = {}): KindDefinition<T> {
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
 * A **relocation delta** — the facts a relocated built-in kind diverges from its base
 * on. Two faces, either or both:
 *
 * - `edgeFields`, *added* to whatever the base already declares (never replacing them,
 *   which would drop a shipped kind's own edges silently). Each added field names a key
 *   of the relocated kind's typed surface `T`, so an edge can never be declared over a
 *   field the kind does not carry.
 * - `governs`, *replacing* the base's `at` locus root and glob — moving where the kind's
 *   members are found, the one fact the engine's own overlay exists to apply
 *   (`src/compose.rs`'s `overlay_builtin_kind`).
 *
 * Every other fact — format, unit shape, registration, content, templates, the declared
 * leaf set — rides through unchanged, which is exactly what makes the emitted row still
 * read as a relocation rather than a name collision on the reading side.
 */
export interface KindRelocation<T> {
  /** The edge fields this relocation adds, each over a field of the kind's own surface. */
  readonly edgeFields?: readonly {
    readonly field: keyof T & string;
    readonly to: readonly [string, ...string[]];
  }[];
  /**
   * The locus this relocation moves the kind's members to — root and glob, the two
   * columns the lock reader's overlay writes back. A file locus's `commitment` class is
   * deliberately absent: the overlay writes `Governs { root, glob }` and nothing else, so
   * a `commitment` delta would author a fact the reader drops in silence — a built-in's
   * commitment class stays the built-in's.
   */
  readonly governs?: { readonly root: string; readonly glob: string };
}

/**
 * The locus a `governs` delta moves the base to: the base's `at` root and glob replaced,
 * every other locus fact (its commitment class among them) riding through.
 *
 * # Throws
 * If the base's locus is not `at` — an embedded or nested-file kind governs no glob at
 * all (its members compose their paths from a host's unit, or own no file), so there is
 * no locus to move and its row carries no `governs` columns to move it to.
 */
function relocatedLocus(
  base: KindFacts,
  governs: NonNullable<KindRelocation<never>["governs"]>,
): Extract<Locus, { kind: "at" }> {
  if (base.locus.kind !== "at") {
    throw new Error(
      `relocating kind \`${base.name}\`: a \`governs\` delta moves the ` +
        `path glob a kind's members are found at, and this kind's locus is \`${base.locus.kind}\` ` +
        `— it governs no glob of its own. Drop the \`governs\` face of the delta ` +
        `(specs/model/representation.md, "locus").`,
    );
  }
  return { ...base.locus, root: governs.root, glob: governs.glob };
}

/**
 * **Relocate** a built-in kind: the sanctioned way an adopting corpus moves a kind it
 * does not own to its own root, adds an edge field to it, or both. Returns a fresh
 * constructor over the widened typed surface `T` (the base's fields plus the added edge
 * fields, spelled by the caller as one interface), carrying the base's facts with
 * `delta`'s edge fields appended, `delta`'s `governs` in place of the base's locus root
 * and glob, and the base's own {@link KindDefinition.render} hook preserved. Ownership,
 * not privilege — a relocated built-in is an ordinary kind value from here on, and its
 * added edge reaches the lock as an assembly `edge` row keyed by `from`, exactly as any
 * kind's does (`declarations.ts`), never as a column on a kind-fact row.
 *
 * A moved locus, by contrast, *is* a kind-fact row column pair: the emitted row carries
 * the delta's `governs_root`/`governs_glob` while `format`, `unit_shape` and
 * `registration` stay the base's — which is precisely the three-fact test the lock
 * reader applies before overlaying the row onto its compiled-in built-in
 * (`src/compose.rs`'s `row_relocates_builtin`), so the engine reads members at the new
 * root rather than treating the row as a colliding kind.
 *
 * The produced facts carry `relocates`, naming the base — the marker that tells a
 * legitimate relocation from a genuine name collision when two same-named kinds are in
 * play. Identity travels by import: the base is the imported built-in value, so the
 * provenance is proven here rather than inferred downstream.
 *
 * # Throws
 * If an added edge field re-declares one the base already carries, or one another
 * entry of the same delta already added — two `edge` rows over one `<from, field>`
 * cross-wire the graph instead of declaring one relationship. If a `governs` delta
 * names a base whose locus is not `at` ({@link relocatedLocus}). And if the delta
 * declares neither face, which would mint a marker-bearing clone of the base — a second
 * same-named kind diverging on nothing, which is a name collision spelled as a
 * relocation.
 */
export function relocate<T extends object>(
  base: KindDefinition<any>,
  delta: KindRelocation<T>,
): KindDefinition<T> {
  const declaredEdges = delta.edgeFields ?? [];
  if (declaredEdges.length === 0 && delta.governs === undefined) {
    throw new Error(
      `relocating kind \`${base.facts.name}\`: a relocation declares at least one diverging ` +
        `fact — \`edgeFields\`, \`governs\`, or both. A delta declaring neither mints a ` +
        `second kind of the base's own name that diverges on nothing, which reads as a name ` +
        `collision rather than a relocation (specs/model/representation.md, "kind").`,
    );
  }
  const locus = delta.governs === undefined ? undefined : relocatedLocus(base.facts, delta.governs);
  const inherited = base.facts.edgeFields ?? [];
  const claimed = new Set(inherited.map((edge) => edge.field));
  const added: EdgeField[] = [];
  for (const edge of declaredEdges) {
    if (claimed.has(edge.field)) {
      throw new Error(
        `relocating kind \`${base.facts.name}\`: edge field \`${edge.field}\` is already ` +
          `declared, and a second declaration of one field cross-wires the graph rather than ` +
          `adding a relationship (specs/model/representation.md, "kind").`,
      );
    }
    claimed.add(edge.field);
    added.push({ field: edge.field, to: edge.to });
  }
  const edgeFields = [...inherited, ...added];
  const relocated = { ...base.facts, relocates: base.facts.name, edgeFields };
  // Two spellings, not one with an optional `locus`: `KindFacts` is a union discriminated
  // on the locus, so the moved case must carry the `at` locus as its own literal branch.
  const facts: KindFacts = locus === undefined ? relocated : { ...relocated, locus };
  // The base's leaf-set witness rides through as data, and the assertion is that
  // pass-through spelled: a relocation appends *edge fields* over the kind's own surface,
  // never a leaf, so the set the base's own constructor already bound still names what a
  // member carries. `T` here is the widened surface (the base's fields plus the added
  // edges), so re-binding the witness to `keyof T` would demand the edge fields be leaves.
  return kind<T>(facts as KindFacts<T>, { render: base.render });
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
 * The set is exactly these five facts; no convenience additions beyond.
 */
export interface EdgeTargetFacts {
  /** The target member's identity within its kind. */
  readonly name: string;
  /** The target's `kind:name` address — what the edge field's leaf authored. */
  readonly address: string;
  /** The target member's kind. */
  readonly kind: string;
  /** The target's projection, relative to the host member's own projection — used by a render hook to spell a link from the host body. */
  readonly path: string;
  /** The target's projection rooted at the repository — used by a render hook consuming edge targets from repo root. */
  readonly repoRootedPath: string;
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
export function embeddedMemberValue<T extends object>(init: {
  kind: KindDefinition<T>;
  key: string;
  leaves: Readonly<Record<keyof T, string | Text>>;
  collections?: EmbeddedMemberValue["collections"];
}): EmbeddedMemberValue;
export function embeddedMemberValue(init: {
  kind: string;
  key: string;
  leaves: Readonly<Record<string, string | Text>>;
  collections?: EmbeddedMemberValue["collections"];
}): EmbeddedMemberValue;
export function embeddedMemberValue(init: {
  kind: string | KindDefinition<any>;
  key: string;
  leaves: Readonly<Record<string, string | Text>>;
  collections?: EmbeddedMemberValue["collections"];
}): EmbeddedMemberValue {
  const definition = typeof init.kind === "string" ? undefined : init.kind;
  const kindKey = definition?.key ?? (init.kind as string);
  refuseReservedLeaf(kindKey, init.key, init.leaves);
  for (const [collection, entries] of Object.entries(init.collections ?? {})) {
    for (const entry of entries) {
      refuseReservedLeaf(kindKey, `${init.key}.${collection}.${entry.key}`, entry.leaves);
    }
  }
  const render = definition?.render;
  const edgeFields = definition?.facts.edgeFields;
  return {
    kind: kindKey,
    key: init.key,
    leaves: init.leaves,
    collections: init.collections ?? {},
    ...(render !== undefined ? { render } : {}),
    ...(edgeFields !== undefined ? { edgeFields } : {}),
  };
}

/**
 * Refuse a composed leaf named {@link RESERVED_LEAF} — the key a read member's own span
 * owns, so a second meaning under it is a coincident leaf address, refused at compose
 * rather than resolved by precedence (0051). Loud at the authoring seam, where the author
 * can rename the field, not at emit over bytes already written.
 *
 * # Throws
 * If `leaves` carries the reserved key.
 */
function refuseReservedLeaf(
  kind: string,
  key: string,
  leaves: Readonly<Record<string, string | Text>>,
): void {
  if (!Object.hasOwn(leaves, RESERVED_LEAF)) return;
  throw new Error(
    `embedded member \`${kind}\` \`${key}\`: leaf \`${RESERVED_LEAF}\` is reserved for a ` +
      `member's own span — rename the field, or author the words as the member's prose`,
  );
}
