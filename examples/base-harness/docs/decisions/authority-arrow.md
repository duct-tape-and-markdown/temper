Repositories that document themselves must choose which side is authoritative when docs and code disagree. The conventional answer makes code the source of truth and docs a trailing description, held close by a per-change duty ("update the affected docs") that reviewers enforce. In a repository whose code is substantially agent-authored, that answer reads intent out of an artifact — the one direction this corpus forbids.

## Ruling

The corpus is authoritative. The documents under `docs/` render declared intent; code and configuration reconcile toward them. When docs and code disagree, the code has the bug or the corpus needs an authored amendment — never a silent doc edit to match the drift.

## Consequences

Documentation duties invert: there is no "remember to update the docs" step, because the docs move first and the delta drives the work. The cost is honesty about the return path — evidence against the corpus must enter through an authored surface (an amendment, an open question), which is slower than hot-patching code and letting docs decay. That cost is the point.

## Alternatives

### Code authoritative

Rejected. Rational when humans author the code and decisions live nowhere else, but agent-authored code is output, and deriving intent from output is mining. It also defeats the docs' own purpose for agents: a doc that trails code cannot be trusted at planning time.

### Per change documentation duty

Rejected as the primary mechanism. A convention held by reviewer vigilance fails silently, one forgotten change at a time. It survives here only as a courtesy, downstream of the arrow — see the superseded ruling this decision replaces.
