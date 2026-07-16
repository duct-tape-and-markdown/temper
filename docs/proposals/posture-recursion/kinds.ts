// The house posture vocabulary — configuration, not engine. Each posture
// is a type extending member: the property shape carries what the role
// needs, the render template says how the posture speaks, and the engine
// resolves it all without understanding a single name here.
//
// Host-agnostic (ruled 07-16): no withinHosts — a posture type is declared
// once and admitted wherever a host kind's layout says it may appear.
import { kind } from "@dtmd/temper";
import type { KindDefinition, Member, Prose } from "@dtmd/temper";

/** Context that orients — budgeted push posture; carries no imperative. */
export const orientation = kind<{ prose: Prose }>({
  name: "orientation",
  locus: "embeddable",
});

/** A binding statement — what holds and what breaks. The only posture
 * allowed imperative force; short by contract, not by convention. */
export const directive = kind<{ prose: Prose }>({
  name: "directive",
  locus: "embeddable",
});

/** One action of a procedure, in execution order. */
export const step = kind<{ prose: Prose }>({
  name: "step",
  locus: "embeddable",
});

/**
 * A citation of an invocable skill. The render template speaks the way an
 * agent acts on a skill: by name. `{cite.name}` is an engine-derived fact;
 * a rendered citation is true by construction.
 */
export const consult = kind<{ prose: Prose; cite: Member }>({
  name: "consult",
  locus: "embeddable",
  edgeFields: [{ field: "cite", to: "skill" }],
  render: "{prose}: the {cite.name} skill.",
});

/**
 * A citation of a path-addressed member (a supporting doc, a rule). The
 * render template speaks the way an agent acts on a file: by its
 * projection path, derived relative to the citing member's own projection.
 */
export const reference = kind<{ prose: Prose; cite: Member }>({
  name: "reference",
  locus: "embeddable",
  edgeFields: [{ field: "cite" }], // target kind open; resolution corpus-wide
  render: "{prose}: {cite.path}.",
});

/**
 * Supporting docs per skill package — unchanged from the live program;
 * the nested-root kind is sanctioned machinery today and collapses into
 * whatever native skill-package form the nesting work takes.
 */
export interface SupportingDoc {
  readonly prose?: Prose;
}
const supportingDocs = (skillName: string): KindDefinition<SupportingDoc> =>
  kind<SupportingDoc>({
    name: `${skillName}-doc`,
    locus: { kind: "at", root: `.claude/skills/${skillName}/docs`, glob: "*.md" },
    unitShape: "file",
    registration: [],
  });

export const harnessMetaDoc = supportingDocs("harness-meta");
export const platformDoc = supportingDocs("platform");
