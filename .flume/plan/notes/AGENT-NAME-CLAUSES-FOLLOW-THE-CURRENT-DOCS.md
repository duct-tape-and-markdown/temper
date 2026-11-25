# Agent name: charset dropped, 256 bound added; `:` and leading `-` are a hold

Re-verified code.claude.com/docs/en/sub-agents on 2026-11-25 against the
full page (`.md`, 114 KB): "lowercase" appears nowhere, and the docs' own
example is `reviewer-v2` (a digit). The `[a-z-]` `allowed_chars` clause is
gone. In its place is the one documented rule the vocabulary can spell:
`max_len("name", 256)`.

Hold, which needs a ledger key: the docs skip a file whose `name` contains `:`
or starts with `-`. Both rules are decidable, but neither can be spelled:
`allowedChars` is a whitelist and `deny` matches whole values. The header of
`agentDefaultContract` names the hold and the widening that closes it, a
forbidden-character or leading-character predicate. For now the clause
guidance teaches both rules. No test pins the hold yet.

Every agent clause now cites 2026-11-25, `description` included, since that
row was re-read too. The uniqueness guidance still says "unique across the
whole tree". The docs now say "across subfolders" and describe a
same-directory collision, and the clause behaves the same either way.
