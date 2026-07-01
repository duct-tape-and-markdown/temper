<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- MATCH-ERADICATE (spec `01d4f59`): remove the name-`match` selector entirely — opt-in
  `satisfies` is the sole binding; `kind`/`contract` typing stays.
  - `src/compose.rs`: drop `MatchSelector` (enum + `parse`/`selector_from`), the `match`
    key from the requirement allowlist + `Requirement.selector` field, and any `Match*`
    ComposeError variants. A `match = {...}` key is now an unknown key (rejected).
  - `src/roster.rs`: the set a requirement quantifies over becomes its **satisfier set**
    (artifacts of the requirement's `kind` whose representation `satisfies` it), not a
    selector-filtered set. `check`/`conformance`/set-scope predicates (count/membership/
    unique) range over satisfiers; drop `fillers(selector,..)` and the `selector.is_none()
    → skip` branch. `conformance` validates the satisfiers against the requirement's
    `contract`. Whole-kind population constraints quantify over the `kind` (all artifacts
    of it); intent subsets over the satisfier set.
  - `src/graph.rs`: `degree` selects its node set by the requirement's `kind` + satisfiers,
    not a `MatchSelector`.
  - `src/coverage.rs`: already satisfies-based; remove the now-moot "requirement WITH a
    selector is skipped here" branch (no selectors exist anymore) — coverage checks every
    requirement's satisfier presence + dangling.
  - tests: update `tests/requirement_roster.rs` / `tests/temper_toml.rs` / `tests/graph.rs`
    fixtures to define sets via `satisfies` (representation) + `kind`, not `match = {name}`;
    assert a `match` key is now rejected. Re-record insta snapshots deliberately.
  - ATOMIC: the selector removal ripples compose→roster→graph→coverage at once (like
    CONSOLIDATE-REQUIREMENT); sole `open` entry, no parallel wave. Stay green.
  - NOTE: temper.toml already uses satisfies-only (no match) — the dogfood is the target.

- REPRESENTATION deepening — HOLD: a design pass on making `.temper/` contents a true
  *representation of the artifact* (not a byte copy + thin header) is under discussion with
  the human. Do NOT file representation/import/IR changes yet; the direction isn't set.
