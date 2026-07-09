import { emit, harness } from "@dtmd/temper";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_collaboration } from "./rules/collaboration.ts";
import { rule_pendingEntry } from "./rules/pending-entry.ts";
import { rule_rust } from "./rules/rust.ts";
import { rule_sdk } from "./rules/sdk.ts";
import { skill_captureFriction } from "./skills/capture-friction.ts";

const program = harness({
  members: [
    memory_CLAUDE,
    rule_collaboration,
    rule_pendingEntry,
    rule_rust,
    rule_sdk,
    skill_captureFriction,
  ],
});

process.stdout.write(emit(program).seam);
