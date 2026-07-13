import { file } from "@dtmd/temper";
import { system } from "../../kinds.ts";

export const system_corpus = system({
  name: "corpus",
  satisfies: ["documented-spine"],
  prose: file(import.meta.url, "./corpus.md"),
});
