<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

#13 — `schema`'s kind domain is a two-kind fossil; every other kind's guidance
is unreachable at the keystroke surface. `temper schema` refuses all but
`skill` and `rule` (`src/main.rs:46` `BUILTIN_DEFAULT_CONTRACT_KINDS`;
its error text "temper models: skill, rule" is false today). The spec states
no restriction — `distribution.md:44`, "generates a JSON Schema from the
compiled clauses" — and `contract.md`'s guidance clause (20a6f54, "delivery
follows the author") now binds `schema` for every kind. Downstream is
already generic: `install` places a modeline for any kind whose
`.temper/schema/<kind>.json` exists (`src/install.rs:674`), keyed by name,
no allowlist. Upstream, agent/hook/command/plugin-manifest/dial/
supporting-doc and every adopter-declared kind carry authored `guidance`
lowered to the lock and unreachable — #12's class, authored-but-unsurfaced.
Ruled (interactive, 2026-07-23): **build-ready for the widening half** —
`schema` serves every kind in play, builtin and declared; cite
`distribution.md:44` plus the contract.md guidance clause. One face is NOT
build's to invent: the modeline mechanism is frontmatter-YAML only, and
keystroke wiring for a JSON document (settings, plugin-manifest) or the
dial's TOML is unspecced — ship the YAML-frontmatter kinds now and surface
the JSON/TOML wiring as its own face (a distribution.md note or a fork),
never half-invent it. observed at fc77716.
#14 — `install`'s scaffold `member_dir` is frozen at the skill/rule/memory
era, and the catch-all it dumps everything else into is a silent-overwrite
hazard. `src/install.rs:1238` special-cases `skill`/`rule`/`memory` and
routes every other kind to `_ => "members"`. `scaffold`
(`src/install.rs:1268`) iterates every `Content::File` kind generically
(10 of the 14 builtins), so command/agent/supporting-doc/marketplace/
plugin-manifest/settings-local/dial members all land in that shared bucket,
keyed by bare `member.id` with no kind segment (`src/install.rs:1300`)
and written with no collision check (`write_creating_parents`,
`src/install.rs:1465`). Decision 0014 keeps command and agent as separate
identity spaces, so a command and an agent sharing a name (both "deploy")
scaffold to the same `members/deploy.ts`: one silently clobbers the other
on first `temper install`. Era boundary is clean in git: `member_dir`
landed 7102391 (07-06), command/agent kinds landed one day later (bdc938d),
the table never grew; scaffold tests exercise skill/rule/memory only, so
the collision path is untested. Ruled (interactive, 2026-07-23):
**build-ready** — no spec clause or Decision rules scaffold directory
naming (0014 is silent on the lift), so the fix is implementation domain:
a kind-named directory per `Content::File` kind (drop the catch-all or
make it collision-proof) plus a regression test that scaffolds two kinds
sharing a member name. observed at 11626a1.
