#!/usr/bin/env node
/**
 * The cargo validator the harness package's `scriptRunner` drives
 * (`declaration.ts`, `runner`). Invoked once per judge operation, in the tree
 * under judgment, with each requested `tests[]` line as an argument; answers
 * every line on stdout in the package's verdict-line encoding
 * (`flume<TAB>pass<TAB><file><TAB><name>` / `flume<TAB>fail<TAB><TAB><name>`).
 *
 * A line names a Rust test by its words (PROTOCOL.md, "Tests"). A test
 * function carries it when either holds:
 * - its name, letters and digits only, contains the line's letters and digits
 *   (`a layout's frontmatter reads` is carried by `a_layouts_frontmatter_reads`);
 * - the line's words appear in order among the name's words, so a name may
 *   spell out punctuation an identifier cannot hold (`` a `.`-rooted locus ``
 *   is carried by `a_dot_rooted_locus`).
 *
 * Runs nextest filtered to the tests that could carry a requested line — a
 * pre-filter the answer never depends on: a test the filter misses answers
 * `fail`, which the judge reports loud, never a false `pass`.
 */
import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

/** A line or a name reduced to the letters and digits the match compares. */
export const squash = (text) => text.toLowerCase().replace(/[^a-z0-9]+/g, "");

/** A line or a name split into its lowercase words. */
export const words = (text) => text.toLowerCase().split(/[^a-z0-9]+/).filter(Boolean);

/** True when every word of `want` appears in `have`, in order, gaps allowed. */
export const inOrder = (want, have) => {
  let at = 0;
  for (const word of have) if (at < want.length && word === want[at]) at++;
  return want.length > 0 && at === want.length;
};

/** Whether a test function name carries a requested line. */
export const carries = (fnName, line) =>
  (squash(line).length > 0 && squash(fnName).includes(squash(line))) || inOrder(words(line), words(fnName));

/**
 * A nextest filter regex over-approximating `carries`: the line's words in
 * order with anything between. The verdict, never the filter, decides.
 */
export const lineRegex = (line) => words(line).join(".*");

/**
 * Split a libtest-json name (`temper::it$acceptance::a_case`) into its binary
 * id and module path. A name with no `$` is a library test of the crate root.
 */
export const parseName = (full) => {
  const dollar = full.indexOf("$");
  if (dollar < 0) return { binary: full.split("::")[0], path: full.split("::").slice(1) };
  return { binary: full.slice(0, dollar), path: full.slice(dollar + 1).split("::") };
};

/**
 * The run-relative file holding a test, from its binary and module path. The
 * crate is flat (`specs/process/architecture.md`): a lib test's first module
 * segment is `src/<seg>.rs`, the integration binary `it` is `tests/it/`, and
 * anything unresolvable falls back to the binary's root file.
 */
export const fileOf = (binary, path, exists = existsSync, root = ".") => {
  const mods = path.slice(0, -1);
  const isIt = /(^|::)it$/.test(binary);
  const base = isIt ? "tests/it" : "src";
  const rootFile = isIt ? "tests/it/main.rs" : binary.includes("bin/") ? "src/main.rs" : "src/lib.rs";
  for (let depth = mods.length; depth >= 1; depth--) {
    const rel = mods.slice(0, depth).filter((m) => m !== "tests").join("/");
    if (rel.length === 0) continue;
    for (const candidate of [`${base}/${rel}.rs`, `${base}/${rel}/mod.rs`]) {
      if (exists(join(root, candidate))) return candidate;
    }
  }
  return rootFile;
};

/** One test event stream (libtest-json lines) → passing and ignored test names. */
export const readEvents = (stdout) => {
  const passed = [];
  const ignored = [];
  for (const line of stdout.split("\n")) {
    if (!line.startsWith("{")) continue;
    let event;
    try {
      event = JSON.parse(line);
    } catch {
      continue;
    }
    if (event.type !== "test") continue;
    if (event.event === "ok") passed.push(event.name);
    if (event.event === "ignored") ignored.push(event.name);
  }
  return { passed, ignored };
};

/** Answer each requested line from the passing test names. */
export const verdicts = (lines, passed, exists = existsSync) =>
  lines.map((line) => {
    const carrier = passed.find((full) => carries(parseName(full).path.at(-1) ?? "", line));
    if (!carrier) return `flume\tfail\t\t${line}`;
    const { binary, path } = parseName(carrier);
    return `flume\tpass\t${fileOf(binary, path, exists)}\t${line}`;
  });

const main = (lines) => {
  if (lines.length === 0) return 0;
  const filter = lines.map((l) => `test(/${lineRegex(l)}/)`).join(" | ");
  const run = spawnSync(
    "cargo",
    ["nextest", "run", "--no-fail-fast", "--no-tests=warn", "--test-threads", "4", "--message-format", "libtest-json-plus", "-E", filter],
    {
      encoding: "utf8",
      maxBuffer: 256 * 1024 * 1024,
      stdio: ["ignore", "pipe", "inherit"],
      env: { CARGO_BUILD_JOBS: "4", ...process.env, NEXTEST_EXPERIMENTAL_LIBTEST_JSON: "1" },
    },
  );
  if (run.error) {
    process.stderr.write(`runner: cargo nextest did not start: ${run.error.message}\n`);
    return 1;
  }
  const { passed } = readEvents(run.stdout ?? "");
  process.stdout.write(`${verdicts(lines, passed).join("\n")}\n`);
  return 0;
};

if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) {
  process.exitCode = main(process.argv.slice(2));
}
