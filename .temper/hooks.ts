import { GUARD_COMMAND, SESSION_START_COMMAND, hook } from "@dtmd/temper/claude-code";

// The four session-layer hooks, authored where every other member lives.
// Fields-only registration members: no prose, no adjacent document — each
// folds into its `hooks.<Event>` entry in the settings.json projection.

// The gate commands are the SDK's (`GUARD_COMMAND`, `SESSION_START_COMMAND`),
// generated from the engine constants `install` compares against, so nothing
// here is a hand-kept copy.

/** The advisory gate report at session open — always exits zero. */
export const hook_sessionStart = hook({
  name: "SessionStart",
  hooks: [{ type: "command", command: SESSION_START_COMMAND }],
});

/** The write-boundary guard; mode is read live from the lock (default warn). */
export const hook_guard = hook({
  name: "PreToolUse",
  matcher: "Write|Edit|MultiEdit",
  hooks: [{ type: "command", command: GUARD_COMMAND }],
});

/**
 * The guard's other edge: a Bash write carries no file path, so it is judged
 * after the call. One command serves both edges — the guard reads the firing
 * event from its payload — so it fires GUARD_COMMAND like the hook above.
 */
export const hook_postToolUseBash = hook({
  name: "PostToolUse",
  matcher: "Bash",
  hooks: [{ type: "command", command: GUARD_COMMAND }],
});

/** Keep Rust formatted as the agent edits; never fails the tool call. */
export const hook_fmtOnWrite = hook({
  name: "PostToolUse",
  matcher: "Edit|Write",
  hooks: [{ type: "command", command: "cargo fmt --quiet >/dev/null 2>&1 || true" }],
});

// No tap hook is authored here on purpose. Tap hooks are *synthesized* from a
// telemetry verifier — `pipeline.md`, "Telemetry": a telemetry declaration
// projects as tap hook registrations in the emitted manifest. The declaration
// lives on the `context-arrives` requirement in `harness.ts`; the command
// (`TAP_COMMAND`) and the per-event matchers (`TELEMETRY_EVENT_HOOKS`) are the
// SDK's, so there is nothing to hand-mirror and nothing to keep in sync.
