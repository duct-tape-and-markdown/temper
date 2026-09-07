// Plan-prompt input: the bill of gate-reverted attempts — each entry build
// tried, the gate that reverted it, and the exact paths its commit needed
// outside files[]. Records live in the PRIMARY checkout's runtime dir
// (.flume/prior-attempts, gitignored), never in the worktree plan renders
// in, so resolve the primary via git's common dir. Never fails the render:
// an unreadable record set prints as unavailable.
import { readdirSync, readFileSync } from "node:fs";
import { execSync } from "node:child_process";
import { join, dirname } from "node:path";
let dir = ".flume/prior-attempts";
try {
  const common = execSync("git rev-parse --git-common-dir", { encoding: "utf8" }).trim();
  dir = join(dirname(common.endsWith("/.git") || common === ".git" ? (common === ".git" ? process.cwd() + "/.git" : common) : common + "/x"), ".flume", "prior-attempts");
} catch {}
let out = "";
try {
  for (const f of readdirSync(dir).filter((f) => f.endsWith(".json")).sort()) {
    try {
      const d = JSON.parse(readFileSync(join(dir, f), "utf8"));
      if (d.mode !== "gate-revert") continue;
      out += `== ${f.replace(/\.json$/, "").toUpperCase()} — ${d.gate} @ ${d.at ?? ""}\n   ${d.message}\n`;
      if (d.details) out += d.details.split("\n").map((l) => "   " + l).join("\n") + "\n";
    } catch {}
  }
} catch {
  out = `(gate-revert records unavailable at ${dir})\n`;
}
process.stdout.write(out || "(no gate-reverted attempts on record)\n");
