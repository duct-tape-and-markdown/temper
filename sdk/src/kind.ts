/**
 * Kinds — the engine room (`specs/model/representation.md`, "A kind is a
 * constructor plus five facts"). A kind is a plain typed surface — an interface
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

/** Whether a member is a lone file (identity from the stem) or a directory with an entry file. */
export type UnitShape = "file" | "directory";

/**
 * A kind's **registration** — the declared edge between a member and the world
 * (fact 4, `15-kinds.md`, "Registration"). Reachability is graph reachability
 * from the world node over these edges.
 */
export type Registration =
  | { readonly via: "always" }
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
 * fenced blocks inside host documents (`genre`). An `at` locus is split root +
 * glob so the kind fact row carries `governs_root`/`governs_glob` directly.
 */
export type Locus =
  | { readonly kind: "at"; readonly root: string; readonly glob: string }
  | { readonly kind: "genre"; readonly withinHosts: readonly string[] };

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
  /** Fact 4, registration — the world edge. */
  readonly registration: Registration;
  /**
   * The frontmatter key the member's name writes under (a skill's `name`), or
   * absent when identity is the file stem (a rule). A layout detail: it shapes
   * the projected frontmatter, never the model.
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
 * Define a **genre** — a kind whose locus is `genre(within hosts)`: its members
 * live as typed fenced blocks inside host documents instead of at their own
 * paths (`15-kinds.md`, "A genre is a kind at the block locus"). Registration
 * inherits through the host, so a genre carries no world edge of its own.
 */
export function genre<T extends object>(facts: {
  name: string;
  provider?: string;
  withinHosts: readonly string[];
  edgeFields?: readonly EdgeField[];
}): KindDefinition<T> {
  return kind<T>({
    name: facts.name,
    provider: facts.provider,
    locus: { kind: "genre", withinHosts: facts.withinHosts },
    unitShape: "file",
    registration: { via: "always" },
    edgeFields: facts.edgeFields,
  });
}
