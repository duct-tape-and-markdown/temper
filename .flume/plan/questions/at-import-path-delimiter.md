# Where does an `@path` import's path end?

Driver: a shipped build note (A-FULLY-IMPORT-REACHED-DIRECTORY-IS-CLAIMED). `See @.claude/notes/a.md.` extracts `.claude/notes/a.md.`, so the import reads unbacked. Verified at HEAD: `src/extract.rs` `line_at_imports` takes the path as the whole non-whitespace run after `@`.

What the sources say (code.claude.com/docs/en/memory, "Import additional files", retrieved 2026-11-25):
- Trailing punctuation: nothing. The page's examples never end a path with punctuation.
- "Without the backslashes, the path ends at the first space"; `@Design\ Docs/api-conventions.md` imports a path containing a space. temper's reader stops at the space and yields `Design\`. That is a second, documented mismatch in the same grammar.

`specs/model/contract.md`, "edge" (import directive), says an import is a reference the format itself executes, resolved by path, and `builtins.md` says format facts are documented truth. The documented rule doesn't settle trailing punctuation. Stripping it would wrongly cut a real `a.md.` file. Keeping it gives a false unbacked pointer on ordinary prose. Decide which binds: (a) a lab probe of Claude Code's parser, (b) keep the literal run, or (c) strip trailing sentence punctuation. Then file one entry for the delimiter grammar, covering the backslash-space escape too.
