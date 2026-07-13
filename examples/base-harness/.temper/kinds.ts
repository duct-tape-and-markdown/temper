/**
 * The base harness's own doc kinds — declared with the same `kind()` every
 * provider uses. Each is a layout-content kind: its document is the authored
 * home, read under the declared layout, never regenerated. Fields live in the
 * body as heading sections (regions bind to top-level headings in document
 * order); an edge slot's entries are addresses, not prose.
 */

import { kind } from "@dtmd/temper";
import type { KindDefinition } from "@dtmd/temper";

/** Layout kinds carry no frontmatter fields; their slots live in the body. */
type BodyOnly = Record<never, never>;

/**
 * `system` — one area of declared behavior, `docs/systems/<name>.md`.
 * Preamble prose, a purpose section, an invariants collection (each invariant
 * an addressable member), and a `satisfies` edge section naming the
 * requirements this system fills.
 */
export const system: KindDefinition<BodyOnly> = kind<BodyOnly>({
  name: "system",
  locus: { kind: "at", root: "docs/systems", glob: "*.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
  content: {
    regions: [
      { region: "prose" },
      { region: "field", slot: "purpose" },
      { region: "collection", memberKind: "invariant" },
      { region: "field", slot: "satisfies" },
    ],
  },
});

/**
 * `flow` — behavior that crosses systems, `docs/flows/<name>.md`. Its
 * `participants` section is an edge slot: each entry is a `system:<name>`
 * address the gate resolves.
 */
export const flow: KindDefinition<BodyOnly> = kind<BodyOnly>({
  name: "flow",
  locus: { kind: "at", root: "docs/flows", glob: "*.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
  edgeFields: [{ field: "participants", to: "system" }],
  content: {
    regions: [
      { region: "prose" },
      { region: "field", slot: "trigger" },
      { region: "field", slot: "participants" },
      { region: "field", slot: "steps" },
    ],
  },
});

/**
 * `decision` — an accepted ruling, `docs/decisions/<name>.md`. Lifecycle is
 * positional, not a status field: a decision in this directory is current; a
 * superseded one moves to `docs/decisions/superseded/` and changes kind.
 * Rejected alternatives are members of the decision that rejected them.
 */
export const decision: KindDefinition<BodyOnly> = kind<BodyOnly>({
  name: "decision",
  locus: { kind: "at", root: "docs/decisions", glob: "*.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
  content: {
    regions: [
      { region: "prose" },
      { region: "field", slot: "ruling" },
      { region: "field", slot: "consequences" },
      { region: "collection", memberKind: "alternative" },
    ],
  },
});

/**
 * `superseded-decision` — a replaced ruling, `docs/decisions/superseded/<name>.md`.
 * The successor edge is unconditionally required here: the conditional
 * "`superseded_by` iff superseded" dissolves into the kind partition.
 */
export const supersededDecision: KindDefinition<BodyOnly> = kind<BodyOnly>({
  name: "superseded-decision",
  locus: { kind: "at", root: "docs/decisions/superseded", glob: "*.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
  edgeFields: [{ field: "superseded-by", to: "decision" }],
  content: {
    regions: [
      { region: "prose" },
      { region: "field", slot: "ruling" },
      { region: "field", slot: "superseded-by" },
    ],
  },
});

/**
 * `glossary` — one host document, `docs/glossary.md`, whose terms collection
 * makes each term an addressable member a mention can target.
 */
export const glossary: KindDefinition<BodyOnly> = kind<BodyOnly>({
  name: "glossary",
  locus: { kind: "at", root: "docs", glob: "glossary.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
  content: {
    regions: [{ region: "prose" }, { region: "collection", memberKind: "term" }],
  },
});
