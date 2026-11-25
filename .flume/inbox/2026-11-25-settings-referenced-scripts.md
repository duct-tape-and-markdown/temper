# Scripts that settings point at (hook scripts, the status line) are unclaimed, and the model stops at the command string

From the same real-harness probe (0.0.25):
- a hook-scripts folder that 28 hook commands reach through a shell prefix that finds the repo root and runs the script;
- a status-line script that settings reference.

Both draw `coverage.unclaimed-entry`. The hook kind stops at the command string, and parsing a command for a path would be mining (intent invariant 1), so that is not the fix. This is a question for plan to raise, not to settle. The options I see: (1) a declared input on the hook or status-line member (0064's record-plus-input shape, written by the author); (2) a settings-level declared script edge; (3) name such scripts unmodeled, as one grouped coverage note. 0064 already rules committed files temper does not write, which suggests (1). The ergonomics question remains for a harness with 28 of them.
