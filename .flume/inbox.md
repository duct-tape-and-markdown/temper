<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- (John's routing, 2026-07-06 — the pre-v0.1 residue set, curated from the
  held audit; derive entries from the CORPUS anchors, not from this note.)
  Three demolitions whose spec text already exists and whose gates are
  discharged: (1) **the package-era noun** — 10-contracts.md:207 names the
  residue ("the `package` facet on requirements and the resolver behind it …
  retires with the noun"); living code: `PackageResolver`,
  `Requirement.package`, `Membership.source_package`/`conforms_to`
  (compose.rs), the drift.rs `RequirementRow.package`/
  `MembershipRow.source_package` columns, the sdk `RequirementRow.package`
  column, and the `kinds/*/KIND.md` + `packages/*/PACKAGE.md` product tree
  (whose bodies claim "build.rs embeds it" while build.rs is `fn main(){}`).
  Likely a short blockedBy chain, engine → lock rows → sdk column → tree.
  (2) **the reachability dial** — retired by dadfd54 (40-composition "there
  is no fifth field"; 45-governance "an ordinary `reachable` clause"); living
  code: `reachability_from_declarations` (main.rs:1188, read at :567), the
  conditional `graph::reachable` opt-in, the drift.rs reachability fact
  discriminator with its severity scalar. (3) **the sdk requirement recut** —
  10-contracts.md:227 (`Requirement = means·kind·required·clauses?·
  verifiedBy?`, facet spelling rejected); living code: contract.ts still
  enumerates `count?/unique?/membership?/degree?` and lacks `clauses?`;
  disjoint sdk/ entry (genre-unship precedent), SEAM_VERSION bumps both
  sides. Related open question to REGISTER (not pending): `(authority-home)`
  — declarations.ts:151 unconditionally emits `{fact:"authority",
  value:"shared"}` but "shared" is corpus-uncoined, the vocabulary is
  note/warn/block, and `Harness` has no authority field; where does the
  authored posture live in the four-field assembly? Needs John. Do NOT spiral
  into spec-text corrections beyond these anchors — text-only reformulations
  ride John's next ceremony, not this queue.
