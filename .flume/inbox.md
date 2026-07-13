<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- PACKAGING-CHANNELS partially shipped in-session (John's ruling, 07-11):
  channel 2's linux-x64 + win32-x64 first cut is live — release.yml builds
  and idempotently publishes @dtmd/temper-{linux-x64,win32-x64} then the
  SDK; @dtmd/temper@0.0.7 carries the launcher (sdk/bin/temper.js) and
  exact-pinned optionalDependencies; verified by a no-cargo scratch-dir
  `npm install` + `npx temper check --harness`. Deliberate deviation from
  the entry's filed shape: the launcher + optionalDependencies live on the
  SDK package per the spec's own text ("pinned by the SDK",
  specs/distribution.md, "What ships") — the root package.json stays the
  private flume manifest untouched; the entry's edit-file claim is stale.
  The entry as filed is SUPERSEDED, not re-scopable (John, 07-11): both
  its file claims are dead — `.github/workflows/release.yml` now exists on
  disk (so the `new` claim fails the refs gate the moment the entry
  opens), and the root-package.json launcher was ruled out by the spec
  text. Retire it and file the remainder fresh: darwin binaries (Apple
  notarizing, on John), channel 3 (plugin + marketplace.json), standalone
  tag assets (workflow path exists, unexercised until a tag), and the
  binary self-reporting its crate version (0.1.0) while npm carries
  0.0.x — lockstep lands at the v0.1 tag. observed at 56012d0

- Ruled, needs its docs line (John, 07-10, in session): a host that
  interleaves prose with typed members **is a layout source document** —
  temper builds no composed text-with-members posture, and programmatic
  structure-splicing (fabricated `Text` templates, self-rendered fences
  spliced into `text()` bodies — the centercode testbed's own hack, broken
  by two consecutive SDK shape changes) is territory temper won't go. The
  composed side stays what it is: `blocks()` for all-members bodies,
  `text()`/`file()` for prose. Remainder to route: (a) the consumer
  guidance nowhere written — "if your host mixes prose and members, declare
  a layout and author the document" belongs in the docs' custom-kind path;
  (b) small residue on the composed side: a `render()` hook's output is
  still wrapped inside the ```` ```member.<kind> <key> ```` fence
  (`sdk/src/emit.ts:143`) — if deliberate (a visible member seam in a
  projection), the rationale is unwritten (the nearby comment argues only
  byte-stability for `render`-less kinds); if residual, unfencing is one
  line. Observed at 8c00159, re-verified at c2f8a2c.

- A 0019-loud instance the shipped refusals don't cover yet:
  UNTEMPLATED-NESTED-MEMBER-LOUD (4752b06) rejects the blocks-side orphan
  (a declared value no host templates), but a `member.<kind> <key>` fence
  sitting in a `text()`/`file()` body — the entire input class the
  pre-0018 fold used to read — is dead text with no finding on any path.
  Live repro: the centercode harness was green pre-0018 with ~57 embedded
  members resolving at leaf grain; it re-emits and re-checks green today
  with that whole layer silently gone (`explain`: "carries no nested
  member"; coverage: "21 documents carry no nested members"). With
  0019-content ruled, a `member.` fence is meaningful nowhere except as
  `blocks()` default rendering (layouts reject per-entity fences by
  decision), so one appearing in a text body and naming a kind the assembly
  declares is near-certainly an author error. Cheap refusal: fence info
  string parses as `member.` + a declared kind, no `nested_member` row
  matches the host → finding ("a member-shaped fence no declaration
  backs — dead text or a missing declaration"). Observed at 8c00159.

- Layout probe results (centercode testbed, migrated 07-10 off the fence
  hack onto clean prose per John's ruling; a relocation-shaped `rule` kind
  row was then declared with `content: [prose, collection(directive)]`):
  (1) **Good news, a prediction refuted**: emit honors a relocated
  built-in's declared content — `temper::layout::unadmitted` refused loud
  and located ("cls.md: heading `CLS (Shared Libraries)` fits no declared
  layout region") before writing a byte. The 0019-loud posture works
  end-to-end on this path. (2) **Untested divergence to close**: the gate
  path resolves a built-in kind via `overlay_builtin_kind`
  (`src/main.rs:896-919` at c2f8a2c), which lifts exactly `governs` +
  `templates` —
  not `content` — so a document that *fits* its layout would emit rows
  fine while `check`'s `layout_unit` reconstruction may silently fall back
  to the plain frontmatter read for built-in kinds. Same gap class T18 was
  (templates overlay), one fact later; unverified past emit since no
  testbed document fits yet. (3) **The consumer design question**: a
  layout binds the whole kind — every member's document must fit it — but
  a real rule corpus is heterogeneous: directive-carrying documents
  (cls.md, protocol.md) sit beside one-sentence pointer rules
  (omegaone.md, web-ui.md) that have no collection heading at all. 0019's
  own answer is "two kinds" — but both must govern `.claude/rules/*.md`,
  and what happens when two kinds share a governs glob is unspecified
  (collision? first-match? partition by fit?). That's the piece a consumer
  hits immediately on adopting layout for any built-in kind. Observed at
  8c00159.

- Corpus hygiene, small: two decision records wear the number 0019 —
  `0019-loud-or-nothing.md` (07-09, "sixth invariant") and
  `0019-content-is-a-declared-kind-fact.md` (07-09, "invariant 7", cited
  as "0019" by 0020's context). Decisions are addressed by number (status
  lines say "superseded-by 0018"), so a shared number is a live addressing
  ambiguity the first supersession will trip over. One of them renumbers
  to the next free slot — mechanical, but it touches whichever
  cross-references exist, so it's a plan-routed entry, not a hand-fix.
  Observed at 8c00159, still live at c2f8a2c.
