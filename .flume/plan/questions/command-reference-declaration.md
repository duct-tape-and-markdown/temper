# Where is a command reference declared, and what reads it?

Driver: 0100 (`authoring.md`, "The SDK", command paragraph) and 0098 as
amended (`pipeline.md`, "Emit": the typecheck is "a command reference").
Neither is derived until this is answered; 0098 rests on 0100's shape.

The corpus fixes the three variants and that runners reach a command "by
declared edges", with "a command nothing runs" an advisory finding. It
does not fix:

1. **The declaration's home.** A command is "stated once": a member of a
   new shipped kind (`builtins.md` lists fourteen and none is it; the name
   `command` is the legacy skill kind), or a harness-level declaration
   with its own lock row family. Edges (`contract.md`) run between
   members, which points at a kind, but no kind is spelled.
2. **The advisory finding.** Which verb raises "nothing runs it" (check
   or emit), and its finding id.
3. **The lockfile read.** "Its version the lockfile's": which lockfiles
   (`pnpm-lock.yaml` needs a YAML reader outside the sanctioned crate
   set), what the version is used for (lock row? check?), and what a bin
   absent from the lockfile reports.
4. **The default.** "A temper package bin unless declared": the package
   name and bin the default renders, and whether it replaces `temper` on
   the PATH for an unrepresented harness.

Likely also answers `settings-referenced-scripts` (option 1, via
`{ file }`). A ruling amends `authoring.md` or `builtins.md`.
