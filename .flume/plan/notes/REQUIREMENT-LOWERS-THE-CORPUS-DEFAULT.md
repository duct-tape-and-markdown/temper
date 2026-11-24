# The SDK's unset `required` now lowers to true

Shipped: `requirementRows` lowers `required ?? true`; `Requirement.required` docs the
inherited default; two SDK-through-check tests in `tests/it/requirement_roster.rs`
(unset fails unfilled; `required: false` stays advisory).

Observed, outside this fence:

- `examples/base-harness/.temper/harness.ts` declares `standards` and `operations`
  without `required`, and its comment says they are deliberately not floored ("a
  required empty slot would force filler"). Under the corpus default they now lower
  `required = true` (lock re-emitted; check stays green because both carry exemplary
  satisfiers). The author's stated intent is advisory, so the example should spell
  `required: false` on both — an `examples/base-harness/.temper/harness.ts` edit plus
  a re-emit, which this fence did not admit.
- `tests/it/coverage.rs` fixtures write lock rows directly (`common::requirement(_,
  bool, _)`), never through the SDK, so none relied on the old default; untouched.
- No SDK test (`sdk/test/**`) asserted the lowered `false`; the SDK suite is green.
- The dogfood's `required: true` restatement stays the human's
  (question dogfood-requirement-restates-default).
