# Does the contract grow a predicate correlating two edges' sources at one member?

Driver: GH #59. Shape: `writer --writes--> target <--presents-- presenter --gatedBy--> condition`; the defect is a gated presenter whose target has an ungated writer. The vocabulary misses it by construction: `degree` counts edges at one member, `membership` tests one field against a fixed set, `reached-from` follows reachability, never a correlation between two sources. A ruling is a `specs/model/contract.md` "clause" addition, human-authored.

Stance (John, 2026-09-29): a general path predicate is rejected, since it makes the contract language a query language. The narrow sibling form ("every sibling source into X over field F carries the edge G that the source over field H carries") clears the evidence bar only when a driver authors it and has to delete it; the driver is a second corpus, a feature and never a founding assumption (`specs/model/representation.md`, "Reach"). Open for that evidence. No dependents.

**Second driver (2026-11-27, 0115).** A field report from a second harness: a `permissions.allow` rule that no longer matches the command a hook or script actually invokes, so prompts return and get approved blind. That is a correlation between a settings permission and a `tool` member's rendered command, the same shape as this question.
