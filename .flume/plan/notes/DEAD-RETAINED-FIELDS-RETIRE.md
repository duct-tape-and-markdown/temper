# DEAD-RETAINED-FIELDS-RETIRE shipped

Both symbols are gone: `CollectionAddress.manifest` and `install::GATE_COMMAND_BINDINGS`. The diff is net-negative and every gate passed locally.

`collection_address_from_row` still rejects a row whose `governs_glob` is missing or starred. It now only validates the glob and stores nothing. The single-manifest locus invariant stays enforced at load, and `CustomKind::manifest` stays the one place that reads it.

`RegistrationRow.manifest` (the lock column) is a different field and was left alone. A mechanical sweep that matches `manifest:` followed by `key_path:` hits those literals too, so scope any such sweep to `CollectionAddress {`.
