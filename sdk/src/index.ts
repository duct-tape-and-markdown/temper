/**
 * temper's authoring face — the typed module library
 * (`specs/intent/00-intent.md`, the authoring-face Decision;
 * `specs/architecture/20-surface.md`). Author members, kinds, genre values,
 * and the assembly as typed values; `emit` compiles them into the inert
 * manifest the gate reads — all Turing-completeness quarantined at authoring
 * time.
 */

export type { Body, Mention, Mentionable, Prose, ProseAsset } from "./prose.js";
export { fromFile, md } from "./prose.js";

export type {
  ManifestGenreValue,
  ManifestMember,
  ManifestPublishedRequirement,
  ManifestSection,
} from "./manifest.js";

export type { Alternative } from "./genres.js";
export { bound, decision, genre, law } from "./genres.js";

export type { Member, PublishedRequirement, SatisfiesClaim } from "./members.js";
export { customMember, memory, rule, skill } from "./members.js";

export type { Harness, KindBinding, Requirement } from "./assembly.js";
export { defineHarness } from "./assembly.js";

export type { EmitOptions, EmitResult, ResolveOptions } from "./emit.js";
export {
  emit,
  emitManifestMembers,
  serializeManifestMember,
  toManifestMember,
  writeEmit,
} from "./emit.js";

export type { Projection } from "./project.js";
export {
  isProjectedKind,
  projectBytes,
  projectionPath,
  projectMember,
  renderField,
} from "./project.js";

export type { LockRow } from "./lock.js";
export { lockRow, sha256Hex, stampLock } from "./lock.js";
