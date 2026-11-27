/**
 * The `.bin` launcher (`sdk/bin/temper.js`) under the fail-loud invariant:
 * every path that cannot reach the engine exits 2 — the hook runtime's
 * blocking exit — with install instructions on stderr, and an engine that
 * ran passes its own status through.
 *
 * Each case copies the launcher into a fresh temp directory so the platform
 * package resolves only from what the case seats beside it, never from the
 * SDK's own `node_modules`.
 */

import { test } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, copyFileSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const LAUNCHER = resolve(dirname(fileURLToPath(import.meta.url)), "../../bin/temper.js");

/** Only the linux key seats a runnable fake engine (a shell script). */
const LINUX_X64 = process.platform === "linux" && process.arch === "x64";

/**
 * Run a copy of the launcher from a temp root holding `package.json` (so the
 * copy loads as ESM) and whatever `seat` places, with `nodeArgs` before it.
 */
function launch(seat: (root: string) => void = () => {}, nodeArgs: string[] = []) {
  const root = mkdtempSync(join(tmpdir(), "temper-launcher-"));
  try {
    writeFileSync(join(root, "package.json"), '{ "type": "module" }\n');
    mkdirSync(join(root, "bin"));
    copyFileSync(LAUNCHER, join(root, "bin", "temper.js"));
    seat(root);
    const { NODE_PATH: _, ...env } = process.env;
    return spawnSync(process.execPath, [...nodeArgs, join(root, "bin", "temper.js"), "check"], {
      encoding: "utf8",
      env,
    });
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

/** Seat a fake linux-x64 platform package whose engine is `script`. */
function seatEngine(script: string, mode: number) {
  return (root: string) => {
    const pkg = join(root, "node_modules", "@dtmd", "temper-linux-x64");
    mkdirSync(join(pkg, "bin"), { recursive: true });
    writeFileSync(join(pkg, "package.json"), '{ "name": "@dtmd/temper-linux-x64" }\n');
    writeFileSync(join(pkg, "bin", "temper"), script);
    chmodSync(join(pkg, "bin", "temper"), mode);
  };
}

test("a missing platform package exits 2 with the install instructions on stderr", () => {
  const run = launch();
  assert.equal(run.status, 2, run.stderr);
  assert.match(run.stderr, /is not installed/);
  assert.match(run.stderr, /npm install @dtmd\/temper-/);
});

test("an unsupported platform exits 2 with the build-from-source instructions on stderr", () => {
  const spoof = 'data:text/javascript,Object.defineProperty(process,"platform",{value:"aix"})';
  const run = launch(undefined, ["--import", spoof]);
  assert.equal(run.status, 2, run.stderr);
  assert.match(run.stderr, /no prebuilt engine binary for aix/);
  assert.match(run.stderr, /cargo install --git /);
});

test("an engine binary that fails to spawn exits 2 with the reinstall instructions on stderr", {
  skip: !LINUX_X64 && "the fake engine is seated only under the linux-x64 key",
}, () => {
  const run = launch(seatEngine("#!/bin/sh\nexit 0\n", 0o644));
  assert.equal(run.status, 2, run.stderr);
  assert.match(run.stderr, /failed to run/);
  assert.match(run.stderr, /npm install @dtmd\/temper-linux-x64/);
});

test("an engine that ran passes its own exit status through", {
  skip: !LINUX_X64 && "the fake engine is seated only under the linux-x64 key",
}, () => {
  const run = launch(seatEngine("#!/bin/sh\nexit 1\n", 0o755));
  assert.equal(run.status, 1, run.stderr);
  assert.equal(run.stderr, "");
});
