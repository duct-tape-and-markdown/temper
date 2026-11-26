# Host-unit cede generalized and narrowed

`compose::cede_host_units` (was `cede_builtin_host_units`) now takes every
directory-unit host with a template, custom or built-in, and cedes its unit
subtree only from loci that `kind::reaches_into_at_any_depth` the subtree: a
pattern carrying `**` whose literal root above the `**` does not already lie
inside the host's unit. Fixed-depth loci, and any-depth loci rooted inside the
unit (`domains/*/notes/**/*.md`), keep their documents.

"Overlaps" is approximated: an any-depth reach starting outside the unit cedes
even where its tail can never match a file there (e.g. `command`'s
`.claude/commands/**/*.md` gains the skill subtree). Matching is unaffected,
but those patterns show up in `ceded`. THE-LOCK-RECORDS-EACH-CLAIM-RESOLUTION
should decide whether the recorded resolution needs an exact overlap test.

Still open, as before: `drift::member_projection_path` derives ceded patterns
from subkind rows only, so emit does not see the host cede. That is the
blocked entry's job.
