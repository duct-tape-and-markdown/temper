# Install refuses a skill named like the starter reference

Shipped. `scaffold` split into `plan_lift` (read + judge, no writes) and
`scaffold` (writes). `run_represented` calls `plan_lift` before
`ensure_package_json`, so both the new `ReferenceNameCollision` refusal and the
existing `MemberFileName` refusal now land before `.temper/package.json` or any
npm spawn; previously `MemberFileName` fired after the package manifest write.

Debt: `STARTER_REFERENCE_NAME` in src/install.rs duplicates the SDK's
`REFERENCE_NAME` (sdk/src/reference.ts) and bundle.rs's `PLUGIN_NAME`: three
homes for one `"temper"`, no test ties them. A pin asserting the Rust constant
against the built SDK's export would close it.

The check is by lifted skill id only; a starter that grows more fixed-name
members (beyond the gate hooks, already reserved in `module_stems`) owes the
same preflight.
