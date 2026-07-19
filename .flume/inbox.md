<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## Field report (release-smoke fixture, 0.0.9): install --yes scaffolds a program node cannot run

Observed at a8fb82c, found by the new release smoke run locally against
0.0.9. `temper install --yes` in a plain npm project (root package.json
without `"type": "module"` — npm init default) scaffolds
`.temper/harness.ts` as ESM but writes no `.temper/package.json`, so
the first emit dies: "SyntaxError: Cannot use import statement outside
a module" (drift::sdk_program_failed). The on-ramp is broken for
exactly the stranger audience it exists for; temper's own dogfood
never hits it because `.temper/package.json` (type module, SDK dep)
is hand-provisioned. Fix at install's scaffold: write
`.temper/package.json` (`"type": "module"`, `"private": true`, the
`@dtmd/temper` dependency install already "ensures") alongside
harness.ts, and run the dependency install there, not the project
root. Acceptance: `npm init -y && npm i @dtmd/temper && npx temper
install --yes .` in a scratch dir reaches a green first emit — the
release smoke job (release.yml) runs exactly this sequence and gates
every future publish on it.
