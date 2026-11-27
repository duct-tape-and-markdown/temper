---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
paths: ["src/**","tests/**","benches/**","sdk/**","build.rs"]
---
# Engineering posture

How code enters this repository. Each line below is an invariant, and the
engineering skill holds its full section: its **tell** (the observable sign of
a break at one place) and its **holder** (the strongest thing that holds it
today, on the ladder: a type, then a test or pin, then a clause, then prose and
review). A **misfit** is code the next holder up would refuse. A **downgrade**
is a fix that holds the property lower than before, and is residue.

- One job, one home: extend the existing surface before adding one.
- Libraries before hand-rolls.
- A shared concept is one type.
- Derived state is computed, never stored beside its source.
- The fix lands at the mechanism.
- The gauntlet corpus holds every composition the model claims legal.
- Cost scale is hoisted, and pinned by count: a run reads each input once, through one owner.
- A green verdict is proven non-vacuous.
- A reader refuses or reports what it consumes.
- A fix ships the test that would have caught it.
- A seam gate reads what the real writer wrote.
- An export earns its consumer.
- Narration is the ladder's bottom rung.
- The tree stays flat.
- The provider face is data.

For a procedure in this area, use the `engineering` skill: it routes to one topic per procedure.
