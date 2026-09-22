<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->





## explain kind narrates built-in kinds with no fact row behind them — observed at 4b25d0f3

Drained from friction capture build-explain-kind-sees-no-builtin-fact-row
(human-routed). `explain_target` sources kind fact rows from
`declarations.kinds` (src/read.rs:2436, off the lock family at :2304), which
carries only the kinds this surface's lock declares. A built-in kind the
surface has no member of still resolves, through its default contract, but
narrates with nothing behind it: `temper explain kind:agent` prints no locus,
cite, registration, commitment, template or address form. The same holds for
`command`, `mcp-server`, `settings-local`, `known-marketplace`,
`installed-plugin` and `dial`. Guidance is unaffected; it rides `contracts`.
This is the adopter's moment the locus and address-form strands were built
for: no member yet. `compose::assemble_lock_family` already returns
`overlaid_builtin_kinds` beside `declarations` at that call site, so the fix
is to read the built-ins' own rows under the lock's, with the lock's winning
where both exist. Pre-existing, not a regression.

## The import relation stops at memory, but Claude Code recurses through import targets — observed at 4b25d0f3

Drained from friction capture build-import-relation-is-memory-only
(human-routed, question for the corpus). Only `memory` composes the
`at-import` directive primitive (src/builtin_kind.rs:322), and no lock
`KindFactRow` can spell `Directives`. So an `@`-line in any other kind's
body is extracted by nobody. Claude Code: "CLAUDE.md files can import
additional files using `@path/to/import` syntax … Imported files can
recursively import other files, with a maximum depth of four hops"
(code.claude.com/docs/en/memory, retrieved 2026-09-22). The docs are silent
on `@`-lines in a rule loaded as a rule. But a file reached *through* an
import has its own `@`-lines expanded, whatever its kind. So CLAUDE.md →
@.claude/rules/r.md → @CLAUDE.md is a ring the runtime walks and temper
cannot see: `acyclic` and `reachable` both miss the middle hop.
`contract.md` "edge" names "a memory file's `@path` import" as its example
of the directive locus. Whether the relation covers directives in any
import target is a corpus question, so route it to open-questions and do not
build on it. Acyclicity shipped on the reachable memory ↔ memory half
(09ad535a).
