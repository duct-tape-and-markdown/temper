/**
 * Contracts — clauses and requirements as typed values.
 * A clause is `predicate · severity · guidance · cite`; a requirement is
 * `prose · kind · required · clauses? · verifier?`. Both erase to compiled data
 * at the seam: the author composes typed objects, the engine
 * consumes their rows. The predicate vocabulary is the closed algebra — a clause
 * outside it is a squiggle, not a runtime rejection.
 */

import type { Shape, ValueType } from "./generated/index.js";
import type { KindDefinition, Member, Remedy } from "./kind.js";

/** A clause's delivery posture: `required` gate-blocks, `advisory` reports. */
export type Severity = "required" | "advisory";

/**
 * A member of the closed predicate algebra. Both a kind's own `expect` clauses and a
 * requirement's `clauses` compile to the row's full `key`/`field`/`severity`/argument
 * shape (`declarations.ts` `clauseRow`) — one spelling, whichever selection the clause
 * binds to: `bound`/`charset`/`keys`/`values` ride the row alongside
 * `count`/`target`/`degree`, so the lock encodes the clause losslessly rather than
 * identity+severity alone.
 */
export interface Predicate {
  /** The predicate's clause key (`required`, `max_len`, `extent`, …). */
  readonly key: string;
  /**
   * The field (or marker) the predicate constrains, when it names one.
   *
   * A value predicate's `field` is an **addressing path**: name segments walk into an
   * object (`owner.name`), and `[*]` is the each-grain over an array's elements, so
   * `plugins[*].source` decides once per entry and indicts each offending one by its own
   * address. Nothing else is spellable — an index, a slice, a filter, and a recursive
   * descent are all refused when the contract is checked, not silently evaluated. The
   * subset is the whole surface on purpose: a clause names *where* a value lives, never
   * a pattern that matches it.
   *
   * `forbidden_keys` and `must_define` name a top-level **key**, not a path, and
   * `closed-keys` names neither — it reads the key set its sibling clauses declare.
   */
  readonly field?: string;
  /** The predicate's scalar bounds, keyed per predicate (`min`/`max`,
   * `incoming_min`/`incoming_max`/`outgoing_min`/`outgoing_max`). */
  readonly args?: Readonly<Record<string, number>>;
  /**
   * The **requirement name** a clause reads its source set from — `membership`'s
   * allowed-value target and `reached-from`'s closure roots, one naming scheme for
   * both. A separate slot from `field` since `membership` names a checked field too,
   * and `reached-from` names no field at all.
   */
  readonly target?: string;
  /**
   * `mention-reachable`'s and `trigger-agrees`'s **target-side gate field** — a separate
   * slot from `field` (which carries the source-side scope field) since they name a
   * field on *both* ends — and `listing`'s withholding field, the one that gates a member
   * out of the listing. Spelled to match the lock's own `gate` column.
   */
  readonly gate?: string;
  /**
   * `type`'s declared source kinds over the closed scalar/container lattice — the set
   * the field may carry any one of. Spelled to match the lock's own `value_type`
   * column.
   */
  readonly value_type?: readonly ValueType[];
  /**
   * `shape`'s declared shape — one member of the closed set, spelled to match the lock's
   * own `shape` column.
   */
  readonly shape?: Shape;
  /** `allowed_chars`'s declared character class. */
  readonly charset?: Charset;
  /** `forbidden_keys`'s forbidden key list. */
  readonly keys?: readonly string[];
  /** `enum`/`deny`'s permitted or forbidden value list. */
  readonly values?: readonly string[];
  /** `range`'s inclusive numeric bound. */
  readonly range?: { readonly min: number; readonly max: number };
  /** `section_contains`'s heading-text prefix and the marker each governed section must carry. */
  readonly section?: { readonly heading: string; readonly marker: string };
  /** `require_sections`'s required heading list. */
  readonly sections?: readonly string[];
  /**
   * A by-incidence clause's **field set** — the fields whose edges the selection is
   * filtered to. `degree`'s bound filter and `reached-from`'s via set both ride it:
   * one concept, one slot, mirroring the lock's own shared `fields` column rather than
   * a `degree`-private key. Absent ⇒ unfiltered, every edge at the member.
   */
  readonly fields?: readonly string[];
  /** `extent`'s declared unit — the render-side size proxy the bound is measured in. */
  readonly unit?: ExtentUnit;
}

/** The unit an `extent` bound is measured in — the closed set of stable render-side size
 * proxies. Token count is deliberately absent: a verdict that moves when a tokenizer
 * updates is a gate that changes its mind with no diff. */
export type ExtentUnit = "lines" | "characters";

/**
 * The character class `allowed_chars` admits — inclusive ranges plus individual
 * characters, e.g. `[a-z0-9-]`. Each range is a `"<lo>-<hi>"` two-character
 * span. `negated` turns the class into the forbidden set (`[^:]`); `position:
 * "leading"` governs only the first character (an empty value holds), where the
 * default governs every character.
 */
export interface Charset {
  readonly ranges?: readonly string[];
  readonly chars?: string;
  readonly negated?: boolean;
  readonly position?: "leading";
}

// Node-scope predicates. A value predicate takes an addressing path as its `field` — see
// `Predicate.field` for the subset an author may spell, and for the key-naming and
// fieldless exceptions that sit here too.
/**
 * A field or marker is present. Presence is asked of the path's trailing name segment,
 * so `required("plugins[*].source")` fires once per entry that omits it — and a path
 * ending in `[*]` names elements rather than a key, which is refused.
 */
export const required = (field: string): Predicate => ({ key: "required", field });
/**
 * The field's parsed source kind is one of the declared ones. `kinds` is a set: a
 * format that documents a field as `string|array` is gated by `["string", "list"]`,
 * and a one-element `["list"]` is the plain single-kind check.
 */
export const type = (field: string, kinds: readonly ValueType[]): Predicate => ({
  key: "type",
  field,
  value_type: kinds,
});
/** The field's value is at least `n` characters. */
export const minLen = (field: string, n: number): Predicate => ({ key: "min_len", field, args: { min: n } });
/** The field's value is at most `n` characters. */
export const maxLen = (field: string, n: number): Predicate => ({ key: "max_len", field, args: { max: n } });
/** The field's characters are drawn from a declared class (`allowed_chars`). */
export const allowedChars = (field: string, charset: Charset): Predicate => ({
  key: "allowed_chars",
  field,
  charset,
});
/**
 * The field's value holds one named `Shape` from the closed set the engine generates
 * across the seam — the union *is* the vocabulary, so an author picks a member and can
 * spell nothing else. There is no escape to a hand-written pattern, which is the point.
 *
 * - `hyphen-placement` — a hyphen neither leads, trails, nor doubles (`pdf-processing`,
 *   never `-pdf`, `pdf-`, or `pdf--processing`). Says nothing about the alphabet, which
 *   is `allowedChars`'s.
 * - `no-xml-tags` — the value carries no XML tag (`<br/>`, `</note>`, `<a href="x">`).
 *   Prose spelling a comparison (`use when x < y`) is not a tag and does not fire.
 */
export const shape = (field: string, shape: Shape): Predicate => ({ key: "shape", field, shape });
/**
 * The selected item's **rendered** extent is at most `bound`, in the declared `unit`
 * (`lines` or `characters`). Node-scope and render-side: the measure is the bytes the
 * item contributes to its projection, not the source body a count reads before a
 * reference resolves or a render hook runs.
 */
export const extent = (unit: ExtentUnit, bound: number): Predicate => ({
  key: "extent",
  unit,
  args: { max: bound },
});
/** The forbidden keys (e.g. the Cursor `globs`/`alwaysApply` keys) are absent. */
export const forbiddenKeys = (keys: readonly string[]): Predicate => ({ key: "forbidden_keys", keys });
/**
 * The kind's declared key set is exhaustive — a member carrying any other top-level key
 * is a finding. `forbiddenKeys`' complement: a deny-list names a finite set over an open
 * key space, this closes the space.
 *
 * It takes no arguments, and that is the point: the allow-list is the contract's own
 * `required`/`optional` clauses, so the key set is declared once. Adding `optional("x")`
 * admits `x` with no second edit here — and a contract declaring no key at all fails
 * admissibility rather than indicting every key of every member.
 */
export const closedKeys = (): Predicate => ({ key: "closed-keys" });
/** The field's value is none of `values` (forbidden values). */
export const deny = (field: string, values: readonly string[]): Predicate => ({
  key: "deny",
  field,
  values,
});
/** The named headings are present. */
export const requireSections = (sections: readonly string[]): Predicate => ({ key: "require_sections", sections });
/** The member's name matches its directory. */
export const nameMatchesDir = (): Predicate => ({ key: "name-matches-dir" });
/** Names are unique within the artifact kind (a scope-wide identity collision). */
export const uniqueName = (): Predicate => ({ key: "unique-name" });
/** No member of the kind sits at a shallower placement under the same leaf name (`apps:web:terse` shadows `terse`). */
export const unshadowed = (): Predicate => ({ key: "unshadowed" });
/**
 * The named field may be present — always satisfied, recording the key as part of a
 * declared (closed) schema. `dependency-exists` has no constructor: the engine holds
 * it back absent a decidable reference syntax, so a hand-authored clause would fail
 * admissibility.
 */
export const optional = (field: string): Predicate => ({ key: "optional", field });
/** The field's numeric value lies within the inclusive `[min, max]` bound. */
export const range = (field: string, min: number, max: number): Predicate => ({
  key: "range",
  field,
  range: { min, max },
});
/** The field's value is one of `values`. Spelled `enumOf` — `enum` is a reserved word. */
export const enumOf = (field: string, values: readonly string[]): Predicate => ({
  key: "enum",
  field,
  values,
});
/** The named body marker is defined (e.g. `disable-model-invocation`). */
export const mustDefine = (marker: string): Predicate => ({ key: "must_define", field: marker });
/** Every body section whose heading *starts with* `heading` carries `marker` in its body. */
export const sectionContains = (heading: string, marker: string): Predicate => ({
  key: "section_contains",
  section: { heading, marker },
});
/** Every glob the field carries parses under globset (brace-expansion aware). */
export const globValid = (field: string): Predicate => ({ key: "glob-valid", field });
/**
 * Every mention a selected member authors can fire where its target can be invoked. A
 * target whose `gateField` carries globs is gated — removed from every invocation
 * channel until a file tool (read, edit, write) touches a matching path, a shell or
 * search read of that path opening none — so a mention of it is actionable only
 * inside that gate. Fires on a scoped source whose `scopeField` globs are not contained
 * in the target's gate, and on an unscoped source mentioning a gated target.
 *
 * Generic over both ends, hard-coding no kind: the trigger is the target's gate field
 * carrying a value, never its registration set. Containment is judged over sample paths
 * each source glob matches, not decided, so a verdict can be wrong.
 */
export const mentionReachable = (scopeField: string, gateField: string): Predicate => ({
  key: "mention-reachable",
  field: scopeField,
  gate: gateField,
});
/**
 * The converse of {@link mentionReachable}: where a scoped member mentions a gated one,
 * the target's `gateField` globs must be contained in the source's `scopeField`, so the
 * two triggers of one area cannot drift apart. A gate reaching paths the scope does not
 * leaves the target invocable where the member naming it never loads. Silent on an
 * unscoped source and an ungated target; containment is judged by the same sample paths,
 * so a verdict can be wrong.
 */
export const triggerAgrees = (scopeField: string, gateField: string): Predicate => ({
  key: "trigger-agrees",
  field: scopeField,
  gate: gateField,
});
/**
 * Every selected member is reachable from the world — its own inbound registration edge
 * is live, or a reachable member imports it. A member whose every declared registration
 * channel is provably dead (a `paths` glob set matching no file, a blank
 * description-trigger field) and that no live importer reaches is authored configuration
 * the harness never loads.
 *
 * Names **no field**: the grain is each-grain over the *root* selection — the whole
 * governed forest — and the verdict is read off the reference graph's reachability
 * closure, never off any member's own fields. So it composes into a root `contract`,
 * where a member-grain clause is refused: there is no one kind whose schema a root
 * clause could name a field in.
 */
export const reachable = (): Predicate => ({ key: "reachable" });
/**
 * Every lock row a selected member owns still matches disk — its projection's byte
 * fingerprint, and each fingerprinted source dependency a layout import or a
 * composed-prose include recorded. A drifted row means the authored source changed and
 * `emit` has not run, or the emitted bytes were hand-edited.
 *
 * Drift is one comparison in one vocabulary, so one predicate carries all of it: a
 * hand-edited projection, a moved import target and a moved include target are the same
 * freshness fact over the same rows, and hardening freshness hardens it once.
 *
 * Names **no field**, like {@link reachable}: the argument is the committed lock read
 * against disk, never any member's own fields. So it composes into a root `contract`,
 * whose selection is the whole governed forest — a per-kind binding beside the root's
 * would double-report one row, and a user-declared kind carries no default contract to
 * bind it in.
 */
export const fresh = (): Predicate => ({ key: "fresh" });
/**
 * Every document discovery finds at a **governed locus** is one the lock declares a
 * member for — a file-content kind's committed document joined against the provenance
 * rows, a layout host's document against the `layout_source` record `emit` writes when
 * it reads one. A stranger at a governed locus is loaded by Claude Code and maintained
 * by nobody: `emit` never reaps it, `guard` never bound it, and every address under it
 * resolves to nothing.
 *
 * Kept apart from {@link fresh} rather than folded into it: a stale pin and an
 * undeclared document are different facts with different remedies — re-emit versus
 * declare the member — and a read-only ground kind draws the second routinely while
 * its pins stay fresh, so hardening one must not harden the other.
 *
 * Names **no field**, like {@link fresh}: the subject is the discovery walk read
 * against the lock's declarations, never any member's own fields. So it composes into a
 * root `contract` — what is governed is a locus, not a population, and a per-kind
 * binding would have no member to hang off.
 */
export const locusDeclared = (): Predicate => ({ key: "locus-declared" });
/**
 * The engine that wrote the committed lock is the engine running the gate. The lock
 * names the version that laid it down (`specs/model/pipeline.md`, "The lock"), and a
 * run by a different one says so: the rows it is reading were compiled by a compiler it
 * is not, so a normalized older spelling and a genuine behaviour change read alike from
 * the inside. The remedy is one `emit`, which rewrites the lock whole in the running
 * engine's canonical form.
 *
 * Names **no field**, like {@link locusDeclared}: the comparison is the lock's own
 * stamp against the running binary's version, never any member's fields. So it composes
 * into a root `contract` — the stamp is one fact about the whole harness, and there is
 * no per-kind population for it to range over.
 *
 * Lock-only and offline, like {@link fresh}: a lock recording no stamp reads *unknown*
 * rather than mismatched, and a harness with no committed lock has nothing to compare,
 * so both are silent.
 */
export const engineMatches = (): Predicate => ({ key: "engine-matches" });
/**
 * Every declared reference and mention a member carries resolves to exactly one target —
 * neither dangling nor a bare key several hosts share. Names no field, like
 * {@link fresh}: the route walk is one pass over the whole forest, so it composes into a
 * root `contract`.
 */
export const resolves = (): Predicate => ({ key: "resolves" });
/**
 * Every `@path` import a member's format executes resolves to a member or a repository
 * file. Names no field, and composes into a root `contract`, for the reason
 * {@link resolves} does; kept apart from it because an import resolves by path against
 * disk, never by identity.
 */
export const importBacked = (): Predicate => ({ key: "import-backed" });
/**
 * Every edge the member's kind declares is placed by the format that renders the member
 * — a format that omits one renders a contract the prose does not represent. Names no
 * field: the selection is the member's whole incident edge set, at the `each` grain.
 * Decided over the placement `emit` observes while rendering and lowers into the
 * member's `nested_member` row, since the engine never sees a format.
 */
export const formatPlacesEdges = (): Predicate => ({ key: "format-places-edges" });

// Set predicates — they range over the **selection** their clause binds to, not one
// member's own fields: a kind's `expect` binds them to that kind's whole population
// (the universal binding), a requirement's `clauses` to its opt-in satisfiers (the
// existential one). The quantifier is the clause's grain, so the same value says the
// same thing in either place, riding the same four-channel `clause()` shape as the
// member-scope predicates above.
/** The selection's size lies in the inclusive `[min, max]` bound. */
export const count = (bounds: { min?: number; max?: number }): Predicate => {
  const args: Record<string, number> = {};
  if (bounds.min !== undefined) args.min = bounds.min;
  if (bounds.max !== undefined) args.max = bounds.max;
  return { key: "count", args };
};
/** The field's extracted value does not repeat across the selection. */
export const unique = (field: string): Predicate => ({ key: "unique", field });
/**
 * The text the selection lists sums to at most `budget` characters. One member's entry is
 * its `fields` values joined in order, counted in characters and cut at `entry`; a member
 * whose `withheldBy` field is boolean `true` is not listed and adds nothing. The shape of a
 * runtime listing that carries every member's trigger text into each request under one
 * budget — Claude Code's skill listing is the shipped instance.
 */
export const listing = (
  fields: readonly string[],
  bounds: { readonly entry: number; readonly budget: number; readonly withheldBy?: string },
): Predicate => ({
  key: "listing",
  fields,
  gate: bounds.withheldBy,
  args: { entry: bounds.entry, max: bounds.budget },
});
/** Every selected member's `field` value is drawn from a feature over the selection `target` declares. */
export const membership = (field: string, target: string): Predicate => ({
  key: "membership",
  field,
  target,
});
/**
 * The in/out edge-count bound every selected member must land in. A bound naming no
 * direction constrains nothing and is refused at admissibility.
 *
 * `fields` filters the by-incidence selection both bounds range over — the clause's
 * own filter, not either direction's. Omit it and the bound ranges over every edge at
 * the member; name a set and it ranges over the union of those fields' edges, so two
 * fields to one target count two. Containment (`contains:<kind>`) counts only where a
 * filter names it.
 */
export const degree = (bounds: {
  incoming?: { min?: number; max?: number };
  outgoing?: { min?: number; max?: number };
  fields?: readonly string[];
}): Predicate => {
  const args: Record<string, number> = {};
  if (bounds.incoming?.min !== undefined) args.incoming_min = bounds.incoming.min;
  if (bounds.incoming?.max !== undefined) args.incoming_max = bounds.incoming.max;
  if (bounds.outgoing?.min !== undefined) args.outgoing_min = bounds.outgoing.min;
  if (bounds.outgoing?.max !== undefined) args.outgoing_max = bounds.outgoing.max;
  return bounds.fields === undefined
    ? { key: "degree", args }
    : { key: "degree", args, fields: bounds.fields };
};
/**
 * Every selected member lies in the forward closure of `roots` over the `via` field
 * set — `degree`'s global counterpart. `degree` asks the local question (how many arcs
 * sit at this member), so the first orphan of a dead chain fires and none behind it;
 * this asks the global one, so every member the roots cannot reach is equally a
 * finding.
 *
 * `roots` names a **requirement**, spelled exactly as `membership` names its target
 * set: rootness is a role, and a role is opt-in, so the roots are declared once and
 * never re-derived from a kind. A root holds trivially, and the walk visits each node
 * once, so a cycle inside the closure terminates and moves no verdict.
 *
 * `via` is the same by-incidence field set `degree`'s filter rides. Omit it and the
 * closure follows every edge at a member but containment; name a set and it follows
 * the union of those fields' arcs, so a mention or an import counts only where the set
 * names its field.
 *
 * Each-grain over the **selection** the clause binds to, and judged by the reference
 * graph rather than any member's own fields. It belongs in **no default contract**: a
 * clause demanding every member be reached is the declaration-density demand
 * `specs/intent.md`'s invariant 1 forbids unless the author declares it
 * (`specs/decisions/0056-reached-from-joins-the-vocabulary.md`).
 */
export const reachedFrom = (roots: string, via?: readonly string[]): Predicate =>
  via === undefined
    ? { key: "reached-from", target: roots }
    : { key: "reached-from", target: roots, fields: via };

/**
 * A guarded clause — a predicate (restricted to `type` or `enum`) that acts as a
 * guard, with a body of ordinary clauses that fire only where the guard holds.
 * The guard and body share the guard's address binding: the body's field paths
 * evaluate at the concrete element address the guard locates.
 *
 * A guard other than a `type` or `enumOf` predicate over the decidable field it
 * addresses is refused — not here, but by emit's admissibility check.
 *
 * Its options follow `clause()`: the author states the guarded clause's severity —
 * there is no default, since the tool never decides error-versus-warning — with
 * optional `guidance` and `cite`.
 */
export function when(
  guard: Predicate,
  body: readonly Clause[],
  opts: { severity: Severity; guidance?: string; cite?: string },
): Clause {
  return {
    predicate: { key: "when" },
    severity: opts.severity,
    guidance: opts.guidance,
    cite: opts.cite,
    when_guard: guard,
    when_body: body,
  };
}

/**
 * A clause — a predicate the author marks with a severity, the just-in-time
 * guidance the predicate cannot encode, and the external-fact `cite` that makes
 * a maintained floor auditable. A `when` clause carries an additional guard
 * predicate and a body of nested clauses that fire only where the guard holds.
 */
export interface Clause {
  readonly predicate: Predicate;
  readonly severity: Severity;
  readonly guidance?: string;
  readonly cite?: string;
  /** The guard predicate for a `when` clause; absent for all others. */
  readonly when_guard?: Predicate;
  /** The nested body clauses for a `when` clause; absent for all others. */
  readonly when_body?: readonly Clause[];
}

/** Compose a clause value — a predicate under a declared severity, with optional guidance/cite. */
export function clause(
  predicate: Predicate,
  opts: { severity: Severity; guidance?: string; cite?: string },
): Clause {
  return { predicate, severity: opts.severity, guidance: opts.guidance, cite: opts.cite };
}

/**
 * A requirement's **typed verifier** — the declared delegate that judges the
 * behavioral remainder, a species-tagged union the gate resolves at admissibility
 * and never runs. Three species: a `script` (path-resolved), a `telemetry`
 * declaration (named documented harness events), and a `review` (the reader whose
 * judgment holds it, named in prose). A probe stays a
 * documented pattern until a consumer types it — no `probe()` constructor is minted.
 */
export type Verifier =
  | {
      readonly species: "script";
      readonly path: string;
      /** The tool that runs the script — absent where the program names none ({@link script}). */
      readonly tool?: Member;
    }
  | {
      readonly species: "telemetry";
      readonly events: readonly string[];
      /** The tool the declaration's tap hooks run — absent for the temper package's bin
       * ({@link telemetry}). */
      readonly tool?: Member;
    }
  | { readonly species: "review"; readonly prose: string };

/** What a {@link telemetry} declaration states beyond its events. */
export interface TelemetryInit {
  /**
   * The `tool(...)` member its tap hooks run as temper — the same member each
   * `gateHook(...)`'s `tool` reaches, where an author runs every hook on one tool. Omit it
   * for the temper package's bin.
   */
  readonly tool?: Member;
  readonly binary?: Remedy<"`binary` gave way to `tool`: write `telemetry(events, { tool: tool({ package, bin }) })`, or omit it for the temper package's bin">;
}

/** What a {@link script} verifier states beyond its path. */
export interface ScriptInit {
  /**
   * The `tool(...)` member that runs the script — the same member a `{ file }` tool
   * homes the command at. Naming it makes the requirement the tool's runner, so the
   * tool's `degree` counts it. Omit it where no composed tool runs the script.
   */
  readonly tool?: Member;
}

/**
 * A script verifier — a path-resolved reference to the test or CI job that executes
 * the behavioral judgment. The gate checks the `path` resolves, never runs it. The
 * verifier may name the tool that runs it (`init.tool`); that reference is a `runs`
 * edge from the requirement.
 *
 * # Throws
 * If `init.tool` is not a `tool(...)` member.
 */
export function script(path: string, init: ScriptInit = {}): Verifier {
  if (init.tool !== undefined && init.tool.kind !== "tool") {
    throw new Error(
      "script: `tool` names what runs the script — pass a `tool(...)` member, or omit it.",
    );
  }
  return init.tool === undefined
    ? { species: "script", path }
    : { species: "script", path, tool: init.tool };
}

/**
 * A telemetry verifier — the named documented harness events the emitted tap records
 * to a local-locus log, judged by reading the field record. Each name must be a
 * documented harness lifecycle event — the gate checks each resolves, never records
 * into it. The vocabulary is the engine's own `TAP_HOOK_BINDINGS` table
 * (`src/builtin_kind.rs`), machine-written across the `generated/` seam and read by
 * `builtins.ts`'s `TELEMETRY_EVENT_HOOKS`. The declaration also names the tool its taps
 * run (`init.tool`, the temper package's bin unless declared); the engine renders each tap
 * command from it, anchored at the project.
 *
 * # Throws
 * If `init.tool` is not a `tool(...)` member.
 */
export function telemetry(events: readonly string[], init: TelemetryInit = {}): Verifier {
  if (init.tool !== undefined && init.tool.kind !== "tool") {
    throw new Error(
      "telemetry: `tool` names what the tap hooks run — pass a `tool(...)` member, or omit it " +
        "for the temper package's bin.",
    );
  }
  return init.tool === undefined
    ? { species: "telemetry", events }
    : { species: "telemetry", events, tool: init.tool };
}

/**
 * A review verifier — the reader whose judgment holds the requirement, named in prose:
 * the author as it writes, a reviewer at the diff. The gate carries the prose verbatim
 * and never judges it.
 *
 * # Throws
 * If `prose` is blank — a review naming no reader holds nothing.
 */
export function review(prose: string): Verifier {
  if (prose.trim() === "") {
    throw new Error(
      "review: `prose` is blank — name the reader whose judgment holds the requirement " +
        "(the author as it writes, a reviewer at the diff).",
    );
  }
  return { species: "review", prose };
}

/**
 * A requirement — a named obligation on the harness. `prose` is the authored
 * intent, carried never interpreted; `kind` constrains what may fill it —
 * the kind's `KindDefinition`, since a kind's identity travels by import, never by
 * string, and the slot carries only that identity for coverage resolution, never its
 * field type: a kind whose fields carry required members (skill, hook) assigns here,
 * where `KindDefinition<never>` would have rejected it. `required` is the
 * posture declaration; `clauses` bind to the requirement's **opt-in selection** —
 * ordinary [`Clause`] values, the same four-channel clause as everywhere, judged
 * over that selection by the same algebra a kind's own clauses are judged over its
 * population; `verifier` wires the behavioral remainder as a typed species union.
 */
export interface Requirement {
  readonly prose: string;
  readonly kind?: KindDefinition<any>;
  /**
   * Whether the requirement must be filled. Unset inherits the corpus's shipped
   * default — at least one satisfier, at error severity — so an unfilled
   * requirement fails `check`; `false` is the advisory override.
   */
  readonly required?: boolean;
  readonly clauses?: readonly Clause[];
  readonly verifier?: Verifier;
}

/**
 * What a requirement map (`harness({ require })`, a member's `requires`) holds under each
 * name: a {@link Requirement} literal, or a member of the shipped `requirement` kind or a
 * subkind of it (`requirement(…)`, `builtins.ts`), whose identity the map's key supplies.
 */
export type RequirementValue = Requirement | Member;
