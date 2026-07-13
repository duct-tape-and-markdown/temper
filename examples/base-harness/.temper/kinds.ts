/**
 * The base harness's own doc kinds. Two postures, both first-class:
 *
 * - `system`/`flow`/`decision`/`superseded-decision` are **composed**: each
 *   member is a typed value in the program, its narrative prose a
 *   module-adjacent markdown file, its relationships typed fields — and the
 *   document under `docs/` is a projection. An edge is an import: a dangling
 *   participant is a compile error before the gate ever runs.
 * - `glossary` is a **layout source**: the document is the authored home,
 *   read under its declared layout, never regenerated — the posture for
 *   prose-first content whose sections are model structure (each term an
 *   addressable member).
 */

import { kind } from "@dtmd/temper";
import type { KindDefinition, Member } from "@dtmd/temper";
import type { Prose } from "@dtmd/temper";

/** Spell a member list as the name array a frontmatter edge field carries. */
export const names = (...members: readonly Member[]): readonly string[] =>
  members.map((member) => member.name);

/**
 * `system` — one area of declared behavior, projected to
 * `docs/systems/<name>.md`. No fields: the body is the authored narrative,
 * and spine membership is the member's own `satisfies` declaration.
 */
export const system: KindDefinition<{ readonly prose?: Prose }> = kind({
  name: "system",
  locus: { kind: "at", root: "docs/systems", glob: "*.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
});

/** A flow's typed fields: the systems it crosses, as a frontmatter edge field. */
export interface Flow {
  readonly participants: readonly string[];
  readonly prose?: Prose;
}

/**
 * `flow` — behavior that crosses systems, projected to
 * `docs/flows/<name>.md`. `participants` is an edge field the gate resolves
 * within `system`; author it with `names(...)` so the reference is an import.
 */
export const flow: KindDefinition<Flow> = kind<Flow>({
  name: "flow",
  locus: { kind: "at", root: "docs/flows", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [{ via: "always" }],
  edgeFields: [{ field: "participants", to: "system" }],
});

/**
 * `decision` — an accepted ruling, projected to `docs/decisions/<name>.md`.
 * Lifecycle is positional: a decision here is current; superseding one is a
 * typed operation (`supersede`) that constructs its replacement's record in
 * `docs/decisions/superseded/`, never an edit.
 */
export const decision: KindDefinition<{ readonly prose?: Prose }> = kind({
  name: "decision",
  locus: { kind: "at", root: "docs/decisions", glob: "*.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
});

/** A superseded ruling's typed fields: the successor edge, unconditionally required. */
export interface SupersededDecision {
  readonly "superseded-by": string;
  readonly prose?: Prose;
}

/**
 * `superseded-decision` — a replaced ruling, projected to
 * `docs/decisions/superseded/<name>.md`. The conditional
 * "`superseded_by` iff superseded" dissolves twice over: into the kind
 * partition, and into the field's own type — a member without the edge does
 * not compile.
 */
export const supersededDecision: KindDefinition<SupersededDecision> = kind<SupersededDecision>({
  name: "superseded-decision",
  locus: { kind: "at", root: "docs/decisions/superseded", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [{ via: "always" }],
  edgeFields: [{ field: "superseded-by", to: "decision" }],
});

/**
 * Supersede: the lifecycle transition as a typed operation. Takes the
 * successor as a value (an import, so it must exist) and the replaced
 * ruling's record; returns the `superseded-decision` member.
 */
export const supersede = (
  successor: Member,
  record: { readonly name: string; readonly prose?: Prose },
): Member =>
  supersededDecision({
    name: record.name,
    "superseded-by": successor.name,
    prose: record.prose,
  });

/**
 * `glossary` — one host document, `docs/glossary.md`, whose terms collection
 * makes each term an addressable member a mention can target. A layout
 * source: the file is the authored home; edit it in place.
 */
export const glossary: KindDefinition<Record<never, never>> = kind({
  name: "glossary",
  locus: { kind: "at", root: "docs", glob: "glossary.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
  content: {
    regions: [{ region: "prose" }, { region: "collection", memberKind: "term" }],
  },
});
