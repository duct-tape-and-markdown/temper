<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-04 (John, session ceremony): the four SDK dogfood-migration seam
  forks are RULED — each on the filed recommendation:
  - `(sdk-placement-round-through)`: (a) — SDK emit reads the committed
    projection and rounds install's placement lines through, mirroring the
    Rust EMIT-OWNED-PLACEMENTS resolution; the two projectors agree by
    construction.
  - `(gate-kind-spelling-and-unknown-kind)`: the gate resolves a qualified
    kind to its bare key before lookup (identity is `<provider>.<kind>`,
    `15-kinds.md`), AND an unrecognized kind in the manifest is a loud
    finding — never a silent `checked 0`.
  - `(gate-reads-assembly-artifacts)`: (a) — the gate learns to read
    `roster.toml`/`bindings.toml` as the assembly source; the artifacts
    exist by ratified ruling, the gate is the side not yet reading them.
  - `(module-carriage-manifest-shape)`: converge — module carriage
    serializes per-heading exactly like the importer ("three carriages, one
    feature shape"; every consumer stays carriage-blind). The `line_count`
    +1 trailing-newline bug is fileable within this shape.
