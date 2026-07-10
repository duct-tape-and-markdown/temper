# Plan state

- Spec derived through: a0fccaf
- Audited through: 55c8299
- Residue swept through: 9e32fa8
- This tick: Ship audit — 008a995..HEAD shipped two entries, both verified
  on disk. BUILTIN-CONTRACT-ARRAY-SURGERY (4144b20): `compose::effective`
  retired (grep-clean save a test doc comment, compose.rs:233 — sweep's
  class), rows-are-the-contract with embedded-default fallback at main.rs
  50/74/76/623, module docs restated (contract.rs:6, schema.rs:6), gate
  assertions at acceptance.rs 190/209/255. UNTEMPLATED-NESTED-MEMBER-LOUD
  (4752b06): SDK refusal at declarations.ts:450, engine reject landed as
  `nested_member_admissibility` in main.rs (not graph.rs — depth-rule
  latitude), get_mut now a backstop, tests at tests/graph.rs:598 and
  refusals.test.ts 137/155/169; graph.rs incident narration cut — riding
  debt record deleted. LOCK-ROW-REJECT-LOUD unblocked (gate→open), main.rs
  edge read re-anchored 1269→1325; drift.rs/kind.rs untouched, anchors
  hold. PACKAGING-CHANNELS parked reason re-verified still true.
- Queue: LOCK-ROW-REJECT-LOUD (open); PACKAGING-CHANNELS (parked).

Plan continues: yes — residue sweep trails HEAD (swept through 9e32fa8;
4144b20 and 4752b06 unswept — compose.rs:233's retired-`effective` mention
already sighted).
