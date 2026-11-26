# Install's scaffold writes `packageManager`, testable without a real package manager

This is split from PACKAGE-MANAGER-FIELD-NAMES-THE-MANAGER, whose install half twice drew "no test" from the judge. The tool half stays in that entry. The rule is `specs/model/authoring.md`: the scaffold writes the repository root's own `packageManager` where that is declared, otherwise the manager install ran, with its version.

Install spawns a real manager (`spawn_npm_install`, `src/install.rs`), which a hermetic integration test cannot run. Give the spawn a seam the tests can drive: the manager command resolved through one function that a test points at a stub script recording its argv. The lines are then provable under `tests/it/install.rs` (PROTOCOL, Tests). Acceptance:
- **new:** a fresh install writes `packageManager` into `.temper/package.json`;
- **new:** with a root `packageManager: "pnpm@…"`, the scaffold copies it and install invokes `pnpm` (observed through the stub);
- **kept:** an existing `package.json` stays byte-identical on re-install (already pinned).
