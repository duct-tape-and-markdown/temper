import { file } from "@dtmd/temper";
import { flow, names } from "../../kinds.ts";
import { system_corpus } from "../systems/corpus.ts";
import { system_gate } from "../systems/gate.ts";

export const flow_driftRepair = flow({
  name: "drift-repair",
  participants: names(system_gate, system_corpus),
  prose: file(import.meta.url, "./drift-repair.md"),
});
