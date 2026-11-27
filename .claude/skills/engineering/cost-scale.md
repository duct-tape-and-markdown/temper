<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# Cost scale is hoisted, and pinned by count

Work that scales with the consumer's input — tree walks, file reads,
glob compilations — is hoisted by design: whole-input work computes
once per run and is shared, never recomputed per kind, member, or call
site. The expectation is enforced the only way this repository enforces
expectations — decidably:

- **A run reads each input once, through one owner.** A verb invocation
  holds one owner per input it judges (the tree's file index, a file's
  bytes, the parsed lock), and every judge, reporter and verb branch reads
  through that owner, never from disk. The owner fills an input on first
  use, so a verb pays only for what it reaches, and a run that writes
  reads its own writes back through it. The discovery index is built in
  one place behind the fences every discovery reader shares: the ignore
  rules, `.git/`, and a nested governed root. A kind that claims ignored
  documents reads them under its own locus, never by a whole-tree walk with
  the rules waived. A reader the contract holds to raw disk (a path-resolved
  edge) keeps raw disk, and reads only as far as the paths it resolves
  reach. Where an input has no owner yet, the work that first needs one
  builds it rather than adding another reader.
- **The pin is a count, never a clock.** A test asserts the work count
  per verb, taken at the owners rather than at one mechanism's counter,
  which a second mechanism walks past: one index per run, one pass over it
  serving every kind, one matcher per glob, no file opened twice —
  deterministic and machine-independent, where a wall-clock
  threshold is the gate that cries wolf. Timing stays a manual signal a
  human reads; the field's session experience is the true bar, and no
  corpus number stands in for it.
- **Diagnosis is measure-first.** When the field reports a cost, a
  generated fixture — synthetic input at consumer scale, built at test
  time, never committed — names where the cost concentrates before any
  cut; the numbers pick the cut, never the guess, and the fix lands
  with its count-pin.

**Tell:** a walk, read or glob compile inside a per-member or per-kind loop;
a wall-clock threshold in a test. **Held by:** count pins.
