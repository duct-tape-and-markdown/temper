// Plan-prompt input: the sizing digest — one line per recent tick from
// `.flume/tick-verdicts.jsonl` (phase, tags, model, turns, minutes, output
// tokens, outcome, trunk sha). The log is runtime state in the PRIMARY
// checkout, gitignored, never in the worktree plan renders in, so resolve the
// primary via git's common dir. Never fails the render: an unreadable log
// prints as unavailable.
import { readFileSync } from "node:fs";
import { execSync } from "node:child_process";
import { dirname, join } from "node:path";
let file = ".flume/tick-verdicts.jsonl";
try {
  const common = execSync("git rev-parse --path-format=absolute --git-common-dir", { encoding: "utf8" }).trim();
  file = join(dirname(common), ".flume", "tick-verdicts.jsonl");
} catch {}
const LAST = 30;
let out = "";
try {
  const rows = readFileSync(file, "utf8").split("\n").filter(Boolean).slice(-LAST);
  for (const line of rows) {
    try {
      const r = JSON.parse(line);
      const inv = r.invocations ?? [];
      const turns = inv.reduce((n, i) => n + (i.turns ?? 0), 0);
      const mins = (inv.reduce((n, i) => n + (i.durationMs ?? 0), 0) / 60000).toFixed(1);
      const outTok = inv.reduce((n, i) => n + (i.outputTokens ?? 0), 0);
      const model = [...new Set(inv.map((i) => i.model))].join("+") || "-";
      const outcome = r.committed
        ? (r.shippedTags?.length ? `shipped ${r.shippedTags.length}` : "committed")
        : (r.noCommit ?? "no-commit");
      const tags = (r.tags ?? []).join(",") || "-";
      out += `${(r.at ?? "").slice(0, 16)} ${r.phaseName} ${tags} · ${model} · ${turns} turns · ${mins} min · ${outTok} out · ${outcome} · ${(r.headSha ?? "").slice(0, 8)}\n`;
    } catch {}
  }
} catch {}
process.stdout.write(out.length > 0 ? out : `(tick verdicts unavailable at ${file})\n`);
