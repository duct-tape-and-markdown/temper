# INSTALL-RUN-HOLDS-ONE-OWNER-PER-INPUT

Shipped as the interface states: `install::execute(root, workspace, dry_run, decide)`
runs discovery, the gate verdict and the placements over one `import::Discovery`; the
`decide` closure is main's question (render, settled-fork match, prompt). Gate gained
`gate::gate_in` (caller-held Discovery) and `gate::verdict_in`, which absorbed main's
load-fault lowering and harness_root re-rooting, so `check` and `install` share one
verdict function (main's `harness_diagnostics` is gone).

Beyond the named files: `coverage_note::script_note` read settings.json raw via
`bytes::open_to_string` — the last reopen on the pin after threading. It now takes the
run's `Bytes`. That reopen hit `check` too on an unrepresented harness whose settings
the settings kind reads; check's existing pin fixture never reached it.

`discover_in` reads collection manifests through the owner (was `Manifest::read`, raw).

Left standing (Represent::Yes, outside acceptance): `evaluate_placements` after emit
keeps a fresh `Bytes` on purpose — `drift::emit_program` writes through its own owner,
so the run's held bytes are stale there. Threading one owner through emit_program would
close it. `hoist_settings_residue` (install.rs ~3925) still reads `harness.ts` through
an ad-hoc `Bytes::new()`, and `plan_lift`'s member reads were not audited.
