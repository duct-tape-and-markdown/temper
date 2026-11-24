# Derive 0078: one fact, one spelling (GH #60)

Spec commit a97f66b2 landed at the adoption cursor; derive `specs/model/authoring.md`, "The SDK", "One fact, one spelling".

Derivable now: #60. `sdk/src/declarations.ts` (~597) lowers `required: requirement.required ?? false`; `specs/model/contract.md`, "requirement", makes the at-least-one-satisfier clause at error severity the shipped default. This repo's `.temper/harness.ts` sets `required: true` to get it, so it is a consumer of the fix (human territory: route that half to a question, not build's fence).

Audited unbound restatements in `kind()` (session audit 2026-11-24; verify on disk before filing): layout slot names in both `content.regions[].slot` and `leaves` with no subset check (`sdk/src/declarations.ts` ~388); a manifest path in both `locus.glob` and `collectionAddress.manifest` (`sdk/src/builtins.ts` ~554/559, ~664/668); `registration` required on embedded kinds whose only legal value is empty (`sdk/src/kind.ts` ~281). Each is an entry only where the single spelling is derivable; where it needs a design call, file a question.

Not derivable, rulings pending in-session: #61 (import fingerprinting), #64 (placement vs match), #65 (gate command spelling).
