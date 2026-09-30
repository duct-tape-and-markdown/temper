# Migrating to 0.0.21

**This note covers `0.0.20` to `0.0.21` and nothing earlier.** It is the
first note in the series; for older upgrades, read the `Upgrading` section
of each release in [`CHANGELOG.md`](../CHANGELOG.md).

Most harnesses need steps 1 and 2. The rest apply only if you use what they
name.

## 1. Re-emit and commit the lock

The lock now records the temper version that wrote it, and carries rows for
the new `handler` and `mcp` kinds. The first `temper emit` after upgrading
rewrites it.

```toml
# before (0.0.20)
[[memory]]
name = "CLAUDE"

# after (0.0.21)
engine = "0.0.21"

[[memory]]
name = "CLAUDE"
```

**What to do.** Run `temper emit` and commit the lock. If CI runs `temper
emit --frozen` and then `git diff --exit-code`, it fails until you do. The
`engine` line changes on every upgrade from now on, so re-emit whenever you
bump temper. A gate run by a different temper version than the one that
wrote the lock reports `root.engine-matches` as advisory.

## 2. Write each hook's handlers in a `hooks` array

A hook member is now one matcher group, and its handlers are a list. The
flat fields moved into that list.

```ts
// before (0.0.20)
export const guard = hook({
  name: "PreToolUse",
  matcher: "Write|Edit|MultiEdit",
  type: "command",
  command: "temper guard .",
});

// after (0.0.21)
export const guard = hook({
  name: "PreToolUse",
  matcher: "Write|Edit|MultiEdit",
  hooks: [{ type: "command", command: "temper guard ." }],
});
```

**What to do.** Rewrite each `hook()` call this way. A group that runs
several commands on one event and matcher is one call with several
handlers, not several calls. The emitted `settings.json` is unchanged.

If your hook runs a temper gate, fire the SDK's constant instead of
spelling the command yourself:

```ts
import { GUARD_COMMAND, hook } from "@dtmd/temper/claude-code";

export const guard = hook({
  name: "PreToolUse",
  matcher: "Write|Edit|MultiEdit",
  hooks: [{ type: "command", command: GUARD_COMMAND }],
});
```

Two groups with the same event and matcher are now refused, where one used
to replace the other silently.

## 3. Address a hook by its event and matcher

A hook's name is its event joined to its matcher with `:`. A hook with no
matcher keeps the bare event.

```console
$ temper explain hook:PreToolUse
No member, requirement, kind, or leaf address named `hook:PreToolUse` is in the surface. ...

$ temper explain 'hook:PreToolUse:Write|Edit|MultiEdit'
Member `PreToolUse:Write|Edit|MultiEdit` (hook) — everything that holds it in place:
```

Quote the address in a shell: `|` is a pipe.

Each handler is a nested member: `hook:PreToolUse:Write|Edit|MultiEdit/handler/0`.

**What to do.** Search your program and prose for `hook:` (edge fields,
mentions, scripts calling `explain`) and add the matcher to each address
that names a hook with one. `check` reports any you miss as a
dangling edge. A matcher containing `/` is refused, since the address
cannot spell it.

## 4. Declare `.mcp.json` if you author MCP servers

`.mcp.json` now belongs to the `mcp` kind, as `.claude/settings.json`
belongs to `settings`. A program that authors `mcpServer()` members but not
the file draws a root `locus-declared` finding on it.

```ts
// after (0.0.21)
import { mcp } from "@dtmd/temper/claude-code";

export const mcpManifest = mcp({ name: ".mcp" });
```

**What to do.** Add an `mcp()` member beside your servers. Any top-level
key other than `mcpServers` goes in its `residue`.

## 5. An empty contract now means no clauses

```ts
harness({
  members,
  expect: [{ kind: skill, clauses: [] }], // 0.0.20: skill's default contract applied
                                          // 0.0.21: no clauses apply to skill
  contract: [],                           // the same change at the root
});
```

**What to do.** If you wrote `[]` meaning "use the default", drop the
binding (or the root `contract` field) instead. If you meant "no clauses",
nothing changes except that it now works.

## 6. Names temper now refuses

- A kind named `engine` or `declaration`: the lock uses both as its own
  keys. Rename the kind.
- A nested member key that is empty or contains `/`: it would spell a
  different member's address. Choose a key without `/`.
