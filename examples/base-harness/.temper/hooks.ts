import { hook } from "@dtmd/temper/claude-code";

// The two gate placements, authored as fields-only registration members —
// each folds into its `hooks.<Event>` entry in the settings.json projection.

/** The advisory gate report at session open — always exits zero. */
export const hook_sessionStart = hook({
  name: "SessionStart",
  type: "command",
  command: "temper check .temper --reporter session-start",
  satisfies: ["governance"],
});

/** The write-boundary guard; mode is read live from the lock (default warn). */
export const hook_guard = hook({
  name: "PreToolUse",
  matcher: "Write|Edit|MultiEdit",
  type: "command",
  command: "temper guard .",
  satisfies: ["governance"],
});
