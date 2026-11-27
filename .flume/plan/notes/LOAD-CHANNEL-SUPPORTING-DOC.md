# LOAD-CHANNEL-SUPPORTING-DOC: derived `on-read` channel shipped

New `Registration::OnRead` (explain label `on-read`). It is derived and never
declared: no lock label parses to it, and the SDK `Registration` union does not
carry it. `CustomKind::from_kind_fact_row` supplies it only to a row named
`supporting-doc` (`kind::SUPPORTING_DOC`) that has no locus and no declared
registration. So a custom nested-file kind keeps an empty registration, as the
entry's notes require, and the lock and its engine stamp stay unchanged.

The registration-locus fence (`registration_locus_fault`) now counts only
channels other than `OnRead`. The derived channel never counts as a declaration.

`channel_load_moment` maps `OnRead` to on-demand without conditions. It is never
dead in `graph::dead_registration`, and the kind's `degree` clause still judges
reach. A skill whose body never references the doc still gets its
`supporting-doc.degree` finding (pinned).

The derivation is keyed on the kind's name. That fits while the spec names one
kind. If a second nested-file kind gains a documented load moment, a fact on the
host's template would be the better home.
