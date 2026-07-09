/**
 * Kinds — the engine room. A kind is a plain typed surface — an interface
 * `T` and a constructor `kind<T>()` — plus five facts of runtime residue: label,
 * locus, layout, registration, and edge fields. `tsc` is the keystroke wall; every
 * type erases at the seam, and what a kind leaves behind is those five facts,
 * riding the lock as rows. Identity travels by import, never by string — a `kind`
 * reference is the imported value (`15-kinds.md`, the built-ins-are-a-module
 * Decision).
 */

import type { Prose } from "./prose.js";
import type { Capability } from "./needs.js";
import type { Requirement } from "./contract.js";

/** The shape of the on-disk artifact a member projects to (fact 3, layout). */
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
  | { readonly via: "connection" };

/** One of a kind's fields that is a reference to another member — a graph edge (fact 5). */
export interface EdgeField {
  readonly field: string;
  /** The target kind's name — the far end of the edge. */
  readonly to: string;
}

/**
 * A kind's **locus** (fact 2): members live at path globs (`at`) or as typed
 * fenced blocks inside host documents (`embedded`). An `at` locus is split root +
 * glob so the kind fact row carries `governs_root`/`governs_glob` directly.
 */
export type Locus =
  | { readonly kind: "at"; readonly root: string; readonly glob: string }
  | { readonly kind: "embedded"; readonly withinHosts: readonly string[] };

/** The five facts of a kind's runtime residue (`15-kinds.md`). */
export interface KindFacts {
  /** Fact 1, label — the compiled debug label findings speak; the kind's name. */
  readonly name: string;
  /** The declared provider authority, when the kind qualifies by one. */
  readonly provider?: string;
  /** Fact 2, locus — where members live. */
  readonly locus: Locus;
  /** Fact 3a, layout — the projection format; omitted for a frontmatterless kind. */
  readonly format?: Format;
  /** Fact 3b, layout — the on-disk unit shape. */
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
}

/**
 * One authored member — a typed value in the library (`20-surface.md`, "The
 * member"). Kind identity travels by import (`facts`), never by string; the
 * typed fields are flat at the top level, carried as an ordered pair list so the
 * projected frontmatter key order is the author's.
 */
export interface Member {
  /** The kind's name — its declaration-row and lock identity. */
  readonly kind: string;
  /** The kind's five facts — carried for projection and the declaration rows. */
  readonly facts: KindFacts;
  /** Identity within the kind. */
  readonly name: string;
  /** The member's words (`20-surface.md`, "Prose"). */
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
const FRAMEWORK_KEYS = new Set(["name", "prose", "satisfies", "requires", "needs"]);

/** The init a kind constructor takes — the framework keys plus the kind's typed fields `T`. */
export type MemberInit<T> = {
  readonly name: string;
  readonly prose?: Prose;
  readonly satisfies?: readonly string[];
  readonly requires?: Readonly<Record<string, Requirement>>;
  readonly needs?: readonly Capability[];
} & T;

/**
 * A kind — a callable constructor carrying its five facts. Calling it builds a
 * member; `key` (its name) keys `expect` and a `kind` reference in a requirement.
 * The value *is* the identity (`15-kinds.md`, "identity travels by import").
 */
export interface KindDefinition<T> {
  (init: MemberInit<T>): Member;
  readonly facts: KindFacts;
  readonly key: string;
}

/**
 * Build the ordered projected-frontmatter fields for a member: nothing for a
 * frontmatterless kind (memory declares no `format`), else the identity field
 * (when the kind writes its name into frontmatter) followed by the typed fields
 * in the author's declared order.
 */
function orderedFields(facts: KindFacts, init: MemberInit<object>): Array<readonly [string, unknown]> {
  if (facts.format === undefined) return [];
  const typed: Array<readonly [string, unknown]> = [];
  for (const [key, value] of Object.entries(init)) {
    if (!FRAMEWORK_KEYS.has(key)) typed.push([key, value]);
 }
  const head: Array<readonly [string, unknown]> =
    facts.identityField !== undefined ? [[facts.identityField, init.name]] : [];
  return [...head, ...typed];
}

/**
 * Define a kind (`15-kinds.md`). Returns a constructor over the kind's typed
 * fields `T`; every type erases at the seam, so what the returned member carries
 * into emit is the five facts plus flat field data.
 */
export function kind<T extends object>(facts: KindFacts): KindDefinition<T> {
  const construct = (init: MemberInit<T>): Member => ({
    kind: facts.name,
    facts,
    name: init.name,
    prose: init.prose,
    fields: orderedFields(facts, init),
    satisfies: init.satisfies ?? [],
    requires: init.requires ?? {},
    needs: init.needs ?? [],
  });
  return Object.assign(construct, { facts, key: facts.name });
}

/**
 * One entry in a sibling collection: its own key plus its leaf fields
 * (`rejected."baked-projection"`) — an ordered list element, never a positional
 * index; the entry's `key` is what a leaf address carries (`20-surface.md`, the
 * leaf-address Decision).
 */
export interface EmbeddedMemberCollectionEntry {
  /** The entry's key among its collection's siblings. */
  readonly key: string;
  /** The entry's own leaf fields: field name → authored string. */
  readonly leaves: Readonly<Record<string, string>>;
}

/**
 * An **embedded member's** composed value (posture 3, passed to `blocks()`):
 * leaves are authored strings keyed by field name; sibling collections are keyed
 * by collection name, each an authored-order list of entries — leaf addresses
 * are structural and keyed (`20-surface.md`, the leaf-address Decision). Read
 * back byte-identically by the engine's `parse_embedded_member` fold off the
 * `member.<kind> <key>` fence `blocks()` renders (`src/extract.rs`). There is no
 * prescribed child-kind ontology — a corpus that wants one declares its own
 * child kind with the same machinery.
 */
export interface EmbeddedMemberValue {
  /** The child kind this value instantiates — the fence info string's `member.<kind>`. */
  readonly kind: string;
  /** The value's key — the identity a leaf address carries (`surface-authority`). */
  readonly key: string;
  /** Prose leaves: authored strings, law-5 protected one by one. */
  readonly leaves: Readonly<Record<string, string>>;
  /** Sibling collections: collection name → its entries, in authored order. */
  readonly collections: Readonly<Record<string, readonly EmbeddedMemberCollectionEntry[]>>;
}

/** Compose an embedded member's value for `blocks()` — the shape any project's own child kind uses. */
export function embeddedMemberValue(init: {
  kind: string;
  key: string;
  leaves: Readonly<Record<string, string>>;
  collections?: EmbeddedMemberValue["collections"];
}): EmbeddedMemberValue {
  return {
    kind: init.kind,
    key: init.key,
    leaves: init.leaves,
    collections: init.collections ?? {},
  };
}
