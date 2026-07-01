+++
# An inadmissible package: `word_count` is not in the closed vocabulary, so this
# package is rejected at load — the exact unsound-proxy trapdoor the algebra keeps
# shut (`specs/10-contracts.md`, "Decision: kill the heuristic rule registry"). It
# reaches temper's parser exactly as the clean package does; the difference is the
# definition check, not the medium.

[[clause]]
severity = "required"
predicate = "word_count"
field = "description"
min = 10
+++
# Deliberately broken package

This body would be package guidance — but the header never loads, because a clause
names a predicate outside the closed algebra.
