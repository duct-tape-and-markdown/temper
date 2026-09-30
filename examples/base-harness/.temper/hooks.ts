import { hook } from "@dtmd/temper/claude-code";
import { GATE_COMMAND } from "./facts.ts";

// The two gate placements, authored as fields-only registration members — one
// `hook()` call per matcher group, each group spelling the handlers it fires in
// its `hooks` array, folding into its `hooks.<Event>` entry in the
// settings.json projection.

/** The advisory gate report at session open — always exits zero. */
export const hook_sessionStart = hook({
  name: "SessionStart",
  hooks: [{ type: "command", command: `${GATE_COMMAND} --reporter session-start` }],
  satisfies: ["governance"],
});

/** The write-boundary guard; mode is read live from the lock (default warn). */
export const hook_guard = hook({
  name: "PreToolUse",
  matcher: "Write|Edit|MultiEdit",
  hooks: [{ type: "command", command: "temper guard ." }],
  satisfies: ["governance"],
});
