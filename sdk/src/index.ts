/**
 * temper's authoring face — the six-noun core as a typed module library.
 * A harness author imports plain nouns — `harness()`, the generic `kind`
 * constructor, the clause and requirement constructors, `needs`, and the three
 * prose constructors — and composes members as typed values. `emit` compiles the
 * whole into the declaration rows and the projected members' erased payload —
 * the JSON pipe printed to stdout; the engine is the sole compiler of every
 * projection and the whole lock.
 * Every type erases at the seam, and Turing-completeness
 * stays quarantined at authoring time.
 *
 * The first-party Claude Code provider face — the built-in `skill`/`rule`/
 * `memory` kinds — lives at the `./claude-code` subpath, never here.
 */

// Prose — three constructors, one field type; references (mention · include) ride `text`.
export type { Blocks, File, Include, Mention, Mentionable, Prose, Reference, Text } from "./prose.js";
export { blocks, file, include, mentionOf, renderText, text } from "./prose.js";

// Needs — the derived permission union's source.
export type { Capability } from "./needs.js";
export { bash, capability, permissionUnion } from "./needs.js";

// Contracts — clauses, predicates, requirements.
export type { Charset, Clause, ExtentUnit, Predicate, Requirement, Severity, Verifier } from "./contract.js";
export {
  allowedChars,
  clause,
  closedKeys,
  count,
  degree,
  deny,
  enumOf,
  extent,
  forbiddenKeys,
  formatPlacesEdges,
  globValid,
  maxLen,
  membership,
  mentionReachable,
  minLen,
  mustDefine,
  nameMatchesDir,
  optional,
  range,
  required,
  requireSections,
  requirement,
  script,
  sectionContains,
  shape,
  telemetry,
  type,
  unique,
  uniqueName,
} from "./contract.js";

// The engine room — kinds as typed constructors, plus the embedded-member value
// shape `blocks()` composes and the resolved shape a `render` hook receives.
export type {
  CollectionAddress,
  EdgeField,
  EdgeTargetFacts,
  EmbeddedMemberCollectionEntry,
  EmbeddedMemberValue,
  Format,
  KindDefinition,
  KindFacts,
  KindOptions,
  Layout,
  LayoutRegion,
  Locus,
  Member,
  MemberInit,
  Registration,
  ResolvedEmbeddedMemberCollectionEntry,
  ResolvedEmbeddedMemberValue,
  Shape,
  Template,
  UnitShape,
} from "./kind.js";
export { embeddedMemberValue, kind } from "./kind.js";

// The assembly — `harness()` and its six fields.
export type { Admission, EnforcementMode, ExpectBinding, Harness } from "./assembly.js";
export { harness } from "./assembly.js";

// Emit — the compile to the seam's JSON pipe; the engine is the sole compiler
// of every projection and the whole lock.
export type { EmitResult, RegistrationFact, ResolveOptions, SettingsResidue } from "./emit.js";
export { emit } from "./emit.js";

// The dial — temper's own shipped kind, and the one kind value the root exports: it
// declares how this machine reads temper's gate, which is a fact about temper rather
// than about any provider's harness. A provider's kinds stay behind their own subpath.
export type { Dial, DialEntry } from "./dial.js";
export { dial, dialDefaultContract } from "./dial.js";
