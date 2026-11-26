# plugin-manifest admits the documented keys

plugins-reference (re-fetched 2026-11-26) no longer has `#metadata-fields`,
`#component-path-fields`, `#required-fields` or `#experimental-components`
anchors: the page is now one "Fields" table (`#fields`), with union forms under
"Component path forms" (`#component-path-forms`), and `name` at `#name`. All
plugin-manifest cites now point at the live anchors. `commands`' object form is
documented by the reference itself, so its type clause no longer cites the schema.

Not done here (scope): `workflows` (`string|list`) and `experimental.*` get no
`type` clause, like `outputStyles`; a manifest with a wrong-typed `workflows`
passes. The vendored schema fixture (tests/fixtures/plugin-manifest/schema.json,
2026-07-16) predates these keys, so manifest_schema_oracle does not see them.
The `keywords` type clause still cites `#unrecognized-fields` (2026-07-16); the
type-mismatch rule now sits under "Validate the manifest".
