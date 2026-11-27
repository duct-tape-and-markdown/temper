/**
 * Declaration rows — the composed program's erased declarations.
 * Every type erases at the seam: kinds, clauses, requirements, and
 * assembly facts compile to plain rows the engine reads. The **row shapes are the
 * generated `ts-rs` bindings** (`./generated/`, derived from `src/drift.rs`), so a
 * Rust-side row rename is a compile error here, never a silent shape drift. This
 * module authors the builders that fill them; the same rows ride the internal
 * versioned JSON pipe ({@link encodeSeam}) — not a designed IR, versioned
 * in lockstep.
 */

import { fileURLToPath } from "node:url";
import { isDeepStrictEqual } from "node:util";

import type { Harness } from "./assembly.js";
import type { EdgeField, EmbeddedMemberValue, KindFacts, Layout, Member, Registration } from "./kind.js";
import {
  absenceGatesOf,
  baseOf,
  collectionAddressOf,
  edgeTargetNames,
  locusPattern,
  manifestOf,
  registrationOf,
  templateMatches,
  templateReachesBeneath,
  templatesOf,
} from "./kind.js";
import type { Clause, Predicate, Requirement, Verifier } from "./contract.js";
import type { Include, MentionScope } from "./prose.js";
import { isTextSpan, resolveEmbeddedLeaves, span } from "./prose.js";
import * as shippedBuiltins from "./builtins.js";
import { dial } from "./dial.js";
import { COUNSEL_RULE_NAME, GUIDANCE_RULE_PREFIX, SDK_COUNSEL } from "./counsel.js";
import {
  GATE_TOOL_KEY,
  HOOK_HANDLER_KEY,
  HOOK_MATCHER_KEY,
  ROOT_HOSTED_KINDS,
  RUNS_FIELD,
  SETTINGS_MANIFEST,
  TELEMETRY_EVENT_HOOKS,
  gateHookTool,
  hook,
  hookCollectionKey,
  requirement,
  requirementOf,
  requirementSource,
  rule,
  settings,
  tapHookRegistration,
  tool,
} from "./builtins.js";
import { ROOT_ADDRESS, leafAddress, memberAddress, nestedAddress } from "./member-address.js";
import { playbookArea } from "./playbook.js";

import type {
  AreaRow,
  AssemblyFactRow,
  ClauseRow,
  CollectionAddressRow,
  Declarations,
  IncludeRow,
  InputRow,
  KindFactRow,
  LayoutRegionRow,
  LayoutRow,
  MentionRow,
  NestedMemberRow,
  Payload,
  RegistrationRow,
  RequirementRow,
  RunsRow,
  SatisfiesRow,
  SettingsRow,
  TemplateRow,
  TypecheckRow,
  Verifier as VerifierRow,
} from "./generated/index.js";

// The row shapes the authoring API surfaces re-export from here, so `index.ts`'s
// public face is unchanged by the move to generated bindings.
export type {
  AssemblyFactRow,
  ClauseRow,
  Declarations,
  KindFactRow,
  RequirementRow,
  SatisfiesRow,
} from "./generated/index.js";

/**
 * Compile one `Clause` into its lock row: the shared `key`/`field`/`severity`/
 * `guidance`/`cite` columns — the clause's four channels surviving erasure
 * — plus, when the
 * predicate carries them, the `count`/`target`/`degree` argument columns a
 * requirement's own set-/edge-scope demand needs, and the
 * `bound`/`charset`/`keys`/`values` argument columns a kind's own node-scope
 * floor clause needs (`min-len`/`max-len`/`extent`'s bound, `extent`'s unit,
 * `allowed-chars`'s charset, `forbidden-keys`'s keys, `deny`'s values) — so
 * the lock encodes the floor losslessly, not identity+severity alone. `kind`
 * is supplied only for a kind's own `expect` clause; a requirement's nested
 * clause carries none.
 */
function clauseRow(clause: Clause, kind?: string): ClauseRow {
  const { predicate } = clause;

  if (predicate.key === "when") {
    const guardPredicate = clause.when_guard;
    if (!guardPredicate) {
      throw new Error(
        `a \`when\` clause carries no guard predicate — a guarded clause is one guard plus ` +
          `the body it gates (specs/model/contract.md, "clause").`,
      );
    }

    // The guard's argument columns ride through *whole*: a `when` row is the guard's own
    // row with the clause's seven columns set over it, so a column a newly-guardable
    // predicate adds reaches a guard's row by construction rather than by maintaining a
    // list here. `src/contract.rs` `predicate_from_row` is the total counterpart on the
    // read side, rebuilding the guard row by the same spread.
    const {
      kind: _kind,
      predicate: guardKey,
      severity: _severity,
      guidance: _guidance,
      cite: _cite,
      guard_predicate: _guardPredicate,
      body: _body,
      ...guardColumns
    } = clauseRow({ predicate: guardPredicate, severity: clause.severity }, undefined);

    return {
      ...guardColumns,
      kind,
      predicate: "when",
      severity: clause.severity,
      guidance: clause.guidance,
      cite: clause.cite,
      guard_predicate: guardKey,
      body: clause.when_body ? clause.when_body.map((c) => clauseRow(c, undefined)) : undefined,
    };
  }

  return {
    kind,
    predicate: predicate.key,
    field: clauseField(predicate),
    severity: clause.severity,
    guidance: clause.guidance,
    cite: clause.cite,
    count:
      predicate.key === "count"
        ? { min: predicate.args?.min ?? 0, max: predicate.args?.max ?? Number.MAX_SAFE_INTEGER }
        : undefined,
    // The requirement-name column `membership`'s allowed-value target and
    // `reached-from`'s closure roots share — one naming scheme for "the requirement
    // whose satisfiers this clause reads" (`src/contract.rs` `predicate_from_row`).
    target:
      predicate.key === "membership" || predicate.key === "reached-from"
        ? predicate.target
        : undefined,
    degree:
      predicate.key === "degree"
        ? {
            incoming: edgeBoundArgs(predicate.args, "incoming"),
            outgoing: edgeBoundArgs(predicate.args, "outgoing"),
 }
        : undefined,
    // The by-incidence field set rides its own shared column, not the direction-only
    // `degree` bound — the slot `reached-from`'s via set joins it. Copied into a
    // fresh array: the predicate's set is read-only, the row's column is not.
    fields:
      (predicate.key === "degree" ||
        predicate.key === "reached-from" ||
        predicate.key === "listing") &&
      predicate.fields
        ? [...predicate.fields]
        : undefined,
    gate:
      predicate.key === "mention-reachable" ||
      predicate.key === "trigger-agrees" ||
      predicate.key === "listing"
        ? predicate.gate
        : undefined,
    value_type:
      predicate.key === "type" && predicate.value_type ? [...predicate.value_type] : undefined,
    shape: predicate.key === "shape" ? predicate.shape : undefined,
    bound: nodeScopeBoundArgs(predicate),
    unit: predicate.key === "extent" ? predicate.unit : undefined,
    // The generated rows carry mutable columns; the predicate's `value_type`/
    // `charset`/`keys`/`values` are read-only, so copy each into a fresh
    // array/object — the same bytes, a shape the row will accept.
    charset:
      predicate.key === "allowed-chars" && predicate.charset !== undefined
        ? {
            ...predicate.charset,
            ranges: predicate.charset.ranges ? [...predicate.charset.ranges] : undefined,
          }
        : undefined,
    keys: predicate.key === "forbidden-keys" && predicate.keys ? [...predicate.keys] : undefined,
    values:
      (predicate.key === "deny" ||
        predicate.key === "enum" ||
        predicate.key === "listing" ||
        predicate.key === "loads") &&
      predicate.values
        ? [...predicate.values]
        : undefined,
    range:
      predicate.key === "range" && predicate.range !== undefined
        ? { min: predicate.range.min, max: predicate.range.max }
        : undefined,
    section:
      predicate.key === "section-contains" && predicate.section !== undefined
        ? { heading: predicate.section.heading, marker: predicate.section.marker }
        : undefined,
    sections:
      predicate.key === "require-sections" && predicate.sections
        ? [...predicate.sections]
        : undefined,
  };
}

/**
 * The `field` column for one predicate: the field it names, or — for the four
 * predicates that name a *section*, a field *set*, or a *requirement* rather than a
 * field — an identity synthesized from the arguments the row already carries.
 *
 * `section-contains`, `require-sections`, `degree` and `reached-from` set no `field`,
 * and the column is what emit stamps a clause's label from (`stamp_clause_label`,
 * `src/drift.rs`), so reading `Predicate.field` folds every clause of one of those
 * predicates on one kind into one label — two rows wearing one label, which
 * admissibility refuses as a malformed lock. Synthesizing here keeps the whole fix at
 * the lowering: the Rust reader reconstructs all four predicates from the
 * `section`/`sections`/`fields`/`target` columns and never from this one, so nothing
 * round-trips through the synthesized text.
 */
export function clauseField(predicate: Predicate): string | undefined {
  if (predicate.key === "section-contains") {
    const { section } = predicate;
    return section === undefined ? undefined : `${section.heading}.${section.marker}`;
  }
  if (predicate.key === "require-sections") {
    // Joined with `+` rather than the label's own `.`, so the segment reads as the
    // set it is and two different heading lists cannot fold to one label.
    return predicate.sections?.join("+");
  }
  if (predicate.key === "degree") {
    // A `degree` clause names no field of its own: its by-incidence filter is what
    // distinguishes two bounds on one kind, so the filter is the segment — sorted and
    // `+`-joined by the rule directly above, so two field lists spelling one set land
    // one address and two different sets never fold to one. An unfiltered bound adds
    // no segment: it ranges over every edge at the member, so two of them on one kind
    // are a redundancy the author collapses, never a distinction an address must hold.
    return predicate.fields === undefined ? undefined : [...predicate.fields].sort().join("+");
  }
  if (predicate.key === "listing") {
    // A `listing` names no field of its own: the fields it lists are its identity, kept
    // in listed order since the entry joins them in that order.
    return predicate.fields?.join("+");
  }
  if (predicate.key === "reached-from") {
    // A `reached-from` clause names no field either: its identity is the pair it walks.
    // The roots requirement leads, and a declared via set follows after a `.` — the
    // two-argument `section-contains` precedent — sorted and `+`-joined by `degree`'s
    // rule directly above. So two closures rooted at one requirement over different
    // arcs land two addresses instead of folding into one, and two spellings of one via
    // set land one. An unfiltered closure adds no second segment: the roots are its
    // whole identity, and two of them on one kind are a redundancy the author
    // collapses.
    const { target } = predicate;
    if (target === undefined) return undefined;
    return predicate.fields === undefined
      ? target
      : `${target}.${[...predicate.fields].sort().join("+")}`;
  }
  return predicate.field;
}

/** `min-len`/`max-len`/`extent`/`listing`'s scalar bound off their shared `min`/`max`
 * args keys, plus `listing`'s per-entry `entry` cap — `undefined` for every other
 * predicate, and for these four when no endpoint is present. */
function nodeScopeBoundArgs(
  predicate: Predicate,
): { readonly min?: number; readonly max?: number; readonly entry?: number } | undefined {
  if (
    predicate.key !== "min-len" &&
    predicate.key !== "max-len" &&
    predicate.key !== "extent" &&
    predicate.key !== "listing"
  ) {
    return undefined;
  }
  const min = predicate.args?.min;
  const max = predicate.args?.max;
  const entry = predicate.args?.entry;
  if (min === undefined && max === undefined && entry === undefined) return undefined;
  // `entry` is `listing`'s alone, so the other three rows keep their two-key bound.
  return entry === undefined ? { min, max } : { min, max, entry };
}

/** One `degree` direction's `{min, max}` off its flat `<dir>_min`/`<dir>_max` args
 * keys — `undefined` when neither is present (that direction is unconstrained). */
function edgeBoundArgs(
  args: Readonly<Record<string, number>> | undefined,
  direction: "incoming" | "outgoing",
): { readonly min?: number; readonly max?: number } | undefined {
  const min = args?.[`${direction}_min`];
  const max = args?.[`${direction}_max`];
  return min === undefined && max === undefined ? undefined : { min, max };
}

/**
 * The lock label for a kind's declared unit shape: `file`/`directory`/`starred-segment`/`path`
 * verbatim, or `named-field(<identityField>)` / `named-field-or-file(<identityField>)` for
 * the field-sourced modes — the same
 * `<name>(<field>)` call syntax [`registrationLabel`] uses, so the id source round-trips
 * through the row rather than degrading to a bare, unreconstructable `"named-field"`.
 */
function unitShapeLabel(facts: KindFacts): string | undefined {
  if (facts.unitShape !== "named-field" && facts.unitShape !== "named-field-or-file") {
    return facts.unitShape;
  }
  return `${facts.unitShape}(${facts.identityField})`;
}

/** The lock label for one declared registration channel. */
function registrationLabel(registration: Registration): string {
  switch (registration.via) {
    case "always":
      return "always";
    case "user-invoked":
      return "user-invoked";
    case "description-trigger":
      return `description-trigger(${registration.field})`;
    case "paths-match":
      return `paths-match(${registration.field})`;
    case "on-read-beneath":
      return "on-read-beneath";
    case "paths-listing":
      return `paths-listing(${registration.field})`;
    case "event":
      return `event(${registration.field})`;
    case "connection":
      return "connection";
    case "enablement":
      return `enablement(${registration.field})`;
    case "registry":
      return "registry";
    case "selected":
      return `selected(${registration.field})`;
 }
}

/** The lock labels for a kind's declared registration **set**, in declaration order —
 * `undefined` for an empty set, the same omit-the-column tolerance `templatesFor` takes. */
function registrationLabels(registration: readonly Registration[]): string[] | undefined {
  return registration.length > 0 ? registration.map(registrationLabel) : undefined;
}

/** The stable-sort ordering every declaration row family shares. */
export function compareStrings(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0;
}

/**
 * Build a `Map<K, V>` from an iterable of key-value pairs, refusing on duplicate
 * identity keys. A map the caller builds must have unique keys; a duplicate is
 * corruption, not a shadowing rule.
 *
 * @throws Error when a key appears more than once in the iterable, naming the
 * colliding key.
 */
export function uniqueMap<K, V>(entries: Iterable<[K, V]>): Map<K, V> {
  const map = new Map<K, V>();
  for (const [key, value] of entries) {
    if (map.has(key)) {
      throw new Error(`duplicate identity key \`${key}\``);
    }
    map.set(key, value);
  }
  return map;
}

/**
 * A host kind's nesting templates, from its two declaration loci. The kind's own
 * declared templates carry each layer — child kind, plus a file layer's path pattern.
 * An adopting corpus that admits its own embedded kinds over the host overrides the
 * *embedded* grain only: an admission names a host and a child kind but no path, so it
 * can only speak for the pathless (embedded) layer. The path-carrying file layers the
 * host declares are the host's own facts and stand, joined with the admitted rows —
 * else composing a body over a host wipes its declared file layer and the engine, finding
 * no file template, refuses the host's nested file children.
 *
 * `undefined` when nothing is declared or admitted, so the row omits the column rather
 * than carrying an empty array.
 */
function templatesFor(facts: KindFacts, admissions: AdmissionsByHost): TemplateRow[] | undefined {
  const declared = (facts.templates ?? []).map((template): TemplateRow => ({
    kind: template.kind.key,
    path: template.path,
  }));
  const admitted = [...(admissions.get(facts.name) ?? [])].sort(compareStrings);
  if (admitted.length === 0) return declared.length > 0 ? declared : undefined;
  const fileLayers = declared.filter((template) => template.path !== undefined);
  return [...fileLayers, ...admitted.map((kind): TemplateRow => ({ kind }))];
}

/**
 * Lower a kind's declared {@link Layout} into its `content` row — one flat
 * discriminator-plus-columns [`LayoutRegionRow`] per region, `memberKind` spelled as the
 * wire's snake_case `member_kind`. `undefined` for a `file`-content kind (no declared
 * content), so its row omits the column and stays byte-identical.
 */
function contentRow(content: Layout | undefined): LayoutRow | undefined {
  if (content === undefined) return undefined;
  const regions = content.regions.map((region): LayoutRegionRow => {
    switch (region.region) {
      case "prose":
        return { region: "prose", import: region.import };
      case "field":
        return { region: "field", slot: region.slot };
      case "collection":
        return { region: "collection", member_kind: region.memberKind, key: region.key };
    }
  });
  return { regions };
}

/**
 * Lower a kind's declared {@link CollectionAddress} into its `collection_address` row —
 * `keyPath` spelled as the wire's snake_case `key_path`, and `entryShape` passed through
 * as a string wire format (`object`, `scalar(field)`, or `group-array(member_key;lifted_fields)`).
 * `undefined` for a file-locus kind, so its row omits the column and stays byte-identical.
 */
function collectionAddressRow(facts: KindFacts): CollectionAddressRow | undefined {
  const address = collectionAddressOf(facts);
  if (address === undefined) return undefined;
  return { key_path: address.keyPath, entry_shape: address.entryShape };
}

/**
 * Lower a kind's exhaustive leaf-set witness into the row's `leaves` column: the
 * witness's own keys, in the order its author declared them (`kind.ts`'s `LeafSet`,
 * decision 0053 — the type is the declaration, and TypeScript erases it, so the record is
 * how the corpus hands the key set over). `undefined` for a kind declaring no witness, so
 * its row omits the column.
 */
function leafSetRow(leaves: KindFacts["leaves"]): string[] | undefined {
  return leaves === undefined ? undefined : Object.keys(leaves);
}

/**
 * One kind's fact row — an `at` locus supplies `locus_root`/`locus_glob` and any
 * other locus neither (a nested-file kind's path composes from its host's unit and the
 * host template's pattern; an embedded kind owns no unit at all — both govern no glob).
 * A file locus's `commitment` class and `exclude` paths ride the same spelling, absent for
 * the committed, nothing-excluded default. `templates` names the embedded kinds the corpus admits over it, and `content`
 * lowers a declared layout (absent for a `file`-content kind). A registration kind
 * extends the row with its `shape` marker and `collection_address`. A declared leaf-set
 * witness lowers to `leaves` ([`leafSetRow`]), absent for a kind declaring none. Advisory
 * `guidance`/`cite` pair rides alongside, locus-optional so an embedded kind's own
 * counsel reaches the lock the same way a nested-file kind's already does (decision
 * 0045) — callable for any locus; [`kindFactKindsInPlay`] decides which embedded kinds
 * actually take a row.
 */
function kindFactRow(facts: KindFacts, admissions: AdmissionsByHost): KindFactRow {
  const at = facts.locus.kind === "at" ? facts.locus : undefined;
  return {
    name: facts.name,
    provider: facts.provider,
    base: facts.base,
    locus_root: at?.root,
    // Always the list: the lock column is one or more templates.
    locus_glob: at === undefined ? undefined : [...templatesOf(at)],
    commitment: at?.commitment,
    // Spread rather than an `undefined` column: a row excluding nothing carries no key.
    ...(at?.exclude?.length ? { exclude: [...at.exclude] } : {}),
    ...(at !== undefined && absenceGatesOf(at).length
      ? { absence_gate: absenceGatesOf(at).map((gate) => ({ template: gate.glob, absent: [...gate.absent] })) }
      : {}),
    ...("leniency" in facts && facts.leniency?.length ? { leniency: [...facts.leniency] } : {}),
    format: facts.format,
    unit_shape: unitShapeLabel(facts),
    // An embedded or nested-file kind declares no set — its locus already states it empty.
    registration: registrationLabels(registrationOf(facts)),
    templates: templatesFor(facts, admissions),
    content: contentRow(facts.content),
    shape: facts.shape,
    leaves: leafSetRow(facts.leaves),
    collection_address: collectionAddressRow(facts),
    guidance: facts.guidance,
    cite: facts.cite,
  };
}

/**
 * The locus a colliding kind is declared at, for {@link resolveNameCollision}'s refusal:
 * an `at` kind names the path its members are found at, and every other locus names
 * itself — a kind that governs no glob has no path of its own to name.
 */
function locusLabel(facts: KindFacts): string {
  const { locus } = facts;
  if (locus.kind !== "at") return `the \`${locus.kind}\` locus`;
  return templatesOf(locus)
    .map((glob) => `\`${locus.root === "." ? glob : `${locus.root}/${glob}`}\``)
    .join(" and ");
}

/**
 * A kind's facts with the faces a relocation may diverge from its base on erased: the
 * `relocates` marker, the appended `edgeFields`, an `at` locus's moved root and glob and
 * its added exclusions, and the replaced counsel (`kind.ts`'s `KindRelocation`). Two values agreeing here are one kind up to a
 * relocation delta; the delta's own faces are checked on their own terms.
 */
function relocationInvariants(facts: KindFacts): Record<string, unknown> {
  const bare: Record<string, unknown> = { ...facts };
  delete bare.relocates;
  delete bare.edgeFields;
  delete bare.guidance;
  delete bare.fieldGuidance;
  delete bare.cite;
  delete bare.mode;
  if (facts.locus.kind === "at") {
    const { exclude: _added, ...locus } = facts.locus;
    bare.locus = { ...locus, root: "", glob: "" };
  }
  return bare;
}

/**
 * Whether `relocation` was derived from `base` — the provenance the relocation-wins rule
 * rests on. The marker names the base, `relocate` only ever *appends* edge fields (so the
 * base's are a prefix of the relocation's), and every other fact but the counsel a
 * relocation replaces by design rides through unchanged:
 * the base's facts are a subset by construction, which is exactly what makes keeping the
 * relocation lossless. Checked rather than assumed, because the marker carries a name and
 * a name is what is in dispute — a value bearing it whose facts are not the other's
 * superset is a third kind of the same name, and dropping *that* would be the very silent
 * loss this decision exists to end.
 */
function isRelocationOf(relocation: KindFacts, base: KindFacts): boolean {
  if (relocation.relocates !== base.name || base.relocates !== undefined) return false;
  const inherited = base.edgeFields ?? [];
  const declared = relocation.edgeFields ?? [];
  return (
    declared.length >= inherited.length &&
    isDeepStrictEqual(declared.slice(0, inherited.length), inherited) &&
    keepsExclusionsOf(relocation, base) &&
    isDeepStrictEqual(relocationInvariants(relocation), relocationInvariants(base))
  );
}

/**
 * Whether `relocation` still excludes every path of `base`'s its own templates reach — the
 * half of the erased `exclude` face a relocation may not change. `relocate` only *appends*
 * exclusions, and a moved locus keeps each base exclusion its new template still matches
 * or reaches beneath while dropping the rest, which would subtract nothing
 * ({@link relocatedLocus}). So a base exclusion the relocation's templates no longer reach
 * owes nothing, while a value of the name dropping one they still reach claims paths the
 * base never governed, and is a third kind, not a relocation.
 */
function keepsExclusionsOf(relocation: KindFacts, base: KindFacts): boolean {
  if (base.locus.kind !== "at") return true;
  const excluded = base.locus.exclude ?? [];
  if (relocation.locus.kind !== "at") return excluded.length === 0;
  const { root } = relocation.locus;
  const kept = relocation.locus.exclude ?? [];
  const patterns = templatesOf(relocation.locus).map((glob) => locusPattern(root, glob));
  const reached = (path: string) =>
    patterns.some((pattern) => templateMatches(pattern, path) || templateReachesBeneath(pattern, path));
  return excluded.every((path) => kept.includes(path) || !reached(path));
}

/**
 * Which of two `KindFacts` values sharing one name is the kind in play — a decision that
 * must not depend on the order the two arrived in, since {@link kindsInPlay} draws its
 * facts from `members`, then `expect`, then `admit`, then embedded templates, and a kind
 * legitimately reaches it through any of them.
 *
 * A **relocation** wins over its base whichever way round they arrive
 * ({@link isRelocationOf} proves the provenance): the base's facts are the relocation's
 * minus the delta, so keeping the base would silently drop the relocation's added edge
 * fields and moved locus. Two values that are structurally equal are one kind declared
 * twice — keep the first. Anything else is a genuine collision and refuses: kind identity
 * travels by import, never by string (`representation.md`, "kind"), while the lock is a
 * string-keyed medium, so the second kind has no name to reach the engine
 * under and dropping it is a silent loss of everything it declared. Two relocations of
 * one base collide the same way — each is still a kind of the base's own name.
 *
 * # Throws
 * On a collision, naming the kind and both loci — the two declarations an author has to
 * go look at.
 */
function resolveNameCollision(held: KindFacts, arriving: KindFacts): KindFacts {
  // At most one holds: each direction demands the other value carry no marker at all.
  if (isRelocationOf(held, arriving)) return held;
  if (isRelocationOf(arriving, held)) return arriving;
  if (isDeepStrictEqual(held, arriving)) return held;
  throw new Error(
    held.relocates !== undefined && arriving.relocates !== undefined
      ? `two relocations of kind \`${held.name}\` are in play, at ${locusLabel(held)} and ` +
        `${locusLabel(arriving)}, and they diverge. A relocation is still a kind of its base's ` +
        `name, so a second one collides with the first: relocate the base once and import that ` +
        `one value wherever it is used (specs/model/representation.md, "kind").`
      : `two kinds named \`${held.name}\` are in play, at ${locusLabel(held)} and ` +
        `${locusLabel(arriving)}. Kind identity travels by import, never by string, and the lock ` +
        `is keyed by name — the second kind has no name of its own to reach the engine under. ` +
        `Import the declared kind rather than redeclaring it; to move a built-in to another root ` +
        `or add an edge field to it, use \`relocate()\` (specs/model/representation.md, "kind").`,
  );
}

/**
 * Every kind in play, at any locus — member kinds ∪ expect kinds ∪ their embedded
 * children — name-sorted, so every family derived from it inherits one stable order.
 *
 * The embedded children close transitively over both channels a host names one through:
 * `admit` (the adopting corpus's declaration) and a path-less `templates` entry (the
 * embedded layer). An embedded kind reaches the lock through its host's `templates`
 * column rather than a row of its own, so those channels are the only way one is in play
 * at all — and its declared edge fields are still owed their assembly facts.
 *
 * Beside them, a template's **subkind** child is drawn in: an extended host names it in its
 * base's place, so the host's own row is what puts it in play. Any other path-carrying
 * template is the nested-file layer, whose child owns a unit and reaches the lock through
 * `expect` like any other unit kind; pulling one in here would forge it a kind-fact row
 * it never declared.
 *
 * A synthesized tap hook ({@link tapHookRows}) puts the `hook` kind in play as an authored
 * hook member would: its row lands in `registrations` under `hook`, so the kind is owed
 * the fact row the engine resolves that row's kind against.
 *
 * Two facts values arriving under one name are decided by {@link resolveNameCollision},
 * never by arrival order: the deduped facts are what `assemblyFactRows` derives every
 * `edge` row from, so first-wins would drop a relocation's added edge on the floor
 * whenever the base happened to be named first.
 */
export function kindsInPlay(harness: Harness): KindFacts[] {
  const byName = new Map<string, KindFacts>();
  const pending: KindFacts[] = [];
  const admit = (facts: KindFacts): void => {
    const held = byName.get(facts.name);
    if (held === facts) return;
    const winner = held === undefined ? facts : resolveNameCollision(held, facts);
    if (winner === held) return;
    byName.set(facts.name, winner);
    pending.push(winner);
  };
  for (const member of harness.members) admit(member.facts);
  for (const binding of harness.expect) admit(binding.kind.facts);
  for (const { admits } of harness.admit) for (const child of admits) admit(child.facts);
  // A synthesized tap is a hook registration row, so it brings the hook kind with it:
  // a program whose only hook content is a tap still owes `hook` its kind-fact row.
  if (tapHookRows(harness).length > 0) admit(hook.facts);
  // A requirement subkind's members are the root's nested members, never roster members,
  // so the requirement map is what puts the subkind — and each subkind up its chain to the
  // shipped `requirement` — in play: its row's `base` is how the engine binds its members
  // where a selection of `requirement` reaches.
  for (const [, declared] of requirementEntries(harness)) {
    const source = requirementSource(declared)?.facts;
    for (let facts = source; facts !== undefined && facts.base !== undefined; facts = baseOf(facts)) admit(facts);
  }
  for (let facts = pending.pop(); facts !== undefined; facts = pending.pop()) {
    for (const template of facts.templates ?? []) {
      const child = template.kind.facts;
      if (child.locus.kind === "embedded" || child.base !== undefined) admit(child);
    }
  }
  return [...byName.values()].sort((a, b) => compareStrings(a.name, b.name));
}

/** The distinct discoverable (`at`) kinds in play. */
function atLocusKindsInPlay(allKinds: readonly KindFacts[]): KindFacts[] {
  return allKinds.filter((facts) => facts.locus.kind === "at");
}

/**
 * The distinct kinds in play that take a kind-fact row: every non-embedded locus
 * unconditionally (a nested-file kind owns a file the engine must place, and places it
 * off its row, though it governs no glob to be discovered at), plus an embedded kind that
 * has something of its own for a row to carry: `guidance` or `cite` (decision 0045), or a
 * declared leaf set (decision 0053), or a `base` — a subkind the engine binds off its row. The leaf set is on this list for the reason the
 * column exists — it teaches what a child carries *before* the surface holds a member of
 * it, and an embedded kind taking no row never reaches the lock to teach it. An embedded
 * kind declaring none of the three has nothing for the row to carry, and its members
 * already reach the corpus through their host's `templates` column alone (`kindsInPlay`),
 * never the row.
 */
function kindFactKindsInPlay(allKinds: readonly KindFacts[]): KindFacts[] {
  return allKinds.filter(
    (facts) =>
      facts.locus.kind !== "embedded" ||
      facts.base !== undefined ||
      facts.guidance !== undefined ||
      facts.cite !== undefined ||
      facts.leaves !== undefined,
  );
}

/**
 * Every kind temper ships, by facts identity — what a program reaches by import alone.
 * Read off the provider module's exports rather than listed, so a built-in added there
 * can never project its shipped counsel by being forgotten here.
 */
const SHIPPED_KINDS: ReadonlySet<KindFacts> = new Set(
  [...Object.values(shippedBuiltins), dial].flatMap((value: unknown) =>
    typeof value === "function" && "facts" in value ? [(value as { readonly facts: KindFacts }).facts] : [],
  ),
);

const COUNSEL_RULE_PATHS = ".temper/**";

/** The synthesized guidance rules, so a harness already carrying them is not re-synthesized. */
const SYNTHESIZED_RULES = new WeakSet<Member>();

/**
 * The repository-relative file globs a kind's members occupy — the `paths` a rule scoped
 * to them lists. An `at` kind is its own root and glob (an `exclude` is not expressible
 * in `paths`, so the scope over-reaches it, which advisory counsel tolerates); a nested
 * file kind sits at each templating host's unit joined with the template's pattern; an
 * embedded kind owns no file and takes the loci of every host that templates or admits
 * it, so its counsel is stated once over all of them rather than per host.
 */
function locusGlobs(
  facts: KindFacts,
  kinds: readonly KindFacts[],
  admissions: AdmissionsByHost,
  seen: ReadonlySet<string> = new Set(),
): string[] {
  const locus = facts.locus;
  if (locus.kind === "at") {
    const root = locus.root;
    return templatesOf(locus).map((glob) => [root, glob].filter((part) => part !== "" && part !== ".").join("/"));
  }
  if (seen.has(facts.name)) return [];
  const visiting = new Set([...seen, facts.name]);
  const globs = new Set<string>();
  for (const host of kinds) {
    const layers = (host.templates ?? []).filter((template) => template.kind.key === facts.name);
    if (locus.kind === "nested-file") {
      for (const layer of layers) {
        if (layer.path === undefined || host.unitShape !== "directory") continue;
        for (const hostGlob of locusGlobs(host, kinds, admissions, visiting)) {
          const slash = hostGlob.lastIndexOf("/");
          if (slash >= 0) globs.add(`${hostGlob.slice(0, slash)}/${layer.path}`);
        }
      }
    } else if (
      layers.some((layer) => layer.path === undefined) ||
      admissions.get(host.name)?.has(facts.name) === true
    ) {
      for (const hostGlob of locusGlobs(host, kinds, admissions, visiting)) globs.add(hostGlob);
    }
  }
  return [...globs].sort(compareStrings);
}

/** A list item whose text may span lines — continuation lines indent under the bullet. */
function bullet(head: string, text: string): string {
  return `- ${head}: ${text.trim().split("\n").join("\n  ")}`;
}

/** The kind names an edge field's `to` set lists, for a line that states where it points. */
export function targetList(to: readonly string[]): string {
  return to.map((target) => `\`${target}\``).join(", ");
}

/** A kind's counsel for one field, when it declares any that is not blank. */
function fieldCounsel(owner: KindFacts, field: string): string | undefined {
  const counsel = (owner.fieldGuidance as Record<string, string | undefined> | undefined)?.[field];
  return counsel === undefined || counsel.trim() === "" ? undefined : counsel;
}

/**
 * One declared edge and its intent — the counsel its source kind gives the edge field,
 * absent when the source declares none. `shipped` marks a source kind temper ships, whose
 * counsel a guidance rule never projects.
 */
export interface EdgeIntent {
  readonly from: string;
  readonly field: string;
  readonly to: readonly string[];
  readonly intent: string | undefined;
  readonly shipped: boolean;
}

/**
 * The edges at a kind's two ends, with their intent: `outbound` its own edge fields in
 * declared order, `inbound` every kind-in-play's edge field whose `to` set names it, in
 * `kinds` order. The one assembly of edge intent the guidance rules and the reference's
 * kind topics both state, so a relationship's intent reads the same at every surface.
 */
export function kindEdges(
  facts: KindFacts,
  kinds: readonly KindFacts[],
): { readonly outbound: readonly EdgeIntent[]; readonly inbound: readonly EdgeIntent[] } {
  const of = (source: KindFacts, edge: EdgeField): EdgeIntent => ({
    from: source.name,
    field: edge.field,
    to: edgeTargetNames(edge),
    intent: fieldCounsel(source, edge.field),
    shipped: SHIPPED_KINDS.has(source),
  });
  return {
    outbound: (facts.edgeFields ?? []).map((edge) => of(facts, edge)),
    inbound: kinds.flatMap((source) =>
      (source.edgeFields ?? []).filter((edge) => edgeTargetNames(edge).includes(facts.name)).map((edge) => of(source, edge)),
    ),
  };
}

/**
 * The repository-relative file globs a kind's members occupy over `harness` — what a
 * guidance rule's `paths` lists and a kind topic names as its locus.
 */
export function kindLocusGlobs(facts: KindFacts, harness: Harness): string[] {
  return locusGlobs(facts, kindsInPlay(harness), admissionsByHost(harness));
}

/**
 * A requirement as the guidance rules and the reference state it: its kind (when
 * declared), its prose, what holds its behavior (absent when unheld), its satisfiers.
 */
export interface ContractedRequirement {
  readonly name: string;
  readonly kind: string | undefined;
  readonly prose: string;
  readonly verifier: Verifier | undefined;
  readonly satisfiers: readonly { readonly kind: string; readonly address: string }[];
}

/**
 * Every declared requirement — assembly `require` ∪ each member's `requires`, the
 * namespace {@link requirementRows} lowers — with the members claiming to fill it, name
 * sorted, satisfiers in address order.
 */
export function contractedRequirements(harness: Harness): ContractedRequirement[] {
  const entries = requirementEntries(harness);
  const satisfiers = new Map<string, Array<{ kind: string; address: string }>>();
  for (const member of harness.members) {
    for (const name of member.satisfies) {
      const list = satisfiers.get(name) ?? [];
      list.push({ kind: member.kind, address: memberAddress(member) });
      satisfiers.set(name, list);
    }
  }
  return [...uniqueMap(entries).entries()]
    .sort(([a], [b]) => compareStrings(a, b))
    .map(([name, requirement]) => ({
      name,
      kind: requirement.kind?.key,
      prose: requirement.prose,
      verifier: requirement.verifier,
      satisfiers: (satisfiers.get(name) ?? []).sort((a, b) => compareStrings(a.address, b.address)),
    }));
}

/**
 * The body a kind's guidance rule carries — the kind's own counsel; its counselled
 * non-edge fields under `Fields`, in authored order; each counselled edge's intent
 * ({@link kindEdges}) under `Edges`, outbound ones (the kind's own, in declared order)
 * before inbound ones (another kind's field targeting this one), so an edge's intent
 * reads at both its ends; and under
 * `Requirements` every requirement this kind's members satisfy or that declares this
 * kind, with its prose and satisfiers — what is already contracted, before an agent adds
 * to the set. A shipped kind contributes none of its shipped counsel (`shipped`): its
 * rule carries only the program's contracts. A body with nothing in it is none, and
 * projects no rule.
 */
function guidanceBody(
  facts: KindFacts,
  kinds: readonly KindFacts[],
  requirements: readonly ContractedRequirement[],
): string | undefined {
  const shipped = SHIPPED_KINDS.has(facts);
  const edges = kindEdges(facts, kinds);
  const ownEdges = new Set(edges.outbound.map((edge) => edge.field));
  const own = shipped ? "" : (facts.guidance?.trim() ?? "");
  const fields: string[] = [];
  for (const field of shipped ? [] : Object.keys(facts.fieldGuidance ?? {})) {
    const counsel = fieldCounsel(facts, field);
    if (counsel !== undefined && !ownEdges.has(field)) fields.push(bullet(`\`${field}\``, counsel));
  }
  const projected = (edge: EdgeIntent): edge is EdgeIntent & { intent: string } =>
    !edge.shipped && edge.intent !== undefined;
  const outbound = edges.outbound
    .filter(projected)
    .map((edge) => bullet(`\`${edge.field}\` → ${targetList(edge.to)}`, edge.intent));
  const inbound = edges.inbound
    .filter(projected)
    .map((edge) => bullet(`\`${edge.from}\`.\`${edge.field}\` → here`, edge.intent));
  const contracted = requirements.flatMap((requirement) => {
    const mine = requirement.kind === facts.name || requirement.satisfiers.some((s) => s.kind === facts.name);
    if (!mine) return [];
    const declared = requirement.kind === undefined ? "" : ` (kind \`${requirement.kind}\`)`;
    const filled =
      requirement.satisfiers.length === 0
        ? "No member satisfies it yet."
        : `Satisfied by ${requirement.satisfiers.map((s) => `\`${s.address}\``).join(", ")}.`;
    return [bullet(`\`${requirement.name}\`${declared}`, `${requirement.prose.trim()}\n${filled}`)];
  });
  const sections = [
    own,
    fields.length === 0 ? "" : `## Fields\n\n${fields.join("\n")}`,
    outbound.length + inbound.length === 0 ? "" : `## Edges\n\n${[...outbound, ...inbound].join("\n")}`,
    contracted.length === 0 ? "" : `## Requirements\n\n${contracted.join("\n")}`,
  ].filter((section) => section !== "");
  if (sections.length === 0) return undefined;
  // A body of the kind's own counsel alone keeps it exactly as authored.
  if (sections.length === 1 && own !== "") return facts.guidance;
  return sections.join("\n\n");
}

/**
 * The {@link SDK_COUNSEL} rule over the program directory, always, then one `rule`
 * member per **guided corpus kind**: a kind in play whose {@link guidanceBody} is
 * non-empty — the program's own counsel, edge intent and contracts over it. A built-in
 * the program only imports projects none of its shipped counsel (every harness would
 * otherwise get rules it never asked for), so it takes a rule only when the program
 * contracts over it. A relocated built-in is the program's own:
 * `relocate` replaces the base's counsel with the relocation's, so it projects exactly what
 * its relocation declares, and nothing when it declares none. Its `paths` are the kind's
 * {@link locusGlobs}, its body the counsel, so the runtime hands the counsel to an agent
 * the first time a session touches a member's document. A kind whose locus resolves to no
 * file (an embedded kind nothing hosts) has nowhere to scope a rule and projects none.
 *
 * Synthesized the way telemetry synthesizes tap hooks: memberless in the program, an
 * ordinary member at the seam. It takes the program's own `rule` kind when one is in play
 * (a relocated `rule` places it at the relocated root), else the built-in.
 *
 * # Throws
 * If an authored `rule` member already holds a synthesized rule's name — those names
 * are temper's, so two writers at one file is a collision.
 */
function guidanceRules(harness: Harness): Member[] {
  const kinds = kindsInPlay(harness);
  const admissions = admissionsByHost(harness);
  const ruleFacts = kinds.find((facts) => facts.name === rule.key) ?? rule.facts;
  const authored = new Set(harness.members.filter((member) => member.kind === rule.key).map((member) => member.name));
  const synthesize = (name: string, prose: string, paths: string[], owner: string): Member => {
    if (authored.has(name)) {
      throw new Error(
        `rule \`${name}\`: the name is the ${owner} — the \`${GUIDANCE_RULE_PREFIX}\` names and ` +
          `\`${COUNSEL_RULE_NAME}\` are temper's own, so rename the authored rule.`,
      );
    }
    const member: Member = {
      kind: rule.key,
      facts: ruleFacts,
      name,
      prose: span(prose),
      fields: [["paths", paths]],
      satisfies: [],
      requires: {},
      needs: [],
      inputs: [],
    };
    SYNTHESIZED_RULES.add(member);
    return member;
  };
  const rules = [
    synthesize(COUNSEL_RULE_NAME, SDK_COUNSEL, [COUNSEL_RULE_PATHS], "SDK counsel rule temper synthesizes over the program"),
  ];
  const requirements = contractedRequirements(harness);
  for (const facts of kinds) {
    const guidance = guidanceBody(facts, kinds, requirements);
    if (guidance === undefined) continue;
    const paths = locusGlobs(facts, kinds, admissions);
    if (paths.length === 0) continue;
    rules.push(
      synthesize(
        `${GUIDANCE_RULE_PREFIX}${facts.name}`,
        guidance,
        paths,
        `guidance rule temper synthesizes for kind \`${facts.name}\``,
      ),
    );
  }
  return rules;
}

/**
 * `harness` with its {@link guidanceRules} joined into the member roster — the one view
 * every compiled face reads, so the declaration rows and the projected members agree on
 * what was synthesized. Idempotent: a harness already carrying them comes back as is.
 */
export function withGuidanceRules(harness: Harness): Harness {
  if (harness.members.some((member) => SYNTHESIZED_RULES.has(member))) return harness;
  return { ...harness, members: [...harness.members, ...guidanceRules(harness)] };
}

/**
 * Every declared requirement as a `[name, requirement]` entry — the assembly's (its
 * `require` map and the requirement members its roster composed, which `harness()` already
 * resolved), then each member's published `requires`, a requirement member among them read
 * through {@link requirementOf}. The one source every requirement reader walks.
 */
function requirementEntries(harness: Harness): Array<[string, Requirement]> {
  const entries: Array<[string, Requirement]> = Object.entries(harness.require);
  for (const member of harness.members) {
    for (const [name, value] of Object.entries(member.requires)) entries.push([name, requirementOf(name, value)]);
  }
  return entries;
}

/**
 * The requirement rows — assembly `require` and every member's `requires`, one namespace.
 * An unset `required` lowers the corpus's shipped default — "this must be filled": at
 * least one satisfier, at error — so only an explicit `required: false` is advisory.
 */
function requirementRows(harness: Harness): RequirementRow[] {
  const merged = uniqueMap(requirementEntries(harness));
  return [...merged.entries()]
    .sort(([a], [b]) => compareStrings(a, b))
    .map(([name, requirement]) => ({
      name,
      kind: requirement.kind?.key,
      required: requirement.required ?? true,
      clauses: (requirement.clauses ?? []).map((clause) => clauseRow(clause)),
      verifier: verifierRow(requirement.verifier),
      prose: requirement.prose,
    }));
}

/**
 * Lower a typed verifier to its species-tagged wire row — `species` plus the
 * variant's own payload. The generated row carries a mutable `events` column, so
 * the telemetry species copies its read-only names into a fresh array (the same
 * read-only→mutable copy `charset`/`keys`/`values` make above).
 */
function verifierRow(verifier: Verifier | undefined): VerifierRow | undefined {
  if (verifier === undefined) return undefined;
  switch (verifier.species) {
    case "script":
      return { species: "script", path: verifier.path };
    case "telemetry":
      return { species: "telemetry", events: [...verifier.events] };
    case "review":
      return { species: "review", prose: verifier.prose };
  }
}

/**
 * The assembly-scope facts, in a stable order: the root member's declared `mode` and the
 * kind and member overrides beneath it, then one `contract` declaredness marker per `expect`-bound kind (kind-sorted, as `expect`'s
 * own clause rows are) plus the root's where the author declared one, then one edge row
 * per kind edge field.
 *
 * The `contract` marker is what makes an authored empty clause array a declared contract:
 * a clause row count cannot tell `expect(kind, [])` from a kind the program never bound,
 * nor a root `contract: []` from silence, so declaredness rides its own fact and
 * `compose`'s two rows-or-default branches read it (decision 0072). It names its kind in
 * the row's `from` column — the same column an `edge` fact names its source kind in — and
 * omits `from` entirely for the root, the kind-absent-means-root discriminator a root
 * clause row already uses.
 *
 * `kinds` is every kind in play at *any* locus, not just the unit-owning ones that take
 * a kind-fact row: an edge is a declared relationship at any grain, so an embedded
 * kind's edge fields are owed their rows even though the kind itself reaches the lock
 * through its host's `templates` column alone.
 */
function assemblyFactRows(harness: Harness, kinds: readonly KindFacts[]): AssemblyFactRow[] {
  const facts: AssemblyFactRow[] = [{ fact: "mode", value: harness.mode }, ...modeOverrideRows(harness, kinds)];
  for (const binding of [...harness.expect].sort((a, b) => compareStrings(a.kind.key, b.kind.key))) {
    facts.push({ fact: "contract", from: binding.kind.key });
 }
  if (harness.contractDeclared) facts.push({ fact: "contract" });
  for (const kind of kinds) {
    for (const edge of kind.edgeFields ?? []) {
      facts.push({
        fact: "edge",
        from: kind.name,
        field: edge.field,
        to: edgeTargetNames(edge),
        ...(edge.provided !== undefined ? { provided: [...edge.provided] } : {}),
      });
 }
 }
  return facts;
}

/**
 * The `mode` overrides beneath the root's: one per kind declaring its own (kind-sorted,
 * named in `from` as a `contract` marker names its kind), then one per member declaring
 * its own (address-sorted, named in `member`).
 */
function modeOverrideRows(harness: Harness, kinds: readonly KindFacts[]): AssemblyFactRow[] {
  const byKind: AssemblyFactRow[] = kinds
    .flatMap((kind) => (kind.mode === undefined ? [] : [{ fact: "mode", value: kind.mode, from: kind.name }]))
    .sort((a, b) => compareStrings(a.from ?? "", b.from ?? ""));
  const byMember: AssemblyFactRow[] = harness.members
    .flatMap((member) =>
      member.mode === undefined ? [] : [{ fact: "mode", value: member.mode, member: memberAddress(member) }],
    )
    .sort((a, b) => compareStrings(a.member ?? "", b.member ?? ""));
  return [...byKind, ...byMember];
}

/**
 * The `satisfies` rows — every member's fill claims, member-then-requirement sorted.
 * The `member` is the filler's own {@link memberAddress}, the same identity
 * `mentionRows` writes, so the read side joins on a kind-qualified label a same-named
 * member of another kind can never collide with — and a nested file child joins on the
 * host-qualified one it resolves at, never a name its host's sibling also carries.
 */
function satisfiesRows(harness: Harness): SatisfiesRow[] {
  const rows: SatisfiesRow[] = [];
  for (const member of harness.members) {
    const address = memberAddress(member);
    for (const requirement of member.satisfies) {
      rows.push({ member: address, requirement });
 }
 }
  return rows.sort((a, b) => compareStrings(a.member, b.member) || compareStrings(a.requirement, b.requirement));
}

/**
 * The `mention` rows — every member's authored `n` targets, member-then-target
 * sorted. `text`-kind prose contributes one row per mention, keyed to the
 * member's own {@link memberAddress}. A `blocks()` composed body keys each child
 * to what it is: a prose span's mentions are host-level, keyed to the member's
 * own address like a `text` body; an embedded value's `Text`-leaf
 * mentions are keyed to that leaf's own
 * `<host-address>/<kind>/<key>/<child-path>` address
 * ([`embeddedLeafMentionRows`]), headed by the very `address` computed here —
 * every row site in this function composes a host's address through
 * {@link memberAddress}, the leaf branch included. A `file()` body names none.
 * Recorded off the raw authored address, unconditionally — resolution is
 * `emit`'s own refusal (`emit.ts`), not this row's concern.
 */
function mentionRows(harness: Harness): MentionRow[] {
  const rows: MentionRow[] = [];
  for (const member of harness.members) {
    const address = memberAddress(member);
    if (member.prose?.kind === "text") {
      for (const mention of member.prose.mentions) {
        rows.push({ member: address, target: mention.target.address });
 }
 }
    if (member.prose?.kind === "blocks") {
      for (const value of member.prose.values) {
        if (isTextSpan(value)) {
          for (const mention of value.mentions) {
            rows.push({ member: address, target: mention.target.address });
 }
        } else {
          rows.push(...embeddedLeafMentionRows(address, value));
 }
 }
 }
 }
  return rows.sort((a, b) => compareStrings(a.member, b.member) || compareStrings(a.target, b.target));
}

/**
 * The mention rows one `blocks()`-declared embedded-member value's `Text` leaves
 * contribute — top-level leaves addressed by their bare field name, a
 * collection entry's leaves addressed `<collection>.<entry>.<field>` (one layer
 * deep, matching the row's own shape) — each row keyed to the leaf's own
 * structural address ({@link leafAddress}). A bare-string leaf names no mention.
 *
 * `host` is the hosting member's **own whole address** ({@link memberAddress}), at
 * whatever grain the host sits: `kind:name` for a top-level host, the host's whole
 * `<host-address>/<kind>/<key>` address under a nested one. A nested file child's
 * identity *is* its host-qualified address (`compose.rs`'s `host_qualified`), so a
 * head spelled off the bare name alone would name no member at `read.rs`'s
 * `resolve_leaf` — and two sibling children each keyed `home` would head one string
 * where the grammar scopes them to their hosts.
 */
function embeddedLeafMentionRows(host: string, value: EmbeddedMemberValue): MentionRow[] {
  const rows: MentionRow[] = [];
  const addressed = (childPath: string): string => leafAddress(host, value.kind, value.key, childPath);
  for (const [field, leaf] of Object.entries(value.leaves)) {
    if (typeof leaf === "string") continue;
    for (const mention of leaf.mentions) {
      rows.push({ member: addressed(field), target: mention.target.address });
 }
 }
  for (const [collection, entries] of Object.entries(value.collections)) {
    for (const entry of entries) {
      for (const [field, leaf] of Object.entries(entry.leaves)) {
        if (typeof leaf === "string") continue;
        for (const mention of leaf.mentions) {
          rows.push({ member: addressed(`${collection}.${entry.key}.${field}`), target: mention.target.address });
 }
 }
 }
 }
  return rows;
}

/**
 * The `include` rows — every member's `text`-body includes, in member-then-authored
 * order. Each carries the host member's own {@link memberAddress} and the include target's
 * path resolved against the stating module ({@link fileURLToPath} over the include's own
 * `moduleUrl`), never the workspace — the engine reads, splices, and fingerprints it.
 * Member order stays authored (never target-sorted): the body's include slots ride the
 * same order, so the engine pairs the k-th slot with the k-th row. Includes ride a `text`
 * body and a composed body's prose spans alike, in authored order across the interleave; an
 * embedded leaf carries none (refused at `prose.ts`'s `resolveLeaf`) and a `file()` body names none.
 */
function includeRows(harness: Harness): IncludeRow[] {
  const rows: IncludeRow[] = [];
  const push = (address: string, include: Include): void => {
    rows.push({ member: address, source_path: fileURLToPath(new URL(include.path, include.moduleUrl)) });
  };
  for (const member of harness.members) {
    const address = memberAddress(member);
    if (member.prose?.kind === "text") {
      for (const include of member.prose.includes) push(address, include);
    } else if (member.prose?.kind === "blocks") {
      for (const value of member.prose.values) {
        if (!isTextSpan(value)) continue;
        for (const include of value.includes) push(address, include);
      }
    }
  }
  return rows;
}

/**
 * The `input` rows — every member's declared inputs, in member-then-authored order.
 * Each carries the declaring member's own {@link memberAddress} and the input's path resolved
 * against the stating module ({@link fileURLToPath} over the input's own `moduleUrl`),
 * never the workspace — exactly as an include's is; the engine reads and fingerprints
 * it, and splices nothing.
 *
 * Unlike an include, an input pairs with no body slot, so nothing downstream depends on
 * this order — the authored one is kept anyway, so a re-emit is byte-stable and the
 * author reads their own declarations back.
 */
function inputRows(harness: Harness): InputRow[] {
  const rows: InputRow[] = [];
  for (const member of harness.members) {
    const address = memberAddress(member);
    for (const declared of member.inputs) {
      rows.push({ member: address, source_path: fileURLToPath(new URL(declared.path, declared.moduleUrl)) });
    }
  }
  return rows;
}

/**
 * The `module` rows — the program's loaded local modules, deduplicated and sorted so the
 * row order is the set's, never the load order a refactor of import statements reshuffles.
 */
function moduleRows(modules: readonly string[]): string[] {
  return [...new Set(modules)].sort(compareStrings);
}

/**
 * Each composed embedded value's placed edge fields, by {@link nestedAddress} — the very
 * address the `nested_member` row each entry feeds is identified by — `emit`'s record of
 * which declared edges each value's format actually rendered (`emit.ts`'s
 * `edgePlacements`). A value with no entry had no format observe it, which is distinct
 * from a format that placed nothing: the row omits the column entirely and the
 * `format-places-edges` clause stays undecided rather than indict a format that never ran.
 */
export type EdgePlacements = ReadonlyMap<string, readonly string[]>;

/** One composed embedded value's rendered extent — the line and character count of the
 * block `emit` projected for it ({@link RenderedExtents}), the span an `extent` clause
 * budgets. */
export interface RenderedExtent {
  readonly lines: number;
  readonly chars: number;
}

/**
 * Each composed embedded value's rendered extent, by {@link nestedAddress} — `emit`'s
 * record of the span it projected for each value (`emit.ts`'s `renderedExtents`). A value
 * with no entry had no format rendered it (a member read off a layout host's source),
 * which the row spells by omitting both span columns rather than a captured zero: an
 * unmeasured member's `extent` stays undecidable, never a zero read as a pass.
 */
export type RenderedExtents = ReadonlyMap<string, RenderedExtent>;

/**
 * One host member's declared embedded-member value as its declaration row
 * ([`NestedMemberRow`]) — its leaves through the shared walk
 * ({@link resolveEmbeddedLeaves}), the identical resolution `emit.ts` runs on the same
 * leaf's way into the rendered fence — plus the `placed_edges` record `emit` observed
 * while rendering the same value, which is the only way an edge's placement reaches
 * the engine.
 */
function nestedMemberRow(
  host: string,
  value: EmbeddedMemberValue,
  scope: MentionScope,
  placements: EdgePlacements | undefined,
  extents: RenderedExtents | undefined,
): NestedMemberRow {
  const { leaves, collections } = resolveEmbeddedLeaves(value, scope);
  const key = nestedAddress(host, value.kind, value.key);
  const placed = placements?.get(key);
  const extent = extents?.get(key);
  return {
    host,
    kind: value.kind,
    key: value.key,
    leaves,
    collections,
    // Omitted, never `undefined`-valued: an absent column is the wire's own spelling of
    // "no format placement was observed here", so a value with no edge to place keeps
    // the row it has always written. The generated row carries a mutable column and the
    // placement record is read-only, so the copy is a fresh array.
    ...(placed === undefined ? {} : { placed_edges: [...placed] }),
    // The rendered span, on the same omitted-not-null discipline: a value no render
    // observed (compiled without `emit`'s measurement) keeps the row it has always
    // written, and its `extent` reads as undecidable rather than a captured zero.
    ...(extent === undefined ? {} : { rendered_lines: extent.lines, rendered_chars: extent.chars }),
  };
}

/** The `nested_member` row of a root-hosted roster member, its string fields its leaves. */
function rootHostedRow(member: Member): NestedMemberRow {
  const leaves: Record<string, string> = {};
  for (const [field, value] of member.fields) {
    if (typeof value === "string") leaves[field] = value;
  }
  return { host: ROOT_ADDRESS, kind: member.kind, key: member.name, leaves, collections: {} };
}

/**
 * The `nested_member` rows of the declared requirements — each the root's nested member
 * `root/<kind>/<name>`, its kind the member's own (a subkind's name, `requirement` for a
 * literal) and its string fields its leaves, so a subkind's own fields reach `check` and
 * the read verbs at leaf grain. The requirement row still carries the contract-facing
 * template (fill posture, clauses, verifier, prose); this row is the member's identity.
 */
function requirementHostedRows(harness: Harness): NestedMemberRow[] {
  return [...uniqueMap(requirementEntries(harness)).entries()].map(([name, declared]) => {
    const source = requirementSource(declared);
    const leaves: Record<string, string> = {};
    for (const [field, value] of source?.fields ?? []) {
      if (typeof value === "string") leaves[field] = value;
    }
    return { host: ROOT_ADDRESS, kind: source?.kind ?? requirement.key, key: name, leaves, collections: {} };
  });
}

/** Each host kind's admitted embedded kinds, by host kind name — the corpus's `admit`, indexed. */
type AdmissionsByHost = ReadonlyMap<string, ReadonlySet<string>>;

/**
 * The corpus's `admit` declarations, indexed by host kind name — the one source both
 * {@link templatesFor} and {@link nestedMemberRows} read. Admission carries kind values,
 * so an admitted kind is imported and thereby in play; repeated hosts union.
 *
 * # Throws
 * If an admission names a non-embedded kind: a body composes embedded members, and a
 * file-locus kind owns a file instead — admitting one declares a nesting no locus backs.
 */
function admissionsByHost(harness: Harness): AdmissionsByHost {
  const map = new Map<string, Set<string>>();
  for (const { host, admits } of harness.admit) {
    const admitted = map.get(host.key) ?? new Set<string>();
    for (const child of admits) {
      if (child.facts.locus.kind !== "embedded") {
        throw new Error(
          `host kind \`${host.key}\` admits \`${child.key}\`, which is not an embedded kind — ` +
            `a composed body admits embedded members only ` +
            `(specs/model/representation.md, "nesting").`,
        );
      }
      admitted.add(child.key);
    }
    map.set(host.key, admitted);
  }
  return map;
}

/**
 * The `nested_member` rows — every host member's `blocks()`-declared embedded-member
 * values, host-then-kind-then-key sorted, each keyed to its host's own
 * {@link memberAddress} so a host that is itself a nested file child keys by the
 * host-qualified address `check` resolves it at. Only a composed body's embedded values
 * carry them (a `file()`/`text` body — and a composed body's prose spans — name none),
 * beside each root-hosted roster member ({@link rootHostedRow}); the
 * fence rendering itself is unchanged
 * (`emit.ts`'s `resolveBody`) — this row is a second *read* of the same authored
 * value, never a second copy the engine reads back (0018).
 *
 * Refuses an unadmitted nesting before a byte is written: the corpus must admit the
 * value's kind over the hosting member's kind. `templates` derives from that same
 * admission ({@link templatesFor}), so an unadmitted value would reach the lock as a
 * `nested_member` row no `templates` column admits, to be unmodeled without a word —
 * admission is the adopting corpus's own declaration, so an unadmitted nested member is
 * an unresolved input, not output to write over.
 */
function nestedMemberRows(
  harness: Harness,
  admissions: AdmissionsByHost,
  scope: MentionScope,
  placements: EdgePlacements | undefined,
  extents: RenderedExtents | undefined,
): NestedMemberRow[] {
  const rows: NestedMemberRow[] = [];
  for (const member of harness.members) {
    if (ROOT_HOSTED_KINDS.has(member.kind)) {
      rows.push(rootHostedRow(member));
      continue;
    }
    if (member.prose?.kind !== "blocks") continue;
    const host = memberAddress(member);
    for (const value of member.prose.values) {
      if (isTextSpan(value)) continue;
      if (!admissions.get(member.kind)?.has(value.kind)) {
        throw new Error(
          `member \`${member.name}\`: embedded value \`${value.key}\` is of kind ` +
            `\`${value.kind}\`, which does not nest within host kind \`${member.kind}\` — a ` +
            `\`blocks()\` value's kind must be one the harness \`admit\`s over the host kind ` +
            `(specs/model/representation.md, "nesting").`,
        );
      }
      rows.push(nestedMemberRow(host, value, scope, placements, extents));
 }
 }
  rows.push(...requirementHostedRows(harness));
  return rows.sort(
    (a, b) => compareStrings(a.host, b.host) || compareStrings(a.kind, b.kind) || compareStrings(a.key, b.key),
 );
}

/**
 * The `registration` rows — every fields-only registration member (a hook, an MCP server)
 * erased for the manifest write face, kind-then-key sorted so double emit is byte-stable.
 * Each carries its **collection key** (`kind`/`key`), its manifest (the kind's locus,
 * {@link manifestOf}) and key path (the wire's snake_case `key_path`), and its folded typed fields —
 * the entry value the engine's write face places under `key`. The authored half alone:
 * the synthesized tap hooks join these rows in {@link mergedRegistrationRows}, which is
 * what both readers of the family take.
 *
 * The row's `key` is where the entry **writes**, which for every kind but one is the
 * member's own name. A hook's name joins its matcher onto the event because the group is
 * the member, while the group still writes under `hooks.<Event>` — so the row keys the
 * event back off the name ({@link hookCollectionKey}) and the member's address keeps the
 * join.
 *
 * # Throws
 * If a fields-only member declares no collection address — it surfaces in no host manifest.
 */
function registrationRows(harness: Harness): RegistrationRow[] {
  return harness.members
    .filter((member) => member.facts.shape === "fields" && member.facts.locus.kind !== "embedded")
    .map((member): RegistrationRow => {
      const address = collectionAddressOf(member.facts);
      if (address === undefined) {
        throw new Error(
          `member \`${member.name}\`: a fields-only registration kind declares no ` +
            `collection address — it surfaces in no host manifest (specs/model/authoring.md, "The SDK").`,
        );
      }
      return {
        kind: member.kind,
        key: member.kind === hook.key ? hookCollectionKey(member.name) : member.name,
        manifest: manifestOf(member.facts),
        key_path: address.keyPath,
        // The generated row carries a mutable field list; the member's is read-only, so
        // copy each pair into a fresh tuple — the same values, a shape the row accepts.
        fields: [
          ...member.fields.map(([name, value]): [string, unknown] => [name, projectedField(member, name, value)]),
          // A gate hook carries its tool's address across the seam beside its fields; the
          // engine renders the handler's command from it and drops the key ({@link gateHook}).
          ...gateToolField(harness, member),
        ],
      };
    })
    .sort((a, b) => compareStrings(a.kind, b.kind) || compareStrings(a.key, b.key));
}

/** The settings key a status line rides under — {@link Settings}' `statusLine`. */
const STATUS_LINE_KEY = "statusLine";

/** `value` without its {@link RUNS_FIELD} edge, where it is an object carrying one. */
function withoutRuns(value: unknown): unknown {
  if (typeof value !== "object" || value === null || Array.isArray(value) || !(RUNS_FIELD in value)) return value;
  const { [RUNS_FIELD]: _runs, ...projected } = value as Record<string, unknown>;
  return projected;
}

/**
 * A member's field as its artifact carries it: a hook's handlers and a settings member's
 * `statusLine` each lose the {@link RUNS_FIELD} edge, which is temper's and rides the lock
 * as a `runs` row ({@link runsRows}), never a key in `settings.json`. Every other field is
 * as authored. The one projection of a runner's fields, read by the registration rows here
 * and by the payload members `emit.ts` hands the engine.
 */
export function projectedField(member: Member, name: string, value: unknown): unknown {
  if (member.kind === hook.key && name === HOOK_HANDLER_KEY && Array.isArray(value)) return value.map(withoutRuns);
  if (member.kind === settings.key && name === STATUS_LINE_KEY) return withoutRuns(value);
  return value;
}

/**
 * Each runner a member carries, paired with the `runs` value it names: a hook's handlers,
 * each at its own address (`<hook-address>/handler/<position>`, where the engine reads the
 * handler back), and a settings member's `statusLine`, at the member's own address.
 */
function runners(member: Member): Array<readonly [string, unknown]> {
  const field = (name: string): unknown => member.fields.find(([key]) => key === name)?.[1];
  const runsOf = (value: unknown): unknown =>
    typeof value === "object" && value !== null ? (value as Record<string, unknown>)[RUNS_FIELD] : undefined;
  if (member.kind === hook.key) {
    const handlers = field(HOOK_HANDLER_KEY);
    if (!Array.isArray(handlers)) return [];
    return handlers.map(
      (handler: unknown, position) =>
        [nestedAddress(memberAddress(member), "handler", String(position)), runsOf(handler)] as const,
    );
  }
  if (member.kind === settings.key) return [[memberAddress(member), runsOf(field(STATUS_LINE_KEY))]];
  return [];
}

/**
 * The `runs` rows — each runner naming the {@link tool} it runs (a hook handler, a
 * settings status line, the root for its typecheck, a requirement for its telemetry tap or script verifier), keyed to the runner's own address and the tool's
 * (`root/tool/<name>`), in member-then-position order. The edge is declared, never read
 * off the runner's `command`: the program states which tool a command runs.
 *
 * # Throws
 * If a `runs` is not a `tool(...)` member, or names a tool the harness does not compose —
 * the edge would point at nothing the lock carries.
 */
function runsRows(harness: Harness): RunsRow[] {
  const rows: RunsRow[] = [];
  for (const member of harness.members) {
    for (const [runner, runs] of runners(member)) {
      if (runs === undefined) continue;
      rows.push({ member: runner, tool: composedToolAddress(harness, runner, "runs", runs) });
    }
  }
  // The harness root declares the typecheck, so the root is the compiler tool's runner.
  if (harness.typecheck !== undefined) {
    rows.push({ member: ROOT_ADDRESS, tool: composedToolAddress(harness, "typecheck", "tool", harness.typecheck.tool) });
  }
  // A telemetry requirement declares the tool its tap hooks run, and a script requirement the
  // tool that runs its script, so the requirement is the runner.
  const verified = requirementEntries(harness).sort(([a], [b]) => compareStrings(a, b));
  for (const [named, declared] of verified) {
    const verifier = declared.verifier;
    if (verifier === undefined || verifier.species === "review" || verifier.tool === undefined) continue;
    rows.push({
      member: nestedAddress(ROOT_ADDRESS, requirement.key, named),
      tool: composedToolAddress(harness, named, "tool", verifier.tool),
    });
  }
  return rows;
}

/**
 * The address (`root/tool/<name>`) of the composed {@link tool} a runner names under
 * `field` — the one check every runner→tool reference passes, a `runs` edge and the
 * typecheck alike.
 *
 * # Throws
 * If the value is not a `tool(...)` member, or names a tool the harness does not compose —
 * the reference would point at nothing the lock carries.
 */
function composedToolAddress(harness: Harness, runner: string, field: string, value: unknown): string {
  if (typeof value !== "object" || value === null || (value as Member).kind !== tool.key) {
    throw new Error(`\`${runner}\`: \`${field}\` names the tool the command runs — pass a \`tool(...)\` member.`);
  }
  if (!harness.members.includes(value as Member)) {
    throw new Error(
      `\`${runner}\`: \`${field}\` names tool \`${(value as Member).name}\`, which the harness ` +
        `does not compose — add it to \`members\`.`,
    );
  }
  return memberAddress(value as Member);
}

/**
 * The typecheck row — the declared compiler's tool address and the project file, resolved
 * against the stating module like any {@link input} — or `null` where the program declares
 * none. Seam-inbound only: the engine runs it before compiling and lowers the project to a
 * root-owned input.
 *
 * # Throws
 * If the typecheck's `tool` is not a composed `tool(...)` member ({@link composedToolAddress}).
 */
function typecheckRow(harness: Harness): TypecheckRow | null {
  const declared = harness.typecheck;
  if (declared === undefined) return null;
  return {
    tool: composedToolAddress(harness, "typecheck", "tool", declared.tool),
    project: fileURLToPath(new URL(declared.project.path, declared.project.moduleUrl)),
  };
}

/** The gate hook row's tool field — `[GATE_TOOL_KEY, address]` for a {@link gateHook}
 * member, `null` declaring none, and nothing for every other member.
 *
 * # Throws
 * If the gate hook names a tool the harness does not compose ({@link composedToolAddress}). */
function gateToolField(harness: Harness, member: Member): Array<[string, unknown]> {
  const reached = gateHookTool(member);
  if (reached === undefined) return [];
  return [[GATE_TOOL_KEY, reached === null ? null : composedToolAddress(harness, memberAddress(member), "tool", reached)]];
}

/**
 * Builds a collision-safe dedup key for a tap hook (event, matcher) pair.
 * Two distinct pairs cannot produce the same key (e.g., ("Foo", "BarBaz") ≠ ("FooBar", "Baz")).
 * @internal
 */
export function buildTapHookDedupeKey(event: string, matcher: string): string {
  return JSON.stringify([event, matcher]);
}

/**
 * The synthesized tap-hook `registration` rows — one deduped `hooks.<Event>`
 * registration per (lifecycle event, matcher) any telemetry verifier names. Scans the
 * same requirement sources {@link requirementRows} reads (assembly `require` ∪ each
 * member's `requires`), keeps the telemetry-species verifiers ({@link Verifier}), and
 * unions the lifecycle events they name into one dumb registration apiece: the tap
 * records every fire and read time joins raw events to members, so however many
 * verifiers name an event it takes exactly one hook — the derived-aggregate precedent
 * the permission union sets ({@link permissionUnion}). Each row is one matcher group
 * under the event's documented matcher ({@link TELEMETRY_EVENT_HOOKS}) whose one handler
 * carries the address of the tool the naming declarations reach — the engine renders the
 * tap command from it, anchored at the project (`src/drift.rs`) — its key-path and fields sourced from the provider face
 * ({@link tapHookRegistration}, `builtins.ts`); an event-name outside that table is the
 * roster's inadmissibility finding, never a row. Synthesized in isolation here — whether a
 * row stands alone or joins an authored group at the same pair is
 * {@link mergedRegistrationRows}'s call, so this pass need not know what the program
 * authored.
 *
 * Declarations sharing a tap but naming different tools each keep a handler, named for
 * the first declaration naming it: whether two tools agree is the command the engine
 * renders (an undeclared one is the engine's default), so the engine merges or refuses
 * them — this pass never resolves the default.
 *
 * # Throws
 * If a declaration names a tool the harness does not compose ({@link composedToolAddress}).
 */
function tapHookRows(harness: Harness): RegistrationRow[] {
  const deduped = new Map<
    string,
    {
      readonly event: string;
      readonly matcher: string;
      readonly declarations: { readonly tool: string | null; readonly named: string }[];
    }
  >();
  const collect = (named: string, requirement: Requirement): void => {
    if (requirement.verifier?.species !== "telemetry") return;
    const reached = requirement.verifier.tool;
    const tool = reached === undefined ? null : composedToolAddress(harness, named, "tool", reached);
    for (const name of requirement.verifier.events) {
      const mapping = TELEMETRY_EVENT_HOOKS[name];
      if (mapping === undefined) continue;
      const key = buildTapHookDedupeKey(mapping.event, mapping.matcher);
      const tap = deduped.get(key) ?? { ...mapping, declarations: [] };
      deduped.set(key, tap);
      if (!tap.declarations.some((declared) => declared.tool === tool)) {
        tap.declarations.push({ tool, named });
      }
    }
  };
  for (const [named, requirement] of requirementEntries(harness)) collect(named, requirement);
  return [...deduped.values()]
    .sort((a, b) => compareStrings(a.event, b.event) || compareStrings(a.matcher, b.matcher))
    .map(({ event, matcher, declarations }): RegistrationRow => {
      const { keyPath, fields } = tapHookRegistration(declarations, matcher);
      return {
        kind: hook.key,
        key: event,
        manifest: SETTINGS_MANIFEST,
        key_path: keyPath,
        fields,
      };
    });
}

/** A hook registration row's matcher — the group-level half of its identity, an omitted
 * one read as the empty matcher it means (`"*"`/`""`/absent all name every tool). */
function groupMatcher(row: RegistrationRow): string {
  const found = row.fields.find(([name]) => name === HOOK_MATCHER_KEY)?.[1];
  return typeof found === "string" ? found : "";
}

/** A hook registration row's handlers — the entries of its `hooks` array, or none where the
 * row spells no array at all. */
function groupHandlers(row: RegistrationRow): readonly unknown[] {
  const found = row.fields.find(([name]) => name === HOOK_HANDLER_KEY)?.[1];
  return Array.isArray(found) ? found : [];
}

/** `row` with `extra` appended to its handler array — a fresh row over a fresh array, never
 * a mutation: the authored array is the composing program's own value, and `emit` must be
 * callable twice on one harness for the same bytes. */
function withHandlers(row: RegistrationRow, extra: readonly unknown[]): RegistrationRow {
  const handlers = [...groupHandlers(row), ...extra];
  const carried = row.fields.some(([name]) => name === HOOK_HANDLER_KEY);
  const fields = row.fields.map(([name, value]): [string, unknown] =>
    name === HOOK_HANDLER_KEY ? [name, handlers] : [name, value],
  );
  return { ...row, fields: carried ? fields : [...fields, [HOOK_HANDLER_KEY, handlers]] };
}

/**
 * Every `registration` row the program carries — the authored fields-only members
 * ({@link registrationRows}) with the synthesized tap hooks ({@link tapHookRows}) **joined
 * in**, and the one home both readers take: `compileDeclarations` writes it to
 * `declarations.registrations` and `emit.ts`'s public {@link RegistrationFact} view maps
 * from it, so the seam payload and the `EmitResult` sibling cannot disagree on whether a
 * tap joined an authored group (`specs/process/engineering.md`, "One job, one home").
 *
 * A tap hook is an ordinary matcher group, so a tap whose (event, matcher) an authored
 * group already claims **is** that group's further handler rather than a second group at
 * the same key (0075): its handlers append to the claimant's `hooks` array in tap order,
 * and only an unclaimed pair mints a row of its own. The claimant is the first authored
 * row at the pair — a repeated pair joins one group rather than fanning the tap across
 * every one of them. Authored rows keep their own order and position, so the merge moves
 * no byte that did not need to move.
 *
 * # Throws
 * If a fields-only member declares no collection address ({@link registrationRows}).
 */
export function mergedRegistrationRows(harness: Harness): RegistrationRow[] {
  const authored = registrationRows(harness);
  const claimant = new Map<string, number>();
  authored.forEach((row, index) => {
    if (row.kind !== hook.key) return;
    const key = buildTapHookDedupeKey(row.key, groupMatcher(row));
    if (!claimant.has(key)) claimant.set(key, index);
  });

  const joined = new Map<number, unknown[]>();
  const unclaimed: RegistrationRow[] = [];
  for (const tap of tapHookRows(harness)) {
    const index = claimant.get(buildTapHookDedupeKey(tap.key, groupMatcher(tap)));
    if (index === undefined) {
      unclaimed.push(tap);
      continue;
    }
    const handlers = joined.get(index) ?? [];
    handlers.push(...groupHandlers(tap));
    joined.set(index, handlers);
  }

  const rows = authored.map((row, index) => {
    const extra = joined.get(index);
    return extra === undefined ? row : withHandlers(row, extra);
  });
  return [...rows, ...unclaimed];
}

/**
 * The `settings` rows — the assembly's harness-level residual settings keys, each folded
 * into the settings.json manifest's opaque residue at emit. Key-sorted so double emit is
 * byte-stable. Seam-inbound: the value lives in the projected manifest, never the lock.
 */
export function settingsRows(harness: Harness): SettingsRow[] {
  return Object.entries(harness.settings)
    .map(([key, value]): SettingsRow => ({ manifest: SETTINGS_MANIFEST, key, value }))
    .sort((a, b) => compareStrings(a.key, b.key));
}

/** The playbook areas a guard attachment binds, one row per guarded playbook, in member
 * order — what `temper guard` reads an edit's path against. */
function areaRows(harness: Harness): AreaRow[] {
  return harness.members.flatMap((member) => {
    const area = playbookArea(member);
    return area === undefined ? [] : [{ rule: area.rule, paths: [...area.paths], mode: area.mode }];
  });
}

/** Every requirement name a `satisfies` claim may fill — assembly `require` ∪ member `requires`. */
export function declaredRequirements(harness: Harness): Set<string> {
  const set = new Set<string>();
  for (const name of Object.keys(harness.require)) set.add(name);
  for (const member of harness.members) {
    for (const name of Object.keys(member.requires)) set.add(name);
 }
  return set;
}

/**
 * Every address a mention may name — declared requirement names ∪ each member's own
 * {@link memberAddress} ∪ each `blocks()`-declared embedded member's host-scoped
 * `<host-kind>:<host-name>/<kind>/<key>` address. Shared by `emit.ts` (a
 * member-level `Text` body's mentions) and this module (an embedded member's
 * `Text` leaves) — the one resolution-check set, so a leaf mention and a member
 * mention are held to the identical bar. The embedded address is host-scoped,
 * never a flat `<kind>:<key>` — flat would force corpus-wide key uniqueness on
 * embedded kinds, and a nested **file** child's own address carries its host for
 * exactly that reason: two hosts may each carry a `home`, and a mention names one.
 */
export function declaredAddresses(harness: Harness): Set<string> {
  const set = declaredRequirements(harness);
  for (const member of harness.members) {
    set.add(memberAddress(member));
    if (member.prose?.kind !== "blocks") continue;
    for (const value of member.prose.values) {
      if (isTextSpan(value)) continue;
      set.add(nestedAddress(memberAddress(member), value.kind, value.key));
    }
  }
  return set;
}

/**
 * Every discoverable (`at`-locus) kind the program declares — the deferral signal a
 * dangling mention is measured against (`prose.ts`'s `defersToGate`): a mention whose
 * address is a host address of one of these and names no composed value defers to
 * `check`, while a mention naming no declared kind refuses at emit. Member kinds ∪ `expect` kinds; an embedded
 * kind is excluded — its members are composed within a host, never discovered, so a
 * flat `kind:name` mention of one has no discovery locus to defer to.
 */
export function declaredAtLocusKinds(harness: Harness): Set<string> {
  return new Set(atLocusKindsInPlay(kindsInPlay(harness)).map((facts) => facts.name));
}

/** The full {@link MentionScope} the program resolves a mention against — its addresses and its deferral kinds. */
function mentionScope(harness: Harness): MentionScope {
  return { mentionable: declaredAddresses(harness), deferrableKinds: declaredAtLocusKinds(harness) };
}

/**
 * Compile a harness into its declaration families — the erased program.
 *
 * `placements` and `extents` are `emit`'s record of what each embedded value's format
 * rendered (`emit.ts`'s `edgePlacements`) and the span it spanned (`renderedExtents`); this
 * pass compiles declarations and never renders, so it observes neither itself. Omitted,
 * every `nested_member` row omits its `placed_edges`/`rendered_lines`/`rendered_chars`
 * columns — honest (nothing observed a render) but undecidable for a `format-places-edges`
 * or `extent` clause, so a whole compile goes through `emit`, never this alone.
 */
export function compileDeclarations(
  program: Harness,
  placements?: EdgePlacements,
  extents?: RenderedExtents,
  modules: readonly string[] = [],
): Declarations {
  const harness = withGuidanceRules(program);
  const allKinds = kindsInPlay(harness);
  const admissions = admissionsByHost(harness);
  const clauses: ClauseRow[] = [];
  for (const binding of [...harness.expect].sort((a, b) => compareStrings(a.kind.key, b.kind.key))) {
    for (const clause of binding.clauses) {
      clauses.push(clauseRow(clause, binding.kind.key));
 }
 }
  // The root member's own clauses — the third source of a clause row, beside `expect`'s
  // kind-keyed rows and a requirement's nested ones. They lower with **no** `kind`
  // column: that absence at the top level is the discriminator
  // `compose::root_contract_from_rows` reads, and `drift::stamp_clause_labels` addresses
  // them under the `root` owner segment. Declaration order within the array, appended
  // past the kind-sorted `expect` rows — a fixed position, so double emit is byte-stable.
  for (const clause of harness.contract) {
    clauses.push(clauseRow(clause, undefined));
 }
  return {
    kinds: kindFactKindsInPlay(allKinds).map((facts) => kindFactRow(facts, admissions)),
    clauses,
    requirements: requirementRows(harness),
    assembly: assemblyFactRows(harness, allKinds),
    satisfies: satisfiesRows(harness),
    mentions: mentionRows(harness),
    runs: runsRows(harness),
    includes: includeRows(harness),
    inputs: inputRows(harness),
    modules: moduleRows(modules),
    typecheck: typecheckRow(harness),
    nested_members: nestedMemberRows(harness, admissions, mentionScope(harness), placements, extents),
    registrations: mergedRegistrationRows(harness),
    settings: settingsRows(harness),
    areas: areaRows(harness),
  };
}

/** The SDK's pinned engine/interchange version — the JSON pipe rides it in lockstep. */
const SEAM_VERSION = 2;

/**
 * Serialize the seam payload to the internal versioned JSON pipe — `encodeSeam`
 * stamps `version`, so the caller supplies the rest of the {@link Payload}.
 * Deterministic: insertion-ordered keys and a trailing newline, so a re-emit is
 * byte-identical.
 */
export function encodeSeam(payload: Omit<Payload, "version">): string {
  return JSON.stringify({ version: SEAM_VERSION, ...payload }, null, 2) + "\n";
}
