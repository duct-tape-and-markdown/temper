<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


## Field report (centercode, 0.0.9): enablement wire label reader/writer split

Observed at 7410b1d. 30c52e1 respelled the enablement registration
channel to field-carrying, but only two of the three seam parties
moved: the engine reader (`registration_from_label`, src/kind.rs:1036)
demands `enablement(<field>)` and refuses bare; the SDK declaration
(sdk/src/builtins.ts:481) declares `{ via: "enablement", field:
"enabled" }`; the SDK serializer (`registrationLabel`,
sdk/src/declarations.ts:206) still returns bare `"enablement"` —
missed. src/builtin_lock.toml:90 embeds the stale bare form, which is
why CI stayed green: builtin_lock_frozen.rs byte-compares writer
against embedded snapshot (writer vs writer), and 30c52e1's reader
tests use hand-authored `enablement(enabled)` rows — no test
round-trips the real serializer through the real reader. Field
symptom: every harness with an installed-plugin member fails `check`
on the emit-regenerated lock (`registration` value `enablement`
outside the closed vocabulary) while emit reports full parity.

Three fixes, one entry chain:
1. `registrationLabel` case "enablement" returns
   `enablement(${registration.field})`, matching the other
   field-carrying channels (event, paths-match, description-trigger).
2. Regenerate src/builtin_lock.toml per its own header discipline
   (re-run the memberless builtin emit builtin_lock_frozen.rs embeds;
   re-embed derived rows verbatim, never hand-edit) — expected delta:
   line 90 becomes `registration = ["enablement(enabled)"]`.
3. The gate that makes a one-sided respell impossible: extend
   builtin_lock_frozen.rs (or sibling) to parse the derived lock back
   through the engine's lock reader after the byte-compare —
   serialize → parse across the real seam. Mechanizes sdk.md's
   two-sided-seam rule.

Release rides separately (0.0.10, session's job) once this ships.
