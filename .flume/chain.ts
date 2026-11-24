/**
 * This repository's chain — the harness package's factory applied to the
 * declaration beside it, and nothing else (`@dtmd/flume`,
 * spec/harness.md, *Adoption and upgrade*). The engine loads this file and
 * refuses a tick without it; every slice, prompt, judge and gate it then
 * runs comes from the package.
 *
 * Written by `flume-harness init`, and the same hop in every repository
 * that adopts the package. What this environment decides it declares in
 * `./declaration.ts` — a behavior edited in here is one no version bump
 * carries forward.
 */

import type { ChainFactory } from "@dtmd/flume";
import { harnessChain } from "@dtmd/flume/harness";

// The `.js` names the `declaration.ts` beside this file: a TypeScript
// import carries the extension the emit would have, which every
// `moduleResolution` mode — and the loader that runs this file — resolves
// back to the source.
import { declaration } from "./declaration.js";

const factory: ChainFactory = (api) => ({
  chain: harnessChain({ api, declaration }),
});

export default factory;
