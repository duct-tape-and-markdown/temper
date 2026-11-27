# Does a playbook's member carry `satisfies`, and in what shape?

BASE-HARNESS-HYDRATES-A-PLAYBOOK (98581749) added
`PlaybookInit.satisfies?: { rule?, skill? }` to the SDK so the example's
`standards`/`operations` requirements fill. `specs/distribution.md` ("What
ships — three channels") names the playbook's axes and carries no
`satisfies`. `gateHooks()` has the same gap (it takes no `satisfies`).

Checked: distribution.md's playbook axes, specs/model requirement text,
`ReferenceInit.satisfies` (a flat list). A flat list fails here because a
requirement that declares `kind: skill` refuses the playbook's rule
(`requirement.<name>.kind`). That is why the SDK keys it per surface.

Open: should the spec ratify per-surface `satisfies` on a playbook (and an
equivalent on `gateHooks()`), or rule against it and have requirements
filled some other way? Until this is answered the SDK surface is
unratified behavior.
