---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
paths: ["README.md","CHANGELOG.md","sdk/README.md","docs/**/*.md",".github/**"]
---
# Public-facing prose: reader first

Path-scoped to the surfaces a visitor reads: the READMEs, `CHANGELOG.md`,
`docs/`, and `.github/`. Inside `docs/`, `ledger.md`, `horizons.md`, and
`market-formats.md` are internal working documents, exempt from this
register.

## Voice

- Lead with what the reader can do. Self-description is a tagline and a
  short gap statement (what adjacent tools don't cover); usage comes before
  theory.
- Show real output. Run the built binary and paste or render what it prints,
  never output typed from memory. An example condensed from a real file in
  this repo beats one invented for the page.
- Plain sentences, no em dashes. Use a comma, colon, semicolon, parenthesis,
  or a sentence break instead.
- Natural register, never pitchy: no marketing adjectives and no assistant
  tics (simply, seamlessly, powerful, dive in).
- Tables for enumerable facts (commands, flags, formats); prose for
  everything else.

## What stays out

- Project narration: release philosophy, development-process exposition,
  self-referential thesis talk. State what the tool does, not what the
  project believes about itself. The one disclosure that stays is
  AI authorship, and CONTRIBUTING owns the full statement.
- Meta-captions explaining how an asset was produced.
- The spec corpus as a table of contents. Link `docs/` pages as the primary
  navigation and the corpus once, as the operational definition.

## Verification

- Docs defer to specs. A public page summarizes; when a page and a spec
  disagree, the spec is right and the page has a bug.
- Every stated flag, version, path, and link is checked against the built
  binary or the file on disk before it lands. This is the `collaboration`
  rule's external-facts bar applied to the project's own claims.
- An npm-facing page (`sdk/README.md`) describes the shipped package;
  install and platform claims are verified against the live registry.
