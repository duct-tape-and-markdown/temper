# SDK root-hosted/tool predicate lives in kind.ts, not builtins.ts

The entry placed the predicate in `builtins.ts` beside `isRequirement`. It
lands in `kind.ts` (`isRootHostedKind`, `isRootHosted`, `isTool`, beside
`extendsKind`) instead: two of the judges, `prose.ts` (`mentionOf`) and
`contract.ts` (`script`/`telemetry`), sit beneath `builtins.ts` in the
evaluation order (builtins calls their exports at top level), so importing
the shipped kinds' facts back into them is an ESM evaluation cycle. The
chain is walked by identity (BASES) and its root judged by name against
`ROOT_HOSTED_KINDS`, as the engine's `kinds_extending` does.

`memberAddress` now takes the judgement as a required second argument
(the grammar still owes the member model nothing); every caller passes
`isRootHosted`. Also folded beyond the entry's list: `gateHook`'s and
`composedToolAddress`'s bare `kind === tool.key` tool checks.

Left by name: `reference.ts` `facts.name === "tool"` choosing which kind
topic lists the commands. A tool subkind's own topic lists none; the
`tool` topic lists every tool-chain member. A debatable topic layout, not
an address/row seam, so it was left as is.
