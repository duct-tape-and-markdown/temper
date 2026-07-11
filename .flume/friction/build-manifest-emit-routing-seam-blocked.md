## Symptom

`MANIFEST-WRITE-EMIT-FACE` (entry 2/5) scoped two edits: the write face in
`json_manifest.rs` (landed) and "engine.rs routing represented manifests through
it" (not landable this tick). Two mismatches:

- **Wrong file.** `src/engine.rs` is the contract-validation engine; it knows
  nothing of manifests. The emit engine is `src/drift.rs` (`emit`/`emit_program`).
- **Seam-blocked.** Entry 1 (`MANIFEST-WRITE-SDK-ERASURE`) erased hook/mcp-server
  members into `RegistrationFact`s but deliberately kept them out of the seam —
  `encodeSeam({ declarations, members })` carries no registrations, and the Rust
  `Payload`/`Declarations` have no such family (its commit body: "no seam/engine
  change this tick"). So `drift::emit` has no represented-manifest instances to
  route: `PayloadMember`s are projected members only (`isProjected` excludes
  `isRegistration`), and the collection-address facts name only which *kind*
  addresses which manifest, never the concrete members that populate it. Routing
  emit through the write face cannot land until the seam carries the registration
  member instances.

Shipped the write face (`write_manifest` + `CollectionSegment`) beside
`json_splice`, which stays the unrepresented path — the half the entry's own
notes frame as the deliverable ("adds the write face beside it, not replacing
it"). The emit-routing half is deferred.

## Cost this tick

~0 reverts — caught before committing red. ~20 min of investigation to confirm
the seam gap rather than fabricate drift.rs wiring that could not be exercised.

## Suggested fix

Re-scope the chain: fold "emit routes represented manifests through the write
face" into the seam-wiring step (the entry that lands the `RegistrationFact`
family in `Payload`/`Declarations` and the reader in `read.rs`), and name
`src/drift.rs` (not `engine.rs`) as its emit home. The write face is ready for
that caller.
