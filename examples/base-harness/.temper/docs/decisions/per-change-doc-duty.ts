import { file } from "@dtmd/temper";
import { supersede } from "../../kinds.ts";
import { decision_authorityArrow } from "./authority-arrow.ts";

// The lifecycle transition as a typed operation: the successor arrives as
// an import, so a superseded ruling without a live replacement cannot exist.
export const decision_perChangeDocDuty = supersede(decision_authorityArrow, {
  name: "per-change-doc-duty",
  prose: file(import.meta.url, "./per-change-doc-duty.md"),
});
