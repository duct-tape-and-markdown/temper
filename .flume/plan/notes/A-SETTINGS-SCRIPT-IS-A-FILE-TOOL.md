# Runner→tool edges ship as a `runs` lock family; a `{ file }` tool's script is an input

- Seam: a hook handler (`hooks: [{ ..., runs: someTool }]`) and the settings
  `statusLine` (`statusLine: { ..., runs }`, now a typed `Settings` field cited to
  code.claude.com/docs/en/statusline) name the tool they run. The SDK strips `runs` from
  the projection (`projectedField`, declarations.ts) and emits `[[declaration.runs]]`
  rows `{ member, tool }`. `check` folds each onto the runner's features under `runs`
  (`compose::fold_runs`), where the new `handler.runs` / `settings.runs` → `tool` edge
  facts resolve it. Status line edge rides the settings member's own address.
- A `{ file }` tool's `file` leaf is read as harness-relative: the engine fingerprints it
  as the tool's `input` at emit (`drift::resolve_tool_files`), refusing a missing file.
  Choice worth a ruling: authoring.md anchors path references to the stating module,
  but `tool({ file })` carries no module URL; the command runs from the project root.
- Every lock `input` row now claims its file for `coverage.unclaimed-entry`
  (authoring.md: an input is how a file joins the roster), not only tool scripts.
- SDK `memberAddress(toolMember)` still spells `tool:<name>`; runs rows use a local
  `root/tool/<name>` helper. The seam gap from TOOL-KIND-SHIPS remains for other rows.
- Not done: gate hooks, taps and the typecheck pointing at tools (other entries).
