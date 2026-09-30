/**
 * scripts/directInvocation.mjs — "was this module the one invoked?", for the
 * scripts in this directory.
 *
 * Compare the caller's `import.meta.url` against `process.argv[1]` resolved
 * through symlinks, so importing a script for its named exports runs no side
 * effect — only running it as a script does. Node resolves the main entry's
 * URL through links while `argv[1]` keeps the invoked path verbatim, so the
 * resolve is what lets a checkout reached through a link still match.
 */

import { realpathSync } from "node:fs";
import { fileURLToPath } from "node:url";

export function isDirectInvocation(moduleUrl) {
  const invoked = process.argv[1];
  if (invoked === undefined) return false;
  try {
    return realpathSync(fileURLToPath(moduleUrl)) === realpathSync(invoked);
  } catch {
    return false;
  }
}
