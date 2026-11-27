# Which kinds' content "never enters context", so that a loads guard over them refuses at load?

`specs/model/contract.md`, "clause" (0120 amendment): "a guard over a kind whose content never enters context refuses at load", with a hook, an MCP server and settings as the examples. Registration answers this for some kinds and not others. `hook` (`event`), `mcp-server` (`connection`), `installed-plugin` (`enablement`) and `known-marketplace` (`registry`) register only through channels that never load content. But `settings` declares an **empty** registration (`src/builtin_lock.toml`), and so do `output-style`, whose body becomes the system prompt and so enters context, and `supporting-doc`, which loads through its host. The same goes for `mcp`, `dial`, `handler`, `marketplace`, `plugin-manifest`, `requirement`, `tool` and `settings-local`. Nothing in the model separates `settings` from `output-style` here.

Two readings each fit the corpus:
1. Refuse only where every declared channel is content-less (event / connection / enablement / registry). That misses `settings`, the spec's own example.
2. Add a kind-level fact (or a registration channel such as output-style's system-prompt load) that says whether a kind's content enters context, cited per kind, and refuse wherever it doesn't. That is a model and seam change (`specs/model/kind.md`, the declaration row).

Underneath both: what load moment does `output-style` have, and does an embedded or nested-file member inherit its host's moment? Without an answer, a guard over either stays vacuous, which invariant 6 forbids.

Dependents: the refusal half of the 0120 amendment. LOAD-MOMENT-AT-INVOCATION does not wait on this.
