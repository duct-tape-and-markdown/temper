/**
 * The assembly on the authoring face (`specs/architecture/40-composition.md`):
 * kind registrations, requirement declarations, and member selection as one
 * typed value. `emit` compiles it into the inert manifest (`temper.toml` +
 * the lock) — the only thing the gate reads; no language runtime at check
 * time (`specs/intent/00-intent.md`, the authoring-face Decision).
 */

import type { Member } from "./members.js";

/** A declared kind registration — the package it binds, by name. */
export interface KindBinding {
  readonly package: string;
}

/** A harness-level requirement the assembly declares. */
export interface Requirement {
  readonly means: string;
  readonly kind: string;
  readonly required?: boolean;
}

/** The composed harness: everything emit serializes. */
export interface Harness {
  readonly kinds: Readonly<Record<string, KindBinding>>;
  readonly requirements: Readonly<Record<string, Requirement>>;
  readonly members: readonly Member[];
}

/** Compose the harness from its declared parts — ordinary code, quarantined at authoring time. */
export function defineHarness(init: {
  kinds?: Readonly<Record<string, KindBinding>>;
  requirements?: Readonly<Record<string, Requirement>>;
  members: readonly Member[];
}): Harness {
  return {
    kinds: init.kinds ?? {},
    requirements: init.requirements ?? {},
    members: init.members,
  };
}
