# Migrating to 0.0.22

**This note covers `0.0.21` to `0.0.22` and nothing earlier.** For the
upgrade before it, read [`MIGRATING-0.0.21.md`](MIGRATING-0.0.21.md).

Every harness needs step 1. The rest apply only if you use what they name;
most of them concern corpus-declared kinds, and none moves a built-in
projection.

## 1. Re-emit and commit the lock

The `engine` line moves, and a nested file child's rows are keyed by its
full address. The first `temper emit` after upgrading rewrites the lock.

```text
before (0.0.21)  engine = "0.0.21"   nested file child keyed  supporting-doc:home
after  (0.0.22)  engine = "0.0.22"   nested file child keyed  skill:alpha/supporting-doc/home
```

The key moves in the `[[declaration.nested_member]]`, `layout_source`,
`layout_prose`, `satisfies`, `mention`, `include` and `input` rows. A leaf
mention row is headed by its host's own address too.

**What to do.** Run `temper emit` and commit the lock. A CI step running
`temper emit --frozen` then `git diff --exit-code` fails until you do. A
harness with no nested file member sees only the `engine` line move.

## 2. A directory kind's glob names its entry one segment deep

`emit` now refuses a projection that its own kind's `governs` glob cannot
find. A `unitShape: "directory"` kind lands its entry file inside the
member's directory, so a glob naming the bare file never matched it.

```ts
// before (0.0.21): emitted docs/alpha/GUIDE.md, which discovery never found
locus: { kind: "at", root: "docs", glob: "GUIDE.md" }

// after (0.0.22)
locus: { kind: "at", root: "docs", glob: "*/GUIDE.md" }
```

**What to do.** Put `*/` in front of the entry file.

## 3. A member name is one address segment

A name containing `/` is refused at `emit`. It used to place a file at a
depth no discovery walk read back.

```ts
// before (0.0.21)
note({ name: "a/b/sub/x", ... })

// after (0.0.22): one segment, depth through a nested kind
const page = kind({ name: "page", locus: { kind: "nested-file" }, ... });
const area = kind({ ..., templates: [{ kind: page, path: "*/PAGE.md" }] });
```

**What to do.** Rename the member to one segment. If the slashes expressed
depth, declare that depth as nested kinds: 0.0.22 composes nested file
layers to any depth (`area:ops/page/gate/leaf/home`).

## 4. A template's literal directory is not part of a child's key

A nested child under a template pattern such as `notes/*.md` is keyed by
its file stem. 0.0.21 folded the directory into the key.

```text
before (0.0.21)  skill:alpha/supporting-doc/notes-checklist
after  (0.0.22)  skill:alpha/supporting-doc/checklist
```

**What to do.** Respell any mention, edge or `explain` target written
against the old key. No built-in template has a literal directory, so only
corpus-declared kinds are affected.

## 5. Mention a nested file child through `mentionOf` or its full address

The SDK keys a nested file child by its host-qualified address, so a
hand-written `<kind>:<name>` string for one now refuses at `emit` as
dangling.

```ts
// before (0.0.21)
text`See ${{ address: "supporting-doc:home", display: "home" }}.`

// after (0.0.22)
text`See ${mentionOf(home)}.`   // spells skill:alpha/supporting-doc/home
```

**What to do.** Build the target with `mentionOf(child)`, which spells the
address for you.

## 6. A `**/` glob places by its own pattern

A leading `**/` names where a glob matches, never where a projection lands.
0.0.21 ignored the rest of such a glob and wrote `<name>.md`.

```text
glob: "**/sub/*.json"
before (0.0.21)  docs/alpha.md
after  (0.0.22)  docs/sub/alpha.json
```

**What to do.** Re-emit. If the file at the old path is still on disk,
delete it. Built-in kinds (`memory`'s `**/CLAUDE.md`, `agent`'s `**/*.md`)
emit the same bytes as before.

## 7. `when` bodies judge only what the element has

A `closed-keys` clause inside a `when` body now reads the body's own
`required` and `optional` rows, not the host contract's. Clauses that need
the member's document (`require_sections`, `section_contains`,
`name-matches-dir`, `format-places-edges`, each-grain `extent`) are refused
inside a body; the verdict they appeared to give was never computed from
the document.

**What to do.** Declare the element's keys with `required`/`optional` rows
in the same `when` body, and move refused clauses to the kind's own
`expect`.

## 8. `temper guard` needs a hook payload

`temper guard` exits 1 with an error when stdin is not a JSON hook payload.
It used to exit 0.

**What to do.** Run it only from a `PreToolUse` or `PostToolUse` hook row,
the way `temper install` wires it.

## 9. `.mcp.json` is a guarded projection

When your lock declares `mcp-server` members, `.mcp.json` joins the files
`emit` owns: the guard binds writes to it, and `install` places its
managed-by note there.

**What to do.** Edit the owning module and re-emit, as for any other
projection.

## 10. `install --yes` refuses Windows-reserved names

A registration key carrying `:`, `|`, a trailing dot or a device name would
scaffold a member module a Windows checkout cannot write. `temper install
--yes` now refuses it on every platform.

**What to do.** Rename the manifest entry so its key names one portable
file, as the refusal's help says.

## 11. Message text that CI may grep

Codes, clause labels, severities and exit codes are unchanged. The text
moved:

- `root.fresh` and the `nested_file_locus` and `ungoverned_projection`
  refusals name a member by its address (`area:beta/page/one/intent/home`,
  `skill:coordinate`), where they printed a bare name.
- `emit` reports a projection whose hand-edit it replaced as
  `overwritten`, and the tally gains an `overwritten` count:

  ```text
  before (0.0.21)  9 emitted, 0 unchanged, 0 reaped, 0 orphan-drift, 0 member-reaped
  after  (0.0.22)  0 emitted, 0 overwritten, 9 unchanged, 0 reaped, 0 orphan-drift, 0 member-reaped
  ```

- `root.locus-declared`, the guard's governed-locus message and the
  `PostToolUse` preambles are reworded.
- `explain` over a requirement with a `kind` facet lists wrong-kind
  satisfiers, matching what `check` counts.
- `JsonManifestError::NoDeclaredIdentity` is replaced by `NoPathIdentity`
  (`temper::json_manifest::no_path_identity`).

**What to do.** Update any pinned string; grep for the member's address
rather than its bare name.
