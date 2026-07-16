# base-harness

A starter harness authored as a temper program, whole: the Claude Code
artifacts (memory, rules, skills, hooks) and a documentation corpus are
typed members of one program, their cross-references are edges the gate
resolves, and the files on disk are projections. The corpus is
spec-authoritative: documents are declared intent, and the code under
`src/` reconciles toward them (see `docs/decisions/authority-arrow.md`).

The governed code is a toy on purpose, a three-file checklist summarizer:

```sh
$ node src/main.js TODO.md
2/3 done
```

What the harness demonstrates:

- **The harness is organized domain first, mechanism second.** Five
  domains (conduct, orientation, standards, operations, governance) are
  declared as requirements in `.temper/harness.ts` on day one, and every
  member names the domain it fills with a `satisfies` entry. Growth is
  additive: new knowledge joins an existing domain. Conduct, orientation,
  and governance are floored (`required: true`); remove the conduct rule
  and re-emit, and `temper check` fails:

  ```
  requirement.unfilled

    x required requirement `conduct` is unfilled: no artifact declares a `satisfies` link naming it
  ```

- **One fact, one home.** Commands and expected outputs that more than one
  surface states live once in `.temper/facts.ts`; the memory's map, the
  verify skill's procedure, and the settings manifest all interpolate the
  same constants, so one edit moves every projection and the surfaces
  cannot contradict each other.
- **A skill can be path-gated, and its claims are edges.** `verify-summary`
  declares `paths: ["src/**", "TODO.md"]`, so it registers only when the
  work touches what it knows how to verify — and its body mentions the
  entrypoint it tests, so moving `src/main.js` fails the gate:

  ```
  x `skill:verify-summary` mentions `source:main`, which resolves to no member in the discovered corpus
  ```
- **Typed claims can go false, and the gate catches them.** Each system
  document names the `src/` files implementing it (`implemented-by`,
  resolved within the `source` kind). Delete `src/scan.js` and
  `temper check` fails:

  ```
  graph.route

    x `scanner` `implemented-by` routes to `scan`, which resolves to no `source` artifact
  ```

What the documentation corpus demonstrates:

- **Documents compose from declared members.** A system's invariants, a
  flow's steps, and a decision's rejected alternatives are embedded
  members (`.temper/kinds.ts`), each with its own markdown rendering;
  `emit` composes the documents under `docs/` from them. Editing a
  rendered document is drift; the discipline is construction, not
  convention.
- **Derived renderings replace authored duplicates.** A flow's steps each
  carry an edge to the system they happen in; the participants line in the
  projected document is rendered from the steps, so it can never disagree
  with them. There is no participants field to forget.
- **Lifecycle is positional and typed.** Superseding a decision is the
  `supersede()` operation: the replaced ruling's record lands in
  `docs/decisions/superseded/`, where the successor edge is required by
  the field's own type.
- **Both content faces, deliberately.** `docs/glossary.md` is the one
  layout source, the authored home for prose-first content, read under its
  declared layout, each term an addressable member.

## Run it

From this directory, with the `temper` binary installed and the SDK built
(`pnpm -C ../../sdk install && pnpm -C ../../sdk build` from here, once):

```sh
npm -C .temper install
temper emit
temper check
temper explain scanner
```

`explain scanner` narrates the member's place in the graph, including the
outward edge and the step that points at it:

```
Edges out (the resolved references it declares, the exact set the gate ranges over):
  • it points at `scan` (source) via its `implemented-by` field
Edges in (the resolved references that point at it):
  • `summarize/step/scan/in` (requirement) points at it via its `mention` field
```

Standalone (outside this repository), replace the `file:../../../sdk`
dependency in `.temper/package.json` with the published `@dtmd/temper` and
use the installed `temper` binary.
