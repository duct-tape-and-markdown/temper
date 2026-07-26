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

// The three lifecycle events `temper tap` classifies (src/builtin_kind.rs,
// `classify_claude_code_hook_payload`): InstructionsLoaded, UserPromptExpansion,
// and PostToolUse — the last unmatched, so it sees every tool, since Skill calls
// are what become SkillInvoked records.
//
// Soft-fail, deliberately NOT the `failLoud` preamble the gate hooks carry: the
// tap is an advisory recorder that always exits zero, so temper being off PATH
// must cost a missing record, never a broken tool call.
//
// `install` places no tap hook and the product declares no canonical tap command
// constant, so unlike the two above this form is the dogfood's own — a hand-mirror
// with nothing yet to mirror. When the product settles the canonical form, this
// adapts to it (the dogfood consumes the product; the product is never reshaped
// to spare the dogfood the copy).
const tapSoft = "command -v temper >/dev/null 2>&1 && temper tap . || true";

/** Record which memory and rule files actually entered context, and why. */
export const hook_tapInstructions = hook({
  name: "InstructionsLoaded",
  type: "command",
  command: tapSoft,
});

/** Record which commands expanded into prompts. */
export const hook_tapPromptExpansion = hook({
  name: "UserPromptExpansion",
  type: "command",
  command: tapSoft,
});

/** Record every completed tool call — Skill calls land as SkillInvoked. */
export const hook_tapToolUse = hook({
  name: "PostToolUse",
  type: "command",
  command: tapSoft,
});
