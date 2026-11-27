# Changelog

All notable changes to `temper` are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims
to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

`temper` is pre-1.0: while the major version is `0`, minor releases may carry
breaking changes. Releases are small and frequent.

## [Unreleased]

## [0.0.26] — 2026-11-27

### Upgrading from 0.0.25

Each line is what a 0.0.25 harness may see first, then what to do.
[`docs/MIGRATING-0.0.26.md`](docs/MIGRATING-0.0.26.md) walks each one with
before and after.

- **`check` reports `root.engine-matches`.** The lock was written by 0.0.25.
  It still reads, with its old built-in rows read as the embedded built-ins,
  so re-emit once the program emits (the next lines).
- **`emit` fails with "does not provide an export named 'emit'".** End the
  program with `export default program;` in place of the stdout write.
- **`emit` refuses with `temper::drift::typecheck_undeclared`.** Declare
  `typecheck` on `harness()`, with a `tsconfig.json` and the `typescript`
  dev dependency.
- **`check` refuses with `temper::tool::no_package_manager`.** Add
  `"packageManager"` to `.temper/package.json`.
- **The SDK refuses a hook's `name`, or `tsc` prints a remedy.** Run the
  compiler over `.temper/` and apply the renames it names.
- **`install.gate-installed` reports your gate hooks.** Re-emit: gate hooks
  now run the program's own `temper` bin.
- **`check` reports members under a subdirectory's `.claude/`, an
  `AGENTS.md`, or a `CLAUDE.local.md`.** Declare them, or leave a fixture
  tree ungoverned with `relocate(<kind>, { exclude })`.
- **A dial entry keyed `label` is refused.** Rename the key to `id` and
  spell the clause id in kebab case.
- **`emit` writes `.claude/rules/temper-sdk.md` and
  `temper-guidance-<kind>.md` rules.** Commit them with the lock.

### Breaking

- **`emit` typechecks the program and refuses one that declares no
  typecheck.** `harness({ typecheck: { tool, project } })` names a composed
  `tool(...)` and the project file; emit runs it with `--noEmit --project`
  and refuses on a type error with the compiler's diagnostics, so a type
  error Node's type stripping used to skip now stops `emit`.
- **The program's `.temper/package.json` must declare `packageManager`.**
  `check` and `install` refuse without it, and read the lockfile of the
  manager it names. npm and pnpm are read; yarn and bun are refused by
  name.
- **A program ends `export default program;`.** `emit`, `EmitResult`,
  `RegistrationFact` and `SettingsResidue` leave the `@dtmd/temper` root for
  the new `@dtmd/temper/pipe` subpath, which `temper emit` drives; a program
  writes nothing to stdout.
- **`temper emit` takes its workspace positionally.** `--into` is removed:
  `temper emit --into X` is now `temper emit X`.
- **Predicate keys are kebab case** (`max-len`, `min-len`, `forbidden-keys`,
  `allowed-chars`, `require-sections`, `must-define`, `section-contains`),
  and every clause id with them, such as `skill.max-len.name`. A 0.0.25 lock's
  snake-case spellings still read until `temper emit` rewrites them.
- **A dial entry names its clause `[[clause]] id = …`.** The `label` key is
  refused by the dial's closed key set, and the SDK's `DialEntry` field is
  `id`.
- **Five finding ids are renamed.** `graph.route` is `root.resolves`,
  `graph.directive-unbacked` is `root.import-backed`, `requirement.unfilled`
  is `requirement.count`, `requirement.dangling` is `requirement.resolves`,
  and `kind.governs-collision` is `kind.locus-collision`. The first four are
  now default-contract clauses a dial entry can lower.
- **`hook()` and `gateHook()` take the lifecycle event as `event`.** A
  `name` fails `tsc` with a remedy naming `event`, and refuses at runtime in
  the same words.
- **`gateHook()` and `telemetry()` take a `tool(...)` member as `tool`,
  never a `binary` string.** With none, a represented harness's gate hooks
  and taps run `./.temper/node_modules/.bin/temper`, where they used to run
  `temper` on the PATH.
- **`relocate(kind, { governs })` is `relocate(kind, { locus })`.** The lock
  columns follow (`locus_root`, `locus_glob`); a lock with the old columns
  still reads.
- **`memoryAnthropicDefaultContract` is `memoryDefaultContract`**, so every
  kind's default contract is `<kind>DefaultContract`.
- **`when()` takes a required `severity`**, as `clause()` does.
- **An edge field's `to` and a requirement's `kind` take kind values**, not
  kind-name strings. A self-edge names its kind through a getter,
  `get to() { return [self] as const; }`.
- **`permissionUnion()` returns `{ allow, ask }`** in place of a flat allow
  array, and a `capability()` takes `list: "ask"`.
- **`settings()` types `statusLine`.** A `statusLine` passed through
  `residue` is refused; move it to the typed field.
- **Built-in kinds claim more of the tree.** Skills, rules and output styles
  under any subdirectory's own `.claude/`, commands in subfolders of
  `.claude/commands/`, every `AGENTS.md`, and every `CLAUDE.local.md` are
  members now, so a tree holding fixture harness files may fail `check`
  until it excludes them.

### Added

- **`@dtmd/temper/patterns`.** `defaultPatterns({ typecheck, gates,
  reference, playbooks })` returns `{ members, typecheck }` for `harness()`:
  the typecheck and its `tsc` tool, the three gate hooks, and the harness
  reference. `gateHooks()`, `typecheck()` and `reference()` compose alone.
- **`playbook()` turns one area of guidance into a path rule, a routing
  skill, and one topic file per procedure**, linked by typed mentions. Its
  trigger is `paths`, `{ nested: "<dir>" }` or none, and `attachments` adds
  `clauses`, an ask `fence`, exact-allow `tasks`, and a `guard` that
  `temper guard` enforces over the area's paths.
- **`reference()` emits a harness reference skill at
  `.claude/skills/temper/`**, rendered from the program's declarations with
  one topic file per kind and per requirement. `temper bundle` ships the
  same reference as the plugin's skill.
- **Context-cost advisories.** `root.listing.description+when_to_use` sums
  the skill and command listing against the 8,000-character budget. A rule,
  subdirectory memory, `CLAUDE.local.md` or import target loaded on demand
  draws an advisory past 40,000 characters, and `trigger-agrees` flags a rule
  scoped narrower than its mentioned skill's `paths`.
- **`explain` states what loads a member, when, and what it costs** in
  characters. It also names what holds a requirement, the anchor each
  permission path rule resolves at, and an agent that shares a built-in
  agent's name.
- **The `tool` kind** is the one home of a command the harness runs:
  `tool({ package, bin })`, `tool({ file })` or `tool({ system })`. A hook
  handler, the `statusLine`, a gate hook, a tap, the typecheck and a
  `script()` verifier reach it by a `runs` edge, and `check` reports
  `tool.unlocked` when the program's lockfile does not lock a package tool.
- **Nested placements.** A nested skill, rule, command, output style or
  `CLAUDE.md` is a member named by its path (`skill:apps:web:deploy`,
  `command:frontend:component`) and emits back where it was read. A member
  whose name carries such a prefix declares its `location`.
- **The `output-style` kind**, with the settings `outputStyle` edge and an
  `unshadowed` advisory for a nested style that takes over a shallower one.
- **The `memory-local` kind for `CLAUDE.local.md`**, read in place and never
  emitted, and `AGENTS.md` as memory behind the runtime's absence gate.
- **`extend(base, { name, … })` mints a subkind** that keeps its base's facts
  and contract and adds fields, edges and counsel. `extend(requirement,
  { name })` adds a requirement vocabulary of your own.
- **`loads("launch" | "on-demand")` is a `when()` guard** over a member's
  load moment.
- **Emit delivers guidance as rules.** `.claude/rules/temper-sdk.md` carries
  the SDK's authoring counsel over `.temper/**`, and one
  `temper-guidance-<kind>.md` per guided kind carries its `guidance`,
  `fieldGuidance`, edges and requirements over the kind's locus.
- **`relocate()` takes `exclude`, `guidance`, `fieldGuidance` and `cite`**,
  so a relocation can leave a tree ungoverned and carry its own counsel.
- **A member or a kind can declare its own enforcement `mode`.** Each guard
  edge and the dial resolve the member's mode first, then its kind's, then
  the root's.
- **`review("…")` is a requirement verifier** naming the reader whose
  judgment holds it.
- **`mentionOf(member, { as: "path" })`** renders the target's path relative
  to the host.
- **The SDK package ships a JSON Schema per frontmatter kind** under
  `schemas/`, and `install --no-represent` seats a version-pinned
  `yaml-language-server` modeline pointing at it.

### Changed

- **An optional requirement left unfilled reports an advisory**
  (`requirement.count`), where it used to stay silent.
- **A finding names its severity as `required`, `advisory` or
  `disclosure`** on the terminal help line and in SARIF
  `properties.severity`. SARIF `level` and GitHub annotations keep their
  own vocabularies.
- **A nested rule's id joins its folders with `:`**:
  `.claude/rules/sub/x.md` is `rule:sub:x`, where it was `rule:sub-x`.
- **A skill, command or output style whose frontmatter YAML does not parse
  loads with no fields set**, as Claude Code loads it, and draws one
  `member.non-portable-yaml` advisory naming the keys read as unset. An
  agent of that shape is still a load fault.
- **An `.mcp.json` server with no `type` is judged a stdio server**, so a
  url-only entry draws `mcp-server.when.type=stdio.required.command`.
- **An agent `name` is held to the documented rules:** at most 256
  characters, no `:`, no leading `-`. The lowercase charset is gone, so
  `Explore` and `reviewer-v2` pass.
- **`plugin.json` admits every top-level key the plugins reference
  documents**, and `outputStyles`, `workflows` and `experimental` are
  typed.
- **A collection key of the wrong JSON type is an error**
  (`manifest.rejected-collection`), such as `"mcpServers": []`, at `check`
  and at the guard, and its value is kept.
- **A reference the runtime may answer from outside the tree** (a user
  output style, a marketplace declared in user settings) is one
  `coverage.off-tree-targets` note, never a route error.
- **On an unrepresented harness**, unreached supporting docs fold into one
  `coverage.unreached-supporting-docs` note, and a settings file that runs
  commands draws one `coverage.unrepresented-scripts` note.
- **Emit refuses** a program whose two runs differ
  (`temper::drift::nondeterministic`), two members deriving one projection
  path (`temper::drift::projection_path_collision`), and a hook group added
  or edited by hand in a settings file no member owns
  (`temper::drift::ownerless_manifest_entries`,
  `temper::drift::ownerless_registration_drift`). `check` reports that hand
  edit as drift.
- **A root-family clause bound where its judge never reads it** (on a kind
  or a requirement) is refused as vacuous.
- **`install` prints the contract's findings** before it asks to represent
  the harness. On a fresh scaffold it writes `packageManager`, a
  `tsconfig.json`, the typecheck dev dependencies and a program over
  `defaultPatterns()`; it never rewrites an existing `.temper/package.json`,
  and lists a missing dependency instead.
- **`install` re-points a stale schema modeline** and refuses a represented
  install over a skill named `temper`
  (`temper::install::reference_name_collision`).
- **An unreleased build stamps its locks `X+unreleased.<digest>`**, which
  moves only when the built-in facts do. A lock from another engine reads as
  a stale snapshot in every verb: a moved built-in row reads as the embedded
  built-in with the `root.engine-matches` advisory, never as a kind
  collision.
- **Every shipped clause carries guidance and a dated cite.**
- **Without a lock, `temper guard` allows every write.**

### Fixed

- **A dangling bare one-element edge target defers to `check`** like its
  qualified spelling (GH #52).
- **Nested `CLAUDE.md` files lift and emit at their own paths.** They used
  to all land at the root `CLAUDE.md`.
- **A relocation keeps only the base exclusions its new template still
  reaches**, so moving `memory` off `.` no longer refuses at lock load.
- **`check` reads a committed layout document's entries and `satisfies`
  section off the document**, so a hand edit is judged without a re-emit.
- **An `@path` import honors the backslash-space escape**
  (`@Design\ Docs/api.md`).
- **A file or `.claude/` directory a memory import reaches is no longer an
  unclaimed entry**, and the `.claude/` entry scan honors the root
  `.gitignore`. An unignored `.claude/worktrees/` names the documented
  remedy.
- **A program that prints to stdout is reported as a payload parse
  failure**, not as SDK version skew.
- **`install.gate-installed` reports a projection it could not read** in
  place of passing. On a tree with no program, it judges a seated
  SessionStart hook and its schema modelines.
- **`explain`'s impact strand reads the gate's own reachability**, custom
  kinds and declared edges included.

## [0.0.25] — 2026-11-25

### Upgrading from 0.0.24

Each line is what a 0.0.24 harness may see first, then what to do.
[`docs/MIGRATING-0.0.25.md`](docs/MIGRATING-0.0.25.md) walks each one with
before and after.

- **`install.gate-installed` reports your gate hooks.** Re-emit (or re-run
  `install` on an unrepresented harness) to pick up the anchored commands.
- **`check` reports `kind.governs-collision`.** Add the named path to the
  `exclude` of the kind that does not own it.
- **A layout document refuses with `layout::swallowed_heading`.** Add the
  missing section it names.
- **`emit` refuses with `drift::ownerless_manifest_keys`.** Re-run
  `install` to hoist the hand-added keys into a `settings` member.

### Breaking

- **Every gate command is anchored at the project root** (GH #67). It
  opens with `cd "${CLAUDE_PROJECT_DIR:-.}" || exit 1;`, so its binary and
  its `.` target resolve at the root after a Bash `cd`, where the guard used
  to exit 127 or judge a subdirectory. The installed-gate check recognizes
  the anchored form only.
- **Two kinds whose loci reach one discovered document are reported**,
  naming the document and both kinds. The literal `exclude` on the kind
  that does not own it answers it.
- **A heading with sub-headings swallowed into an earlier region refuses
  whenever any later region starves** (GH #44), not only the next one.
- **`emit` refuses, never deletes, a hand-added key in a settings file no
  member owns** (GH #40 follow-up). `install` hoists those keys into a
  `settings` member.

### Added

- **`gateHook()` takes `satisfies`**, so a gate hook can fill a requirement
  like any other member.

### Fixed

- **A directory `exclude` reaches beneath a rooted any-depth locus**, such
  as `docs/a/b` under `docs` + `**/*.md`, where it used to refuse at load.
- **The `root.locus-declared` guidance no longer says `emit` never writes**
  an undeclared manifest.

## [0.0.24] — 2026-11-25

### Upgrading from 0.0.23

Each line is what a 0.0.23 harness may see first, then what to do.
[`docs/MIGRATING-0.0.24.md`](docs/MIGRATING-0.0.24.md) walks each one with
before and after.

- **Your lock changes on the next `emit`.** The `engine` line moves to
  `0.0.24`, the lock gains `[[declaration.module]]` rows for the program's
  own modules, and a registration kind's `collection_address` drops its
  `manifest` key. Re-emit once and commit the lock.
- **`emit` needs Node 22.15 or later.** It refuses on an older Node rather
  than write a lock that fingerprints nothing.
- **A requirement that omits `required` is now required.** An unfilled one
  fails `check`. Spell `required: false` where you meant it advisory.
- **`GUARD_COMMAND` and `SESSION_START_COMMAND` are gone from
  `@dtmd/temper/claude-code`.** Rewrite each gate hook as `gateHook({ name,
  matcher })`, adding `binary` if temper is not on your PATH.
- **A locus glob that is not a placement template fails `check`.** Drop
  character classes, braces, extglobs and `?`; narrow names with an
  `allowedChars` clause, and leave a sibling file out with the new
  `exclude`.
- **`tsc` refuses `registration: []` on an embedded or nested-file kind,
  and `collectionAddress.manifest` on any kind.** Delete both.
- **The guard may report what it used to miss**, and stop reporting what it
  used to invent. It now judges the way `check` does.
- **Rules in subdirectories of `.claude/rules/` are now governed.** Claude
  Code loads them, so `check` now reads them. A `CLAUDE.md` under
  `.claude/rules/`, `agents/`, `commands/` or `skills/` is no longer a
  memory member.

### Breaking

- **An unset requirement `required` lowers to `true`** (GH #60), the
  default the model states. `required: false` is the advisory spelling.
- **`emit` requires Node 22.15 or later**, for the module load hook that
  records what the program imports.
- **`GUARD_COMMAND` and `SESSION_START_COMMAND` are no longer exported.** A
  gate hook is `gateHook({ name, matcher?, binary? })`; the engine renders
  its command, and `install` scaffolds that form.
- **A file locus glob must be a placement template**: literal segments with
  at most one `*`, optionally under a leading `**/`. Anything else is
  refused at load, where it used to match nothing.
- **`CollectionAddress` loses `manifest`.** A registration kind's manifest
  is its starless `at` locus, and a starred or non-`at` locus is refused.
- **`registration` is `never` on embedded and nested-file kinds**, and a
  layout `field` region's `slot` must be a key of the kind's typed surface.
- **A layout kind over a `json-document` or `toml-document` format is
  refused** at load. A layout types a markdown body.
- **A layout document's member identity follows its kind's unit shape.** A
  `named-field` or `directory` layout kind used to key its member by the
  file stem on `check` and by the program's name in the lock.
- **`rule` governs `.claude/rules/**/*.md`** and **`memory` excludes**
  `.claude/agents/`, `.claude/commands/`, `.claude/rules/` and
  `.claude/skills/`, matching what Claude Code loads as each.

### Added

- **`gateHook()`**, a gate hook that states the temper binary it runs, for a
  workspace install off the PATH (GH #65).
- **`exclude` on a file locus**: paths, or directories, the kind does not
  govern, such as a `README.md` beside its members (GH #64).
- **Module fingerprinting.** `emit` records every module the program loads
  that the repository tracks, so editing an imported module reads stale on
  `check` without an `input()` (GH #61). Gitignored build output and
  packages under `node_modules` are left out.

### Changed

- **The guard judges with `check`'s judges** (GH #66). Both edges classify
  sites, walk the tree and judge freshness the way `check` does, so a
  finding the guard raises is one `check` raises on the same tree. In
  practice:
  - declared layout documents no longer draw `root.locus-declared`;
  - the walk honors ignore rules, including a linked worktree's common
    `info/exclude`;
  - the shell edge reports a moved import, include or input;
  - `.`-rooted loci, nested layout children, named-field layout documents
    and members at a committed `local` locus are judged at both edges.
- **`install.gate-installed` judges the gate hook member**, so a hook that
  runs temper by path reads installed.

### Fixed

- **A layout kind dropped its frontmatter fields** (GH #63). A layout
  structures the body; frontmatter reads as on any kind.
- **Clauses on a layout collection's entries never fired** through a host
  path such as `invariant[*].test` (GH #62).
- **A character class in a locus glob broke placement** (GH #64): the name
  replaced `*` and the class stayed literal. The glob is now refused at
  load instead.
- **Emit could place a member on an excluded path**, and coverage counted
  excluded paths as governed.

### Known gaps

Re-reading each built-in locus against the current Claude Code docs found
two more it does not yet claim. Both land in a later release:

- **Commands in subdirectories of `.claude/commands/`**, which Claude Code
  loads as `/frontend:component`. The `command` kind reads the top level only.
- **Nested `<subdir>/.claude/skills/` and `.claude/agents/`**, which load
  once Claude works in that subdirectory. The built-in kinds claim the root
  folders only.

## [0.0.23] — 2026-10-16

### Upgrading from 0.0.22

Each line is what a 0.0.22 harness may see first, then what to do.
[`docs/MIGRATING-0.0.23.md`](docs/MIGRATING-0.0.23.md) walks each one with
before and after.

- **Your lock changes on the next `emit`.** The `engine` line moves to
  `0.0.23`. Re-emit once and commit the lock.
- **Path-scoped rules you were told are dead may come back to life.** If
  you ran `check --harness <path>` with anything but `.`, rules whose
  `paths` globs are anchored at the root (`scripts/**`) were reported dead.
  Undo any workaround you made for that, such as a `**/` prefix or a dial.
- **A new `coverage.unclaimed-entry` advisory under `.claude/`.** A kind
  whose locus glob has an interior directory (`prompts/*.md`) no longer
  claims files outside it. Model the entry with a kind, or remove it.
- **A relocated built-in kind moves its stray advisories.** The root you
  relocated *off* is now scanned for strays, and the root you relocated
  *to* no longer is. A relocation under `.temper/` now fails `check` with
  `at-locus.under-workspace`.
- **The guard blocks a hand write it used to allow** when your lock still
  carries old `./`-prefixed rows. Re-emitting rewrites them.

### Breaking

- **A built-in kind relocated under `.temper/` is refused at `check`**
  (`at-locus.under-workspace`), as `emit` already refused it.
- **`coverage.unclaimed-entry` reads a kind's whole locus glob**, not its
  last segment. Entries the last segment used to mask are now reported;
  `.claude/CLAUDE.md` stops being reported.
- **A relocated built-in kind is scanned at its new locus**: the stray scan
  advises over the root it left and stops advising over the root it moved to.
- **The guard reads an old lock's `./x` rows** as the projections they
  name, so a hand write to one is refused, matching `check`.
- **`explain`'s member field strand names tap events by their documented
  name** (`SkillInvoked`), not the log's snake_case (`skill_invoked`).
- **A lock `collection_address` row with no `entry_shape` is refused**
  where it was silently filled in. Only a hand-built lock can lack it.

### Fixed

- **`root.reachable` reported path-scoped rules dead** unless `--harness`
  was spelled `.`: globs anchored at the root (`scripts/**`,
  `scripts/a.sh`) matched no file when the harness root was given as an
  absolute or a relative path.
- **`.claude/CLAUDE.md` was reported both a `memory` member and an
  unclaimed entry** in the same run.
- **A relocated built-in kind's new locus drew stray advisories**, and its
  old one drew none.
- **`emit` refused a `local` kind's member rooted under `.temper/`**,
  which `check` accepted.
- **On Windows, `bundle`'s file list and `explain`'s kind narration mixed
  `\` and `/`** in one path.

## [0.0.22] — 2026-10-01

### Upgrading from 0.0.21

Each line is what a 0.0.21 harness may see first, then what to do.
[`docs/MIGRATING-0.0.22.md`](docs/MIGRATING-0.0.22.md) walks each one with
before and after.

- **Your lock changes on the next `emit`.** The `engine` line moves to
  `0.0.22`, and a nested file child's rows are keyed by its full address
  (`skill:alpha/supporting-doc/home`, not `supporting-doc:home`). Re-emit
  once and commit the lock.
- **`emit` refuses a projection its own kind cannot find.** The usual cause
  is a `unitShape: "directory"` kind whose glob is the bare entry file:
  spell `GUIDE.md` as `*/GUIDE.md`.
- **`emit` refuses a member name containing `/`.** A name is one address
  segment. Rename it, and express depth with a nested kind.
- **A nested child under a template with a literal directory is keyed by
  its file stem.** Under `notes/*.md`, `notes/checklist.md` is `checklist`,
  where it was `notes-checklist`. Respell any address that used the old key.
- **A mention written as a bare `supporting-doc:home` string for a nested
  file child now dangles.** Build it with `mentionOf(child)`, or spell the
  full address.
- **A corpus kind whose glob starts with `**/` places by its own pattern.**
  `**/sub/*.json` lands `sub/<name>.json`, where it landed `<name>.md`.
  Re-emit, and delete the old file if it is still on disk.
- **A `when` body may fail `check` where it passed.** Its `closed-keys` now
  reads the body's own `required`/`optional` rows, and `require_sections`,
  `section_contains`, `name-matches-dir`, `format-places-edges` and
  each-grain `extent` are refused there. Declare the element's keys in the
  body, and move the refused clauses to the kind's own `expect`.
- **The guard blocks writes to `.mcp.json`** when your lock declares
  `mcp-server` members. It is a projection like any other: edit the owning
  module and re-emit.
- **A CI step that greps temper's output may stop matching.** Findings and
  refusals now name members by address, `emit` prints `overwritten` for a
  hand-edited projection it replaced, and the `root.locus-declared` and
  guard messages are reworded. Codes, severities and exit codes are
  unchanged.

### Breaking

- **Nested file children are keyed by their host-qualified address** in
  every lock row and in the SDK (`mentionOf`). A hard-coded
  `<kind>:<name>` mention of one is dangling.
- **A projection path must match its own kind's glob**, and a member name
  is one segment. Both were emitted before to files no discovery walk could
  find.
- **A template's literal leading directory is no longer folded into a
  nested child's key.**
- **A `**/` prefix names where a glob matches, never where a projection
  lands.** A corpus kind's projection path can move; every built-in
  projection is byte-for-byte unchanged.
- **An address with an odd number of segments reads as a member**, at any
  depth. A leaf's child path is joined with `.`; a leaf address whose child
  path carried `/` now resolves as a member and is refused as not found.
- **`when` bodies:** `closed-keys` reads the body's own rows, and clauses
  that need the member's document are refused inside the body.
- **`temper guard` exits 1 when stdin is not a hook payload**, where it
  exited 0. Invoke it only from a `PreToolUse` or `PostToolUse` hook row.
- **`.mcp.json` is emit-owned** when the lock declares `mcp-server`
  members.
- **`temper install --yes` refuses a registration key with a
  Windows-reserved shape** (`:`, `|`, a trailing dot, a device name), where
  it scaffolded the member on Unix.
- **`emit`'s report** marks a replaced hand-edit `overwritten`, and the
  tally gains an `overwritten` count after `emitted`.
- **Message text:** `root.fresh` and the placement refusals name members by
  address; `root.locus-declared`, the guard's governed-locus message and
  the `PostToolUse` preambles are reworded; `explain` over a requirement
  with a `kind` facet names wrong-kind satisfiers;
  `JsonManifestError::NoDeclaredIdentity` is replaced by `NoPathIdentity`.

### Added

- **Nesting to any depth.** A nested file child can host a layer of its
  own, so `area` → `page` → `intents/*.json` composes, discovers and
  resolves, and each child is addressed through every host
  (`area:ops/page/gate/leaf/home`).
- **Literal directories in globs and template patterns.** `docs/*.md` and
  `notes/*.md` place under the directory, where they were refused.
- **`json-document` kinds can take their identity from a directory**, as a
  markdown kind does (`*/area.json`).
- **`emit` names a hand-edit it replaced**: `overwritten    intent  home`.

### Changed

- **One address grammar at every depth.** Findings, refusals, lock rows and
  `explain` spell a member by its full address.
- **A lock written by another temper version that this one cannot load**
  names both versions and the remedy, instead of rejecting rows the reader
  never wrote.
- **`check` makes fewer whole-corpus passes per run**: the resolved edges
  and the containment family are each derived once.

### Fixed

- **A `**/` glob projected `<name>.md` whatever the kind's format.**
- **Two hosts each carrying a same-named nested file child** (two skills
  with a `home.md`) drew a false `member.admissibility` refusal, and
  `explain` could name neither child.
- **A `when` row naming `when` as its own guard** crashed `check` with a
  stack overflow. It is now refused at load.
- **`explain address:<leaf>`** answered as if asked for a member.
- **`install` scaffolded member modules a Windows checkout cannot write.**
- **A `closed-keys` clause in a `when` body** flagged element keys the host
  never declared.

## [0.0.21] — 2026-09-30

### Upgrading from 0.0.20

Each line is what a 0.0.20 harness may see first, then what to do.
[`docs/MIGRATING-0.0.21.md`](docs/MIGRATING-0.0.21.md) walks each one with
before and after.

- **Your lock changes on the next `emit`.** It gains an `engine = "..."`
  line naming the temper version that wrote it, and rows for the new
  `handler` and `mcp` kinds. Re-emit once and commit the lock. From now on
  the `engine` line moves with every upgrade.
- **`hook()` no longer type-checks with a flat handler.** Write the handlers
  in a `hooks` array: `hook({ name: "PreToolUse", matcher: "Bash", hooks: [{
  type: "command", command: "..." }] })`. The emitted `settings.json` is the
  same.
- **An edge or mention naming a hook by its event alone may now dangle.** A
  hook with a matcher is addressed as `hook:<Event>:<matcher>`, so
  `hook:PreToolUse` becomes `hook:PreToolUse:Write|Edit|MultiEdit`. A hook
  with no matcher keeps `hook:<Event>`.
- **A root `locus-declared` finding on `.mcp.json`.** The file now belongs
  to the `mcp` kind. A program that authors `mcpServer()` members but not
  the file itself leaves it undeclared; add an `mcp()` member.
- **A gate hook advisory on a represented harness now says what to write.**
  When a gate hook is missing or runs an old command, `check` names the hook
  and the command its module should carry, where it used to suggest `temper
  install`, which writes no settings on that path. Fire the SDK's
  `GUARD_COMMAND` or `SESSION_START_COMMAND` from the hook and re-emit.
- **The `PostToolUse` gate row runs the guard now, not `check`.** A harness
  scaffolded earlier wires `temper check . --reporter session-start` at
  `PostToolUse`, a row that could never carry a finding. Re-run `temper
  install` to rewire it, or, for a represented harness, fire
  `GUARD_COMMAND` from that hook and re-emit.

### Breaking

- **A hook member is one matcher group.** Its name is the event joined to
  its matcher with `:`, and each entry of its `hooks` array is a nested
  `handler` member, addressed `hook:<Event>:<matcher>/handler/<n>`. The
  `Hook` type carries `matcher` and `hooks`; the flat `type`, `command` and
  `timeout` fields moved to `Handler`.
- **Two members of one kind at one address are refused** as a malformed
  lock, instead of one silently winning. This includes two hook groups with
  the same event and matcher.
- **An empty contract is a declared contract.** An `expect` binding with `clauses: []`
  and a root `contract: []` now mean "no clauses". They used to fall back to the
  default contract.
- **Some names are refused.** A kind named `engine` or `declaration`
  collides with the lock's own keys and is refused. A nested member key
  containing `/` or empty is refused, since it would spell another member's
  address.

### Added

- **The `mcp` kind** governs `.mcp.json` as a whole, as `settings` governs
  `.claude/settings.json`: a hand edit anywhere in the file is drift.
- **The `handler` kind** types each entry of a hook group, with
  `handlerDefaultContract` checking its `type` and the field each type
  requires (`command`, `url`, `server` and `tool`, or `prompt`).
- **The lock names the engine that wrote it**, and the root clause
  `engine-matches` reports, as advisory, a lock written by a different
  temper version. A lock with no `engine` line stays silent.
- **`temper --version` names the build commit**; `-V` stays the bare
  version.
- **The SDK exports `GUARD_COMMAND` and `SESSION_START_COMMAND`**, generated
  from the engine's own gate commands.

### Changed

- **`temper guard` runs at both edges of a tool call.** Before a file-writing
  tool it judges the pending write, as before. After a shell tool, whose
  writes no payload names, it judges the tree the call left for projection
  drift. In `warn` mode the finding now reaches the session's context,
  where before it went to stderr, which Claude Code shows nowhere.
- **A lone heading at the very top of a layout document is its title.** Its
  span is the document's own prose, and the regions bind to its child
  headings.
- **A file reached by an `@import` carries the directives its format runs,
  whatever its kind**, so an import cycle through a rule is now seen.
- **An embedded edge whose target the program does not compose is judged by
  `check`**, not refused by `emit`. A `render` that reads such a target's
  facts is refused by name.
- **`check` names the harness root it resolved** when that differs from the
  path it was given.
- **The coverage note's total counts every checked member**, nested ones
  included, and `explain` resolves composed members such as
  `hook:PreToolUse:Bash/handler/0`.
- **The `coverage.unmodeled-surface` note is retired.** Both config files
  now belong to a kind.

### Fixed

- **`DirectoryAdded`, `PreModelSwitch` and `PostModelSwitch` hooks** no
  longer draw a false `hook.enum.event` finding.
- **A handler with no `type`** is now a finding; it used to pass.
- **Tap records written from a linked worktree** record paths relative to
  the root the tap ran from, so one rule no longer reads as one member per
  worktree.
- **A dangling mention of a leaf** names the missing leaf, not its member.

## [0.0.20] — 2026-09-23

### Upgrading from 0.0.19

Each line is what a 0.0.19 harness may see first, then what to do.

- **A lock row's `source_path` changes on the next `emit`.** An `input()`,
  `include()` or layout import outside the program root used to record an
  absolute, machine-specific path. It now records a path relative to the
  program root (`../../src/x.txt`). Re-emit once and commit the lock; it is
  then the same on every checkout. A target on a different Windows drive has
  no relative path and is refused with the path named.
- **`membership` over a list-valued field may now report findings.** It used
  to pass silently whatever the list held. It now checks each element, and
  each element outside the allowed set is its own finding.
- **`unique` over a list-valued field is now a finding** saying that
  `unique` over a list is not defined, where it used to pass silently. If
  the field is meant to be a scalar, narrow it with a `type` clause.
- **The `guard` stops binding stray documents unless the contract asks it
  to.** A write of a document the program never declared, at a governed
  locus, binds the guard only where a `locus-declared` clause is bound. The
  shipped root default binds one, so nothing changes unless your root
  contract drops it. If you dropped it and still want the guard to catch
  strays, bind it again.

### Fixed

- **A source dependency outside the program root no longer breaks the
  lock's portability.** The committed lock is now byte-identical across
  checkouts, worktrees and machines, and it no longer carries Windows'
  `//?/` path prefix.
- **`membership` and `unique` no longer pass silently over a list.**

### Changed

- **The `guard` asks the contract.** Whether an undeclared document at a
  governed locus is a problem is the contract's call (`locus-declared`);
  what happens about it is the enforcement mode's (`block`, `warn`, `note`).
- **The reserved-`prose` refusal says to rename the field.** On an embedded
  value it no longer suggests authoring the words as member prose, which an
  embedded value cannot do.

## [0.0.19] — 2026-09-23

### Upgrading from 0.0.18

Each line is what a 0.0.18 harness may see first, then what to do.

- **`emit` fails with "leaf `prose` is reserved".** `prose` now names a
  member's own words, so an embedded value may not use it as a field name.
  Rename the field (`leaves: { words: prose }`) and the renderer that reads it.
  The projection bytes do not change.
- **`root.locus-declared` on `.claude/settings.json`.** The committed settings
  file is now governed whole. Declare a `settings` member (`settings({ name:
  "settings", … })` from `@dtmd/temper/claude-code`) and move the harness-level
  `settings:` keys into it. A key the settings reference does not document goes
  in its `residue: { … }` bag.
- **A hand edit to `.claude/settings.json` is refused by `guard` and reported
  by `check`.** The file is a projection now, so 0.0.18's allowance for
  residue-only edits is gone. Author the change in the program and re-emit.
- **A dial entry or CI filter stops matching.** These rule ids moved to clause
  labels: `config.stale`, `prose.include-stale` and `layout.import-stale` are
  now `root.fresh`; `locus.undeclared-member` and `layout.undeclared-member` are
  now `root.locus-declared`. `when` labels now carry the guard's values
  (`mcp-server.when.type=stdio`; seven shipped `marketplace` and `mcp-server`
  labels moved, six of them former collisions), a `when` body clause has its
  own label under its host's, and a filtered `degree` clause carries its fields
  (`state.degree.writes`). Respell each entry from the finding's new `rule` id.
- **A set predicate inside a `when` body is refused.** `degree`,
  `membership`, `count`, `unique` and `kind` in a guard's body were accepted and
  never judged. Move the clause to the kind's top level, or split the members
  into two kinds.
- **New findings may appear.**
  - `root.reachable` names a member whose every registration channel is dead;
    this check had not run since 0.0.8.
  - A member-grain clause bound through a requirement is now judged over the
    requirement's satisfiers.
  - An `@import` ring among memory files now fails `graph.acyclic`.
- **Workarounds for field-edge cycles can go.** A cycle through declared
  fields (view → signal → path → view) is legal now; only the import relation
  must be acyclic.

### Added

- **A root contract.** `harness({ contract })` binds clauses over the whole
  harness, and `rootDefaultContract` applies when a program declares none.
  It binds three predicates at `advisory`:
  - `reachable`: a member whose every registration channel is dead;
  - `fresh`: a lock row that no longer matches disk;
  - `locus-declared`: a document at a governed locus that the program never
    declared.

  Compose the array to change a severity, or dial the label (`root.fresh`
  at `required` makes a drifted pin fail CI).
- **`reachedFrom(roots, via)`.** Each selected member must be reachable from
  the satisfiers of the `roots` requirement over the named edge fields. It
  catches a member stranded behind another unreachable member, which a
  one-hop `degree` cannot. It follows cycles and ships in no default contract.
- **Field-filtered `degree`.** `degree({ incoming: { min: 1 }, fields:
  ["writes"] })` counts only the named edge fields. Two filtered bounds on one
  kind are separate clauses with separate labels.
- **Member inputs.** `inputs: [input(import.meta.url, "./code/login.asp")]`
  fingerprints a file a member's claims rest on without copying any of its
  bytes into the projection. When the file changes, `root.fresh` names the
  member and says to re-verify its claims before re-emitting.
- **The `settings` kind.** It governs the committed `.claude/settings.json`
  whole: documented keys are typed, the rest goes in a named `residue` bag, and
  the lock fingerprints the file. A `residue` key that shadows a typed field is
  refused.
- **Containment edges.** A host carries one `contains:<kind>` edge to each
  embedded member its body composes. An unfiltered `degree` ignores them; name
  them in `fields` to count them.
- **`span(words)`** builds a prose span from a computed string. The `` text`…` ``
  tag treats its interpolations as references, never as words.
- **`explain kind:<name>` for an unused kind.** A built-in kind the harness has
  no member of now narrates its locus, registration and address form, and a
  kind's leaf set shows before any member exists.
- **A layout member's own span** lands on its reserved `prose` leaf, addressed
  `<host-address>/<kind>/<key>/prose`.

### Changed

- **Acyclicity covers the import relation only.** Cycles through declared
  field edges are legal. `@import` directives are now checked, and before
  this release they were not.
- **`install`** brings an existing `.claude/settings.json` into the program
  as a `settings` member, and places its gate hooks through the program.

### Fixed

- **A CRLF file no longer mismatches its own fingerprint.** An include,
  layout-import or input target with Windows line endings was hashed raw at
  emit and line-ending-normalized at check, so it read as changed on every run.

## [0.0.18] — 2026-09-08

### Upgrading from 0.0.17

Each line is what a 0.0.17 harness may see first, then what to do.

- **A hand `Write` or `Edit` into a governed locus exits 2 in `block` mode**
  — `specs/*` under a kind that governs it, or an undeclared path under
  `.claude/` — with the locus named. Declare the member in the program and
  `emit` it; the path was never temper's to leave open.
- **An `Edit` to `.claude/settings.json` is judged by the file it would
  land.** An edit touching only residue temper does not own (`permissions`,
  `autoMemoryEnabled`) passes. An edit that drops a hook the lock declares
  (`guard.manifest-dropped-member`) or leaves the file unparseable
  (`guard.manifest-unparseable`) is refused; author the hook change in the
  program instead.
- **A stray document at a governed locus is a warn**
  (`locus.undeclared-member`), and `coverage.checked` now splits declared
  from undeclared per kind. Under `--deny-advisories` that stray fails the
  run, while a clean harness with only `coverage.checked` no longer does.
  Declare the document or move it out of the locus.
- **A bare nested-member key that two hosts carry is refused**, naming both
  hosts; the same key declared twice under one host is a malformed lock
  (`nested-member.admissibility`). Spell the full
  `<host-address>/<kind>/<key>`.
- **SDK: two kinds in play under one name refuse**, naming the kind and both
  loci, where 0.0.17 kept the first by authored order. `relocate(base,
  delta)` is the sanctioned spelling for widening a built-in's edge fields
  or moving its locus.
- **SDK types narrowed.** `embeddedMemberValue` leaves are
  `Record<keyof T, string | Text>` and `registration` is narrowed by locus,
  so `tsc` may fail where 0.0.17 passed; each failure is a leaf the engine
  would have refused at `check`.
- **Two lock re-spells show as drift once, then hold**: one
  `[[declaration.layout_source]]` row per layout document, and a label per
  clause on `section_contains` and `require_sections` rows. Re-emit once;
  both converge.
- **`check --reporter session-start` prints a payload on a load fault** (the
  fault's own rule id, `gate.load-fault` when the report carries none)
  where 0.0.17 printed nothing. A session that opened clean over a broken
  lock now opens with the fault in view; terminal, GitHub, and SARIF
  reporters still exit non-zero.

### Added

- `check` names a document at a governed locus the lock declares no member
  for (`locus.undeclared-member`, warn). A stray `.claude/rules/x.md` or
  `.claude/agents/x.md` that Claude Code would load, `emit` would never
  maintain, and `check` previously counted in silence now carries a finding
  naming the document and its kind's locus; the coverage line reports
  declared and undeclared counts per kind. Under `--deny-advisories` this
  exits non-zero. (GH #58)
- `guard` binds a write into a represented committed kind's governed locus
  when no member is declared there, at the kind's enforcement mode. In
  `block` mode a hand `Write` creating a file under any governed locus,
  `specs/` included for a harness that governs it, is refused with the
  locus named; declared projections keep their drift wording. (GH #58)
- `guard` refuses a `Write`, `Edit`, or `MultiEdit` that would leave a
  represented manifest unparseable (`guard.manifest-unparseable`). An
  unparseable `.claude/settings.json` makes the harness unloadable and
  Claude Code skips the file whole, hooks included, so the boundary is the
  only placement that can say so. (GH #40)
- `explain kind:<name>` is the authoring entry point: alongside guidance and
  cite it now renders the document shape a layout kind reads, the child
  kinds a host admits, the kind's locus and commitment, and the address form
  a reference to one of its members takes, before any member exists. `explain` also accepts the engine's own
  `<kind>:<name>` member address and the full `<host-address>/<kind>/<key>`
  nested spelling. (GH #47, #49)
- A nested member can be addressed by its full host-qualified address
  (`<host-address>/<kind>/<key>`) in edge fields and in the SDK's
  `embeddedMemberValue`, so two same-keyed members under different hosts no
  longer force a rename. (GH #50, #51; decision 0049)
- A built-in kind gains a sanctioned relocation form for added edge fields:
  `relocate(builtinKind, { edgeFields })` emits one `edge` fact row and no
  kind-fact change, and `{ governs: { root, glob } }` moves the kind's
  locus, the one fact the engine's relocation overlay applies. (GH #53, #54)
- SDK: `embeddedMemberValue`'s leaves are typed against the kind's field
  schema, and `KindFacts.registration` is narrowed per locus, so an omitted
  leaf or a registration on an embedded kind fails `tsc` rather than
  `check`. (GH #46)

### Changed

- A bare nested-member key that more than one host carries is refused at
  resolution, naming every host, in edges, mentions, and `explain`; it
  previously resolved to whichever member the scan reached first. A
  same-host duplicate `(kind, key)` is refused at admissibility. Address
  the member by its full spelling. (decision 0049, amended)
- Two `edge` fact rows spelling one `(from, field)` slot are refused at
  admissibility as a malformed lock; they previously resolved every
  reference twice and reported spurious `graph.route` findings.
- A layout declaring more than one verbatim prose region is refused at
  admissibility; only the first could ever carry bytes.
- `--deny-advisories` no longer fails a clean harness: disclosure notes such
  as `coverage.checked` are reported at a `note` severity that never
  blocks, and only declared-clause violations escalate. (GH #42)
- SDK: two kinds in play under one name are refused, naming the kind and
  both loci, unless one is a relocation of the other (the relocation wins)
  or the two are structurally equal. `kindsInPlay` previously kept the
  first by authored order, which dropped a relocated kind's added edge
  whenever the corpus had no member of it yet. (GH #53)
- Admissibility refusals name the declaration to open, not only the
  collision that exists.
- `emit` places the managed-by note for a frontmatter projection itself,
  so a fresh emit no longer reports `install.gate-installed` until a
  separate `install` converges it. (GH #57)
- Every `section_contains` and `require_sections` clause carries its own
  compiled label, so a kind may declare more than one of each. (GH #48)
- An embedded member's host rides its identity, not a synthetic key in its
  fields; a `closed-keys` clause on an embedded kind no longer indicts a key
  no author wrote.

### Fixed

- The `session-start` reporter runs over a harness that cannot be loaded.
  A load fault (malformed lock, non-UTF-8 member, duplicate declaration
  key) previously aborted before any reporter, so the SessionStart hook
  received empty stdout and the session opened with no verdict, the shape
  of a clean pass. It now carries one blocking finding whose rule is the
  fault's own code; terminal, GitHub, and SARIF reporters still exit
  non-zero.
- `guard` blocks under the invocation `install` wires (`temper guard .`):
  a relative root is resolved before a `file_path` is relativized against
  it, and an absolute `file_path` matches a single-segment target such as
  `CLAUDE.md`. `mode: block` on the root memory file was cosmetic before
  this. (GH #39)
- `guard` judges an `Edit` or `MultiEdit` to a represented manifest by the
  manifest it would land, so an edit touching only co-owned residue of
  `.claude/settings.json` (`permissions`, `autoMemoryEnabled`) is allowed
  and a member the lock declares cannot be dropped through an edit.
  (GH #38, #40)
- `.claude/settings.json` is recognised as emit-owned: registration
  members live under `[[declaration.registration]]`, which the ownership
  scan did not read, leaving the manifest unguarded.
- A discovered layout document the lock declares no member for is named
  (`layout.undeclared-member`) instead of reading as an empty host. Emit
  records every layout source it reads on the lock, so a declared layout
  member whose document lowers into no content row (field regions only, or
  an empty collection) is not reported as undeclared. (GH #43)
- A layout's positional heading binding reports the mismatch when a leading
  title or a missing section shifts every later region. (GH #41, #45)
- `install` reports a synthesized hook placement an authored hook member
  supersedes as its own outcome, and `check` no longer flags that hook as
  needing install forever.
- A render hook or embedded value can cite a nested member as an edge
  target; the member table previously indexed top-level members only.
  (GH #50)
- A local-commitment layout member's captured prose reaches `explain`;
  the read kept its collection and fill rows and dropped the spans.
- `explain` on a full nested-member address renders the member and stops;
  it previously appended a "not a well-formed leaf address" refusal after
  a successful resolution.

## [0.0.17] — 2026-09-04

### Fixed

- `emit` no longer refuses a harness that authors two `hook` members on one
  event. 0.0.16's duplicate-identity refusal keyed every member by
  `kind:name`, and a hook's name is its event, so two `PostToolUse` groups
  (a shape Claude Code admits) failed with `duplicate identity key
  hook:PostToolUse`. A projected member's address is its file and still
  refuses a duplicate; a registration member's address is its group key
  and admits several. If 0.0.16 broke your `emit`, this is the fix.

## [0.0.16] — 2026-09-04

### Fixed
- `rule.mention-reachable.paths` no longer reports a strict-subset scope as
  unreachable. Containment compared glob strings for equality, so a rule
  scoped to `database/x/**/*.sql` mentioning a skill gated on `**/*.sql`
  was flagged. Containment is now decided over the path sets the globs
  denote, brace alternation and negated classes included.
- An `at` locus rooted under `.temper/` is refused at `emit` and at `check`
  with a named error. Discovery fences the workspace by design, so such a
  kind emitted and locked its members while `check` never read them back:
  a silent gate blind spot that surfaced only as a zero in the coverage
  line.
- A requirement's declared `kind` is held against its satisfiers. A member
  of another kind that claims to satisfy a kind-narrowed requirement is a
  required-severity finding, and `explain` no longer lists it as filling
  the requirement.
- `temper guard` binds `.claude/settings.json`. The file is the spliced
  projection of every `hook`, `installed-plugin`, and `known-marketplace`
  member, but it was not in the guard's target set, so every spelling of a
  direct edit to it was silent.
- Two `hook` members declaring the same event no longer share one address
  silently on the edge-resolution side of a duplicate-key map; the engine
  and the SDK now build every identity map through one constructor that
  refuses a duplicate key by name. (The hook kind's own identity is a
  separate open question, tracked as GH #32.)

- `temper guard` no longer binds unrelated files that merely share a
  projection's filename. The guard compared a declared projection against the
  incoming path as a bare string suffix, so a root `CLAUDE.md` projection
  bound every `CLAUDE.md` anywhere in the tree, the authoring source under
  `.temper/memory/` included. The match now has to land on a path-segment
  boundary.
- A mention addressing an embedded leaf (`<member>/<kind>/<key>/<path>`) no
  longer dangles at `check`. `emit` accepted the address while `check`
  parsed it as a bare requirement and looked in the roster; both verbs now
  resolve it through the same leaf resolver.
- Two kinds declaring the same `collectionAddress` are refused at
  declaration with a named finding (`kind.collection-address-collision`).
  Previously each kind's selection silently became the union of every entry
  at that address, so counts multiplied by the number of kinds and one
  kind's clauses indicted another kind's members.
- `temper tap` records for `InstructionsLoaded` now join to their members.
  The record carried an absolute path and the reader compared it against a
  member id, so no instructions-loaded evidence had ever joined on any
  machine. The tap record is now version 2: identity is repo-relative,
  each record carries an ISO-8601 timestamp, and the reader maps a path to
  its member through the lock, so nested `CLAUDE.md` members with folded
  ids join too. Version 1 records still read.
- The synthesized tap hook runs `temper tap "$CLAUDE_PROJECT_DIR"`, so a
  hook firing from a subdirectory or a linked worktree appends to the
  primary checkout's log instead of a log that dies with the worktree.
  Re-run `temper install` to pick up the new hook command.

### Added

- The `skill` default contract carries `mention-reachable(paths, paths)`,
  matching the `rule` contract: a skill that mentions a member gated
  narrower than its own `paths` is flagged at advisory severity.
- `EdgeTargetFacts` gains `repoRootedPath` beside `path`. `path` stays
  relative to the host's projection directory; `repoRootedPath` is what a
  reader resolving from the repository root needs, such as a skill body
  citing a rule.
- `temper install` places a `PostToolUse` hook on the `Bash` tool that runs
  drift detection over emit-owned targets after the call. The `PreToolUse`
  guard binds tool-mediated writes only (`Write`, `Edit`, `MultiEdit`), and
  its messages now say so; a shell-mediated write is caught after the fact
  instead of never.
- The built-in kind × clause matrix is a reviewed snapshot in the test
  suite, so a clause present on one kind and absent on its twin is a
  visible diff rather than a default.
- An emit-then-discover round-trip test over every locus shape asserts
  that `check` reads back exactly the member set `emit` locked.

### Changed

- `coverage.checked` marks embedded-locus kinds as `<kind> (N embedded)`
  instead of `(0)`. The zero was correct for artifact members and read as
  "this kind's clauses never evaluate," which they do.
- The `InstructionsLoaded` tap hook registers for every documented
  `load_reason` (`session_start`, `nested_traversal`, `path_glob_match`,
  `include`, `compact`), not only `path_glob_match`, so always-on members
  loaded at session start are recorded.
- `emit` places the managed-projection banner on every frontmatterless
  markdown projection it owns. The banner was placed by `install`, so a
  member added by a flow that ran `emit` alone shipped bannerless and only
  an advisory noticed. The banner is now part of the projected bytes and
  the lock hash; on first `emit` after upgrading, frontmatterless
  projections that lacked it are rewritten once. The read face strips it
  from a member's `body`, so `extent`, headings, and layout contracts see
  authored prose only.

## [0.0.15] — 2026-07-27

### Fixed

- `temper tap` no longer loses records inside a linked git worktree. The tap
  resolved its log path against the working directory, so a hook firing in a
  worktree wrote to that checkout's own `.temper/tap.jsonl`, an untracked
  file deleted with the worktree. The tap now follows the worktree's `.git`
  file to the primary checkout (relative `gitdir` and `commondir` paths
  included, per gitrepository-layout) and appends there, so telemetry
  survives worktree cleanup. Hooks stay `temper tap` with no argument.
- `explain` on an unfilled requirement with a telemetry verifier narrated no
  evidence at all. It now reports counts against the declared member corpus
  when the requirement has no satisfiers.

### Added

- `explain <member>` states telemetry absence instead of staying silent:
  when the lock declares tap registrations and the log holds no records, the
  field strand says so. A quiet log with wired hooks is now a visible fact,
  which is how a stale pre-tap binary on PATH was caught in the field.
- `explain <requirement>` narrates a telemetry verifier's field record: per
  declared event, the record count and the distinct members and sessions,
  plus the declared members with zero records (the dead-weight list).

## [0.0.14] — 2026-07-24

### Fixed

- CRLF checkouts no longer lose managed-by notes or report drift on correct
  projections. The frontmatter reader accepted only an `---\n` opening
  delimiter, so on a working tree git rewrote to CRLF (the `core.autocrlf`
  default) every frontmatter-carrying projection read as frontmatterless:
  `emit` dropped the managed-by note it should have preserved, the schema
  modeline was never placed, `install` did not converge, and `check` reported
  `install.gate-installed` drift on files that were correct. The reader now
  accepts `---\r\n` as well. A repository that committed stripped notes
  restores them with one `temper install`.

## [0.0.13] — 2026-07-24

### Added

- `explain` now narrates a bare kind: `temper explain <kind>` (or
  `kind:<name>`) reports a kind's authoring guidance before any member of
  it exists.
- An embedded kind's guidance now reaches every delivery surface. It takes
  a kind-fact row in the lock (locus columns absent), so `schema` and
  `explain` carry its counsel like any other kind's.
- `schema --kind` serves every YAML-frontmatter kind, built-in or declared,
  not just `skill` and `rule`. Kind and field guidance ride the schema as
  editor hover text.

### Fixed

- `install`'s lift scaffolds each kind into its own directory. A command
  and an agent sharing a name no longer overwrite each other's scaffolded
  module.
- `schema --kind`'s help and unknown-kind error report the live kind
  domain instead of a hardcoded list.

## [0.0.12] — 2026-07-23

### Fixed

- Windows checkouts no longer report every projection as drifted. Drift
  comparison treats line endings as layout: a working tree git rewrote to
  CRLF (the `core.autocrlf` default) reads clean against the LF-emitted lock,
  while `emit` still writes LF.
- `check` no longer aborts on a malformed or nameless member. The load fault
  is collected as a diagnostic and the run continues, so every finding
  surfaces on a foreign harness instead of the first crash hiding the rest.
- `command` frontmatter is fully optional, matching Claude Code — the
  invocation name comes from the filename. A command missing `name` or
  `description` is no longer flagged.
- The `install` gate-installed advisory no longer fires on a repository that
  has not adopted temper (no `.temper/`).
- Placed hook commands fail loudly when the `temper` binary is not on `PATH`,
  instead of dying silently and leaving the gate unreported.
- An engine/SDK version skew reports a version hint rather than a bare
  payload-parse error.
- The session-start reporter surfaces advisory diagnostics, not only blocking
  findings.

## [0.0.11] — 2026-07-21

Entries begin here; earlier `0.0.x` releases predate this changelog.

### Added

- Kind guidance now flows into the contract and rides its findings: `schema`
  carries it as editor hover text and `explain` narrates it, so guidance and
  the member it advises travel together whether or not a clause failed.

### Fixed

- `when`-body clauses now evaluate at the guarded array element's scope, so a
  conditional requirement no longer judges the wrong element.
- Root-scoped `when`-guard findings no longer carry a stray `: ` prefix.
- Import recursion is capped at four hops (the guard was off by one).
