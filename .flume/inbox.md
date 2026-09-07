<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at 492b432b (cascade-integrations, live on a real harness;
  code confirmed by the session) — GH #42 (ii) is open and has no entry:
  `install.rs` `POST_TOOL_USE_COMMAND` is byte-identical to
  `SESSION_START_COMMAND` (`temper check . --reporter session-start`), so
  the session-start reporter's pass-time disclosure (`Checked: …`, ~960
  bytes of additionalContext) is injected on EVERY tool call, not once at
  session open. `specs/distribution.md` "Session start" requires that
  surface to never pass silently — the disclosure is that non-silence,
  once — and names no PostToolUse surface at all; install wires one
  anyway. Challenge before filing: what is PostToolUse FOR under the
  intent? If it is "a write drifted a projection, say so now", its
  reporter carries findings only and is empty on pass (silence is correct
  mid-session; the session already disclosed), a `--reporter post-tool-use`
  or a `--quiet-on-pass` flag on the same reporter; if nothing, install
  stops wiring it. Either way distribution.md gains the surface (or its
  absence) before code moves — a fork if the ruling is not derivable.
  Cascade will run the issue's repro against the landing sha.
