# Discovery should fence Claude Code's own runtime folders, and group the rest of the runtime output

From the same real-harness probe (0.0.25). `coverage.unclaimed-entry` fired on Claude Code's own worktrees folder under `.claude/`, and on six runtime-output folders the harness writes (logs, state, scratch, ledgers, evals, one custom). The worktrees folder is runtime state Claude Code writes, so discovery fences it the way it fences `.git`. Cite the folder from the docs first. The adopter's own runtime folders are its to ignore (gitignore, which discovery already honors). Their coverage notes should be one line per folder, never one finding per entry, so a real harness's coverage report stays readable (`specs/model/adoption.md`, discovery).

Also noted: a check of the committed harness read the gitignored local settings file and said so in its header. That is by design (a local member is read-side, per `representation.md`, "locus"), so no change.
