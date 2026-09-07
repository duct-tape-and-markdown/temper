<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


- observed at ab61fea4 (cascade-integrations, live on a real harness with
  kinds but no spec members; GH #58 filed, assessment on #47) — two
  findings. (1) **GH #58, a hole, the file-locus counterpart of
  676467cf:** a document the lock declares no member for, dropped into
  `.claude/rules/`, `.claude/skills/<x>/SKILL.md`, or `.claude/agents/`,
  passes `guard .` (creation is never an emit-owned target), is admitted
  and counted by `check` (`coverage.checked` 11 → 13, no finding), and
  `emit` reports 0 orphan-drift — identical on 0.0.17, and worse in
  direction than the layout case because the document is silently present
  in what Claude Code loads. Position: the read represents what it was
  given (0048) — a discovered document at a represented kind's locus with
  no declared member is a named finding, path and kind, with declared and
  undeclared counted separately in the disclosure; the guard's part
  follows the kind's enforcement mode (block mode refuses creation at an
  undeclared path in a represented kind's locus), and a partly
  represented harness opts the locus down the way `local` does, never by
  silence. `per` the discovery/adoption section; cascade will audit the
  entry's files[] when cut. (2) **#47 assessment, not a refutation:**
  `explain kind:<name>` (3dcc73f1) renders guidance, the hosted-kinds list
  with leaves derived from existing members (`corpus_leaves`,
  `read.rs` ~817), file children, and layout regions — and nothing else.
  Of the entry's "fields, layout, registration locus, address grammar"
  only layout renders: on a kind with no members yet it prints one
  guidance line, no leaves, no nested address form; `kind:hook` omits its
  registration locus. Leaves share the lock's gap (the kind row carries no
  leaf schema; with typed leaves landed, `keyof T` can lower onto it);
  registration locus and the address form are computable today. Re-open
  or follow-on with those three lines named in acceptance, each verified
  on a member-less kind.

- observed at ab61fea4 (cascade-integrations, reproduced with
  `settings.json` = `{ not json`) — premise correction for
  SESSION-START-LOAD-ERROR-BYPASSES-REPORTER before it is built. The bypass
  is real (`check --reporter session-start` exits 1 with the raw miette
  error on stderr and no hookSpecificOutput at all), and the reporter fix
  is right for every OTHER load error. But for the hooks manifest itself
  it cannot deliver the loud statement: the SessionStart hook that would
  run the reporter is declared in the file that no longer parses, so
  Claude Code never invokes it — nothing temper does at session start can
  reach a session whose hooks are off. The only point of control for that
  file is the guard at write time, which `manifest_write_findings`
  currently waves through as a parse hiccup left to CI. The entry must
  therefore either pair with a guard refusal — under block mode an
  unparseable Write/Edit/MultiEdit to a manifest carrying hook
  registrations is refused, since it is the one write that switches every
  guard off at once — or state in its body that it does not cover the
  hooks manifest. Position: pair; the refusal is decidable at the boundary
  (parse the would-be manifest; a parse failure on a hook-bearing file is
  the finding), costs nothing on the pass path, and is the one case where
  "defer to CI" leaves the session with no guard at all.
