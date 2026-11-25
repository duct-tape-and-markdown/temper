# Guard binds nested layout children: pending creation now binds

Finished the remainder the continuing note named. `import::nested_file_placement` is the
single-path placement test over check's host units. `nested_file_members` and the
placement now share one host loop (`kind_host_units`, plus `HostUnit::child`). So there
is no second host derivation, and none in compose.rs. `LayoutGuard::hosted_member` lost
its `is_file()` short-circuit. A pending write is placed whether its file exists or not,
and `hosted_member_address` addresses it.

Left undone: a new-file case in the gauntlet pre-edge list. `tests/it/gauntlet.rs` was
outside this tick's fence, and `tests/it/install.rs` carries the pin. A later entry may
add it there if gauntlet coverage of creation matters.

Two observations:
- The placement checks discoverability with `discovers_path`, which fences the surface
  workspace with `local_governs = false`. The scan's index honors a host's `local`
  commitment. So a hosted child under a `local` host inside `.temper/` would diverge
  between the edges. The governed pending branch has the same gap.
- The `cargo doc` gate exits 0 over about 130 pre-existing private-link warnings, so it
  pins nothing about doc links.
