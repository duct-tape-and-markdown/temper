# BUNDLE-STRIPS-THE-MANAGED-NOTE

Shipped. `bundle::strip_managed_note` (pub, so the byte-equality pin can state the
expected written bytes) drops the `---\n<NOTE_COMMENT>\n` frontmatter note or a leading
`BANNER` via `placement::strip_leading_banner`, keyed on placement's constants only.
`src/bundled_reference/` is untouched, so the SDK byte-equality pin holds; its tail
assertion now compares the written file to the stripped embedded bytes. The determinism
snapshot lists file paths only and did not move.
