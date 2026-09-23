/**
 * Shared fixtures for the SDK test suites under `sdk/test/` — the one home for
 * scaffolding the suites would otherwise hand-spell per file, the counterpart of
 * `tests/common/mod.rs` on the engine side.
 *
 * Not itself a test module: `node --test` globs only `.test.js` files under
 * `dist/test/`, and this file is not one, so it compiles into `dist/test/` as a
 * plain module the suites import.
 */

import type { ClauseRow } from "../src/generated/index.js";

/**
 * A [`ClauseRow`] naming `predicate` at `severity`, every other column
 * `undefined` — the one default-filling home for the family, the shape
 * `clauseRow` in `sdk/src/declarations.ts` lowers a non-guard clause to. Call
 * sites spread it and override only the columns they diverge on (`kind`, and the
 * predicate's own argument column: `count`/`bound`/`charset`/…).
 *
 * The key *set* is the contract, not just the values: `node:assert/strict`'s
 * `deepEqual` compares own keys, so this literal spells every column the lowering
 * writes — a base short one key passes a case that should fail. `label`,
 * `guard_predicate` and `body` are absent for the same reason: the non-guard
 * lowering does not write them, and a runtime row lacking a key the expectation
 * carries fails just as loudly.
 */
export function clauseRow(predicate: string, severity: string): ClauseRow {
  return {
    kind: undefined,
    predicate,
    field: undefined,
    severity,
    guidance: undefined,
    cite: undefined,
    count: undefined,
    target: undefined,
    degree: undefined,
    gate: undefined,
    value_type: undefined,
    shape: undefined,
    bound: undefined,
    unit: undefined,
    charset: undefined,
    keys: undefined,
    values: undefined,
    range: undefined,
    section: undefined,
    sections: undefined,
  };
}
