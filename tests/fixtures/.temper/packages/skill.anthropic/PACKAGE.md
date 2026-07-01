+++
# A package authored on temper's own surface: clauses in the fenced header, the
# guidance prose below. Its clauses mirror `contracts/skill.anthropic.toml` so a
# test can prove the two forms decide identically — the machinery-first fixture
# the resolved sequencing calls for.

# --- name: required, charset, length, reserved words ---

[[clause]]
severity = "required"
predicate = "required"
field = "name"

[[clause]]
severity = "required"
predicate = "min_len"
field = "name"
min = 1

[[clause]]
severity = "required"
predicate = "allowed_chars"
field = "name"
ranges = ["a-z", "0-9"]
chars = "-"

[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 64

[[clause]]
severity = "required"
predicate = "deny"
field = "name"
values = ["anthropic", "claude"]

[[clause]]
severity = "required"
predicate = "name-matches-dir"

# --- description: required, non-empty, length cap ---

[[clause]]
severity = "required"
predicate = "required"
field = "description"

[[clause]]
severity = "required"
predicate = "min_len"
field = "description"
min = 1

[[clause]]
severity = "required"
predicate = "max_len"
field = "description"
max = 1024

# --- body: progressive-disclosure budget (advisory) ---

[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 500

# --- no Cursor frontmatter leaking in ---

[[clause]]
severity = "required"
predicate = "forbidden_keys"
keys = ["globs", "alwaysApply"]
+++
# Anthropic skill package

The best-practice prose the clauses cannot encode — the second channel, delivered
to the authoring agent and (via the emitted schema) to humans as hover docs.

A skill's `description` is its whole retrieval surface: write it in the third
person, name *when* to use the skill, and keep it concrete. The clauses gate the
decidable floor (a name is a slug, a description exists); this prose carries the
taste the algebra deliberately cannot.
