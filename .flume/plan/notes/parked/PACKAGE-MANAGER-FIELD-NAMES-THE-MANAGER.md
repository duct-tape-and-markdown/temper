# Manager field: the refusal half rests on the open scaffold question

Parked, nothing built. Decision 0100 rules the `packageManager` field "read
and never guessed", so a needed manager with no field must refuse. That is
sound, but every fresh install trips it on temper's own output:

- `ensure_package_json` (src/install.rs) scaffolds `.temper/package.json`
  with no `packageManager` field. A fresh install's deps never resolve, so
  `spawn_npm_install` is needed and the field is missing by construction.
  Install would refuse and tell the author to edit the file it just wrote.
- The scaffolded harness composes `tool({ package: "typescript", bin: "tsc" })`
  (install.rs ~3433). Once the field names the manager, the tool's version
  read (src/tool.rs `read_lockfile`, sdk/src/lockfile.ts) needs it too. So
  the first emit (reference topic) and every `check` (`unlocked_findings`)
  refuse on an unedited scaffold. So do most install tests and
  the dogfood: `.temper/package.json` has no field, so session-open `check`
  refuses, and a build tick can't write `.temper/`.

The entry's notes say to raise the scaffold's manager, not pick one. Without
an answer the refusal makes `temper install` refuse on every fresh repo.

Questions for plan/human:
1. What does the first-install scaffold write into `packageManager`? npm
   (what install spawns today), or a probe of the PATH? The field's
   `name@version` form also needs a version source, e.g. `npm --version`.
2. The dogfood's `.temper/package.json` needs the field via a human
   `chore(harness):` commit before or with this entry.

Once (1) is answered the rest is mechanical: parse `<name>@<ver>` from the
program's package.json. `pnpm` spawns `pnpm install` (and `.cmd` on
windows); `npm` keeps `npm_program`. The lockfile read takes only that
manager's lockfile, at the nearest ancestor as today. An absent field
refuses naming the package.json path; yarn/bun keep the by-name refusal.
Fixtures from `common::wire_sdk_harness` write no package.json, so they
would need one carrying the field.
