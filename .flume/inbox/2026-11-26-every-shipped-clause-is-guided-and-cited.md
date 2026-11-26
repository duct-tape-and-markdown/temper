# Every shipped clause carries guidance and a dated cite, as the bar says

`specs/builtins.md`, "The clauses live in code", says every shipped clause is **guided** and **cited and dated**. The spec census (2026-11-26) counted 35 of 107 shipped clause rows with no `guidance`: all 24 `plugin-manifest.optional.*`, the `when` guard rows for handler, marketplace and mcp-server, and `dial.type.name`. It counted 12 with no `cite`: `tool.degree`, all 8 `dial.*`, and 3 root clauses. The test "every default contract clause carries a non-empty cite" skips the tool, dial and output-style contracts (`sdk/test/builtins.test.ts:60-77`), and it never checks for a date.

The fix is to give each clause its guidance and its cite (a doc URL and a retrieved date), or for a temper-owned clause like `dial.*` a cite naming the corpus section that rules it. Widen the test to every shipped contract, derived from the kinds in play and not from a hand list, so a new kind cannot be skipped. Acceptance:
- **new:** a shipped clause with no guidance, or no dated cite, fails the SDK suite (SDK test: no `tests[]` line);
- **new:** the test's contract set derives from the built-in kinds.
