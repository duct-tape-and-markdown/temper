/**
 * Scaffold smoke: author one rule and one decision genre value on the face,
 * emit the manifest fragment, and hold the disciplines the ratified corpus
 * names — deterministic ordering, double-emit stability, keyed (never
 * positional) collections, and loud failures on the deliberately-absent
 * slices (fromFile, mentions).
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import {
  decision,
  defineHarness,
  emitManifestMembers,
  fromFile,
  md,
  rule,
  toManifestMember,
} from "../src/index.js";

const surfaceAuthority = decision({
  key: "surface-authority",
  chosen: "the assembly declares the posture; enforcement reads it",
  rejected: {
    "baked-projection": { because: "bakes a stance law 2 says the author owns" },
  },
});

function harness() {
  return defineHarness({
    members: [
      rule({
        name: "rust",
        fields: { paths: ["src/**/*.rs"] },
        satisfies: {
          "engineering-standards": { rationale: "carries the Rust conventions" },
        },
        body: md`
          # Rust conventions

          Errors via miette/thiserror; clippy clean under -D warnings.
        `,
        genres: [surfaceAuthority],
      }),
    ],
  });
}

test("emit serializes the member into the manifest schema", () => {
  const toml = emitManifestMembers(harness());
  assert.match(toml, /^\[\[member\]\]$/m);
  assert.match(toml, /kind = "claude-code\.rule"/);
  assert.match(toml, /satisfies = \["engineering-standards"\]/);
  assert.match(toml, /^\[\[member\.section\]\]$/m);
  assert.match(toml, /# Rust conventions/);
});

test("a genre value serializes whole — leaves flat, collections keyed, never positional", () => {
  const toml = emitManifestMembers(harness());
  assert.match(toml, /^\[\[member\.genre\]\]$/m);
  assert.match(toml, /genre = "decision"/);
  assert.match(toml, /key = "surface-authority"/);
  assert.match(toml, /^\[member\.genre\.leaves\]$/m);
  assert.match(toml, /chosen = "the assembly declares the posture; enforcement reads it"/);
  assert.match(toml, /^\[member\.genre\.collections\.rejected\.baked-projection\]$/m);
  assert.doesNotMatch(toml, /rejected\.0/);
});

test("double-emit is byte-stable", () => {
  assert.equal(emitManifestMembers(harness()), emitManifestMembers(harness()));
});

test("the leaf address rides structure: member + genre key + field path", () => {
  const manifest = toManifestMember(harness().members[0]);
  const value = manifest.genres[0];
  assert.equal(value.key, "surface-authority");
  assert.equal(value.collections.rejected["baked-projection"].because.length > 0, true);
});

test("the absent slices fail loud, never silently", () => {
  const withAsset = defineHarness({
    members: [rule({ name: "long", body: fromFile("./long.md") })],
  });
  assert.throws(() => emitManifestMembers(withAsset), /fromFile resolution is not in the scaffold/);

  const withMention = defineHarness({
    members: [
      rule({
        name: "mentions",
        body: md`A ${{ address: "kind:rule", display: "rule" }} is declared.`,
      }),
    ],
  });
  assert.throws(() => emitManifestMembers(withMention), /mention resolution is not in the scaffold/);
});
