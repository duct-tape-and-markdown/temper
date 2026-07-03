# Example config — `temper` governing itself (illustrative)

> **Status: illustrative north-star, not normative and not the live config.** The
> contract engine isn't built yet (the flume loop is building toward it). This is
> a worked example of what `temper`'s *own* config could look like once the
> engine exists — a target the build can aim at, and a concrete read on the model
> in `specs/`. Every clause here is consistent with `specs/architecture/10-contracts.md` and
> `specs/architecture/30-landscapes.md`. When the engine lands, the real config supersedes
> this; until then, treat it as a picture, not a promise.

The point of the picture: `temper` governs *itself*. The harness it manages, the
specs it's defined by, and (eventually) its own code are all landscapes under one
engine. Below, watch the gate fail on `temper`'s own specs for a true reason —
that's what self-hosting should feel like.

## Layout

```
.temper/
  contracts/
    skill.toml      # artifact contract — instance of the engine
    rule.toml       # artifact contract — .claude/rules/*
    harness.toml    # harness contract — author's .claude/ roster
    spec.toml       # spec-landscape contract — the specs/ corpus
  model/
    domain.toml     # the declared domain model the prose binds to
```

The `contracts/` and `model/` files are the *config* — the author-declared
intent. The imported surface (`.temper/skills/...`, `author.toml`) is the
*subject* the contracts are checked against (`specs/architecture/20-surface.md`).

## An artifact contract (skills) — `contracts/skill.toml`

```toml
extends = "templates/skill.anthropic"   # adopt the template, override below

[fields.name]
required = true
pattern  = "^[a-z0-9-]+$"        # decidable
max_len  = 64
deny     = ["anthropic", "claude"]
severity = "required"            # gate-blocking (tier-1)

[fields.description]
required = true
max_len  = 1024
severity = "required"

[fields.disable-model-invocation]
must_define = true               # the KEY exists — decidable (Pocock's invocation axis).
severity    = "advisory"         # NOT "must have a trigger" — that's undecidable, excluded.

[structure]
body.max_lines = 500

[references]
syntax   = "markdown-link"       # refs checked ONLY as [text](path), never grepped from prose
severity = "required"

forbidden_keys = ["globs", "alwaysApply"]
```

## The contract that would have caught a real bug — `contracts/rule.toml`

```toml
# .claude/rules/*.md  —  Claude Code's real scoping key is `paths`.
[fields.paths]
optional = true

forbidden_keys = ["description", "globs", "alwaysApply"]   # CURSOR keys; CC ignores them
severity       = "required"
```

Run this against the `rust.md` / `collaboration.md` that were hand-authored early
in this project with `description/globs/alwaysApply` frontmatter, and:

```
✗ rule.forbidden_keys :: .claude/rules/rust.md
  found `globs`, `alwaysApply` — Cursor frontmatter; Claude Code ignores these  [required]
```

A mistake made by hand, caught structurally. The whole pitch in one diagnostic.

## The harness contract — `contracts/harness.toml`

```toml
requires = ["memory", "rust-conventions", "collaboration", "settings"]

[role.memory]
artifact = "claude_md"
match    = { path = "CLAUDE.md" }
contract = { max_lines = 200 }            # 77 today — passes
required = true

[role.rust-conventions]
artifact = "rule"
match    = { name = "rust" }
contract = { must_define = ["paths"] }    # must be path-scoped
required = true

[role.collaboration]
artifact = "rule"
match    = { name = "collaboration" }
contract = { forbidden_keys = ["paths"] } # must load unconditionally → no paths
required = true

[role.settings]
artifact    = "settings"
contract    = { fields = { autoMemoryEnabled = { equals = false } } }
verified_by = "pnpm exec flume render plan"   # behavior (does it load?) → delegated, not run by author
required    = true

[skills]
min_count = 1
```

## The declared model (the graph) — `model/domain.toml`

The prose specs *bind to* this. This is the structure; the prose is the meaning
(`specs/architecture/30-landscapes.md`).

```toml
[[entity]]
name = "Primitive"
owned_by = "specs/architecture/10-contracts.md"
depends_on = []

[[entity]]
name = "Contract"
owned_by = "specs/architecture/10-contracts.md"
depends_on = ["Primitive", "Extractor"]

[[entity]]
name = "Landscape"
owned_by = "specs/architecture/30-landscapes.md"
depends_on = ["Contract"]

[[entity]]
name = "Extractor"
owned_by = "specs/architecture/30-landscapes.md"
depends_on = []

[[entity]]
name = "Surface"
owned_by = "specs/architecture/20-surface.md"
depends_on = ["Provenance"]
```

A spec binds with one minimal marker — *declared, not a template* (this resolves
`specs/process/90-spec-system.md`'s "no frontmatter": it's a comment, invisible in render):

```markdown
<!-- owns: Contract, Primitive -->
# Contracts — the two-layer model
...prose stays free prose...
```

## The graph in practice — blast radius (tier-1, no LLM)

```
$ temper graph --blast Primitive

Primitive            specs/architecture/10-contracts.md
└─ Contract          specs/architecture/10-contracts.md        (depends_on Primitive)
   └─ Landscape      specs/architecture/30-landscapes.md        (depends_on Contract)
└─ src/contract/primitive.rs::Primitive            (spec⟷code seam)

  removing `Primitive` perturbs 3 entities, 2 specs, 1 code symbol.
```

Delete a load-bearing concept and you see the wreckage before you cause it.
Deterministic, no model call.

## `temper check` — the three tiers, live

```
$ temper check

contracts/rule.toml
  ✓ .claude/rules/rust.md          (paths defined, no Cursor keys)
  ✓ .claude/rules/collaboration.md (loads unconditionally)

harness.toml
  ✓ memory            CLAUDE.md         77 ≤ 200
  ✓ rust-conventions  rust              paths ✓
  ✓ collaboration     collaboration     no paths ✓
  ✓ settings          autoMemoryEnabled = false
  ⊘ settings          verified_by `flume render plan` — wired (not run; CI's job)
  ✓ skills            2 ≥ 1

spec.toml :: specs/
  ✓ 5 files · all ≤ 150 lines · one topic each
  ✓ every ## Decision names rejected alternatives        (4/4)
  ✗ coverage: entity `VerdictTier` declared, no owning spec   [required]
        └ 00-intent.md says "## Three verdict tiers" but binds no entity
  ⊘ fidelity (tier-2, judged): DEFERRED — 0/23 atoms judged (out of scope)

  group: 4 ✓ · 1 ✗ · gate: FAIL (1 required violation)
```

What each line *is*:

- the harness rows = the **harness contract** (roles filled + conforming);
- `⊘ verified_by … not run` = the **delegation seam** — `temper` proves the judge
  is *wired*, never runs it (`specs/architecture/10-contracts.md`, `verified_by`);
- `✗ coverage: VerdictTier …` = a real **tier-1 catch** — and it's catching a
  naming slip in the live corpus: `VerdictTier` is named in `00-intent.md` but
  bound to no spec. "Name the same concept the same way," now structural;
- `⊘ fidelity … DEFERRED` = tier-2, visibly **not pretending to be a gate**.

The punchline: the gate **fails on `temper`'s own specs, for a true reason.**
That's self-hosting — the tool unimpressed with its authors.
