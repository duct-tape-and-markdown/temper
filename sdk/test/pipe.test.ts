/**
 * The one pipe — a program's module default-exports its harness, and the pipe reads that
 * export, compiles it and prints its seam. The program writes no plumbing of its own.
 */
import assert from "node:assert/strict";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { fileURLToPath, pathToFileURL } from "node:url";

import { rule } from "../src/claude-code.js";
import { emit } from "../src/emit.js";
import { harness, text } from "../src/index.js";
import { programSeam } from "../src/pipe.js";

const DIST_SRC = resolve(dirname(fileURLToPath(import.meta.url)), "../src");

/** A scratch program module at `<tmp>/harness.mjs`, importing the built SDK by URL. */
function withProgram(body: string, run: (entry: string) => Promise<void>): Promise<void> {
  const dir = mkdtempSync(join(tmpdir(), "temper-pipe-"));
  const entry = join(dir, "harness.mjs");
  const sdk = (module: string): string => pathToFileURL(join(DIST_SRC, module)).href;
  writeFileSync(
    entry,
    `import { harness, text } from ${JSON.stringify(sdk("index.js"))};\n` +
      `import { rule } from ${JSON.stringify(sdk("claude-code.js"))};\n` +
      body,
  );
  return run(entry).finally(() => rmSync(dir, { recursive: true, force: true }));
}

test("a program that default-exports its harness compiles to that harness's seam", async () => {
  await withProgram(
    "export default harness({ members: [rule({ name: \"rust\", prose: text`# Rust` })] });\n",
    async (entry) => {
      const expected = emit(harness({ members: [rule({ name: "rust", prose: text`# Rust` })] })).seam;
      assert.equal(await programSeam(entry), expected);
    },
  );
});

test("a program with no default export refuses, naming the export it owes", async () => {
  await withProgram("export const program = harness({ members: [] });\n", async (entry) => {
    await assert.rejects(programSeam(entry), /exports no harness as its default \(found no default export\)/);
  });
});

test("a default export that is not a harness refuses", async () => {
  await withProgram("export default { members: 'none' };\n", async (entry) => {
    await assert.rejects(programSeam(entry), /exports no harness as its default \(found a default export of type object\)/);
  });
});
