import { emit, harness } from "@dtmd/temper";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_collaboration } from "./rules/collaboration.ts";
import { rule_rust } from "./rules/rust.ts";

const program = harness({
  members: [memory_CLAUDE, rule_collaboration, rule_rust, ],
});

process.stdout.write(emit(program).seam);
