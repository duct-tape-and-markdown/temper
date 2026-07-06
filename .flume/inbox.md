<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- (verification, 2026-07-06) PACKAGING-CHANNELS' parked reason cites
  "marketplace publish, signing/publish creds" — partially UNVERIFIED and now
  corrected against code.claude.com/docs (retrieved 2026-07-06, session
  verify): a plugin marketplace is a git repo + `.claude-plugin/
  marketplace.json`, added via `/plugin marketplace add owner/repo` — NO
  credentials, registration, or signing exist anywhere in the plugin system
  (trust is social; community-marketplace submission is a free web form
  gated by `claude plugin validate`). npm creds are DONE (07-05). The only
  credential-shaped release remainder is optional Apple notarization for the
  standalone mac binary (decide-at-release). Re-word the parked reason to:
  needs the release workflow build-out + John's decide-at-release calls
  (notarization, USPTO screen); the creds framing is stale.
