# Reference topics say requirement

Shipped. The reference skill's index heading is `### Requirements`, its index
lines say "the `<r>` requirement", a kind topic's section is `## Requirements`,
and a requirement topic is `requirement-<r>.md` titled "# The `<r>` requirement".
The SDK test titled "a contracted requirement's reference topic is titled and
named as a requirement, never a domain" holds it (SDK gate; no tests[] line).

Owed by humans, outside build's writable paths:
- The dogfood's `.claude/skills/temper/domain-*.md` move to `requirement-*.md`
  on the next `temper emit` of `.temper/` (emit reaps the old names).
- The release cut owes a migration-note line: an adopter's committed
  `.claude/skills/temper/domain-*.md` are reaped and re-emitted as
  `requirement-*.md`; links into the old paths break.
