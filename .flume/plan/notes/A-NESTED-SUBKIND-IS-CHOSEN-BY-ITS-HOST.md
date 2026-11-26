# A nested subkind is chosen by its host

Shipped the nested-file case. `resolve_subkinds` binds a subkind whose base also governs
no locus, ceding nothing (the host template places it). `extend` accepts a `nested-file`
base, refuses `governs`/`exclude` on it, and takes `template:` to swap the host layer
whose child is the subkind's base.

Not taken: embedded subkinds (Cascade's `invariant`). `extend` still refuses an embedded
base: the engine's embedded composition keys members by the host's `templates` name, and
an embedded kind takes a kind-fact row only with guidance/cite/leaves, so the subkind
could vanish from the kinds map. That half wants its own entry.

`kindsInPlay` (`sdk/src/declarations.ts`) now draws in a file-template child whose facts
carry a `base`, so a host alone puts its subkind child in the lock. A non-subkind
nested-file child still reaches the lock only via `expect`/members; a host template
naming a kind with no row discovers nothing, silently. Worth a plan look.
