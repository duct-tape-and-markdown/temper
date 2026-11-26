# Unrepresented install places the CDN modeline

Shipped. `install::run(Represent::No)` now goes through `place_unrepresented`:
the SessionStart splice (`place_session_start`) plus `place_cdn_modelines`. That
seats `# yaml-language-server: $schema=https://unpkg.com/@dtmd/temper@<VERSION>/schemas/<kind>.json`
in every discovered member of a YamlFrontmatter built-in kind. The kinds come
from the same `format` filter `temper schema`'s domain uses, so they are the
SDK's shipped schemas. A frontmatterless file gets no modeline. An existing
modeline is kept verbatim.

CDN choice, cited in `placement::cdn_schema_url`: unpkg, not jsDelivr. unpkg's
homepage documents `:package` as including the scope (`@scope/name`). jsDelivr's
README grammar (`/npm/package@version/file`) documents no scoped spelling.

Not done, for plan to weigh:
- `gate_installed`'s unrepresented face still verifies the session-start hook
  only. It never checks the CDN modelines, because doing so would need a
  discovery walk inside `check`. That face is reached only when `.temper/`
  exists and holds no `harness.ts`. An unrepresented install never creates
  `.temper/`, so in the common case `check` never self-verifies this placement
  at all. Whether that holds up against the spec's line "check verifies its own
  gate is installed" is plan's call.
- A modeline a human pointed elsewhere, or a relative one left by a represented
  past, is never re-pointed at the CDN. Re-runs key on presence only. A
  version bump therefore leaves an old `@<version>` URL standing.
  Content-keying would converge it, the way the note wording converges, but
  that would also re-point a human's own modeline. Plan should decide whether
  the CDN modeline is content-keyed like the other managed lines.
