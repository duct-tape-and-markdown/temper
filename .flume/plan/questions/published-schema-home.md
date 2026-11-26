# Which published package carries the version-pinned schema the unrepresented modeline names?

`specs/distribution.md`, "The placements and their enforcement modes" (Keystroke, 0108): the unrepresented modeline points at "the schema the engine's npm package ships for its own version (a version-pinned CDN URL)". Code today: install places no modeline on an unrepresented harness (`src/install.rs` `place_settings_only`), and no package ships schema JSON.

Two readings, each consistent with the corpus:
- **the platform engine packages** (`@dtmd/temper-<platform>`): literally "the engine's", but there are several, and they are assembled only in `release.yml`, which is human-only, so build cannot land it;
- **the SDK** (`@dtmd/temper`): distribution ch. 1 calls it the one npm package, it pins the engine at the same version, and its `files` list is in build's fence (the schemas committed and byte-compared like the built-in lock).

Lean: the SDK. Also uncited: the CDN host and its URL grammar (jsDelivr, unpkg). The modeline entry waits on both answers.
