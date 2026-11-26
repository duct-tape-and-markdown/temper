/**
 * The version a `{ package, bin }` tool runs at — its program's lockfile's: the one the
 * `packageManager` field of the program's `package.json` names (npm's `package-lock.json`,
 * pnpm's `pnpm-lock.yaml`), read from the workspace the engine runs the program in (its
 * cwd), else from the workspace root its package manager resolves (the nearest ancestor
 * declaring a workspace, read only when it includes the program), keyed by the program's
 * path below it. The reference states it; the engine's `src/tool.rs` is the same read for
 * `check` and `explain`, and the two must agree: an absent field, or another manager,
 * refuses by name.
 *
 * Formats (external facts): `packageManager` (`<name>@<version>`, an optional `+<hash>`) —
 * https://github.com/nodejs/corepack/blob/main/README.md (retrieved 2026-11-26);
 * https://docs.npmjs.com/cli/v10/configuring-npm/package-lock-json,
 * https://github.com/pnpm/spec/blob/master/lockfile/9.0.md and https://pnpm.io/lockfile
 * (all retrieved 2026-11-25); workspace roots: https://pnpm.io/workspaces,
 * https://pnpm.io/pnpm-workspace_yaml and https://docs.npmjs.com/cli/v10/using-npm/workspaces
 * (retrieved 2026-11-26).
 */

import { existsSync, readFileSync } from "node:fs";
import { dirname, join, matchesGlob, relative, resolve, sep } from "node:path";

/** npm's lockfile. */
export const NPM_LOCKFILE = "package-lock.json";
/** pnpm's lockfile. */
export const PNPM_LOCKFILE = "pnpm-lock.yaml";
/** npm's publishable lockfile, read before `package-lock.json`: beside an npm program it refuses by name. */
const NPM_SHRINKWRAP = "npm-shrinkwrap.json";

/** The managers temper reads, by the name `packageManager` spells before its `@`. */
const MANAGER_LOCKFILES: Readonly<Record<string, string>> = { npm: NPM_LOCKFILE, pnpm: PNPM_LOCKFILE };

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
 * Read the lockfile of the program in `workspace`: its own directory's, else the one at
 * the nearest ancestor declaring a workspace, read only when that declaration includes
 * the program; `undefined` otherwise — an ancestor's lockfile no workspace of the
 * program's names locks some other project, and is never read.
 *
 * # Throws
 * When the program's `package.json` declares no `packageManager` or names a manager temper
 * does not read — a manager is never guessed — when an npm program's directory holds
 * `npm-shrinkwrap.json`, or when `package-lock.json` is not JSON.
 */
export function readLockfile(workspace: string): Lockfile | undefined {
  const program = resolve(workspace);
  const lockfile = declaredLockfile(program);
  const own = readLockfileIn(program, "", lockfile);
  if (own !== undefined) return own;
  for (let dir = dirname(program); ; dir = dirname(dir)) {
    const globs = workspaceGlobs(dir);
    if (globs !== undefined) {
      const importer = relative(dir, program).split(sep).join("/");
      return includes(globs, importer) ? readLockfileIn(dir, importer, lockfile) : undefined;
    }
    if (dirname(dir) === dir) return undefined;
  }
}

/** The lockfile of the manager the `packageManager` field of `program`'s `package.json` names. */
function declaredLockfile(program: string): string {
  const path = join(program, "package.json");
  const text = readText(path);
  let declared: Json | undefined;
  if (text !== undefined) {
    try {
      declared = field(JSON.parse(text) as Json, "packageManager");
    } catch (error) {
      throw new Error(`\`${path}\` does not parse: ${String(error)}`);
    }
  }
  if (typeof declared !== "string") {
    throw new Error(
      `\`${path}\` declares no \`packageManager\`, so the program's package manager is unknown — temper never guesses it. ` +
        "Add `\"packageManager\": \"npm@<version>\"` (or `pnpm@<version>`) to it, naming the manager that installs the program's packages.",
    );
  }
  const lockfile = MANAGER_LOCKFILES[declared.split("@")[0] ?? ""];
  if (lockfile === undefined) {
    throw new Error(
      `\`${path}\` declares the package manager \`${declared}\`, which temper does not read — ` +
        "a `{ package, bin }` tool's version is read from npm's or pnpm's lockfile.",
    );
  }
  return lockfile;
}

/** pnpm's workspace declaration, beside the root's lockfile. */
const PNPM_WORKSPACE = "pnpm-workspace.yaml";

/** `path`'s text, `undefined` when it cannot be read. */
function readText(path: string): string | undefined {
  try {
    return readFileSync(path, "utf8");
  } catch {
    return undefined;
  }
}

/**
 * The workspace globs `dir` declares, when it declares a workspace at all: pnpm's
 * `pnpm-workspace.yaml` `packages` (absent, the root alone), then `package.json`'s
 * `workspaces` array, npm's. A declaration that cannot be read or parsed declares nothing.
 */
function workspaceGlobs(dir: string): string[] | undefined {
  let globs: string[] | undefined;
  const pnpm = readText(join(dir, PNPM_WORKSPACE));
  if (pnpm !== undefined) globs = pnpmPackages(pnpm);
  const npm = readText(join(dir, "package.json"));
  if (npm !== undefined) {
    let manifest: Json | undefined;
    try {
      manifest = JSON.parse(npm) as Json;
    } catch {
      manifest = undefined;
    }
    const workspaces = field(manifest, "workspaces");
    if (Array.isArray(workspaces)) {
      globs = [...(globs ?? []), ...workspaces.filter((item): item is string => typeof item === "string")];
    }
  }
  return globs;
}

/**
 * The top-level `packages` sequence of a `pnpm-workspace.yaml`, block (`- glob` lines) or
 * flow (`[a, b]`) style — the shapes pnpm's docs write it in.
 */
function pnpmPackages(text: string): string[] {
  const lines = text.split(/\r?\n/);
  const at = lines.findIndex((line) => /^packages:(\s|$)/.test(line));
  if (at < 0) return [];
  const rest = (lines[at] ?? "").slice("packages:".length).replace(/\s+#.*$/, "").trim();
  if (rest.startsWith("[") && rest.endsWith("]")) {
    return rest
      .slice(1, -1)
      .split(",")
      .map(unquote)
      .filter((glob) => glob !== "");
  }
  const globs: string[] = [];
  for (const line of lines.slice(at + 1)) {
    const body = line.trim();
    if (body === "" || body.startsWith("#")) continue;
    if (line.length === line.trimStart().length && !body.startsWith("-")) break;
    if (body.startsWith("-")) globs.push(unquote(body.slice(1).replace(/\s+#.*$/, "")));
  }
  return globs;
}

/** Whether `globs` include the project at `path`: a positive one matches, no `!` one does. */
function includes(globs: readonly string[], path: string): boolean {
  const matches = (glob: string) => matchesGlob(path, glob.replace(/^(\.\/)+/, "").replace(/\/+$/, ""));
  const negated = globs.filter((glob) => glob.startsWith("!")).map((glob) => glob.slice(1));
  return globs.some((glob) => !glob.startsWith("!") && matches(glob)) && !negated.some(matches);
}

/** The lockfile named `lockfile` in `dir` alone, for the program at `importer` below it. */
function readLockfileIn(dir: string, importer: string, lockfile: string): Lockfile | undefined {
  if (lockfile === NPM_LOCKFILE && existsSync(join(dir, NPM_SHRINKWRAP))) {
    throw new Error(
      `the program's packages are locked by \`${join(dir, NPM_SHRINKWRAP)}\`, which temper does not read — ` +
        "a `{ package, bin }` tool's version is read from `package-lock.json` or `pnpm-lock.yaml`.",
    );
  }
  const path = join(dir, lockfile);
  if (!existsSync(path)) return undefined;
  const text = readFileSync(path, "utf8");
  if (lockfile === NPM_LOCKFILE) {
    const document = JSON.parse(text) as Json;
    return { name: NPM_LOCKFILE, version: (pkg) => npmVersion(document, importer, pkg) };
  }
  return { name: PNPM_LOCKFILE, version: (pkg) => pnpmVersion(text, importer, pkg) };
}
