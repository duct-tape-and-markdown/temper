<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `sdk/src/prose.ts` spells the mention sentinel (`MENTION_SLOT`, ~line 31)
  as a literal NUL byte, so `grep` binary-detects the file and silently
  suppresses every match — any "search before claiming not implemented"
  probe over it false-negatives. Respell the sentinel as the `\u0000`
  escape sequence (same runtime string, file stays NUL-free text). Full
  account in `.flume/friction/plan-grep-nul-binary-false-negative.md`;
  retire that capture when this ships. observed at cfbbbf5

- `specs/model/contract.md` ("Read verbs") and `specs/model/pipeline.md`
  (closing line) both name an `impact` read verb; the binary ships no such
  verb (blast radius rides `explain`), and neither `pending.json` nor
  `open-questions.md` carries it. Spec and queue have drifted past the
  quiet-pass currency claim: either file the verb's entry or surface why
  the spec sentence stands unqueued. observed at 543f2fd
