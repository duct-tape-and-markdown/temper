<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# An export earns its consumer

Public surface with no consumer is residue: an export born as
scaffolding outlives its scaffold and becomes API someone must
excavate later. Grep-verifiable, so the sweep holds it mechanically:

- An SDK root export, a `pub` item, a widened `pub(crate)` — each needs
  a caller outside its own module (a test counts), **or a re-export
  from the package's public entry point** (`sdk/src/index.ts`) — the
  entry point is itself the consuming edge for a symbol shipped as
  public API; a pre-adoption library's downstream callers are real even
  when none live in this repo yet. Zero-consumer surface is fileable
  against this section only when neither condition holds.
- A seam opened for a planned second consumer carries that consumer's
  name at the export site; when the plan dies, the export dies with it.

**Tell:** a `pub` item or SDK export with no caller outside its module and
no public re-export. **Held by:** the sweep's grep, a mechanical read.
