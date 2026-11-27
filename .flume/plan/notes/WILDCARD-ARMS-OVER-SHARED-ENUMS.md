# WILDCARD-ARMS-OVER-SHARED-ENUMS

Both sites shipped as named: `gate::site_class` now answers
`Content::File | Content::Fields => None` after the governed-File guard arm,
and `without_body_prose` in tests/it/rule_contract.rs lists every non-`When`
Predicate variant (38 of them). A new Content or Predicate variant now fails
to compile at both sites.

The drift.rs site d2f1d4f9 named was not looked at, as the entry scoped it out.
