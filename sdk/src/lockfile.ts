/**
 * The version a `{ package, bin }` tool runs at — its program's lockfile's, read from the
 * nearest directory at or above the workspace the engine runs the program in (its cwd)
 * that carries one, keyed by the program's path below it. The reference states it; the
 * engine's `src/tool.rs` is the same read for `check` and `explain`, and the two must
 * agree: npm's `package-lock.json`, else pnpm's `pnpm-lock.yaml` (every YAML document in
 * it), and any other manager's lockfile refused by name.
 *
 * Formats (external facts): https://docs.npmjs.com/cli/v10/configuring-npm/package-lock-json,
 * https://github.com/pnpm/spec/blob/master/lockfile/9.0.md and https://pnpm.io/lockfile
 * (all retrieved 2026-11-25).
 */

import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";

/** npm's lockfile. */
export const NPM_LOCKFILE = "package-lock.json";
/** pnpm's lockfile. */
export const PNPM_LOCKFILE = "pnpm-lock.yaml";
/** The other managers' lockfiles: alone in the workspace, they refuse by name. */
const UNREAD_LOCKFILES = ["yarn.lock", "bun.lock", "bun.lockb", "npm-shrinkwrap.json"] as const;

/** A read lockfile: its file name, and the version it records for a direct dependency. */
export interface Lockfile {
  readonly name: string;
  version(pkg: string): string | undefined;
}

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };

function field(value: Json | undefined, key: string): Json | undefined {
  return value !== null && typeof value === "object" && !Array.isArray(value) ? value[key] : undefined;
}

function str(value: Json | undefined): string | undefined {
  return typeof value === "string" ? value : undefined;
}

/**
 * `pkg`'s version in an npm lockfile: v2/v3's `packages` — the importer's own
 * `node_modules` before the hoisted root one, following a link — else v1's `dependencies`.
 */
function npmVersion(document: Json, importer: string, pkg: string): string | undefined {
  const packages = field(document, "packages");
  if (packages !== undefined) {
    const hoisted = `node_modules/${pkg}`;
    const entry = (importer === "" ? undefined : field(packages, `${importer}/${hoisted}`)) ?? field(packages, hoisted);
    const version = str(field(entry, "version"));
    if (version !== undefined) return version;
    const resolved = field(entry, "link") === true ? str(field(entry, "resolved")) : undefined;
    return resolved === undefined ? undefined : str(field(field(packages, resolved), "version"));
  }
  return str(field(field(field(document, "dependencies"), pkg), "version"));
}

/** A YAML scalar as pnpm writes it: bare, or single- or double-quoted. */
function unquote(raw: string): string {
  const text = raw.trim();
  if (text.length >= 2 && (text[0] === "'" || text[0] === '"') && text.at(-1) === text[0]) {
    return text.slice(1, -1).replaceAll(text[0] === "'" ? "''" : '\\"', text[0] === "'" ? "'" : '"');
  }
  return text;
}

/**
 * `pkg`'s version in a pnpm lockfile: the first of its documents — split at their `---`
 * marker lines, a leading boot (env) document before the project's — that records it.
 */
function pnpmVersion(text: string, importer: string, pkg: string): string | undefined {
  const documents: string[][] = [[]];
  for (const line of text.split(/\r?\n/)) {
    if (line.trimEnd() === "---") documents.push([]);
    else documents.at(-1)?.push(line);
  }
  for (const document of documents) {
    const version = pnpmDocumentVersion(document, importer, pkg);
    if (version !== undefined) return version;
  }
  return undefined;
}

/**
 * `pkg`'s version in one pnpm lockfile document — the importer's dependency groups (v6+),
 * else the top-level groups (v5). pnpm writes block-style YAML at two-space indents, so
 * the read walks the indentation of exactly that path rather than parsing YAML in general.
 */
function pnpmDocumentVersion(document: string[], importer: string, pkg: string): string | undefined {
  const lines = document.filter((line) => line.trim() !== "" && !line.trimStart().startsWith("#"));
  const entries = lines.map((line) => {
    const indent = line.length - line.trimStart().length;
    const body = line.trim();
    const colon = body.match(/^('(?:[^']|'')*'|"(?:[^"\\]|\\.)*"|[^:]+):(?:\s+(.*))?$/);
    return colon === null
      ? { indent, key: undefined, value: undefined }
      : { indent, key: unquote(colon[1] ?? ""), value: colon[2] === undefined ? undefined : unquote(colon[2]) };
  });
  /** The children of the mapping entry at `at`, as indices. */
  const children = (at: number): number[] => {
    const parent = at < 0 ? -1 : (entries[at]?.indent ?? 0);
    const out: number[] = [];
    let depth: number | undefined;
    for (let i = at + 1; i < entries.length; i++) {
      const indent = entries[i]?.indent ?? 0;
      if (indent <= parent) break;
      depth ??= indent;
      if (indent === depth) out.push(i);
    }
    return out;
  };
  const child = (at: number, key: string) => children(at).find((i) => entries[i]?.key === key);
  const importers = child(-1, "importers");
  const root = importers === undefined ? -1 : child(importers, importer === "" ? "." : importer);
  if (root === undefined) return undefined;
  for (const group of ["dependencies", "devDependencies", "optionalDependencies"]) {
    const at = child(root, group);
    const entry = at === undefined ? undefined : child(at, pkg);
    if (entry === undefined) continue;
    const bare = entries[entry]?.value;
    const versionAt = bare === undefined ? child(entry, "version") : undefined;
    const version = bare ?? (versionAt === undefined ? undefined : entries[versionAt]?.value);
    if (version !== undefined) return version.split("(")[0];
  }
  return undefined;
}

/**
 * Read the lockfile of the program in `workspace`, from the nearest directory at or above
 * it carrying one; `undefined` when none up to the filesystem root does.
 *
 * # Throws
 * When that directory holds only another manager's lockfile, naming it — a version
 * temper cannot read is never guessed — or when `package-lock.json` is not JSON.
 */
export function readLockfile(workspace: string): Lockfile | undefined {
  const program = resolve(workspace);
  for (let dir = program; ; dir = dirname(dir)) {
    const lockfile = readLockfileIn(dir, relative(dir, program).split(sep).join("/"));
    if (lockfile !== undefined) return lockfile;
    if (dirname(dir) === dir) return undefined;
  }
}

/** The lockfile in `dir` alone, for the program at `importer` below it. */
function readLockfileIn(dir: string, importer: string): Lockfile | undefined {
  const npm = join(dir, NPM_LOCKFILE);
  if (existsSync(npm)) {
    const document = JSON.parse(readFileSync(npm, "utf8")) as Json;
    return { name: NPM_LOCKFILE, version: (pkg) => npmVersion(document, importer, pkg) };
  }
  const pnpm = join(dir, PNPM_LOCKFILE);
  if (existsSync(pnpm)) {
    const text = readFileSync(pnpm, "utf8");
    return { name: PNPM_LOCKFILE, version: (pkg) => pnpmVersion(text, importer, pkg) };
  }
  const other = UNREAD_LOCKFILES.find((name) => existsSync(join(dir, name)));
  if (other !== undefined) {
    throw new Error(
      `the program's packages are locked by \`${join(dir, other)}\`, which temper does not read — ` +
        "a `{ package, bin }` tool's version is read from `package-lock.json` or `pnpm-lock.yaml`.",
    );
  }
  return undefined;
}
