# Which binary does a synthesized tap hook run?

Driver: inbox record of 2026-11-25 (after GH #67). `sdk/src/declarations.ts` `TAP_COMMAND` renders a bare `temper tap "$CLAUDE_PROJECT_DIR"`. On a harness whose gate hooks state `binary: ".temper/node_modules/.bin/temper"` (0082) with temper off the PATH, every `InstructionsLoaded` and `PostToolUse` `Skill` tap exits 127, and the telemetry (`specs/model/pipeline.md`, "Telemetry") never records. The tap has no binary fact and no project anchor (0086), and a relative binary would also need the anchor after a Bash `cd`.

The corpus does not decide where the tap's binary comes from. 0082 makes the binary each gate hook's own fact and rejects a harness-wide field; 0078 wants one fact in one spelling.

Options: (1) the telemetry declaration states its own `binary`, defaulting to `temper` as the gate hook constructor does, and the tap command takes the 0086 anchor. (2) The tap borrows the gate hooks' binary, which needs a rule for no gate hooks or disagreeing ones. (3) Reopen 0082's harness-wide field.

Recommendation: 1. It follows 0082's reasoning (the binary is the hook's own fact), and an author shares one constant across hooks to keep 0078. (2) makes one member read another's field. Sections: `specs/model/pipeline.md` "Telemetry"; `specs/model/adoption.md` "Install".
