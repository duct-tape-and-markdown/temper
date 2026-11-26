# A fully import-reached directory is claimed

Shipped: `coverage_note::fully_reached_dirs` walks `.claude/` once under the
stray scan's ignore policy and claims a top-level directory when it holds at
least one file and every file beneath it is in `reached_files`. An empty
directory stays unclaimed (vacuous "every" would silence it); a gitignored
file neither claims nor blocks.

Observed:
- Import parsing keeps a trailing sentence period in the path:
  `See @.claude/notes/a.md.` resolves to `a.md.` and the import is reported
  unbacked. Whether Claude Code's own `@path` parser strips trailing
  punctuation is UNVERIFIED; if it does, temper forges an unbacked-pointer
  finding on ordinary prose. Worth a cited check against the memory docs.
