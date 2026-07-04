/**
 * Projection writing — the projection half of emit: compile each member to its
 * harness format under `.claude/**`, whole-file and byte-faithful
 * (`specs/architecture/20-surface.md`, "Content-faithful, deterministically
 * emitted (law 5)"; the two-projectors seam). A port of the Rust emit projector
 * (`src/drift.rs` `project_bytes` / `render_field`): a member with frontmatter
 * fields projects to a fresh `---`-delimited block over its body; a fieldless
 * member projects to its body alone. The words are the author's, untouched — emit
 * never stamps metadata into the projection (law 5); the managed-by note and the
 * schema modeline ride `install`, not emit.
 *
 * Only the built-in projected kinds have a `.claude/**` locus: `rule`
 * (`.claude/rules/<name>.md`) and `skill` (`.claude/skills/<name>/SKILL.md`),
 * the two kinds the Rust projector emits (`BUILTIN_KINDS`). A member of any other
 * kind has no projection here and is a loud error — never a silently faked path.
 */

import type { ManifestMember } from "./manifest.js";

/** One projected harness file: where it lands and the byte-faithful content. */
export interface Projection {
  /** The `.claude/**` slash path, relative to the harness root. */
  readonly path: string;
  /** The whole-file projection bytes — frontmatter (if any) over the body. */
  readonly bytes: string;
}

/** The bare kind name (`claude-code.rule` → `rule`) the projection locus keys on. */
function bareKind(kind: string): string {
  const parts = kind.split(".");
  return parts[parts.length - 1];
}

/**
 * Whether a member of `kind` has a `.claude/**` projection — the built-in
 * projected kinds (`rule`, `skill`). Memory (`CLAUDE.md`) and custom kinds carry
 * no projection, so emit lands them in the manifest but writes neither a
 * projection file nor a lock fingerprint for them.
 */
export function isProjectedKind(kind: string): boolean {
  const bare = bareKind(kind);
  return bare === "rule" || bare === "skill";
}

/**
 * The `.claude/**` locus a member of `kind` named `name` projects onto — the
 * built-in projected kinds' harness paths (`specs/architecture/20-surface.md`,
 * "Artifact kinds & package binding"): a rule is a flat `.claude/rules/<name>.md`,
 * a skill a `.claude/skills/<name>/SKILL.md` under its own directory.
 *
 * # Throws
 * If the kind is not a projected built-in — memory (`CLAUDE.md`) and custom kinds
 * have no `.claude/**` projection, so a request for one is a loud error rather
 * than a guessed path.
 */
export function projectionPath(kind: string, name: string): string {
  switch (bareKind(kind)) {
    case "rule":
      return `.claude/rules/${name}.md`;
    case "skill":
      return `.claude/skills/${name}/SKILL.md`;
    default:
      throw new Error(
        `kind \`${kind}\` has no \`.claude/**\` projection — only the built-in ` +
          `\`rule\` and \`skill\` kinds are projected (specs/architecture/20-surface.md).`,
      );
  }
}

/**
 * One frontmatter field as `key: <value>\n`, or `null` to omit a null/undefined
 * value. The value is compact JSON — valid YAML flow, so it round-trips back to
 * the same JSON on the next parse — matching the Rust `render_field`
 * (`serde_json::to_string`, which `JSON.stringify` mirrors byte-for-byte for the
 * scalars, string arrays, and objects a member field carries).
 */
export function renderField(key: string, value: unknown): string | null {
  if (value === null || value === undefined) return null;
  return `${key}: ${JSON.stringify(value)}\n`;
}

/**
 * The whole-file projection bytes for one member — `project_bytes`: a member with
 * no fields projects to its body alone (no frontmatter block, so no place a
 * modeline/note could sit); one with fields projects to a fresh `---`-delimited
 * frontmatter (install's preserved `placements` first, then every field in order)
 * over the byte-faithful body. `fields` is an ordered pair list so the frontmatter
 * key order is the caller's, not an alphabetization the Rust projector never does.
 */
export function projectBytes(
  fields: ReadonlyArray<readonly [string, unknown]>,
  body: string,
  placements: readonly string[] = [],
): string {
  const rendered = fields
    .map(([key, value]) => renderField(key, value))
    .filter((line): line is string => line !== null);
  if (rendered.length === 0) return body;
  const frontmatter = placements.map((line) => `${line}\n`).join("") + rendered.join("");
  return `---\n${frontmatter}---\n${body}`;
}

/**
 * Project one manifest member onto its harness file — its `.claude/**` locus and
 * the whole-file bytes. The body is the member's resolved section body (the
 * content-faithful body emit already resolved); the fields are its frontmatter in
 * declared order. The seam the byte-parity fixtures pin against the Rust projector.
 *
 * # Throws
 * If the member's kind has no projection ([`projectionPath`]).
 */
export function projectMember(member: ManifestMember): Projection {
  const path = projectionPath(member.kind, member.name);
  const fields = Object.entries(member.fields);
  const body = member.sections[0]?.body ?? "";
  return { path, bytes: projectBytes(fields, body) };
}
