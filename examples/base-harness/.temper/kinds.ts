/**
 * The base harness's own doc kinds. Three postures, each first-class:
 *
 * - `system`/`flow`/`decision`/`superseded-decision` are **composed**: each
 *   member is a typed value in the program, its body composed from declared
 *   embedded members (`passage`, `invariant`, `step`, `alternative`) whose
 *   markdown rendering is each kind's own writer-only `render` hook — and
 *   the document under `docs/` is a projection. An edge is an import: a
 *   dangling reference is a compile or emit error before the gate ever runs.
 * - `glossary` is a **layout source**: the document is the authored home,
 *   read under its declared layout, never regenerated — the posture for
 *   prose-first content whose sections are model structure.
 * - `source` is **read-only ground**: it makes each file under `src/`
 *   an addressable member so a system's `implemented-by` claim is an edge
 *   the gate resolves — and a claim naming a moved or deleted file fails.
 */

import { embeddedMemberValue, kind, mentionOf, text } from "@dtmd/temper";
import type { EmbeddedMemberValue, KindDefinition, Member, Prose, Text } from "@dtmd/temper";

/**
 * `passage` — a narrative span of a composed document, rendered verbatim.
 * A member's body is one prose constructor, so a host with typed children
 * carries its narrative as passages; a passage may hold its own markdown
 * headings, which stay prose (the model's own line: what does not fit the
 * three primitives is prose).
 */
export const passage: KindDefinition<Record<never, never>> = kind(
  {
    name: "passage",
    locus: { kind: "embedded", withinHosts: ["system", "flow", "decision"] },
    unitShape: "file",
    registration: [{ via: "always" }],
  },
  { render: (value) => value.leaves.text },
);

/** Compose one passage value. */
export const passageOf = (key: string, body: string | Text): EmbeddedMemberValue =>
  embeddedMemberValue({ kind: passage, key, leaves: { text: body } });

/**
 * `invariant` — one declared property of a system, addressable structure
 * rather than a heading convention. Rendered as its own section.
 */
export const invariant: KindDefinition<Record<never, never>> = kind(
  {
    name: "invariant",
    locus: { kind: "embedded", withinHosts: ["system"] },
    unitShape: "file",
    registration: [{ via: "always" }],
  },
  { render: (value) => `### ${value.leaves.title}\n\n${value.leaves.body}` },
);

/** Compose one invariant value. */
export const invariantOf = (key: string, title: string, body: string | Text): EmbeddedMemberValue =>
  embeddedMemberValue({ kind: invariant, key, leaves: { title, body } });

/**
 * `step` — one move of a flow, carrying the edge to the system it happens
 * in as a mention on its own `in` leaf: the graph sees flow → system
 * whether or not the narrative names it.
 */
export const stepKind: KindDefinition<Record<never, never>> = kind(
  {
    name: "step",
    locus: { kind: "embedded", withinHosts: ["flow"] },
    unitShape: "file",
    registration: [{ via: "always" }],
  },
  { render: (value) => `### ${value.leaves.title}\n\n${value.leaves.body}\n\nSystem: ${value.leaves.in}.` },
);

/** One authored step: the embedded value plus the system it happens in. */
export interface Step {
  readonly value: EmbeddedMemberValue;
  readonly in: Member;
}

/** Compose one step; `in` is the system member itself, an import. */
export const step = (init: {
  readonly key: string;
  readonly title: string;
  readonly in: Member;
  readonly body: string | Text;
}): Step => ({
  in: init.in,
  value: embeddedMemberValue({
    kind: stepKind,
    key: init.key,
    leaves: { title: init.title, in: text`${mentionOf(init.in)}`, body: init.body },
  }),
});

/**
 * The participants roll-up, derived from the steps — a rendering, never a
 * stored field: the steps are the one authored surface, so the summary can
 * never disagree with them.
 */
export const participantsLine = (steps: readonly Step[]): string => {
  const names = [...new Set(steps.map((entry) => entry.in.name))];
  return `Participants: ${names.join(", ")}.`;
};

/**
 * `alternative` — one rejected road of a decision, addressable so a later
 * ruling that adopts it can point at it rather than paraphrase it.
 */
export const alternative: KindDefinition<Record<never, never>> = kind(
  {
    name: "alternative",
    locus: { kind: "embedded", withinHosts: ["decision"] },
    unitShape: "file",
    registration: [{ via: "always" }],
  },
  { render: (value) => `### ${value.leaves.title}\n\n${value.leaves.body}` },
);

/** Compose one alternative value. */
export const alternativeOf = (key: string, title: string, body: string | Text): EmbeddedMemberValue =>
  embeddedMemberValue({ kind: alternative, key, leaves: { title, body } });

/** A system's typed fields: the source files implementing it, an edge field. */
export interface System {
  readonly "implemented-by": readonly string[];
  readonly prose?: Prose;
}

/**
 * `system` — one area of declared behavior, projected to
 * `docs/systems/<name>.md`. `implemented-by` is the outward arrow: an edge
 * the gate resolves within `source`, so the claim fails when the file goes.
 */
export const system: KindDefinition<System> = kind<System>({
  name: "system",
  locus: { kind: "at", root: "docs/systems", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [{ via: "always" }],
  edgeFields: [{ field: "implemented-by", to: "source" }],
});

/**
 * `flow` — behavior that crosses systems, projected to
 * `docs/flows/<name>.md`. It carries no participants field: the steps own
 * the flow → system edges, and the roll-up is rendered from them.
 */
export const flow: KindDefinition<{ readonly prose?: Prose }> = kind({
  name: "flow",
  locus: { kind: "at", root: "docs/flows", glob: "*.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
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

/**
 * `source` — the governed implementation, one member per file under `src/`.
 * Read-only ground: never projected, never contracted; it exists so an
 * `implemented-by` entry is an address the gate can refuse.
 */
export const source: KindDefinition<Record<never, never>> = kind({
  name: "source",
  locus: { kind: "at", root: "src", glob: "*.js" },
  unitShape: "file",
  registration: [{ via: "always" }],
});
