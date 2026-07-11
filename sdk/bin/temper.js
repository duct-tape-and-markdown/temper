#!/usr/bin/env node
/**
 * The engine launcher — channel 2's npm face (`specs/distribution.md`,
 * "What ships"): resolve the platform's prebuilt engine binary from its
 * `optionalDependencies` package and exec it. Fail-loud invariant: a
 * missing platform binary is an install error with instructions, never a
 * silent skip — if it cannot check, it fails loud.
 */
import { createRequire } from "node:module";
import { spawnSync } from "node:child_process";

const require = createRequire(import.meta.url);

/** platform+arch → [platform package, binary path inside it]. */
const PLATFORMS = {
  "linux x64": ["@dtmd/temper-linux-x64", "bin/temper"],
  "win32 x64": ["@dtmd/temper-win32-x64", "bin/temper.exe"],
};

const key = `${process.platform} ${process.arch}`;
const entry = PLATFORMS[key];

if (!entry) {
  const supported = Object.keys(PLATFORMS).join(", ");
  process.stderr.write(
    `temper: no prebuilt engine binary for ${key} yet (prebuilt: ${supported}).\n` +
      `Build from source with a Rust 1.96+ toolchain instead:\n` +
      `  cargo install --git https://github.com/duct-tape-and-markdown/temper\n`,
  );
  process.exit(1);
}

const [pkg, binPath] = entry;
let bin;
try {
  bin = require.resolve(`${pkg}/${binPath}`);
} catch {
  process.stderr.write(
    `temper: the platform engine package ${pkg} is not installed.\n` +
      `It ships as an optionalDependency of @dtmd/temper — an installer run\n` +
      `with optional dependencies disabled skips it. Restore it with:\n` +
      `  npm install ${pkg}\n`,
  );
  process.exit(1);
}

const result = spawnSync(bin, process.argv.slice(2), { stdio: "inherit" });
if (result.error) {
  process.stderr.write(`temper: failed to run ${bin}: ${result.error.message}\n`);
  process.exit(1);
}
process.exit(result.status ?? 1);
