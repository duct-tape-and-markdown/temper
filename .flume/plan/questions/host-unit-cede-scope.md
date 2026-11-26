# Whom does a host's unit-subtree cede bind?

0096 rules that "a built-in host's template claim owns the files under its
unit at any depth, over memory's reach". The shipped cede
(`compose::cede_builtin_host_units`, c3a7515f) applies it against **every
other governing kind**, custom kinds included, and only for **built-in**
hosts.

The corpus underdetermines both edges:

- **Against whom.** 0096 names memory only. "Every other kind" also takes
  files from a custom kind that deliberately claims inside a skill's
  directory (no fixture does today). Narrowing to memory (or built-ins) is
  equally consistent with 0096's text.
- **Which hosts.** `representation.md` ("kind") says built-in and
  user-declared kinds are the same construct — ownership, not privilege —
  which argues a custom directory-unit host cedes the same way; 0096 says
  "built-in host".

Consequence for emit: `drift::member_projection_path` derives its ceded set
only from rows naming the kind as `base`, so it misses the host cede and can
place a (contrived) nested memory where check no longer discovers it. Rows do
not say which kinds are built-ins, so whether drift must learn that — or
reads the cede off the lock for any host — follows from the answer above.

Checked: 0084, 0096, 0099, `builtins.md` (skill), `representation.md`
("kind", "nesting").
