# Does the guard judge contract clauses over layout source documents?

Driver: GH #62, the guard half of LAYOUT-ENTRIES-READ-OFF-THE-DOCUMENT-AT-CHECK (re-scoped to check alone). The report names "both guard edges let the edit through" as a symptom, but under the standing corpus that is specified: `specs/distribution.md`, "Per tool call", binds projection drift and undeclared documents where `locus-declared` is bound; 0060 rejected judging "the whole contract instead of the write boundary" per call. No guard edge judges a required clause on any layout document today, fields or entries. The one precedent, `install::manifest_write_findings`, judges a pending co-owned manifest's members against their contract, pre edge only.

Options: (1) keep the guard at the write boundary; check alone catches a layout edit that breaks a required clause. (2) The pre edge judges a pending layout source's own contract, as it does a manifest's; the post edge stays as is. (3) Both edges, which reopens 0060.

Recommendation: 2. A layout document is the one governed source an agent writes directly, and the manifest precedent already judges a pending source's members at the pre edge. That costs one document's clauses per write, not the tree's. Ruling is a `specs/distribution.md` "Per tool call" amendment first.
