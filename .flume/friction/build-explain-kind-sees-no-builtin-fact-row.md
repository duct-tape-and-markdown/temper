## Symptom

`narrate_kind` reads every strand but guidance off a kind's **fact row**, and
`explain_target` sources those rows from `declarations.kinds` (src/read.rs:2436,
off the lock family assembled at :2304). That slice carries only the kinds
THIS surface's lock declares — for temper's own harness, four: `hook`,
`memory`, `rule`, `skill` (.temper/lock.toml). Every other built-in kind
resolves (its default contract is the existence oracle) and then narrates with
no row behind it:

    $ temper explain kind:agent
    Kind `agent`:

    No authoring guidance is declared for `agent`, and it declares neither a
    locus a member of it lands at nor a body layout, hosts no kind and is
    hosted by none — …

Guidance is not affected: it rides `contracts`, so `kind:marketplace` prints
its counsel today. Everything the row carries does not — locus, `cite`,
registration channels, commitment, templates, address form. `agent` ships a
`governs` pair, a registration set and a cite in src/builtin_kind.rs and none
of it reaches the reader; same for `command`, `mcp-server`, `settings-local`,
`known-marketplace`, `installed-plugin`, `dial`.

Pre-existing, not a regression: those kinds narrated the same empty line
before EXPLAIN-KIND-REGISTRATION-LOCUS-AND-ADDRESS-FORM. The new locus and
address-form strands make the hole visible, because the moment they were built
for — an adopter with **no member of the kind yet** — is exactly the moment a
built-in kind is absent from the lock.

## Cost this tick

~5 minutes, no retries, no reverted commit: found while eyeballing the new
strands against the real harness. Outside the entry's fence (the fix is in how
`explain_target` sources kind rows, not in `narrate_kind`), so nothing was
taken.

## Suggested fix

Product work — an inbox item. Give `explain` the built-in kinds' own fact rows
under the lock's, so a surface that declares a kind still wins and one that has
never authored a member of it still reads the shipped facts.
`compose::assemble_lock_family` already hands back `overlaid_builtin_kinds`
(a `BTreeMap<String, CustomKind>`) beside `declarations` at src/read.rs:2304,
so the overlay may already be in hand at the call site.
