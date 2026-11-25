# check compares the ownerless registration hash

Shipped. `config_stale_from_doc` now ends with `drifted_ownerless_registrations`,
which lifts the lock's registration rows, collects the manifests a hashed row
targets, and runs the same `drifted_registration_entries` emit's refusal runs —
its `build: &ManifestBuild` parameter narrowed to `rendered: &[RenderedEntry]`,
`check` passing `&[]` (no render offline). One comparison, two callers.

`src/gate.rs` needed no change: the judge already holds the lock doc and the
harness root. The guard's post edge (`install.rs`, `config_stale_from_doc`)
inherits the finding with no edit.

Observed: the fresh judge now lifts `declarations_from_doc` a second time per
check (gate.rs already lifts it elsewhere). Cheap at kilobyte locks; if the gate
ever threads a lifted `Declarations`, this judge is a consumer.
