# Plan state

- **Phase:** reconciled the queue after build shipped README-RECONCILE-SURFACE;
  verified the CLI surface and README/AGENTS on disk against 20-surface's verbs.
- **Last shipped:** README-RECONCILE-SURFACE (build 1656abe / chore 1dd6d60) —
  README/AGENTS carry the ratified surface; verified on disk: no stale
  `import`/`diff`/`apply` verb refs, real subcommands are init / check / schema /
  emit / guard / install / bundle / explain.
- **This tick:** no change to pending. README-RECONCILE-SURFACE is drained from
  the queue (build removed it on ship). Inbox empty. No un-gated pickable work —
  the six-noun demolition core is SDK-front-door-gated and PACKAGING-CHANNELS is
  parked on human release creds; filing onto that foundation would invent intent.
- **In flight:** nothing pickable. PACKAGING-CHANNELS stays parked (npm scope in
  motion — John's @dtmd; marketplace/signing creds).
- **What's next:** John's hand unblocks the rest — the SDK-primary front door
  (demolition core, `init` re-shape), the corpus shadow of the dogfood
  deactivation, and the @dtmd package that un-parks PACKAGING-CHANNELS.

Plan continues: no — the queue is reconciled and quiescent, the inbox is drained,
and every remaining thread is human-gated (SDK front door / release creds). There
is no `open` entry for build to pick and nothing left to re-plan this turn.
