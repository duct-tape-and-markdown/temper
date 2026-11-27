# A relocation that adds exclusions collides with members built from its own built-in

Found adopting `playbook()` in this repository's harness. `.temper/kinds.ts` relocates `rule` with `exclude: ["tests/fixtures", "examples/demo"]`, the use `specs/builtins.md` ("Built-ins — what ships") names: "An author may add exclusions to a built-in through its relocation … a relocation, which replaces the built-in there." `playbook()` builds its rule, skill and topics from the built-in constructors, so the harness holds a member of the base `rule` beside the relocation. Emit refuses: "two kinds named `rule` are in play, at `**/.claude/rules/**/*.md` and `**/.claude/rules/**/*.md`".

The cause is in `sdk/src/declarations.ts`. `relocationInvariants` blanks a relocated locus's `root` and `glob`, but keeps its `exclude`. So an excluding relocation fails `isRelocationOf`, and `resolveNameCollision` treats it as a third kind of the same name. Any shape that builds a built-in member (the playbook, `defaultPatterns()`'s reference skill) breaks the same way for every adopter who excluded a fixture tree.

Acceptance (SDK tests, so no `tests[]` line per PROTOCOL, Tests, question 2):
- **new:** a harness relocating `rule` with an `exclude` emits beside a member built from the built-in `rule`, and the member is governed by the relocation;
- **kept:** two relocations of one kind that diverge still refuse.
