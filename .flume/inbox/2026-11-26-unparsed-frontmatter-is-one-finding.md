# A member whose frontmatter does not parse draws one finding, never one per unset field (0103)

An instance of class 0103 (a finding is true for its reader). After bdfe4035, a skill with an unquoted `description: Use when testing: colons` loads, as 0101 rules, and draws three findings for one root cause: `member.non-portable-yaml`, `skill.required.name` and `skill.required.description`. The two `required` findings follow from the docs' "loads with no fields set". Each is true, but neither tells the reader anything the YAML finding doesn't, and one quoted scalar fixes all three. On the probed real harness that is about 165 findings where there are 55 causes.

Entry: when a member's frontmatter did not parse, report the non-portable-YAML finding alone. It names the fields the runtime reads as unset ("Claude Code reads this skill with no `name` or `description` until the scalar is quoted"), and the field clauses over that member stay silent. Acceptance:
- **new:** the fixture skill draws exactly one finding;
- **new:** that finding names `name` and `description` as read unset;
- **kept:** an agent with unparsed YAML still draws `member.load-fault`;
- **kept:** a skill whose YAML parses with no `description` still draws `skill.required.description`.
