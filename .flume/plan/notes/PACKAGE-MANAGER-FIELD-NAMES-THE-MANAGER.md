# packageManager names the manager: shipped, scaffold half included

`tool::declared_manager` is now the one reader of `packageManager`. It owns
`PackageManager::{lockfile, program}`. The SDK's `readLockfile` agrees with
it, and the lockfile read looks only for the declared manager's lockfile.

**The install scaffold half shipped here too, deliberately.** The refusal of a
fieldless `.temper/package.json` would break every fresh install unless the
scaffold writes the field. So the scaffold writes the root's
`packageManager`, else `npm@<npm --version>`, and install spawns the declared
manager. Inbox `2026-11-26-install-writes-the-package-manager` still wants
its seam, but its two "new" lines now pass on the base. File them as pins,
not tests[], or they revert. A test titled
`a_root_package_manager_field_naming_pnpm_is_copied_and_install_runs_pnpm`
already sits in `tests/it/install.rs`. It drives a stub pnpm through a
shadowed PATH, and the judge reported it unnamed. Find out why before
re-filing; a `cfg(unix)` gate is one suspect.

Other behaviour changes:
- A `yarn.lock` beside an npm program no longer refuses. yarn and bun are
  refused by the field's name instead. `npm-shrinkwrap.json` still refuses.
- The workspace-root walk still reads both managers' workspace
  declarations. Narrowing it to the declared manager's is open.
- The PATH-shadowing tests now pass PATH to the child process, and
  `PATH_MUTEX` is gone. A process-wide `set_var` raced the concurrent
  `npm --version` spawns.

The migration note owes this: a fieldless `.temper/package.json` now refuses
at install, emit, `check` and `explain`. The remedy is `"packageManager":
"npm@<version>"`. The `missing` render line still says "your package
manager".
