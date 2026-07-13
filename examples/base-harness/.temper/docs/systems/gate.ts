import { file } from "@dtmd/temper";
import { system } from "../../kinds.ts";

export const system_gate = system({
  name: "gate",
  satisfies: ["documented-spine"],
  prose: file(import.meta.url, "./gate.md"),
});
