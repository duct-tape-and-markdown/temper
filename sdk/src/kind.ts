/**
 * Kinds — the engine room. A kind is a plain typed surface — an interface
 * `T` and a constructor `kind<T>()` — plus seven facts of runtime residue: label,
 * locus, projection, registration, edge fields, content, and template. A registration kind (a hook,
 * an MCP server) extends the content fact with a fields-only `shape` and a
 * `collectionAddress` naming the key path it surfaces at, inside the manifest its starless
 * `at` locus names. `tsc` is the keystroke
 * wall; every type erases at the seam, and what a kind leaves behind rides the lock as
 * rows. Identity travels by import, never by string — a `kind` reference is the imported
 * value.
 */

import type { Prose, Text } from "./prose.js";
import { memberProse } from "./prose.js";
import type { Capability } from "./needs.js";
import type { Requirement } from "./contract.js";
import { isOneSegment } from "./member-address.js";

/** The brand {@link Remedy} keys on — unexported, so no value outside this module can carry it. */
declare const remedy: unique symbol;

/**
 * A type no authored value inhabits, whose argument is the fix for writing one. It stands
 * where a known mistake would otherwise meet `never` or a bare missing-name error — a field
 * the locus already derives, a field that moved, a retired export — so the compiler's own
 * error prints the remedy (`Type 'never[]' is not assignable to type 'Remedy<"…">'`).
 */
export type Remedy<Message extends string> = { readonly [remedy]: Message };

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
 * one — or a lone file named by its whole path under the locus, each `/` a `:` and the
 * extension dropped (`"path"`: a command's `frontend/component.md` is
 * `frontend:component`). The identity source is this spelled fact, never inferred from
 * the glob.
 */
export type UnitShape = "file" | "directory" | "named-field" | "starred-segment" | "path";

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
      /** A placement template: literal `/`-separated segments with at most one `*`,
       * optionally under a leading `**` + `/` for any depth. Any other pattern syntax
       * (a `?`, a character class, braces, an extglob, a second star) is refused when
       * the engine loads the lock. */
      readonly glob: string;
      readonly commitment?: "local";
      /** Paths the template would govern that the kind leaves ungoverned — each a
       * starless, harness-relative literal the template matches (`domains/README.md`
       * under `domains` + `*.md`), or a bare directory (no trailing `/`) the template
       * reaches beneath, leaving every path under it ungoverned (`domains/legacy` under
       * `domains` + `* /SKILL.md`, read without the space). Anything else is refused when
       * the engine loads the lock. */
      readonly exclude?: readonly string[];
    }
  | { readonly kind: "embedded" }
  | { readonly kind: "nested-file" };

/**
 * One region of a kind's **layout** — one of the three corpus primitives over the
 * body's heading tree. `prose` is a verbatim span, or an `import` reference resolving to
 * a file's contents; `field` is a heading whose span fills a named field `slot` — a key
 * of the kind's own typed surface `T`, so a slot the member does not carry is a compile
 * error rather than a second, free-hand spelling of the field set;
 * `collection` is a heading whose child headings are each one member of `memberKind`,
 * identity the slugged child heading unless an explicit `key` overrides it.
 */
export type LayoutRegion<T extends object = Record<string, unknown>> =
  | { readonly region: "prose"; readonly import?: string }
  | { readonly region: "field"; readonly slot: keyof T & string }
  | { readonly region: "collection"; readonly memberKind: string; readonly key?: string };

/**
 * A declared **layout** — the ordered regions a `layout`-content kind's body is read as.
 * Declaring one on a kind's `content` makes the kind `layout`-content; leaving `content`
 * absent leaves it `file`-content (one verbatim prose body, the default).
 */
export interface Layout<T extends object = Record<string, unknown>> {
  readonly regions: readonly LayoutRegion<T>[];
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
 * A registration member's **collection address** — where inside its host manifest its
 * registration surfaces: the `keyPath` it keys at, one of the four addresses the shipped
 * kinds surface at. The manifest itself is the kind's starless `at` locus — one home, so
 * the address and the locus can never name two different files ({@link manifestOf}).
 * Carried by a fields-only registration kind; absent for a kind that owns its own file
 * locus.
 */
export interface CollectionAddress {
  readonly keyPath:
    | "hooks.<Event>"
    | "mcpServers.*"
    | "enabledPlugins.*"
    | "extraKnownMarketplaces.*";
  readonly entryShape: string;
  /** Retired: the manifest is the kind's starless `at` locus glob, stated once there. */
  readonly manifest?: Remedy<"the manifest is the kind's `at` locus glob: delete `manifest`">;
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
 * Per-field authoring counsel over a kind's typed surface `T` — one optional counsel per
 * key of `T`, so a field the surface does not carry cannot be counselled. Advisory only:
 * it never gates, and it reaches an author through the kind's path-scoped guidance rule.
 *
 * A surface with no fields resolves to `never`: the mapped object over no keys would be
 * `{}`, which the compiler checks no excess property against, so the empty case is
 * spelled as the refusal it is rather than left open to any key.
 */
export type FieldGuidance<T extends object> = [keyof T & string] extends [never]
  ? never
  : { readonly [K in keyof T & string]?: string };

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
      /** The kind this one is a **subkind** of, by name — set only by {@link extend} under a
       * new name. It reaches the kind-fact row's `base` column. */
      readonly base?: string;
      /** Fact 2, locus — where members live, and for a file locus whether their documents
       * are committed: a `local` commitment class declares the kind reviewed and its
       * members' documents not. */
      readonly locus: Extract<Locus, { kind: "at" }>;
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
      readonly content?: Layout<T>;
      /** Fact 6b, content — the fields-only body shape (`"fields"`, no body slot); absent
       * leaves the kind body-bearing (`file` or a {@link Layout}). */
      readonly shape?: Shape;
      /** The registration member's {@link CollectionAddress} — the key path its
       * registration surfaces at inside the manifest this kind's starless locus names;
       * absent for a kind that owns its own file locus. */
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
      /** Advisory counsel per field, keyed by the kind's typed surface `T` — a key naming a
       * field the surface lacks fails `tsc` (decision 0091, bounded as 0078 bounds a leaf
       * set). It carries no predicate and rides the kind's guidance rule beside the kind's
       * own counsel, in the order the author wrote it. */
      readonly fieldGuidance?: FieldGuidance<T>;
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
      /** The kind this one is a **subkind** of, by name — set only by {@link extend} under a
       * new name. It reaches the kind-fact row's `base` column. */
      readonly base?: string;
      /** Fact 2, locus — where members live, and for a file locus whether their documents
       * are committed: a `local` commitment class declares the kind reviewed and its
       * members' documents not. */
      readonly locus: { readonly kind: "embedded" };
      /** Fact 3a, projection — the artifact format; omitted for a kind that declares none. */
      readonly format?: Format;
      /** Fact 3b, projection — the on-disk unit shape. */
      readonly unitShape: UnitShape;
      /** Fact 4, registration — derived, never declared: an embedded member registers
       * nothing, so the locus already states the empty set and the lowering supplies it
       * (`declarations.ts`). */
      readonly registration?: Remedy<"an embedded locus registers nothing: delete `registration`">;
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
      readonly content?: Layout<T>;
      /** Fact 6b, content — the fields-only body shape (`"fields"`, no body slot); absent
       * leaves the kind body-bearing (`file` or a {@link Layout}). */
      readonly shape?: Shape;
      /** No collection address: a registration kind's manifest is its `at` locus, and
       * this locus names no file. */
      readonly collectionAddress?: Remedy<"a collection address keys inside an `at` locus's manifest: declare that locus or delete `collectionAddress`">;
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
      /** Advisory counsel per field, keyed by the kind's typed surface `T` — a key naming a
       * field the surface lacks fails `tsc` (decision 0091, bounded as 0078 bounds a leaf
       * set). It carries no predicate and rides the kind's guidance rule beside the kind's
       * own counsel, in the order the author wrote it. */
      readonly fieldGuidance?: FieldGuidance<T>;
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
      /** The kind this one is a **subkind** of, by name — set only by {@link extend} under a
       * new name. It reaches the kind-fact row's `base` column. */
      readonly base?: string;
      /** Fact 2, locus — where members live, and for a file locus whether their documents
       * are committed: a `local` commitment class declares the kind reviewed and its
       * members' documents not. */
      readonly locus: { readonly kind: "nested-file" };
      /** Fact 3a, projection — the artifact format; omitted for a kind that declares none. */
      readonly format?: Format;
      /** Fact 3b, projection — the on-disk unit shape. */
      readonly unitShape: UnitShape;
      /** Fact 4, registration — derived, never declared: a nested-file member registers
       * nothing, so the locus already states the empty set and the lowering supplies it
       * (`declarations.ts`). */
      readonly registration?: Remedy<"a nested-file locus registers nothing: delete `registration`">;
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
      readonly content?: Layout<T>;
      /** Fact 6b, content — the fields-only body shape (`"fields"`, no body slot); absent
       * leaves the kind body-bearing (`file` or a {@link Layout}). */
      readonly shape?: Shape;
      /** No collection address: a registration kind's manifest is its `at` locus, and
       * this locus names no file. */
      readonly collectionAddress?: Remedy<"a collection address keys inside an `at` locus's manifest: declare that locus or delete `collectionAddress`">;
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
      /** Advisory counsel per field, keyed by the kind's typed surface `T` — a key naming a
       * field the surface lacks fails `tsc` (decision 0091, bounded as 0078 bounds a leaf
       * set). It carries no predicate and rides the kind's guidance rule beside the kind's
       * own counsel, in the order the author wrote it. */
      readonly fieldGuidance?: FieldGuidance<T>;
    };

/**
 * A **declared input** — a file the member's claims rest on, fingerprinted by the lock
 * and moved nowhere (`specs/model/authoring.md`, "The SDK"). The path resolves against
 * the stating module ({@link moduleUrl}), never the workspace — the same anchor
 * `include()` and a `file()` body take.
 *
 * It lives here rather than beside the prose references it rhymes with: an input rides
 * no text span and pairs with no body slot, so it is a member-grain framework key, not a
 * word in a body. When the input's bytes move, the member's own freshness finding routes
 * the author to re-verify the claims *before* re-emitting — temper never judges whether
 * the claim still holds, only names the place to look.
 */
export interface Input {
  readonly kind: "input";
  /** Path to the input, resolved against {@link moduleUrl}. */
  readonly path: string;
  /** The declaring module's own `import.meta.url` — what {@link path} resolves against. */
  readonly moduleUrl: string;
}

/**
 * Declare a file a member's claims rest on — the lock fingerprints it and no byte
 * reaches the projection. The path resolves against the stating module, so the call is
 * `input(import.meta.url, "./schema.json")`.
 */
export function input(moduleUrl: string, path: string): Input {
  return { kind: "input", path, moduleUrl };
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
  /** The files this member's claims rest on — fingerprinted, never moved. */
  readonly inputs: readonly Input[];
}

/** The framework keys of a member init — everything else is a typed field (flat). */
const FRAMEWORK_KEYS = new Set(["name", "host", "prose", "satisfies", "requires", "needs", "inputs", "residue"]);

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
 * The bag a surface `T` declares its {@link RESIDUE_KEY} channel as — the keys `T` does
 * *not* type, and only those. The two halves partition the format's key space, so a key
 * in both is a mis-spelling rather than an override: open by key, then narrowed against
 * `keyof T`, which leaves a typed key's slot here holding nothing a value can fill.
 *
 * The narrowing is the front door, never the whole gate — a bag the program computed
 * arrives as a bare `Record<string, unknown>` and types fine, so the constructor refuses
 * the collision again over the keys it actually holds ({@link orderedFields}).
 */
export type Residue<T> = Readonly<Record<string, unknown>> & {
  readonly [K in keyof T]?: Remedy<`\`${K & string}\` is a typed field: write it beside \`residue\`, not in it`>;
};

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
  /**
   * The files this member's claims rest on ({@link input}) — fingerprinted by the lock,
   * with no byte reaching the projection. The surface binds a *member*: an embedded
   * value takes no `MemberInit`, and its claims are its host's.
   */
  readonly inputs?: readonly Input[];
} & T;

/**
 * A kind — a callable constructor carrying its seven facts. Calling it builds a
 * member; `key` (its name) keys `expect` and a `kind` reference in a requirement.
 * The value *is* the identity — it travels by import, never by string. `Init` is the
 * constructor's input, {@link MemberInit} over `T` unless a kind widens it with
 * {@link Remedy} slots for fields that moved off its surface.
 */
export interface KindDefinition<T, Init = MemberInit<T>> {
  (init: Init): Member;
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
  /**
   * Where the **plain fields** an {@link extend} delta added live — every one named, so
   * construction never projects it as a frontmatter key, and the delta's body `render`,
   * when declared, which builds a member's body from its values. Set only by `extend`;
   * absent on a kind declaring no added plain field.
   */
  readonly body?: BodyPlacement;
}

/**
 * The plain fields a kind's body holds rather than its frontmatter, and the render that
 * places those a `"render"` placement names ({@link KindExtension.fields}). Erased at the
 * emit seam like {@link KindDefinition.render}: a member reaches emit with the field out
 * of its projected fields and its value already in its prose.
 */
export interface BodyPlacement {
  /** Every added plain field, whichever placement holds it. */
  readonly plain: readonly string[];
  /** The member's body built from its values — the base's own render, then the delta's. */
  readonly render?: (values: Readonly<Record<string, unknown>>) => Prose;
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
 * merely carries, it sorts. A `plain` field — one an {@link extend} delta placed in the
 * body ({@link BodyPlacement}) — is never projected: it lives where the base's format
 * admits it, never as an undocumented key.
 */
function orderedFields(
  facts: KindFacts,
  init: MemberInit<object>,
  plain: ReadonlySet<string>,
): Array<readonly [string, unknown]> {
  if (facts.format === undefined && facts.shape !== "fields") return [];
  const typed: Array<readonly [string, unknown]> = [];
  for (const [key, value] of Object.entries(init)) {
    if (!FRAMEWORK_KEYS.has(key) && !plain.has(key)) typed.push([key, value]);
  }
  const head: Array<readonly [string, unknown]> =
    facts.identityField !== undefined ? [[facts.identityField, init.name]] : [];
  const projected = [...head, ...typed];
  return [...projected, ...residueFields(facts, init, projected)];
}

/**
 * The init's residue bag as projected fields, key-sorted and disjoint from what the
 * member already projects. The bag is a record, so it carries no authored order to
 * preserve — sorting is what makes the projection a function of the keys alone, the same
 * stability the harness-level residue rows already take (`declarations.ts`'s
 * `settingsRows`).
 *
 * # Throws
 * If a residue key is one `projected` already carries. The bag holds what the surface
 * does not type, so a key in both is a mis-spelling, not an override — and a silent one
 * downstream, since the engine's writer collects the field list into a map and the
 * residue pair, spliced last, would take the typed value's place.
 */
function residueFields(
  facts: KindFacts,
  init: MemberInit<object>,
  projected: ReadonlyArray<readonly [string, unknown]>,
): Array<readonly [string, unknown]> {
  const residue = (init as { readonly [RESIDUE_KEY]?: Readonly<Record<string, unknown>> })[RESIDUE_KEY];
  if (residue === undefined) return [];
  const already = new Set(projected.map(([key]) => key));
  const fields: Array<readonly [string, unknown]> = [];
  // Default `sort()` is UTF-16 code-unit order — the same total order `compareStrings`
  // gives every declaration family, reached without importing `declarations.ts` (which
  // imports `builtins.ts`, which imports this module).
  for (const key of Object.keys(residue).sort()) {
    if (already.has(key)) {
      throw new Error(
        `member \`${init.name}\` of kind \`${facts.name}\`: \`${RESIDUE_KEY}\` key \`${key}\` is ` +
          `already a field this member projects — the bag carries the keys the kind's surface ` +
          `does not type, so spell this one as the field it is ` +
          `(specs/builtins.md, "The shipped kinds").`,
      );
    }
    fields.push([key, residue[key]]);
  }
  return fields;
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

/** {@link KindOptions} plus the {@link BodyPlacement} only {@link extend} hands over. */
interface DefineOptions extends KindOptions {
  readonly body?: BodyPlacement;
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
 *
 * One overload per locus: {@link KindFacts} discriminates on the nested `locus.kind`,
 * which the compiler cannot narrow a union by, so a single union parameter reports a
 * wrong field against an arbitrary branch. Per-locus overloads let it judge the branch
 * the author wrote, and that branch's {@link Remedy} slots are what its error prints.
 */
export function kind<T extends object>(typed: Extract<KindFacts<T>, { readonly locus: { readonly kind: "at" } }>, options?: KindOptions): KindDefinition<T>;
export function kind<T extends object>(typed: Extract<KindFacts<T>, { readonly locus: { readonly kind: "embedded" } }>, options?: KindOptions): KindDefinition<T>;
export function kind<T extends object>(typed: Extract<KindFacts<T>, { readonly locus: { readonly kind: "nested-file" } }>, options?: KindOptions): KindDefinition<T>;
export function kind<T extends object>(typed: KindFacts<T>, options: KindOptions = {}): KindDefinition<T> {
  return defineKind(typed, options);
}

/** {@link kind} over the whole facts union — the body every overload shares, and the entry
 * {@link relocate} takes, since a relocated facts value is any locus's. */
function defineKind<T extends object>(typed: KindFacts<T>, options: DefineOptions): KindDefinition<T> {
  // The erasure, spelled: `T` binds the witness and the field slots at the keystroke and
  // then leaves. `keyof T` reads contravariantly, so the compiler will not widen a slot
  // set bound to `T` to the erased `string` on its own — every key of `T` is a string
  // the erased surface admits, which is all this assertion claims.
  const facts = typed as KindFacts;
  if (collectionAddressOf(facts) !== undefined) manifestOf(facts);
  const body = options.body;
  const plain = new Set(body?.plain ?? []);
  const construct = (init: MemberInit<T>): Member => ({
    kind: facts.name,
    facts,
    name: init.name,
    host: hostOf(facts, init),
    prose: memberProse(
      body?.render === undefined ? init.prose : body.render(init as Readonly<Record<string, unknown>>),
      init.name,
      facts.name,
    ),
    fields: orderedFields(facts, init, plain),
    satisfies: init.satisfies ?? [],
    requires: init.requires ?? {},
    needs: init.needs ?? [],
    inputs: init.inputs ?? [],
  });
  return Object.assign(construct, {
    facts,
    key: facts.name,
    render: options.render,
    ...(body === undefined ? {} : { body }),
  });
}

/** The `at`-locus branch of {@link KindFacts} — the one that declares a registration set
 * and may carry a collection address; the other loci hold {@link Remedy} slots there. */
type AtKindFacts = Extract<KindFacts, { readonly locus: { readonly kind: "at" } }>;

function isAtLocus(facts: KindFacts): facts is AtKindFacts {
  return facts.locus.kind === "at";
}

/** A kind's registration channel set — declared on an `at` locus, empty on every other. */
export function registrationOf(facts: KindFacts): readonly Registration[] {
  return isAtLocus(facts) ? facts.registration : [];
}

/** A kind's collection address — only an `at` locus names a manifest to key inside. */
export function collectionAddressOf(facts: KindFacts): CollectionAddress | undefined {
  return isAtLocus(facts) ? facts.collectionAddress : undefined;
}

/**
 * The host manifest a registration kind surfaces in — its `at` locus glob, which must
 * name exactly one file (no `*`): a starred glob governs a family of files, and a
 * collection address keys inside one.
 *
 * # Throws
 * If the kind's locus is not `at`, or its glob carries a `*`.
 */
export function manifestOf(facts: KindFacts): string {
  if (facts.locus.kind !== "at" || facts.locus.glob.includes("*")) {
    const spelled = facts.locus.kind === "at" ? `\`${facts.locus.glob}\`` : `a \`${facts.locus.kind}\` locus`;
    throw new Error(
      `kind \`${facts.name}\`: a collection address keys inside the one manifest the kind's ` +
        `locus names, and ${spelled} names no single file — declare an \`at\` locus whose ` +
        `glob carries no \`*\` (specs/model/representation.md, "Reach").`,
    );
  }
  return facts.locus.glob;
}

/**
 * A **relocation delta** — the facts a relocated built-in kind diverges from its base
 * on. Four faces, any one or more:
 *
 * - `edgeFields`, *added* to whatever the base already declares (never replacing them,
 *   which would drop a shipped kind's own edges silently). Each added field names a key
 *   of the relocated kind's typed surface `T`, so an edge can never be declared over a
 *   field the kind does not carry.
 * - `governs`, *replacing* the base's `at` locus root and glob — moving where the kind's
 *   members are found, the one fact the engine's own overlay exists to apply
 *   (`src/compose.rs`'s `overlay_builtin_kind`).
 * - `exclude`, *appended* to the base exclusions the (possibly moved) locus still
 *   reaches — leaving paths the built-in would claim ungoverned (decision 0085).
 * - `guidance`, `fieldGuidance` and `cite`, *replacing* the base's counsel — the
 *   relocated kind's guidance rule carries exactly what the relocation declares, and a
 *   relocation declaring none carries none: a built-in's own shipped counsel never
 *   projects into an adopter's harness (decision 0091).
 *
 * Every other fact — format, unit shape, registration, content, templates, the declared
 * leaf set — rides through unchanged, which is exactly what makes the emitted row still
 * read as a relocation rather than a name collision on the reading side.
 */
export interface KindRelocation<T extends object> {
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
  /**
   * Paths the relocated kind leaves ungoverned, *appended* to the base exclusions its
   * template (moved or not) still reaches — the way an author answers a collision with a
   * built-in, or leaves a fixture tree holding harness files ungoverned. Each is spelled
   * as {@link Locus}'s `exclude` is: a starless, harness-relative literal the template
   * matches, or a bare directory it reaches beneath. Only an `at` kind carries a locus
   * to exclude from.
   */
  readonly exclude?: readonly string[];
  /** Counsel for the relocated kind as a whole, projected as its path-scoped guidance rule. */
  readonly guidance?: string;
  /** Counsel per field of the relocated kind's surface, riding the same rule. */
  readonly fieldGuidance?: FieldGuidance<T>;
  /** External-fact source backing the relocation's guidance. */
  readonly cite?: string;
}

/**
 * The locus a `governs` delta moves the base to: the base's `at` root and glob replaced,
 * every other locus fact (its commitment class among them) riding through. Of the base's
 * exclusions it keeps each the new template still matches or reaches beneath — moving one
 * loader's reach moves no other's — and drops the rest, which the engine would refuse as
 * subtracting nothing.
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
  const pattern = locusPattern(governs.root, governs.glob);
  const kept = (base.locus.exclude ?? []).filter(
    (path) => templateMatches(pattern, path) || templateReachesBeneath(pattern, path),
  );
  const { exclude: _replaced, ...rest } = base.locus;
  return { ...rest, root: governs.root, glob: governs.glob, ...(kept.length > 0 ? { exclude: kept } : {}) };
}

/**
 * The locus an `exclude` delta leaves: `moved` (or the base's own `at` locus, when the
 * delta moves nothing) with the authored paths appended to the exclusions it already
 * carries.
 *
 * # Throws
 * If the base's locus is not `at` — there is no template to subtract from. If an
 * authored path is not starless, is not one the template matches or reaches beneath
 * (the engine refuses it as subtracting nothing when it loads the lock), or is already
 * excluded — a second entry for one path subtracts nothing new.
 */
function excludedLocus(
  base: KindFacts,
  moved: Extract<Locus, { kind: "at" }> | undefined,
  authored: readonly string[],
): Extract<Locus, { kind: "at" }> {
  if (base.locus.kind !== "at") {
    throw new Error(
      `relocating kind \`${base.name}\`: an \`exclude\` delta subtracts paths from the ` +
        `glob a kind's members are found at, and this kind's locus is \`${base.locus.kind}\` ` +
        `— it governs no glob of its own. Drop the \`exclude\` face of the delta ` +
        `(specs/model/representation.md, "locus").`,
    );
  }
  const locus = moved ?? base.locus;
  const pattern = locusPattern(locus.root, locus.glob);
  const exclude = [...(locus.exclude ?? [])];
  for (const path of authored) {
    const starless = !/[*?[\]{}\\]/.test(path) && !path.split("/").some((part) => part === "");
    if (!starless || !(templateMatches(pattern, path) || templateReachesBeneath(pattern, path))) {
      throw new Error(
        `relocating kind \`${base.name}\`: excluded path \`${path}\` must be a starless, ` +
          `harness-relative path the locus template \`${pattern}\` matches, or a bare ` +
          `directory it reaches beneath — anything else subtracts nothing ` +
          `(specs/model/representation.md, "locus").`,
      );
    }
    if (exclude.includes(path)) {
      throw new Error(
        `relocating kind \`${base.name}\`: path \`${path}\` is already excluded, and a ` +
          `second exclusion of one path subtracts nothing new.`,
      );
    }
    exclude.push(path);
  }
  return { ...locus, exclude };
}

/** A file locus as one harness-relative template, a root-locus kind's `.` dropped. */
function locusPattern(root: string, glob: string): string {
  return root === "." ? glob : `${root}/${glob}`;
}

/**
 * Whether every path the template `sub` matches, `base` matches too — the engine's
 * `locus_within`, decided over the same witnesses: each `*` a fresh token no literal
 * matches, a `**` zero, one and two fresh components.
 */
function locusWithin(sub: string, base: string): boolean {
  const fresh = "\u0001";
  const segments = sub.split("/").map((segment) => (segment === "**" ? segment : segment.split("*").join(fresh)));
  const depths = segments.includes("**") ? [0, 1, 2] : [0];
  return depths.every((depth) =>
    templateMatches(
      base,
      segments.flatMap((segment) => (segment === "**" ? Array.from({ length: depth }, (_, n) => `${fresh}${n}`) : [segment])).join("/"),
    ),
  );
}

/** Whether one path component matches one template segment: a literal, or one `*`. */
function segmentMatches(segment: string, component: string): boolean {
  const star = segment.indexOf("*");
  if (star < 0) {
    return segment === component;
  }
  const prefix = segment.slice(0, star);
  const suffix = segment.slice(star + 1);
  return component.length >= prefix.length + suffix.length && component.startsWith(prefix) && component.endsWith(suffix);
}

/** Whether every leading component matches the template segment at its position. */
function segmentsMatch(components: readonly string[], segments: readonly string[]): boolean {
  return components.every((component, i) => i >= segments.length || segmentMatches(segments[i], component));
}

/**
 * Whether the placement template `pattern` matches `path` whole — a `**` segment standing
 * for any number of components, none included. Mirrors the engine's glob match over the
 * placement-template subset, the only syntax a locus may carry.
 */
function templateMatches(pattern: string, path: string): boolean {
  const template = pattern.split("/");
  const components = path.split("/");
  const anyDepth = template.indexOf("**");
  if (anyDepth < 0) {
    return components.length === template.length && segmentsMatch(components, template);
  }
  const root = template.slice(0, anyDepth);
  const body = template.slice(anyDepth + 1);
  return (
    components.length >= root.length + body.length &&
    segmentsMatch(components.slice(0, root.length), root) &&
    segmentsMatch(components.slice(components.length - body.length), body)
  );
}

/**
 * Whether the placement template `pattern` matches some path strictly beneath the
 * directory `dir` — the engine's reach test for a directory exclusion (`src/kind.rs`,
 * `template_reaches_beneath`), spelled over the same segments.
 */
function templateReachesBeneath(pattern: string, dir: string): boolean {
  const template = pattern.split("/");
  const components = dir.split("/");
  const prefixMatches = (prefix: readonly string[], segments: readonly string[]) =>
    prefix.length < segments.length && segmentsMatch(prefix, segments);
  const anyDepth = template.indexOf("**");
  if (anyDepth < 0) {
    return prefixMatches(components, template);
  }
  const root = template.slice(0, anyDepth);
  const body = template.slice(anyDepth + 1);
  if (components.length <= root.length) {
    return segmentsMatch(components, root);
  }
  if (!segmentsMatch(components.slice(0, root.length), root)) {
    return false;
  }
  for (let start = root.length; start <= components.length; start += 1) {
    if (prefixMatches(components.slice(start), body)) {
      return true;
    }
  }
  return false;
}

/**
 * **Relocate** a built-in kind: the sanctioned way an adopting corpus moves a kind it
 * does not own to its own root, adds an edge field to it, excludes paths from it, or any
 * combination. Returns a fresh constructor over the widened typed surface `T` (the
 * base's fields plus the added edge fields, spelled by the caller as one interface),
 * carrying the base's facts with `delta`'s edge fields appended, `delta`'s `governs` in
 * place of the base's locus root and glob, `delta`'s `exclude` appended to the locus's
 * exclusions, and the base's own {@link KindDefinition.render} hook preserved. Ownership,
 * not privilege — a relocated built-in is an ordinary kind value from here on, and its
 * added edge reaches the lock as an assembly `edge` row keyed by `from`, exactly as any
 * kind's does (`declarations.ts`), never as a column on a kind-fact row.
 *
 * A moved locus, by contrast, *is* a kind-fact row column pair: the emitted row carries
 * the delta's `governs_root`/`governs_glob` while `format`, `unit_shape` and
 * `registration` stay the base's — which is precisely the three-fact test the lock
 * reader applies before overlaying the row onto its compiled-in built-in
 * (`src/compose.rs`'s `row_relocates_builtin`), so the engine reads members at the new
 * root rather than treating the row as a colliding kind. An `exclude` delta rides the
 * same row's `exclude` column — the reached base exclusions, then the authored ones —
 * which the overlay writes back whole.
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
 * names a base whose locus is not `at` ({@link relocatedLocus}), or an `exclude` delta
 * does, or names a path the locus cannot subtract ({@link excludedLocus}). And if the
 * delta declares no face, which would mint a marker-bearing clone of the base — a second
 * same-named kind diverging on nothing, which is a name collision spelled as a
 * relocation.
 */
export function relocate<T extends object>(
  base: KindDefinition<any>,
  delta: KindRelocation<T>,
): KindDefinition<T> {
  const { name: _name, ...relocation } = delta as KindExtension<T>;
  // A relocation keeps the base's surface by construction — it adds no plain field — so
  // the surface witness `extend` demands holds; the generic `T` only defers its proof.
  return extend<any, T>(base, relocation as KindExtension<T> & KeepsBaseSurface<any, T>);
}

/**
 * The witness that an extension's surface `T` keeps every field of its base's `B` —
 * `unknown` when it does, a {@link Remedy} the delta cannot supply when it drops one.
 * `T extends B` alone admits dropping an *optional* base field (a surface without it is
 * still assignable), so the keys are compared on their own.
 */
export type KeepsBaseSurface<B, T> = [Exclude<keyof B, keyof T>] extends [never]
  ? unknown
  : {
      readonly droppedBaseFields: Remedy<"a subkind keeps every field of its base's surface — extend the base's interface rather than omit one of its fields">;
    };

/**
 * An **extension delta** — a {@link KindRelocation} plus an optional `name`. A name other
 * than the base's mints a subkind; none (or the base's own) is a relocation.
 */
export interface KindExtension<T extends object> extends KindRelocation<T> {
  /** The subkind's name; omitted, the extension relocates the base under its own. */
  readonly name?: string;
  /**
   * The **plain fields** this extension adds over the widened surface `T`, each with the
   * place the base's format admits it — never a new frontmatter key, which the base's
   * documented format does not carry:
   *
   * - `"region"` — a `field` region of the base's declared {@link Layout} whose slot is
   *   the field's name; the member's body holds it under that heading, read as the base
   *   reads it.
   * - `"render"` — this delta's body {@link KindExtension.render}, which builds the
   *   member's body from its values; admitted where the base's body is one verbatim
   *   prose value (`file` content).
   */
  readonly fields?: { readonly [K in keyof T & string]?: "region" | "render" };
  /**
   * The extension's body **render** — the template a member's body is built from: its
   * typed values in (the `"render"`-placed fields and its own `prose` among them), its
   * body out. Declared only beside a `"render"`-placed field. Over a base that renders its
   * own body, the base's render builds the `prose` this one receives.
   */
  readonly render?: (values: T) => Prose;
  /**
   * A **subkind of a nested kind** this host names as its child: the host's template
   * layer whose child is `template`'s base takes `template` in its place, keeping the
   * layer's path. A nested member's kind is its host's template's, so this is the one
   * way to place a nested subkind's documents — it declares no locus of its own.
   */
  readonly template?: KindDefinition<any>;
}

/**
 * **Extend** a kind. Under a new `name` the result is a **subkind**: it keeps the base's
 * facts, adds `delta`'s edge fields, plain fields and counsel, governs a locus within the
 * base's (the base's own unless `governs` moves it), and names the base in its `base`
 * fact — the engine then reads a document in that locus as the subkind's one member.
 * Under the base's name it is the relocation {@link relocate} documents, the same
 * operation.
 *
 * A plain field lives where the base's format admits it ({@link KindExtension.fields}):
 * a `field` region of the base's layout, or the body the delta's `render` builds. Either
 * way a member never projects it as a frontmatter key, so the subkind's kind-fact row and
 * its members' headers stay the base's documented format.
 *
 * The surface is typed: `T extends B` and {@link KeepsBaseSurface} make `tsc` refuse a
 * delta whose surface retypes or drops a base field, optional ones included — every
 * selection of the base binds the subkind's documents, so its members must read as the
 * base's. Spell both: `extend<Rule, Standard>(rule, { … })`.
 *
 * # Throws
 * As {@link relocate}; for a subkind, if the base's locus is embedded, a nested base's
 * subkind declares a locus, or an `at` subkind's locus does not lie within its base's; if
 * a `template` names no subkind of a layer the host templates; and if a plain field has no place the base's format
 * admits, or a `render` places none ({@link placedBody}).
 */
export function extend<B extends object, T extends B>(
  base: KindDefinition<B, any>,
  delta: KindExtension<T> & KeepsBaseSurface<B, T>,
): KindDefinition<T> {
  const name = delta.name ?? base.facts.name;
  const subkind = name !== base.facts.name;
  if (subkind) {
    if (base.facts.locus.kind === "embedded") {
      throw new Error(
        `extending kind \`${base.facts.name}\` as \`${name}\`: a subkind governs a locus within ` +
          `its base's, and this kind's locus is \`embedded\` — it governs no glob ` +
          `of its own (specs/model/representation.md, "kind").`,
      );
    }
    if (base.facts.locus.kind === "nested-file" && (delta.governs !== undefined || (delta.exclude ?? []).length > 0)) {
      throw new Error(
        `extending kind \`${base.facts.name}\` as \`${name}\`: a nested kind's documents are placed by ` +
          `its host's template, so its subkind declares no locus — name it from an extended host ` +
          `instead: \`extend(host, { name, template: ${name} })\` (specs/model/representation.md, "kind").`,
      );
    }
    if (delta.governs !== undefined && base.facts.locus.kind === "at") {
      const own = locusPattern(delta.governs.root, delta.governs.glob);
      const held = locusPattern(base.facts.locus.root, base.facts.locus.glob);
      if (!locusWithin(own, held)) {
        throw new Error(
          `extending kind \`${base.facts.name}\` as \`${name}\`: the subkind's locus \`${own}\` ` +
            `lies outside the base's \`${held}\`, and a subkind governs a locus within its ` +
            `base's (specs/model/representation.md, "kind").`,
        );
      }
    }
  }
  const declaredEdges = delta.edgeFields ?? [];
  const counselled = delta.guidance !== undefined || delta.fieldGuidance !== undefined;
  const body = placedBody(base, name, delta as KindExtension<Record<string, unknown>>, declaredEdges);
  const templates = delta.template === undefined ? undefined : hostedTemplates(base, name, delta.template);
  if (
    !subkind &&
    body === base.body &&
    templates === undefined &&
    declaredEdges.length === 0 &&
    delta.governs === undefined &&
    (delta.exclude ?? []).length === 0 &&
    !counselled
  ) {
    throw new Error(
      `relocating kind \`${base.facts.name}\`: a relocation declares at least one diverging ` +
        `fact — \`edgeFields\`, \`fields\`, \`governs\`, \`exclude\`, \`template\`, or guidance. A delta declaring none mints a ` +
        `second kind of the base's own name that diverges on nothing, which reads as a name ` +
        `collision rather than a relocation (specs/model/representation.md, "kind").`,
    );
  }
  const authored = delta.exclude ?? [];
  const moved = delta.governs === undefined ? undefined : relocatedLocus(base.facts, delta.governs);
  const locus = authored.length === 0 ? moved : excludedLocus(base.facts, moved, authored);
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
  // The base's counsel is dropped, not inherited: the relocation's own, where declared,
  // takes its place, so a guidance rule never projects a built-in's shipped counsel.
  const { guidance: _guidance, fieldGuidance: _fieldGuidance, cite: _cite, ...uncounselled } = base.facts;
  const { relocates: _relocates, base: _base, ...unmarked } = uncounselled;
  const relocated = {
    ...(subkind ? { ...unmarked, name, base: base.facts.name } : { ...uncounselled, relocates: base.facts.name }),
    edgeFields,
    ...(templates === undefined ? {} : { templates }),
    ...(delta.guidance === undefined ? {} : { guidance: delta.guidance }),
    ...(delta.fieldGuidance === undefined ? {} : { fieldGuidance: delta.fieldGuidance }),
    ...(delta.cite === undefined ? {} : { cite: delta.cite }),
  };
  // Two spellings, not one with an optional `locus`: `KindFacts` is a union discriminated
  // on the locus, so the moved case must carry the `at` locus as its own literal branch.
  // A moved locus is only ever an `at` base's (`relocatedLocus` refuses the rest), and an
  // `at` kind declares its registration set — so the base's set and address ride through
  // as declared, read off the `at` branch rather than the erased union's remedy slots.
  const { registration: _registration, collectionAddress: _address, ...unregistered } = relocated;
  const address = collectionAddressOf(base.facts);
  const facts: KindFacts =
    locus === undefined
      ? relocated
      : {
          ...unregistered,
          locus,
          registration: registrationOf(base.facts),
          ...(address === undefined ? {} : { collectionAddress: address }),
        };
  // The base's leaf-set witness rides through as data, and the assertion is that
  // pass-through spelled: a relocation appends *edge fields* over the kind's own surface,
  // never a leaf, so the set the base's own constructor already bound still names what a
  // member carries. `T` here is the widened surface (the base's fields plus the added
  // edges), so re-binding the witness to `keyof T` would demand the edge fields be leaves.
  return defineKind<T>(facts as KindFacts<T>, { render: base.render, ...(body === undefined ? {} : { body }) });
}

/**
 * The host's templates with `child` named in place of its base: the one layer whose child
 * is `child`'s base takes `child`, path kept.
 *
 * # Throws
 * If `child` is no subkind, or the host templates no layer of `child`'s base.
 */
function hostedTemplates(host: KindDefinition<any>, name: string, child: KindDefinition<any>): readonly Template[] {
  const at = `extending kind \`${host.facts.name}\`${name === host.facts.name ? "" : ` as \`${name}\``}`;
  const childBase = child.facts.base;
  if (childBase === undefined) {
    throw new Error(
      `${at}: \`template\` names kind \`${child.facts.name}\` as the host's child, and it extends no ` +
        `kind — a template names a subkind of a child the host already templates ` +
        `(specs/model/representation.md, "kind").`,
    );
  }
  const declared = host.facts.templates ?? [];
  if (!declared.some((layer) => layer.kind.key === childBase)) {
    throw new Error(
      `${at}: \`template\` names \`${child.facts.name}\`, a subkind of \`${childBase}\`, and the ` +
        `host templates no \`${childBase}\` layer for it to take the place of ` +
        `(specs/model/representation.md, "kind").`,
    );
  }
  return declared.map((layer) => (layer.kind.key === childBase ? { ...layer, kind: child } : layer));
}

/**
 * The {@link BodyPlacement} an extension of `base` carries: the base's own, with the
 * delta's plain fields appended and its render composed over the base's. Returns
 * `base.body` itself when the delta places nothing, which is how {@link extend} tells a
 * delta adding no plain field.
 *
 * # Throws
 * If a plain field is already one the base places, or one the delta declares an edge
 * over; if a `"region"` field names no `field` region of the base's layout; if a
 * `"render"` field sits over a base whose body is not one verbatim prose value (a layout,
 * a fields-only kind, an embedded locus), or the delta declares no render to place it;
 * and if a render is declared that places no field.
 */
function placedBody(
  base: KindDefinition<any>,
  name: string,
  delta: KindExtension<Record<string, unknown>>,
  edges: readonly { readonly field: string }[],
): BodyPlacement | undefined {
  const declared = Object.entries(delta.fields ?? {}).filter(
    (entry): entry is [string, "region" | "render"] => entry[1] !== undefined,
  );
  const at = `extending kind \`${base.facts.name}\`${name === base.facts.name ? "" : ` as \`${name}\``}`;
  const inherited = base.body?.plain ?? [];
  const edged = new Set([...(base.facts.edgeFields ?? []).map((edge) => edge.field), ...edges.map((edge) => edge.field)]);
  const slots = new Set(
    (base.facts.content?.regions ?? []).flatMap((region) => (region.region === "field" ? [region.slot] : [])),
  );
  const prose = base.facts.content === undefined && base.facts.shape !== "fields" && base.facts.locus.kind !== "embedded";
  let rendered = false;
  for (const [field, place] of declared) {
    if (inherited.includes(field) || edged.has(field) || FRAMEWORK_KEYS.has(field) || field === base.facts.identityField) {
      throw new Error(
        `${at}: plain field \`${field}\` is already a field the kind carries — a framework key, ` +
          `its identity, an edge field, or a plain field its base placed — and one field takes ` +
          `one place (specs/model/representation.md, "kind").`,
      );
    }
    if (place === "region" && !slots.has(field)) {
      throw new Error(
        `${at}: plain field \`${field}\` is placed in a body region, and the base's layout ` +
          `declares no \`field\` region of slot \`${field}\` — a field lives where the base's ` +
          `format admits it, never as an undocumented key. Place it with a \`render\` over a ` +
          `\`file\`-content base, or extend a base whose layout holds the slot ` +
          `(specs/model/representation.md, "kind").`,
      );
    }
    if (place === "render" && !prose) {
      throw new Error(
        `${at}: plain field \`${field}\` is placed by the kind's render, and the base's body is ` +
          `not one verbatim prose value — a layout reads its body by region, a fields-only or ` +
          `embedded kind writes no body of its own — so a render has nowhere to put it. A field ` +
          `lives where the base's format admits it, never as an undocumented key ` +
          `(specs/model/representation.md, "kind").`,
      );
    }
    if (place === "render" && delta.render === undefined) {
      throw new Error(
        `${at}: plain field \`${field}\` is placed by the kind's render, and the delta declares ` +
          `no \`render\` — declare one that writes it into the body, or the field is an ` +
          `undocumented key (specs/model/representation.md, "kind").`,
      );
    }
    rendered ||= place === "render";
  }
  if (delta.render !== undefined && !rendered) {
    throw new Error(
      `${at}: a \`render\` places the plain fields a delta adds, and this one places none — ` +
        `name each it writes under \`fields\` as \`"render"\` (specs/model/representation.md, "kind").`,
    );
  }
  if (declared.length === 0) return base.body;
  const outer = delta.render as ((values: Readonly<Record<string, unknown>>) => Prose) | undefined;
  const inner = base.body?.render;
  const render =
    outer === undefined
      ? inner
      : inner === undefined
        ? outer
        : (values: Readonly<Record<string, unknown>>) => outer({ ...values, prose: inner(values) });
  return { plain: [...inherited, ...declared.map(([field]) => field)], ...(render === undefined ? {} : { render }) };
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
   * The target facts of each edge field this value *fills and resolves*, keyed by the edge
   * field's own name — the data a `render` hook selects to spell a reference. Two cases
   * read absent: an unfilled field (no edge at all), and a kind declaring no edge fields —
   * or a value composed off a bare kind name, which carries none. A third derives no facts
   * but does not read absent: a filled field whose target defers to `check`, naming a
   * declared `at`-locus kind's uncomposed member (`pipeline.md`, "Emit", the "Refusing"
   * bullet) — reading it refuses by name at emit, naming the member, the value, the field
   * and the authored address, rather than handing a hook an absent key to trip over. The
   * default TOML view is unaffected either way, writing the authored address leaf whether
   * the program resolves it or the gate does.
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
  refuseSegmentedKey(kindKey, init.key);
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
 * Refuse an embedded member's key that is not **one address segment** — empty, or carrying
 * the `/` the member-address grammar cuts an address at ({@link isOneSegment}). A member's
 * identity *is* its `<host-address>/<kind>/<key>` address, so a key carrying the separator
 * spells an address one segment too long: `authority/rejected` spells what the sibling
 * keyed `authority` spells for its `rejected` leaf, and every reader answers that address
 * at leaf grain.
 *
 * Refused at compose for the posture {@link refuseReservedLeaf} states for the sibling
 * case: a coincident address is loud at the authoring seam, where the author can still
 * rename, never at emit over bytes already written (0051). The engine's own judge is not
 * this one twice — it judges a committed lock (hand-edited, or written by an older SDK),
 * this one judges the composing program.
 *
 * Scoped to the member's own key: a `collections` entry key lands inside the **leaf tail**,
 * which the grammar defines as the whole remainder after the third slash, so it aliases
 * nothing and is deliberately left legal.
 *
 * # Throws
 * If `key` is empty or carries `/`.
 */
function refuseSegmentedKey(kind: string, key: string): void {
  if (isOneSegment(key)) return;
  throw new Error(
    `embedded member \`${kind}\` \`${key}\`: a member's key is one address segment — ` +
      `non-empty and carrying no \`/\`, the separator its address is cut at`,
  );
}

/**
 * Refuse a composed leaf named {@link RESERVED_LEAF} — the key a read member's own span
 * owns, so a second meaning under it is a coincident leaf address, refused at compose
 * rather than resolved by precedence (0051). Loud at the authoring seam, where the author
 * can rename the field, not at emit over bytes already written. The rename is the whole
 * remedy: an embedded value carries leaves and collections alone, so there is no
 * member-level prose here to move the words into.
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
      `member's own span — rename the field`,
  );
}
