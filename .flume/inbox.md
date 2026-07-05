<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

## The dogfood is DEACTIVATED (John, 2026-07-04) — reconcile the fork and the queue

John's ruling on `(inplace-lock-producer)`: **deactivate the dogfood — it's
cumbersome.** The session executed the mechanical half this commit: `.temper/`
and root `temper.toml` deleted (all cargo gates green without them — the
product never read its host repo's dogfood), selfCheckGate removed from
chain.ts (not just deactivated), the session-start hook and the projection
guard removed from `.claude/settings.json`, projection headers stripped from
`.claude/rules/`, CLAUDE.md / PROTOCOL.md / docs/ledger.md updated.
Validation lives in `tests/` fixtures; a real dogfood returns when
SDK-primary authoring (`harness.ts` → emit) is the product's own front door.

Plan should reconcile:
- `(inplace-lock-producer)` narrows to the external-user question only (who
  compiles an in-place `temper.toml` into lock rows until SDK emit is the
  producer) — the dogfood-data restoration half dies. Not urgent; it rides
  the SDK-primary foundation like everything else human-gated.
- Any open-questions/state wording that assumes the dogfood confirmation
  pass, the self-gate re-arm, or temper.toml-as-dogfood-assembly is stale.
- GATE-FAIL-LOUD-EMPTY-ASSEMBLY stays — it's correct product behavior for
  real users, discovered by the dogfood on its way out.
- Corpus shadow (00-intent's self-hosting finish line, 90-spec-system's
  confirmation recipe) is JOHN's hand, not plan's and not build's — listed
  on the ledger.
