# Root face carries no Claude Code built-in kind — pinned

Shipped as `the package root exports no Claude Code built-in kind` in
`sdk/test/root_exports.test.ts`: it imports both faces at runtime and
compares kind values (functions carrying `facts` + `key`) by identity, so
a renamed re-export is caught too. Verified red by appending
`export { skill as skillKind }` to `src/index.ts`.

Observed: `tool` is exported by both the root and the provider face (since
9470198b, deliberately — the core's embedded kind, model/authoring.md). The
test names it as the one core exception. builtins.md's "never the root
export" reads over all shipped kinds; if `tool` (or `requirement`/`dial`,
root-only) should be worded as core kinds outside that sentence, that is a
spec wording question for a human, not a build change.
