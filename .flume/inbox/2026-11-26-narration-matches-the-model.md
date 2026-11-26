# Doc comments and messages stop claiming what the model retired (a fold)

This is the narration-drift family, on its second noting (b02a40d3, d2f1d4f9). Code comments, guidance strings and messages still assert retired spec text, which is false guidance to every agent that reads the code (intent invariant 8). The instances:
- delivered "at authoring time via `schema` hover or `explain`", which 0091 retired (`sdk/src/kind.ts:298`, `:370`, `:442`; `src/drift.rs:5405`);
- "`temper` on the PATH" as the default, which 0100 and 0104 retired (`contract.ts`, `builtins.ts`);
- stale counts: "seven families" (`drift.rs:1127`, `:1136`, `:5244`, `:6198`; `emit.ts:1059`, `:1131`), the admissibility count and `pattern` (`engine.rs:125-130`, `:177`, `:3277`), and "two halves" (`install.rs:1372`) with its dry-run pointer (`:871`);
- `bundle.rs:235` `hook_events` and the SDK_COUNSEL rule names, where they lag.

Each sentence states what the model now says, or goes if it restates code. The census is every such claim found by behavior, not only these lines. This is a fold: pins only, net-neutral or negative.
