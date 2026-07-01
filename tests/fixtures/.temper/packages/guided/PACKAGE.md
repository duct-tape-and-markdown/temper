+++
# A package whose clauses carry colocated `guidance` — the hover-sized *why*, keyed
# on the clause it explains so it can never dangle from it. When a member breaks the
# clause, this prose rides its diagnostic: the violation is the teaching moment.

[[clause]]
severity = "required"
predicate = "required"
field = "name"
guidance = "Every skill declares a `name` — it is the slug the harness binds to."

[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 64
guidance = "Keep the name short and slug-like; it becomes a directory and an id."
+++
# Guided package

Package-level guidance body. The per-clause guidance above is the just-in-time
channel; this prose is the always-on rationale for the authoring agent.
