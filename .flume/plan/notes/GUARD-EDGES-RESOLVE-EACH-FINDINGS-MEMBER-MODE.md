# Guard edges and dial softening resolve each finding's member mode

Shipped. The shell edge, the co-owned manifest write and the gate's dial decision now
resolve member, then kind, then root, as the pending-write edge already did.

- Shell edge: the drift judges gained member-paired variants (`drift::MemberFinding`:
  `config_stale_members_from_doc`, `undeclared_layout_member_findings_from_doc`,
  `source_dependency_stale_members_from_doc`). A file-locus stranger names no member, so
  it resolves at its kind's mode. Locus sites are judged one kind-run at a time to keep
  that kind.
- One aggregator, `install::decide_by_mode`, serves both edges. `note` findings are
  dropped. If any finding is `block`, the call is refused, and the refusal lists the
  block findings first, then the warn ones. A warn finding therefore reaches the session
  inside the refusal reason, not through `additionalContext`. I did not verify whether
  PostToolUse can carry `decision` and `hookSpecificOutput` together. If it can, plan may
  want the warn findings split out.
- Manifest: a member-grain finding is mapped to its member through its `artifact`, which
  equals the feature id. A dropped key maps to `address_of(kind, key)`. Unparseable and
  unreconstructable findings resolve at the kind's mode.
- Dial: `two_greens_dispatch` dials at the kind's mode. A member whose own mode lands on
  the other side of `block` is judged by a contract dialed at its own mode. Kind
  selections dial at the kind's mode, requirement and root selections at the root's.

Gap: a requirement's member-grain clauses (`judge_members` over OptIn selections) still
dial at the root mode, even for a satisfier declaring `block`. Fixing that needs
per-member splitting of the selection. An entry should be filed if wanted.
Input/module source-dependency rows fall back to the root mode unless their `member`
column parses as an address.
