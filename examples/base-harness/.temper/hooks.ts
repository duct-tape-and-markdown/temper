import { gateHook } from "@dtmd/temper/claude-code";

// The three gate placements, authored as gate hook members: each states only
// the event and matcher it rides, and the engine renders the temper command
// it runs, anchored at the project root, into the settings.json projection.

/** The advisory gate report at session open — always exits zero. */
export const hook_sessionStart = gateHook({ name: "SessionStart", satisfies: ["governance"] });

/** The write-boundary guard; mode is read live from the lock (default warn). */
export const hook_guard = gateHook({
  name: "PreToolUse",
  matcher: "Write|Edit|MultiEdit",
  satisfies: ["governance"],
});

/** The guard's other edge: a Bash write carries no file path, so it is judged after the call. */
export const hook_postToolUseBash = gateHook({
  name: "PostToolUse",
  matcher: "Bash",
  satisfies: ["governance"],
});
