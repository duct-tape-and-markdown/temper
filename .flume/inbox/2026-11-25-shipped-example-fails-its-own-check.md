# The shipped example harness fails its own `check --deny-advisories` on 0.0.24

`cd examples/base-harness && temper check . --deny-advisories` exits 1:

- `install.gate-installed`: `.temper/hooks.ts` still builds its SessionStart and PreToolUse gates with `hook()` and command strings, not `gateHook()` (Decision 0082). The example is what adopters copy, so it teaches the form 0.0.24 removed.
- `root.locus-declared` ×4: `CLAUDE.md`, `.claude/rules/conduct.md`, `.claude/rules/docs-discipline.md` and `.claude/settings.json` sit at built-in loci, and the example's lock declares no member for them. Either the committed projections are stale against the program, or the program emits them elsewhere. Find out which, and make the committed tree what its own emit produces.

Bar: the example passes `check --deny-advisories` from its own root, and the test that compares the example's emit byte for byte (`tests/it/emit.rs`, `emit_program_runs_the_shipped_example_harness`) pins it. Also observed: a plain-string `prose: "…"` where the `text` tag belongs crashes emit with `TypeError: mentions is not iterable`. The SDK types reject it, but an untyped program gets no named refusal. A refusal naming the member and the field fits here or as its own entry.
