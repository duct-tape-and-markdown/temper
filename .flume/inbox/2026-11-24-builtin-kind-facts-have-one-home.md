# The built-in kinds' facts are spelled three times; derive the Rust population from the embedded lock

Observed this session: BUILTIN-LOCI-CLAIM-WHAT-THE-HARNESS-LOADS parked twice on one change (rule glob, memory exclude). Each park found another copy of the same built-in fact. The copies:

1. `sdk/src/builtins.ts`, which emits `src/builtin_lock.toml` (pinned by `tests/it/builtin_lock_frozen.rs`).
2. `src/builtin_kind.rs`: 16 hand-written `claude_code_*() -> CustomKind` constructors behind `definitions()`. Discovery, check, emit and install read these.
3. Test pins that restate facts: `src/builtin_lock.rs` `every_lock_kind_row_lifts_to_the_facts_the_rust_population_spells`, and `tests/it/lock_declaration_rows.rs` `the_embedded_lock_kind_facts_match_todays_hand_written_kinds` (`rule.governs_glob == "*.md"` at ~2620).

The lift test already proves the embedded lock's kind rows lift to the Rust population, so (2) can be derived from (1) rather than kept beside it (`specs/process/engineering.md`, "Derived state is computed, never stored beside its source"; "A shared concept is one type"). File it as a fold with the whole census, by behavior (`.flume/PROTOCOL.md`, "What an entry means here", 2): every site that spells a built-in kind fact as a literal. Any fact the lock does not carry, such as a Rust-only render or cite, is the census's first finding, and either joins the row or stays as a named per-kind exception. Pins in (3) that restate facts become pins of the derivation.
