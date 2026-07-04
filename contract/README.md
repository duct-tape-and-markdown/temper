# `contract/` — the shared interchange corpus

The one golden set both implementations test against. `temper` ships a Rust
toolchain and a TypeScript SDK that must agree byte-for-byte on the manifest the
gate reads (`specs/architecture/50-distribution.md`, "the SDK is … versioned
against the interchange schema … both implementations tested against one golden
set"). Rather than each suite carry its own transcribed fixtures — two copies
that silently drift — both read from here.

## What lives here

- **`manifest/*.toml` — byte-parity manifest goldens.** One `[[member]]` block
  per fixture (`rule`, `skill`, `decision`, `memory`, `empty-rejected`) plus the
  kind-then-name-sorted multi-member `corpus.toml`. Each is the exact output of
  the Rust emitter (`write_manifest_members`, `src/compose.rs`) over a member —
  key order, table-vs-header choice, string escaping (basic / literal /
  multiline), blank-line placement, and array spelling all pinned. These are the
  **extraction goldens** too: a serialized member is the pre-extracted `Features`
  form, and every golden round-trips (parse back to members, re-serialize,
  byte-identical), so the write shape and the read shape are proven one.

- **`schema/manifest.schema.json` — the interchange JSON Schema (2020-12).**
  Generated Rust-first from the manifest IR (`ManifestMember` and the `Features`
  it carries) via `schemars`. It describes the **typed IR** — the nested Rust
  shape (`ManifestMember { kind, features }`, `Features.id` / `body_lines` /
  `published_requirements`, the tagged `FeatureValue`), the source of truth the
  gate actually reads — not the flattened TOML wire the byte goldens pin. The two
  are complementary: the goldens are the **byte** contract of the wire, the schema
  is the **type** contract of the IR, and both are versioned together as one set.

- **`schema/manifest.d.ts` — the interchange TypeScript types.** The same IR
  graph generated Rust-first via `ts-rs`, so the type contract has a TS spelling
  with no hand-transcription to drift.

## The producer, and the discipline

The Rust suite `tests/contract_fixtures.rs` is the **single producer**: it builds
each member as a typed value, emits it through the one Rust serializer, and
byte-matches the result against the committed golden; it also generates the schema
and TS artifacts (schemars / ts-rs). The SDK suite `sdk/test/emit.test.ts`
**consumes** the same `manifest/*.toml` files — a one-byte divergence in either
serializer fails a suite.

**A fixture edit is an interface-version event.** `contract/**` sits inside the
flume build fence (`.flume/chain.ts`), so changing a golden is a deliberate change
to the wire both implementations speak, never an incidental churn. After an
intentional emitter change, regenerate every committed artifact with:

```
BLESS=1 cargo test --test contract_fixtures
```

and review the diff as the interface change it is.
