/**
 * `span()` — the computed-prose constructor. `` text`…` `` is a tagged template whose
 * interpolations are references by design, so words the program derived have no slot
 * to ride in; `span()` is the same inline prose from a plain string. The cases below
 * hold the three things that makes it: a reference-free `Text`, the same dedent, and a
 * `blocks()` child that emits byte-identical to the authored words.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { blocks, embeddedMemberValue, emit, harness, kind, span, text } from "../src/index.js";
import { memory } from "../src/claude-code.js";

/** The embedded kind the composed-body case nests, plus the admission its host needs. */
const decision = kind<Record<never, never>>({
  name: "decision",
  locus: { kind: "embedded" },
  unitShape: "file",
  registration: [],
});
const admitDecision = { host: memory, admits: [decision] };

/** The shape of a derived line: computed from values, never typed as a literal. */
function participantsLine(names: readonly string[]): string {
  return `Participants: ${names.join(", ")}.`;
}

test("a computed string lands as prose carrying no references at all", () => {
  const line = span(participantsLine(["rust", "sdk"]));
  assert.equal(line.kind, "text");
  assert.equal(line.template, "Participants: rust, sdk.");
  assert.deepEqual(line.mentions, []);
  assert.deepEqual(line.includes, []);
});

test("span() dedents by the same rule as the text tag, so the two agree word for word", () => {
  const words = `
          # Participants

          Participants: rust, sdk.
        `;
  assert.deepEqual(span(words), text`
          # Participants

          Participants: rust, sdk.
        `);
  assert.equal(span(words).template, "# Participants\n\nParticipants: rust, sdk.\n");
});

test("a span composes inside blocks() beside an embedded member and emits the authored words byte-identical", () => {
  const result = emit(
    harness({
      members: [
        memory({
          name: "CLAUDE",
          prose: blocks(
            span(participantsLine(["rust", "sdk"])),
            embeddedMemberValue({
              kind: decision,
              key: "surface-authority",
              leaves: { chosen: "the composition surface is canonical" },
            }),
          ),
        }),
      ],
      admit: [admitDecision],
    }),
  );

  const member = result.members.find((m) => m.name === "CLAUDE")!;
  assert.equal(
    member.body,
    "Participants: rust, sdk.\n\n" +
      '```member.decision surface-authority\nchosen = "the composition surface is canonical"\n```\n',
  );
  // A narrative span is prose: it mints no member and declares no edge of its own.
  assert.deepEqual(result.declarations.mentions, []);
  assert.deepEqual(
    result.declarations.nested_members.map((row) => row.kind),
    ["decision"],
  );
});

test("a computed string carrying a reserved reference marker is a loud error, never a silent mis-split", () => {
  // The words after the marker would be dropped on render — a computed string is
  // exactly where a stray one arrives, since no author typed it.
  assert.throws(() => span("Participants:\u0000 rust."), /reserved reference marker/);
  assert.throws(() => span("Participants:\u0001 rust."), /reserved reference marker/);
});
