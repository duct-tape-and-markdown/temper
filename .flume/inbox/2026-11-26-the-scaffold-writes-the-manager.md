# Install's scaffold writes `packageManager`: the root's declaration, else the manager it ran

This answers PACKAGE-MANAGER-FIELD-NAMES-THE-MANAGER's park (5d068260) under 0100's own principle that temper never guesses the toolchain. `specs/model/authoring.md` now says what the scaffold writes:
- the repository root's own `packageManager`, where that is declared (a documented field the consumer owns, read and not guessed);
- otherwise the manager install itself ran, with its version (`npm@<version>`). That is a record of what produced the lockfile, and the consumer may change it.

A `.temper/package.json` with no field is then a harness written before this rule. Refuse it with the remedy (add the field), and leave existing harnesses to the migration note. Acceptance:
- **new:** a fresh install writes `packageManager` into `.temper/package.json`;
- **new:** with a root `packageManager: "pnpm@…"`, the scaffold copies it and install runs pnpm;
- **kept:** an existing `package.json` stays byte-identical on re-install.
