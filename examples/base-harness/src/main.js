/** CLI wiring: read a checklist file, scan it, print the summary. */

import { readFileSync } from "node:fs";
import { scan } from "./scan.js";
import { render } from "./render.js";

const path = process.argv[2];
if (path === undefined) {
  console.error("usage: node src/main.js <checklist.md>");
  process.exit(2);
}
console.log(render(scan(readFileSync(path, "utf8"))));
