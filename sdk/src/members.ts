/**
 * Module-carried members — the altitude's authoring spelling
 * (`specs/architecture/20-surface.md`, "Member carriage"): the member is a
 * typed value in the library. Fields are typed object keys, so a malformed
 * member is a squiggle before the gate ever runs — the authoring wall
 * (`specs/architecture/10-contracts.md`, the two walls; tsc is the keystroke
 * channel, `specs/architecture/50-distribution.md`).
 */

import type { Body } from "./prose.js";
import type { ManifestGenreValue } from "./manifest.js";

/** A `satisfies` claim: the requirement it fills, with the author's rationale. */
export interface SatisfiesClaim {
  readonly rationale: string;
}

/** A demand this member publishes for another member to fill. */
export interface PublishedRequirement {
  readonly means: string;
  readonly kind: string;
  readonly required?: boolean;
}

/** One authored member, carriage-blind at the model layer. */
export interface Member {
  readonly kind: string;
  readonly name: string;
  /**
   * The authored body union — an inline mention-bearing template or a `fromFile`
   * asset. It stays **unresolved** here: the asset is read and the mentions are
   * resolution-checked at emit, against the whole harness's declared values, not
   * at authoring (`specs/architecture/20-surface.md`, "Mentions"; `emit`'s
   * `resolveBody`). Authoring holds the reference; emit resolves it.
   */
  readonly body: Body;
  readonly fields: Readonly<Record<string, unknown>>;
  readonly satisfies: Readonly<Record<string, SatisfiesClaim>>;
  readonly requirements: Readonly<Record<string, PublishedRequirement>>;
  readonly genres: readonly ManifestGenreValue[];
}

interface MemberInit {
  name: string;
  body: Body;
  /** The kind's clause fields (`paths` on a rule, `description` on a skill…). */
  fields?: Readonly<Record<string, unknown>>;
  satisfies?: Readonly<Record<string, SatisfiesClaim>>;
  requirements?: Readonly<Record<string, PublishedRequirement>>;
  genres?: readonly ManifestGenreValue[];
}

function member(kind: string, init: MemberInit): Member {
  return {
    kind,
    name: init.name,
    body: init.body,
    fields: init.fields ?? {},
    satisfies: init.satisfies ?? {},
    requirements: init.requirements ?? {},
    genres: init.genres ?? [],
  };
}

/** A Claude Code rule (`.claude/rules/<name>.md`). */
export function rule(init: MemberInit & { fields?: { paths?: readonly string[] } }): Member {
  return member("claude-code.rule", init);
}

/** A Claude Code skill (`.claude/skills/<name>/SKILL.md`). */
export function skill(init: MemberInit & { fields?: { description?: string } }): Member {
  return member("claude-code.skill", init);
}

/** A memory file (`CLAUDE.md` / `AGENTS.md`). */
export function memory(init: MemberInit): Member {
  return member("claude-code.memory", init);
}

/** A member of an author-declared custom kind. */
export function customMember(kind: string, init: MemberInit): Member {
  return member(kind, init);
}
