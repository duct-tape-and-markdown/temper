# Does the dogfood drop its `required: true` once the SDK lowers the default?

Driver: GH #60, spec 0078 ("A default the SDK lowers is the corpus's default"). `.temper/harness.ts` sets `required: true` on its requirements only to get the shipped default the SDK failed to lower. Once REQUIREMENT-LOWERS-THE-CORPUS-DEFAULT ships, that spelling is a restatement of the default. `.temper/` is human territory (outside build's fence): a `chore(harness):` drop and re-emit, or a ruling that the explicit spelling stays.
