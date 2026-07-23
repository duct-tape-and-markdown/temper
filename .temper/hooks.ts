import { hook } from "@dtmd/temper/claude-code";

// The three session-layer hooks, authored where every other member lives.
// Fields-only registration members: no prose, no adjacent document — each
// folds into its `hooks.<Event>` entry in the settings.json projection.

// PATH-resolvability preamble: a temper-invoking hook fails loud (exit 127)
// when `temper` is off PATH, rather than a silent shell "command not found".
// This string MUST stay byte-identical to src/install.rs's SESSION_START_COMMAND
// / GUARD_COMMAND — `gate_installed` compares the emitted hook against that Rust
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

/** Keep Rust formatted as the agent edits; never fails the tool call. */
export const hook_fmtOnWrite = hook({
  name: "PostToolUse",
  matcher: "Edit|Write",
  type: "command",
  command: "cargo fmt --quiet >/dev/null 2>&1 || true",
});
