# 0031 — a plugin registers by enablement

- **Date:** 2026-07-16 · **Status:** accepted

## Context

The `(plugin-surface)` fork, human-ruled ship 2026-07-16. The corpus
already named the consumer half twice — an installed plugin is a
registration member (`model/representation.md`, "Reach";
`specs/builtins.md`, "The coverage bar") — while no such kind shipped
beside its named siblings `hook` and `mcp-server`, because it does not
derive: a plugin reaches no documented channel itself, it **contributes
members** that do. The producer half was not corpus-carried at all, yet
is live in-product: `temper bundle` writes `.claude-plugin/plugin.json`
and `.claude-plugin/marketplace.json` as hard-coded `serde_json`
(`src/bundle.rs:185,191`) — a delivery surface temper emits and does not
model or gate. Surfaced with the fork: `CLAUDE.md:9` claims the manifests
among temper's projections while `specs/intent.md` does not — an identity
overclaim this ruling makes true rather than cuts. Demand: simulated
(war game, 07-16) for the producer half; the consumer half's warrant is
the corpus's own two sentences. Schemas re-fetched raw this session
(code.claude.com/docs/en/plugins-reference and /plugin-marketplaces,
retrieved 2026-07-16).

## Decision

Three kinds ship; the roster grows seven → ten.

- **installed-plugin** — a registration member: one enablement entry
  under `settings.json`'s `enabledPlugins`. Its channel is the
  **enablement entry itself** — the documented act that puts the plugin
  in play, the exact sense in which `mcp-server`'s channel is its
  connection. The members a plugin contributes live outside the corpus
  (the versioned plugin cache), and their reach stays **unmodeled and
  named as such** — the honest subset, the `settings.json`
  partial-governance posture and `supporting-doc`'s markdown-only
  template as precedent.
- **plugin-manifest** — a file member at `.claude-plugin/plugin.json`,
  JSON document format (a deliberate format-inventory addition — format
  mechanics are engine code). Identity from its `name` field, the one
  required field, kebab-case. The default contract mirrors the
  strictest documented profile: `claude plugin validate --strict` —
  the runtime tolerates unrecognized fields, `--strict` is the portable
  bar, and wrong-typed fields fail everywhere.
- **marketplace** — a file member at `.claude-plugin/marketplace.json`.
  Identity from `name` (kebab-case, checked against the documented
  reserved-names deny list); `owner.name` required; each `plugins[]`
  entry requires `name` plus `source`, the source union as documented
  (relative path, `github`, `url`, `git-subdir`, `npm`, with `sha`
  beating `ref` where both appear).

Per-field precedence between a manifest and its marketplace entry
(documented per field: `version` — plugin.json wins; `defaultEnabled` —
the entry wins) is clause-and-cite territory in code, never a spec
enumeration. The two manifest kinds sit **outside the domain partition**:
they carry distribution metadata, no session content, and the partition
stays a content map. `bundle`'s hard-coded writers reconcile onto the
kinds — emit through the kind system, one home; the entry names the
unification.

## Rejected

- **A separate producer package** (manifest kinds outside the built-ins):
  the plugin author's corpus is a first-class consumer — temper ships a
  plugin itself (`specs/intent.md`, "Positioning") — and a parallel home
  for two claude-code-documented formats is the duplicate-surface
  disease.
- **Modeling contributed-member reach now**: a plugin's skills and hooks
  live in the cache, not the governed corpus; typing their channels
  today invents a graph over files no gate reads. The condition that
  reopens: a consumer who needs to gate what a plugin contributes, not
  that it is enabled.
- **Waiting for real (non-simulated) producer demand**: the gap is not
  speculative — `bundle` already emits both manifests ungated, so the
  product currently writes files it refuses to check; that is temper's
  own bar failing in temper.
- **Cutting the `CLAUDE.md` claim instead**: shipping was ruled; the
  claim goes true.

## Consequences

`builtins.md`'s roster and coverage text carry the three kinds — same
commit, this record. The `(plugin-surface)` fork record deletes. Plan
derives the entries: the JSON document format mechanics, three kinds
with default contracts (clauses cite the two doc pages, re-fetched raw
at encode time per `builtins.md`), the `bundle` consolidation. Temper's
own repo becomes a candidate plugin-author corpus once the producer
kinds exist — the dogfood extends to the surface it ships.
