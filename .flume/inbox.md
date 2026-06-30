<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- `contracts/skill.anthropic.toml` is now human-authored (curated guidance —
  `contracts/` is human territory like `specs/`/`.claude/`, NOT build-writable;
  see `.flume/chain.ts`). SKILL-CONTRACT-TEMPLATE's data file is therefore DONE.
  Reconcile the entry to its remaining build-side work only: `tests/contract_template.rs`
  (load the file via `contract::Contract`, pin the clause vector). Then CHECK-CUTOVER
  embeds it (e.g. `include_str!`) as the default and wires `check` to the engine;
  RETIRE-HEURISTICS deletes `src/rules.rs`. Build EMBEDS contracts/, never writes it.
