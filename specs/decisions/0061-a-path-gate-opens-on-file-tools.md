# 0061 — a path gate opens on file tools

- **Date:** 2026-09-24 · **Status:** accepted

## Context

An adopter's harness audit (item 3). A rule gated on its `paths` recorded
0 path loads across 161 sessions that spent their time reading inside the
gate: about 3,900 shell calls against 106 file-tool reads. `builtins.md`
said a path gate opens when "a matching file is in play" or "is read", and
the shipped `mention-reachable.paths` guidance says "until Claude reads a
matching file". Probed on Claude Code 2.1.281, 2026-09-24: `cat` through
the shell tool on a path under `src/**` neither loaded a `paths: src/**`
rule nor listed a `paths: src/**` skill; a `Read` of the same file did
both. The adopter also reports that some sessions are steered toward shell
reads. Ruled by John 2026-09-24.

## Decision

**A path gate opens when a file tool touches a matching path; a shell or
search read opens nothing.** `builtins.md` states it for skills and rules
alike, cited to the probe, and a reach judgment over a path gate says it
cannot see reads made any other way. temper judges reach; it does not
deliver.

## Rejected

- **A `temper inject` verb** (the adopter's option b): a PostToolUse hook
  on shell and search tools that pulls paths out of the call and injects
  unseen gated rules once per session. It makes temper a second loader
  running on a heuristic: paths held in shell variables stay unresolved,
  and a file-tool read in the same batch delivers the rule twice. The
  intent's line is that temper makes a harness correct and leaves
  delivery to the harness. An adopter who wants an injector authors it as
  a `hook` member, and temper types and gates it like any other.

## Consequences

The `mention-reachable.paths` guidance and the rule kind's `paths`
guidance take the file-tool wording (code, the built-in lock's source).
The adopter's evidence is the kind of observation the tap exists to
surface; the tap's own gap is 0062.
