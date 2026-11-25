# Built-in guidance states load semantics

Shipped: rule, skill, command, and supporting-doc guidance each say what reaches
context and when. Each of the four kinds now carries a kind-level `cite`. Before
this, `command` and `supporting-doc` had no guidance at all. The rule text no longer
says "always-on".

Beyond `files[]`: `examples/base-harness/.temper/lock.toml` carries the rule and
skill rows, so `emit_program_runs_the_shipped_example_harness` failed until I
re-emitted it. Future guidance edits to those kinds touch that lock as well.

Human chore left open: the dogfood `.temper/lock.toml` (and any `.claude/`
projection carrying these rows) needs a re-emit outside build's fence.

One claim in the new skill text rests on the skills docs' invocation table rather
than on a field doc already in `builtins.ts`: that
`disable-model-invocation: true` drops the description from context. Recheck
it at the next docs refresh.
