# Nested rules shipped; root-relative locations are an adopter break

`rule` now claims `**/.claude/rules/**/*.md` at root `.`, so a declared
`location` is spelled from the harness root. A lifted root rule in a
subfolder (`frontend:react`) used to declare `location: "frontend/react.md"`
under the old `.claude/rules` root. After this change emit refuses it with
MemberLocation until it is re-lifted or rewritten as
`.claude/rules/frontend/react.md`. The release cut owes a migration note.
This repo's `.temper/` lock still carries the old rule row, which reads as a
relocation onto the old locus, and it stays green until a human re-emits.

Matchers beyond the named files followed the grammar:
- `sdk/src/kind.ts` `templateMatches` / `templateReachesBeneath`
- `src/kind.rs` `template_reaches_beneath`
- the SDK `spliceName` twin

Each assumed one `**`; each is now recursive over any number.
`scan_locus` dedups, because two `**` can reach one file twice.

The coincidence finding (`member.admissibility`) now names the paths its
members were read from, which is how "refused naming both paths" ships.

Identity: in a file-shaped kind whose template names a literal folder after
a `**`, identity joins the `**` spans and leaves the literal folders out.
Any custom kind of that shape (`**/notes/*.md`, file shape) now reads
`apps:x` where it used to read `apps:notes:x`, per representation.md
"locus". No test pinned the old reading.

Because `rule` now reaches into subtrees at any depth, it also cedes custom
directory hosts' unit subtrees (gauntlet: `docs/guides/*/**` to `guide`),
the same way `memory` does.
