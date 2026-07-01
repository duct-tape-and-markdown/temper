<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- `specs/55-offering.md` names fileable root-doc build work not yet in pending:
  README rewritten to the front-door contract (Decision: hero is the diagnostic),
  `LICENSE-MIT` + `LICENSE-APACHE` (Decision: dual before launch; Cargo.toml
  `license` field follows), `CONTRIBUTING.md` (two-sided AI policy Decision),
  `SECURITY.md`, issue forms. All in build's writable paths (root docs,
  `.github/**`). Independent of the machinery chain; sequence at your judgment.
- `specs/10-contracts.md` ("named for its source, and cited to it") adds a
  per-clause `source` citation key to the package format — the PACKAGE.md
  loader (PACKAGE-DOCUMENT's) needs to parse/preserve it; small entry, needed
  before the packages/ std-lib authoring session consumes packages/DOSSIER.md.
