# Install report shows the contract

Shipped: `check`'s gate-or-load-fault branch moved out of the `Check` arm into
`gate_verdict` (src/main.rs), which `install` now calls with no layers before
rendering discovery; `render_discovery` takes the findings and renders them via
`check::render` under a `contract:` heading after the per-kind counts.

Observed: on a represented harness the report shows the lock's contract, not the
built-in default — it is bare `check`'s verdict, which the entry asked for; the
spec's "built-in default contract" wording reads as the unrepresented case only.
Install now walks the tree twice (discover + gate); the two walks are not shared.
`cargo doc` prints pre-existing private-link warnings in src/admissibility.rs.
