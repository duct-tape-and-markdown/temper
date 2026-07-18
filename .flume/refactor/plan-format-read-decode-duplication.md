## Surface

The read+UTF-8-decode boilerplate duplication engineering.md's "One job,
one home" targets is wider than the currently-filed
`JSON-MANIFEST-READ-DECODE-CONSOLIDATE` entry scopes (that entry
consolidates only `json_manifest.rs`'s own two sites,
`DocumentMember::read` 145-155 and `Manifest::read` 331-344, into one
file-local private helper). Two more copies of the identical job live
elsewhere in the formats subsystem, verified on disk:

- `src/frontmatter.rs`'s `Member::from_source_rooted` (172-180):
  `fs::read(source_file).map_err(|source| FrontmatterError::Io { .. })?`
  then `String::from_utf8(bytes).map_err(|source| FrontmatterError::NotUtf8 { .. })?`
  — same shape, with a `crate::hash::sha256_hex(&bytes)` call sandwiched
  between the two steps (176).
- `src/toml_document.rs`'s `read` (33-42): the identical
  `fs::read` → `map_err(Io)` → `String::from_utf8` → `map_err(NotUtf8)`
  pair, no hash step.

Four total copies of one job across the subsystem (2 already filed in
json_manifest.rs, these 2 not). Each wraps the same two-step failure into
its own format's error enum (`FrontmatterError`, `JsonManifestError`,
`TomlDocumentError`) — the only per-site variation.

## Observed at

9e197d6 (HEAD when observed — posture sweep, formats subsystem, job 4).

## Suggested consolidation

Not folded into the already-open `JSON-MANIFEST-READ-DECODE-CONSOLIDATE`
entry — that entry's shape (a private, file-local helper) doesn't fit a
cross-format consolidation, and widening it mid-flight would change its
acceptance criteria. A shared primitive here needs to be generic over
each format's own error type (return the raw `io::Error`/`FromUtf8Error`
pair, or take error-constructor closures, and let each format map to its
own enum) and needs a home — a foundation candidate (`src/hash.rs`
already computes `sha256_hex` from the same raw bytes frontmatter reads,
so a `read_and_hash`-shaped primitive there could serve both the hash and
the decode in one read) versus a smaller shared helper each format calls
independently. Land after `JSON-MANIFEST-READ-DECODE-CONSOLIDATE` ships,
so this doesn't race its file-local helper.
