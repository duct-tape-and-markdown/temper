---
paths: [".flume/plan/open-questions.md"]
---
# fork-lifecycle — open-questions.md holds OPEN forks only

- Each fork is keyed `(slug)` so a pending entry can declare
  `dependsOnForks: ["slug"]` and be held until resolved.
- **Resolution = encode the ruling** (a corpus Decision, or the resolving
  commit body) **and DELETE the record** — git history is the archive;
  "kept as the decision record" is a retired category.
- Reconciliation evidence (DATUMs) goes in the plan commit body, never
  appended to a record.
- The file is inlined whole into every plan prompt: a dead line is a
  per-tick context tax, a stale record a latent misroute.
- The "Kept on purpose" section lists deliberate asymmetries — each a
  choice with a condition. Re-read it every tick; never file work
  against one; when its condition arrives, surface it, don't silently
  break it.
