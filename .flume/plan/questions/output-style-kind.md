# Output styles: a built-in kind, or named unmodeled?

Driver: a real-harness probe (0.0.25). An output-styles folder that settings select by name draws `coverage.unclaimed-entry`.

Verified (code.claude.com/docs/en/output-styles, retrieved 2026-11-25): a project style lives in `.claude/output-styles/`, loaded from each such folder between the working directory and the repo root. It is "a Markdown file: frontmatter for metadata, then the instructions". The optional fields are `name`, `description`, `keep-coding-instructions` and `force-for-plugin`. "The file name becomes the style name unless you set `name`". Settings select it with `outputStyle`, an exact, case-sensitive name match. `/output-style` writes the selection to `.claude/settings.local.json`.

`specs/builtins.md`, "The coverage bar", entails a kind: it is a documented artifact surface. The selection is a settings edge to it, but the setting is usually written to the local settings file. Kinds are a human corpus amendment (`builtins.md`, "The shipped kinds"), so plan files no entry until one lands. One more point is open: whether the `outputStyle` edge is declared on `settings`, on `settings-local`, or on both.
