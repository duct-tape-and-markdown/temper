import { gateHook, hook } from "@dtmd/temper/claude-code";

// The four session-layer hooks, authored where every other member lives.
// Fields-only registration members: no prose, no adjacent document — each
// folds into its `hooks.<Event>` entry in the settings.json projection.

// The three gate hooks run the engine the `.temper` workspace installs
// (`npm -C .temper install`, the fresh-clone step), never a PATH guess. The
// engine renders each command from that one fact, anchored at the project.
const binary = ".temper/node_modules/.bin/temper";

/** The advisory gate report at session open — always exits zero. */
export const hook_sessionStart = gateHook({ name: "SessionStart", binary });

/** The write-boundary guard; mode is read live from the lock (default warn). */
export const hook_guard = gateHook({ name: "PreToolUse", matcher: "Write|Edit|MultiEdit", binary });

/** The guard's other edge: a Bash write carries no file path, so it is judged after the call. */
export const hook_postToolUseBash = gateHook({ name: "PostToolUse", matcher: "Bash", binary });

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
