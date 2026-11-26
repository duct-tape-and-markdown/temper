# A file locus governs through templates: three seams for plan

Shipped: `Governs.globs` (non-empty), the lock's `governs_glob` written as an
array (a bare string still reads as one template), the SDK `Locus.glob` taking
`string | Templates`. Discovery, membership, cedes, subkind containment,
collisions, guard loci and `explain` each walk every template.

- **Exclusions stay kind-wide.** 0109 says each template carries "its own
  exclusions". An `exclude` here is one list per kind, and each entry must be
  matched or reached by some template. Because exclusions are literal paths,
  that subtracts the same set. A spec reading that wants per-template
  exclusion columns needs a ruling (`representation.md`, "locus").
- **Placement across several templates is interim.** `drift::placing_template`
  and `emit.ts`'s `placingTemplate` pick the first template whose star takes
  the name, or whose starless filename stem is the name's leaf
  (`apps:web:AGENTS` goes to `**/AGENTS.md`). 0109's "placement is the
  member's own path fact" supersedes this, so the entry that builds it should
  retire both copies.
- **A manifest is one template.** `manifest()`, `manifest_target_path`,
  `bundle`'s write path and `main::manifest_path` read `sole_glob()`. A
  registration row with several templates refuses at lift.
- **The dogfood lock** (`.temper/lock.toml`, not writable here) still spells
  bare strings. It reads fine, and the next `temper emit` rewrites them as
  arrays, so the human re-emit will show that diff.

Not done here: the `AGENTS.md` absence gate and memory claiming the second
template (MEMORY-CLAIMS-AGENTS-MD-…).
