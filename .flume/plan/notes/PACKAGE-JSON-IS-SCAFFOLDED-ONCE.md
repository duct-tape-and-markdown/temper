# package.json is scaffolded once

`ensure_package_json` (src/install.rs) now writes `.temper/package.json` only
when absent; an existing manifest is parsed, never written. Required
dependencies it lacks (in neither `dependencies` nor `devDependencies`) come
back as `InstallOutcome::missing_dependencies` (`MissingDependency { package,
range, dev }`) and `install::render` prints one `missing` line per package
saying "add it with your package manager".

For PACKAGE-MANAGER-FIELD-NAMES-THE-MANAGER: that render line is the seam
where the declared manager (`packageManager`) could be named as the command
(`pnpm add -D ...`) instead of the generic phrase.

Not done: a missing `@dtmd/temper` that also does not resolve still flows on
to `npm install` and then emit, which fails on the unresolved SDK; the named
line is printed only if install completes. Whether install should refuse
up front in that case is unspecified.
