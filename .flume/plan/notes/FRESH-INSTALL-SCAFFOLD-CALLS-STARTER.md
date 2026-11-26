# FRESH-INSTALL-SCAFFOLD-CALLS-STARTER

Shipped: `harness_entry_source` scaffolds `const base = starter({ typecheck: { project } })`
and `harness({ ...base, members: [...base.members, <lifted>] })`; the scaffold writes no
gate hook module and no `tsc` tool member. `gate_hook_module_source` is gone.

Observed:

- The gate hook remedy (`gate_hook_clause`) still tells an author to write
  `.temper/hooks/<Event>.ts` and import it — a path no scaffold writes any more. It is
  still a valid remedy, so `module_stems` keeps reserving those stems for it, but under a
  starter program the natural remedy is "restore the starter's gate hook" (or drop the
  filter that removed it). Whether the remedy should name the starter is a spec question
  for `adoption.md` "Install" / the installed-gate check, not taken here.
- Replacing one starter gate hook takes a `filter` over `base.members` by
  `member.kind === "hook" && member.name.split(":")[0] === event` (a hook's name joins
  its matcher). The tests do exactly that (`replace_starter_gate_hook`). Starter offers no
  per-gate opt-out; an adopter replacing one default will hit the same friction.
- `starter()` also composes the harness `reference` (the `temper` skill), so a fresh
  install now emits `.claude/skills/temper/`. A pre-existing lifted skill named `temper`
  would collide with it; no guard covers that case at install time.
