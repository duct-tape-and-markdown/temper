<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at 913dfc2e (cascade-integrations, live `guard .` runs on a real
  block-mode harness; posted on GH #40) — two residuals on the co-owned
  manifest contract, neither refuting ef98e2cd. (1) **The guard can be
  switched off through the manifest it protects, silently.** Replacing
  `temper guard .` with `true` in the PreToolUse command — by Edit,
  MultiEdit, or whole-file Write — exits 0: the key is present so
  `expected_keys` passes, and the registration row carries no command or
  bytes to compare. Afterwards `check` exits 0 with no gate-installed and
  no drift finding; the only trace is `coverage.checked` counting 10
  members instead of 11, and the next `emit` quietly re-places the command.
  Position: BOTH shapes, because they answer different bars — the
  registration row carries a digest of the emitted member's bytes and
  `manifest_write_findings` compares at the boundary (block mode blocks a
  write that changes a governed member's bytes: a hook's command IS the
  member); and `check` treats a governed member whose bytes differ from
  what emit would place as manifest drift (loud, 0048), never a silent
  count drop. (2) **Invalid JSON written to settings.json exits 0** by the
  design comment at `manifest_write_findings` (only the next check fails).
  For the file that carries the guard, an unparseable settings.json means
  Claude Code drops every hook, the guard included — that deferral is a
  ruling to make, not the parse-hiccup default: position, a write that
  makes a hook-bearing manifest unparseable is a block-mode finding at the
  boundary. Cascade holds the guard-hole demolition row open until (1) is
  ruled; cite the co-owned manifest section for `per`. Cascade's stated
  bar (on #40): emit repair after the fact is not a close — the hook IS
  the guard, so a rewritten command is the hole by definition, and a
  check that exits 0 while the checked count drops is a thinnest-pass
  failure; command-content comparison for members temper placed, or at
  minimum a check finding on the count drop. On (2) they accept a
  check-time failure IF the session-start report states outright that
  settings.json is unparseable and hooks are off — an acceptable
  alternative to the boundary block, provided it is that loud.
