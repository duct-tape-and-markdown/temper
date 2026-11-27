<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# Supersede a decision

1. Declare the successor as its own decision module under
   `.temper/docs/decisions/`.
2. Rewrite the old ruling's module as `supersede(successor, { … })` from
   `.temper/kinds.ts`, the successor imported: its record moves to
   `docs/decisions/superseded/`, carrying the successor edge.
3. Never edit the accepted text into something new. Evidence against a
   standing ruling enters as a proposed amendment, never as a quiet
   rewrite.
4. Run `temper emit`, then `temper check .`.
