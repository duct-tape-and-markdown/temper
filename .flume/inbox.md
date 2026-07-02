<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- DECLARED-ADAPTER WAVE (specs/15-kinds.md, new Decision "the adapter faces
  are declared"): a kind declares its projection format (`format =
  "yaml-frontmatter"`, unit shape file-vs-directory); one generic frontmatter
  adapter implements it for import/re-add/apply/drift; the per-kind modules
  src/skill.rs + src/rule.rs and their typed IRs RETIRE (files deleted at the
  end of the wave); drift's per-kind field serializers go with them. The
  `kinds/*/KIND.md` format/unit declarations are curated territory — surface
  the need and the humans add the two lines; do NOT file kinds/ paths.
  Sequencing: open with an equivalence pin — the existing import-idempotence
  and apply round-trip insta snapshots are the baseline; add any missing
  byte-fidelity pins (YAML ordering, unknown keys, no-frontmatter rule,
  companions) BEFORE swapping implementations. After the wave drains,
  reconcile AGENT-KIND against the new architecture (it should become: two
  curated data files + tests, near-zero engine code).
