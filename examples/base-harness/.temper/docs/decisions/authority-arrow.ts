import { file } from "@dtmd/temper";
import { decision } from "../../kinds.ts";

export const decision_authorityArrow = decision({
  name: "authority-arrow",
  prose: file(import.meta.url, "./authority-arrow.md"),
});
