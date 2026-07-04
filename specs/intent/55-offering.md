# The offering — the public front door and the road to adoption

`50-distribution.md` places the **gate**; this file places the **project**: the
repo as front door, the community surface around it, and the posture by which it
reaches its audience. The offering is not a second product — it is the wedge
(`00-intent.md`: a zero-config bare `temper check` over any existing `.claude/` —
the stranger gate, `50-distribution.md`)
made visible, and every asset here exists to put that one command in front of a
stranger. The evidence bar is sobering: a competing harness linter shipped 114
rules and a full docs site and sits at single-digit stars — **building well ≠
being found**. The offering is real work with its own contract.

## The front door — the README is the landing page

The exemplary shape (the Astral/Biome/atuin pattern, now the genre standard) is
a **short README, ~800 words**: dark/light-aware logo, a one-line tagline, a
badge row, **one hero visual**, a highlights list, and a run-before-install
quickstart — everything else links out. What converts is above the fold; the
manual is not the front door. Specific obligations:

- **The tagline follows the proven structure** — *category the reader knows +
  differentiator* — and names the incumbent frame: temper is the **type
  checker** for the harness, not another linter; "makes it *good*" vs
  rulesync's "makes it portable" stays the positioning sentence
  (`00-intent.md`).
- **The quickstart is the wedge itself**: the first command shown runs a bare
  `temper check` against the reader's own `.claude/` with zero declaration
  and produces real findings — the growth loop knip/ruff/oxlint proved ("run it
  on your repo, it finds problems immediately"), shown before any install
  ceremony where possible.
- **Honest status is a trust feature**, not a confession: a pre-1.0 version
  policy stated plainly (the `ty` model), a CHANGELOG at root, small frequent
  tagged releases with prebuilt binaries — cadence is the loudest alive-signal.
- **`AGENTS.md` at root** (with `CLAUDE.md` sourcing it) — the 2025-26
  convention for agent-readable contributor docs, adopted by every top-tier
  tool. For *this* project it is also self-description: the primary
  contributor it instructs is the pipeline's own agents.

### Decision: the hero visual is the diagnostic, not a benchmark

**Chosen:** the README's one hero visual is a **rich diagnostic** — a real
`check` finding rendered by `miette`, guidance attached, produced by the exact
command captioned under it. Speed charts sell "faster incumbent" tools; temper
has no speed story and needs none — its product *is* the verdict and the
teaching moment (`50-distribution.md`, the gate teaches), and the research
finding is that even the best linters under-exploit this (their READMEs sell
speed, never the diagnostic). The demo asset is **committed and regenerable**
(a scripted terminal-capture pipeline in-repo, rerun when output changes) — the
demo is a *projection* of the tool's real output, never a hand-curated
screenshot that drifts. **Rejected:** a benchmark hero (no claim to make); a
one-off screen recording (drifts from real output — the exact silent-drift
failure the tool hunts, as marketing).

## The positioning burden — why not the rule-pile linters

Harness linters exist (claudelint, cclint et al.). The front door must carry
the differentiation in one breath: a linter ships *its* opinions about your
files; temper ships a **model** — your harness becomes a typed, composed,
projected artifact with requirements you declare and a graph you can traverse.
"ESLint for `.claude/`" is the category the reader knows; "tsc, not ESLint" is
the twist that names the difference. The wedge demo earns the claim: the
linters nag about files; `temper check` also answers *what fills your
requirements and what would strand them*.

## The community surface — small, honest, gated

Launch set (all root-level, all in build's writable paths):

- **`LICENSE-MIT` + `LICENSE-APACHE`** (Decision below).
- **`CONTRIBUTING.md`** — small and honest: how to file a good issue, "search
  first," PRs by prior discussion, and the AI-authorship policy (Decision
  below). It is expectation-setting, not an invitation to co-maintain.
- **`SECURITY.md`** + private vulnerability reporting enabled — with an
  explicit evidence bar for reports (the curl lesson: unverified AI-generated
  vulnerability reports are the new spam; a report must demonstrate, not
  speculate).
- **Issue forms**: one bug form (version, OS, repro), one feature form.

Deferred until demand exists: GitHub Discussions (a second inbox that looks
abandoned when empty), a code of conduct (an enforcement commitment made when
there is a community to enforce it for, not boilerplate before), stale-bots
and label taxonomies. Deliberately *never* at launch: unguarded
`good-first-issue` bait — in 2026 it attracts generated slop, not
contributors.

### Decision: the license is MIT OR Apache-2.0, dual, before launch

**Chosen:** relicense from bare MIT to the Rust-ecosystem dual **MIT OR
Apache-2.0** (Apache's patent grant + MIT's GPLv2 compatibility; the API
guidelines' recommendation; uv/Biome's choice) — **now, while there are no
external contributors**: relicensing later means chasing consent from every
contributor (the wgpu/ratatui campaigns). **Rejected:** staying MIT-only
(fine — ruff/oxc do — but the dual grant costs nothing today and buys patent
cover); anything copyleft (a gate people must adopt voluntarily cannot carry
adoption friction in its license).

### Decision: the AI-authorship policy is two-sided disclosure

**Chosen:** one policy, both directions. *Outbound:* the project states its own
authorship plainly — this codebase is largely agent-built under human-authored
specs and gated commits, with the commit trailer as the provenance record and
the flume history as the audit trail. Stated in the README and CONTRIBUTING as
fact, not fine print — in 2026 this is a normal, defensible statement, and for
*this* project it is the product's own thesis (the gate exists because agents
author harnesses). *Inbound:* AI-assisted contributions are welcome **with
disclosure**, and the contributor must understand and be able to defend the
change without the assistant (the Ghostty line: disclosure-not-prohibition;
"if the human effort is less than the review effort, don't submit").
**Rejected:** a ban (hypocrisy — the repo is agent-built); no policy (the
review-burden asymmetry lands on the sole maintainer). A project that is
agent-built has standing to demand disclosure *only because* it discloses.

## The launch posture — ecosystem first, relaunch on capability

Sequencing is plan's; posture is contract:

- **Soft-launch to the niche before the general stage.** The audience lives in
  the Claude Code ecosystem: the curated awesome-lists (submission requires the
  repo public ≥1 week — publish early), the plugin marketplaces (the companion
  plugin, `50-distribution.md`, is the zero-friction on-ramp and the native
  discovery rail), the community subreddits and Discord. General channels
  (Show HN, This Week in Rust — which requires a written article, so the
  launch blog post is the admission ticket) come second, carrying the social
  proof the soft launch produced.
- **The wedge demo is a findings table over famous public harnesses**: run
  the bare `temper check` against the most-starred public Claude Code setups and
  publish what it finds — temper's CPython moment, reproducible by any reader
  on their own repo with one command. Show, never claim.
- **Every shipped capability is a relaunch moment** (`emit`, `explain`,
  `bundle`) — rolling discovery beats one launch day, and the channels reward
  substance-per-post.

### Decision: no launch before the quickstart survives a stranger

**Chosen:** the launch gate is mechanical: prebuilt binaries install on all
three OSes without a Rust toolchain (the audience is Node-side;
`cargo install` is the worst on-ramp, `50-distribution.md`'s channels are the
real ones), the zero-config wedge produces real findings on a clean machine,
and the demo asset regenerates from the shipped binary. A dead quickstart is
the one documented launch-killer every postmortem shares. **Rejected:**
launching on the README's promise ahead of the binary's reality — the offering
inherits the gate's own law: fail loud, never wave through (`50-distribution.md`).

## The name

The name collides on every registry that matters: the `temper` crate is taken
and active, npm's `temper` is squatted-dead, and a funded company ships a
programming *language* named Temper in the same developer-tooling category,
with the `temper-*` crate prefix already theirs.

### Decision: the name stays `temper`, carried on scoped registries

**Chosen:** keep the name — a provisional keep, **reaffirmed at launch**, the
last moment a rename is cheap. Public registry surfaces use uncontested forms:
the crate publishes as `temper-cli` (the binary stays `temper`), npm is scoped
(`@<org>/temper`) or prefixed, Homebrew rides the project's own tap. None of
the contested entries are load-bearing for the audience, which installs via
npm / brew / the plugin (`50-distribution.md`), not `cargo install`. The costs
are accepted eyes-open: `cargo install temper` installs someone else's crate;
search mindshare is shared with Temper-the-language; and the trademark
exposure stands until checked — a USPTO screen (and ideally a non-objection
from Temper Systems) remains the due-diligence item before launch.
**Rejected (for now):** a pre-launch rename — the identity is worth more than
the contested registries, and the audience discovers through the Claude Code
ecosystem's channels, not crate search. Revisit only if the due-diligence item
turns up a registered mark or confusion materializes. (Resolves
`(project-name)`.)
