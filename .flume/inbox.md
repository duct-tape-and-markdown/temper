<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-03 (human): DIRECTIVE-PATH-NORMALIZE — first real @import in the
  repo (CLAUDE.md imports @docs/ledger.md, both committed) fires a FALSE
  unbacked-pointer advisory: the member's provenance source_path is written
  unnormalized ("././CLAUDE.md" — import_custom_unit joins harness_path "."
  raw), so relative target resolution yields "././docs/ledger.md" which
  string-mismatches the repo_file_set entry "docs/ledger.md". Law 3 in the
  forbidden direction: the classing forges a finding on a backed edge.
  Normalize path components on BOTH sides (provenance at write, resolution
  at compare — belt and suspenders; a lexical normalize, no fs canonicalize,
  round-trip discipline). Live repro: `temper check` on this repo — the
  graph.directive-unbacked advisory on `CLAUDE` should disappear; deleting
  docs/ledger.md should bring it back (that canary is the ledger's
  enforcement working as designed).
