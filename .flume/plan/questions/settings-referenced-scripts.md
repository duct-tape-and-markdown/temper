# How does a script that settings run (hook script, status line) join the model?

Driver: a real-harness probe (0.0.25). 28 hook commands reach one hook-scripts folder through a shell prefix, and settings reference a status-line script. Both draw `coverage.unclaimed-entry`. The hook kind stops at the command string. Parsing a command for a path is mining (intent invariant 1), so it is ruled out.

Options, each consistent with the corpus:
1. a declared input on the hook or status-line member, the record-plus-input shape of 0064 ("a file temper does not write is a declared input"), written by the author;
2. a settings-level declared script edge;
3. name such scripts as unmodeled, reported as one grouped coverage note.

0064 points at (1). What is still open is the ergonomics for a harness with 28 of them, and what an unrepresented harness, which declares no inputs, reports. A ruling amends `specs/model/representation.md` or `contract.md`.
