import { hook } from "@dtmd/temper/claude-code";

// The four session-layer hooks, authored where every other member lives.
// Fields-only registration members: no prose, no adjacent document — each
// folds into its `hooks.<Event>` entry in the settings.json projection.

// PATH-resolvability preamble: a temper-invoking hook fails loud (exit 127)
// when `temper` is off PATH, rather than a silent shell "command not found".
// This string MUST stay byte-identical to src/install.rs's SESSION_START_COMMAND
// / GUARD_COMMAND / POST_TOOL_USE_COMMAND — `gate_installed` compares the emitted hook against that Rust
// constant. The dogfood mirrors the product's canonical form by hand on purpose:
// this harness is a consumer of temper, so it adapts to the product's gate; the
// product is never reshaped to spare the dogfood the copy.
const failLoud =
  'command -v temper >/dev/null 2>&1 || { echo "temper: command not found" >&2; exit 127; } &&';

/** The advisory gate report at session open — always exits zero. */
export const hook_sessionStart = hook({
  name: "SessionStart",
  type: "command",
  command: `${failLoud} temper check . --reporter session-start`,
});

/** The write-boundary guard; mode is read live from the lock (default warn). */
export const hook_guard = hook({
  name: "PreToolUse",
  matcher: "Write|Edit|MultiEdit",
  type: "command",
  command: `${failLoud} temper guard .`,
});

/**
 * The post-write drift check on shell-mediated writes: the PreToolUse guard
 * binds Write/Edit only, so a Bash write is checked after the call instead
 * of never. Mirrors src/install.rs's POST_TOOL_USE_COMMAND byte-for-byte,
 * the same way the two hooks above mirror their constants.
 */
export const hook_postToolUseBash = hook({
  name: "PostToolUse",
  matcher: "Bash",
  type: "command",
  command: `${failLoud} temper check . --reporter session-start`,
});

/** Keep Rust formatted as the agent edits; never fails the tool call. */
export const hook_fmtOnWrite = hook({
  name: "PostToolUse",
  matcher: "Edit|Write",
  type: "command",
  command: "cargo fmt --quiet >/dev/null 2>&1 || true",
});

// No tap hook is authored here on purpose. Tap hooks are *synthesized* from a
// telemetry verifier — `pipeline.md`, "Telemetry": a telemetry declaration
// projects as tap hook registrations in the emitted manifest. The declaration
// lives on the `context-arrives` requirement in `harness.ts`; the command
// (`TAP_COMMAND`) and the per-event matchers (`TELEMETRY_EVENT_HOOKS`) are the
// SDK's, so there is nothing to hand-mirror and nothing to keep in sync.
