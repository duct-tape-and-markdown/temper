# Migrating to 0.0.25

**This note covers `0.0.24` to `0.0.25` and nothing earlier.** For the
upgrade before it, read [`MIGRATING-0.0.24.md`](MIGRATING-0.0.24.md).

Every harness needs step 1. The rest apply only if `check` or `emit`
reports what they name.

## 1. Re-emit for the anchored gate commands

Claude Code runs a hook in the session's current directory, which follows
a Bash `cd`. Every gate command now enters the project root first.

```text
before (0.0.24)  command -v temper >/dev/null 2>&1 || { …; exit 127; } && temper guard .
after  (0.0.25)  cd "${CLAUDE_PROJECT_DIR:-.}" || exit 1; command -v temper >/dev/null 2>&1 || { …; exit 127; } && temper guard .
```

**What to do.** Represented harness: run `temper emit` and commit the lock
and `.claude/settings.json`. Unrepresented: re-run `temper install`. Until
then, `install.gate-installed` reports the unanchored command as not
installed. A relative `gateHook` `binary` now resolves from the project
root whatever the session's directory.

## 2. Answer a governs collision

`check` now reports `kind.governs-collision` for each real document that
two file kinds' loci both reach, such as a `docs/CLAUDE.md` that a
`docs/*.md` kind and the built-in `memory` both claim.

**What to do.** Add that literal path to the `exclude` of the kind that
does not own it. An overlap that no document occupies draws nothing.

## 3. Fix a swallowed heading the reader now catches

A layout reader used to bind a shifted heading silently when a section
was missing two or more regions above the region it starved. Now it
refuses that case with `layout::swallowed_heading`, naming the heading,
the region that consumed it, and the region left unbound (GH #44).

**What to do.** Add the missing section the finding names, or reorder the
document to the layout.

## 4. Hoist hand-added keys in an ownerless settings file

When no `settings` member owns `.claude/settings.json`, `emit` used to
delete keys added there by hand. Now it refuses with
`drift::ownerless_manifest_keys` and names each key.

**What to do.** Re-run `temper install`. It hoists exactly those keys into
a new `settings` member and composes it into `harness.ts`. Or delete keys
you don't want.
