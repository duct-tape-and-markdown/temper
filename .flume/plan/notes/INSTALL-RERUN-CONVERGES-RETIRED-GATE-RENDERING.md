# Retired gate rendering: re-run converges, check names emit

Shipped as the entry names it. Three things plan should know:

- New `ApplyOutcome::Retired` (src/drift.rs). The represented split needed a third
  stale shape beside missing and superseded-by-member. `InstallEntry` carries only the
  outcome, so the outcome is the channel. The install report's tally line gains
  "N retired".
- Unanchored residue. The pre-anchor form (`command -v temper … exit 127`, no `cd`)
  is still read as foreign and appended beside, per
  `a_gate_command_in_the_unanchored_form_reads_as_no_rendered_gate`. The entry
  recognizes exactly one prior form. A settings.json that old still ends with two
  SessionStart groups.
- Both-present edge. A document holding today's rendering *and* the retired one at
  the same address gets the retired one re-rendered in place. That leaves two
  identical groups. Install never deletes a group, so this is not collapsed.
  Unlikely (it needs a hand-merge), but it is not "one group per address".
