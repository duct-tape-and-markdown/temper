/**
 * Projection — emit compiles each member to its harness format under `.claude/**`,
 * whole-file and byte-faithful (`specs/architecture/20-surface.md`, "Emit —
 * total"). A member with frontmatter fields projects to a fresh `---`-delimited
 * block over its resolved body; a frontmatterless kind (memory) projects to its
 * body alone. The words are the author's, untouched — emit never stamps metadata
 * into the projection (law 5); the managed-by note and the schema modeline ride
 * `install`, and a re-emit round-trips them through the whole-file write.
 *
 * The locus and layout come from the kind's five facts (`15-kinds.md`), never a
 * hardcoded kind name: a directory unit lands its entry file under a per-member
 * directory, a lone file lands at the stem, a frontmatterless any-depth memory
 * lands the root `<name>.md`.
 */

import { readFileSync } from "node:fs";
import { join } from "node:path";

import type { KindFacts } from "./kind.js";

/** One projected harness file: where it lands and the byte-faithful content. */
export interface Projection {
  /** The slash path relative to the harness root (`.claude/**`, or a root memory). */
  readonly path: string;
  /** The whole-file projection bytes — frontmatter (if any) over the body. */
  readonly bytes: string;
}

/** The resolved member emit hands the projector — facts, name, ordered fields, resolved body. */
export interface ProjectionInput {
  readonly facts: KindFacts;
  readonly name: string;
  readonly fields: ReadonlyArray<readonly [string, unknown]>;
  readonly body: string;
}

/** Emit-time inputs beyond the member — where install's placements are read from. */
export interface ProjectOptions {
  /**
   * The harness root the **committed** projection is read from to carry install's
   * frontmatter placements (the schema modeline + managed-by note) through the
   * whole-file re-emit — the two-projectors seam. Absent — or an absent committed
   * file — carries no placements: emit writes the projection fresh.
   */
  readonly projectionDir?: string;
}

/** The schema modeline marker install places and emit preserves (`src/install.rs`). */
const MODELINE_MARKER = "# yaml-language-server:";
/** The managed-by note's stable marker (`src/install.rs`). */
const NOTE_MARKER = "# temper: managed projection";

/** Whether `line` is one of install's managed metadata comments. */
function isPlacementComment(line: string): boolean {
  const trimmed = line.replace(/^\s+/, "");
  return trimmed.startsWith(MODELINE_MARKER) || trimmed.startsWith(NOTE_MARKER);
}

/**
 * A string's lines the Rust `str::lines` way: split on `\n`, a trailing newline
 * opens no line, a trailing `\r` is stripped from each.
 */
function lines(textValue: string): string[] {
  if (textValue === "") return [];
  const parts = textValue.split("\n");
  if (parts[parts.length - 1] === "") parts.pop();
  return parts.map((line) => (line.endsWith("\r") ? line.slice(0, -1) : line));
}

/**
 * The frontmatter interior of `rest` — everything after the opening `---\n` up to
 * the closing `---` line — or `null` when there is no closing delimiter (an
 * opening `---` that is really prose). A port of the Rust `install::frontmatter_inner`.
 */
function frontmatterInner(rest: string): string | null {
  let offset = 0;
  let cursor = 0;
  while (cursor < rest.length) {
    const newline = rest.indexOf("\n", cursor);
    const end = newline === -1 ? rest.length : newline + 1;
    const piece = rest.slice(cursor, end);
    const content = piece.endsWith("\n") ? piece.slice(0, -1) : piece;
    if (content.replace(/\s+$/, "") === "---") return rest.slice(0, offset);
    offset += piece.length;
    cursor = end;
  }
  return null;
}

/**
 * The install-placed frontmatter comment lines present in `source`, in on-disk
 * order — the schema modeline and the managed-by note. `emit` round-trips these
 * through its whole-file re-emit so its content-faithful projection (law 5)
 * carries install's metadata instead of dropping it (`20-surface.md`).
 */
export function placementLines(source: string): string[] {
  if (!source.startsWith("---\n")) return [];
  const inner = frontmatterInner(source.slice("---\n".length));
  if (inner === null) return [];
  return lines(inner).filter(isPlacementComment);
}

/** Join non-empty, non-`.` path segments with `/` — a `.` root drops out (root memory). */
function joinSlash(...parts: string[]): string {
  return parts.filter((part) => part !== "" && part !== ".").join("/");
}

/**
 * The harness locus a member of `facts` named `name` projects onto, derived from
 * the kind's locus and unit shape (`15-kinds.md`): a directory unit lands its
 * entry file under `<root>/<name>/`; a lone file replaces the glob's `*` with the
 * name (an any-depth memory lands the root `<name>.md`).
 *
 * # Throws
 * If the kind is a genre — a block-locus member has no standalone projection.
 */
export function projectionPath(facts: KindFacts, name: string): string {
  if (facts.locus.kind !== "at") {
    throw new Error(
      `kind \`${facts.name}\` is a genre — its members live inside host documents ` +
        `and have no standalone projection (specs/architecture/15-kinds.md).`,
    );
  }
  const { root, glob } = facts.locus;
  if (facts.unitShape === "directory") {
    // `*/SKILL.md` → the entry file after the first slash, under a per-member dir.
    const entry = glob.slice(glob.indexOf("/") + 1);
    return joinSlash(root, name, entry);
  }
  // A lone file: any-depth glob (`**/CLAUDE.md`) is the root `<name>.md`; a simple
  // glob (`*.md`) replaces its single star with the name.
  const filename = glob.includes("**") ? `${name}.md` : glob.replace("*", name);
  return joinSlash(root, filename);
}

/**
 * One frontmatter field as `key: <value>\n`, or `null` to omit a null/undefined
 * value. The value is compact JSON — valid YAML flow, round-tripping to the same
 * JSON on the next parse — matching the Rust `render_field` (`serde_json::to_string`).
 */
export function renderField(key: string, value: unknown): string | null {
  if (value === null || value === undefined) return null;
  return `${key}: ${JSON.stringify(value)}\n`;
}

/**
 * The whole-file projection bytes for one member: no surviving field ⇒ the body
 * alone (no frontmatter block, so no place a modeline/note could sit); one or
 * more ⇒ a fresh `---` frontmatter (install's preserved `placements` first, then
 * every field in order) over the byte-faithful body.
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
 * Read the install-placed frontmatter lines from the committed projection at
 * `projectionDir/path`, or `[]` when none is read (no `projectionDir`, or the file
 * is absent — emit writes it fresh). Reads are of committed bytes, never a clock,
 * so the double-emit purity check still holds.
 *
 * # Throws
 * On a read failure that is not "file absent".
 */
function committedPlacements(projectionDir: string | undefined, path: string): string[] {
  if (projectionDir === undefined) return [];
  try {
    return placementLines(readFileSync(join(projectionDir, path), "utf8"));
  } catch (cause) {
    if ((cause as NodeJS.ErrnoException).code === "ENOENT") return [];
    throw new Error(`failed to read committed projection \`${path}\` under \`${projectionDir}\`.`, {
      cause,
    });
  }
}

/**
 * Project one resolved member onto its harness file — its locus and the whole-file
 * bytes. With `options.projectionDir` set, install's placement lines ride through
 * the re-emit (the two-projectors seam).
 *
 * # Throws
 * If the member's kind is a genre ([`projectionPath`]), or the committed projection
 * cannot be read for a reason other than absence.
 */
export function projectMember(member: ProjectionInput, options: ProjectOptions = {}): Projection {
  const path = projectionPath(member.facts, member.name);
  const placements = committedPlacements(options.projectionDir, path);
  return { path, bytes: projectBytes(member.fields, member.body, placements) };
}
