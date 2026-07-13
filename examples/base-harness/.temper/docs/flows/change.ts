import { file } from "@dtmd/temper";
import { flow, names } from "../../kinds.ts";
import { system_corpus } from "../systems/corpus.ts";
import { system_gate } from "../systems/gate.ts";

export const flow_change = flow({
  name: "change",
  participants: names(system_corpus, system_gate),
  prose: file(import.meta.url, "./change.md"),
});
