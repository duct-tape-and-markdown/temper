# Field report: four drift classes in hand-built harnesses

This is an observation, not a ruling. Another harness's design session surveyed patterns its owners had each built by hand more than once, across a monorepo, a coordinating desk and a new project. It reported what goes wrong when those patterns are written by hand. Four of the failures are drift between surfaces that temper types. Each may already be covered by a shipped clause, by a class decision (0101 reads as the runtime reads, 0103 a finding is true for its reader), or by nothing:

1. **Permission patterns that miss path forms.** An `ask` or `deny` rule over a path does not match a relative or absolute spelling of the same file, or a shell write that sidesteps the Edit tool. Settings, `permissions`.
2. **A subagent override whose name does not match the built-in it means to replace.** The override silently becomes a second agent. Agent kind, name against the runtime's built-in agent names.
3. **A subagent that silently inherits the default model** where its role was meant to pin one. Agent kind, `model`. Requiring a model is the consumer's choice, so this would be an opt-in clause at most, never a default.
4. **An allow rule that no longer matches the command as actually invoked.** The prompts come back and get approved blind. Settings `permissions.allow` against a hook's or a script's command reference.

A fifth class, a hook command that resolves only from one working directory (exit 127), is already held by 0086.

Route each: covered (cite the clause), class-covered with spec text missing (a question), or uncovered (a question). Nothing here asks for a default that encodes taste.
