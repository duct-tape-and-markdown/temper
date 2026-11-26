# Vocabulary pass: code-side instances of 0114

The inventory and blind name test that 0114 rests on found these instances in code and docs. This is evidence for derive, not a ruling: the rulings are `authoring.md` ("One concept, one name", the default export) and `contract.md` (clause id). The full tables are in the session scratchpad and are volatile, so the instances that matter are copied here.

- **Counts.** `sdk/src/prose.ts:2,227` say "four constructors", but `Prose` has three (`:231` already says three). `sdk/package.json` describes a "six-noun model" whose six nouns are named nowhere. The `sdk/src/assembly.ts:3` header lists five `Harness` fields while `harness()` takes eight keys.
- **Severity words.** `src/main.rs:104` says "advisory (warn-severity)". `src/main.rs:87` says "Lint a harness" where the verb is the gate.
- **Requirement as "domain".** `sdk/src/reference.ts:43,50,441`: "the domains it fills", "contracted domains".
- **Clause id spellings.** `DialEntry.label` and the lock column `label`; the finding field `rule` and the `*_RULE` constants; the reference skill text "the `code`".
- **Predicate casing.** `max_len`, `min_len`, `forbidden_keys`, `allowed_chars`, `require_sections`, `must_define` and `section_contains` sit beside kebab-case `closed-keys` and `name-matches-dir`, so ids read `skill.max_len.name` next to `skill.name-matches-dir`.
- **Default contract name.** `memoryAnthropicDefaultContract` (`sdk/src/builtins.ts:2685`).
- **Workspace.** `emit` takes it only as `--into` (`src/main.rs:140`); the other verbs take it positionally.
- **Hook `name`.** `hook` and `gateHook` (`sdk/src/builtins.ts:1048`) take the event as `name`.
- **`governs`.** The relocate/extend delta key (`sdk/src/kind.ts:1009`), the lock's `governs_root`/`governs_glob`, and the finding `kind.governs-collision`.
- **The program's tail.** `.temper/harness.ts` and the shipped example end in `process.stdout.write(emit(program).seam)`.

Each rename owes a line in the next migration note.
