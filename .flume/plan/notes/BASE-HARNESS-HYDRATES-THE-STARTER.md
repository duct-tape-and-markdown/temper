# Base harness composes starter(); playbook half owed

- `examples/base-harness/.temper/harness.ts` spreads `starter()` into
  `harness()`. The typecheck, the three gate hooks and the reference come
  from it. The worked customization is a hydrated reference description
  plus `satisfies: ["governance"]`. `hooks.ts` is retired.
- No patterns to hydrate. The blocker STARTER-SHIPS-THE-OPT-IN-PATTERNS was
  retired by 0111, and no playbook entry has shipped yet. So `kinds.ts` is
  untouched. When the playbook ships, the example owes a playbook
  customization, because the spec says the example hydrates "the shapes".
  That needs a follow-up entry.
- Fixed in `sdk/src/reference.ts`, needed for acceptance: the reference
  linked its topic files with hand-typed markdown. No mention rows reached
  them, so every topic tripped `supporting-doc.degree`. Any harness
  composing `starter()` failed `check --deny-advisories`. The links are now
  path-form mentions, with byte-identical output. Pinned by an SDK test.
- `gateHooks()` takes no `satisfies`. Before, the example filed its gate
  hooks under `governance`. Now it can't, so the `temper-guidance-hook` rule
  (which only carried that requirement) is reaped. Real adopters who file
  gates under a domain will hit this. Plan should decide whether the shape
  gets a `satisfies` option.
